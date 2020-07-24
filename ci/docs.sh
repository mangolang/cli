#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/shared.sh"

# Currently, this just builds the documentation, to see that it works.
# It does not publish the documentation automatically.

(
    set -x
    docker run --rm 'mango_cli_build'\
        cargo --offline doc --no-deps --all-features
)
