usage:
	@echo 'Usage: make (test)'
.PHONY: usage

test: appengine-test cargo-test bazel-test
.PHONY: test

%-test:
	cd $* && make test
.PHONY: $*-test

cargo-test:
	cargo test
.PHONY: cargo-test

bazel-test:
	bazel test ...
.PHONY: bazel-test
