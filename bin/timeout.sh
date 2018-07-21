#!/bin/bash
# unagi-upload

source imosh || exit 1
DEFINE_int timeout 1 'Time limit in seconds.'
eval "${IMOSH_INIT}"

"$@" &
child_pid="$!"

maybe_exit() {
	if ! kill -0 "${child_pid}" 2>/dev/null; then
		LOG INFO 'Stopping watch process...'
		sub::exit
	fi
}

{
	count=0
	while :; do
		maybe_exit
		sleep 0.1
		(( count += 3 ))
		if (( count > FLAGS_timeout * 10 )); then
			kill -XCPU "${child_pid}"
			for i in `seq 10`; do
				maybe_exit
				sleep 0.1
			done
			kill -9 "${child_pid}"
			sub::exit
		fi
	done
} &

wait
