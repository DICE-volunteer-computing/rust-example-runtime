SHELL := /bin/zsh

PROJECT := $(shell basename "$$PWD")

all:
	@echo ''
	@echo "DICE Runtime Examples - Rust"
	@echo "Vale Tolpegin (valetolpegin@gmail.com)"
	@echo ''
	@echo "-----------------------"
	@echo ''
	@echo " Project: $(PROJECT)"

init:
	@rustup target add wasm32-wasi
	@cargo install twiggy

clean:
	@rm -rf target

build:
	@cargo build --release --target=wasm32-wasi
	@wasm-opt -O3 --strip-debug target/wasm32-wasi/release/$(PROJECT).wasm -o target/wasm32-wasi/release/$(PROJECT).wasm
	@tar -czf target/wasm32-wasi/release/$(PROJECT).zip target/wasm32-wasi/release/$(PROJECT).wasm

test:
	@wasmer run --dir data target/wasm32-wasi/release/$(PROJECT).wasm test-1.json

inspect:
	@echo ''
	@echo 'Top'
	@echo "-----------------------"
	@echo ''
	@twiggy top target/wasm32-wasi/release/$(PROJECT).wasm
	@echo ''
	@echo ''
	@echo 'Garbage'
	@echo "-----------------------"
	@echo ''
	@twiggy garbage target/wasm32-wasi/release/$(PROJECT).wasm
	@echo ''