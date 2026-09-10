import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
const require=createRequire('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/package.json');
const runtime=await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')).href);
const api=await import('file:///home/charl/Moriarty/.worktrees/sp05-fee-comparison/experiments/moriarty-midnight-financial/ledger/financial-comparison.mjs').catch(()=>({}));
const h=n=>n.toString(16).padStart(64,'0');
const bytes=x=>Uint8Array.from(Buffer.from(x,'hex'));
const pad=x=>Uint8Array.from(Buffer.from(x.padEnd(32,'\0')));
const roles={firstSecret:bytes(h(71)),secondSecret:bytes(h(72)),firstAddress:h(101),secondAddress:h(102)};
const networkTag=h(301),address=h(401);
const state=(a)=>Object.fromEntries(a.map((v,i)=>['f'+i,BigInt(v)]));
const effect=a=>Object.fromEntries(a.map((v,i)=>['v'+i,BigInt(v)]));
const result=(a,remaining,revision,effects)=>({after:state(a),remaining:BigInt(remaining),revision:BigInt(revision),...Object.fromEntries(effects.map((e,i)=>['effect'+i,effect(e)]))});
// Synthetic unit-test observations, manually transcribed independently of the
// production expectations file. No unit-test PASS is real receipt acceptance.
function fixture(kind='loan') {
 const isLoan=kind==='loan';
 const program=isLoan?'95b46e39a9039e19063bb3d618128aec6cbd9ee656b6e635b3587e7f3f5235b2':'b00a55b8da611d23c08461a11853151aa50ec7ee8150f9d16afbc66d62e018fa';
 const names=isLoan?['borrower','lender']:['trader','provider'];
 const ds=isLoan?['moriarty:sp05:usd:v1']:['moriarty:sp05:asset-a:v1','moriarty:sp05:asset-b:v1'];
 const domains=ds.map(pad),colors=domains.map(d=>runtime.rawTokenType(d,address));
 const vectors=isLoan?[[0,0,0,0,0,0,0,0,0],[5000000000,0,0,0,0,20000000000,0,0,0],[4500000000,500000000,33972602,0,0,20000000000,0,1,0],[4500000000,0,0,500000000,33972602,19466027398,533972602,2,1]]:[[0,0,0,0,0,0,0],[1000000,2000000,100000,0,0,0,0],[1010000,1980257,90000,19743,0,0,0],[0,0,90000,19743,1010000,1980257,1]];
 const rem=isLoan?[0,2,1,0]:[0,8,7,6],rev=[0,0,1,2];
 const r0=isLoan?result(vectors[2],1,1,[[4,2,5,1,500000000],[3,2,5,1,33972602]]):result(vectors[2],7,1,[[0,4,2,10000],[1,2,4,19743]]);
 const r1=isLoan?result(vectors[3],0,2,[[0,2,5,533972602],[4,2,5,1,500000000,0],[3,2,5,1,33972602,0]]):result(vectors[3],6,2,[[0,2,3,1010000],[1,2,3,1980257]]);
 const z0=isLoan?result(vectors[0],0,0,[[0,0,0,0,0],[0,0,0,0,0]]):result(vectors[0],0,0,[[0,0,0,0],[0,0,0,0]]);
 const z1=isLoan?result(vectors[0],0,0,[[0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0]]):z0;
 const stages=isLoan?['deploy','initialize','accrue','settle']:['deploy','initialize','swap','close'];
 const makeOutput=(owner,color,value,i=0)=>({segment:0,section:'guaranteed',owner,type:color,value:String(value),intentHash:h(801),offerIndex:i});
 const makeInput=(owner,color,value)=>({segment:0,section:'guaranteed',owner,type:color,value:String(value),intentHash:h(901),outputNo:0});
 const claim=(color,recipient,amount,recipientKind='user')=>({type:color,recipientKind,recipient,amount:String(amount)});
 const rows=stages.map((stage,i)=>{
  const s={programDigest:bytes(program),networkTag:bytes(networkTag),initialized:i>0,remaining:BigInt(rem[i]),revision:BigInt(rev[i]),kernelState:state(vectors[i])};
  names.forEach((name,j)=>{s[name+'Address']={bytes:bytes(j?roles.secondAddress:roles.firstAddress)};s[name+'Capability']=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[pad(`moriarty:sp05:${kind}:${name}`),bytes(networkTag),bytes(program),j?roles.secondSecret:roles.firstSecret]);});
  if(isLoan){s.usdDomain=domains[0];s.usdColor=bytes(i?colors[0]:h(0));s.lastAccrue=structuredClone(i>=2?r0:z0);s.lastSettle=structuredClone(i>=3?r1:z1);}
  else {s.assetADomain=domains[0];s.assetBDomain=domains[1];s.colorA=bytes(i?colors[0]:h(0));s.colorB=bytes(i?colors[1]:h(0));s.lastSwap=structuredClone(i>=2?r0:z0);s.lastClose=structuredClone(i>=3?r1:z1);}
  const effects={unshieldedMints:{},unshieldedInputs:{},unshieldedOutputs:{},claimedUnshieldedSpends:[]};
  let inputs=[],outputs=[],balances={};
  if(isLoan&&i===1){effects.unshieldedMints[Buffer.from(domains[0]).toString('hex')]='20000000000';effects.claimedUnshieldedSpends=[claim(colors[0],roles.firstAddress,20000000000)];outputs=[makeOutput(roles.firstAddress,colors[0],20000000000)];}
  if(isLoan&&i===3){effects.unshieldedInputs[colors[0]]='533972602';effects.unshieldedOutputs[colors[0]]='533972602';effects.claimedUnshieldedSpends=[claim(colors[0],roles.secondAddress,533972602)];inputs=[makeInput(roles.firstAddress,colors[0],20000000000)];outputs=[makeOutput(roles.firstAddress,colors[0],19466027398),makeOutput(roles.secondAddress,colors[0],533972602,1)];}
  if(!isLoan&&i===1){effects.unshieldedInputs={[colors[0]]:'1000000',[colors[1]]:'2000000'};effects.unshieldedMints[Buffer.from(domains[0]).toString('hex')]='1100000';effects.unshieldedMints[Buffer.from(domains[1]).toString('hex')]='2000000';effects.claimedUnshieldedSpends=[claim(colors[0],address,1000000,'contract'),claim(colors[1],address,2000000,'contract'),claim(colors[0],roles.firstAddress,100000)];outputs=[makeOutput(roles.firstAddress,colors[0],100000)];balances={[colors[0]]:'1000000',[colors[1]]:'2000000'};}
  if(!isLoan&&i===2){effects.unshieldedInputs[colors[0]]='10000';effects.unshieldedOutputs[colors[1]]='19743';effects.claimedUnshieldedSpends=[claim(colors[1],roles.firstAddress,19743)];inputs=[makeInput(roles.firstAddress,colors[0],100000)];outputs=[makeOutput(roles.firstAddress,colors[0],90000),makeOutput(roles.firstAddress,colors[1],19743,1)];balances={[colors[0]]:'1010000',[colors[1]]:'1980257'};}
  if(!isLoan&&i===3){effects.unshieldedOutputs={[colors[0]]:'1010000',[colors[1]]:'1980257'};effects.claimedUnshieldedSpends=[claim(colors[0],roles.secondAddress,1010000),claim(colors[1],roles.secondAddress,1980257)];outputs=[makeOutput(roles.secondAddress,colors[0],1010000),makeOutput(roles.secondAddress,colors[1],1980257,1)];}
  const txId=h(501+i),tx={schema:'moriarty.native-financial-transaction/1',rawSha256:h(601+i),transactionHash:h(701+i),identifiers:[txId,h(751+i)],inputs,outputs,actions:[i?{segment:0,kind:'call',address,entryPoint:stage,transcripts:[{section:'guaranteed',effects}]}:{segment:0,kind:'deploy',address}],grossByAsset:Object.fromEntries(inputs.map(x=>[x.type,x.value])),dustFee:'123',proofVerified:false,ledgerAccepted:false};
  return {receipt:{schema:'moriarty.finalized-financial-stage/1',txId,contractAddress:address,circuitId:stage,transaction:tx,blockHash:h(1001+i),blockHeight:10+i,finalizedHead:'0x'+h(1001+i),finalizedHeight:10+i,protocolVersion:1000000,indexerIdentifiers:[txId,h(751+i)],fees:{nativeDebit:{asset:'DUST',unit:'SPECK',amount:'123'},indexerReported:{paid:'987',estimated:'999',sourceUnitLabel:'DUST',encoding:'unresolved',nativeDebitRelationship:'unresolved'}},contractBalances:balances,acceptance:'uncertified-I2-observation'},state:s};
 });
 return {kind,stages,rows,colors};
}
async function comparator(kind='loan') {assert.equal(typeof api.createFinancialComparator,'function','financial comparator must exist');return api.createFinancialComparator({kind,roles,networkTag,expectedProtocolVersion:1000000,dustFeeCap:492n});}
async function run(fx,mutate) {const c=await comparator(fx.kind);for(let i=0;i<fx.rows.length;i++){if(mutate)mutate(fx.rows[i],i,fx);c.verifyStage(fx.stages[i],fx.rows[i]);}return c.finish();}


test('independent zero ceiling accepts only a complete zero-native-fee trace', async()=>{
 for(const kind of ['loan','swap']){
  const fx=fixture(kind); for(const x of fx.rows)x.receipt.fees.nativeDebit.amount=x.receipt.transaction.dustFee='0';
  const c=await api.createFinancialComparator({kind,roles,networkTag,expectedProtocolVersion:1000000,dustFeeCap:0n});
  fx.rows.forEach((row,i)=>c.verifyStage(fx.stages[i],row));assert.equal(c.finish().status,'PASS');
  const bad=fixture(kind);const b=await api.createFinancialComparator({kind,roles,networkTag,expectedProtocolVersion:1000000,dustFeeCap:0n});
  assert.throws(()=>b.verifyStage('deploy',bad.rows[0]),{message:'NATIVE_FEE_CAP_EXCEEDED'});
 }
});
test('independent historical groups cannot subsidize one another',async()=>{
 const fx=fixture(),c=await api.createFinancialComparator({kind:'loan',roles,networkTag,expectedProtocolVersion:1000000,dustFeeCap:1000n,historicalFeeAllocations:[{stages:['deploy'],dustFeeCap:1000n},{stages:['initialize'],dustFeeCap:122n}]});
 c.verifyStage('deploy',fx.rows[0]);assert.throws(()=>c.verifyStage('initialize',fx.rows[1]),{message:'NATIVE_FEE_CAP_EXCEEDED'});
});
test('independent uint128 cumulative boundary uses exact arithmetic',async()=>{
 const fx=fixture(),max=(1n<<128n)-1n;
 fx.rows[0].receipt.fees.nativeDebit.amount=fx.rows[0].receipt.transaction.dustFee=String(max);
 const c=await api.createFinancialComparator({kind:'loan',roles,networkTag,expectedProtocolVersion:1000000,dustFeeCap:max});
 c.verifyStage('deploy',fx.rows[0]);assert.throws(()=>c.verifyStage('initialize',fx.rows[1]),{message:'NATIVE_FEE_CAP_EXCEEDED'});
});
