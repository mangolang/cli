#!/usr/bin/env bash

# Run tests using miri to detect more possible issues.

(
    set -x
    docker run --rm 'mango_cli_build'\
        ./run_tests_with_miri.sh
)
