#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

# Compute code coverage. This needs a separate set of flags, so there is a dedicated script.

CHECK find . -maxdepth 1
CHECK bash "cargo_for_coverage.sh" check
