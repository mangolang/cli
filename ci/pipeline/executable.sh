#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/shared.sh"

(
    set -x
    docker build -t 'mangocode/mango' -f  'ci/image/executable.Dockerfile' .
)
