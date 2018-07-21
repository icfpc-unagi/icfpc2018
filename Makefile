usage:
	@echo 'Usage: make (test)'
.PHONY: usage

test: appengine-test cargo-test sim-test
.PHONY: test

%-test:
	cd $* && make test
.PHONY: $*-test

cargo-test:
	cargo test
.PHONY: cargo-test

sim-test:
	cd sim && make test
.PHONY: sim-test
