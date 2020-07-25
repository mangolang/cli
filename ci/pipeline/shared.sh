#!/usr/bin/env bash

if [[ ! -v _IS_SHARED_SCRIPT_SOURCED ]]
then
    _IS_SHARED_SCRIPT_SOURCED='yes'

    set -e  # fail if a command fails
    set -E  # technical change so traps work with -E
    set -o pipefail  # also include intermediate commands in -e
    set -u  # undefined variables are errors

    if [[ ! -d '.git' ]] || [[ ! -f 'Cargo.toml' ]]
    then
        printf 'must run from the project root\n' 1>&2
        exit 1
    fi

    if ! docker pull 'mangocode/mango_daily_base:latest' > /dev/null
    then
        printf '***************************************************************************\n' 1>&2
        printf '* Could not find base Docker image "mangocode/mango_daily_base" !         *\n' 1>&2
        printf '* It will be built. This means pre-compiled dependencies are not working, *\n' 1>&2
        printf '* and the build will be much slower than it should be.                    *\n' 1>&2
        printf '***************************************************************************\n' 1>&2

        (
            set -x
            docker build -t 'mangocode/mango_daily_base' -f  'ci/image/base.Dockerfile' .
        )
    fi

    printf 'setup completed\n'
fi
