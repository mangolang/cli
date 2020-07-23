#!/usr/bin/env bash

source 'ci/shared.sh'

(
    set -x
    docker build -t 'mangocode/mango' -f  'ci/executable.Dockerfile' .
)
