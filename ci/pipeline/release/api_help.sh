#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

sub_commands="$(CHECK_NIGHTLY bash -c "cargo run -- --help" | python -c "
from sys import stdin
lines = stdin.readlines()
# Find the start of 'subcommands' section
for i, line in enumerate(lines):
    if 'SUBCOMMANDS:' in line:
        subcommand_start = i + 1
        break
# Iterate over the subcommands, until empty line
for i in range(subcommand_start, len(lines)):
    line = lines[i]
    if len(line) >= 5 and line[5] != ' ':
        cmd = line.split()[0].strip()
        if cmd != 'help':
            print('{}'.format(cmd))
    if not line.strip():
        break
")"

CHECK_NIGHTLY bash -c "cargo run -- --help | tee /release/cli-help.txt && chmod 777 /release/cli-help.txt"

for sub_cmd in $sub_commands
do
    printf 'help for subcommand "%s"\n' "$sub_cmd"
    printf '\n\n== subcommand: %s ==\n\n' "$sub_cmd" >> "$RELEASE_PATH/cli-help.txt"
    CHECK_NIGHTLY bash -c "cargo run -q -- help $sub_cmd >> /release/cli-help.txt"
done
