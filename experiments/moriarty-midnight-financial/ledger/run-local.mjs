import {EXISTING_SWAP,INITIALIZED_SWAP} from './continue-initialized-swap.mjs';
import {CONTRACT_BALANCE_FAILURE_CODES} from './contract-balances.mjs';
import {readFileSync} from 'node:fs';
import {loadFinancialContractsSdk} from './prepare-deployment.mjs';
import {INITIALIZED_LOAN} from './continue-initialized-loan.mjs';
import {SWAP_WALLET_FAILURE_CODES} from './swap-wallet.mjs';

const bindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url),'utf8'));
const BASE=1n<<64n;
function mul(a,b) {return {aLo:a%BASE,aHi:a/BASE,bLo:b%BASE,bHi:b/BASE,lo:(a%BASE)*(b%BASE)%BASE,carry:(a%BASE)*(b%BASE)/BASE};}
function div(n,d) {const q=n/d;return {q,r:n%d,product:mul(d,q)};}
function bytes(hex) {if(typeof hex!=='string'||!/^[0-9a-f]{64}$/.test(hex))throw Error('INVALID_PUBLIC_BINDING');return Uint8Array.from(Buffer.from(hex,'hex'));}
function secret(x) {if(!(x instanceof Uint8Array)||x.length!==32)throw Error('INVALID_ROLE_SECRET');return Uint8Array.from(x);}
function time(now) {const t=now();if(typeof t!=='bigint'||t<0n||t>=2000000000n)throw Error('INVALID_CURRENT_TIME');return t;}


const publicFailureCodes=new Set(['INVALID_INITIALIZED_SWAP','NOT_FINALIZED','FINALIZED_HASH','FINALIZED_HEADER','FINALIZED_HEIGHT','FINALITY_REGRESSION','FINALITY_CANONICAL_MISMATCH','NONCANONICAL_FINALIZED_BLOCK','NONCANONICAL_BLOCK','TRANSACTION_STATUS','NATIVE_TRANSACTION_REQUIRED','TRANSACTION_ID_MISMATCH','IDENTIFIERS_MISMATCH','TRANSACTION_HASH_MISMATCH','CONTRACT_ACTION_COUNT','CONTRACT_ACTION_MISMATCH','SEGMENT_FAILURE','UNSUPPORTED_PROTOCOL','INDEXED_INPUTS_MISMATCH','INDEXED_OUTPUTS_MISMATCH','MISSING_CONTRACT_STATE','MISSING_CONTRACT_BALANCES','DUPLICATE_BALANCE_ASSET','OBSERVATION_TIMEOUT_UNKNOWN','DEADLINE_EXPIRED','RPC_DEADLINE','FINANCIAL_COMPARISON_REQUIRED_PASS','DRIVER_CLEANUP_INCOMPLETE']);
for(const code of SWAP_WALLET_FAILURE_CODES)publicFailureCodes.add(code);
for(const code of CONTRACT_BALANCE_FAILURE_CODES)publicFailureCodes.add(code);
publicFailureCodes.add('AMOUNT_CONTRACT_BALANCES');
/** Closed public diagnostic shared by the producer and durable launcher boundary. */
export function validatePublicDriverFailure(failure){
  const invalid=()=>{throw Error('INVALID_PUBLIC_DRIVER_FAILURE');};
  if(!failure||Object.getPrototypeOf(failure)!==Object.prototype)invalid();
  const fields=Object.getOwnPropertyDescriptors(failure),keys=Reflect.ownKeys(fields);
  if(keys.length!==3||!['phase','stage','code'].every(k=>Object.hasOwn(fields,k))||keys.some(k=>!Object.hasOwn(fields[k],'value')||!fields[k].enumerable))invalid();
  const phase=fields.phase.value,stage=fields.stage.value,code=fields.code.value;
  if(!['preflight','call','observe','compare'].includes(phase)||![null,'deploy','initialize','accrue','settle','swap','close'].includes(stage)||(code!=='UNCLASSIFIED_DRIVER_FAILURE'&&!publicFailureCodes.has(code)))invalid();
  return {phase,stage,code};
}
function publicFailure(error,phase,stage){
  const message=Object.getOwnPropertyDescriptor(error,'message');
  const code=message&&Object.hasOwn(message,'value')&&publicFailureCodes.has(message.value)?message.value:'UNCLASSIFIED_DRIVER_FAILURE';
  return validatePublicDriverFailure({phase,stage,code});
}


/**
 * Executes the fixed target-bound I2 sequence. Caller must supply the reviewed native
 * observer and full financial comparator; neither can be omitted. No CLI or
 * public dispatch is exposed until the complete integration is reviewed.
 * verifyStage must return an object with status PASS; every other outcome stops.
 * This function owns provider cleanup even when preflight fails. Failures expose
 * error.publicResult with known public IDs, reservations and cleanup disposition.
 * Incomplete containment cannot return PASS. SDK private results are not copied.
 */
export function runLocalFinancialCase(options){return runFinancialCase(options,'undeployed');}
/** Preview stage sequence only; caller owns chain/build/role validation and admission.
 * Controlled callbacks can test sequencing, never establish Preview acceptance.
 */
export function runPreviewFinancialCase(options){return runFinancialCase(options,'preview');}
async function runFinancialCase({kind,network,providers,compiledContract,roles,networkTag,now,observe,verifyStage,sdk,existingDeployment,initializedLoan,initializedSwap},target) {
  if(!providers||typeof providers.cleanup!=='function')throw Error('OWNED_PROVIDER_CLEANUP_REQUIRED');
  const stages=[],transactionIds=new Set();let address,failure,cleanup,diagnosticPhase='preflight',diagnosticStage=null;
  try {
    if(network!==target)throw Error(target==='undeployed'?'LOCAL_DRIVER_REQUIRES_UNDEPLOYED_NETWORK':'PREVIEW_DRIVER_REQUIRES_PREVIEW_NETWORK');
    if(target==='preview'&&(existingDeployment!==undefined||initializedLoan!==undefined||initializedSwap!==undefined))throw Error('PREVIEW_LOCAL_RECOVERY_FORBIDDEN');
    if(!['loan','swap'].includes(kind))throw Error('UNKNOWN_FINANCIAL_CASE');
    if(typeof observe!=='function'||typeof verifyStage!=='function'||typeof now!=='function')throw Error('OBSERVATION_AND_FINANCIAL_COMPARATOR_REQUIRED');
    if(!providers||typeof providers.execute!=='function'||typeof providers.cleanup!=='function'||typeof providers.stop!=='function'||typeof providers.getState!=='function'||typeof providers.getExecutionBinding!=='function')throw Error('GUARDED_PROVIDERS_REQUIRED');
    // Integration supplies this identity only after public verification and private
    // restore round trips. These shape checks do not establish those predicates.
    let recovery,continuation;
    if(initializedSwap!==undefined){
      if(kind!=='swap'||existingDeployment!==undefined||initializedLoan!==undefined||!initializedSwap||Object.getPrototypeOf(initializedSwap)!==Object.prototype)throw Error('INVALID_INITIALIZED_SWAP');
      const f=Object.getOwnPropertyDescriptors(initializedSwap);if(Reflect.ownKeys(f).length!==3||!['contractAddress','deployTxId','initializeTxId'].every(k=>Object.hasOwn(f,k)&&Object.hasOwn(f[k],'value'))||f.contractAddress.value!==INITIALIZED_SWAP.contractAddress||f.deployTxId.value!==EXISTING_SWAP.txId||f.initializeTxId.value!==INITIALIZED_SWAP.txId)throw Error('INVALID_INITIALIZED_SWAP');
      continuation={contractAddress:f.contractAddress.value,deployTxId:f.deployTxId.value,initializeTxId:f.initializeTxId.value};
    }
    if(initializedLoan!==undefined){
      if(kind!=='loan'||existingDeployment!==undefined||!initializedLoan||Object.getPrototypeOf(initializedLoan)!==Object.prototype)throw Error('INVALID_INITIALIZED_LOAN');
      const fields=Object.getOwnPropertyDescriptors(initializedLoan),keys=Reflect.ownKeys(fields);
      if(keys.length!==3||!['contractAddress','deployTxId','initializeTxId'].every(k=>Object.hasOwn(fields,k))||keys.some(k=>!Object.hasOwn(fields[k],'value')))throw Error('INVALID_INITIALIZED_LOAN');
      if(fields.contractAddress.value!==INITIALIZED_LOAN.contractAddress||fields.initializeTxId.value!==INITIALIZED_LOAN.txId||fields.deployTxId.value!=='00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9')throw Error('INVALID_INITIALIZED_LOAN');
      continuation={contractAddress:fields.contractAddress.value,deployTxId:fields.deployTxId.value,initializeTxId:fields.initializeTxId.value};
    }
    if(existingDeployment!==undefined) {
      if(kind!=='loan')throw Error('EXISTING_DEPLOYMENT_REQUIRES_LOAN');
      if(existingDeployment===null||typeof existingDeployment!=='object'||Object.getPrototypeOf(existingDeployment)!==Object.prototype)throw Error('INVALID_EXISTING_DEPLOYMENT');
      const fields=Object.getOwnPropertyDescriptors(existingDeployment),keys=Reflect.ownKeys(fields);
      if(keys.length!==2||!keys.includes('contractAddress')||!keys.includes('txId')||keys.some(key=>!Object.hasOwn(fields[key],'value')))throw Error('INVALID_EXISTING_DEPLOYMENT');
      const contractAddress=fields.contractAddress.value,txId=fields.txId.value;
      if(typeof contractAddress!=='string'||!/^[0-9a-f]{64}$/.test(contractAddress)||typeof txId!=='string'||!/^[0-9a-f]{66}$/.test(txId))throw Error('INVALID_EXISTING_DEPLOYMENT');
      recovery={contractAddress,txId};
    }
    const first=secret(roles.firstSecret),second=secret(roles.secondSecret);
    const firstAddress=bytes(roles.firstAddress),secondAddress=bytes(roles.secondAddress);
    if(roles.firstAddress===roles.secondAddress)throw Error('DISTINCT_FINANCIAL_PARTICIPANTS_REQUIRED');
    const program=bytes(bindings[kind].programDigest),net=bytes(networkTag);
    const payerAddress=roles.firstAddress;
    let initialBinding;
    async function checkBinding() {
      const observed=await providers.getExecutionBinding();
      const prefix=target==='undeployed'?'LOCAL':'PREVIEW';
      if(observed?.network?.networkId!==target||observed.sdkNetworkId!==target||observed.payerAddress!==payerAddress)throw Error(prefix+'_EXECUTION_BINDING_MISMATCH');
      if(target==='preview'){
        // These are configured public targets, not a fresh chain identity check.
        if(!['https://rpc.preview.midnight.network','https://rpc.preview.midnight.network/'].includes(observed.network.node)||observed.network.indexer!=='https://indexer.preview.midnight.network/api/v4/graphql'||observed.network.indexerWS!=='wss://indexer.preview.midnight.network/api/v4/graphql/ws')throw Error('PREVIEW_EXECUTION_BINDING_ENDPOINT');
      }
      for(const [key,protocol] of target==='undeployed'?[['node','http:'],['indexer','http:'],['indexerWS','ws:'],['proofServer','http:']]:[['proofServer','http:']]) {
        const url=new URL(observed.network[key]);
        if(url.protocol!==protocol||!['127.0.0.1','localhost','[::1]'].includes(url.hostname)||url.username||url.password||url.hash)throw Error(prefix+'_EXECUTION_BINDING_ENDPOINT');
      }
      const snapshot=JSON.stringify(observed);
      if(initialBinding!==undefined&&snapshot!==initialBinding)throw Error(prefix+'_EXECUTION_BINDING_CHANGED');
      initialBinding=snapshot;
    }
    await checkBinding();
    sdk??=await loadFinancialContractsSdk();
    let deployTxId;
    if(continuation){
      address=continuation.contractAddress;deployTxId=continuation.deployTxId;
    } else if(recovery) {
      address=recovery.contractAddress;
      deployTxId=recovery.txId;
    } else {
      diagnosticPhase='call';diagnosticStage='deploy';
      const deployed=await providers.execute('deploy',async()=>{await checkBinding();return sdk.deployContract(providers,{compiledContract,privateStateId:`sp05-${kind}`,initialPrivateState:{},args:[first,second,{bytes:firstAddress},{bytes:secondAddress},program,net]});});
      address=deployed.deployTxData.public.contractAddress;bytes(address);
      deployTxId=deployed.deployTxData.public.txId;
    }
    async function record(circuitId,txId) {
      if(typeof txId!=='string'||!txId)throw Error('MISSING_TRANSACTION_ID');
      transactionIds.add(txId);diagnosticPhase='observe';diagnosticStage=circuitId;
      const observation=await providers.execute('observe:'+circuitId,()=>observe({circuitId,txId,contractAddress:address,...(target==='preview'?{network:'preview'}:{})}));
      diagnosticPhase='compare';
      const comparison=await providers.execute('compare:'+circuitId,()=>verifyStage(circuitId,observation));
      if(comparison?.status!=='PASS')throw Error('FINANCIAL_COMPARISON_REQUIRED_PASS');
      stages.push(observation.receipt);
    }
    await record('deploy',deployTxId);
    if(continuation)await record('initialize',continuation.initializeTxId);
    async function call(circuitId,args) {
      diagnosticPhase='call';diagnosticStage=circuitId;
      const result=await providers.execute(circuitId,async()=>{await checkBinding();return sdk.submitCallTx(providers,{compiledContract,contractAddress:address,privateStateId:`sp05-${kind}`,circuitId,args});});
      await record(circuitId,result.public.txId);
    }
    if(kind==='loan') {
      if(!continuation)await call('initialize',[first,program,net,2n,time(now)]);
      await call('accrue',[first,program,net,0n,2n,time(now),{h0:mul(5000000000n,8n),h1:mul(40000000000n,31n),h2:mul(100n,365n),h3:div(1240000000000n,36500n)}]);
      await call('settle',[first,program,net,1n,2n,0n,533972602n,time(now),{unused:0n}]);
    } else {
      if(!continuation)await call('initialize',[second,program,net,3n,time(now)]);
      await call('swap',[first,program,net,0n,4n,4n,0n,1n,10000n,19700n,time(now),{h0:mul(10000n,997n),h1:mul(9970000n,2000000n),h2:mul(1000000n,1000n),h3:div(19940000000000n,1009970000n)}]);
      await call('close',[second,program,net,1n,3n,time(now),{unused:0n}]);
    }
  } catch(error) {
    failure=error instanceof Error?error:Error('DRIVER_OPERATION_FAILED');
    try {providers.stop?.('driver failed');}catch{/* Preserve the first failure. */}
  } finally {
    try {
      const result=await providers.cleanup();
      cleanup={walletStopped:result?.walletStopped===true,pendingOperations:Number.isSafeInteger(result?.pendingOperations)&&result.pendingOperations>=0?result.pendingOperations:null,containmentComplete:result?.containmentComplete===true};
    } catch {cleanup={walletStopped:false,pendingOperations:null,containmentComplete:false};}
  }
  // Public operational evidence excludes provider reasons, private SDK results and keys.
  let operationalState;
  try {
    const state=providers.getState?.();
    for(const id of state?.identifiers??[])if(typeof id==='string'&&id)transactionIds.add(id);
    operationalState={reservedSubmissions:state?.reservedSubmissions??null,reservedDustFee:state?.reservedDustFee?.toString()??null,reservedGrossByAsset:Object.fromEntries(Object.entries(state?.reservedGrossByAsset??{}).map(([asset,value])=>[asset,value.toString()]))};
  } catch {operationalState={unavailable:true};}
  const contained=cleanup.walletStopped&&cleanup.pendingOperations===0&&cleanup.containmentComplete;
  // Preview command completion does not attest to external process containment.
  const previewComplete=target==='preview'&&stages.length===4&&cleanup.walletStopped&&cleanup.pendingOperations===0;
  const publicResult={schema:target==='undeployed'?'moriarty.local-financial-run/1':'moriarty.preview-financial-run/1',status:failure?'FAILED':contained?'PASS':previewComplete?'FINANCIAL_COMPLETE':'INCOMPLETE',kind,contractAddress:address,stages,transactionIds:[...transactionIds],cleanup,operationalState,...(failure?{failure:publicFailure(failure,diagnosticPhase,diagnosticStage)}:{}),scope:'Fixed I2 execution and supplied financial comparison; not mandatory PCD acceptance'};
  if(failure||(!contained&&!previewComplete)) {
    const error=failure??Error('DRIVER_CLEANUP_INCOMPLETE');
    error.publicResult=publicResult;throw error;
  }
  return publicResult;
}
