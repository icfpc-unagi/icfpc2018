#!/bin/bash

mount -t tmpfs tmpfs /home/unagi/icfpc2018-master

sudo --login --user unagi bash -c '
    cp -R /github/* ~/icfpc2018-master/;
    touch ~/icfpc2018-master/.pull;'

start_time="$(date +'%s')"
while :; do
    sudo --login --user unagi score_run --alsologtostderr
    sudo --login --user unagi kill -9 -1
    current_time="$(date +'%s')"
    if (( current_time > start_time + 60 )) ; then exit; fi
done
