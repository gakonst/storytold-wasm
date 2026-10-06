import {build} from 'esbuild';
import {copyFileSync, mkdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
process.chdir(fileURLToPath(new URL('..',import.meta.url)));
mkdirSync('dist',{recursive:true});
await build({entryPoints:['src/worker.js'],outfile:'dist/worker.mjs',bundle:true,format:'esm',platform:'browser',target:'es2022',plugins:[{name:'wasm',setup(b){b.onResolve({filter:/\.wasm$/},()=>({path:'./core_bg.wasm',external:true}));}}]});
copyFileSync('pkg/core_bg.wasm','dist/core_bg.wasm');
