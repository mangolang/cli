#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/shared.sh"

# Checks that dependencies are up-to-date and safe.

(
    set -x

    #TODO @mark: store these reports somewhere
    # This does not check anything, just puts the dependencies in the log:
    docker run --rm 'mango_cli_build' cargo --offline tree --all-features
    # This does not check anything, just reports which things take space:
    docker run --rm 'mango_cli_build' cargo --offline bloat --release --all-features -n 50
)

#TODO @mark: check unused dependencies: cargo +nightly udeps --all-targets
