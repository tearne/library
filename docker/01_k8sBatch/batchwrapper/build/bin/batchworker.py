#!/usr/bin/env python3

import boto3, json, argparse, subprocess, os, logging, sys, uuid, shutil, re
from tempfile import TemporaryDirectory
from pathlib import Path
from time import sleep

def setup_logger():
    # TODO save log to file and upload frequently
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
    log.info("Arguments: %s", args)
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
    log.info("SQS Region: %s, Account: %s, QName: %s",region, account, name)
    resource = boto3.resource('sqs', region_name = region)
    return resource.get_queue_by_name(QueueName = name, QueueOwnerAWSAccountId = account)

class JobConfig:
    # Parses the job message and extracts details such as user script name and results files to upload.
    # Also creates the temporary directory which the job will be copied into and executed from.
    def __init__(self, raw_json):
        parsed = json.loads(raw_json)

        try:
            self.id = self.__get_job_id(parsed)
            self.job_dir = Path(parsed["job"]["app"]["directory"])
            self.script_path = Path(parsed["job"]["app"]["executable"])
            assert (self.job_dir/self.script_path).exists(), "Path {} doesn't exist".format(self.job_dir/self.script_path)
            self.bucket_name = parsed["job"]["results"]["bucket"]
            self.results_paths = list(map(lambda s: Path(s), parsed["job"]["results"]["uploadData"]))
        except KeyError as e:
            log.exception("Failed to parse job config\n%s\n", json.dumps(parsed, indent=2))

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
                return str(pod_uid+"_"+format(processed_counter, '03'))
            else:
                log.info("Falling back on UUID for job ID")
                return str(uuid.uuid1())

    def __repr__(self):
        # TODO simpler way to do nice __str__?
        return self.__class__.__name__+'JobConfig(' \
            'id = %s, job_dir = %s, script_path = %s, results_paths = %s, bucket_name = %s, tmp_dir = %s)' \
            % (self.id, self.job_dir, self.script_path.absolute(), self.results_paths, self.bucket_name, self.tmp_dir)



def run_job(job_config, processed_counter):
    job_dir = job_config.job_dir.resolve()
    tmp_dir = job_config.tmp_dir
    raw_json = job_config.raw_json
    job_id = job_config.id
    script_path = job_config.script_path

    log.info("Setup temporary run dir: copying %s to %s", str(job_dir), str(tmp_dir))
    shutil.copytree(str(job_dir), str(tmp_dir))

    config_file = job_config.tmp_dir/'job_config.json'
    with open(str(config_file), 'w', encoding='utf-8') as file:
        file.write(raw_json)
    log.info("Wrote config file: %s", str(config_file))

    job_env = os.environ.copy()
    job_env["JOB_ID"] = job_id

    command = [str(tmp_dir/script_path),  str(tmp_dir/config_file.name)]
    log.info("Running command: %s", ' '.join(command))

    try:
        completed = subprocess.run(command, check=True, env=job_env)
        log.info(completed)
    except subprocess.CalledProcessError as e:
        log.error("command '{}' return with error (code {}): {}".format(e.cmd, e.returncode, e.output))


def clean_up(job_config):
    tmp_dir = str(job_config.tmp_dir)
    log.info("Delete temporary run directory %s", tmp_dir)
    shutil.rmtree(tmp_dir)


def upload_results(job_config):
    log.info("Uploading: {}".format(job_config.results_paths))
    for path_str in job_config.results_paths:
        path = job_config.tmp_dir/path_str
        command = ["aws", "s3", "cp", str(path.resolve()), "s3://"+job_config.bucket_name+"/"+job_config.id, "--acl",  "bucket-owner-full-control"]
        if path.is_dir():
            command.append("--recursive")

        log.info("Running upload command: %s",' '.join(command))
        try:
            completed = subprocess.run(command, check=True)
            log.info(completed)
        except subprocess.CalledProcessError as e:
            # TODO e.output just says 'None' upon error
            log.error("S3 upload command '{}' return with error (code {}): {}".format(e.cmd, e.returncode, e.output))


def get_msg(queue):
    msg_list = queue.receive_messages(MaxNumberOfMessages=1)
    if len(msg_list) == 0:
        log.info("No more messages.")
        return None
    else:
        msg = msg_list[0]
        msg.delete()
        return msg


limit = args.limit_jobs
processed_counter = 0
sqs = setup_queue()
empty_retries = 60 # 10 min
retries_remaining = empty_retries
retry_seconds = 10

def keep_looping():
    return retries_remaining > 0 and (limit == 0 or processed_counter < limit)

while keep_looping():
    # Dequeue a message
    msg = get_msg(sqs)
    if msg is None:
        retries_remaining -= 1
        log.warning("No messages from SQS, sleeping for %i seconds.  Attempts left %i", retry_seconds, retries_remaining)
        sleep(retry_seconds)
        continue
    else:
        retries_remaining = empty_retries
    log.debug("Raw message body: \n%s", msg.body)
    processed_counter += 1

    # Parse msg to a job config
    job_config = JobConfig(msg.body)
    log.info(str(job_config))

    run_job(job_config, processed_counter)

    upload_results(job_config)

    clean_up(job_config)

log.info("Worker ran %i job(s), exiting", processed_counter)
