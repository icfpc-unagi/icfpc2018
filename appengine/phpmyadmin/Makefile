usage:
	@echo 'Usage: make (build|install|clean)'
.PHONY: usage

build: phpmyadmin.zip
	-rm -rf public_html
	unzip phpmyadmin.zip -x '__MACOSX/*' -d public_html
	make update
.PHONY: build

phpmyadmin.zip:
	curl -o phpmyadmin.zip https://storage.googleapis.com/icfpc-phpmyadmin.appspot.com/phpmyadmin.zip

secret:
	@echo 'env_variables:' > public_html/secret.yaml
	@echo "  UNAGI_PASSWORD: ${UNAGI_PASSWORD}" >> public_html/secret.yaml
.PHONY: secret

update:
	cp -Ra config/* public_html/
	make secret
.PHONY: update

install: update
	gcloud app deploy --project icfpc-phpmyadmin public_html/
.PHONY: install

clean:
	-rm phpmyadmin.zip
	-rm -rf public_html
.PHONY: clean
