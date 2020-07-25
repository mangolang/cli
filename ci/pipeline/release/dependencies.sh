#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

# Show the dependency tree
CHECK cargo tree --all-features | tee 'target/dependency-tree.txt'

# Show the licenses of all the dependencies
CHECK cargo --offline deny list --format=Human --layout=Crate -t 0.95 | tee 'target/dependency-licenses.txt'

