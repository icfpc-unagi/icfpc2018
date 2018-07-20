#!/bin/bash
# unagi-watch watches directories and prints a line if some event happens.

source imosh || exit 1
DEFINE_string watch_command "$(which fswatch)" 'Command to watch directories.'
DEFINE_int watch_fallback_interval 3 \
    'Interval to watch directories with ls command.'
DEFINE_int watch_timeout 30 'Timeout to watch.'
DEFINE_int watch_minimum_interval 3 'Minimum interval to trigger.'
eval "${IMOSH_INIT}"

if [ "$#" -eq 0 ]; then
    directories=('.')
else
    directories=("$@")
fi

keep_watching() {
    if [ "${FLAGS_watch_command}" != '' ]; then
        ${FLAGS_watch_command} "$(sub::implode ':' directories)"
    else
        last_hash=''
        while :; do
            current_hash="$(ls -lAR "${directories[@]}" | stream::md5)"
            if [ "${current_hash}" != "${last_hash}" ]; then
                echo "${current_hash}"
                last_hash="${current_hash}"
            fi
            sleep "${FLAGS_watch_fallback_interval}"
        done
    fi &
    while :; do
        echo 'optional'
        sleep 1
    done
}

last_timestamp=0
watch_timeout="${FLAGS_watch_timeout}"
keep_watching | while read line; do
    timestamp="$(date "+%s")"
    if [ "${line}" == 'optional' ] &&
       (( last_timestamp + watch_timeout > timestamp )); then
       continue
    fi
    if (( last_timestamp + FLAGS_watch_minimum_interval > timestamp )); then
        watch_timeout=0
        continue
    fi
    last_timestamp="$(date "+%s")"
    watch_timeout="${FLAGS_watch_timeout}"
    date '+%s'
done
