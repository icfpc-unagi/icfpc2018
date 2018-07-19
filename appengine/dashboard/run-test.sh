#!/bin/bash

set -e -u -x
cd "$(dirname "${BASH_SOURCE}")/public_html"

for file in *.php; do
	php -l "$file"
done

php go.php >/dev/null
