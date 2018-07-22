#!/bin/bash

sudo --login --user unagi bash -c '
    rm -rf ~/icfpc2018-master;
    cp -Ra /github ~/icfpc2018-master;
    touch ~/icfpc2018-master/.pull;'

start_time="$(date +'%s')"
while :; do
    sudo --login --user unagi execute_run --alsologtostderr
    sudo --login --user unagi kill -9 -1
    current_time="$(date +'%s')"
    if (( current_time > start_time + 60 )) ; then exit; fi
done
