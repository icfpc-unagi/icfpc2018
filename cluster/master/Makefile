usage:
	@echo 'Usage: make (install)'
.PHONY: usage

install-%: upload-%
	ssh -t ubuntu@$* sudo icfpc2018/cluster/master/install.sh
.PHONY: install-%

install: install-master.icfpc.sx9.jp
.PHONY: install

../../ssh/unagi.pem: ../../ssh/unagi.encrypted
	cd ../../ssh && make unagi.pem

upload-%: ../../ssh/unagi.pem
	rsync -a --delete ../.. ubuntu@$*:icfpc2018
.PHONY: upload-%
