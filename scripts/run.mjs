import {readFileSync,existsSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import path from 'node:path';
const root=fileURLToPath(new URL('..',import.meta.url));
const components=JSON.parse(readFileSync(path.join(root,'components.json'),'utf8'));
const [action,...requested]=process.argv.slice(2);
if(!['build','test','bundle','dev','deploy'].includes(action)) throw new Error('Unknown action');
if(['dev','deploy'].includes(action)&&requested.length!==1) throw new Error(`Choose one component: npm run ${action} -- photocraft`);
const names=requested.length?requested:Object.keys(components);
for(const name of names) if(!Object.hasOwn(components,name)) throw new Error(`Unknown component ${name}; choose ${Object.keys(components).join(', ')}`);
const env={...process.env};
const localBindgen=path.join(root,'toolchain/bin/wasm-bindgen');
if(!env.WASM_BINDGEN&&existsSync(localBindgen)) env.WASM_BINDGEN=localBindgen;
const npm=process.platform==='win32'?'npm.cmd':'npm';
for(const name of names){
 const component=components[name];
 console.log(`\n${action}: ${name}`);
 if(action==='build'||action==='test') execFileSync(npm,['run',action,'--workspace',component.package],{cwd:root,env,stdio:'inherit'});
 else {
  if(action==='deploy') execFileSync(npm,['run','build','--workspace',component.package],{cwd:root,env,stdio:'inherit'});
  const args=action==='bundle'?['deploy','--dry-run','--outdir','dist-deploy']:[action];
  execFileSync(process.execPath,[path.join(root,'node_modules/wrangler/bin/wrangler.js'),...args],{cwd:path.join(root,component.path),env,stdio:'inherit'});
 }
}
