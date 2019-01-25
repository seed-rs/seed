#!/usr/bin/env bash
mkdir -p pkg
cargo build --lib --target wasm32-unknown-unknown --no-default-features
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/websocket.wasm --no-modules --out-dir ./pkg
cargo build --bin server --features server
