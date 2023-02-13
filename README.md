wasm-opt -O3 --strip-debug -o target/wasm32-wasi/release/wasmer-rust-bin-test-optimized.wasm 
target/wasm32-wasi/release/wasmer-rust-bin-test.wasm

cargo build --target=wasm32-wasi --release

cargo build --release

wasmer run target/wasm32-wasi/release/wasmer-rust-bin-test-optimized.wasm

./target/release/wasmer-rust-bin-test

rustup target add wasm32-wasi

------

wasm-opt (brew install binaryen)
wasmer

cargo add twiggy

# Rust Runtime Example

An example implemention of a DICE runtime utilizing the Rust programming language, demonstrating how to build and test an input artifact in Rust.

## Commands

### Clean

`make clean`

### Build

`make build`

### Test

`make test`

### Inspect

`make inspect`