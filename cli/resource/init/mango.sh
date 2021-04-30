#!/usr/bin/env sh

# Wrapper that runs Mango using Docker. Arguments are passed on to Mango CLI.
# A continuously-running daemon container is started, if it is not running.

if [ -z "$PROJECT_NAME" ];
then
    PROJECT_NAME="$(basename "$(pwd)")"
fi

if [ -z "$MANGO_DOCKER_IMAGE" ];
then
    MANGO_DOCKER_IMAGE="mangocode/mango:latest"
fi


if ! command -v docker 1> /dev/null 2> /dev/null
then
    echo "Docker must be installed to run Mango"
    exit 1
fi

docker run \
    --name "mangod-$PROJECT_NAME" \
    --rm \
    --entrypoint /mangod \
    "$MANGO_DOCKER_IMAGE"
