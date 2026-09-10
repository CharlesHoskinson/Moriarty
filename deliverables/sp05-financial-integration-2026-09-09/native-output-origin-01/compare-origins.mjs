import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from '../../../experiments/moriarty-midnight-financial/ledger/providers.mjs';
const start=performance.now(),ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const raw=readFileSync(new URL('../local-recovery-03/run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin',import.meta.url));
const indexed=JSON.parse(readFileSync(new URL('../local-recovery-03/stopped-indexer-effects.json',import.meta.url)));
const tx=ledger.Transaction.deserialize('signature','proof','binding',raw),intents=[];
for(const [segment,intent] of tx.intents){
 const zero=intent.intentHash(0),physical=intent.intentHash(segment);
 const outputs=[];
 for(const [section,offer] of [['guaranteed',intent.guaranteedUnshieldedOffer],['fallible',intent.fallibleUnshieldedOffer]])for(const [index,output] of (offer?.outputs??[]).entries()){
  const row=indexed.rows.find(r=>r.ownerRaw===output.owner&&r.tokenTypeRaw===output.type&&r.output_index===index);
  outputs.push({section,index,owner:output.owner,type:output.type,value:output.value.toString(),indexedIntentHash:row?.intentHash,matchesZero:row?.intentHash===zero,matchesPhysical:row?.intentHash===physical});
 }
 intents.push({physicalSegment:segment,intentHashZero:zero,intentHashPhysical:physical,outputs});
}
console.log(JSON.stringify({schema:'moriarty.native-output-origin-observation/1',transactionSha256:createHash('sha256').update(raw).digest('hex'),ledgerVersion:JSON.parse(readFileSync(join(PINNED_NM,'@midnight-ntwrk/ledger-v8/package.json'))).version,wasmSha256:createHash('sha256').update(readFileSync(join(PINNED_NM,'@midnight-ntwrk/ledger-v8/midnight_ledger_wasm_bg.wasm'))).digest('hex'),intents,elapsedMs:performance.now()-start,scope:'Actual retained public transaction through pinned native API compared with actual stopped-indexer output rows; not finality or complete financial acceptance'},null,2));
