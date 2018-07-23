#!/bin/bash
# unagi-build

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
eval "${IMOSH_INIT}"

if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
	LOG FATAL "$(pwd) is not inside a git directory." >&2
fi

if [ "$#" != 2 ]; then
	LOG FATAL "Target (e.g. destroy_iwiwi) and binary name (e.g., iwiwi-001) must be specified."
fi

if [ "$(uname)" != 'Darwin' ]; then
	LOG FATAL "This program must be run on Mac."
fi

target="$1"
name="$2"

bin_directory="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)"
git_directory="$(cd "$(pwd)/$(git rev-parse --show-cdup)" && pwd)"

set -x
pushd "${git_directory}" >/dev/null
cargo build --release --package wata --bin "${target}"
cp "./target/release/${target}" "./bin/solvers/${name}.Darwin"
unagi-upload
unagi-ssh master "bash --login -c 'cd ${USER}; cargo build --release --package wata --bin ${target}'"
scp -S unagi-ssh master:${USER}/target/release/${target} "./bin/solvers/${name}.Linux"
pwd
cp "./bin/solvers/destroy_iwiwi" "./bin/solvers/${name}"
chmod +x "./bin/solvers/${name}"{,.Darwin,.Linux}
set +x
