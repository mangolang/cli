#!/usr/bin/env bash

printf '== step: build ==\n'
source 'ci/build.sh'

printf '== step: test ==\n'
source 'ci/test.sh'

printf '== step: lint ==\n'
source 'ci/lint.sh'

printf '== step: style ==\n'
source 'ci/style.sh'

printf '== step: executable ==\n'
source 'ci/executable.sh'

printf 'done\n'

