#!/usr/bin/env bash

# Run tests using miri to detect more possible issues.

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK ./run_tests_with_miri.sh
