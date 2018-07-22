#!/bin/bash

mount -t tmpfs tmpfs /home/unagi/icfpc2018-master
chown unagi:unagi /home/unagi/icfpc2018-master

start_time="$(date +'%s')"
while :; do
    sudo --login --user unagi bash -c '
        rsync -a --delete /github/ ~/icfpc2018-master/;
        touch ~/icfpc2018-master/.pull;'
    sudo --login --user unagi execute_run --alsologtostderr
    sudo --login --user unagi kill -9 -1
    current_time="$(date +'%s')"
    if (( current_time > start_time + 60 )) ; then exit; fi
done
