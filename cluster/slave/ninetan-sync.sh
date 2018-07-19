#!/bin/bash

set -e -u

while :; do
	rsync -a --delete /efs/dropbox/ /dropbox
	rsync -a --delete /efs/github/ /github
	sleep 10
done
