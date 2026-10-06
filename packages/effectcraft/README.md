# effectcraft WASM

Composition, effects, expressions and CPU rendering.

From the repository root:

```sh
npm run build -- effectcraft
npm test -- effectcraft
npm run bundle -- effectcraft
npm run dev -- effectcraft
npm run deploy -- effectcraft
```

`GET /capabilities`; `POST /run` with `{commands:[{command,params}], files?:[{path,base64}], project?, render?}`; `POST /render` returns a PNG. Discover commands through `command.list` and `command.describe`. Results include state, project and output files; command errors preserve prior completed edits. Boa expressions/scripts and portable wasmi plugins use an isolated instance per request.

See the adapter tests in `apps/effectcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
