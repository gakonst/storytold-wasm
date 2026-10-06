#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/../../target-worker}"
bindgen="${WASM_BINDGEN:-wasm-bindgen}"
if [[ "$($bindgen --version)" != 'wasm-bindgen 0.2.129' ]]; then
  echo 'Set WASM_BINDGEN to a wasm-bindgen 0.2.129 binary' >&2
  exit 1
fi
cargo build --locked --manifest-path Cargo.toml --target wasm32-unknown-unknown --release
"$bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/artcraft_worker.wasm" --target web --out-dir pkg
node bundle.mjs
