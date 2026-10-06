export async function journey(call,a) {
 const p={xyz:[1,-2,3.5],rgba:[1,0,0.5,1]};
 const bytes=await call('encode',{points:[p,p]});a.equal(bytes.length,64);a.deepEqual(bytes.slice(0,4),[0,0,128,63]);
 a.deepEqual(await call('decode',{bytes}),[p,p]);
 const g=await call('generate',{count:4097,min:-2,max:5,seconds:4,seed:7});a.equal(g.points.length,4097);a.deepEqual(g.points[0].rgba,[1,0,0,1]);a.ok(g.points.every(p=>p.xyz.every(x=>x>=-2&&x<=5)));
 a.deepEqual(await call('decode',{bytes:g.bytes}),g.points);
 await call('decode',{bytes:[0]},400);await call('generate',{count:-1},400);
 a.equal((await call('generate',{count:1})).points.length,1);
}
