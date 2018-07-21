#!/bin/bash
# score_run scores a run on the database.

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
eval "${IMOSH_INIT}"

score_run="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)/score_run.php"

mkdir -p "$TMPDIR/score_run"
cd "$TMPDIR/score_run"
php "${score_run}"
