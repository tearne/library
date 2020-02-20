#!/usr/bin/env python3

import boto3, json, argparse, subprocess, os, logging, sys, uuid, shutil, re
from tempfile import TemporaryDirectory
from pathlib import Path
from time import sleep

def setup_logger():
    # TODO save log to file and upload frequently
    # Only send warn or worse to stderr
    log = logging.getLogger(__name__)
    log.setLevel(logging.DEBUG)
    h1 = logging.StreamHandler(sys.stdout)
    h1.setLevel(logging.DEBUG)
    h1.setFormatter(logging.Formatter(logging.BASIC_FORMAT))
    h1.addFilter(lambda record: record.levelno <= logging.INFO)
    h2 = logging.StreamHandler()
    h2.setLevel(logging.WARNING)
    h2.setFormatter(logging.Formatter(logging.BASIC_FORMAT))
    log.addHandler(h1)
    log.addHandler(h2)
    return log
log = setup_logger()


def parse_args():
    example_config = json.loads("""
{
    "job" : {
        "app": {
            "directory" : "/app",
            "executable": "bin/run.sh"
        },
        "results" : {
            "bucket" : "my-results-bucket",
            "uploadData" : ["results.csv", "logs"]
        },
        "id": "myJob-0012"
    }
}
    """)
    description_txt = \
        "Pulls one JSON config from SQS at a time, executes the job, and uploads results at the end.  " \
        "The config MUST specify job script and results upload data, and may specify a job ID.  For example:\n\n" \
        + json.dumps(example_config, indent = 2) + \
        "\nTo keep the environment clean, for each run the entire working directory is " \
        "copied, executed, then deleted, before starting the next run."
    parser = argparse.ArgumentParser(
        formatter_class=argparse.RawDescriptionHelpFormatter,
        description = description_txt
    )
    parser.add_argument('queue', help = "SQS queue to pull jobs from.")
    parser.add_argument('--debug', '-d', dest = 'debug', default = False, action='store_true', help='Switch on debug mode')
    parser.add_argument('--limit-jobs', '-l', nargs = '?', default = 0, type=int, help='limit the number of jobs to process before exiting')
    args = parser.parse_args()
    log.debug(f"Arguments: {args}")
    return args
args = parse_args()


if not args.debug:
    log.setLevel(level=logging.INFO)


def check_my_role():
    sts = boto3.client('sts')
    # I prefer the look of printing json to pprinting a python dictionary,
    # so will do that instead of pprint(sts.get_caller_identity())
    log.debug(json.dumps(sts.get_caller_identity(), indent=2, sort_keys=True))
check_my_role()


def setup_queue():
    m = re.search("([^.]+).amazonaws.com\/([a-zA-Z0-9]{12})\/(\w+)", args.queue)
    region = m.group(1)
    account = m.group(2)
    name = m.group(3)
    log.debug(f"SQS Region: {region}, Account: {account}, QName: {name}")
    resource = boto3.resource('sqs', region_name = region)
    return resource.get_queue_by_name(QueueName = name, QueueOwnerAWSAccountId = account)

class JobConfig:
    # Parses the job message and extracts details such as user script name and results files to upload.
    # Also chooses the temporary directory which the job will be copied into and executed from.
    def __init__(self, raw_json):
        parsed = json.loads(raw_json)

        try:
            self.id = self.__get_job_id(parsed)
            self.job_dir = Path(parsed["job"]["app"]["directory"])
            self.script_path = Path(parsed["job"]["app"]["executable"])
            assert (self.job_dir/self.script_path).exists(), f"Path {self.job_dir/self.script_path} doesn't exist"
            self.bucket_name = parsed["job"]["results"]["bucket"]
            self.results_paths = list(map(lambda s: Path(s), parsed["job"]["results"]["uploadData"]))
        except KeyError as e:
            log.exception(f"Failed to parse job config\n{json.dumps(parsed, indent=2)}\n")

        self.tmp_dir = Path(TemporaryDirectory().name)
        self.raw_json = raw_json

    def __get_job_id(self, parsed):
        try:
            jid = parsed["job"]["id"]
            log.info("Found job ID in json config")
            return str(jid)
        except KeyError:
            # Try to get WORKER_ID from environment variable, else make up UUID
            if "WORKER_ID" in os.environ:
                pod_uid = os.environ.get("WORKER_ID")
                log.info("Found $WORKER_ID. Will append with local task number to form job ID")
                return f"{pod_uid}_{processed_counter:03d}"
            else:
                log.info("Falling back on UUID for job ID")
                return str(uuid.uuid1())

    def __repr__(self):
        return f"{self.__class__.__name__}(" \
               f"id = {self.id}, job_dir = {self.job_dir}, script_path = {self.script_path.absolute()}, " \
               f"results_paths = self.results_paths, bucket_name = self.bucket_name, tmp_dir = self.tmp_dir)"



def run_job(job_config, processed_counter):
    job_dir = job_config.job_dir.resolve()
    tmp_dir = job_config.tmp_dir
    raw_json = job_config.raw_json
    job_id = job_config.id
    script_path = job_config.script_path

    log.info(f"Setup temporary run dir: copying {job_dir} to {tmp_dir}")
    shutil.copytree(str(job_dir), str(tmp_dir))

    config_file = job_config.tmp_dir/'job_config.json'
    with open(str(config_file), 'w', encoding='utf-8') as file:
        file.write(raw_json)
    log.debug(f"Wrote config file: {config_file}")

    job_env = os.environ.copy()
    job_env["JOB_ID"] = job_id

    command = [str(tmp_dir/script_path),  str(tmp_dir/config_file.name)]
    log.info(f"Running command: {' '.join(command)}")

    try:
        completed = subprocess.run(command, check=True, env=job_env)
        log.debug(completed)
    except subprocess.CalledProcessError as e:
        log.error(f"command '{e.cmd}' return with error (code {e.returncode}): {e.output}")


def clean_up(job_config):
    tmp_dir = str(job_config.tmp_dir)
    log.info(f"Delete temporary run directory {tmp_dir}")
    shutil.rmtree(tmp_dir)


def upload_results(job_config):
    log.info(f"Uploading: {job_config.results_paths}")
    log.debug(f"The working directory contains files/folders: {os.listdir(job_config.tmp_dir)}")

    for path_str in job_config.results_paths:
        path = job_config.tmp_dir/path_str

        if path.is_dir():
            command = [
                "aws", "s3",
                "cp", str(path.resolve()),
                f"s3://{job_config.bucket_name}/{job_config.id}/{path.name}",
                "--acl",
                "bucket-owner-full-control",
                "--recursive"
            ]
        else:
            command = [
                "aws", "s3",
                "cp", str(path.resolve()),
                f"s3://{job_config.bucket_name}/{job_config.id}",
                "--acl",
                "bucket-owner-full-control"
            ]

        log.debug(f"Running upload command: {' '.join(command)}")
        try:
            completed = subprocess.run(command, check=True)
            log.debug(completed)
        except subprocess.CalledProcessError as e:
            # TODO e.output just says 'None' upon error
            log.error(f"S3 upload command '{e.cmd}' return with error (code {e.returncode}): {e.output}")

def get_msg(queue, retries, retry_seconds):
    msg_list = queue.receive_messages(MaxNumberOfMessages=1)
    for _ in range(retries):
        if msg_list:
            msg = msg_list[0]
            msg.delete()
            return msg
        log.warning(f"No messages from SQS, sleeping for {retry_seconds} seconds.  Attempts left {retries_remaining}")
        sleep(retry_seconds)
    else:
        log.debug("No more messages")
        return None

job_limit = args.limit_jobs
sqs = setup_queue()
RETRY_LIMIT = 30
RETRY_SECONDS = 10


for job_number in range(job_limit):
    msg = get_msg(sqs, RETRY_LIMIT, RETRY_SECONDS)
    if msg:
        log.debug(f"Raw message body: \n{msg.body}", )
        job_config = JobConfig(msg.body)
        log.info(str(job_config))
        run_job(job_config, job_number)
        upload_results(job_config)
        clean_up(job_config)
    else:
        break
else:
    job_number = job_limit

log.info(f"Worker ran {job_number} job(s), exiting")
