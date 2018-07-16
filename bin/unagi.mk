update:
	git pull
	touch ../.pull
.PHONY: update

check:
	@if ! find ../.pull -mmin 5 2>/dev/null; then \
		make -f unagi.mk update >/dev/null; \
	fi
.PHONY: check
