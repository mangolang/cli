#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

# Compute code coverage. This needs a separate set of flags, so there is a dedicated script.

#TODO @mark: TEMPORARY! REMOVE THIS!
#mkdir -p "$RELEASE_PATH/coverage"
#docker run --rm --mount type=bind,src="$RELEASE_PATH/coverage",dst='/coverage' 'mango_ci:latest' \
#    bash 'cargo_for_coverage.sh' run
CHECK_NIGHTLY /bin/bash -s <<EOF
cargo --offline test --all-targets --all-features

EOF
