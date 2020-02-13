#!/bin/bash

printf "==========  Environment  ==========\n"
printf "WORKER_ID = $WORKER_ID\n"
printf "AWS_WEB_IDENTITY_TOKEN_FILE = $AWS_WEB_IDENTITY_TOKEN_FILE\n"
printf "AWS_ROLE_ARN = $AWS_ROLE_ARN\n"
printf "Test container role access with AWS CLI. get-caller-identity: \n$(aws sts get-caller-identity)\n"
#
# Get directory holding this script and change into it
#
SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
DIR="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
APP_DIR=$(dirname $DIR)
printf "Switching to app directory: $APP_DIR\n"
cd $APP_DIR
printf "==============  End  ==============\n"


printf "Running application with args: $@\n"
java $JAVA_OPTS -Dlog4j2.configurationFile=etc/log4j2.xml -classpath "lib/*" example.HelloWorld "$@"
