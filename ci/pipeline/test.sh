#!/usr/bin/env bash

# Run unit tests. There is a separate script to run tests with miri.

(
    set -x
    docker run --rm 'mango_cli_build'\
        cargo --offline test --all-targets --all-features
)
