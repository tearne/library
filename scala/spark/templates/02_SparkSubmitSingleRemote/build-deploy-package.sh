#!/usr/bin/env bash

set -eu

sbt -batch stage

source config.sh

mkdir -p $DEPLOY_DIR

cat <<END > $DEPLOY_DIR/config.sh
set -eu
#!/usr/bin/env bash
APP_JAR=$APP_JAR
CLASS=$CLASS
ARGS="$ARGS"

 SPARK=$(basename $SPARK)
HADOOP=$(basename $HADOOP)
  JAVA=$(basename $JAVA)
END
chmod +x $DEPLOY_DIR/config.sh

rsync -avhP --delete \
    templates/submit.sh $JAVA $SPARK $HADOOP $LIB $RESOURCES \
    $DEPLOY_DIR

echo "Deployment package built at $DEPLOY_DIR"
