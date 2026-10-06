# ONNX inference in a Cloudflare Worker

CPU inference using the vendored Candle ONNX evaluator, compiled to WebAssembly.
The Worker accepts a self-contained ONNX model and named Safetensors inputs and
returns named Safetensors outputs. No model weights are included or downloaded.
No GPU, training, model server, credentials, or external model data resolution is
provided. Operator/dtype/shape support is exactly the vendored evaluator's subset;
unsupported operations return errors rather than fabricated results.

## Build and run

Requirements: Node.js 22+, npm dependencies from the package or repository lock, Rust toolchain
`1.97` (tested with 1.97.1), `wasm32-unknown-unknown`, and wasm-bindgen CLI **0.2.129**.
Install Rust/CLI prerequisites explicitly in your environment; scripts never change
global toolchains. Protobuf code generation uses the locked `protoc-bin-vendored`
package (or an explicit `PROTOC` override), not a system protoc dependency.

```sh
# From this package (standalone installation):
npm ci --workspaces=false
WASM_BINDGEN=/absolute/path/to/wasm-bindgen npm run build
npm test
npx wrangler dev
npm run deploy:dry-run
# Authenticated Cloudflare account required for actual deployment:
WASM_BINDGEN=/absolute/path/to/wasm-bindgen npm run deploy
```

The build uses `cargo build --locked --release -j 1`, defaults
`RUSTUP_TOOLCHAIN=1.97`, and honors an explicit toolchain override. It writes Rust
artifacts into `core/target` unless `CARGO_TARGET_DIR` is set (relative paths are
relative to this package), emits wasm-bindgen files in `pkg`, then bundles the
Worker and copies its Wasm into `dist`. `core/Cargo.lock` is committed. npm dependency
versions are pinned in package.json. The package-local package-lock.json supports
`npm ci --workspaces=false`; the repository lock also supports workspace installs.

## HTTP protocol

`POST /evaluate` accepts raw bytes:

1. Four-byte unsigned **little-endian** ONNX model length.
2. Exactly that many ONNX protobuf bytes.
3. Remaining bytes: a Safetensors file, keyed by the model's input names.

A 200 response is `application/octet-stream` containing a Safetensors file keyed
by output names. Errors returned by the evaluator and malformed request framing
produce HTTP 400 JSON `{ "error": "..." }`. Other methods on this route return 405;
unknown routes return 404. This format has no application-specific payload cap;
Cloudflare's platform memory, request-size, CPU and deployment limits still apply.
The entire model, inputs and outputs are held in memory per request.

`tests/fixtures.mjs` includes a tiny protobuf encoder and Safetensors encoder/decoder
for reproducible fixture generation without Python, ONNX downloads or pretrained
weights. `tests/http.test.mjs` launches actual workerd through Miniflare and uses
Node's HTTP fetch against its listening port, checking numeric outputs for Add,
MatMul, ReLU, Softmax and Conv plus malformed-model, malformed-Safetensors,
missing-input and unsupported-operator errors followed by valid inference in the
same isolate. Tests require a completed build.

## CPU portability and licensing

See [vendor/candle/UPSTREAM.md](vendor/candle/UPSTREAM.md). This distribution uses
sequential CPU kernels, including convolution, sorting, softmax, rotary embedding
and quantized iterator paths. Thread-count discovery always returns one, and
matrix multiplication selects GEMM's non-Rayon execution path. Native builds of
this vendored distribution are sequential as well. CPU implementations remain
present; this is not a GPU or training distribution. The evaluator is not a full
ONNX implementation, and arbitrary malformed graphs are not guaranteed to avoid
all upstream Rust panics. Ordinary reported model errors are tested for recovery.

Candle's MIT and Apache-2.0 license files are retained in `vendor/candle`.
