#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target-worker}"
bindgen="${WASM_BINDGEN:-wasm-bindgen}"
[[ "$("$bindgen" --version)" == 'wasm-bindgen 0.2.129' ]] || { echo 'wasm-bindgen 0.2.129 required; set WASM_BINDGEN or install the exact CLI version.' >&2; exit 1; }
cargo build --locked --release --target wasm32-unknown-unknown -p vectorcraft-worker
"$bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/vectorcraft_worker.wasm" --target web --out-dir apps/vectorcraft-worker/pkg
cd apps/vectorcraft-worker
mkdir -p dist
# Generated module bootstrap only. Routing and all application logic live in Rust.
cat > dist/entry.js <<'JS'
import wasm from './vectorcraft_worker_bg.wasm';
import { initSync, worker_fetch } from '../pkg/vectorcraft_worker.js';
initSync({ module: wasm });
export default { fetch: worker_fetch };
JS
node build.mjs
cp pkg/vectorcraft_worker_bg.wasm dist/vectorcraft_worker_bg.wasm
