import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from '../../../experiments/moriarty-midnight-financial/ledger/providers.mjs';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
import {decodeNativeFinancialTransaction} from '../../../experiments/moriarty-midnight-financial/ledger/receipt.mjs';
const sha=x=>createHash('sha256').update(x).digest('hex');
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
const stateBytes=readFileSync(new URL('./indexed-initialize-state.bin',import.meta.url));
if(sha(stateBytes)!=='1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f')throw Error('INDEXED_STATE_HASH');
const raw=readFileSync(new URL('./run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin',import.meta.url));
const loaded=await loadProvenFinancialContract({case:'loan',receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
try{
 const state=loaded.decodeState(ledger.ContractState.deserialize(stateBytes).data),native=decodeNativeFinancialTransaction(raw,ledger);loaded.assertFresh();
 console.log(JSON.stringify({schema:'moriarty.actual-indexed-initialize-decode/1',transactionHash:native.transactionHash,stateSha256:sha(stateBytes),initialized:state.initialized,remaining:state.remaining.toString(),revision:state.revision.toString(),kernelState:Object.fromEntries(Object.entries(state.kernelState).map(([k,v])=>[k,v.toString()])),usdColor:Buffer.from(state.usdColor).toString('hex'),outputs:native.outputs,actions:native.actions,dustFee:native.dustFee,scope:'Decoded actual retained indexed state and submitted native bytes using approved build/pinned runtime. No live RPC, wallet, private store, full financial comparison or independent finality.'},null,2));
}finally{await loaded.cleanup();}
