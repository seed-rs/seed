cargo build --target wasm32-unknown-unknown
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/window_events.wasm --no-modules --out-dir ./pkg