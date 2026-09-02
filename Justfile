dev-css:
	npx @tailwindcss/cli -i input.css -o static/css/styles.css --watch

build-css:
	npx @tailwindcss/cli -i input.css -o static/css/styles.css --minify