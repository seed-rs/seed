param([switch] $release)

cd "$PSScriptRoot/frontend""

if($release) {
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen ./target/wasm32-unknown-unknown/release/frontend.wasm --no-modules --out-dir ../pkg --out-name package
} else {
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen ./target/wasm32-unknown-unknown/debug/frontend.wasm --no-modules --out-dir ../pkg --out-name package
}
