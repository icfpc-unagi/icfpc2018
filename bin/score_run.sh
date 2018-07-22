#!/bin/bash
# score_run scores a run on the database.

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
DEFINE_bool dry_run false 'Enable dry run mode.'
DEFINE_int run_id 0 'Run ID to run.'
DEFINE_string simulator_binary '' 'Binary to run with.'
eval "${IMOSH_INIT}"

score_run="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)/score_run.php"

mkdir -p "$TMPDIR/score_run"
cd "$TMPDIR/score_run"

DRYRUN="${FLAGS_dry_run}" RUN_ID="${FLAGS_run_id}" \
SIMULATOR_BINARY="${FLAGS_simulator_binary}" \
	php "${score_run}"
