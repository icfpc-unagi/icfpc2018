#!/bin/bash

run() {
	sudo docker run --rm \
		-v /efs:/efs -v /github:/github:ro -v /dropbox:/dropbox:ro \
		unagi2018/master:master \
		sudo --login --user=unagi bash -c "
			rm -rf ~/icfpc2018-master;
			cp -Ra /github ~/icfpc2018-master;
			touch ~/icfpc2018-master/.pull;
			$1"
}

start_time="$(date +'%s')"
while :; do
	run 'execute_run --alsologtostderr'
	current_time="$(date +'%s')"
	if (( current_time > start_time + 60 )); then
		break
	fi
done
