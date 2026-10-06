import {test} from 'node:test';
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdirSync,copyFileSync,readFileSync} from 'node:fs';
import {build} from 'esbuild';
import {Miniflare} from 'miniflare';
import {fileURLToPath} from 'node:url';
const root=fileURLToPath(new URL('..',import.meta.url));
test('workerd HTTP graph and frame parity against original Python',async()=>{
 const dir=root+'/tests/.bundle'; mkdirSync(dir,{recursive:true});
 await build({entryPoints:[root+'/src/worker.js'],outfile:dir+'/worker.mjs',bundle:true,format:'esm',external:['*.wasm']});
 copyFileSync(root+'/pkg/core_bg.wasm',dir+'/core_bg.wasm');
 // Bundling leaves the original relative WASM import unchanged.
 let code=readFileSync(dir+'/worker.mjs','utf8').replace('../pkg/core_bg.wasm','./core_bg.wasm');
 const mf=new Miniflare({modules:[{type:'ESModule',path:dir+'/worker.mjs',contents:code},{type:'CompiledWasm',path:dir+'/core_bg.wasm'}],compatibilityDate:'2026-02-10',port:0});
 try {
 const url=await mf.ready;
 const post=async(path,body)=>fetch(new URL(path,url),{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});
 const cases=[];
 const base={model_file:'model.safetensors',prompt:'A lighthouse — 海',width:864,height:480,length:124,steps:20,seed:42};
 for(const task of ['t2v','i2v','ref2v']) for(const ref_count of [0,1,2,9,12]) for(const ref_downscale of [0,0.25]) cases.push({...base,task,ref_count,ref_downscale});
 cases.push({...base,task:'i2v',image:'custom.png',sampler:'euler',scheduler:'normal',filename_prefix:'output/custom'});
 const seconds=[-100,0,0.1,0.5,1,5,15,120,0.9375,1.0625,10000];
 const oracle=JSON.parse(execFileSync('python3',['-c',`import importlib.util,json,sys\nspec=importlib.util.spec_from_file_location('original',sys.argv[1]); m=importlib.util.module_from_spec(spec); spec.loader.exec_module(m)\ndata=json.load(sys.stdin); print(json.dumps({'graphs':[m.build_graph(**c) for c in data['cases']], 'lengths':[m.snap_length(s) for s in data['seconds']]}))`,root+'/tests/upstream/minimax_bench.py'],{input:JSON.stringify({cases,seconds}),encoding:'utf8'}));
 for(let i=0;i<cases.length;i++){const r=await post('/graph',cases[i]); assert.equal(r.status,200); assert.deepEqual(await r.json(),oracle.graphs[i]);}
 for(let i=0;i<seconds.length;i++){const r=await post('/snap-length',{seconds:seconds[i]});assert.equal(r.status,200);assert.deepEqual(await r.json(),{length:oracle.lengths[i]});}
 // No application prompt-size or reference-count caps.
 const big={...base,task:'ref2v',ref_downscale:0.25,prompt:'x'.repeat(1024*1024+1),images:Array.from({length:64},(_,i)=>`image-${i}.png`)};
 const r=await post('/graph',big);assert.equal(r.status,200);const graph=await r.json();assert.equal(graph['5'].inputs.prompt.length,big.prompt.length);assert.equal(graph['83'].inputs.image,'image-63.png');assert.equal(graph['scale_83'].class_type,'ImageScaleToTotalPixels');assert.deepEqual(graph['5'].inputs['ref_images.ref_image_63'],['scale_83',0]);
 assert.equal((await post('/graph',{...base,task:'bad'})).status,400);
 assert.equal((await post('/graph',{})).status,400);
 assert.equal((await post('/snap-length',{seconds:'5'})).status,400);
 assert.equal((await fetch(new URL('/graph',url))).status,405);
 assert.equal((await post('/unknown',{})).status,404);
 assert.equal((await fetch(new URL('/graph',url),{method:'POST',body:'{'})).status,400);
 console.log(`Verified ${cases.length} original graph comparisons, ${seconds.length} numerical cases, large body/64 references, and HTTP errors over ${url}`);
 }finally {await mf.dispose();}
});
