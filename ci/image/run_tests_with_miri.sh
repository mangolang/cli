#!/usr/bin/env bash

if [[ ! -v NIGHTLY_VERSION ]]; then
    printf "*********************************************\n" 1>&2
    printf "* NIGHTLY_VERSION not defined, using latest *\n" 1>&2
    printf "* will fail if components are not available *\n" 1>&2
    printf "*********************************************\n" 1>&2
    NIGHTLY_VERSION=nightly
fi

# shellcheck disable=SC2034
(
    set -x
    CARGO_TARGET_DIR="target/miri"
    cargo +"$NIGHTLY_VERSION" install xargo
    rustup +"$NIGHTLY_VERSION" component add miri
    cargo +"$NIGHTLY_VERSION" --offline miri test --all-targets --all-features
)
