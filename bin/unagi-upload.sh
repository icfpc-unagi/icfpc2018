#!/bin/bash
# unagi-upload

source imosh || exit 1
DEFINE_bool --alias=v verbose false 'Enables verbose mode.'
eval "${IMOSH_INIT}"

if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
	LOG FATAL "$(pwd) is not inside a git directory." >&2
fi

bin_directory="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)"
git_directory="$(cd "$(pwd)/$(git rev-parse --show-cdup)" && pwd)"

if [ "$#" -eq 0 ]; then
	target="${USER}"
else
	target="$1"
	shift
fi

pushd "${git_directory}" >/dev/null
LOG INFO "Uploading '${git_directory}' to 'master:~/${target}'..."

rsync_options=(-e "${bin_directory}/unagi-ssh" -a --delete --exclude target/ "$@")
if (( FLAGS_verbose )); then
	rsync_options+=(-v)
fi

exec -- rsync "${rsync_options[@]}" ./ "master:~/${target}"
