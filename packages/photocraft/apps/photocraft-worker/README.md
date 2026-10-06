# PhotoCraft on Cloudflare Workers

This package runs the upstream PhotoCraft engine as WebAssembly without egui, a window, a DOM, or a GPU. It exposes the entire command registry; availability and validation come from the upstream engine. There are no adapter-specific request-size, image-dimension, command-count or command allowlist restrictions.

## Build and run

Install the root pinned Rust toolchain, `rustup target add wasm32-unknown-unknown`, Node 22+, and wasm-bindgen-cli **0.2.129**. Set `WASM_BINDGEN` to a matching private binary if another CLI version is installed.

```sh
cd packages/photocraft/apps/photocraft-worker
npm install
npm run build
npm test
# After configuring your own Cloudflare account:
npm run deploy
```

`build` performs a release WASM build and a Wrangler deployment dry run. `test` sends real HTTP requests into Miniflare/workerd. The same generated `pkg/` module also works in browser/server WASM hosts using wasm-bindgen's initialization API. Generated artifacts are ignored.

## API

- `GET /commands` returns upstream IDs, parameter descriptions and current enabled status for an empty session.
- `POST /run` creates a fresh session, optionally imports `{input:{name,base64}}`, executes `{commands:[{id,params}]}`, and optionally exports `{output:"png"}` (or another upstream export format). It returns command results, session state, import warnings and base64 output with export warnings.
- `POST /render` accepts the upstream `file.new` parameters plus `commands`; returns PNG bytes.
- `POST /convert?format=bmp` accepts encoded image bytes and returns encoded bytes through upstream codecs.

```json
{"commands":[{"id":"file.new","params":{"width":1200,"height":800,"background":"#ff0000","depth":16}},{"id":"image.adjustments.invert"}],"output":"psd"}
```

The `PhotoCraft` WASM class offers `commands`, `execute`, `inspect`, `open`, `export`, and `warnings`. Each HTTP call owns and frees its engine instance; no document state is shared across requests. A caller can retain the class for a longer local session.

## Coverage and real limits

The adapter links the upstream engine and its core dependency chain: geometry, color/CMS, raster, document/history, algorithms, painting, text/vector, CPU composition, PSD, codecs/raw, native format, import/export and plugins. This does not mean every function or command has been runtime-tested.

The actual workerd journey covers 764 registry entries, 8/16/32-bit image creation and invert, PSD export/re-import, undo, PNG/BMP round-trip, invalid input recovery, and requests beyond 512 pixels and 16 commands. `cargo build` verifies compiled dependencies; it does not establish feature parity with Photoshop or validate every registry command.

Upstream filesystem commands remain disabled on WASM. Use `open`/`export` or `/run` byte input/output. Desktop UI, device input, GPU acceleration and filesystem automation are not provided by this headless adapter. The desktop and browser UI applications are intentionally excluded from this repository. Upstream format/fidelity limitations and warnings remain applicable.

Workers enforce their own memory, CPU, upload and startup limits. The measured dry-run bundle was 20,394.54 KiB uncompressed / 8,118.42 KiB gzip on 2026-10-06; this needs a plan that accepts that upload size. Large workloads can use the same WASM module in a host with adequate resources. No deployment, large-document capacity benchmark, authentication policy, or all-command conformance is claimed by the local test. Put the endpoint behind your application's normal authentication when publishing it.
