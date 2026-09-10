import {writeFileSync,readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const O='/tmp/moriarty-preview-readiness-20260910/',url='https://rpc.preview.midnight.network/';
if(!readFileSync(O+'node-endpoints.txt','utf8').includes('`'+url+'`'))throw Error('ENDPOINT_NOT_DOCUMENTED');
const receipts=[];
for(const [index,method,params] of [[1,'chain_getBlockHash',[0]],[2,'chain_getFinalizedHead',[]]]){
 const request={jsonrpc:'2.0',id:index,method,params},started=Date.now(),record={requestedUrl:url,method,request,startedAt:new Date(started).toISOString(),maxMilliseconds:15000,maxResponseBytes:65536,retries:0,credentialUse:false};
 writeFileSync(O+index+'-request.json',JSON.stringify(record,null,2)+'\n',{flag:'wx'});
 try{
  const response=await fetch(url,{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify(request),redirect:'error',credentials:'omit',signal:AbortSignal.timeout(15000)});record.httpStatus=response.status;record.finalUrl=response.url;
  const reader=response.body.getReader(),chunks=[];let size=0;
  while(true){const {done,value}=await reader.read();if(Date.now()-started>=15000)throw Error('ABSOLUTE_DEADLINE');if(done)break;size+=value.byteLength;if(size>65536){await reader.cancel();throw Error('RESPONSE_SIZE_LIMIT');}chunks.push(value);}
  const raw=Buffer.concat(chunks);record.bytes=raw.length;record.sha256=createHash('sha256').update(raw).digest('hex');writeFileSync(O+index+'-response.json',raw,{flag:'wx'});const body=JSON.parse(raw);
  if(!response.ok||body.jsonrpc!=='2.0'||body.id!==index||body.error||!/^0x[a-f0-9]{64}$/.test(body.result))throw Error('RPC_RESPONSE_NOT_HASH');
  record.status='OBSERVED';record.result=body.result;
 }catch(error){record.status='UNAVAILABLE_OR_UNKNOWN';record.errorClass=error.name;record.errorCode=/^[A-Z][A-Z0-9_]{0,95}$/.test(error.message)?error.message:'BOUNDED_REQUEST_FAILED';}
 record.elapsedMs=Date.now()-started;record.completedAt=new Date().toISOString();receipts.push(record);writeFileSync(O+index+'-receipt.json',JSON.stringify(record,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify(record));
}
writeFileSync(O+'rpc-receipts.json',JSON.stringify(receipts,null,2)+'\n',{flag:'wx'});
