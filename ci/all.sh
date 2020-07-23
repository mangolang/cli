#!/usr/bin/env bash

printf '== step: build =='
bash 'ci/build.sh'

printf '== step: test =='
bash 'ci/test.sh'

printf '== step: lint =='
bash 'ci/lint.sh'

printf '== step: style =='
bash 'ci/style.sh'

printf '== step: executable =='
bash 'ci/executable.sh'

