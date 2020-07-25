#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    docker build -t 'mango_ci_static_exe' -f  'ci/image/executable.Dockerfile' .
    docker run --rm --mount type=bind,src="$RELEASE_PATH",dst='/host' 'mango_ci_static_exe' \
        sh -c 'cp /mango /host/mango'
)
