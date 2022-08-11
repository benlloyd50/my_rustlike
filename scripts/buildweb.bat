cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\release\my_rustlike.wasm --out-dir recent_web_build --no-modules --no-typescript
