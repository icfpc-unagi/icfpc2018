#!/bin/bash

ssh_directory="$(dirname "${BASH_SOURCE}")/../ssh"

if [ ! -f "${ssh_directory}/unagi.pem" ]; then
    pushd "${ssh_directory}" >/dev/null
    make unagi.pem
    popd >/dev/null
fi

if [ "$#" -eq 0 ]; then
	set -- master
fi

exec ssh -F "$(dirname "${BASH_SOURCE}")/unagi-ssh.config" \
    -i "${ssh_directory}/unagi.pem" "$@"
