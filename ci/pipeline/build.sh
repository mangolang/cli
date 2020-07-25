#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/shared.sh"

(
    set -x
    docker build -t 'mango_cli_build' -f  'ci/image/build.Dockerfile' .
)
