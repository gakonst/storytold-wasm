import assert from 'node:assert/strict';
import {Miniflare} from 'miniflare';
import {fileURLToPath} from 'node:url';
const mf = new Miniflare({modules:true,scriptPath:fileURLToPath(new URL('../dist/worker.js',import.meta.url)),modulesRules:[{type:'CompiledWasm',include:['**/*.wasm'],fallthrough:true}],compatibilityDate:'2026-02-10'});
try {
 const get=path=>mf.dispatchFetch('http://artcraft.test'+path);
 const post=(path,body)=>mf.dispatchFetch('http://artcraft.test'+path,{method:'POST',body});
 let r=await get('/v1/capabilities'); assert.equal(r.status,200);assert((await r.json()).image_operations.includes('crop'));
 r=await get('/v1/catalog');assert.equal(r.status,200);assert(Object.keys(await r.json()).length>0);
 r=await post('/v1/utilities',JSON.stringify({op:'trim',text:'  hello  '}));assert.equal(await r.json(),'hello');
 r=await post('/v1/base64/decode','aGVsbG8=');assert.equal(await r.text(),'hello');
 r=await post('/v1/inspect','hello');let info=await r.json();assert.equal(info.bytes,5);assert.equal(info.blake3.length,64);
 const bmp=Buffer.alloc(70);bmp.write('BM');bmp.writeUInt32LE(70,2);bmp.writeUInt32LE(54,10);bmp.writeUInt32LE(40,14);bmp.writeInt32LE(2,18);bmp.writeInt32LE(2,22);bmp.writeUInt16LE(1,26);bmp.writeUInt16LE(24,28);bmp.fill(255,54);
 r=await post('/v1/images/png',bmp);assert.equal(r.status,200);const png=Buffer.from(await r.arrayBuffer());assert.equal(png.readUInt32BE(16),2);assert.equal(png.readUInt32BE(20),2);
 r=await post('/v1/images/thumbnail?dimension=1',png);assert.equal(r.status,200);const thumb=Buffer.from(await r.arrayBuffer());assert.equal(thumb.readUInt32BE(16),1);
 r=await post('/v1/images/info',png);assert.equal(r.status,200);assert.equal((await r.json()).width,2);
 assert.equal((await get('/v1/generation')).status,501);assert.equal((await post('/v1/images/png','bad')).status,400);
 console.log('PASS workerd capabilities, catalog, utilities, base64, hashes, BMP->PNG, thumbnail, image metadata, unavailable and invalid-input HTTP journeys');
} finally {await mf.dispose();}
