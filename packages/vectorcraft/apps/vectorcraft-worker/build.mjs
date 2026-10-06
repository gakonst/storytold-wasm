// Build tooling only: no application logic or dependency installation.
import {build} from 'esbuild';
await build({entryPoints:['dist/entry.js'],bundle:true,format:'esm',platform:'browser',target:'es2022',external:['*.wasm'],outfile:'dist/index.js'});
