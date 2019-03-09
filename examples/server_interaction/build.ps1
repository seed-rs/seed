cargo build --target wasm32-unknown-unknown
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/server_interaction.wasm --no-modules --out-dir ./pkg --out-name package

#cargo build --target wasm32-unknown-unknown --release
#wasm-bindgen ../../target/wasm32-unknown-unknown/release/server_interaction.wasm --no-modules --out-dir ./pkg --out-name package