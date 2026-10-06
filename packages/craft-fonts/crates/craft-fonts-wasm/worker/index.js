import wasm from '../pkg/craft_fonts_wasm_bg.wasm';
import {initSync,catalog,parse_manifest} from '../pkg/craft_fonts_wasm.js';
initSync({module:wasm});
export default {
 async fetch(request,env){
  const url=new URL(request.url);
  if(url.pathname==='/api/fonts' && request.method==='GET') return new Response(catalog(url.searchParams.get('script')||''),{headers:{'content-type':'application/json'}});
  if(url.pathname==='/api/manifest' && request.method==='POST') {
   try {return new Response(parse_manifest(await request.text()),{headers:{'content-type':'application/json'}});}
   catch(error){return Response.json({error:String(error)},{status:400});}
  }
  return env.ASSETS.fetch(request);
 }
};
