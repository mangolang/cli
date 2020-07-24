#!/usr/bin/env bash

printf '== step: build ==\n'
source "${BASH_SOURCE%/*}/build.sh"

printf '== step: test ==\n'
source "${BASH_SOURCE%/*}/test.sh"

printf '== step: lint ==\n'
source "${BASH_SOURCE%/*}/lint.sh"

printf '== step: style ==\n'
source "${BASH_SOURCE%/*}/style.sh"

printf '== step: executable ==\n'
source "${BASH_SOURCE%/*}/executable.sh"

printf 'done\n'

