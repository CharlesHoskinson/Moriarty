/** Local startup/balance readiness. These conservative age/lag limits are local
 * policy, not consensus guarantees. No wallet construction or submission retry.
 */
import {createLocalRpc} from './integrate-local.mjs';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const hash=x=>typeof x==='string'&&/^[a-f0-9]{64}$/.test(x);
function endpoint(value){const u=new URL(value);check(u.protocol==='http:'&&['127.0.0.1','[::1]'].includes(u.hostname)&&!u.username&&!u.password&&!u.hash,'TIP_LOCAL_ENDPOINT');return u.href;}
async function indexedBlock(indexer,deadlineMs,fetchImpl){
 const controller=new AbortController();let timer;
 const operation=(async()=>{
  const response=await fetchImpl(indexer,{method:'POST',redirect:'error',headers:{'content-type':'application/json'},body:JSON.stringify({query:'query($offset: BlockOffset) { block(offset: $offset) { height hash timestamp } }',variables:{offset:null}}),signal:controller.signal});
  check(response.ok===true&&response.body,'TIP_HTTP');const reader=response.body.getReader(),chunks=[];let size=0;
  try{while(true){const {done,value}=await reader.read();if(done)break;size+=value.byteLength;check(size<=65536,'TIP_BODY_LIMIT');chunks.push(Buffer.from(value));}}finally{reader.releaseLock();}
  const body=JSON.parse(Buffer.concat(chunks).toString('utf8'));check(!body.errors&&body.data?.block,'TIP_RESPONSE');
  return body.data.block;
 })();
 try{return await Promise.race([operation,new Promise((_,reject)=>{timer=setTimeout(()=>{controller.abort();reject(Error('TIP_REQUEST_DEADLINE'));},Math.max(1,deadlineMs-Date.now()));})]);}
 finally{clearTimeout(timer);controller.abort();}
}
/** At most 60 read-only samples, one-second spacing, 60 seconds total and no
 * extension of the caller's existing wall deadline. A hung request is aborted.
 */
export async function waitForLocalTip({node,indexer,deadlineMs,fetchImpl=globalThis.fetch}){
 node=endpoint(node);indexer=endpoint(indexer);
 check(Number.isSafeInteger(deadlineMs)&&deadlineMs>Date.now()&&typeof fetchImpl==='function','TIP_DEADLINE');
 const stop=Math.min(deadlineMs,Date.now()+60000);
 for(let attempt=0;attempt<60&&Date.now()<stop;attempt++){
  try{
   const requestDeadline=Math.min(stop,Date.now()+5000);
   const block=await indexedBlock(indexer,requestDeadline,fetchImpl);
   check(Number.isSafeInteger(block.height)&&block.height>=0&&hash(block.hash),'TIP_BLOCK');
   // Match the SDK's Date conversion exactly; never scale seconds heuristically.
   check(typeof block.timestamp==='number'||typeof block.timestamp==='string','TIP_TIMESTAMP');
   const timestampMs=new Date(block.timestamp).getTime();
   check(Number.isSafeInteger(timestampMs)&&timestampMs<=Date.now()&&Date.now()-timestampMs<=60000,'TIP_STALE');
   const rpc=createLocalRpc({node,deadlineMs:requestDeadline,fetchImpl});
   const finalizedHash=await rpc('chain_getFinalizedHead',[]);
   check(typeof finalizedHash==='string'&&/^0x[a-f0-9]{64}$/.test(finalizedHash),'TIP_FINALIZED_HASH');
   const header=await rpc('chain_getHeader',[finalizedHash]);
   check(typeof header?.number==='string'&&/^0x[0-9a-f]+$/i.test(header.number),'TIP_HEADER');
   const finalizedHeight=Number(BigInt(header.number));
   check(Number.isSafeInteger(finalizedHeight)&&Math.abs(block.height-finalizedHeight)<=2,'TIP_LAG');
   check(await rpc('chain_getBlockHash',[block.height])==='0x'+block.hash,'TIP_CHAIN_MISMATCH');
   const checkedAt=Date.now();check(checkedAt<stop&&timestampMs<=checkedAt&&checkedAt-timestampMs<=60000,'TIP_STALE');
   return {schema:'moriarty.local-tip-readiness/1',height:block.height,hash:block.hash,timestampMs,finalizedHeight,finalizedHash,checkedAt,status:'READY',scope:'Recent indexed canonical block near finalized head; not transaction finality or consensus validity'};
  }catch{/* Only read-only readiness probes repeat; wallet operations never retry. */}
  if(Date.now()<stop)await new Promise(resolve=>setTimeout(resolve,Math.min(1000,stop-Date.now())));
 }
 throw Error('TIP_NOT_READY');
}
function dustTime(transactions,minTimestamp=0){
 const now=Date.now();
 for(const tx of transactions){
  check(tx?.intents instanceof Map,'TIP_NATIVE_INTENTS');
  for(const intent of tx.intents.values()){
   const dust=intent.dustActions;if(!dust)continue;
   const time=dust.ctime instanceof Date?dust.ctime.getTime():NaN;
   check(Number.isSafeInteger(time)&&time<=now&&now-time<=60000&&time>=minTimestamp,'TIP_DUST_TIME');
  }
 }
}
/** Guard the fixed SDK facade without changing its time source or saved state.
 * Native recipe/type/authorization checks remain mandatory in providers.mjs.
 */
export function guardLocalWallet({wallet,onTip=()=>{},...options}){
 check(wallet&&typeof onTip==='function','TIP_WALLET');
 return new Proxy(wallet,{get(target,key){
  if(key==='balanceUnboundTransaction')return async(...args)=>{
   const tip=await waitForLocalTip(options);onTip(tip);
   check(Date.now()<options.deadlineMs,'TIP_DEADLINE');
   const recipe=await target.balanceUnboundTransaction(...args);
   check(recipe?.type==='UNBOUND_TRANSACTION','TIP_RECIPE');
   dustTime([recipe.baseTransaction,...(recipe.balancingTransaction===undefined?[]:[recipe.balancingTransaction])],Math.floor(tip.timestampMs/1000)*1000);
   check(Date.now()<options.deadlineMs,'TIP_DEADLINE');return recipe;
  };
  if(key==='submitTransaction')return async tx=>{check(Date.now()<options.deadlineMs,'TIP_DEADLINE');dustTime([tx]);return target.submitTransaction(tx);};
  const value=Reflect.get(target,key,target);return typeof value==='function'?value.bind(target):value;
 }});
}
