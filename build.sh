#!/bin/bash
rm ./dist -r
mkdir dist
#cargo +nightly build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/webgl_rust.wasm  --no-modules  --out-dir ./dist
