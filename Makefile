APP_NAME = basecoat
GIT_COMMIT = $(shell git rev-parse --short HEAD)
# Although go 1.18 has the git info baked into the binary now it still seems like there is no support
# For including outside variables except this. So keep it for now.
GO_LDFLAGS = '-X "github.com/clintjedwards/${APP_NAME}/internal/cli.appVersion=$(VERSION)" \
				-X "github.com/clintjedwards/${APP_NAME}/internal/api.appVersion=$(VERSION)"'
SHELL = /bin/bash
SEMVER = 0.0.1
VERSION = ${SEMVER}_${GIT_COMMIT}

## build: build a release version of application
build:
	npm run --prefix ./src/frontend build
	cargo build --release

## run: run development build
run:
	npm run --prefix ./src/frontend rebuild-css
	npm run --prefix ./src/frontend dev
	cargo run -- service start

## rebuild-css: rebuild the tailwind css file which provides css for other items.
rebuild-css:
	npm run --prefix ./src/frontend rebuild-css

## help: prints this help message
help:
	@echo "Usage: "
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
