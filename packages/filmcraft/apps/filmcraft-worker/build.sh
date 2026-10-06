#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target/worker-port}"
bindgen="${WASM_BINDGEN:-wasm-bindgen}"
if [[ "$("$bindgen" --version)" != "wasm-bindgen 0.2.129" ]]; then
  echo 'Set WASM_BINDGEN to the matching 0.2.129 executable (global CLI is not modified).' >&2
  exit 1
fi
cargo build --locked -p filmcraft-worker --target wasm32-unknown-unknown --release
mkdir -p apps/filmcraft-worker/generated
"$bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/filmcraft_worker.wasm" --target web --out-dir apps/filmcraft-worker/generated --out-name filmcraft_worker
