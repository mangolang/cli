#!/usr/bin/env bash

# Note: this must be the first step
printf '== step: build ==\n'
source "${BASH_SOURCE%/*}/pipeline/build.sh"

printf '== step: test ==\n'
source "${BASH_SOURCE%/*}/pipeline/test/test.sh"

printf '== step: lint ==\n'
source "${BASH_SOURCE%/*}/pipeline/test/lint.sh"

printf '== step: style ==\n'
source "${BASH_SOURCE%/*}/pipeline/test/style.sh"

printf '== step: test (miri) ==\n'
source "${BASH_SOURCE%/*}/pipeline/test/test_miri.sh"

printf '== step: dependencies - versions ==\n'
source "${BASH_SOURCE%/*}/pipeline/deps/versions.sh"

printf '== step: dependencies - audit ==\n'
source "${BASH_SOURCE%/*}/pipeline/deps/audit.sh"

printf '== step: dependencies - license ==\n'
source "${BASH_SOURCE%/*}/pipeline/deps/license.sh"

printf '== step: dependencies - unused ==\n'
source "${BASH_SOURCE%/*}/pipeline/deps/unused.sh"

printf '== step: documentation ==\n'
source "${BASH_SOURCE%/*}/pipeline/docs.sh"

printf '== step: release - dependencies ==\n'
source "${BASH_SOURCE%/*}/pipeline/release/dependencies.sh"

printf '== step: release - executable size ==\n'
source "${BASH_SOURCE%/*}/pipeline/release/exe_size.sh"

printf '== step: release - image (distributable) ==\n'
source "${BASH_SOURCE%/*}/pipeline/release/dist_image.sh"

printf '== cleanup ==\n'
# Untag the docker images so next run cannot accidentally rely on old versions.
docker rmi 'mangocode/mango_daily_base:latest'
docker rmi 'mango_ci:latest'
docker rmi 'mangocode/mango:latest'

printf '== done ==\n'
