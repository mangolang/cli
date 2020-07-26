#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK chmod 777 -R "/release"
(
    set -x
    rm -f "./${RELEASE_NAME}.zip"
    zip -rj "./${RELEASE_NAME}.zip" "$RELEASE_PATH"/*
)
