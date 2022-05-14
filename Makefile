#NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
#npx tailwindcss -i ./tailwind.css -o ./public/build/tailwind.css --watch"


autobuild-js:
	cd ./src/frontend && trunk watch

autobuild-css:
	cd ./src/frontend && tailwindcss -i ./tailwind.css -o ./tailwind-generated.css --watch

build-css:
	cd ./src/frontend && NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind-generated.css --minify

build-js:
	cd ./src/frontend && trunk build

start-service:
	cargo run -- service start

build: build-css build-js
	cargo build --release

## run: TODO(clintjedwards):
run: autobuild-js autobuild-css start-service


## help: prints this help message
help:
	@echo "Usage: "
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
