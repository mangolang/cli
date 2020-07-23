#!/usr/bin/env bash

source 'ci/shared.sh'

docker run --rm -it 'mango_cli_build'\
    cargo --offline test --release --all-targets --all-features --all
