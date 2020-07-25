#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

INFO_PTH="$RELEASE_PATH/executable-info.txt"

printf '\n== size ==\n' > "$INFO_PTH"
CHECK du -h 'target/debug/mango' | tee -a "$INFO_PTH"
CHECK du -h 'target/release/mango' | tee -a "$INFO_PTH"
printf '\n== crates ==\n' >> "$INFO_PTH"
CHECK cargo --offline bloat --release --crates --all-features --wide -n 50 | tee -a "$INFO_PTH"
printf '\n== functions ==\n' >> "$INFO_PTH"
CHECK cargo --offline bloat --release --all-features --wide -n 30 | tee -a "$INFO_PTH"
printf '\n== linking ==\n' >> "$INFO_PTH"
CHECK ldd 'target/release/mango' | tee -a "$INFO_PTH"
