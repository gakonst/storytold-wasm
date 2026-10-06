#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target/worker}"
bindgen="${WASM_BINDGEN:-wasm-bindgen}"
"$bindgen" --version | grep -q 'wasm-bindgen 0.2.129' || { echo 'Need wasm-bindgen 0.2.129; set WASM_BINDGEN to a private matching binary' >&2; exit 1; }
cargo build --locked -p printcraft-worker --target wasm32-unknown-unknown --release
"$bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/printcraft_worker.wasm" --target web --out-dir apps/printcraft-worker/pkg
node apps/printcraft-worker/build.mjs
