## autorebuild-css: start tailwind css with watch option
autorebuild-css:
	npm run --prefix ./src/frontend rebuild-css

## autorebuild-js: start esbuild with watch option
autorebuild-js:
	npm run --prefix ./src/frontend rebuild-js

## start-service: start basecoat service
start-service:
	protoc --js_out=import_style=commonjs:./src/frontend/src/ --grpc-web_out=import_style=commonjs,mode=grpcwebtext:./src/frontend/src/ -I ./src/proto/ src/proto/*.proto
	cargo run -- service start

## run: build dev mode with auto-rebuild options; ex: make run -j3
run: autorebuild-css autorebuild-js start-service

## release: build with production flags
release:
	npm run --prefix ./src/frontend release
	cargo build --release

## help: prints this help message
help:
	@echo "Usage: "
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
