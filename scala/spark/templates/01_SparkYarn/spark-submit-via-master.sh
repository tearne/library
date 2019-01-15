#!/bin/bash

set -e

IP=10.0.6.2

sbt -batch clean
sbt -batch assembly
scp target/scala-2.11/spark-yarn-assembly-1.0.jar hadoop@${IP}:

ssh hadoop@${IP} <<'END'
    nohup spark-submit \
        --master yarn \
        --deploy-mode client \
        --class SparkWordCount \
        spark-yarn-assembly-1.0.jar s3://somebucket/file.txt 5 \
        >spark.out 2>&1
    tail -f spark.out
END
