import {initSync,build_graph,snap_length} from '../pkg/core.js';
import module from '../pkg/core_bg.wasm';
initSync({module});
export default {
 async fetch(request) {
  const path=new URL(request.url).pathname;
  if (!['/graph','/snap-length'].includes(path)) return Response.json({error:'Not found'},{status:404});
  if (request.method !== 'POST') return Response.json({error:'Use POST'},{status:405,headers:{Allow:'POST'}});
  try {
   const body=await request.json();
   if(path==='/graph') return Response.json(JSON.parse(build_graph(JSON.stringify(body))));
   if(typeof body.seconds!=='number') throw new Error('seconds must be a number');
   return Response.json({length:snap_length(body.seconds)});
  } catch(error) {return Response.json({error:String(error.message ?? error)},{status:400});}
 }
};
