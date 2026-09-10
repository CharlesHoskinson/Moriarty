import {writeFileSync,readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const dir=new URL('./',import.meta.url),rpcUrl='https://rpc.preview.midnight.network',indexer='https://indexer.preview.midnight.network/api/v4/graphql';
const receipts=[];let serial=0;
async function request(url,body){
 const id=++serial,started=Date.now(),receipt={id,url,body,startedAt:new Date(started).toISOString(),deadlineMs:started+15000,maxBytes:65536,retries:0,privateInput:false};
 writeFileSync(new URL(`${id}-request.json`,dir),JSON.stringify(receipt,null,2)+'\n',{flag:'wx'});
 try{
  const response=await fetch(url,{method:'POST',redirect:'error',credentials:'omit',headers:{'content-type':'application/json'},body:JSON.stringify(body),signal:AbortSignal.timeout(15000)});
  receipt.httpStatus=response.status;const reader=response.body.getReader(),chunks=[];let size=0;
  while(true){const {value,done}=await reader.read();if(Date.now()>=receipt.deadlineMs)throw Error('DEADLINE');if(done)break;size+=value.length;if(size>65536){await reader.cancel();throw Error('BODY_LIMIT');}chunks.push(value);}
  const raw=Buffer.concat(chunks);writeFileSync(new URL(`${id}-response.json`,dir),raw,{flag:'wx'});receipt.bytes=raw.length;receipt.sha256=createHash('sha256').update(raw).digest('hex');
  const value=JSON.parse(raw);if(!response.ok||value.error||value.errors)throw Error('RESPONSE_REJECTED');if(Date.now()>=receipt.deadlineMs)throw Error('DEADLINE');receipt.status='OBSERVED';return value;
 }catch(error){receipt.status='UNKNOWN';receipt.errorClass=error.name;throw error;}
 finally{receipt.elapsedMs=Date.now()-started;receipts.push(receipt);writeFileSync(new URL(`${id}-receipt.json`,dir),JSON.stringify(receipt,null,2)+'\n',{flag:'wx'});}
}
async function rpc(method,params=[]){const id=serial+1,v=await request(rpcUrl,{jsonrpc:'2.0',id,method,params});if(v.jsonrpc!=='2.0'||v.id!==id||!Object.hasOwn(v,'result'))throw Error('RPC_ENVELOPE');return v.result;}
let result;
try{
 const finalizedHash=await rpc('chain_getFinalizedHead');if(!/^0x[a-f0-9]{64}$/.test(finalizedHash))throw Error('FINALIZED_HASH');
 const header=await rpc('chain_getHeader',[finalizedHash]);if(!/^0x[a-f0-9]+$/.test(header.number))throw Error('HEADER');const height=Number.parseInt(header.number,16);if(!Number.isSafeInteger(height))throw Error('HEIGHT');
 const body=await request(indexer,{query:'query($offset: BlockOffset) { block(offset: $offset) { height hash protocolVersion timestamp } }',variables:{offset:{height}}});
 const block=body.data?.block;const canonical=await rpc('chain_getBlockHash',[height]);
 if(block?.height!==height||'0x'+block.hash!==finalizedHash||canonical!==finalizedHash||!Number.isSafeInteger(block.protocolVersion))throw Error('BLOCK_BINDING');
 result={status:'OBSERVED',checkedAt:new Date().toISOString(),scope:'Read-only trusted Preview node/indexer metadata at sampled finalized block; no transaction, wallet, proof or financial acceptance.',finalizedHash,height,indexedBlock:block,canonicalHash:canonical,requests:serial};
}catch(error){result={status:'UNKNOWN',checkedAt:new Date().toISOString(),errorClass:error.name,requests:serial,scope:'Read-only metadata probe failed; no financial acceptance.'};}
result.sourceSha256=createHash('sha256').update(readFileSync(new URL(import.meta.url))).digest('hex');
writeFileSync(new URL('result.json',dir),JSON.stringify(result,null,2)+'\n',{flag:'wx'});writeFileSync(new URL('receipts.json',dir),JSON.stringify(receipts,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify(result));
