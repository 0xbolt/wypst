.PHONY: wasm
wasm:
	cd src/core && wasm-pack build --target web

.PHONY: wasm-dev
wasm-dev:
	cd src/core && wasm-pack build --target web --dev

.PHONY: build
build:
	node esbuild.config.mjs

.PHONY: build-test
build-test:
	cd test && webpack
	cp dist/wypst.wasm test/dist/wypst.wasm

.PHONY: all
all:
	make wasm
	make build
	make build-test
