# photocraft WASM

Raster editing, color, composition, painting, PSD, image codecs and byte import/export.

From the repository root:

```sh
npm run build -- photocraft
npm test -- photocraft
npm run bundle -- photocraft
npm run dev -- photocraft
npm run deploy -- photocraft
```

See the [adapter documentation](apps/photocraft-worker/README.md) for API details, request examples and tested workflows.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
