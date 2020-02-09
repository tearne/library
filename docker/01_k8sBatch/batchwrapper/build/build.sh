#!/bin/bash
#
# Note this needs sbt.  Install with SDK: https://sdkman.io/install
#  sdk install sbt
#

set -e

BUILD_CONTEXT=$HOME/dockerBuildContext
CONTAINER_NAME="batch-test"

COMPILED_STAGE=../target/universal/stage

echo "Your build context is $BUILD_CONTEXT"

build_version=$(
    find '..' -name "build.sbt" |
    head -n1 |
    xargs grep '[ \t]*version[ \t]*:=' |
    head -n1 |
    sed 's/.*"\(.*\)".*/\1/' |
    awk '{print tolower($0)}'
)

echo "Building $CONTAINER_NAME version $build_version"

( cd .. && sbt clean update stage )

echo "Syncing build directory....."

mkdir -p ${BUILD_CONTEXT}/app/etc
mkdir -p ${BUILD_CONTEXT}/app/bin
mkdir -p ${BUILD_CONTEXT}/app/lib
mkdir -p ${BUILD_CONTEXT}/app/archive

echo "copying to $BUILD_CONTEXT/app"

# Application files
rsync --delete -az -v --progress etc ${BUILD_CONTEXT}/app/
rsync --delete -az -v --progress bin ${BUILD_CONTEXT}/app/
rsync --delete -az -v --progress ${COMPILED_STAGE}/lib ${BUILD_CONTEXT}/app/

# Meta for archival
rsync --delete -az -v --progress . ${BUILD_CONTEXT}/app/meta/
rsync -az -v --progress ../build.sbt ${BUILD_CONTEXT}/app/meta/

# Docker build files
rsync -az -v --progress Dockerfile ${BUILD_CONTEXT}

echo "Changing into build context directory ..."
cd $BUILD_CONTEXT

echo "Performing docker build ..."
docker build -t ${CONTAINER_NAME}:${build_version} .

cd ..
echo "Saving docker image"
docker save -o ~/${CONTAINER_NAME}-${build_version}.tar ${CONTAINER_NAME}:${build_version}
