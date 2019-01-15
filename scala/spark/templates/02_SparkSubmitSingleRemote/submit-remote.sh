#!/usr/bin/env bash

set -eu

sbt -batch stage

source config.sh
source build-deploy-package.sh

rsync -avhP --delete $DEPLOY_DIR -e ssh ubuntu@$RUNNER:

ssh ubuntu@$RUNNER <<END
#!/usr/bin/env bash
set -eu
nohup deploy/submit.sh >out.log 2>&1 &
tail -100f out.log
END
