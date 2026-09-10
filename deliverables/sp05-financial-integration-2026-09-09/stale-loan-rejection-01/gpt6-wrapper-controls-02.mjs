// Independent exact-shape classification controls, no service construction/network.
import assert from 'node:assert/strict';import {pathToFileURL} from 'node:url';
import {classifyStaleLoanFailure} from '../../../experiments/moriarty-midnight-financial/ledger/stale-loan-rejection.mjs';
const nm='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';const imp=p=>import(pathToFileURL(nm+'/'+p).href);
const {Effect}=await imp('effect/dist/esm/index.js');
const {TransactionInvalidError,SubmissionError}=await imp('@midnight-ntwrk/wallet-sdk-node-client/dist/effect/NodeClientError.js');
const {SubmissionError:WalletSubmissionError}=await imp('@midnight-ntwrk/wallet-sdk-capabilities/dist/submission/submissionService.js');
const {default:RpcError}=await imp('@polkadot/rpc-provider/coder/error.js');
const raw=new Uint8Array([1,2,3]);const invalid=()=>new TransactionInvalidError({message:'controlled public error',txData:raw});const wrap=cause=>new WalletSubmissionError({message:'Transaction submission error',cause});
const results=[];
for(const [label,error,expected,phase='submit',submitted=raw] of [
 ['direct node class',invalid(),true],['exact service wrapper',wrap(invalid()),true],['two service wrappers',wrap(wrap(invalid())),false],['forged wrapper',{_tag:'SubmissionError',cause:invalid()},false],['generic Error with cause',Object.assign(new Error('submission'),{cause:invalid()}),false],['unrelated inner error',wrap(new Error('node invalid')),false],['wrong phase',wrap(invalid()),false,'prove'],['wrong bytes',wrap(invalid()),false,'submit',new Uint8Array([4])],['node generic submission',wrap(new SubmissionError({cause:new RpcError('controlled1010',1010),txData:raw})),false]
]){
 let thrown;try{await Effect.runPromise(Effect.fail(error));}catch(e){thrown=e;}
 const result=classifyStaleLoanFailure(thrown,phase,submitted);assert.equal(result.nodeRejectionEstablished,expected,label);if(label==='node generic submission'){assert.equal(result.rpcCode,1010);assert.equal(result.status,'OUTCOME_UNKNOWN');}results.push({label,status:'PASS',classification:result});
}
console.log(JSON.stringify({schema:'moriarty.independent-wrapper-correction-controls/1',reviewer:'gpt-6-astra',status:'PASS',fixedFinding:'G6-SLR-01',scope:'Real installed error classes and Effect.runPromise, synthetic public bytes only, no service/network/private/proof/submission',results},null,2));
