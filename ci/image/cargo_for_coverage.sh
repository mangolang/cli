#!/usr/bin/env bash

# Usage: ./cargo_for_coverage.sh test [args]
# This uses grcov and is described in
# https://github.com/mozilla/grcov#example-how-to-generate-gcda-files-for-a-rust-project

if [ $# -lt 1 ]; then
    printf "provide a cargo command as argument, e.g. '$0 test'\n" 1>&2
    exit 1
fi

(
    set -x
    CARGO_TARGET_DIR="target/nightly"
    CARGO_INCREMENTAL=0
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort -Zmacro-backtrace"
    RUSTDOCFLAGS="-Cpanic=abort"
    cargo +nightly --offline "$1" --all-targets --all-features "${@:2}"
)
