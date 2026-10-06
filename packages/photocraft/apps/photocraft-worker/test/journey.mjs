import {Miniflare} from 'miniflare';
import assert from 'node:assert/strict';
import {inflateSync} from 'node:zlib';
import path from 'node:path';
function png(bytes){
 const b=Buffer.from(bytes);assert.equal(b.subarray(1,4).toString(),'PNG');
 const width=b.readUInt32BE(16),height=b.readUInt32BE(20),depth=b[24],type=b[25];
 assert([2,6].includes(type));assert([8,16].includes(depth));
 const chunks=[]; for(let i=8;i<b.length;){const n=b.readUInt32BE(i);if(b.subarray(i+4,i+8).toString()==='IDAT')chunks.push(b.subarray(i+8,i+8+n));i+=12+n;}
 const data=inflateSync(Buffer.concat(chunks));assert(data[0]<=4);
 // At the first pixel all PNG predictor inputs are zero for every filter.
 const rgb=[0,1,2].map(i=>data[1+i*(depth/8)]);return {width,height,rgb};
}
const mf=new Miniflare({modules:true,scriptPath:path.resolve('dist/index.js'),modulesRules:[{type:'CompiledWasm',include:['**/*.wasm'],fallthrough:true}],compatibilityDate:'2026-10-06'});
const post=(route,value)=>mf.dispatchFetch('https://test'+route,{method:'POST',body:JSON.stringify(value)});
try{
 const catalogue=await(await mf.dispatchFetch('https://test/commands')).json();assert(catalogue.some(c=>c.id==='image.adjustments.invert'));
 const red={width:8,height:6,background:'#ff0000'};
 const response=await post('/render',red);assert.equal(response.status,200,await response.clone().text());
 const original=new Uint8Array(await response.arrayBuffer());assert.deepEqual(png(original),{width:8,height:6,rgb:[255,0,0]});
 for(const depth of [8,16,32]){const r=await post('/render',{...red,depth,commands:[{id:'image.adjustments.invert'}]});assert.equal(r.status,200,await r.clone().text());assert.deepEqual(png(await r.arrayBuffer()).rgb,[0,255,255]);}
 const first=await post('/run',{commands:[{id:'file.new',params:red},{id:'image.adjustments.invert'}],output:'psd'});assert.equal(first.status,200,await first.clone().text());
 const job=await first.json();assert.equal(Buffer.from(job.output.base64,'base64').subarray(0,4).toString(),'8BPS');
 const again=await post('/run',{input:{name:'test.psd',base64:job.output.base64},commands:[{id:'image.adjustments.invert'},{id:'edit.undo'}],output:'png'});assert.equal(again.status,200,await again.clone().text());
 const restored=await again.json();assert.deepEqual(png(Buffer.from(restored.output.base64,'base64')).rgb,[0,255,255]);assert(Array.isArray(restored.output.warnings));
 const converted=await mf.dispatchFetch('https://test/convert?format=bmp',{method:'POST',body:original});assert.equal(converted.status,200);
 const bmp=new Uint8Array(await converted.arrayBuffer());assert.deepEqual([...bmp.subarray(0,2)],[66,77]);
 const rt=await mf.dispatchFetch('https://test/convert?format=png',{method:'POST',body:bmp});assert.equal(rt.status,200);assert.deepEqual(png(await rt.arrayBuffer()),png(original));
 const unrestricted=await post('/render',{width:513,height:1,commands:Array.from({length:18},()=>({id:'image.adjustments.invert'}))});assert.equal(unrestricted.status,200,await unrestricted.clone().text());assert.equal(png(await unrestricted.arrayBuffer()).width,513);
 const bad=await post('/run',{commands:[{id:'not.a.command'}]});assert.equal(bad.status,400);assert.match((await bad.json()).error,/unknown command/);
 const corrupt=await mf.dispatchFetch('https://test/convert',{method:'POST',body:'bad'});assert.equal(corrupt.status,400);assert.equal((await post('/render',red)).status,200);
 console.log(JSON.stringify({ok:true,commands:catalogue.length,journeys:['8/16/32-bit invert pixels','PSD roundtrip','undo','PNG/BMP roundtrip','no artificial dimension/command cap','invalid-input recovery']}));
}finally{await mf.dispose();}
