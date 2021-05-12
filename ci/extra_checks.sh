#!/usr/bin/env sh

##
## This isn't very elegant, but it IS very easy.
##

if grep -Ern --include "*.rs" 'use crate::cli' src/daemon
then
    printf 'daemon must not depend on cli\n' 1>&2
    exit 1
fi

if grep -Ern --include "*.rs" 'use crate::daemon' src/cli
then
    printf 'cli must not depend on daemon\n' 1>&2
    exit 1
fi

bad_imports="$(grep -Ern --include "*.rs" '^(pub )?use ' src/ \
    | grep -Ev 'use (::|crate|self|super)')"
if [ -n "$bad_imports" ]
then
    printf '%s' "$bad_imports"
    printf 'crates must be imported with :: prefix\n' 1>&2
    exit 1
fi
