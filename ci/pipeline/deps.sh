#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/shared.sh"

# Checks that dependencies are up-to-date and safe.

(
    set -x
    docker run --rm 'mango_cli_build' cargo --offline outdated --exit-code 1
    docker run --rm 'mango_cli_build' cargo --offline audit --deny-warnings
    docker run --rm 'mango_cli_build' cargo --offline deny init
    docker run --rm 'mango_cli_build' cargo --offline deny check licenses
    docker run --rm 'mango_cli_build' cargo --offline deny check bans
    docker run --rm 'mango_cli_build' cargo --offline deny check advisories

    #TODO @mark: store these reports somewhere
    # This does not check anything, just puts the dependencies in the log:
    docker run --rm 'mango_cli_build' cargo --offline tree --all-features
    # This does not check anything, just reports which things take space:
    docker run --rm 'mango_cli_build' cargo --offline bloat --release --all-features -n 50
)

#TODO @mark: check unused dependencies: cargo +nightly udeps --all-targets
