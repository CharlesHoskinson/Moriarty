import assert from 'node:assert/strict';
import test from 'node:test';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
const require=createRequire('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/package.json');
const runtime=await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')).href);
const api=await import('./financial-comparison.mjs').catch(()=>({}));
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
async function comparator(kind='loan') {assert.equal(typeof api.createFinancialComparator,'function','financial comparator must exist');return api.createFinancialComparator({kind,roles,networkTag,expectedProtocolVersion:1000000});}
async function run(fx,mutate) {const c=await comparator(fx.kind);for(let i=0;i<fx.rows.length;i++){if(mutate)mutate(fx.rows[i],i,fx);c.verifyStage(fx.stages[i],fx.rows[i]);}return c.finish();}

test('complete loan and swap comparison preserves gross spending and residual duties',async()=>{
 for(const kind of ['loan','swap']){const r=await run(fixture(kind));assert.equal(r.status,'PASS');assert.equal(r.networkAcceptance,false);assert.equal(r.proofAcceptance,false);assert.equal(r.stages.length,4);const spend=r.stages[kind==='loan'?3:2];assert.equal(spend.grossByAsset[fixture(kind).colors[0]],kind==='loan'?'20000000000':'100000');assert.equal(spend.nativeFee.amount,'123');}
});
const mutations=[
 ['missing financial field',(o,i)=>{if(i===2)delete o.state.kernelState.f0;}],
 ['extra financial field',(o,i)=>{if(i===2)o.state.kernelState.invented=0n;}],
 ['remaining debt erased',(o,i)=>{if(i===3)o.state.kernelState.f0=0n;}],
 ['string-valued decoded amount',(o,i)=>{if(i===1)o.state.kernelState.f0='5000000000';}],
 ['prior result overwritten',(o,i)=>{if(i===3)o.state.lastAccrue.effect0.v4=0n;}],
 ['effect recipient corrupted',(o,i)=>{if(i===3)o.state.lastSettle.effect0.v2=2n;}],
 ['extra result member',(o,i)=>{if(i===2)o.state.lastAccrue.work=1n;}],
 ['initialization flag wrong',(o,i)=>{if(i===1)o.state.initialized=false;}],
 ['capability changed',(o,i)=>{if(i===1)o.state.borrowerCapability=bytes(h(1));}],
 ['payout address changed',(o,i)=>{if(i===1)o.state.lenderAddress.bytes=bytes(h(999));}],
 ['domain changed',(o,i)=>{if(i===1)o.state.usdDomain=bytes(h(999));}],
 ['color changed',(o,i)=>{if(i===1)o.state.usdColor=bytes(h(999));}],
 ['network changed',(o,i)=>{if(i===1)o.state.networkTag=bytes(h(999));}],
 ['program changed',(o,i)=>{if(i===1)o.state.programDigest=bytes(h(999));}],
 ['native payout changed',(o,i)=>{if(i===3)o.receipt.transaction.actions[0].transcripts[0].effects.claimedUnshieldedSpends[0].recipient=roles.firstAddress;}],
 ['native conversion amount changed',(o,i)=>{if(i===3)o.receipt.transaction.actions[0].transcripts[0].effects.unshieldedInputs[Object.keys(o.receipt.transaction.grossByAsset)[0]]='533972603';}],
 ['hidden native effect',(o,i)=>{if(i===2)o.receipt.transaction.actions[0].transcripts[0].effects.extra={};}],
 ['missing native transcript',(o,i)=>{if(i===2)o.receipt.transaction.actions[0].transcripts=[];}],
 ['actual lender output short',(o,i)=>{if(i===3)o.receipt.transaction.outputs[1].value='533972601';}],
 ['third-party payer',(o,i)=>{if(i===3)o.receipt.transaction.inputs[0].owner=roles.secondAddress;}],
 ['gross debit replaced with net',(o,i)=>{if(i===3)o.receipt.transaction.grossByAsset[Object.keys(o.receipt.transaction.grossByAsset)[0]]='533972602';}],
 ['native fee mismatch',(o,i)=>{if(i===3)o.receipt.fees.nativeDebit.amount='124';}],
 ['unresolved fee treated resolved',(o,i)=>{if(i===3)o.receipt.fees.indexerReported.nativeDebitRelationship='equal';}],
 ['contract address changed',(o,i)=>{if(i===1)o.receipt.contractAddress=h(999);}],
 ['duplicate tx id',(o,i)=>{if(i===2){o.receipt.txId=h(502);o.receipt.transaction.identifiers[0]=h(502);}}],
 ['block order reversed',(o,i)=>{if(i===3)o.receipt.blockHeight=9;}],
 ['unfinalized height',(o,i)=>{if(i===3)o.receipt.finalizedHeight=1;}],
 ['unsupported protocol',(o,i)=>{if(i===3)o.receipt.protocolVersion=8;}],
 ['false native proof claim',(o,i)=>{if(i===3)o.receipt.transaction.proofVerified=true;}],
];
for(const [name,mutate] of mutations)test('rejects '+name,async()=>{assert.equal(typeof api.createFinancialComparator,'function');await assert.rejects(run(fixture(),mutate));});
test('swap must observe contract reserves and mint-to-self claims',async()=>{
 assert.equal(typeof api.createFinancialComparator,'function');
 await assert.rejects(run(fixture('swap'),(o,i)=>{if(i===2)o.receipt.contractBalances={};}));
 await assert.rejects(run(fixture('swap'),(o,i)=>{if(i===1)o.receipt.transaction.actions[0].transcripts[0].effects.claimedUnshieldedSpends[0].recipientKind='user';}));
});
test('rejects skipped stages and remains stopped after a failure',async()=>{
 const fx=fixture(),c=await comparator();assert.throws(()=>c.finish(),/INCOMPLETE/);assert.throws(()=>c.verifyStage('initialize',fx.rows[1]),/STAGE_ORDER/);assert.throws(()=>c.verifyStage('deploy',fx.rows[0]),/STOPPED/);
});
test('rejects duplicate participants and never exposes role secrets',async()=>{
 assert.equal(typeof api.createFinancialComparator,'function');await assert.rejects(api.createFinancialComparator({kind:'loan',roles:{...roles,secondAddress:roles.firstAddress},networkTag,expectedProtocolVersion:1000000}),/PARTICIPANTS/);
 const r=await run(fixture());assert.equal(JSON.stringify(r).includes(Buffer.from(roles.firstSecret).toString('hex')),false);
});

test('requires explicit ledger protocol, distinct from package version',async()=>{
 assert.equal(typeof api.createFinancialComparator,'function');await assert.rejects(api.createFinancialComparator({kind:'loan',roles,networkTag}),/PROTOCOL/);
});
test('rejects indexer identifier omissions, extras and duplicates',async()=>{
 for(const mutation of [r=>r.indexerIdentifiers.pop(),r=>r.indexerIdentifiers.push(h(12345)),r=>r.indexerIdentifiers.push(r.indexerIdentifiers[0])])await assert.rejects(run(fixture(),(o,i)=>{if(i===1)mutation(o.receipt);}));
});
test('rejects zero-valued hidden payout claims',async()=>{
 await assert.rejects(run(fixture(),(o,i)=>{if(i===2)o.receipt.transaction.actions[0].transcripts[0].effects.claimedUnshieldedSpends.push({type:fixture().colors[0],recipientKind:'user',recipient:h(888),amount:'0'});}));
});

test('summary preserves every observed public field and native aggregate without mutable aliases',async()=>{
 for(const kind of ['loan','swap']){
  const fx=fixture(kind),c=await comparator(kind);
  for(let i=0;i<fx.rows.length;i++){
   const observation=fx.rows[i];
   const normalize=value=>typeof value==='bigint'?value.toString():value instanceof Uint8Array?Buffer.from(value).toString('hex'):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).map(([k,v])=>[k,normalize(v)])):value;
   const publicSnapshot=normalize(observation.state);
   const nativeSnapshot=i===0?{unshieldedMints:{},unshieldedInputs:{},unshieldedOutputs:{},claimedUnshieldedSpends:[]}:structuredClone(observation.receipt.transaction.actions[0].transcripts[0].effects);
   const summary=c.verifyStage(fx.stages[i],observation);
   assert.deepEqual(summary.publicState,publicSnapshot);
   assert.deepEqual(summary.nativeEffects,nativeSnapshot);
   observation.state.kernelState.f0=999n;
   observation.state.programDigest.fill(255);
   if(i>0)observation.receipt.transaction.actions[0].transcripts[0].effects.unshieldedMints={corrupted:'1'};
   summary.publicState.kernelState.f0='888';
   summary.nativeEffects.unshieldedMints={corrupted:'2'};
   // Keep independently saved snapshots for the later complete result check.
   fx.rows[i]={publicSnapshot,nativeSnapshot};
  }
  const complete=c.finish();
  complete.stages.forEach((summary,i)=>{assert.deepEqual(summary.publicState,fx.rows[i].publicSnapshot);assert.deepEqual(summary.nativeEffects,fx.rows[i].nativeSnapshot);assert.equal(Object.hasOwn(summary.publicState,'firstSecret'),false);});
  complete.stages[0].publicState.kernelState.f0='777';
  assert.equal(c.finish().stages[0].publicState.kernelState.f0,'0');
 }
});

test('unknown top-level public state cannot leak into a summary',async()=>{
 await assert.rejects(run(fixture(),(o,i)=>{if(i===1)o.state.hiddenSecret=bytes(h(99));}),/FIELDS_PUBLIC_STATE/);
});

test('mint-to-self input counts once as contract funding and never as participant spending',async()=>{
 const fx=fixture('swap'),r=await run(fx),s=r.stages[1];
 assert.deepEqual(s.grossByAsset,{});
 assert.equal(s.nativeEffects.unshieldedInputs[fx.colors[0]],'1000000');
 assert.equal(s.nativeEffects.unshieldedInputs[fx.colors[1]],'2000000');
 assert.equal(s.contractBalances[fx.colors[0]],'1000000');
 assert.equal(s.contractBalances[fx.colors[1]],'2000000');
 assert.equal(s.participantNetDeltas.trader.ASSET_A,'100000');
 await assert.rejects(run(fixture('swap'),(o,i)=>{if(i===1)o.receipt.transaction.actions[0].transcripts[0].effects.unshieldedInputs={};}),/unshieldedInputs/);
 await assert.rejects(run(fixture('swap'),(o,i,f)=>{if(i===1)o.receipt.contractBalances[f.colors[0]]='2000000';}),/CONTRACT_BALANCES/);
});
