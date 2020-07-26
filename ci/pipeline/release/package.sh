#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK rm -f "/release/${RELEASE_NAME}.zip"
CHECK zip -rj "/release/${RELEASE_NAME}.zip" "/release"/*
cp "/${RELEASE_PATH}/${RELEASE_NAME}.zip" "./${RELEASE_NAME}.zip"
