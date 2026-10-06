import {initSync, evaluate} from '../pkg/core.js';
import wasm from '../pkg/core_bg.wasm';
let ready = false;
export default {
  async fetch(request) {
    const path = new URL(request.url).pathname;
    if (path !== '/evaluate') return new Response('Not found', {status:404});
    if (request.method !== 'POST') return new Response('Use POST', {status:405,headers:{Allow:'POST'}});
    try {
      const bytes = new Uint8Array(await request.arrayBuffer());
      if(bytes.length < 4) throw new Error('Expected model length, ONNX model, and Safetensors inputs');
      const modelLength = new DataView(bytes.buffer).getUint32(0,true);
      if(!modelLength || modelLength > bytes.length - 5) throw new Error('Invalid model length');
      if(!ready) { initSync({module:wasm}); ready=true; }
      const result = evaluate(bytes.subarray(4,4+modelLength),bytes.subarray(4+modelLength));
      return new Response(result,{headers:{'Content-Type':'application/octet-stream'}});
    } catch(error) {
      return Response.json({error:error.message || String(error)},{status:400});
    }
  }
};
