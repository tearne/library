#!/usr/bin/env bash

set -eu

SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
DIR="$( cd -P "$( dirname "$SOURCE" )" && pwd )"

cd $DIR
echo "Working dir is $(echo $DIR)"

source config.sh

LIB=lib
JARS=$(files=("$LIB"/*.jar); IFS=,; echo "${files[*]}")

HADOOP_HOME=$DIR/$HADOOP

export            JAVA_HOME=$DIR/$JAVA/jre
export      LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}$HADOOP_HOME/lib/native
export SPARK_DIST_CLASSPATH=$($HADOOP_HOME/bin/hadoop classpath)

echo "Submitting job ..."
$SPARK/bin/spark-submit \
    --driver-java-options "-Dlog4j.configuration=file://$DIR/resources/log4j.properties" \
    --class $CLASS \
    --master local[*] \
    --jars $JARS \
    $LIB/$APP_JAR \
    $ARGS