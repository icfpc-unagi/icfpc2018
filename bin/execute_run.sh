#!/bin/bash
# unagi-upload

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
eval "${IMOSH_INIT}"

execute_run="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)/execute_run.php"

mkdir -p "$TMPDIR/execute_run"
cd "$TMPDIR/execute_run"
php "${execute_run}"
