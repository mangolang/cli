#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

printf '\n== crates ==\n' > "$RELEASE_PATH/executable_size.txt"
CHECK cargo --offline bloat --release --crates --all-features --wide -n 50 | tee -a "$RELEASE_PATH/executable_size.txt"
printf '\n== functions ==\n' >> "$RELEASE_PATH/executable_size.txt"
CHECK cargo --offline bloat --release --all-features --wide -n 30 | tee -a "$RELEASE_PATH/executable_size.txt"
