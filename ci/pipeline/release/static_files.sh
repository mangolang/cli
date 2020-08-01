#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -e
    cp "${BASH_SOURCE%/*}/../../../README.rst" "$RELEASE_PATH/README.rst"
    cp "${BASH_SOURCE%/*}/../../../LICENSE.txt" "$RELEASE_PATH/LICENSE.txt"
)

CHECK ls -ls '/release'  #TODO @mark: TEMPORARY! REMOVE THIS!
(set -e; pwd; ls -ls "${RELEASE_PATH}/")  #TODO @mark: TEMPORARY! REMOVE THIS!
