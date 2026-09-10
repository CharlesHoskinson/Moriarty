import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const dir='/tmp/moriarty-preview-protocol-20260910/',hash=b=>createHash('sha256').update(b).digest('hex');
const sourcePath='/home/charl/.local/state/moriarty/sp05-public-dust-fixture/request.json',source=readFileSync(sourcePath),prior=JSON.parse(source);
const url='https://indexer.preview.midnight.network/api/v4/graphql';
if(prior.url!==url||prior.body.variables.offset.identifier!=='0007458033e8be29bd7ea582bce27b817b85f638c57169e8f27283399e12c36221')throw Error('SOURCE_BINDING');
const body={query:'query($offset: TransactionOffset!) { transactions(offset:$offset) { protocolVersion hash block {height hash} ... on RegularTransaction {transactionResult {status}} } }',variables:prior.body.variables};
writeFileSync(dir+'request.json',JSON.stringify({url,body},null,2)+'\n',{flag:'wx'});
const start=Date.now(),deadline=start+15000,controller=new AbortController();
const receipt={startedAt:new Date(start).toISOString(),url,sourceRequestPath:sourcePath,sourceRequestSha256:hash(source),maxMilliseconds:15000,maxResponseBytes:65536,requests:1,retries:0,credentials:false};
let timer;
try{
 const operation=(async()=>{
  const response=await fetch(url,{method:'POST',redirect:'error',headers:{'content-type':'application/json'},body:JSON.stringify(body),signal:controller.signal});
  if(Date.now()>=deadline)throw Error('DEADLINE');receipt.httpStatus=response.status;
  const reader=response.body.getReader(),chunks=[];let size=0;
  try{for(;;){const {done,value}=await reader.read();if(Date.now()>=deadline)throw Error('DEADLINE');if(done)break;size+=value.byteLength;if(size>65536)throw Error('BODY_LIMIT');chunks.push(Buffer.from(value));}}finally{reader.releaseLock();}
  const raw=Buffer.concat(chunks);writeFileSync(dir+'response.json',raw,{flag:'wx'});receipt.bytes=raw.length;receipt.responseSha256=hash(raw);
  const parsed=JSON.parse(raw);if(!response.ok)throw Error('HTTP_STATUS');if(parsed.errors)throw Error('GRAPHQL_ERROR');
  receipt.rows=parsed.data?.transactions;receipt.status='HISTORICAL_RESPONSE_OBSERVED';
 })();
 await Promise.race([operation,new Promise((_,reject)=>{timer=setTimeout(()=>{controller.abort();reject(Error('DEADLINE'));},Math.max(1,deadline-Date.now()));})]);
 if(Date.now()>=deadline)throw Error('DEADLINE');
}catch(e){receipt.status='FAILED';receipt.errorClass=e.name;receipt.errorCode=['DEADLINE','BODY_LIMIT','HTTP_STATUS','GRAPHQL_ERROR'].includes(e.message)?e.message:'UNCLASSIFIED_PUBLIC_REQUEST_FAILURE';}
finally{clearTimeout(timer);controller.abort();receipt.elapsedMs=Date.now()-start;receipt.completedAt=new Date().toISOString();writeFileSync(dir+'receipt.json',JSON.stringify(receipt,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify(receipt));}
