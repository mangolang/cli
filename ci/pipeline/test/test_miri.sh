#!/usr/bin/env bash

# Run tests using miri to detect more possible issues.

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    docker run --rm 'mango_cli_build'\
        ./run_tests_with_miri.sh
)
