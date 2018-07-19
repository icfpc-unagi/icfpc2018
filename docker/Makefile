usage:
	@echo 'Usage: make (upload|build)'
.PHONY: usage

upload: unagi.pem
	rsync -e "../bin/gcloud-ssh" -a --delete . docker:docker
.PHONY: upload

unagi.pem: ../ssh/unagi.encrypted
	cd ../ssh && make unagi.pem
	cp ../ssh/unagi.pem ./unagi.pem

build-%: %.docker
	sudo docker build --file $*.docker \
	    --build-arg UNAGI_PASSWORD=$${UNAGI_PASSWORD} \
	    -t unagi2018/master:$* .
.PHONY: build-%
