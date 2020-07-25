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

        source "${BASH_SOURCE%/*}/make/base.sh"
    fi

    # Make a debug-mode image for further CI steps.
    source "${BASH_SOURCE%/*}/make/debug.sh"

    # Create a function to run steps inside the image.
    function CHECK() {
        (
            printf "[@mango_ci] $*\n" 1>&2
            docker run --rm 'mango_ci:latest' "$@"
        )
    }

    # Create / clean release directory (this is outside the Docker image)
    CRATE_NAME="$(grep -h -m1 '^name\s*=\s*"[^"]*"' Cargo.toml | sed 's/^name\s*=\s*"\([^"]*\)".*/\1/g')"
    CRATE_VERSION="$(grep -h -m1 '^version\s*=\s*"[^"]*"' Cargo.toml | sed 's/^version\s*=\s*"\([^"]*\)".*/\1/g')"
    GIT_BRANCH="$(git rev-parse --abbrev-ref HEAD | sed 's/_/-/g')"
    if [ "$GIT_BRANCH" = "master" ]; then RELEASE_NAME="${CRATE_NAME}-${CRATE_VERSION}"; else RELEASE_NAME="${CRATE_NAME}-${GIT_BRANCH}-${CRATE_VERSION}"; fi
    RELEASE_PATH="target/$RELEASE_NAME"
    printf 'release name: %s\n' "$RELEASE_NAME"
    mkdir -p "$RELEASE_PATH"
    rm -rf "${RELEASE_PATH:?}/*"

    printf 'setup completed\n'
fi
