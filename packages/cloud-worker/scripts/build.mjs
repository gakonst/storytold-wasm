import {execFileSync} from 'node:child_process';
import {mkdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
process.chdir(fileURLToPath(new URL('..',import.meta.url)));
const run=(c,a)=>execFileSync(c,a,{stdio:'inherit'});
run('cargo',['build','--locked','--release','--target','wasm32-unknown-unknown','--manifest-path','core/Cargo.toml','-j','1']);
mkdirSync('pkg',{recursive:true});
run(process.env.WASM_BINDGEN || 'wasm-bindgen',['--target','web','--out-dir','pkg','--out-name','core','core/target/wasm32-unknown-unknown/release/cloud_worker_core.wasm']);
