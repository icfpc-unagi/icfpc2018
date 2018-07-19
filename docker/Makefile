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
