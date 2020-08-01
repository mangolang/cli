#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK_NIGHTLY rm -f "/release/"*".zip"
# shellcheck disable=SC2010
if ! ls -1qA "${RELEASE_PATH}" | grep -q . ; then
    printf 'did not find any files to release in "%s"\n' "${RELEASE_PATH}"  1>&2
    exit 1
fi
CHECK_NIGHTLY bash -c "cd /release; ls -als; zip -r '${RELEASE_NAME}.zip' ."
mv "${RELEASE_PATH}/${RELEASE_NAME}.zip" "${RELEASE_PATH}/../${RELEASE_NAME}.zip"

CHECK ls -ls '/release'  #TODO @mark: TEMPORARY! REMOVE THIS!
(set -e; pwd; ls -ls "${RELEASE_PATH}/")  #TODO @mark: TEMPORARY! REMOVE THIS!
