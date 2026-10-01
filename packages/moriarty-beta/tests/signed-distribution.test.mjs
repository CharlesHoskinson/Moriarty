import test from 'node:test';
import assert from 'node:assert/strict';
import {spawnSync,execFileSync} from 'node:child_process';
import {mkdtempSync,rmSync,readFileSync,writeFileSync,copyFileSync,chmodSync,existsSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {fileURLToPath} from 'node:url';
const root=fileURLToPath(new URL('..',import.meta.url));
const binary=process.env.MORIARTY_CRYPTO_BINARY??fileURLToPath(new URL('../../../experiments/midnight-crypto/target/debug/moriarty-midnight-crypto',import.meta.url));
const skip=!existsSync(binary)&&!process.env.MORIARTY_REQUIRE_NATIVE;
test('packed signed fixtures verify with the copied actual Rust binary outside checkout and empty PATH',{skip},()=>{
 assert.ok(existsSync(binary),'Required real native verifier');
 const temp=mkdtempSync(join(tmpdir(),'mori-signed-packed-'));
 try{
  const receipt=JSON.parse(execFileSync('npm',['pack','--json','--pack-destination',temp],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}))[0];
  assert.ok(receipt.files.some(x=>x.path==='SIGNED-INTENT.md'));
  execFileSync('npm',['install','--prefix',temp,join(temp,receipt.filename),'--ignore-scripts','--no-audit','--no-fund'],{encoding:'utf8'});
  const pkg=join(temp,'node_modules/@moriarty-lang/beta'),cli=join(pkg,'dist/cli.js'),native=join(temp,'native-verifier');
  copyFileSync(binary,native);chmodSync(native,0o755);
  const run=args=>spawnSync(process.execPath,[cli,...args],{cwd:temp,encoding:'utf8',env:{...process.env,PATH:''},timeout:20000});
  const ready=(name,extra=[])=>{const d=join(pkg,'examples/signed-intent',name);return ['verify-intent',join(d,'program.mori'),'--action',name.startsWith('repay')?'repay_loan':'pay','--scenario',join(d,'scenario.json'),'--signature',join(d,'signature.json'),'--crypto-binary',native,...extra];};
  for(const name of ['transfer-schnorr-raw','transfer-ecdsa-wallet','repay-ecdsa-raw']){
   const r=run(ready(name,['--json']));assert.equal(r.status,0,r.stderr+r.stdout);assert.match(r.stdout,/^[\x20-\x7e\n]*$/);
   const v=JSON.parse(r.stdout),expected=JSON.parse(readFileSync(join(pkg,'examples/signed-intent',name,'expected.json'),'utf8'));
   assert.equal(v.status,'SignedPreparedUnqualified');assert.equal(v.signature.signature_valid,true);assert.equal(v.ledger_accepted,false);assert.equal(v.keyAuthority,'Unverified');
   assert.equal(v.requiredPremises.length,4);assert.equal(v.unverifiedBindings.length,4);
   assert.deepEqual(v.local.result.candidate.effects,expected.effects);assert.deepEqual(v.local.result.candidate.candidatePost,expected.post);
   const readable=run(ready(name,['--review']));assert.equal(readable.status,0,readable.stderr);assert.match(readable.stdout,/^[\x20-\x7e\n]*$/);assert.match(readable.stdout,/SignedPreparedUnqualified/);assert.match(readable.stdout,/Unverified|unverified/);
  }
  const args=ready('transfer-schnorr-raw');
  const source=args[1],scenario=args[args.indexOf('--scenario')+1],sig=args[args.indexOf('--signature')+1];
  const changed=join(temp,'changed.mori');writeFileSync(changed,readFileSync(source,'utf8')+'\n// edited source\n');
  const mismatch=run(args.map((x,i)=>i===1?changed:x));assert.equal(mismatch.status,1);assert.equal(JSON.parse(mismatch.stdout).diagnostics[0].code,'BETA_SIGNATURE_SOURCE_MISMATCH');
  const expired=join(temp,'expired.json');const s=JSON.parse(readFileSync(scenario,'utf8'));s.round='11';writeFileSync(expired,JSON.stringify(s));
  const late=run(args.map((x,i)=>i===args.indexOf('--scenario')+1?expired:x));assert.equal(late.status,1);const l=JSON.parse(late.stdout);assert.equal(l.status,'SignedCoreRejected');assert.equal(l.signature.signature_valid,true);
  const forged=join(temp,'forged.json');const a=JSON.parse(readFileSync(sig,'utf8'));a.signatureHex=a.signatureHex.slice(0,-1)+(a.signatureHex.endsWith('0')?'1':'0');writeFileSync(forged,JSON.stringify(a));
  const bad=run(args.map((x,i)=>i===args.indexOf('--signature')+1?forged:x));assert.equal(bad.status,1);assert.equal(JSON.parse(bad.stdout).status,'SignatureRejected');
  const unavailable=run(args.map((x,i)=>i===args.indexOf('--crypto-binary')+1?join(temp,'missing'):x));assert.equal(unavailable.status,2);assert.equal(JSON.parse(unavailable.stdout).diagnostics[0].code,'BETA_CRYPTO_BINARY_UNAVAILABLE');
 }finally{rmSync(temp,{recursive:true,force:true});}
});
