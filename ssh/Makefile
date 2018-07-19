unagi.pem: unagi.encrypted
	openssl enc -aes-256-cbc -base64 -pass "pass:$${UNAGI_PASSWORD}" \
	    -out $@ -in $^ -md md5 -d
	chmod 600 unagi.pem
