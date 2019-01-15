#!/usr/bin/env bash
# Remember to do SBT assembly to create fat jar
# See https://issues.apache.org/jira/browse/SPARK-15343
# went to etc/hadoop/conf/yarn-site.xml and set yarn.timeline-service.enabled  to false

export YARN_CONF_DIR=~/Spikes/SparkYarn/etc/hadoop/conf

/home/ubuntu/spark-2.2.0-bin-hadoop2.7/bin/spark-submit \
    --master yarn \
    --deploy-mode client \
    --class SparkPi \
    /home/ubuntu/Spikes/spark-yarn/target/scala-2.11/spark-yarn-assembly-1.0.jar

