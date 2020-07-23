#!/usr/bin/env bash

source 'ci/shared.sh'

docker run --rm -it 'mango_cli_build'\
    cargo --offline fmt --all -- --check

#TODO @mark: fail on problems
