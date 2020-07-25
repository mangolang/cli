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

    # If necessary, build the daily pre-compiled-dependencies image.
    # Ideally this should be downloaded instead of built.
    if ! docker pull 'mangocode/mango_daily_base:latest' > /dev/null
    then
        printf '***************************************************************************\n' 1>&2
        printf '* Could not find base Docker image "mangocode/mango_daily_base" !         *\n' 1>&2
        printf '* It will be built. This means pre-compiled dependencies are not working, *\n' 1>&2
        printf '* and the build will be much slower than it should be.                    *\n' 1>&2
        printf '***************************************************************************\n' 1>&2

        source "${BASH_SOURCE%/*}/make/daily.sh"
    fi

    # Make a debug-mode image for further CI steps.
    source "${BASH_SOURCE%/*}/make/debug.sh"

    # Create a function to run steps inside the image.
    function CHECK() {
        (
            printf "[@mango_ci] $*\n"
            docker run --rm 'mango_ci:latest' "$@"
        )
    }

    printf 'setup completed\n'
fi
