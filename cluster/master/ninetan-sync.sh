#!/bin/bash

set -e -u

while :; do
	rsync -a --delete /home/ninetan/Dropbox/ICFPC2018/ /efs/dropbox
	rsync -a --delete /home/ninetan/github/ /efs/github
	sleep 10
done
