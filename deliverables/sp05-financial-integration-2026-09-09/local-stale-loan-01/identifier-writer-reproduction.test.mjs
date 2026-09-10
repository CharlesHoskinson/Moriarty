import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,chmodSync,rmSync,existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {retainPublicIntegrationResult} from '/home/charl/Moriarty/.worktrees/sp05-adverse-integration/experiments/moriarty-midnight-financial/ledger/launch-local.mjs';
const R='/home/charl/Moriarty/.worktrees/sp05-adverse-integration';
// Existing retained PUBLIC SDK/indexer identifiers, not invented hashes and
// not a reconstruction of the unavailable actual post-rejection observation.
const captured=JSON.parse(readFileSync(R+'/experiments/moriarty-midnight-financial/ledger/fixtures/historical-dust/response.json')).data.transactions[0].identifiers;
assert.ok(captured.length>0&&captured.every(id=>/^[a-f0-9]{66}$/.test(id)));
function result(identifiers){return {
 schema:'moriarty.local-financial-integration/1',status:'INCOMPLETE',kind:'loan',sourceTestOnly:false,
 networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,
 build:undefined,assetBindings:undefined,phase:'adverse',driver:undefined,
 cleanup:{status:'INCOMPLETE'},setupPendingOperations:0,comparisons:[],financialComparison:undefined,scope:'Controlled durable-writer regression; actual after-state unknown',
 adverse:{status:'OUTCOME_UNKNOWN',financialNonmutationEstablished:false,reservations:{reservedSubmissions:1,reservedDustFee:'1000000000000000',reservedGrossByAsset:{},identifiers,pendingOperations:0}}
};}
for(const [label,ids] of [['synthetic 64-hex control',['ab'.repeat(32)]],['actual retained SDK 66-hex identifiers',captured]])test('writer retains incomplete result with '+label,()=>{
 const d=mkdtempSync(join(tmpdir(),'moriarty-id-repro-'));chmodSync(d,0o700);
 try{retainPublicIntegrationResult(d,result(ids));assert.deepEqual(JSON.parse(readFileSync(join(d,'integration-result.json'))).adverse.reservations.identifiers,ids);}
 finally{rmSync(d,{recursive:true,force:true});}
});
console.log(JSON.stringify({fixture:'retained historical public identifiers',identifiers:captured,lengths:captured.map(x=>x.length),actualCurrentAfterState:'UNKNOWN'}));
