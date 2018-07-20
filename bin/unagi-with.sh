#!/bin/bash
# unagi-with.

source imosh || exit 1
DEFINE_string name "${USER}" 'Directory name to upload.'
DEFINE_bool --alias=v verbose false 'Enables verbose mode.'
eval "${IMOSH_INIT}"

if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
    LOG FATAL "$(pwd) is not inside a git directory."
fi
current_directory="$(pwd)"
git_directory="$(cd "$(pwd)/$(git rev-parse --show-cdup)" && pwd)"

func::substr relative_directory "${current_directory}" "${#git_directory}"

func::getmypid PID

upload() {
    "$(dirname "${BASH_SOURCE}")/unagi-upload.sh" \
        --name="${FLAGS_name}" \
        --verbose="${FLAGS_verbose}"
}

upload

"$(dirname "${BASH_SOURCE}")/unagi-watch.sh" \
    --alsologtostderr="${FLAGS_alsologtostderr}" \
    --logtostderr="${FLAGS_logtostderr}" \
    "${git_directory}" | \
while read line; do
    if ! kill -0 "${PID}" 2>/dev/null; then
        LOG INFO "Exiting because the parent process ${PID} died..."
        sub::exit
    fi
    if [ "${line}" == '' ]; then
        continue
    fi
    LOG INFO "Uploading..."
    upload
    sleep 1
done &

export UNAGI_DIRECTORY="${FLAGS_name}${relative_directory}"
exec -- "$(dirname "${BASH_SOURCE}")/unagi-ssh" master -o SendEnv=UNAGI_DIRECTORY
