/** Fixed stale settled-loan plan and public historical input checks only.
 * Does not open private paths, reserve resources, establish fresh state, or
 * authorize a transaction. Launcher integration and source reviews remain required.
 */
import {openSync,closeSync,fstatSync,lstatSync,readSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {dirname,isAbsolute,resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {inspectExistingLoanBytes} from './recover-deployment.mjs';
import {inspectInitializedLoanBytes} from './continue-initialized-loan.mjs';
import {decodeNativeFinancialTransaction} from './receipt.mjs';
const D=fileURLToPath(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url));
const check=(condition,code)=>{if(!condition)throw Error('STALE_PLAN_'+code);};
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
function freeze(value){for(const child of Object.values(value))if(child&&typeof child==='object')freeze(child);return Object.freeze(value);}
export const STALE_LOAN=freeze({
  "schema": "moriarty.existing-stale-local-loan/1",
  "contractAddress": "ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713",
  "networkTag": "e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846",
  "expectedProtocolVersion": 1000000,
  "buildReceiptSha256": "51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7",
  "privateStateDirectory": "/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03/contract-state",
  "initializedStateFile": join(D,"local-recovery-03/indexed-initialize-state.bin"),
  "initializedStateSha256": "1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f",
  "settledStateSha256": "552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0",
  "minimumCurrentBlockHeight": 20415,
  "deployTransactionFile": join(D,"local-execution-04/run-public/public-transactions/0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c.bin"),
  "deployTransactionHash": "0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c",
  "deployTxId": "00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9",
  "deployIdentifiers": [
    "00b6140ece5793d57c801512e8e7c9e2ec2687e19b1c48a1f56167f2cd1dfc9e66",
    "00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9"
  ],
  "initializeTransactionFile": join(D,"local-recovery-03/run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin"),
  "initializeTransactionHash": "1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6",
  "initializeTxId": "006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46",
  "initializeIdentifiers": [
    "004b2aa75cdb10a080c3560a0af50ab17ef6e2e2294e1609a9e087f18498dc1d05",
    "006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46"
  ],
  "accrueTransactionFile": join(D,"local-continuation-02/run-public/public-transactions/473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480.bin"),
  "accrueTransactionHash": "473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480",
  "accrueTxId": "001ac8252beed46b047b6032710893182db342ce85e195403517738bf47e7248e7",
  "accrueIdentifiers": [
    "00f8303ec839a493912ae386b4019cefba92c36e3bfc5a6fea05658a9139f37435",
    "001ac8252beed46b047b6032710893182db342ce85e195403517738bf47e7248e7"
  ],
  "settleTransactionFile": join(D,"local-continuation-02/run-public/public-transactions/c201557431820f032743b0fc99c815b59131fa8bde6e298c466009189bce396e.bin"),
  "settleTransactionHash": "c201557431820f032743b0fc99c815b59131fa8bde6e298c466009189bce396e",
  "settleTxId": "00f40afa7fb4eb3e3daca7b5604ba783e7c1c214151766974848f40daedb474114",
  "settleIdentifiers": [
    "00f9b1159e9e600e04a8ec40a24f1e5e4f364b7ca34c72609a3ff28440ee153a6c",
    "00f40afa7fb4eb3e3daca7b5604ba783e7c1c214151766974848f40daedb474114"
  ],
  "deployResultFile": join(D,"local-execution-04/attempt-result.json"),
  "deployResultSha256": "bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54",
  "initializeResultFile": join(D,"local-recovery-03/attempt-result.json"),
  "initializeResultSha256": "858bea821333f092e42afacbf84386a0400ea6f1b350a76c6a23d66754e70aca",
  "continuationResultFile": join(D,"local-continuation-02/attempt-result.json"),
  "continuationResultSha256": "822232e8147b0c26e5012eef7a55b1cd926091f400dc52c2c059929ee8a5143f",
  "continuationIntegrationFile": join(D,"local-continuation-02/run-public/integration-result.json"),
  "continuationIntegrationSha256": "5a237ead68191acd494d2b93d6d5ed0b2e6a54c67ab44f2f9f59e2c83bb3ad80",
  "continuationReviewedResultFile": join(D,"local-continuation-02/reviewed-result.json"),
  "continuationReviewedResultSha256": "306e10064ee26742085366f628b14cd6d7f70264e6e8a7b037922362903fcf82",
  "settledProbeFile": join(D,"local-finalized-state-02/probe-result.json"),
  "settledProbeSha256": "8e651e0fc93055a7500aabf9fe874be1a93937ebe622a8451bbe963d973ec220"
});
const ORIGINAL='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03';
const EXISTING='/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03';
const WALLET={seedFile:'/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed',stateDirectory:'/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2',expectedAddress:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9'};
const ROLES={firstAddress:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',secondAddress:'c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e',secretsFile:ORIGINAL+'/roles.json'};
const BUILD={receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:STALE_LOAN.buildReceiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'};
const NETWORK={networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
// Inspect descriptors before any property value access, including nested arrays.
function data(value,ancestors=new Set()){
 if(value===null||typeof value==='string'||typeof value==='boolean')return;
 if(typeof value==='number'){check(Number.isFinite(value),'DATA');return;}
 check(value&&typeof value==='object'&&!ancestors.has(value),'DATA');
 const array=Array.isArray(value);check(Object.getPrototypeOf(value)===(array?Array.prototype:Object.prototype),'DATA');
 const ds=Object.getOwnPropertyDescriptors(value),keys=Reflect.ownKeys(ds);check(keys.every(k=>typeof k==='string'&&Object.hasOwn(ds[k],'value')&&(array&&k==='length'||ds[k].enumerable)),'DATA');
 if(array)check(keys.length===value.length+1&&keys.every(k=>k==='length'||/^(0|[1-9][0-9]*)$/.test(k)&&Number(k)<value.length),'DATA');
 ancestors.add(value);for(const key of keys)if(!(array&&key==='length'))data(ds[key].value,ancestors);ancestors.delete(value);
}
function closed(value,fields){check(value&&Object.getPrototypeOf(value)===Object.prototype&&Object.keys(value).sort().join(',')===[...fields].sort().join(','),'FIELDS');}
function absolute(path){check(typeof path==='string'&&isAbsolute(path)&&resolve(path)===path&&!path.includes('\0'),'PATH');}
const disjoint=(a,b)=>a!=='/'&&b!=='/'&&a!==b&&!a.startsWith(b+'/')&&!b.startsWith(a+'/');
function descriptor(d){data(d);closed(d,[...Object.keys(STALE_LOAN),'snapshotDirectory','inspectionDirectory']);for(const [key,value]of Object.entries(STALE_LOAN))check(isDeepStrictEqual(d[key],value),'BINDING');absolute(d.snapshotDirectory);absolute(d.inspectionDirectory);return d;}
export function validateStaleLoanPlan(d,p){
 data(p);descriptor(d);
 closed(p,'schema,kind,build,networkConfig,wallet,roles,privateState,networkTag,expectedProtocolVersion,limits,outputDirectory,existingStaleLoan'.split(','));
 check(p.schema==='moriarty.local-financial-launch/1'&&p.kind==='loan'&&isDeepStrictEqual(p.existingStaleLoan,d),'MODE');
 for(const [group,expected]of [['build',BUILD],['wallet',WALLET],['roles',ROLES],['networkConfig',NETWORK]]){closed(p[group],Object.keys(expected));check(isDeepStrictEqual(p[group],expected),'ORIGINAL_'+group.toUpperCase());}
 closed(p.privateState,['directory','passwordFile']);check(p.privateState.directory===STALE_LOAN.privateStateDirectory&&p.privateState.passwordFile===EXISTING+'/password','EXISTING_STORE');
 check(p.networkTag===STALE_LOAN.networkTag&&p.expectedProtocolVersion===STALE_LOAN.expectedProtocolVersion,'NETWORK');
 closed(p.limits,['allocationId','deadlineMs','submissions','dustFee','grossByLogicalAsset']);closed(p.limits.grossByLogicalAsset,['USD_TEST_ASSET']);
 check(p.limits.submissions===1&&typeof p.limits.allocationId==='string'&&/^sp05-stale-loan-[a-zA-Z0-9_-]{1,60}$/.test(p.limits.allocationId),'ALLOCATION');
 const now=Date.now();check(Number.isSafeInteger(p.limits.deadlineMs)&&p.limits.deadlineMs>now&&p.limits.deadlineMs<=now+1200000,'DEADLINE');
 check(p.limits.grossByLogicalAsset.USD_TEST_ASSET==='0','GROSS');check(typeof p.limits.dustFee==='string'&&/^[1-9][0-9]{0,15}$/.test(p.limits.dustFee)&&BigInt(p.limits.dustFee)<=1000000000000000n,'DUST');
 absolute(p.outputDirectory);const destinations=[p.outputDirectory,d.snapshotDirectory,d.inspectionDirectory];
 for(let i=0;i<destinations.length;i++)for(let j=i+1;j<destinations.length;j++)check(disjoint(destinations[i],destinations[j]),'OVERLAP');
 for(const dest of destinations){
  for(const protectedPath of [ORIGINAL,EXISTING,dirname(WALLET.seedFile),dirname(BUILD.receiptPath),resolve(D)])check(disjoint(dest,protectedPath),'PRESERVE_ORIGINAL');
  // Historical local financial campaign directories remain reserved regardless of suffix.
  check(!/^\/home\/charl\/\.local\/state\/moriarty\/sp05-local-(?:loan|swap|finalized-state)(?:-|\/|$)/.test(dest),'HISTORICAL_DESTINATION');
 }
 // Lexical novelty is not an existence test: launcher must use exclusive creation,
 // reject symlinks, and refuse an existing reservation/output before private access.
 return structuredClone(d);
}
function publicBytes(path,max){
 let fd;
 try{
  for(let at=path;;at=dirname(at)){check(!lstatSync(at).isSymbolicLink(),'PUBLIC_SYMLINK');if(dirname(at)===at)break;}
  fd=openSync(path,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK);const st=fstatSync(fd);check(st.isFile()&&st.size>0&&st.size<=max,'PUBLIC_FILE');
  const raw=Buffer.alloc(st.size);let offset=0;while(offset<raw.length){const n=readSync(fd,raw,offset,raw.length-offset,offset);check(n>0,'PUBLIC_CHANGED');offset+=n;}check(readSync(fd,Buffer.alloc(1),0,1,offset)===0,'PUBLIC_CHANGED');return raw;
 }catch(e){throw e.message?.startsWith('STALE_PLAN_')?e:Error('STALE_PLAN_PUBLIC_READ');}finally{if(fd!==undefined)closeSync(fd);}
}
export function readStaleLoanInputs(d,ledger){
 descriptor(d);const history=[];
 for(const stage of ['deploy','initialize','accrue','settle']){
  const raw=publicBytes(d[stage+'TransactionFile'],16*1024*1024);check(hash(raw)===d[stage+'TransactionHash'],'TRANSACTION_HASH');
  const transaction=decodeNativeFinancialTransaction(raw,ledger);check(transaction.transactionHash===d[stage+'TransactionHash']&&isDeepStrictEqual(transaction.identifiers,d[stage+'Identifiers'])&&transaction.identifiers.includes(d[stage+'TxId']),'TRANSACTION_IDENTITY');
  check(transaction.actions.length===1&&transaction.actions[0].address===d.contractAddress&&(stage==='deploy'?transaction.actions[0].kind==='deploy':transaction.actions[0].entryPoint===stage),'HISTORY_ACTION');
  history.push({stage,raw,transaction,txId:d[stage+'TxId']});
 }
 const deployBinding=inspectExistingLoanBytes(history[0].raw,ledger),initializeBinding=inspectInitializedLoanBytes(history[1].raw,ledger);
 const initializedStateRaw=publicBytes(d.initializedStateFile,1024*1024);check(hash(initializedStateRaw)===d.initializedStateSha256,'INITIALIZED_STATE');
 const sources={};for(const name of ['deployResult','initializeResult','continuationResult','continuationIntegration','continuationReviewedResult','settledProbe']){const raw=publicBytes(d[name+'File'],256*1024);check(hash(raw)===d[name+'Sha256'],'SOURCE_HASH');sources[name]=JSON.parse(raw);}
 check(sources.deployResult.allocationId==='sp05-local-loan-03'&&sources.initializeResult.allocationId==='sp05-local-loan-recovery-03'&&sources.continuationResult.allocationId==='sp05-local-loan-continuation-02','HISTORY_ALLOCATION');
 check(sources.continuationIntegration.financialComparison.status==='PASS'&&sources.continuationIntegration.financialComparison.stages.length===4&&sources.continuationIntegration.sourceTestOnly===false,'HISTORY_COMPARISON');
 check(sources.continuationReviewedResult.result.sha256===d.continuationResultSha256&&sources.continuationReviewedResult.status==='REVIEWED_LOCAL_LOAN_TRACE_MATCHED_OUTER_CONTAINED','HISTORY_REVIEW');
 const settledSource=sources.settledProbe.snapshots[0];check(sources.settledProbe.status==='OBSERVED'&&settledSource.contractAddress===d.contractAddress&&settledSource.blockHeight>=d.minimumCurrentBlockHeight&&settledSource.stateSha256===d.settledStateSha256&&hash(Buffer.from(settledSource.serializedStateHex,'hex'))===d.settledStateSha256,'SETTLED_SOURCE');
 check(settledSource.state.remaining==='0'&&settledSource.state.revision==='2'&&isDeepStrictEqual(settledSource.balances,{'e92df6339320f55209d4586ce039b6ef05a7be960999fca913006146cb72cdef':'0'}),'SETTLED_FINANCIAL_STATE');
 return {history,deployBinding,initializeBinding,initializedStateRaw,sources,settledSource:structuredClone(settledSource),freshCurrentStateEstablished:false,executionAuthorized:false,scope:'Pinned historical public inputs only; fresh canonical history/current state, preserved store and admission remain caller obligations'};
}
