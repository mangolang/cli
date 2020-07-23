#!/usr/bin/env bash

source 'ci/shared.sh'

(
    set -x
    docker build -t 'mango_cli_build' -f  'ci/build.Dockerfile' .
)
