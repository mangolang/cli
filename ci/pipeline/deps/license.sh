#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    docker run --rm 'mango_cli_build' sh -c 'cargo deny init && cargo --offline deny check licenses'
)
