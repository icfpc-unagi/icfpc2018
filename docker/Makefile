usage:
	@echo 'Usage: make (upload|build)'
.PHONY: usage

upload: data/unagi.pem data/unagi.pub
	rsync -e "../bin/gcloud-ssh" -a --delete . docker:docker
.PHONY: upload

data/unagi.pem: ../ssh/unagi.encrypted
	mkdir -p data
	cd ../ssh && make unagi.pem
	cp ../ssh/unagi.pem ./data/unagi.pem

data/unagi.pub: ../ssh/unagi.pub
	mkdir -p data
	cp ../ssh/unagi.pub ./data/unagi.pub

build-%: %.docker
	sudo docker build --file $*.docker \
	    --build-arg UNAGI_PASSWORD=$${UNAGI_PASSWORD} \
	    -t unagi2018/master:$* .
.PHONY: build-%

build: build-master
.PHONY: build

sshd-%:
	sudo docker run --init --name $* --interactive --tty --rm \
		--publish 2200:22 unagi2018/master:$* /usr/sbin/sshd -D
.PHONY: sshd-%

sshd: sshd-master
.PHONY: sshd

push-%:
	sudo docker push unagi2018/master:$*
.PHONY: push-%

push: push-master
.PHONY: push
