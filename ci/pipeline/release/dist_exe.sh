#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    echo "RELEASE_PATH = $RELEASE_PATH"
    docker run --rm --mount type=bind,src="$RELEASE_PATH",dst='/host' 'mango_ci:latest' \
        sh -c 'cp /mango/target/release/mango /host/mango'
)
