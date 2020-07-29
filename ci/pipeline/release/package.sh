#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK_NIGHTLY rm -f "/release/"*".zip"
# shellcheck disable=SC2010
if ! ls -1qA . | grep -q . ; then
    printf 'did not find any files to release\n'  1>&2
    exit 1
fi
CHECK_NIGHTLY bash -c "cd /release; zip -r '${RELEASE_NAME}.zip' ."
cp "/${RELEASE_PATH}/${RELEASE_NAME}.zip" "./${RELEASE_NAME}.zip"
