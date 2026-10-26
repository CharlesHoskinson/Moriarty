import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {mkdtempSync, writeFileSync, chmodSync, rmSync, readFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {once} from 'node:events';
import {createServer} from 'node:net';
import test from 'node:test';
const port = () => new Promise(ok => { const s=createServer();s.listen(0,'127.0.0.1',()=>{const p=s.address().port;s.close(()=>ok(p));}); });
const pause = ms => new Promise(ok=>setTimeout(ok,ms));
const active = pid => {try{return !['Z','X'].includes(readFileSync(`/proc/${pid}/stat`,'utf8').split(') ')[1].split(' ')[0]);}catch{return false;}};
async function setup() {
  const root=mkdtempSync(join(tmpdir(),'owned-prover-test-')),binary=join(root,'fake-server.mjs');
  writeFileSync(binary,`#!/usr/bin/env node
import {createServer} from 'node:http';
console.log('PRIVATE_WITNESS_TEST_SENTINEL');console.error('PRIVATE_WITNESS_TEST_SENTINEL');
if(process.env.MIDNIGHT_PROOF_SERVER_VERBOSE!=='false')process.exit(9);
const port=Number(process.argv[process.argv.indexOf('--port')+1]);
createServer((q,r)=>r.end(q.url==='/version'?'9.0.0-rc.5':String(process.pid))).listen(port,'127.0.0.1');
`);chmodSync(binary,0o755);
  const selected=await port();
  const child=spawn(process.execPath,[new URL('./owned-proof-server.mjs',import.meta.url).pathname,binary,String(selected),root],{detached:true,env:{...process.env,MIDNIGHT_PROOF_SERVER_VERBOSE:'true'},stdio:['pipe','pipe','pipe']});
  let logs='';child.stdout.on('data',b=>logs+=b);child.stderr.on('data',b=>logs+=b);
  const exited=once(child,'exit');
  const deadline=Date.now()+5000;
  while(!logs.includes('SHERLOCK_OWNED_PROVER_READY')){assert.ok(Date.now()<deadline,logs);await pause(20);}
  const pid=Number(await(await fetch(`http://127.0.0.1:${selected}/pid`)).text());
  return {root,child,exited,pid,logs:()=>logs};
}
test('normal control EOF reaps the prover before worker exit and suppresses all server logs',async()=>{
  const x=await setup();
  try{x.child.stdin.end('ok\n');const[code]=await x.exited;assert.equal(code,0,x.logs());assert.equal(active(x.pid),false);assert.ok(!x.logs().includes('PRIVATE_WITNESS_TEST_SENTINEL'));}
  finally{try{process.kill(-x.child.pid,'SIGKILL');}catch{}rmSync(x.root,{recursive:true,force:true});}
});
test('a failed wallet operation returns failure instead of training a successful budget',async()=>{
  const x=await setup();
  try{x.child.stdin.end('failed\n');const[code]=await x.exited;assert.equal(code,1,x.logs());assert.equal(active(x.pid),false);}
  finally{try{process.kill(-x.child.pid,'SIGKILL');}catch{}rmSync(x.root,{recursive:true,force:true});}
});
test('process-group cap kill also stops the actual prover',async()=>{
  const x=await setup();
  try{process.kill(-x.child.pid,'SIGKILL');await x.exited;for(let i=0;i<50&&active(x.pid);i++)await pause(10);assert.equal(active(x.pid),false);}
  finally{try{process.kill(-x.child.pid,'SIGKILL');}catch{}rmSync(x.root,{recursive:true,force:true});}
});
