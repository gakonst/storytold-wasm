# Craft Fonts WASM and Worker

This package exposes the original font catalogue, script selection and manifest parser through WASM, and serves every font and its licence through Workers Static Assets. It preserves upstream SHA-256 hashes and source attribution. No arbitrary HTTP size or count limits are introduced.

Install Rust with `wasm32-unknown-unknown`, Node 22+, and wasm-bindgen-cli 0.2.129 (or set `WASM_BINDGEN` to the matching executable).

```sh
cd packages/craft-fonts/crates/craft-fonts-wasm
npm install
npm run build
npm test
# After configuring your own Cloudflare account:
npm run deploy
```

`GET /api/fonts` returns the catalogue. `?script=Jpan` filters by ISO 15924 script. `POST /api/manifest` parses a supplied manifest. Original `fonts/...` paths serve binary fonts, manifest and licences. Unknown files return 404. Upstream parser errors return 400.

The WASM exports are `catalog(script)` and `parse_manifest(text)`. An optional Rust `embed` feature also exports `font_bytes(index)` for standalone hosts that want fonts embedded in the module; the default Worker uses static assets instead. Native directory/environment discovery and filesystem reads are replaced by the asset binding, not emulated as host filesystem access.

The HTTP integration journey runs against real Wrangler/workerd, compares parsed manifest and catalogue, checks every script filter, verifies every font's SHA-256, fetches every licence, and tests malformed input and 404s. The upstream snapshot contains four fonts and two scripts. Runtime/upload limits are those of the selected hosting platform; no production deployment is performed by the tests.
