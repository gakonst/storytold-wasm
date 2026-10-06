#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target-worker}"
bindgen="${WASM_BINDGEN:-wasm-bindgen}"
[[ "$("$bindgen" --version)" == 'wasm-bindgen 0.2.129' ]] || { echo 'wasm-bindgen 0.2.129 required; set WASM_BINDGEN or install the exact CLI version.' >&2; exit 1; }
cargo build --locked --release --target wasm32-unknown-unknown -p designcraft-worker
"$bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/designcraft_worker.wasm" --target web --out-dir apps/designcraft-worker/pkg
node apps/designcraft-worker/build.cjs
