/** Closed plan and public input reader for this one initialized local loan.
 * No private file access, wallet construction, chain query or dispatch authority.
 */
import {openSync,closeSync,fstatSync,lstatSync,readSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {dirname,isAbsolute,resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {EXISTING_LOAN,inspectExistingLoanBytes} from './recover-deployment.mjs';
import {INITIALIZED_LOAN,inspectInitializedLoanBytes} from './continue-initialized-loan.mjs';
const check=(ok,code)=>{if(!ok)throw Error('INITIALIZED_PLAN_'+code);};
const sha=b=>createHash('sha256').update(b).digest('hex');
const D=fileURLToPath(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url));
const ORIGINAL='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03';
const EXISTING='/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03';
const WALLET={seedFile:'/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed',stateDirectory:'/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2',expectedAddress:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9'};
const ROLES={firstAddress:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',secondAddress:'c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e',secretsFile:ORIGINAL+'/roles.json'};
const BUILD={receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:EXISTING_LOAN.buildReceiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'};
const FIXED={schema:'moriarty.existing-initialized-local-loan/1',deployTransactionFile:join(D,'local-execution-04/run-public/public-transactions',EXISTING_LOAN.transactionHash+'.bin'),deployTransactionHash:EXISTING_LOAN.transactionHash,deployIdentifiers:[...EXISTING_LOAN.identifiers],deployTxId:EXISTING_LOAN.txId,deploySourceResultFile:join(D,'local-execution-04/attempt-result.json'),deploySourceResultSha256:'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54',deploySourceAllocationId:'sp05-local-loan-03',initializeTransactionFile:join(D,'local-recovery-03/run-public/public-transactions',INITIALIZED_LOAN.transactionHash+'.bin'),initializeTransactionHash:INITIALIZED_LOAN.transactionHash,initializeIdentifiers:[...INITIALIZED_LOAN.identifiers],initializeTxId:INITIALIZED_LOAN.txId,initializeSourceResultFile:join(D,'local-recovery-03/attempt-result.json'),initializeSourceResultSha256:'858bea821333f092e42afacbf84386a0400ea6f1b350a76c6a23d66754e70aca',initializeSourceAllocationId:'sp05-local-loan-recovery-03',contractAddress:INITIALIZED_LOAN.contractAddress,initializedStateSha256:INITIALIZED_LOAN.initializedStateSha256,buildReceiptSha256:EXISTING_LOAN.buildReceiptSha256,networkTag:'e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846',expectedProtocolVersion:1000000,privateStateDirectory:EXISTING+'/contract-state'};
function closed(value,fields){check(value&&Object.getPrototypeOf(value)===Object.prototype,'FIELDS');const ds=Object.getOwnPropertyDescriptors(value);check(Reflect.ownKeys(ds).length===fields.length&&Object.keys(ds).sort().join(',')===[...fields].sort().join(',')&&Object.values(ds).every(d=>Object.hasOwn(d,'value')),'FIELDS');}
function absolute(path){check(typeof path==='string'&&isAbsolute(path)&&resolve(path)===path,'PATH');}
function disjoint(a,b){return a!=='/'&&b!=='/'&&a!==b&&!a.startsWith(b+'/')&&!b.startsWith(a+'/');}
function descriptor(d){closed(d,[...Object.keys(FIXED),'snapshotDirectory','inspectionDirectory']);for(const [key,value] of Object.entries(FIXED))check(isDeepStrictEqual(d[key],value),'BINDING');absolute(d.snapshotDirectory);absolute(d.inspectionDirectory);return d;}
export function validateInitializedLoanPlan(d,p){
 descriptor(d);
 closed(p,'schema,kind,build,networkConfig,wallet,roles,privateState,networkTag,expectedProtocolVersion,limits,outputDirectory,existingInitializedLoan'.split(','));
 check(p.schema==='moriarty.local-financial-launch/1'&&p.kind==='loan'&&isDeepStrictEqual(p.existingInitializedLoan,d),'MODE');
 closed(p.build,Object.keys(BUILD));closed(p.wallet,Object.keys(WALLET));closed(p.roles,Object.keys(ROLES));closed(p.privateState,['directory','passwordFile']);
 check(isDeepStrictEqual(p.build,BUILD)&&isDeepStrictEqual(p.wallet,WALLET)&&isDeepStrictEqual(p.roles,ROLES),'ORIGINAL_IDENTITY_BUILD');
 check(p.privateState.directory===FIXED.privateStateDirectory&&p.privateState.passwordFile===EXISTING+'/password','EXISTING_STORE');
 check(p.networkTag===FIXED.networkTag&&p.expectedProtocolVersion===1000000,'NETWORK');
 closed(p.networkConfig,['networkId','node','indexer','indexerWS','proofServer']);check(p.networkConfig.networkId==='undeployed','NETWORK');
 for(const key of ['node','indexer','indexerWS','proofServer']){let u;try{u=new URL(p.networkConfig[key]);}catch{throw Error('INITIALIZED_PLAN_ENDPOINT');}check(u.protocol===(key==='indexerWS'?'ws:':'http:')&&['127.0.0.1','[::1]'].includes(u.hostname)&&!u.username&&!u.password&&!u.hash,'ENDPOINT');}
 closed(p.limits,['allocationId','deadlineMs','submissions','dustFee','grossByLogicalAsset']);closed(p.limits.grossByLogicalAsset,['USD_TEST_ASSET']);
 const oldIds=['sp05-local-loan-01','sp05-local-loan-02','sp05-local-loan-03','sp05-local-loan-recovery-01','sp05-local-loan-recovery-02','sp05-local-loan-recovery-03'];
 check(p.limits.submissions===2&&typeof p.limits.allocationId==='string'&&/^[a-zA-Z0-9_-]{1,80}$/.test(p.limits.allocationId)&&!oldIds.includes(p.limits.allocationId),'ALLOCATION');
 check(Number.isSafeInteger(p.limits.deadlineMs)&&p.limits.deadlineMs>Date.now()&&p.limits.deadlineMs<=Date.now()+3600000,'DEADLINE');
 for(const n of [p.limits.dustFee,p.limits.grossByLogicalAsset.USD_TEST_ASSET])check(typeof n==='string'&&/^[1-9][0-9]{0,38}$/.test(n)&&BigInt(n)<(1n<<128n),'LIMIT');
 absolute(p.outputDirectory);const destinations=[p.outputDirectory,d.snapshotDirectory,d.inspectionDirectory];for(let i=0;i<destinations.length;i++)for(let j=i+1;j<destinations.length;j++)check(disjoint(destinations[i],destinations[j]),'OVERLAP');
 for(const path of destinations)for(const protectedPath of [ORIGINAL,EXISTING,dirname(WALLET.seedFile)])check(disjoint(path,protectedPath),'PRESERVE_ORIGINAL');
 return structuredClone(d);
}
function publicBytes(path,max){
 let fd;
 try{
  for(let cursor=path;;cursor=dirname(cursor)){check(!lstatSync(cursor).isSymbolicLink(),'PUBLIC_SYMLINK');if(dirname(cursor)===cursor)break;}
  fd=openSync(path,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK);const s=fstatSync(fd);check(s.isFile()&&s.size>0&&s.size<=max,'PUBLIC_FILE');
  const raw=Buffer.alloc(s.size);let n=0;while(n<raw.length){const count=readSync(fd,raw,n,raw.length-n,n);check(count>0,'PUBLIC_CHANGED');n+=count;}check(readSync(fd,Buffer.alloc(1),0,1,n)===0,'PUBLIC_CHANGED');return raw;
 }catch(e){throw e.message?.startsWith('INITIALIZED_PLAN_')?e:Error('INITIALIZED_PLAN_PUBLIC_READ');}finally{if(fd!==undefined)closeSync(fd);}
}
export function readInitializedLoanInputs(d,ledger){
 descriptor(d); // Fixed public paths prevent caller-selected private reads before the public gate.
 const deployRaw=publicBytes(d.deployTransactionFile,16*1024*1024),initializeRaw=publicBytes(d.initializeTransactionFile,16*1024*1024);
 const deployBinding=inspectExistingLoanBytes(deployRaw,ledger),initializeBinding=inspectInitializedLoanBytes(initializeRaw,ledger);
 const deployResult=publicBytes(d.deploySourceResultFile,128*1024),initializeResult=publicBytes(d.initializeSourceResultFile,128*1024);
 check(sha(deployResult)===d.deploySourceResultSha256&&sha(initializeResult)===d.initializeSourceResultSha256,'RESULT_HASH');
 const a=JSON.parse(deployResult),b=JSON.parse(initializeResult);
 check(a.allocationId===d.deploySourceAllocationId&&a.transactionHash===d.deployTransactionHash&&a.contractAddress===d.contractAddress&&a.reservedSubmissions===1&&a.reservedDustFee==='300000000000001','DEPLOY_RESULT');
 check(b.allocationId===d.initializeSourceAllocationId&&b.newTransaction?.transactionHash===d.initializeTransactionHash&&b.newTransaction.txId===d.initializeTxId&&isDeepStrictEqual(b.newTransaction.identifiers,d.initializeIdentifiers)&&b.reservedSubmissions===1&&b.reservedDustFee==='300000000000001'&&b.resubmitInitialize===false,'INITIALIZE_RESULT');
 return {deployRaw,initializeRaw,deployBinding,initializeBinding};
}
