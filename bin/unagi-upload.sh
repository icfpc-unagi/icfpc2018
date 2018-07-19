#!/bin/bash

if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
	echo "$(pwd) is not inside a git directory." >&2
	exit 1
fi

git_directory="$(cd "$(pwd)/$(git rev-parse --show-cdup)" && pwd)"

if [ "$#" -eq 0 ]; then
	target="${USER}"
else
	target="$1"
	shift
fi

pushd "${git_directory}" >/dev/null
echo "Uploading '${git_directory}' to 'master:~/${target}'..."
exec -- rsync -e unagi-ssh -a --delete --exclude target "$@" ./ "master:~/${target}"
