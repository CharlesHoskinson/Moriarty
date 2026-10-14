import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from '../../../experiments/moriarty-midnight-financial/ledger/providers.mjs';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
import {extractNativeContractBalances} from '../../../experiments/moriarty-midnight-financial/ledger/contract-balances.mjs';
import {decodeNativeFinancialTransaction} from '../../../experiments/moriarty-midnight-financial/ledger/receipt.mjs';
const sha=x=>createHash('sha256').update(x).digest('hex');
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
const compact=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs'));
const indexed=JSON.parse(readFileSync(new URL('./indexed-result.json',import.meta.url)));
const loaded=await loadProvenFinancialContract({case:'swap',receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json',receiptSha256:'3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
try{
 const records=[];
 for(const row of indexed.records){
  const bytes=readFileSync(new URL(row.stateArtifact,import.meta.url));if(sha(bytes)!==row.stateSha256)throw Error('STATE_HASH');
  const state=compact.ContractState.deserialize(bytes);if(!Buffer.from(state.serialize()).equals(bytes))throw Error('STATE_CANONICAL');
  const raw=readFileSync(new URL(row.transactionArtifact,import.meta.url));if(sha(raw)!==row.transactionHash)throw Error('NATIVE_HASH');
  const native=decodeNativeFinancialTransaction(raw,ledger);
  records.push({stage:row.stage,stateSha256:row.stateSha256,nativeContractBalances:extractNativeContractBalances({state}),decodedState:loaded.decodeState(state.data),native});
 }
 loaded.assertFresh();writeFileSync(new URL('./decoded-public.json',import.meta.url),JSON.stringify({schema:'moriarty.swap-close-public-decode/1',records,scope:'Actual stopped indexed native state and transaction decoding using pinned public runtime/build. No synthetic state replacement, original private access, independent finality or acceptance promotion.'},(_,x)=>typeof x==='bigint'?x.toString():x instanceof Uint8Array?Buffer.from(x).toString('hex'):x,2)+'\n');
 console.log(JSON.stringify(records.map(r=>({stage:r.stage,balances:r.nativeContractBalances,remaining:String(r.decodedState.remaining),revision:String(r.decodedState.revision)}))));
}finally{await loaded.cleanup();}
