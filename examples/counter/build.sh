#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/counter.wasm --no-modules --out-dir ./pkg --out-name package


#cargo build --target wasm32-unknown-unknown --release
#wasm-bindgen target/wasm32-unknown-unknown/release/counter.wasm --no-modules --out-dir ./pkg --out-name package