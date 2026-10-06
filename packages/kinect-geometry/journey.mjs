export async function journey(call,a) {
 const frame={xyz:[1000,-2000,3000,0,0,1000,0,0,1000,0,0,0],bgra:Array(4).fill([10,20,30,0]).flat(),width:2,height:2};
 const m=await call('mesh',frame);a.ok(Math.abs(m.positions[1]-2)<1e-6);a.deepEqual(m.colors.slice(0,4),[30,20,10,255]);a.deepEqual(m.indices,[0,1,2,0,0,0]);
 await call('mesh',{...frame,width:3},400);await call('pointCloud',{xyz:[1],bgra:[]},400);
 a.equal((await call('pointCloud',frame)).positions.length,12);
}
