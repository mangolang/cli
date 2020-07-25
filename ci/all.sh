#!/usr/bin/env bash

printf '== step: build ==\n'
source "${BASH_SOURCE%/*}/pipeline/build.sh"

printf '== step: test ==\n'
source "${BASH_SOURCE%/*}/pipeline/test.sh"

printf '== step: lint ==\n'
source "${BASH_SOURCE%/*}/pipeline/lint.sh"

printf '== step: style ==\n'
source "${BASH_SOURCE%/*}/pipeline/style.sh"

printf '== step: test (miri) ==\n'
source "${BASH_SOURCE%/*}/pipeline/test_miri.sh"

printf '== step: dependencies ==\n'
source "${BASH_SOURCE%/*}/pipeline/deps.sh"

printf '== step: documentation ==\n'
source "${BASH_SOURCE%/*}/pipeline/docs.sh"

#TODO @mark: add a performance step? https://github.com/mverleg/rust_template/blob/master/ci/check_performance.sh

printf '== step: executable ==\n'
source "${BASH_SOURCE%/*}/pipeline/dist_image.sh"

printf '== done ==\n'

#TODO @mark: integration tests
#TODO @mark: code coverage
#TODO @mark: source code and git hash in artifact

