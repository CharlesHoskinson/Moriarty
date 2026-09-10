import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from '../../../experiments/moriarty-midnight-financial/ledger/providers.mjs';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
import {decodeNativeFinancialTransaction} from '../../../experiments/moriarty-midnight-financial/ledger/receipt.mjs';
import {createFinancialComparator} from '../../../experiments/moriarty-midnight-financial/ledger/financial-comparison.mjs';
const sha=x=>createHash('sha256').update(x).digest('hex'),root=new URL('./',import.meta.url);
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
const compact=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs'));
const loaded=await loadProvenFinancialContract({case:'swap',receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json',receiptSha256:'3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
try{
 const records=[],actual=[];
 for(const [stage,hash] of [['deploy','31677c341ab6900f144b5205c6be51dbaf81a06607307e7864be378a4472f91d'],['initialize','3fec717c8d31e6da9b9ace76c9f209bdbda11fe9d27f5d870c6edf428d2be7b4']]){
 const stateBytes=readFileSync(new URL('indexed-'+stage+'-state.bin',root));
 const nativeState=ledger.ContractState.deserialize(stateBytes),state=loaded.decodeState(compact.ContractState.deserialize(stateBytes).data);
 const raw=readFileSync(new URL('../local-swap-01/run-public/public-transactions/'+hash+'.bin',root));
 const native=decodeNativeFinancialTransaction(raw,ledger);if(native.transactionHash!==hash)throw Error('NATIVE_HASH');
 actual.push({stage,state,native,balance:nativeState.balance});
 records.push({stage,stateSha256:sha(stateBytes),nativeStateBalance:[...nativeState.balance],decodedState:state,native});
 }
 const runtime=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/compact-runtime/dist/index.js'));
 const networkTag=Buffer.from(actual[0].state.networkTag).toString('hex'),roles={firstAddress:Buffer.from(actual[0].state.traderAddress.bytes).toString('hex'),secondAddress:Buffer.from(actual[0].state.providerAddress.bytes).toString('hex'),firstSecret:new Uint8Array(32).fill(17),secondSecret:new Uint8Array(32).fill(18)};
 const indexed=JSON.parse(readFileSync(new URL('stopped-indexer-result.json',root))),reproductions=[];
 for(const useNativeBalance of [false,true]){
  const comparator=await createFinancialComparator({kind:'swap',roles,networkTag,expectedProtocolVersion:1000000});const outcomes=[];
  for(const item of actual){
   const state=structuredClone(item.state),block=indexed.contractActions.find(x=>x.txHash===item.native.transactionHash);
   for(const [role,secret] of [['trader',roles.firstSecret],['provider',roles.secondSecret]])state[role+'Capability']=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[Uint8Array.from(Buffer.from(('moriarty:sp05:swap:'+role).padEnd(32,'\0'))),state.networkTag,state.programDigest,secret]);
   const receipt={schema:'moriarty.finalized-financial-stage/1',txId:item.native.identifiers.at(-1),contractAddress:block.address,circuitId:item.stage,transaction:item.native,blockHash:block.blockHash,blockHeight:block.height,finalizedHead:'0x'+block.blockHash,finalizedHeight:block.height,protocolVersion:1000000,indexerIdentifiers:item.native.identifiers,fees:{nativeDebit:{asset:'DUST',unit:'SPECK',amount:item.native.dustFee},indexerReported:{paid:'1',estimated:'1',sourceUnitLabel:'DUST',encoding:'unresolved',nativeDebitRelationship:'unresolved'}},contractBalances:useNativeBalance?Object.fromEntries([...item.balance].map(([type,n])=>[type.raw,n.toString()])):{},acceptance:'uncertified-I2-observation'};
   try{outcomes.push({stage:item.stage,status:comparator.verifyStage(item.stage,{receipt,state}).status});}catch(e){outcomes.push({stage:item.stage,status:'REJECTED',error:e.message});break;}
  }
  reproductions.push({balanceSource:useNativeBalance?'actual-native-ContractState.balance':'actual-empty-indexer-contract_balances',outcomes});
 }
 loaded.assertFresh();
 const result={schema:'moriarty.swap-public-decode/1',records,reproductions,scope:'Actual stopped indexed state and retained native bytes. Reproduction replaces ONLY decoded role capabilities with synthetic test-secret commitments and supplies synthetic finalized-head context matching actual inclusion. No original private access; not unchanged secret-bound acceptance, independent canonical finality or complete financial acceptance.'};
 const json=JSON.stringify(result,(_,x)=>typeof x==='bigint'?x.toString():x instanceof Uint8Array?Buffer.from(x).toString('hex'):x,2)+'\n';writeFileSync(new URL('decoded-public.json',root),json);console.log(json);
}finally{await loaded.cleanup();}
