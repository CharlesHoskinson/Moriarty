import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {createFinancialComparator} from '/home/charl/Moriarty/.worktrees/sp05-fee-comparison/experiments/moriarty-midnight-financial/ledger/financial-comparison.mjs';
import {decodeNativeFinancialTransaction} from '/home/charl/Moriarty/.worktrees/sp05-fee-comparison/experiments/moriarty-midnight-financial/ledger/receipt.mjs';
import {PINNED_NM} from '/home/charl/Moriarty/.worktrees/sp05-fee-comparison/experiments/moriarty-midnight-financial/ledger/providers.mjs';
const root='/home/charl/Moriarty/';const d='deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/';
const candidate=JSON.parse(readFileSync(root+d+'actual-result-candidate-01.json'));
const pins={};function raw(path){const r=readFileSync(root+path),h=createHash('sha256').update(r).digest('hex');assert.equal(candidate.files[path],h,path);pins[path]=h;return r;}
const read=file=>JSON.parse(raw(d+'actual-run/'+file));
const plan=read('plan.json'),retained=read('integration-result.json');
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs').href);
const runtime=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/compact-runtime/dist/index.js').href);
const bytes=s=>Uint8Array.from(Buffer.from(s,'hex'));
const roles={firstAddress:plan.roles.firstAddress,secondAddress:plan.roles.secondAddress,firstSecret:new Uint8Array(32).fill(17),secondSecret:new Uint8Array(32).fill(18)};
const scalars=x=>typeof x==='string'?BigInt(x):Object.fromEntries(Object.entries(x).map(([k,v])=>[k,scalars(v)]));
function state(stage){const p=read('stage-'+stage+'.json'),s=structuredClone(p.publicState);
 for(const k of ['programDigest','networkTag','assetADomain','assetBDomain','colorA','colorB'])s[k]=bytes(s[k]);
 for(const k of ['remaining','revision','kernelState','lastSwap','lastClose'])s[k]=scalars(s[k]);
 for(const [role,secret] of [['trader',roles.firstSecret],['provider',roles.secondSecret]]){
  s[role+'Address']={bytes:bytes(s[role+'Address'].bytes)};
  s[role+'Capability']=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[Uint8Array.from(Buffer.from(('moriarty:sp05:swap:'+role).padEnd(32,'\0'))),s.networkTag,s.programDigest,secret]);
 }return s;}
const receipts=retained.driver.stages,total=receipts.reduce((n,r)=>n+BigInt(r.transaction.dustFee),0n);
function normalized(tx){const c=structuredClone(tx);for(const a of c.actions)for(const t of a.transcripts??[])t.effects.claimedUnshieldedSpends.sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));return c;}
for(const r of receipts){const decoded=decodeNativeFinancialTransaction(raw(d+'actual-run/public-transactions/'+r.transaction.transactionHash+'.bin'),ledger);assert.deepEqual(normalized(decoded),normalized(r.transaction));}
async function compare(cap,fail){const c=await createFinancialComparator({kind:'swap',roles,networkTag:plan.networkTag,expectedProtocolVersion:plan.expectedProtocolVersion,dustFeeCap:cap});let n=0;
 for(const r of receipts){const observation={receipt:r,state:state(r.circuitId)};if(fail&&n===3){assert.throws(()=>c.verifyStage(r.circuitId,observation),/NATIVE_FEE_CAP_EXCEEDED/);return;}c.verifyStage(r.circuitId,observation);n++;}assert.equal(c.finish().status,'PASS');}
await compare(BigInt(plan.limits.dustFee),false);await compare(total,false);await compare(total-1n,true);
console.log(JSON.stringify({status:'PASS',checks:['four native raw transactions decode equally with only claimedUnshieldedSpends map entries compared as multiset; input/output order strict','actual admitted cap accepts','exact total cap accepts','total minus one rejects fourth stage'],totalSpeck:total.toString(),admittedCapSpeck:plan.limits.dustFee,scope:'Retained public swap financial replay with synthetic capability commitments only; no renewed finality or proof acceptance',pins},null,2));
