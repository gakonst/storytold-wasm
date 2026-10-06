import module from './output/core.wasm';
import { initialize } from './core.mjs';
initialize(module);
import { pointCloud, mesh } from './core.mjs';
const operations = { pointCloud, mesh };
export default {
  async fetch(request) {
    const name = new URL(request.url).pathname.slice(1);
    if (name === '' && request.method === 'GET') return Response.json({operations: Object.keys(operations)});
    if (!Object.hasOwn(operations, name)) return Response.json({error:'Unknown operation'}, {status:404});
    if (request.method !== 'POST') return new Response('Use POST', {status:405, headers:{Allow:'POST'}});
    try { return Response.json(await operations[name](await request.json())); }
    catch (error) { return Response.json({error:error.message}, {status:400}); }
  }
};
