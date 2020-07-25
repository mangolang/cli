#!/usr/bin/env bash

if [ $# -lt 1 ]; then
    printf "no argument provided; first argument should be 'default' or 'miri'\n" 1>&2
    exit 1
fi

source "${BASH_SOURCE%/*}/shared.sh"

if [ "$1" = 'default' ]; then
    (
        set -x
        docker run --rm 'mango_cli_build'\
            cargo --offline test --all-targets --all-features
    )
elif [ "$1" = 'miri' ]; then
    (
        set -x
        docker run --rm 'mango_cli_build'\
            ./run_tests_with_miri.sh
    )
else
    printf "first argument should be 'default' or 'miri', but was '$1'\n" 1>&2
    exit 2
fi
