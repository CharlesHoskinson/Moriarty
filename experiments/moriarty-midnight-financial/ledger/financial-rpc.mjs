const requireThat=(ok,code)=>{if(!ok)throw Error(code);};
function validDeadline(value){requireThat(Number.isSafeInteger(value)&&value>Date.now(),'INTEGRATION_DEADLINE');}
/** Existing local endpoint policy, unchanged by Preview support. */
export function createLocalRpc({node,deadlineMs,fetchImpl=globalThis.fetch}){
 const url=new URL(node);
 requireThat(url.protocol==='http:'&&['127.0.0.1','localhost','[::1]'].includes(url.hostname)&&!url.username&&!url.password&&!url.hash,'INTEGRATION_LOCAL_ENDPOINT');
 return createReadOnlyRpc({endpoint:url.href,deadlineMs,fetchImpl});
}
/** Source-configured Preview target, not evidence of current endpoint availability.
 * Callers must separately verify genesis, protocol and canonical state identity.
 */
export function createPreviewRpc({node,deadlineMs,fetchImpl=globalThis.fetch}){
 requireThat(node==='https://rpc.preview.midnight.network'||node==='https://rpc.preview.midnight.network/','PREVIEW_RPC_ENDPOINT');
 return createReadOnlyRpc({endpoint:'https://rpc.preview.midnight.network/',deadlineMs,fetchImpl});
}
/** One read request, no redirects/retries, 64KiB body ceiling. */
function createReadOnlyRpc({endpoint,deadlineMs,fetchImpl=globalThis.fetch}){
  validDeadline(deadlineMs);requireThat(typeof fetchImpl==='function','RPC_FETCH_REQUIRED');
  let nextId=0;
  return async(method,params)=>{
    requireThat(['chain_getFinalizedHead','chain_getHeader','chain_getBlockHash','midnight_contractState'].includes(method)&&Array.isArray(params),'RPC_METHOD');
    const validParams=method==='midnight_contractState'
      ?params.length===2&&typeof params[0]==='string'&&/^[a-f0-9]{64}$/.test(params[0])&&typeof params[1]==='string'&&/^0x[a-f0-9]{64}$/.test(params[1])
      :method==='chain_getFinalizedHead'?params.length===0:method==='chain_getHeader'?params.length===1&&typeof params[0]==='string'&&/^0x[a-f0-9]{64}$/i.test(params[0]):params.length===1&&Number.isSafeInteger(params[0])&&params[0]>=0;
    requireThat(validParams,'RPC_PARAMS');
    validDeadline(deadlineMs);const id=++nextId,controller=new AbortController();let timer;
    const operation=(async()=>{
      const response=await fetchImpl(endpoint,{method:'POST',redirect:'error',headers:{'content-type':'application/json'},body:JSON.stringify({jsonrpc:'2.0',id,method,params}),signal:controller.signal});
      requireThat(Date.now()<deadlineMs,'RPC_DEADLINE');
      requireThat(response.ok===true&&response.body,'RPC_HTTP_STATUS');
      const reader=response.body.getReader(),chunks=[];let size=0;
      try{while(true){const {done,value}=await reader.read();requireThat(Date.now()<deadlineMs,'RPC_DEADLINE');if(done)break;size+=value.byteLength;if(size>65536){controller.abort();throw Error('RPC_BODY_LIMIT');}chunks.push(Buffer.from(value));}}
      finally{reader.releaseLock();}
      const body=JSON.parse(Buffer.concat(chunks).toString('utf8'));
      requireThat(body?.jsonrpc==='2.0'&&body.id===id&&Object.hasOwn(body,'result')&&!Object.hasOwn(body,'error'),'RPC_RESPONSE');
      return body.result;
    })();
    try{
      const result=await Promise.race([operation,new Promise((_,reject)=>{timer=setTimeout(()=>{controller.abort();reject(Error('RPC_DEADLINE'));},Math.min(deadlineMs-Date.now(),2147483647));})]);
      // A blocked event loop can let the response win before an overdue timer.
      requireThat(Date.now()<deadlineMs,'RPC_DEADLINE');
      return result;
    }
    finally{clearTimeout(timer);controller.abort();}
  };
}
