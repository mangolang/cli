#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

# Compute code coverage. Note that, unfortunately, code must be recompiled with different flags
# than would be used for normal testing.

(
    export CARGO_TARGET_DIR="target/nightly"
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
    export RUSTDOCFLAGS="-Cpanic=abort"
    set -x
    exit 1 #TODO @mark: this is all wrong, should run in the imge
    CHECK cargo --offline test --release --all-targets --all-features --all
)
