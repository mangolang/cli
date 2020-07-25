#!/usr/bin/env bash

# Build the current crate's code on top of pre-compiled dependencies.
# Note: this is called as part of shared so that all staps have this image,
#   but it could also be called directly as a step to check build.

(
    set -x
    docker build -t 'mango_cli_build' -f  'ci/image/build.Dockerfile' .
)
