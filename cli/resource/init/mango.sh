#!/usr/bin/env sh

# Wrapper that runs Mango using Docker. Arguments are passed on to Mango CLI.
# A continuously-running daemon container is started, if it is not running.

# Configuration is done through environment variables

if [ -z "$PROJECT_NAME" ];
then
    PROJECT_NAME="$(basename "$(pwd)")"
fi

if [ -z "$MANGO_DOCKER_IMAGE" ];
then
    MANGO_DOCKER_IMAGE="mangocode/mango:latest"
fi

#TODO @mark: cache dir etc, from Mango itself during init? but that'd make the wrapper platform-dependent


# Docker & image

if ! command -v docker 1> /dev/null 2> /dev/null
then
    echo "Docker must be installed to run Mango"
    exit 1
fi

# Mango daemon

set -eu

#TODO @mark: port might collide with other projects
docker run \
    --name "mangod-$PROJECT_NAME" \
    --rm \
    --entrypoint /mangod \
    -p 47558:47558 \
    "$MANGO_DOCKER_IMAGE" \
        --hostname 127.0.0.1 --port 47558
