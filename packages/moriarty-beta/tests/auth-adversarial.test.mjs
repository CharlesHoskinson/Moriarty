// S1 adversarial matrix: real Rust verifier + real Rust-signed example artifacts consumed by the real Node entry points.
// Inputs (sources, drafts, expected scope and economics) are the hand-written fixtures in intent-vectors.json, never evaluator output.
// Binary: MORIARTY_CRYPTO_BINARY, else <CARGO_TARGET_DIR or experiments/midnight-crypto/target>/debug. Skips only when absent
// and MORIARTY_REQUIRE_NATIVE!=1; with MORIARTY_REQUIRE_NATIVE=1 an absent binary fails.
import test from 'node:test';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {existsSync,readFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {prepareOwnerIntent,verifyAndPrepare} from '../src/auth.ts';
import {starterScenario,repaymentScenario} from '../src/starter.ts';
const root=fileURLToPath(new URL('../../../',import.meta.url));
const binaryPath=process.env.MORIARTY_CRYPTO_BINARY??join(process.env.CARGO_TARGET_DIR??join(root,'experiments/midnight-crypto/target'),'debug/moriarty-midnight-crypto');
const helper=process.env.MORIARTY_INTENT_FIXTURE??join(dirname(binaryPath),'examples/intent-fixture');
const present=existsSync(binaryPath)&&existsSync(helper),required=process.env.MORIARTY_REQUIRE_NATIVE==='1';
const skip=present||required?false:'native verifier or intent-fixture example not built; set MORIARTY_CRYPTO_BINARY';
const config={binaryPath};
if(!present&&required)test('required native binaries exist',()=>assert.fail(`missing ${binaryPath} or ${helper}`));
const vectors=JSON.parse(readFileSync(join(root,'experiments/midnight-crypto/fixtures/intent-vectors.json'),'utf8'));
const scenarios={starter:starterScenario,repayment:repaymentScenario};
const clone=x=>JSON.parse(JSON.stringify(x));
const flip=s=>s.slice(0,-1)+(s.at(-1)==='0'?'1':'0');
const N=BigInt('0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141');
async function pool(items,fn,size=8){const out=[];for(let i=0;i<items.length;i+=size)out.push(...await Promise.all(items.slice(i,i+size).map(fn)));return out;}
// Real Rust-signed public artifacts; the throwaway key never leaves the helper process.
const matrix=[];
if(present)for(const g of vectors.golden){
 const f=vectors.fixtures.find(x=>x.name===g.fixture),h=spawnSync(helper,[g.scheme,g.framing],{input:JSON.stringify(g.draft),encoding:'utf8'});
 assert.equal(h.status,0,h.stderr);assert.equal(h.stderr,'');
 matrix.push({f,g,scenario:scenarios[f.scenario],artifact:JSON.parse(h.stdout),label:`${g.fixture}/${g.scheme}/${g.framing}`});
}
const check=(m,source=m.f.source,scenario=m.scenario,artifact=m.artifact)=>verifyAndPrepare(source,m.f.draft.actionName,JSON.stringify(scenario),typeof artifact==='string'?artifact:JSON.stringify(artifact),config);
// Never throws: unknown/malformed is `throw:<code>`, a checked-and-false signature is `status:SignatureRejected`.
const outcome=(m,...a)=>check(m,...a).then(r=>`status:${r.status}`,e=>`throw:${e.code}`);
const balance=(post,account)=>post.balances.find(b=>b.account===account)?.amount;
const withOpening=(m,opening)=>{const s=clone(m.scenario);s.balances[0].amount=opening;s.allowance.remaining=opening;return s;};
const ready=()=>assert.equal(matrix.length,20,'native fixture matrix was not built');
const text=m=>JSON.stringify(m.artifact);
const sigOf=m=>m.artifact.statement.signature,bad=(m,patch)=>mutateStatement(m,s=>Object.assign(s.signature,patch));
const mutateStatement=(m,edit)=>{const a=clone(m.artifact);edit(a.statement);return a;};
test('native fixture matrix is the 5 manual fixtures x 2 schemes x 2 framings',{skip},()=>{ready();
 assert.equal(matrix.length,20);assert.equal(vectors.fixtures.length,5);
 assert.equal(new Set(matrix.map(m=>m.artifact.statement.signature.publicKeyHex)).size,20,'one fresh throwaway key per run');
 for(const m of matrix)assert.notEqual(m.artifact.statement.signature.publicKeyHex,m.g.statement.signature.publicKeyHex);
});
test('real Rust-signed artifacts reach unqualified Core with hand-written effects and post state',{skip},async()=>{ready();
 await pool(matrix,async m=>{
  const e=m.f.expected_economics,scope=m.f.expected_scope,signer=scope.signer;
  const built=await prepareOwnerIntent(m.f.source,m.f.draft.actionName,JSON.stringify(m.scenario),{scheme:m.g.scheme,publicKeyHex:m.artifact.statement.signature.publicKeyHex,framing:m.g.framing},config);
  assert.deepEqual(built.statement,m.artifact.statement,`${m.label}: Node-prepared statement differs from the Rust helper's`);
  const r=await check(m),c=r.local.result.candidate,debits=c.effects.filter(x=>x.kind==='Debit'),credits=c.effects.filter(x=>x.kind==='Credit');
  assert.equal(r.status,'SignedPreparedUnqualified',m.label);assert.equal(r.signature.signature_valid,true);
  assert.deepEqual([r.keyAuthority,r.ledger_accepted,r.nativeProof,r.ledger,r.signature.authority_valid],['Unverified',false,'NotChecked','NotSubmitted',null]);
  assert.deepEqual(debits.map(x=>[x.account,x.amount]),[[signer,e.debit]],m.label);
  if(scope.kind==='Transfer')assert.deepEqual(Object.fromEntries(credits.filter(x=>x.account!==signer).map(x=>[x.account,x.amount])),e.credits,m.label);
  else{const o=c.effects.find(x=>x.kind==='SetObligation'),a=e.obligationAfter;assert.deepEqual([o.principal,o.accrued,o.status],[a.principal,a.accrued,a.status],m.label);}
  assert.equal(balance(c.candidatePost,signer),e.payerAfter,m.label);
 });
});
test('source rebinds (network, representation, symbol, account, asset, amount, key, agreement, bytes) never reuse a signature',{skip},async()=>{ready();
 const rebinds=[['network: "preview"','network: "mainnet"','BETA_SIGNATURE_SOURCE_MISMATCH'],['representation: "canonical"','representation: "wrapped"','BETA_SIGNATURE_SOURCE_MISMATCH'],
  ['symbol: "USD"','symbol: "EUR"','BETA_SIGNATURE_SOURCE_MISMATCH'],['scale: 2','scale: 3','BETA_SIGNATURE_SOURCE_MISMATCH'],['key: "key1"','key: "key2"','BETA_SIGNATURE_SOURCE_MISMATCH'],
  ['nonce: "n1"','nonce: "n2"','BETA_SIGNATURE_SOURCE_MISMATCH'],['to: 10)','to: 11)','BETA_SIGNATURE_SOURCE_MISMATCH'],['policy_digest: "policy1"','policy_digest: "policy2"','BETA_SIGNATURE_SOURCE_MISMATCH'],
  ['agreement ','agreement X','BETA_SIGNATURE_SOURCE_MISMATCH'],['id: "Midnight"','id: "Other"','BETA_AUTH_SOURCE'],['id: "A"','id: "B"','BETA_AUTH_SOURCE']];
 let applied=0;const jobs=[];
 for(const m of matrix)for(const [a,b,code] of [...rebinds,[`id: "${m.f.expected_scope.signer}"`,'id: "Mallory"','BETA_AUTH_SOURCE']])if(m.f.source.includes(a))jobs.push({m,a,b,code});
 jobs.push(...matrix.flatMap(m=>[['\n',' \n'],['}',' }']].map(([a,b])=>({m,a,b,code:'BETA_SIGNATURE_SOURCE_MISMATCH'}))));
 await pool(jobs,async({m,a,b,code})=>{applied++;assert.equal(await outcome(m,m.f.source.replace(a,b)),`throw:${code}`,`${m.label}: ${a} -> ${b}`);});
 for(const m of matrix)assert.equal(await outcome(m,m.f.source+'\n'),'throw:BETA_SIGNATURE_SOURCE_MISMATCH');
 assert.ok(applied>=220,`rebind coverage ${applied}`);
});
test('the scenario is not signed: a real valid signature is portable and only Core decides',{skip},async()=>{ready();
 await pool(matrix,async m=>{
  const e=m.f.expected_economics,signer=m.f.expected_scope.signer,wide=(BigInt(e.opening)*5n).toString();
  const late=await check(m,undefined,{...m.scenario,round:'11'});
  assert.deepEqual([late.status,late.signature.signature_valid,late.expiry],['SignedCoreRejected',true,'LocalRoundOutsideWindow'],m.label);
  const poor=await check(m,undefined,withOpening(m,'5'));assert.deepEqual([poor.status,poor.signature.signature_valid,poor.financial],['SignedCoreRejected',true,'CoreRejected'],m.label);
  const spent=await check(m,undefined,{...m.scenario,replay:'consumed'});assert.deepEqual([spent.status,spent.replay,spent.signature.signature_valid],['SignedCoreRejected','LocallyConsumed',true],m.label);
  const rich=await check(m,undefined,withOpening(m,wide));// same signature, different unsigned scenario, different post state
  assert.equal(rich.status,'SignedPreparedUnqualified');assert.equal(balance(rich.local.result.candidate.candidatePost,signer),(BigInt(wide)-BigInt(e.debit)).toString(),m.label);
  assert.equal(rich.state,'LocalStipulationOnly');assert.equal(rich.domainMapping,'SourceClaimsBound');
 });
});
test('hostile artifact JSON and schema fields reject before any Core result',{skip},async()=>{ready();
 await pool(matrix.filter(m=>m.g.fixture==='transfer-fee'),async m=>{
  const t=text(m),a=m.artifact,wrap=(extra)=>`{${extra},${t.slice(1)}`;
  const cases=[['duplicate top key',wrap('"statement":1'),'BETA_JSON_DUPLICATE'],['duplicate signatureHex',wrap(`"signatureHex":"${a.signatureHex}"`),'BETA_JSON_DUPLICATE'],
   ['duplicate nested key',t.replace('"signature":{','"signature":{"framing":"raw",'),'BETA_JSON_DUPLICATE'],['__proto__ key',wrap('"__proto__":{}'),null],['constructor key',wrap('"constructor":1'),'BETA_SIGNATURE_SCHEMA'],
   ['extra top field',{...a,extra:1},'BETA_SIGNATURE_SCHEMA'],['missing statement',{signatureHex:a.signatureHex},'BETA_SIGNATURE_SCHEMA'],['missing signatureHex',{statement:a.statement},'BETA_SIGNATURE_SCHEMA'],
   ['extra signature field',bad(m,{extra:1}),'BETA_SIGNATURE_SCHEMA'],['extra statement field',{...a,statement:{...a.statement,extra:1}},'BETA_SIGNATURE_SOURCE_MISMATCH'],
   ['array artifact','[]','BETA_SIGNATURE_SCHEMA'],['null artifact','null','BETA_SIGNATURE_SCHEMA'],['string artifact','"x"','BETA_SIGNATURE_SCHEMA'],['empty text','',null],['BOM','﻿'+t,null],['trailing comma',t.replace(/}$/,',}'),null],['trailing garbage',t+'x',null],
   ['uppercase signature',{...a,signatureHex:a.signatureHex.toUpperCase()},'BETA_SIGNATURE_SCHEMA'],['0x signature',{...a,signatureHex:'0x'+a.signatureHex},'BETA_SIGNATURE_SCHEMA'],['short signature',{...a,signatureHex:a.signatureHex.slice(2)},'BETA_SIGNATURE_SCHEMA'],
   ['long signature',{...a,signatureHex:a.signatureHex+'00'},'BETA_SIGNATURE_SCHEMA'],['numeric signature',{...a,signatureHex:1},'BETA_SIGNATURE_SCHEMA'],['null statement',{...a,statement:null},'BETA_SIGNATURE_SCHEMA'],
   ['numeric amount',mutateStatement(m,s=>{s.intent.operation.amount=1000;}),null],['number-typed nonce',mutateStatement(m,s=>{s.intent.nonce=1;}),null],['array-typed signature object',mutateStatement(m,s=>{s.signature=[];}),'BETA_SIGNATURE_SCHEMA'],
   ['unknown scheme',bad(m,{scheme:'ed25519'}),'BETA_SIGNATURE_SCHEMA'],['key with 0x',bad(m,{publicKeyHex:'0x'+sigOf(m).publicKeyHex}),'BETA_SIGNATURE_SCHEMA'],
   ['uppercase key',bad(m,{publicKeyHex:sigOf(m).publicKeyHex.toUpperCase()}),'BETA_SIGNATURE_SCHEMA'],['empty key',bad(m,{publicKeyHex:''}),'BETA_SIGNATURE_SCHEMA'],
   ['deep nesting',`{"statement":${'['.repeat(2000)+']'.repeat(2000)},"signatureHex":"${a.signatureHex}"}`,null],['oversized text',`{"statement":"${'a'.repeat(70000)}"}`,null]];
  for(const [name,artifact,code] of cases){
   const got=await outcome(m,undefined,undefined,artifact);
   assert.match(got,code?new RegExp(`^throw:${code}$`):/^throw:BETA_/,`${m.label}: ${name} -> ${got}`);
  }
 },4);
});
test('unknown (thrown) and false (SignatureRejected) stay distinct for real signatures',{skip},async()=>{ready();
 await pool(matrix,async m=>{
  const a=m.artifact,s=BigInt('0x'+a.signatureHex.slice(64)),other=matrix.find(x=>x!==m&&x.g.scheme===m.g.scheme&&x.g.framing===m.g.framing&&x.g.fixture!==m.g.fixture);
  const falsy=[['flip r',{...a,signatureHex:flip(a.signatureHex.slice(0,64))+a.signatureHex.slice(64)}],['flip s',{...a,signatureHex:a.signatureHex.slice(0,64)+flip(a.signatureHex.slice(64))}],
   ['other fixture signature',{...a,signatureHex:other.artifact.signatureHex}],['framing swapped',bad(m,{framing:m.g.framing==='raw'?'midnight-sign-data':'raw'})],
   ['golden key',bad(m,{publicKeyHex:m.g.statement.signature.publicKeyHex})]];
  if(m.g.scheme.startsWith('ecdsa'))falsy.push(['high-S twin',{...a,signatureHex:a.signatureHex.slice(0,64)+(N-s).toString(16).padStart(64,'0')}]);
  for(const [name,artifact] of falsy){const r=await check(m,undefined,undefined,artifact);assert.equal(r.status,'SignatureRejected',`${m.label}: ${name}`);assert.equal(r.signature.signature_valid,false);assert.equal(r.local,null);assert.equal(r.financial,'NotChecked');}
  if(m.g.scheme.startsWith('ecdsa'))assert.equal(await outcome(m,undefined,undefined,bad(m,{publicKeyHex:'05'+a.statement.signature.publicKeyHex.slice(2)})),'throw:BETA_SIGNATURE_SCHEMA','SEC1 prefix alias is unknown, not false');
 });
});
// Same walker as the Rust matrix (a leaf is a non-empty-container terminal incl. empty arrays and null): 868 = 8x44 + 12x43.
function leaves(v,p=[],out=[]){if(v&&typeof v==='object'&&Object.keys(v).length)for(const k of Object.keys(v))leaves(v[k],[...p,k],out);else out.push(p);return out;}
const mutated=v=>typeof v==='string'?(v===''?'x':flip(v)):Array.isArray(v)?['x']:'x';
const expectedFor=path=>path==='signature.publicKeyHex'?/^(status:SignatureRejected|throw:BETA_SIGNATURE_SCHEMA)$/:/^signature\.(scheme|framing)$/.test(path)?/^throw:BETA_SIGNATURE_SCHEMA$/:/^throw:BETA_SIGNATURE_SOURCE_MISMATCH$/;
test('every statement leaf of all 20 real artifacts is bound: mutating any one never yields a prepared result',{skip},async()=>{ready();
 const jobs=matrix.flatMap(m=>leaves(m.artifact.statement).map(p=>({m,p})));
 assert.equal(jobs.length,868,'walker leaf count must equal the Rust matrix (868)');
 const tally={};
 await pool(jobs,async({m,p})=>{
  const path=p.join('.'),a=clone(m.artifact);let o=a.statement;for(const k of p.slice(0,-1))o=o[k];o[p.at(-1)]=mutated(o[p.at(-1)]);
  const got=await outcome(m,undefined,undefined,a);tally[got]=(tally[got]??0)+1;
  assert.match(got,expectedFor(path),`${m.label}: ${path} -> ${got}`);
 },12);
 const sigFlips=await pool(matrix,m=>outcome(m,undefined,undefined,{...m.artifact,signatureHex:flip(m.artifact.signatureHex)}));
 assert.ok(sigFlips.every(x=>x==='status:SignatureRejected'));
 console.log('S1_NODE_COVERAGE '+JSON.stringify({profile:'moriarty-s1-intent-node-adversarial/1',artifacts:matrix.length,leaves:jobs.length,outcomes:tally,signatureHexFlips:sigFlips.length,nativeBinary:'real',claims:{keyAuthority:'Unverified',ledgerAccepted:false}}));
});
