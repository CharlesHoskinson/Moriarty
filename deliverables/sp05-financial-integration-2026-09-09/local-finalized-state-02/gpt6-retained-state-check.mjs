// Independent offline audit: reads retained public snapshots and installed code only.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
const nm='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
const {ContractState}=await import(pathToFileURL(nm+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs').href);
const json=p=>JSON.parse(readFileSync(new URL(p,import.meta.url),'utf8'));
const sha=x=>createHash('sha256').update(x).digest('hex');
function normalize(v){if(typeof v==='bigint')return String(v);if(v instanceof Uint8Array)return {hex:Buffer.from(v).toString('hex')};if(Array.isArray(v))return v.map(normalize);if(v&&typeof v==='object')return Object.fromEntries(Object.entries(v).map(([k,x])=>[k,normalize(x)]));return v;}
function oldFormat(v){if(v&&typeof v==='object'&&!Array.isArray(v)&&Object.keys(v).length===1&&typeof v.hex==='string')return v.hex;if(Array.isArray(v))return v.map(oldFormat);if(v&&typeof v==='object')return Object.fromEntries(Object.entries(v).map(([k,x])=>[k,oldFormat(x)]));return v;}
const result=json('./probe-result.json');assert.equal(result.status,'OBSERVED');assert.equal(result.snapshots.length,2);assert.equal(result.loadersClosed,2);assert.equal(result.elapsedMs,2802);
const cases=[['loan','local-continuation-02','settle','51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7'],['swap','local-swap-continuation-01','close','3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362']];
const checks=[];
for(let i=0;i<cases.length;i++){
 const [kind,dir,stage,receiptSha256]=cases[i],s=result.snapshots[i],raw=Buffer.from(s.serializedStateHex,'hex');
 assert.equal(sha(raw),s.stateSha256);assert.equal(s.blockHeight,20415);assert.equal(s.blockHash,'0xe54358be199c4b34f94a74237452c1bb7d78b8a3cb6bd042a199590cbd092aac');assert.equal(s.authenticatedStateProof,false);assert.equal(s.noInterveningActionsAfterAnchorEstablished,false);
 const historical=json('../'+dir+'/run-public/integration-result.json').financialComparison.stages.find(x=>x.stage===stage);
 const loaded=await loadProvenFinancialContract({case:kind,receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/'+kind+'-output/build/build-receipt.json',receiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
 const native=ContractState.deserialize(raw);let actualBalances;
 try{
  assert.deepEqual(Buffer.from(native.serialize()),raw);
  actualBalances={};assert.ok(native.balance instanceof Map);
  for(const [token,amount] of native.balance){assert.deepEqual(Object.keys(token).sort(),['raw','tag']);assert.equal(token.tag,'unshielded');assert.match(token.raw,/^[0-9a-f]{64}$/);assert.equal(typeof amount,'bigint');assert.ok(amount>=0n);assert.ok(!Object.hasOwn(actualBalances,token.raw));actualBalances[token.raw]=String(amount);}
  assert.deepEqual(actualBalances,s.balances);
  const decoded=normalize(loaded.decodeState(native.data));assert.deepEqual(decoded,s.state);assert.deepEqual(Buffer.from(native.serialize()),raw);
  assert.equal(s.contractAddress,historical.contractAddress);assert.equal(historical.status,'PASS');assert.ok(s.blockHeight>historical.blockHeight);assert.deepEqual(oldFormat(decoded),historical.publicState);
  if(kind==='swap')assert.deepEqual(s.balances,historical.contractBalances);
  else{assert.deepEqual(historical.contractBalances,{});assert.deepEqual(s.balances,{[oldFormat(decoded).usdColor]:'0'});}
 }finally{native.free();assert.deepEqual(loaded.cleanup(),{loaderHooksRemoved:true});}
 checks.push({case:kind,status:'PASS',bytes:raw.length,sha256:s.stateSha256,completeNativeRoundTrip:true,completeGeneratedStateEquality:true,independentNativeBalanceProjection:true,balances:actualBalances,publicFieldCount:Object.keys(s.state).length,historicalStage:stage,historicalBlock:historical.blockHeight,historicalStateEqualityAfterExplicitByteFormatConversion:true,historicalBalancesExact:kind==='swap',historicalBalanceDifference:kind==='loan'?'Current native map retains one zero USD token entry; old stage financial summary is {}. Financial zero equivalence only, not full-state equality.':null,loaderClosed:true});
}
console.log(JSON.stringify({schema:'moriarty.independent-retained-state-verification/1',reviewer:'gpt-6-astra',status:'PASS',mode:'offline retained public bytes, no RPC/services/private input/proof/transaction',observedAt:new Date().toISOString(),probeResultSha256:sha(readFileSync(new URL('./probe-result.json',import.meta.url))),checks,scope:'Current trusted-node observation reconciles all decoded fields with historical successful stages; no evidence of absent intervening actions or failed-transaction nonmutation.'},null,2));
