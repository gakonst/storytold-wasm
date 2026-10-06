let wasm;
export function initialize(module) { wasm = new WebAssembly.Instance(module).exports; }
export async function load(url = new URL('./output/core.wasm', import.meta.url)) {
  initialize(await WebAssembly.compile(await (await fetch(url)).arrayBuffer()));
}
function ready() { if (!wasm) throw new Error('Call load() or initialize(module) first'); }
function number(value) { if (typeof value !== 'number' || !Number.isFinite(value)) throw new Error('Expected finite number'); return value; }
function vector(value, size=3) { if (!Array.isArray(value) || value.length !== size) throw new Error(`Expected ${size} coordinates`); return value.map(number); }

export function pointCloud({xyz,bgra}) {
 ready();if(!Array.isArray(xyz)||xyz.length%3||xyz.some(x=>!Number.isInteger(x)||x<-32768||x>32767))throw new Error('Expected signed int16 XYZ millimeters');
 if(!Array.isArray(bgra)||bgra.length!==xyz.length/3*4||bgra.some(x=>!Number.isInteger(x)||x<0||x>255))throw new Error('Expected aligned BGRA pixels');
 return {positions:xyz.map((x,i)=>wasm.meters(x,i%3===1)),colors:bgra.map((x,i)=>i%4===3?255:bgra[i-i%4+[2,1,0][i%4]])};
}
export function mesh({xyz,bgra,width,height,nearClip=0}) {
 const cloud=pointCloud({xyz,bgra});number(nearClip);
 if(!Number.isSafeInteger(width)||!Number.isSafeInteger(height)||width<1||height<1||width*height!==xyz.length/3)throw new Error('Invalid image dimensions');
 const indices=[],uv=[];
 for(let y=0;y<height;y++)for(let x=0;x<width;x++) {
  const p=y*width+x;uv.push(width===1?0:x/(width-1),height===1?0:y/(height-1));
  if(x+1<width&&y+1<height)for(const tri of [[p,p+1,p+width],[p+width,p+1,p+width+1]])indices.push(...(tri.every(i=>xyz[i*3+2]>nearClip)?tri:[0,0,0]));
 }
 return {...cloud,indices,uv};
}
