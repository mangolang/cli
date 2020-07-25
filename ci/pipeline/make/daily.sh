#!/usr/bin/env bash

# Builds an image with pre-compiled dependencies.
#
# This can be built once a day, so that all the CI tasks during the day will be much faster.

(
    set -x
    docker build -t 'mangocode/mango_daily_base' -t 'tmp_mango_daily' -f  'ci/image/base.Dockerfile' .
)
