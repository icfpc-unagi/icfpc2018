usage:
	@echo 'Usage: make (test)'
.PHONY: usage

test: appengine-test cargo-test sim-test iwiwi-test
.PHONY: test

%-test:
	cd $* && make test
.PHONY: $*-test

cargo-test:
	cargo build --release
	cargo test --release
	./bin/chokudai --version=001 --run_postproc_binary=./target/release/run_postproc --simulate
	./bin/chokudai --version=005 --run_postproc_binary=./target/release/run_postproc --simulate
.PHONY: cargo-test

sim-test:
	cd sim && make test
.PHONY: sim-test

install: iwiwi sim
iwiwi:
	make -C iwiwi
sim:
	make -C sim
.PHONY: install iwiwi sim
