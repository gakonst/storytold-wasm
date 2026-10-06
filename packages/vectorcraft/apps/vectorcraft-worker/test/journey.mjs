// Integration tests run the generated Worker in real workerd via Miniflare. No mocked engine.
import {Miniflare} from 'miniflare';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {inflateSync} from 'node:zlib';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const mf=new Miniflare({modules:true,scriptPath:path.join(root,'dist/index.js'),modulesRules:[{type:'CompiledWasm',include:['**/*.wasm'],fallthrough:true}],compatibilityDate:'2026-02-10'});
const cmd=(command,params={})=>({command,params});
const call=(method,params={})=>({method,params});
const post=(route,value)=>mf.dispatchFetch('https://test'+route,{method:'POST',body:JSON.stringify(value)});
async function ok(response){assert.equal(response.status,200,await response.clone().text());return response;}
async function run(steps,output,input){return (await ok(await post('/run',{steps,output,input}))).json();}
const bytes=out=>Buffer.from(out.dataBase64,'base64');
const input=(name,buffer)=>({name,dataBase64:Buffer.from(buffer).toString('base64')});
// Decode 8-bit RGB/RGBA PNG with all five filters to check actual rasterized pixels.
function png(buffer){
 const b=Buffer.from(buffer);assert.equal(b.subarray(1,4).toString(),'PNG');
 const width=b.readUInt32BE(16),height=b.readUInt32BE(20);assert.equal(b[24],8);assert([2,6].includes(b[25]));
 const channels=b[25]===6?4:3, stride=width*channels, chunks=[];
 for(let i=8;i<b.length;){const n=b.readUInt32BE(i);if(b.subarray(i+4,i+8).toString()==='IDAT')chunks.push(b.subarray(i+8,i+8+n));i+=12+n;}
 const raw=inflateSync(Buffer.concat(chunks)),pixels=Buffer.alloc(stride*height);
 const paeth=(a,b,c)=>{const p=a+b-c,pa=Math.abs(p-a),pb=Math.abs(p-b),pc=Math.abs(p-c);return pa<=pb&&pa<=pc?a:pb<=pc?b:c;};
 for(let y=0;y<height;y++)for(let x=0;x<stride;x++){
  const mode=raw[y*(stride+1)],a=x>=channels?pixels[y*stride+x-channels]:0,b=y?pixels[(y-1)*stride+x]:0,c=y&&x>=channels?pixels[(y-1)*stride+x-channels]:0;
  assert(mode<=4);pixels[y*stride+x]=(raw[y*(stride+1)+1+x]+[0,a,b,Math.floor((a+b)/2),paeth(a,b,c)][mode])&255;
 }
 return {width,height,rgb:(x,y)=>[...pixels.subarray(y*stride+x*channels,y*stride+x*channels+3)]};
}
const draw=[cmd('file.new',{width:64,height:48}),cmd('shape.rectangle',{x:0,y:0,width:64,height:48}),cmd('paint.setFill',{color:'#ff0000'}),cmd('paint.setStroke',{none:true})];
const journeys=[];
try{
 const catalogue=await(await ok(await mf.dispatchFetch('https://test/commands'))).json();
 assert(catalogue.length>400);for(const id of ['shape.rectangle','plugin.install','document.exportForScreens','text.create','command.batch'])assert(catalogue.some(c=>c.id===id),id);
 const formats=await(await ok(await mf.dispatchFetch('https://test/formats'))).json();assert(formats);
 const painted=await run(draw,{format:'png'}), raster=png(bytes(painted.output));
 assert.equal(raster.width,64);assert.equal(raster.height,48);assert.deepEqual(raster.rgb(30,20),[255,0,0]);
 const undo=await run([...draw,cmd('paint.setFill',{color:'#0000ff'}),cmd('edit.undo')],{format:'png'});assert.deepEqual(png(bytes(undo.output)).rgb(30,20),[255,0,0]);
 const redo=await run([...draw,cmd('paint.setFill',{color:'#0000ff'}),cmd('edit.undo'),cmd('edit.redo')],{format:'png'});assert.deepEqual(png(bytes(redo.output)).rgb(30,20),[0,0,255]);
 journeys.push('CPU pixels, paints, undo and redo');
 const native=await run([...draw,cmd('text.create',{x:4,y:30,text:'Hello 日本語',size:10})],{format:'vectorcraft'});
 const opened=await run([],{format:'svg'},input('art.vectorcraft',bytes(native.output)));assert.equal(opened.state.objects,native.state.objects);assert.match(bytes(opened.output).toString(),/<svg/);
 const pdf=await run([],{format:'pdf'},input('art.vectorcraft',bytes(native.output)));assert.equal(bytes(pdf.output).subarray(0,5).toString(),'%PDF-');
 const restored=await run([],null,input('art.pdf',bytes(pdf.output)));assert.equal(restored.state.artboards.length,1);
 journeys.push('native roundtrip, bundled Latin/Japanese text, SVG and PDF');
 for(const format of ['svg','svgz','eps','dxf','emf','wmf','png','jpg','gif','webp','tiff','bmp','tga','psd']){
  const exported=await run(draw,{format});assert(bytes(exported.output).length>20,format);assert(Array.isArray(exported.output.warnings),format);
  if(!['tga','psd'].includes(format)){
   const again=await run([],null,input('art.'+format,bytes(exported.output)));assert(again.state.objects>0,format);
  }
 }
 journeys.push('14 format exports and 12 format reimports');
 const binary=await ok(await mf.dispatchFetch('https://test/convert?name=upload.svg&format=png',{method:'POST',body:bytes(opened.output)}));assert.equal(binary.headers.get('content-type'),'image/png');assert.equal(png(await binary.arrayBuffer()).width,64);assert(Array.isArray(JSON.parse(decodeURIComponent(binary.headers.get('x-vectorcraft-warnings')))));
 journeys.push('binary HTTP conversion and warnings');
 const tools=await run([cmd('file.new',{width:64,height:48}),call('ui.tool.select',{tool:'rectangle'}),call('ui.pointer',{events:[{kind:'down',x:4,y:5},{kind:'drag',x:24,y:25},{kind:'up',x:24,y:25}]}),call('ui.key',{key:'Escape'})],{format:'png'});assert(tools.state.objects>1);
 journeys.push('headless tools and pointer gestures');
 const geometry=await run([cmd('file.new',{width:40,height:20}),cmd('shape.rectangle',{x:0,y:0,width:20,height:20}),cmd('shape.rectangle',{x:10,y:0,width:20,height:20}),cmd('select.all'),cmd('object.pathfinder.unite'),cmd('paint.setFill',{gradient:{stops:[{offset:0,color:'#ff0000'},{offset:1,color:'#0000ff'}]}}),cmd('paint.setStroke',{none:true}),cmd('effect.apply',{effect:'path.offsetPath',params:{offset:1}})],{format:'png'});
 const gp=png(bytes(geometry.output));assert(gp.rgb(4,10)[0]>gp.rgb(4,10)[2]);assert(gp.rgb(26,10)[2]>gp.rgb(26,10)[0]);
 journeys.push('curve booleans, gradients, live offset effect');
 const batch=await run([...draw,cmd('command.batch',{commands:[cmd('paint.setFill',{color:'#00ff00'}),cmd('paint.setStroke',{none:true})]}),cmd('edit.undo')],{format:'png'});assert.deepEqual(png(bytes(batch.output)).rgb(30,20),[255,0,0]);
 const screens=await run([...draw,cmd('document.exportForScreens',{zip:true,formats:[{format:'png',scale:1},{format:'svg'}]})]);assert.equal(bytes(screens.results.at(-1)).subarray(0,2).toString(),'PK');
 journeys.push('transactional batch, multi-format ZIP export');
 const large=await run([cmd('file.new',{width:4097,height:1}),...Array.from({length:257},()=>cmd('file.info'))],{format:'png'});assert.equal(png(bytes(large.output)).width,4097);assert.equal(large.results.length,258);
 const scale=await run([cmd('file.new',{width:2,height:1}),call('ui.render',{scale:17})]);assert.equal(scale.results[1].width,34);
 const largeSvg='<svg xmlns="http://www.w3.org/2000/svg" width="3" height="2"><!--'+'x'.repeat(2*1024*1024)+'--><rect width="3" height="2" fill="red"/></svg>';
 const uploaded=await ok(await mf.dispatchFetch('https://test/convert?name=large.svg&format=png',{method:'POST',body:largeSvg}));assert.deepEqual(png(await uploaded.arrayBuffer()).rgb(1,1),[255,0,0]);
 journeys.push('4097px, 257 commands, 17x render, >2MiB HTTP input without adapter limits');
 const plugin=await readFile(path.join(root,'../../crates/plugins/tests/fixtures/desaturate.wasm'));
 const plugged=await run([...draw,cmd('plugin.install',{dataBase64:plugin.toString('base64')}),cmd('plugin.list')]);
 const pluginId=plugged.results.at(-1).plugins[0].id;
 const filtered=await run([...draw,cmd('plugin.install',{dataBase64:plugin.toString('base64')}),cmd('plugin.run',{id:pluginId})],{format:'png'});const gray=png(bytes(filtered.output)).rgb(30,20);assert.equal(gray[0],gray[1]);assert.equal(gray[1],gray[2]);
 const clean=await run([cmd('plugin.list')]);assert.equal(clean.results[0].plugins.length,0);assert.equal(clean.state,null);
 journeys.push('wasmi plugin execution and request isolation');
 const rpc=[{jsonrpc:'2.0',id:1,method:'initialize',params:{protocolVersion:'2025-06-18',clientInfo:{name:'test',version:'1'},capabilities:{}}},{jsonrpc:'2.0',method:'notifications/initialized'},{jsonrpc:'2.0',id:2,method:'tools/list'},{jsonrpc:'2.0',id:3,method:'tools/call',params:{name:'run_command',arguments:{command:'file.new',params:{width:10,height:10}}}}];
 const mcp=await(await ok(await post('/mcp',rpc))).json();assert.equal(mcp.length,3);assert(mcp[1].result.tools.length>20);assert(!mcp[2].result.isError,JSON.stringify(mcp));
 journeys.push('real upstream MCP initialization, tools and command');
 for(const steps of [[cmd('not.a.command')],[...draw,cmd('document.open',{path:'/etc/passwd'})],[...draw,cmd('document.export',{format:'png',path:'/tmp/art.png'})],[...draw,call('ui.render',{scale:0})],[call('ui.inspect')],[cmd('plugin.install',{path:'/tmp/filter.wasm'})]]){
  const r=await post('/run',{steps});assert.equal(r.status,400);assert.equal(typeof (await r.json()).error,'string');
 }
 for(const body of ['{bad','null','{"steps":[{}]}']){const r=await mf.dispatchFetch('https://test/run',{method:'POST',body});assert.equal(r.status,400);}
 const corrupt=await mf.dispatchFetch('https://test/convert',{method:'POST',body:'garbage'});assert.equal(corrupt.status,400);
 assert.equal((await mf.dispatchFetch('https://test/nope')).status,404);const wrong=await mf.dispatchFetch('https://test/run');assert.equal(wrong.status,405);assert.equal(wrong.headers.get('allow'),'POST');
 const alive=await run(draw,{format:'png'});assert.deepEqual(png(bytes(alive.output)).rgb(30,20),[255,0,0]);
 const independent=await Promise.all([run(draw,{format:'png'}),run([cmd('file.new',{width:7,height:3})],{format:'png'})]);assert.equal(png(bytes(independent[0].output)).width,64);assert.equal(png(bytes(independent[1].output)).width,7);
 journeys.push('native-operation errors, malformed input, recovery, concurrent request isolation');
 console.log(JSON.stringify({ok:true,commands:catalogue.length,journeys},null,2));
} finally {await mf.dispose();}
