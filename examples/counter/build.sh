cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/appname.wasm --no-modules --out-dir ./pkg