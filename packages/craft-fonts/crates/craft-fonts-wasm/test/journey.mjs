import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFile} from 'node:fs/promises';
import {Miniflare} from 'miniflare';
import path from 'node:path';
const origin=process.env.TEST_URL || 'https://fonts.test';
const mf=process.env.TEST_URL ? null : new Miniflare({
 modules:true, scriptPath:path.resolve('dist/index.js'),
 modulesRules:[{type:'CompiledWasm',include:['**/*.wasm'],fallthrough:true}],
 compatibilityDate:'2026-10-06',
 assets:{directory:path.resolve('assets'),binding:'ASSETS',routerConfig:{invoke_user_worker_ahead_of_assets:true,has_user_worker:true},assetConfig:{html_handling:'none',not_found_handling:'none'}}
});
const fetch=(url,init)=>mf ? mf.dispatchFetch(url,init) : globalThis.fetch(url,init);
try {
console.log('Starting font Worker integration');
const manifest=await readFile(new URL('../../../fonts/manifest.txt',import.meta.url),'utf8');
const response=await fetch(origin+'/api/fonts');assert.equal(response.status,200,await response.clone().text());
const fonts=await response.json();console.log('Catalogue loaded');assert(fonts.length>0);
const parsed=await fetch(origin+'/api/manifest',{method:'POST',body:manifest});assert.equal(parsed.status,200);assert.deepEqual(await parsed.json(),fonts);
const scripts=[...new Set(fonts.flatMap(f=>f.scripts))];
for(const script of scripts){assert.deepEqual(await(await fetch(origin+'/api/fonts?script='+script)).json(),fonts.filter(f=>f.scripts.includes(script)));}
for(const f of fonts){
 const file=await fetch(origin+'/'+f.file);assert.equal(file.status,200,f.file);
 const bytes=Buffer.from(await file.arrayBuffer());assert.equal(createHash('sha256').update(bytes).digest('hex'),f.sha256,f.file);
 const licence=await fetch(origin+'/'+f.licence_file);assert.equal(licence.status,200);assert((await licence.text()).length>0);
}
const invalid=await fetch(origin+'/api/manifest',{method:'POST',body:'bad | manifest'});assert.equal(invalid.status,400);assert.match((await invalid.json()).error,/expected 8 fields/);
assert.equal((await fetch(origin+'/no-such-font.ttf')).status,404);
console.log(JSON.stringify({ok:true,fonts:fonts.length,scripts:scripts.length,checks:['WASM manifest parse/catalog/filter','all font bytes SHA256','all licence assets','invalid input/404']}));

} finally { await mf?.dispose(); }
