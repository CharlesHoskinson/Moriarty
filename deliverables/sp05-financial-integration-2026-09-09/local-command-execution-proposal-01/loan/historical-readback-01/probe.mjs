import {isDeepStrictEqual} from 'node:util';
import {readFileSync,openSync,writeFileSync,fsyncSync,closeSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {resolve} from 'node:path';
const ROOT=new URL('../../../../../',import.meta.url),LEDGER=new URL('experiments/moriarty-midnight-financial/ledger/',ROOT);
const {createLocalRpc}=await import(new URL('integrate-local.mjs',LEDGER));
const {loadProvenFinancialContract}=await import(new URL('proven-assets.mjs',LEDGER));
const {extractNativeContractBalances}=await import(new URL('contract-balances.mjs',LEDGER));
const {PINNED_NM}=await import(new URL('providers.mjs',LEDGER));
const {beforeDeadline}=await import(new URL('receipt.mjs',LEDGER));
const {ContractState}=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs'));
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
export const ADDRESS='36e4a923f23495c9b8bbfcaff5a8963cd6efcd28c1498454880059de76a98449',GENESIS='0xe72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
const hash=x=>createHash('sha256').update(x).digest('hex');
const check=(x,code)=>{if(!x)throw Error(code);};
const publicify=x=>typeof x==='bigint'?x.toString():x instanceof Uint8Array?Buffer.from(x).toString('hex'):Array.isArray(x)?x.map(publicify):x&&typeof x==='object'?Object.fromEntries(Object.entries(x).map(([k,v])=>[k,publicify(v)])):x;
export function inspectState({hex,row,summary,loaded,keys,deployInitialHex}){
 check(typeof hex==='string'&&/^(?:[a-f0-9]{2})+$/.test(hex)&&hex.length<=60000,'STATE_ENCODING');const raw=Buffer.from(hex,'hex');let native;
 try{native=ContractState.deserialize(raw);check(Buffer.from(native.serialize()).equals(raw),'STATE_ROUNDTRIP');const state=publicify(structuredClone(loaded.decodeState(native.data)));check(isDeepStrictEqual(state,summary.publicState),'COMPLETE_STATE_MISMATCH');const balances=extractNativeContractBalances({state:native});
  const actual=publicify(balances);for(const asset of new Set([...Object.keys(actual),...Object.keys(summary.contractBalances)]))check(BigInt(actual[asset]??0)===BigInt(summary.contractBalances[asset]??0),'BALANCES_MISMATCH');
  check(isDeepStrictEqual([...native.operations()].sort(),['accrue','initialize','settle']),'OPERATION_SET');const verifierKeys=[];for(const operation of ['accrue','initialize','settle']){const op=native.operation(operation);try{const vk=Buffer.from(op.verifierKey);check(vk.equals(keys[operation]),'VERIFIER_KEY_MISMATCH');verifierKeys.push({operation,sha256:hash(vk)});}finally{op?.free?.();}}
  if(row.stage==='deploy')check(hex===deployInitialHex,'DEPLOY_INITIAL_MISMATCH');check(Buffer.from(native.serialize()).equals(raw),'NATIVE_MUTATED');return {stage:row.stage,blockHash:row.hash,blockHeight:row.height,serializedStateHex:hex,stateSha256:hash(raw),nativeBytes:raw.length,state,balances:actual,verifierKeys,allDecodedFieldsEqual:true,allBalancesEqual:true};
 }finally{native?.free();}
}
export async function captureHistorical({rpc,rows,summaries,loaded,keys,deployInitialHex,deadlineMs}){
 check(Number.isSafeInteger(deadlineMs)&&deadlineMs>Date.now()&&deadlineMs<=Date.now()+60000,'QUERY_DEADLINE');check(rows.length===4&&rows.every((r,i)=>r.stage===['deploy','initialize','accrue','settle'][i]&&r.height===[20455,20459,20463,20467][i]&&/^0x[a-f0-9]{64}$/.test(r.hash)),'STAGE_BINDING');let queries=0;const responses=[];let bytes=0;
 const call=async(method,params)=>{check(Date.now()<deadlineMs&&++queries<=17,'QUERY_BOUND');const value=await beforeDeadline(()=>rpc(method,params,Math.min(deadlineMs,Date.now()+5000)),deadlineMs);check(Date.now()<deadlineMs,'QUERY_DEADLINE');const raw=Buffer.from(JSON.stringify(value));bytes+=raw.length;check(raw.length<=65536&&bytes<=17*65536,'RESPONSE_BOUND');responses.push({method,params,result:value,resultJsonSha256:hash(raw),resultJsonBytes:raw.length});return value;};
 check(await call('chain_getBlockHash',[0])===GENESIS,'GENESIS');const finalizedHash=await call('chain_getFinalizedHead',[]);check(/^0x[a-f0-9]{64}$/.test(finalizedHash),'FINALIZED_HASH');const header=await call('chain_getHeader',[finalizedHash]);check(/^0x[0-9a-f]+$/i.test(header?.number??''),'FINALIZED_HEADER');const finalizedHeight=Number(BigInt(header.number));check(Number.isSafeInteger(finalizedHeight)&&finalizedHeight>=20467,'FINALIZED_HEIGHT');check(await call('chain_getBlockHash',[finalizedHeight])===finalizedHash,'FINALIZED_CANONICAL');const states=[];
 for(const row of rows){check(row.height<=finalizedHeight,'NOT_FINALIZED');check(await call('chain_getBlockHash',[row.height])===row.hash,'HISTORICAL_CANONICAL');const hex=await call('midnight_contractState',[ADDRESS,row.hash]);states.push(inspectState({hex,row,summary:summaries[row.stage],loaded,keys,deployInitialHex}));check(await call('chain_getBlockHash',[row.height])===row.hash,'HISTORICAL_REORG');}
 check(await call('chain_getBlockHash',[finalizedHeight])===finalizedHash,'FINALIZED_REORG');check(Date.now()<deadlineMs,'QUERY_DEADLINE');return {status:'HISTORICAL_STATES_OBSERVED',finalizedHash,finalizedHeight,states,responses,queries,resultBytes:bytes,authenticatedStateProof:false,scope:'Trusted local node exact historical state/canonical-finality observation; no new transaction or proof; complete compiled fields and verifier bytes compared'};
}
export async function probe({setupDeadlineMs,fetchImpl=globalThis.fetch}={}){
 const started=Date.now();check(Number.isSafeInteger(setupDeadlineMs)&&setupDeadlineMs>started&&setupDeadlineMs<=started+120000,'SETUP_DEADLINE');let loaded,reads=0,queryDeadline;const out={status:'UNKNOWN',submissions:0,proofs:0};
 const rpc=async(m,p,deadline)=>{check(++reads<=77,'TOTAL_QUERY_BOUND');return createLocalRpc({node:'http://127.0.0.1:19944',fetchImpl,deadlineMs:Math.min(deadline,Date.now()+5000)})(m,p);};
 try{
  let ready=false;for(let n=0;n<60&&Date.now()<setupDeadlineMs;n++){try{const h=await rpc('chain_getFinalizedHead',[],setupDeadlineMs);check(/^0x[a-f0-9]{64}$/.test(h),'READINESS_HEAD');ready=true;break;}catch(e){if(!(e instanceof TypeError&&e.message==='fetch failed'&&['ECONNREFUSED','UND_ERR_SOCKET'].includes(e.cause?.code)))throw e;await beforeDeadline(()=>new Promise(r=>setTimeout(r,Math.min(1000,Math.max(1,setupDeadlineMs-Date.now()-1)))),setupDeadlineMs);}}check(ready&&Date.now()<setupDeadlineMs,'READINESS_TIMEOUT');const deadlineMs=Date.now()+60000;queryDeadline=deadlineMs;
  const pins=JSON.parse(readFileSync(new URL('./inputs.json',import.meta.url))),records={};for(const [p,sha] of Object.entries(pins.files)){check(p.startsWith('deliverables/')&&!p.split('/').includes('..'),'INPUT_PATH');const raw=readFileSync(new URL(p,ROOT));check(hash(raw)===sha,'INPUT_HASH');records[p]=p.endsWith('.bin')?raw:JSON.parse(raw);}
  const plan=records[pins.planPath];loaded=await beforeDeadline(()=>loadProvenFinancialContract({case:'loan',...plan.build}),deadlineMs);const build=JSON.parse(readFileSync(plan.build.receiptPath));const keys=Object.fromEntries(['accrue','initialize','settle'].map(n=>[n,readFileSync(build.assetsPath+'/keys/'+n+'.verifier')]));let tx;let deployInitialHex;try{tx=ledger.Transaction.deserialize('signature','proof','binding',records[pins.deployRawPath]);const deploy=[...tx.intents.values()].flatMap(i=>i.actions).find(a=>a.address===ADDRESS);check(deploy,'DEPLOY_ACTION');deployInitialHex=Buffer.from(deploy.initialState.serialize()).toString('hex');}finally{tx?.free?.();}
  Object.assign(out,await captureHistorical({rpc,rows:pins.rows,summaries:Object.fromEntries(pins.rows.map(r=>[r.stage,records[r.summaryPath]])),loaded,keys,deployInitialHex,deadlineMs}));check(Date.now()<deadlineMs,'QUERY_DEADLINE');
 }catch(e){out.status='UNKNOWN';out.failure={code:typeof e?.message==='string'&&/^[A-Z][A-Z0-9_]{0,95}$/.test(e.message)?e.message:'PUBLIC_READBACK_FAILURE'};}
 finally{if(loaded){try{check(loaded.cleanup()?.loaderHooksRemoved===true,'LOADER_CLEANUP');out.loaderClosed=true;}catch{out.status='UNKNOWN';out.failure={code:'LOADER_CLEANUP'};}}if(queryDeadline&&Date.now()>=queryDeadline){out.status='UNKNOWN';out.failure={code:'QUERY_DEADLINE'};}out.totalQueries=reads;out.elapsedMs=Date.now()-started;}
 return out;
}
function retain(result){const bytes=Buffer.from(JSON.stringify(result,null,2)+'\n');check(bytes.length<=524288,'OUTPUT_BOUND');const fd=openSync(new URL('./probe-result.json',import.meta.url),constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,bytes);fsyncSync(fd);}finally{closeSync(fd);}const dir=openSync(new URL('.',import.meta.url),constants.O_RDONLY|constants.O_DIRECTORY);try{fsyncSync(dir);}finally{closeSync(dir);}}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url){const setupDeadlineMs=Number(process.argv[2]);const timer=setTimeout(()=>process.exit(2),180000);try{const result=await probe({setupDeadlineMs});retain(result);process.exitCode=result.status==='HISTORICAL_STATES_OBSERVED'?0:2;}finally{clearTimeout(timer);}}
