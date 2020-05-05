#!/bin/bash
set -e

pushd math-bug
echo "Testing in rust"
cargo test

echo "Compiling wasm"
RUSTFLAGS='-C link-arg=-s' cargo +stable build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/math_bug.wasm ./res/
popd

cargo run --release
