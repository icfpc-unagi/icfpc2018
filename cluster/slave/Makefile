usage:
	@echo 'Usage: make (install)'
.PHONY: usage

install-%: upload-%
	ssh -t ubuntu@$* sudo icfpc2018/cluster/slave/install.sh
.PHONY: install-%

upload-%:
	rsync -a --delete ../.. ubuntu@$*:icfpc2018
.PHONY: upload-%
