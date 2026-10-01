import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,readFileSync,chmodSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {prepareOwnerIntent} from '../src/auth.ts';
import {starterSource,starterScenario} from '../src/starter.ts';

test('timeout rejects even when a descendant retains native stdout/stderr pipes',async()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-timeout-')),pidFile=join(dir,'pid'),binary=join(dir,'verifier');
 let watchdog;
 try{
  writeFileSync(binary,`#!${process.execPath}\nconst {spawn}=require('node:child_process');const fs=require('node:fs');const sub=spawn(process.execPath,['-e','setTimeout(()=>{},5000)'],{stdio:['ignore','inherit','inherit']});fs.writeFileSync(${JSON.stringify(pidFile)},String(sub.pid));sub.unref();process.stdin.resume();setInterval(()=>{},1000);\n`);chmodSync(binary,0o755);
  const start=Date.now();
  const result=await Promise.race([
   prepareOwnerIntent(starterSource,'pay',JSON.stringify(starterScenario),{scheme:'schnorr_bip340',publicKeyHex:'00'.repeat(32),framing:'raw'},{binaryPath:binary,timeoutMs:250}).then(()=>null,e=>e.code),
   new Promise(r=>{watchdog=setTimeout(()=>r('still pending'),1500);})
  ]);
  assert.equal(result,'BETA_CRYPTO_TIMEOUT');
  assert.ok(Date.now()-start<1400,'API must settle at its own bound');
 }finally{
  clearTimeout(watchdog);
  try{process.kill(Number(readFileSync(pidFile,'utf8')),'SIGKILL');}catch{}
  rmSync(dir,{recursive:true,force:true});
 }
});
