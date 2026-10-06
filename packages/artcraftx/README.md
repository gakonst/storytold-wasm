# artcraftx WASM

Image conversion, thumbnails, hashes, base64, model metadata and data utilities.

From the repository root:

```sh
npm run build -- artcraftx
npm test -- artcraftx
npm run bundle -- artcraftx
npm run dev -- artcraftx
npm run deploy -- artcraftx
```

`GET /v1/capabilities`, `/v1/catalog`; `POST /v1/inspect`, `/v1/base64/decode`, `/v1/utilities`, `/v1/images/png`, `/v1/images/thumbnail?dimension=...`, `/v1/images/info`. Image endpoints accept raw bytes. These are the portable model/data/image utilities only. Provider authentication, video decoding and remote generation are excluded; unavailable generation returns HTTP 501.

See the adapter tests in `apps/artcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
