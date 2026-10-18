import test from 'node:test';import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';import {readFileSync,mkdtempSync,writeFileSync,rmSync} from 'node:fs';import {tmpdir} from 'node:os';import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {existsSync} from 'node:fs';
import {renderErrorReview} from '../src/intent-display.ts';
const binary=process.env.MORIARTY_CRYPTO_BINARY??fileURLToPath(new URL('../../../experiments/midnight-crypto/target/debug/moriarty-midnight-crypto',import.meta.url));
const skip=!existsSync(binary)&&!process.env.MORIARTY_REQUIRE_NATIVE;
const ascii=/^[\x20-\x7e\n]*$/;
test('public error renderer escapes supplementary rejection detail lines',()=>{
 for(const text of ['\x1b[31mINJECTED','\u202eFAKE','\nFAKE LINE','\tTAB']){
  const r=renderErrorReview({code:'BETA_SIGNATURE_SOURCE_MISMATCH',message:'Mismatch'},'judgment',[text]);assert.match(r,ascii);assert.ok(!r.includes(text));
 }
});
test('real native CLI escapes artifact keys in mismatch review while JSON preserves pointer',{skip},()=>{
 assert.ok(existsSync(binary),'Required native verifier');
 const root=fileURLToPath(new URL('../../..',import.meta.url)),fixture=join(root,'packages/moriarty-beta/examples/signed-intent/transfer-schnorr-raw'),temp=mkdtempSync(join(tmpdir(),'mori-hostile-review-'));
 try{for(const key of ['\x1b[31mINJECTED\u202e','\nFORGED SCREEN','\tKEY']){
  const a=JSON.parse(readFileSync(join(fixture,'signature.json'),'utf8'));a.statement[key]='x';const artifact=join(temp,'sig.json');writeFileSync(artifact,JSON.stringify(a));
  const args=[join(root,'packages/moriarty-beta/src/cli.ts'),'verify-intent',join(fixture,'program.mori'),'--action','pay','--scenario',join(fixture,'scenario.json'),'--signature',artifact,'--crypto-binary',binary];
  const review=spawnSync(process.execPath,[...args,'--review'],{encoding:'utf8',env:{...process.env,PATH:''}});assert.equal(review.status,1);assert.match(review.stdout,ascii);assert.ok(!review.stdout.includes(key));
  const json=spawnSync(process.execPath,[...args,'--json'],{encoding:'utf8',env:{...process.env,PATH:''}});assert.equal(json.status,1);assert.match(json.stdout,ascii);assert.equal(JSON.parse(json.stdout).diagnostics[0].pointer,'/statement/'+key);
 }}finally{rmSync(temp,{recursive:true,force:true});}
});
