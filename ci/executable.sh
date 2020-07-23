#!/usr/bin/env bash

source 'ci/shared.sh'

docker run --rm -it 'mango_cli_build'\
    cargo --offline clippy --release --all-targets --all-features -- -D warnings
