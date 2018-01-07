all:
	make prepare
	make build_all

prepare:
	rm -r build | true
	mkdir build

build_all: build_core build_bridge

build_core:
	cargo build --release --target wasm32-unknown-unknown
	wasm-gc target/wasm32-unknown-unknown/release/particles.wasm build/particles.wasm

build_bridge:
	cd jsbridge && $(WEBPACK_COMMAND)
	cp jsbridge/bridge.js build/particles-bridge.js
	cp jsbridge/demo.html build/

.PHONY: prepare build_core
