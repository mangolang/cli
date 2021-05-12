#!/usr/bin/env sh

# Wrapper that runs Mango using Docker. Arguments are passed on to Mango CLI.
# A continuously-running daemon container is started, if it is not running.

# Configuration is done through environment variables
# - PROJECT_NAME: name of the project (no whitespace)
# - MANGO_DOCKER_IMAGE: docker image including tag to use

if [ -z "$PROJECT_NAME" ];
then
    PROJECT_NAME="$(basename "$(pwd)")"
fi

if [ -z "$MANGO_DOCKER_IMAGE" ];
then
    MANGO_DOCKER_IMAGE="mangocode/mango:latest"
fi

# Checks

set -eu

if ! command -v docker 1> /dev/null 2> /dev/null
then
    printf "Docker must be installed to run Mango\n" 1>&2
    exit 1
fi

#TODO @mark: check that this is the root dir of the Mango project

# Mango daemon

if [ -z "$(docker ps -q -f name=mangod)" ]
then
    printf "Starting mango daemon container (mangod)\n"
    docker run \
        --name "mangod" \
        --label started-by="$PROJECT_NAME" \
        -d --rm \
        -p 47558:47558 \
        "$MANGO_DOCKER_IMAGE" \
            -- run-as-daemon \
            --hostname 127.0.0.1 --port 47558
fi

# Mango cli

docker run \
    --name "mango-$PROJECT_NAME" \
    -it --rm \
    --mount type=bind,source="$(pwd)",target=/code \
    "$MANGO_DOCKER_IMAGE" \
        -- "$@"

