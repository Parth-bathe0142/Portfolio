set dotenv-load := true

dev-css:
	npx @tailwindcss/cli -i input.css -o static/css/styles.css --watch

build-css:
	npx @tailwindcss/cli -i input.css -o static/css/styles.css --minify

watch:
	SPIN_VARIABLE_JWT_SECRET={{env('jwt_secret')}} spin watch

run:
    SPIN_VARIABLE_JWT_SECRET={{env('jwt_secret')}} spin up --build

deploy:
    spin cloud deploy --variable jwt_secret={{env('jwt_secret')}}
