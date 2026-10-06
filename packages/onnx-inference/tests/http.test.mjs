import {test} from 'node:test';
import assert from 'node:assert/strict';
import {Miniflare} from 'miniflare';
import {fileURLToPath} from 'node:url';
import {model,safetensors,decode,envelope} from './fixtures.mjs';
test('real workerd HTTP CPU inference and error recovery',async t=>{
  const mf=new Miniflare({modules:[{type:'ESModule',path:fileURLToPath(new URL('../dist/worker.mjs',import.meta.url))},{type:'CompiledWasm',path:fileURLToPath(new URL('../dist/core_bg.wasm',import.meta.url))}],compatibilityDate:'2026-02-10'});
  t.after(()=>mf.dispose());
  const url=await mf.ready;
  const post=body=>fetch(new URL('/evaluate',url),{method:'POST',body});
  async function check(op,tensors,expected,shape,names) {
    const r=await post(envelope(model(op,names || Object.keys(tensors),tensors,shape),safetensors(tensors)));
    assert.equal(r.status,200,await (r.status===200?Promise.resolve(''):r.text()));
    const y=decode(await r.arrayBuffer()).y;assert.deepEqual(y.shape,shape);assert.equal(y.dtype,'F32');
    y.values.forEach((v,i)=>assert.ok(Math.abs(v-expected[i])<1e-5,`${op}: ${v} != ${expected[i]}`));
    assert.equal(y.values.length,expected.length);
  }
  await check('Add',{x:{shape:[2],values:[1,-2]},b:{shape:[2],values:[3,5]}},[4,3],[2]);
  await check('MatMul',{x:{shape:[2,2],values:[1,2,3,4]},b:{shape:[2,2],values:[5,6,7,8]}},[19,22,43,50],[2,2]);
  await check('Relu',{x:{shape:[3],values:[-1,0,2]}},[0,0,2],[3],['x']);
  await check('Softmax',{x:{shape:[1,3],values:[0,0,0]}},[1/3,1/3,1/3],[1,3],['x']);
  await check('Conv',{x:{shape:[1,1,3,3],values:[1,2,3,4,5,6,7,8,9]},b:{shape:[1,1,2,2],values:[1,0,0,1]}},[6,8,12,14],[1,1,2,2]);
  for(const [label,body,pattern] of [
    ['framing',Buffer.from([0]),/Expected model length/],
    ['protobuf',envelope(Buffer.from([255]),safetensors({})),/./],
    ['safetensors',envelope(model('Add'),Buffer.from('bad')),/./],
    ['unsupported op',envelope(model('UnimplementedTestOperator',['x']),safetensors({x:{shape:[1],values:[1]}})),/unsupported op_type UnimplementedTestOperator/],
    ['missing input',envelope(model('Add'),safetensors({})),/missing input/]
  ]) {
    const r=await post(body);assert.equal(r.status,400);assert.match((await r.json()).error,pattern,label);
    await check('Add',{x:{shape:[1],values:[2]},b:{shape:[1],values:[5]}},[7],[1]);
  }
  assert.equal((await fetch(new URL('/evaluate',url))).status,405);
  assert.equal((await fetch(new URL('/other',url))).status,404);
});
