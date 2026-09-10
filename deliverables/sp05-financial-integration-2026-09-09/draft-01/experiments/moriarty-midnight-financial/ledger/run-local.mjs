import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';

const bindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url),'utf8'));
const BASE=1n<<64n;
function mul(a,b) {return {aLo:a%BASE,aHi:a/BASE,bLo:b%BASE,bHi:b/BASE,lo:(a%BASE)*(b%BASE)%BASE,carry:(a%BASE)*(b%BASE)/BASE};}
function div(n,d) {const q=n/d;return {q,r:n%d,product:mul(d,q)};}
function bytes(hex) {if(typeof hex!=='string'||!/^[0-9a-f]{64}$/.test(hex))throw Error('INVALID_PUBLIC_BINDING');return Uint8Array.from(Buffer.from(hex,'hex'));}
function secret(x) {if(!(x instanceof Uint8Array)||x.length!==32)throw Error('INVALID_ROLE_SECRET');return x;}
function time(now) {const t=now();if(typeof t!=='bigint'||t<0n||t>=2000000000n)throw Error('INVALID_CURRENT_TIME');return t;}
async function contractsSdk() {
  const root=join(PINNED_NM,'@midnight-ntwrk/midnight-js-contracts');
  if(JSON.parse(readFileSync(join(root,'package.json'),'utf8')).version!=='4.1.1')throw Error('CONTRACTS_SDK_VERSION');
  return import(pathToFileURL(join(root,'dist/index.mjs')).href);
}

/**
 * Executes the fixed local I2 sequence. Caller must supply the reviewed native
 * observer and full financial comparator; neither can be omitted. No CLI or
 * public dispatch is exposed until the complete integration is reviewed.
 * SDK results contain private data and are never returned or logged wholesale.
 */
export async function runLocalFinancialCase({kind,network,providers,compiledContract,roles,networkTag,now,observe,verifyStage,sdk}) {
  if(network!=='undeployed')throw Error('LOCAL_DRIVER_REQUIRES_UNDEPLOYED_NETWORK');
  if(!['loan','swap'].includes(kind))throw Error('UNKNOWN_FINANCIAL_CASE');
  if(typeof observe!=='function'||typeof verifyStage!=='function'||typeof now!=='function')throw Error('OBSERVATION_AND_FINANCIAL_COMPARATOR_REQUIRED');
  if(!providers||typeof providers.execute!=='function'||typeof providers.cleanup!=='function'||typeof providers.stop!=='function')throw Error('GUARDED_PROVIDERS_REQUIRED');
  const first=secret(roles.firstSecret),second=secret(roles.secondSecret);
  const firstAddress=bytes(roles.firstAddress),secondAddress=bytes(roles.secondAddress);
  if(roles.firstAddress===roles.secondAddress)throw Error('DISTINCT_FINANCIAL_PARTICIPANTS_REQUIRED');
  const program=bytes(bindings[kind].programDigest),net=bytes(networkTag);
  const stages=[];let address;
  try {
    sdk??=await contractsSdk();
    const deployed=await providers.execute('deploy',()=>sdk.deployContract(providers,{compiledContract,privateStateId:`sp05-${kind}`,initialPrivateState:{},args:[first,second,{bytes:firstAddress},{bytes:secondAddress},program,net]}));
    address=deployed.deployTxData.public.contractAddress;bytes(address);
    async function record(circuitId,txId) {
      if(typeof txId!=='string'||!txId)throw Error('MISSING_TRANSACTION_ID');
      const observation=await providers.execute('observe:'+circuitId,()=>observe({circuitId,txId,contractAddress:address}));
      await providers.execute('compare:'+circuitId,()=>verifyStage(circuitId,observation));
      stages.push(observation.receipt);
    }
    await record('deploy',deployed.deployTxData.public.txId);
    async function call(circuitId,args) {
      const result=await providers.execute(circuitId,()=>sdk.submitCallTx(providers,{compiledContract,contractAddress:address,privateStateId:`sp05-${kind}`,circuitId,args}));
      await record(circuitId,result.public.txId);
    }
    if(kind==='loan') {
      await call('initialize',[first,program,net,2n,time(now)]);
      await call('accrue',[first,program,net,0n,2n,time(now),{h0:mul(5000000000n,8n),h1:mul(40000000000n,31n),h2:mul(100n,365n),h3:div(1240000000000n,36500n)}]);
      await call('settle',[first,program,net,1n,2n,0n,533972602n,time(now),{unused:0n}]);
    } else {
      await call('initialize',[second,program,net,3n,time(now)]);
      await call('swap',[first,program,net,0n,4n,4n,0n,1n,10000n,19700n,time(now),{h0:mul(10000n,997n),h1:mul(9970000n,2000000n),h2:mul(1000000n,1000n),h3:div(19940000000000n,1009970000n)}]);
      await call('close',[second,program,net,1n,3n,time(now),{unused:0n}]);
    }
    return {schema:'moriarty.local-financial-run/1',kind,contractAddress:address,stages,scope:'Fixed I2 execution and supplied financial comparison; not mandatory PCD acceptance'};
  } catch(error) {providers.stop(error.message);throw error;}
  finally {await providers.cleanup();}
}
