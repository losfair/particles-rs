all:
	make prepare
	make build

prepare:
	rm -r build | true
	mkdir build

build: build_core build_bridge

build_core:
	cargo build --release --target wasm32-unknown-unknown
	wasm-gc target/wasm32-unknown-unknown/release/particles.wasm build/particles.wasm

build_bridge:
	cd jsbridge && webpack
	cp jsbridge/bridge.js jsbridge/demo.html build/

.PHONY: prepare build_core