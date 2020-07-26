#!/usr/bin/env bash

python -c "
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
"
