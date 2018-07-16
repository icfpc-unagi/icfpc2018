usage:
	@echo 'Usage: make (install)'
.PHONY: usage

secret:
	@echo 'env_variables:' > public_html/secret.yaml
	@echo "  UNAGI_PASSWORD: ${UNAGI_PASSWORD}" >> public_html/secret.yaml
.PHONY: secret

install: secret
	gcloud app deploy --project icfpc-api public_html/
.PHONY: install

clean:
	-rm public_html/secret.yaml
.PHONY: clean

debug:
	cd public_html && php -S 0.0.0.0:8888 --php-ini php.ini
.PHONY: debug
