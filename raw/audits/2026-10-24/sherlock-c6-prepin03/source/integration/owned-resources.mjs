// Synchronous producer-local ownership only. No SDK imports or live resources.
// Every acquired object is registered before inspection. Caller-owned objects
// are excluded by identity. Cleanup drains distinct acquisitions independently.
export class ProducerCleanupError extends AggregateError {
  constructor(primaryPresent,primary,cleanupErrors) {
    super(primaryPresent?[primary,...cleanupErrors]:cleanupErrors,
      primaryPresent?'ERR_PRODUCER_PRIMARY_AND_RESOURCE_CLEANUP':'ERR_PRODUCER_RESOURCE_CLEANUP',
      primaryPresent?{cause:primary}:undefined);
    this.primaryPresent=primaryPresent;
  }
}
export function producerFailure(error) {
  let cleanup=false;const visited=new Set();
  // Scope nesting is finite: shape/fee -> dry run -> pre-sign. Unwrap only our
  // local type, never arbitrary SDK error getters or untrusted cause chains.
  while(error instanceof ProducerCleanupError&&!visited.has(error)) {
    visited.add(error);
    cleanup=true;if(!error.primaryPresent)break;error=error.cause;
  }
  return {primary:error,cleanup};
}
export function withOwnedResources(body,{borrowed=[]}={}) {
  const excluded=new Set(borrowed),seen=new Set(),acquired=[];
  const own=value=>{
    if(value!==null&&(typeof value==='object'||typeof value==='function')&&!excluded.has(value)&&!seen.has(value)) {
      seen.add(value);acquired.push(value);
    }
    return value;
  };
  const ownCollection=value=>{
    own(value);
    if(Array.isArray(value))for(const item of value)own(item);
    else if(value instanceof Map)for(const item of value.values())own(item);
    return value;
  };
  let result,primary,primaryPresent=false;
  try {result=body(own,ownCollection);}
  catch(error){primary=error;primaryPresent=true;}
  const cleanupErrors=[];
  for(let i=acquired.length-1;i>=0;i--) {
    try {const free=acquired[i].free;if(typeof free==='function')free.call(acquired[i]);}
    catch(error){cleanupErrors.push(error);}
  }
  if(cleanupErrors.length)throw new ProducerCleanupError(primaryPresent,primary,cleanupErrors);
  if(primaryPresent)throw primary;
  return result;
}
