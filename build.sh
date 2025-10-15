#!/bin/bash

set -e

echo "Building ChaCha20-WASM..."

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --target web --out-dir ./pkg-web --omit-imports ./target/wasm32-unknown-unknown/release/chacha_poly_wasm_web.wasm

wasm-bindgen --target deno --out-dir ./pkg-deno --omit-imports ./target/wasm32-unknown-unknown/release/chacha_poly_wasm_web.wasm

wasm-bindgen --target nodejs --out-dir ./pkg-nodejs --omit-imports ./target/wasm32-unknown-unknown/release/chacha_poly_wasm_web.wasm

wasm-bindgen --target bundler --out-dir ./pkg-bundler --omit-imports ./target/wasm32-unknown-unknown/release/chacha_poly_wasm_web.wasm

cp package.json pkg-bundler/package.json
cp README.md pkg-bundler/README.md
cp LICENSE pkg-bundler/LICENSE

# Optimize WASM
echo "⚡ Optimizing WASM..."

if command -v wasm-opt &> /dev/null; then

    wasm-opt -Oz pkg-web/chacha_poly_wasm_web_bg.wasm -o pkg-web/chacha_poly_wasm_web_bg.wasm
    wasm-opt -Oz pkg-deno/chacha_poly_wasm_web_bg.wasm -o pkg-deno/chacha_poly_wasm_web_bg.wasm
    wasm-opt -Oz pkg-nodejs/chacha_poly_wasm_web_bg.wasm -o pkg-nodejs/chacha_poly_wasm_web_bg.wasm
    wasm-opt -Oz pkg-bundler/chacha_poly_wasm_web_bg.wasm -o pkg-bundler/chacha_poly_wasm_web_bg.wasm

    echo "✅ WASM optimized"
else
    echo "⚠️  wasm-opt not found, skipping optimization"
fi
