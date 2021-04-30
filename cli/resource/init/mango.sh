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

img_date="$(docker inspect -f '{{ .Created }}' "$MANGO_DOCKER_IMAGE" 2> /dev/null)"
if [ $? -ne 0 ] || [ -z "$img_date" ]
then
    printf 'pulling docker image %s\n' "$MANGO_DOCKER_IMAGE"
    docker pull "$MANGO_DOCKER_IMAGE"
fi
img_ts="$(date +%s -d $img_date)"
age_s="$(expr "$(date +%s)" - "$img_ts")"
echo "age is $age_s s"
if [ $age_s -gt 43200 ]
then
    printf 'updating docker image %s\n' "$MANGO_DOCKER_IMAGE"
    docker pull "$MANGO_DOCKER_IMAGE"
fi

# Mango daemon

set -eu

docker run \
    --name "mangod-$PROJECT_NAME" \
    --rm \
    --entrypoint /mangod \
    "$MANGO_DOCKER_IMAGE"
