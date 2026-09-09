// Retained skip-zk runtime differential test. Requires explicit existing artifacts.
// State and transcript effects come from generated custody circuits. Native UTXO
// receipts, block/finality/fee fields below are INERT SYNTHETIC transport data;
// this test neither creates proofs nor establishes wallet or ledger acceptance.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createFinancialComparator} from './financial-comparison.mjs';
import {projectTranscriptEffects} from './receipt.mjs';
import {PIN_PATHS} from '../custody/generate.mjs';
const artifacts=process.env.MORIARTY_CUSTODY_ARTIFACTS;
assert.ok(artifacts,'MORIARTY_CUSTODY_ARTIFACTS must identify an existing retained skip-zk custody build; never compile from this test');
const sha=p=>createHash('sha256').update(readFileSync(p)).digest('hex');
const bindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url),'utf8'));
const receipt=JSON.parse(readFileSync(join(artifacts,'build-receipt.json'),'utf8'));
assert.equal(receipt.schema,'moriarty.custody-build/1');assert.equal(receipt.skipZk,true);assert.equal(receipt.keysGenerated,false);assert.equal(receipt.proofsGenerated,false);assert.ok(receipt.commands.every(c=>c.exit===0));
const retainedModules={loan:'514fb8408b5a92b4a4d999b88e475dbf447cab574050417d83405599a991b912',swap:'b64d25359576771789782e2d2bd6ad791c2c49c5028813eaf43debe89154e0d8'};
for(const kind of ['loan','swap']){
 const evidence=receipt.artifacts.find(a=>a.name===kind);assert.ok(evidence);
 assert.equal(evidence.wrapper,sha(new URL(`../custody/${kind}.compact`,import.meta.url)));
 assert.equal(evidence.wrapper,sha(join(artifacts,'stage',kind,kind+'.compact')));
 assert.equal(evidence.kernel,sha(new URL('../../../'+PIN_PATHS[kind+'Kernel'],import.meta.url)));assert.equal(evidence.kernel,bindings.digests[kind+'Kernel']);assert.equal(evidence.kernel,sha(join(artifacts,'stage',kind,'kernel.compact')));
 assert.equal(sha(join(artifacts,'stage',kind,'arithmetic.compact')),bindings.digests.arithmetic);
 assert.equal(sha(new URL('../../../'+PIN_PATHS.arithmetic,import.meta.url)),bindings.digests.arithmetic);
 assert.equal(sha(join(artifacts,kind,'contract/index.js')),retainedModules[kind]);
 const metadata=JSON.parse(readFileSync(join(artifacts,kind,'compiler/contract-info.json'),'utf8'));
 assert.equal(metadata['compiler-version'],'0.31.1');assert.equal(metadata['language-version'],'0.23.0');assert.equal(metadata['runtime-version'],'0.16.0');
}
const runtime=await import(pathToFileURL(join(artifacts,'loan/node_modules/@midnight-ntwrk/compact-runtime/dist/index.js')).href);
const modules={};for(const kind of ['loan','swap'])modules[kind]=await import(pathToFileURL(join(artifacts,kind,'contract/index.js')).href);
const h=n=>n.toString(16).padStart(64,'0'),bytes=x=>Uint8Array.from(Buffer.from(x,'hex'));
const roles={firstSecret:bytes(h(8291)),secondSecret:bytes(h(9373)),firstAddress:h(48271),secondAddress:h(58273)};
const address=h(78191),networkTag=h(99299),TIME=1700000000n,B=1n<<64n;
const mul=(a,b)=>({aLo:a%B,aHi:a/B,bLo:b%B,bHi:b/B,lo:((a%B)*(b%B))%B,carry:((a%B)*(b%B))/B});
const div=(n,d)=>({q:n/d,r:n%d,product:mul(d,n/d)});
function fullLedger(mod,state){const x=mod.ledger(state.data??state);return Object.fromEntries(Object.keys(x).map(k=>[k,x[k]]));}
function context(state,reserves){
 const c=runtime.createCircuitContext(address,'00'.repeat(32),state,{},undefined,undefined,Number(TIME));
 c.currentQueryContext.block={...c.currentQueryContext.block,secondsSinceEpoch:TIME,secondsSinceEpochErr:0,lastBlockTime:TIME,parentBlockHash:'00'.repeat(32),balance:new Map(Object.entries(reserves).map(([raw,n])=>[{tag:'unshielded',raw},n]))};
 return c;
}
function buildObservedTrace(kind){
 const mod=modules[kind],contract=new mod.Contract({}),program=bytes(bindings[kind].programDigest),net=bytes(networkTag);
 const deployed=contract.initialState(runtime.createConstructorContext({},'00'.repeat(32)),roles.firstSecret,roles.secondSecret,{bytes:bytes(roles.firstAddress)},{bytes:bytes(roles.secondAddress)},program,net);
 let state=deployed.currentContractState;const reserves={},rows=[];
 function record(stage,result){
  const i=rows.length,effects=result?projectTranscriptEffects(result.context.currentQueryContext.effects):null;
  const inputs=[],outputs=[];let index=0;
  // Minimal synthetic wallet input exactly covers the actual receive transcript.
  // No expectation fixture supplies financial leaves to this projection.
  for(const [type,value] of Object.entries(effects?.unshieldedInputs??{})){
   // mintUnshieldedToken(...self) is also an input to the contract transcript.
   const selfMint=(effects.claimedUnshieldedSpends??[]).filter(c=>c.type===type&&c.recipientKind==='contract'&&c.recipient===address).reduce((n,c)=>n+BigInt(c.amount),0n);
   if(selfMint){const minted=Object.entries(effects.unshieldedMints).filter(([domain])=>runtime.rawTokenType(bytes(domain),address)===type).reduce((n,[,v])=>n+BigInt(v),0n);assert.ok(minted>=selfMint);}
   const external=BigInt(value)-selfMint;assert.ok(external>=0n);if(external)inputs.push({segment:1,section:'guaranteed',owner:roles.firstAddress,type,value:external.toString(),intentHash:h(30000+i),outputNo:inputs.length});
  }
  for(const claim of effects?.claimedUnshieldedSpends??[]){
   if(claim.recipientKind==='user')outputs.push({segment:1,section:'guaranteed',owner:claim.recipient,type:claim.type,value:claim.amount,intentHash:h(40000+i),offerIndex:index++});
   else assert.equal(claim.recipient,address);
  }
  for(const [type,value] of Object.entries(effects?.unshieldedInputs??{}))reserves[type]=(reserves[type]??0n)+BigInt(value);
  for(const [type,value] of Object.entries(effects?.unshieldedOutputs??{}))reserves[type]=(reserves[type]??0n)-BigInt(value);
  assert.ok(Object.values(reserves).every(n=>n>=0n));
  const txId=h(10000+i),action=result?{segment:1,kind:'call',address,entryPoint:stage,transcripts:[{section:'guaranteed',effects}]}:{segment:1,kind:'deploy',address};
  const ids=[txId,h(20000+i)];
  const r={schema:'moriarty.finalized-financial-stage/1',txId,contractAddress:address,circuitId:stage,transaction:{schema:'moriarty.native-financial-transaction/1',rawSha256:h(50000+i),transactionHash:h(60000+i),identifiers:ids,inputs,outputs,actions:[action],grossByAsset:Object.fromEntries(inputs.map(x=>[x.type,x.value])),dustFee:'31',proofVerified:false,ledgerAccepted:false},blockHash:h(70000+i),blockHeight:100+i,finalizedHead:'0x'+h(70000+i),finalizedHeight:100+i,protocolVersion:1000000,indexerIdentifiers:[...ids],fees:{nativeDebit:{asset:'DUST',unit:'SPECK',amount:'31'},indexerReported:{paid:'1',estimated:'1',sourceUnitLabel:'DUST',encoding:'unresolved',nativeDebitRelationship:'unresolved'}},contractBalances:Object.fromEntries(Object.entries(reserves).map(([k,v])=>[k,v.toString()])),acceptance:'uncertified-I2-observation'};
  rows.push({stage,observation:{receipt:r,state:fullLedger(mod,state)}});
 }
 record('deploy');
 function call(stage,args){const result=contract.circuits[stage](context(state,reserves),...args);state=result.context.currentQueryContext.state;record(stage,result);}
 if(kind==='loan'){
  call('initialize',[roles.firstSecret,program,net,2n,TIME]);
  call('accrue',[roles.firstSecret,program,net,0n,2n,TIME,{h0:mul(5000000000n,8n),h1:mul(40000000000n,31n),h2:mul(100n,365n),h3:div(1240000000000n,36500n)}]);
  call('settle',[roles.firstSecret,program,net,1n,2n,0n,533972602n,TIME,{unused:0n}]);
 }else{
  call('initialize',[roles.secondSecret,program,net,3n,TIME]);
  call('swap',[roles.firstSecret,program,net,0n,4n,4n,0n,1n,10000n,19700n,TIME,{h0:mul(10000n,997n),h1:mul(9970000n,2000000n),h2:mul(1000000n,1000n),h3:div(19940000000000n,1009970000n)}]);
  call('close',[roles.secondSecret,program,net,1n,3n,TIME,{unused:0n}]);
 }
 return rows;
}
for(const kind of ['loan','swap'])test(`${kind} retained generated state and transcript effects match independent comparator (synthetic transport only)`,async()=>{
 const rows=buildObservedTrace(kind),comparator=await createFinancialComparator({kind,roles,networkTag,expectedProtocolVersion:1000000});
 for(const {stage,observation} of rows)assert.equal(comparator.verifyStage(stage,observation).status,'PASS');
 const result=comparator.finish();assert.equal(result.status,'PASS');assert.equal(result.networkAcceptance,false);assert.equal(result.proofAcceptance,false);
 assert.equal(rows.at(-1).observation.state.kernelState.f0,kind==='loan'?4500000000n:0n);
 assert.equal(rows.at(-1).observation.state.remaining,kind==='loan'?0n:6n);
});
test('actual generated financial state and actual transcript payout mutations are rejected',async()=>{
 for(const mutate of [o=>o.state.kernelState.f0=0n,o=>o.receipt.transaction.actions[0].transcripts[0].effects.claimedUnshieldedSpends[0].recipient=roles.firstAddress]){
  const rows=buildObservedTrace('loan'),c=await createFinancialComparator({kind:'loan',roles,networkTag,expectedProtocolVersion:1000000});
  for(const row of rows.slice(0,3))c.verifyStage(row.stage,row.observation);
  mutate(rows[3].observation);assert.throws(()=>c.verifyStage('settle',rows[3].observation));
 }
});
