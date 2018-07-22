#!/bin/bash
# chokudai

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
eval "${IMOSH_INIT}"

if [ "$#" -lt 1 ]; then
	LOG FATAL "Version must be given as a first argument."
fi

version="$1"
shift
mono "$(dirname "${BASH_SOURCE}")/chokudai-solver/${version}.exe" "$@"
