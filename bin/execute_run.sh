#!/bin/bash
# unagi-upload

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
DEFINE_bool dry_run false 'Dry run mode.'
DEFINE_int run_id 0 'Run ID to run.'
eval "${IMOSH_INIT}"

execute_run="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)/execute_run.php"

mkdir -p "$TMPDIR/execute_run"
cd "$TMPDIR/execute_run"
dry_run="${FLAGS_dry_run}" run_id="${FLAGS_run_id}" php "${execute_run}"
