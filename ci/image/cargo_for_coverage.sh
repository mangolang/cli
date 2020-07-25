#!/usr/bin/env bash

# Usage: ./cargo_for_coverage.sh test [args]
# This uses grcov and is described in
# https://github.com/mozilla/grcov#example-how-to-generate-gcda-files-for-a-rust-project

if [[ $# -lt 1 || ( "$1" != 'build' && "$1" != 'run' ) ]]; then
    printf "provide argument 'build' or 'run'\n" 1>&2
    exit 1
fi
if [[ ! -v NIGHTLY_VERSION ]]; then
    printf "*********************************************\n" 1>&2
    printf "* NIGHTLY_VERSION not defined, using latest *\n" 1>&2
    printf "* will fail if components are not available *\n" 1>&2
    printf "*********************************************\n" 1>&2
    NIGHTLY_VERSION=nightly
fi

# shellcheck disable=SC2034
(
    set -eEu -x
    RUSTC_WRAPPER=""
    CARGO_TARGET_DIR="target/coverage"
    CARGO_INCREMENTAL=0
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort -Zmacro-backtrace"
    cargo +"$NIGHTLY_VERSION" install grcov
    cargo +"$NIGHTLY_VERSION" --offline build --verbose --tests --all-targets --all-features
    if [ "$1" = 'run' ]; then
        cargo +"$NIGHTLY_VERSION" --offline test --all-targets --all-features
        mkdir -p '/coverage'
        find . -name '*.gc*'  #TODO @mark: TEMPORARY! REMOVE THIS!
        find /coverage  #TODO @mark: TEMPORARY! REMOVE THIS!
        grcov 'target/debug/' -s . -t html --llvm --branch --ignore-not-existing -o '/coverage'
        #cp -r 'target/debug/deps' '/coverage'
    fi
)
