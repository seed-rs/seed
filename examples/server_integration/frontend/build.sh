#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen ../../../target/wasm32-unknown-unknown/debug/frontend.wasm --no-modules --out-dir ./pkg --out-name package