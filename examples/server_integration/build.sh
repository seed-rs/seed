#!/usr/bin/env bash

cd "$(dirname "$0")"/frontend

if [[ $1 == --release ]]; then
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen ./target/wasm32-unknown-unknown/release/frontend.wasm --no-modules --out-dir ../pkg --out-name package
else
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen ./target/wasm32-unknown-unknown/debug/frontend.wasm --no-modules --out-dir ../pkg --out-name package
fi
