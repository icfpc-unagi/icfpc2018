usage:
	@echo 'Usage: make (install)'
.PHONY: usage

build:
	make clean
	cp -Ra src public_html
	cp -Ra ../../data/problemsF public_html/problems
	@echo 'env_variables:' > public_html/secret.yaml
	@echo "  UNAGI_PASSWORD: ${UNAGI_PASSWORD}" >> public_html/secret.yaml
.PHONY: src

install: build test
	gcloud app deploy --project icfpc-dashboard public_html/
.PHONY: install

clean:
	rm -rf public_html || true
.PHONY: clean

debug:
	cd public_html && php -S 0.0.0.0:8888 --php-ini php.ini
.PHONY: debug

test: build
	bash run-test.sh
.PHONY: test
