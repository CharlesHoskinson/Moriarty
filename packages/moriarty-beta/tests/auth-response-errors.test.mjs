import test from 'node:test';import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';import {mkdtempSync,writeFileSync,chmodSync,rmSync,readFileSync} from 'node:fs';import {join} from 'node:path';import {tmpdir} from 'node:os';import {fileURLToPath} from 'node:url';
import {prepareOwnerIntent} from '../src/auth.ts';import {starterSource,starterScenario} from '../src/starter.ts';
const cli=fileURLToPath(new URL('../src/cli.ts',import.meta.url)),fixture=fileURLToPath(new URL('../examples/signed-intent/transfer-schnorr-raw/',import.meta.url));
function badNative(dir,response){const binary=join(dir,'bad-native');writeFileSync(binary,`#!${process.execPath}\nprocess.stdin.resume();process.stdin.on('end',()=>process.stdout.write(Buffer.from('${Buffer.from(response).toString('hex')}','hex')));\n`);chmodSync(binary,0o755);return binary;}
test('malformed native JSON always reports a crypto response failure, never a caller JSON judgment',async()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-response-error-'));try{
  for(const response of ['{"status":"a","status":"b"}\n','{"amount":1}\n','{"value":"\\ud800"}\n','{"x":'+'['.repeat(40)+']'.repeat(40)+'}\n','{"x":"'+ 'a'.repeat(131073)+'"}\n']){
   const binary=badNative(dir,response);
   await assert.rejects(prepareOwnerIntent(starterSource,'pay',JSON.stringify(starterScenario),{scheme:'schnorr_bip340',publicKeyHex:'00'.repeat(32),framing:'raw'},{binaryPath:binary}),{code:'BETA_CRYPTO_RESPONSE'});
  }
 }finally{rmSync(dir,{recursive:true,force:true});}
});
for(const mode of ['--json','--review'])test(`malformed actual child response in ${mode} is exit2 and unknown signature validity`,()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-response-cli-'));try{
  const binary=badNative(dir,'{"status":"a","status":"b"}\n');
  const r=spawnSync(process.execPath,[cli,'verify-intent',join(fixture,'program.mori'),'--action','pay','--scenario',join(fixture,'scenario.json'),'--signature',join(fixture,'signature.json'),'--crypto-binary',binary,mode],{encoding:'utf8',env:{...process.env,PATH:''}});
  assert.equal(r.status,2,r.stdout+r.stderr);assert.match(r.stdout,/^[\x20-\x7e\n]*$/);
  if(mode==='--json'){const v=JSON.parse(r.stdout);assert.equal(v.diagnostics[0].code,'BETA_CRYPTO_RESPONSE');assert.equal(v.publishedEffects,null);assert.equal(v.publishedPost,null);assert.ok(!Object.hasOwn(v,'signature_valid'));}
  else{assert.match(r.stdout,/no judgment was made/);assert.match(r.stdout,/signature validity is unknown, not false/);assert.ok(!r.stdout.includes('Outcome: rejected.'));}
 }finally{rmSync(dir,{recursive:true,force:true});}
});
test('malformed caller signature JSON remains an input judgment with exit1',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-input-json-'));try{
  const artifact=join(dir,'artifact.json');writeFileSync(artifact,'{"statement":{},"statement":{},"signatureHex":"'+ '00'.repeat(64)+'"}');
  const r=spawnSync(process.execPath,[cli,'verify-intent',join(fixture,'program.mori'),'--action','pay','--scenario',join(fixture,'scenario.json'),'--signature',artifact,'--crypto-binary',join(dir,'never-invoked'),'--json'],{encoding:'utf8',env:{...process.env,PATH:''}});
  assert.equal(r.status,1,r.stdout+r.stderr);assert.equal(JSON.parse(r.stdout).diagnostics[0].code,'BETA_JSON_DUPLICATE');
 }finally{rmSync(dir,{recursive:true,force:true});}
});
