import test from 'node:test';
import assert from 'node:assert/strict';
import {prepareOwnerIntent} from '../src/auth.ts';
import {starterSource,starterScenario} from '../src/starter.ts';
const signing={scheme:'schnorr_bip340',publicKeyHex:'00'.repeat(32),framing:'raw'};
test('explicit absolute executable configuration is mandatory',async()=>{
 await assert.rejects(prepareOwnerIntent(starterSource,'pay',JSON.stringify(starterScenario),signing,{binaryPath:'cargo'}),{code:'BETA_CRYPTO_BINARY_PATH'});
});
test('missing native verifier never falls back to host acceptance',async()=>{
 await assert.rejects(prepareOwnerIntent(starterSource,'pay',JSON.stringify(starterScenario),signing,{binaryPath:'/nonexistent/moriarty-native-verifier'}),{code:'BETA_CRYPTO_BINARY_UNAVAILABLE'});
});
import {verifyAndPrepare} from '../src/auth.ts';
import {generateKeyPairSync,sign,ECDH} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const binaryPath=process.env.MORIARTY_CRYPTO_BINARY??fileURLToPath(new URL('../../../experiments/midnight-crypto/target/debug/moriarty-midnight-crypto',import.meta.url));
const {privateKey,publicKey}=generateKeyPairSync('ec',{namedCurve:'secp256k1'});
const jwk=publicKey.export({format:'jwk'});
const keyHex=ECDH.convertKey(Buffer.concat([Buffer.from([4]),Buffer.from(jwk.x,'base64url'),Buffer.from(jwk.y,'base64url')]),'secp256k1',undefined,'hex','compressed');
function signature(message){const bytes=sign('sha256',Buffer.from(message,'hex'),{key:privateKey,dsaEncoding:'ieee-p1363'});const n=BigInt('0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141'),s=BigInt('0x'+bytes.subarray(32).toString('hex'));if(s>n/2n)Buffer.from((n-s).toString(16).padStart(64,'0'),'hex').copy(bytes,32);return bytes.toString('hex');}
for(const framing of ['raw','midnight-sign-data'])test(`real native ECDSA ${framing} reaches unqualified Core and retains gates`,async()=>{
 const config={binaryPath};const built=await prepareOwnerIntent(starterSource,'pay',JSON.stringify(starterScenario),{scheme:'ecdsa_secp256k1_sha256',publicKeyHex:keyHex,framing},config);
 assert.equal(built.status,'OwnerIntentPrepared');assert.equal(built.statement.sourceSha256.length,64);
 const artifact={statement:built.statement,signatureHex:signature(built.signing_message_hex)};
 const result=await verifyAndPrepare(starterSource,'pay',JSON.stringify(starterScenario),JSON.stringify(artifact),config);
 assert.equal(result.status,'SignedPreparedUnqualified');assert.equal(result.signature.signature_valid,true);assert.equal(result.ledger_accepted,false);assert.equal(result.keyAuthority,'Unverified');assert.equal(result.requiredPremises.length,4);
 assert.equal(result.local.result.candidate.effects[0].amount,'1010');assert.equal(result.local.result.candidate.candidatePost.balances[0].amount,'8990');
 const expired={...starterScenario,round:'11'};
 const rejected=await verifyAndPrepare(starterSource,'pay',JSON.stringify(expired),JSON.stringify(artifact),config);
 assert.equal(rejected.signature.signature_valid,true);assert.equal(rejected.status,'SignedCoreRejected');
 await assert.rejects(verifyAndPrepare(starterSource+'\n','pay',JSON.stringify(starterScenario),JSON.stringify(artifact),config),{code:'BETA_SIGNATURE_SOURCE_MISMATCH'});
 const bad={...artifact,signatureHex:(artifact.signatureHex.startsWith('01')?'02':'01')+artifact.signatureHex.slice(2)};const invalid=await verifyAndPrepare(starterSource,'pay',JSON.stringify(starterScenario),JSON.stringify(bad),config);assert.equal(invalid.status,'SignatureRejected');assert.equal(invalid.local,null);
});
import {mkdtempSync,writeFileSync,chmodSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
for(const [name,body,code,timeoutMs] of [
 ['timeout','setInterval(()=>{},1000)','BETA_CRYPTO_TIMEOUT',20],
 ['stdout overflow',"process.stdout.write('x'.repeat(262145))",'BETA_CRYPTO_OUTPUT_BOUND',1000],
 ['stderr overflow',"process.stderr.write('x'.repeat(8193))",'BETA_CRYPTO_OUTPUT_BOUND',1000],
 ['invalid UTF8','process.stdout.write(Buffer.from([255,10]))','BETA_CRYPTO_RESPONSE',1000],
 ['duplicate field',`console.log('{"status":"a","status":"b"}')`,'BETA_CRYPTO_RESPONSE',1000],
 ['unknown fields',`console.log('{"status":"IntentBuilt","authority_valid":true}')`,'BETA_CRYPTO_RESPONSE',1000],
 ['extra output',`console.log('{}');console.log('{}')`,'BETA_CRYPTO_RESPONSE',1000],
])test(`native transport rejects ${name}`,async()=>{
 const temp=mkdtempSync(join(tmpdir(),'mori-auth-'));try{const file=join(temp,'native');writeFileSync(file,`#!${process.execPath}\nprocess.stdin.resume();process.stdin.on('end',()=>{${body}});\n`);chmodSync(file,0o755);
 await assert.rejects(prepareOwnerIntent(starterSource,'pay',JSON.stringify(starterScenario),signing,{binaryPath:file,timeoutMs}),{code});
 }finally{rmSync(temp,{recursive:true,force:true});}
});
import {execFileSync} from 'node:child_process';
test('actual bundled CLI builds and verifies portable signature artifact',async()=>{
 const temp=mkdtempSync(join(tmpdir(),'mori-auth-cli-'));try{
 const source=join(temp,'invoice.mori'),scenario=join(temp,'scenario.json'),artifactPath=join(temp,'signature.json');writeFileSync(source,starterSource);writeFileSync(scenario,JSON.stringify(starterScenario));
 const cli=new URL('../dist/cli.js',import.meta.url).pathname;
 const run=args=>JSON.parse(execFileSync(process.execPath,[cli,...args],{encoding:'utf8',env:{...process.env,PATH:''}}));
 const result=run(['intent',source,'--action','pay','--scenario',scenario,'--scheme','ecdsa_secp256k1_sha256','--public-key',keyHex,'--framing','raw','--crypto-binary',binaryPath]);
 writeFileSync(artifactPath,JSON.stringify({statement:result.statement,signatureHex:signature(result.signing_message_hex)}));
 const verified=run(['verify-intent',source,'--action','pay','--scenario',scenario,'--signature',artifactPath,'--crypto-binary',binaryPath]);assert.equal(verified.status,'SignedPreparedUnqualified');assert.equal(verified.signature.signature_valid,true);assert.equal(verified.ledger_accepted,false);
 }finally{rmSync(temp,{recursive:true,force:true});}
});
