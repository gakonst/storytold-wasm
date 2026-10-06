# filmcraft WASM

Timeline editing, CPU frame rendering, audio and portable media codecs.

From the repository root:

```sh
npm run build -- filmcraft
npm test -- filmcraft
npm run bundle -- filmcraft
npm run dev -- filmcraft
npm run deploy -- filmcraft
```

`GET /commands`; `POST /run` with `{commands:[{command,params}], files?:[{path,base64}], project?:".fcproj JSON"}`; `/render` returns a PNG and `/file?path=...` returns an exported file. Command parameters may reference prior results with `$ref`. Carry returned project/files into the next request; undo history is request-local. CPU codecs and rendering replace native device paths.

See the adapter tests in `apps/filmcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
