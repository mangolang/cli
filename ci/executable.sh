#!/usr/bin/env bash

source 'ci/shared.sh'

docker run --rm -it 'mangocode/mango' \
    cargo --offline clippy --release --all-targets --all-features -- -D warnings
