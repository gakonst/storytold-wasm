# printcraft WASM

PDF editing, rendering, forms, encryption, signing and document export.

From the repository root:

```sh
npm run build -- printcraft
npm test -- printcraft
npm run bundle -- printcraft
npm run dev -- printcraft
npm run deploy -- printcraft
```

`GET /tools`; `POST /run` with `{files?:[{path,base64}], calls:[{name,arguments}]}` (see `test/journey.mjs` for tool-call examples); `POST /render?page=1&dpi=96` accepts PDF bytes and returns PNG bytes. Memory-backed files support save/reopen and operations on multiple documents. Cryptographic signing uses request-supplied identities; no identities or credentials are bundled.

See the adapter tests in `apps/printcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
