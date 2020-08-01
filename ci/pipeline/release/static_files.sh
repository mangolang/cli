#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -e
    cp "${BASH_SOURCE%/*}/../../../README.rst" "$RELEASE_PATH/README.rst"
    cp "${BASH_SOURCE%/*}/../../../LICENSE.txt" "$RELEASE_PATH/LICENSE.txt"
)

CHECK ls -als '/release'  #TODO @mark: TEMPORARY! REMOVE THIS!
(set -e; pwd; ls -als "${RELEASE_PATH}/")  #TODO @mark: TEMPORARY! REMOVE THIS!
