#!/usr/bin/env bash

set -eEu

function STEP() {
    if [ $# -lt 2 ]; then
        printf "STEP expects 2 arguments: name and script\n" 1>&2
        return 1
    fi
    pth="${BASH_SOURCE%/*}/pipeline/$1"
    if [ ! -f "$pth" ]; then
        printf "STEP script '%s' does not exist at '%s'\n" "$1" "$pth" 1>&2
        return 1
    fi
    printf '== step: %s (%s) ==\n' "$2" "$pth" 1>&2
    source "$pth"
}

# Note: this must be the first step
STEP 'make/base.sh' 'build - dependencies image'

STEP 'make/debug.sh' 'build - ci image'

STEP 'test/test.sh' 'test'

STEP 'test/lint.sh' 'lint'

STEP 'test/style.sh' 'style'

STEP 'test/test_miri.sh' 'test (miri)'

STEP 'test/cov.sh' 'coverage'

STEP 'deps/versions.sh' 'dependencies - versions'

STEP 'deps/audit.sh' 'dependencies - audit'

#STEP 'deps/license.sh' 'dependencies - license'

STEP 'deps/usage.sh' 'dependencies - unused'

STEP 'make/docs.sh' 'documentation'

STEP 'release/dist_image.sh' 'release - image (distributable)'

STEP 'release/dist_exe.sh' 'release - executable (linux)'

STEP 'release/dependencies.sh' 'release - dependencies'

STEP 'release/exe_info.sh' 'release - executable info'

STEP 'release/static_files.sh' 'release - readme, license, etc'


printf '== cleanup ==\n'
# Untag the docker images so next run cannot accidentally rely on old versions.
docker rmi 'mangocode/mango_daily_base:latest'
docker rmi 'mango_ci:latest'
docker rmi 'mangocode/mango:latest'

printf '== done ==\n'
