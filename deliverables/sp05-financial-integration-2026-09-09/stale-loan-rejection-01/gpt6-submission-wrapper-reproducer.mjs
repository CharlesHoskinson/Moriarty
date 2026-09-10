// No service construction or RPC: reproduce the installed default submission
// service's exact mapError + runPromise error shape with real installed classes.
import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
import {classifyStaleLoanFailure} from '../../../experiments/moriarty-midnight-financial/ledger/stale-loan-rejection.mjs';
const nm='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
const imp=p=>import(pathToFileURL(nm+'/'+p).href);
const {Effect}=await imp('effect/dist/esm/index.js');
const {TransactionInvalidError}=await imp('@midnight-ntwrk/wallet-sdk-node-client/dist/effect/NodeClientError.js');
const {SubmissionError:ServiceSubmissionError}=await imp('@midnight-ntwrk/wallet-sdk-capabilities/dist/submission/submissionService.js');
const raw=new Uint8Array([1,2,3]);
const invalid=new TransactionInvalidError({message:'public controlled invalid status',txData:raw});
async function classify(operation){try{await Effect.runPromise(operation);throw Error('CONTROLLED_FAILURE_REQUIRED');}catch(error){return classifyStaleLoanFailure(error,'submit',raw);}}
const direct=await classify(Effect.fail(invalid));
const actualServiceShape=await classify(Effect.fail(invalid).pipe(Effect.mapError(cause=>new ServiceSubmissionError({message:'Transaction submission error',cause}))));
assert.equal(direct.nodeRejectionEstablished,true);
assert.equal(actualServiceShape.nodeRejectionEstablished,false);
assert.equal(actualServiceShape.status,'OUTCOME_UNKNOWN');
console.log(JSON.stringify({schema:'moriarty.independent-submission-wrapper-reproducer/1',reviewer:'gpt-6-astra',status:'BLOCKER_REPRODUCED',scope:'Actual installed error classes and Effect shape; synthetic three-byte public payload. No node/service/wallet/private input/proof/submission.',directNodeError:direct,installedDefaultServiceErrorShape:actualServiceShape,expectedProductionBehavior:'Exact installed service wrapper around byte-matched TransactionInvalidError must be recognized, while unrelated/nested/spoofed errors remain UNKNOWN.',sourceEvidence:'wallet-sdk-capabilities/dist/submission/submissionService.js submit uses Effect.mapError to its own SubmissionError before Effect.runPromise; wallet-sdk-facade submitTransaction rethrows that error after revert.'},null,2));
