# craft-fonts WASM

Font catalogue, manifest parser and licensed font assets.

From the repository root:

```sh
npm run build -- craft-fonts
npm test -- craft-fonts
npm run bundle -- craft-fonts
npm run dev -- craft-fonts
npm run deploy -- craft-fonts
```

See the [adapter documentation](crates/craft-fonts-wasm/README.md) for API details, request examples and tested workflows.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
