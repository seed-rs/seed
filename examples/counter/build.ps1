cargo build --target wasm32-unknown-unknown
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/counter.wasm --no-modules --out-dir ./pkg