build: phpmyadmin.zip
	-rm -rf public_html
	unzip phpmyadmin.zip -x '__MACOSX/*' -d public_html

phpmyadmin.zip:
	curl -o phpmyadmin.zip https://storage.googleapis.com/icfpc-phpmyadmin.appspot.com/phpmyadmin.zip
