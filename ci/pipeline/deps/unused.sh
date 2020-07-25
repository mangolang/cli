#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    exit 1
    #TODO @mark: check unused dependencies: cargo +nightly udeps --all-targets
)
