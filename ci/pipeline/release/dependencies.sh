#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    docker run --rm 'mango_cli_build' cargo tree --all-features
)
