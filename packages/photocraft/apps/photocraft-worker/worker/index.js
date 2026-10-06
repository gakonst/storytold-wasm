import wasm from '../pkg/photocraft_worker_bg.wasm';
import { initSync, PhotoCraft, convert } from '../pkg/photocraft_worker.js';
initSync({ module: wasm });
const types = {png:'image/png',jpg:'image/jpeg',jpeg:'image/jpeg',webp:'image/webp',tiff:'image/tiff',bmp:'image/bmp',psd:'image/vnd.adobe.photoshop'};
const decode64 = text => Uint8Array.from(atob(text), c => c.charCodeAt(0));
function encode64(bytes) {
  let result='';
  for(let i=0;i<bytes.length;i+=32768) result+=String.fromCharCode(...bytes.subarray(i,i+32768));
  return btoa(result);
}
export default {
 async fetch(request) {
  const url=new URL(request.url);
  try {
   if(request.method==='GET' && url.pathname==='/commands') {
    const engine=new PhotoCraft();
    try { return new Response(engine.commands(),{headers:{'content-type':'application/json'}}); }
    finally { engine.free(); }
   }
   if(!['/run','/render','/convert'].includes(url.pathname)) return new Response('Not found',{status:404});
   if(request.method!=='POST') return new Response('Method not allowed',{status:405,headers:{allow:'POST'}});
   if(url.pathname==='/convert') {
    const format=url.searchParams.get('format') || 'png';
    return new Response(convert(new Uint8Array(await request.arrayBuffer()),format),{headers:{'content-type':types[format]||'application/octet-stream'}});
   }
   const job=await request.json();
   if(!job || (job.commands!==undefined && !Array.isArray(job.commands))) throw new Error('commands must be an array');
   const engine=new PhotoCraft();
   try {
    const imported=job.input ? JSON.parse(engine.open(job.input.name||'input',decode64(job.input.base64))) : null;
    if(url.pathname==='/render') engine.execute('file.new',JSON.stringify(job));
    const results=(job.commands||[]).map(command=>{
     if(!command || typeof command.id!=='string') throw new Error('command.id must be a string');
     return JSON.parse(engine.execute(command.id,JSON.stringify(command.params??{})));
    });
    const state=JSON.parse(engine.inspect());
    if(url.pathname==='/render') {
     const bytes=engine.export('png');
     return new Response(bytes,{headers:{'content-type':'image/png','x-photocraft-warnings':engine.warnings()}});
    }
    const output=job.output ? {format:job.output,base64:encode64(engine.export(job.output)),warnings:JSON.parse(engine.warnings())} : null;
    return Response.json({results,state,imported,output});
   } finally { engine.free(); }
  } catch(error) { return Response.json({error:String(error?.message||error)},{status:400}); }
 }
};

