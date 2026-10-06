import assert from 'node:assert/strict';
import { Miniflare } from 'miniflare';
import { fileURLToPath } from 'node:url';
import { mkdir, writeFile } from 'node:fs/promises';
import { journey } from './journey.mjs';
const root = fileURLToPath(new URL('.', import.meta.url));
const mf = new Miniflare({modules:true, scriptPath:root+'worker.mjs', modulesRoot:root,
  compatibilityDate:'2026-02-10', modulesRules:[{type:'CompiledWasm',include:['**/*.wasm'],fallthrough:true}]});
const transcript=[];
try {
  const base=await mf.ready;
  async function call(name, input, status=200) {
    const response=await fetch(new URL(name,base), {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(input)});
    const result=await response.json();
    transcript.push({operation:name, input, expectedStatus:status, observedStatus:response.status, result});
    assert.equal(response.status,status,JSON.stringify(result));
    return result;
  }
  await journey(call,assert);
  assert.equal((await fetch(new URL('unknown',base))).status,404);
  const operations=(await (await fetch(base)).json()).operations;
  assert.equal((await fetch(new URL(operations[0],base))).status,405);
  console.log('PASS real HTTP / workerd journey');
} finally {
  await mkdir(root+'output',{recursive:true});
  await writeFile(root+'output/e2e.json',JSON.stringify(transcript,null,2));
  await mf.dispose();
}
