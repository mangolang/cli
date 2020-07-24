#!/usr/bin/env bash

source 'ci/shared.sh'

(
    set -x
    docker run --rm 'mango_cli_build'\
        cargo --offline test --release --all-targets --all-features --all
)
