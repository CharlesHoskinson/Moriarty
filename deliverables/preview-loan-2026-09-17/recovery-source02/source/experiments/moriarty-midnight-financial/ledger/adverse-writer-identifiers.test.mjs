import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,chmodSync,rmSync,existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {retainPublicIntegrationResult,retainPublicAdverseCandidate} from './launch-local.mjs';
const run=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-stale-loan-01/run-public/',import.meta.url);
// Actual public candidate from the failed attempt; other result fields below
// are controlled. No lost actual after-state or integration result is recreated.
const candidate=JSON.parse(readFileSync(new URL('adverse-candidate.json',run)));
const raw=readFileSync(new URL('adverse-candidate.bin',run));
const captured=candidate.identifiers;
assert.ok(captured.length>0&&captured.every(id=>/^[a-f0-9]{66}$/.test(id)));
function result(identifiers){return {
 schema:'moriarty.local-financial-integration/1',status:'INCOMPLETE',kind:'loan',sourceTestOnly:false,
 networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,
 build:undefined,assetBindings:undefined,phase:'adverse',driver:undefined,
 cleanup:{status:'INCOMPLETE'},setupPendingOperations:0,comparisons:[],financialComparison:undefined,scope:'Controlled durable-writer regression; actual after-state unknown',
 adverse:{status:'OUTCOME_UNKNOWN',financialNonmutationEstablished:false,candidate,reservations:{reservedSubmissions:1,reservedDustFee:'1000000000000000',reservedGrossByAsset:{},identifiers,pendingOperations:0}}
};}
for(const [label,ids] of [['synthetic 64-hex control',['ab'.repeat(32)]],['actual attempted SDK 66-hex identifiers',captured]])test('writer retains incomplete result with '+label,async()=>{
 const d=mkdtempSync(join(tmpdir(),'moriarty-id-repro-'));chmodSync(d,0o700);
 try{await retainPublicAdverseCandidate(d,{raw,transaction:candidate});retainPublicIntegrationResult(d,result(ids));assert.deepEqual(JSON.parse(readFileSync(join(d,'integration-result.json'))).adverse.reservations.identifiers,ids);}
 finally{rmSync(d,{recursive:true,force:true});}
});
test('reservation identifiers reject malformed values without retaining a result',async()=>{
 const d=mkdtempSync(join(tmpdir(),'moriarty-id-reject-'));chmodSync(d,0o700);
 try{
  await retainPublicAdverseCandidate(d,{raw,transaction:candidate});
  for(const bad of ['',captured[0].toUpperCase(),'0x'+captured[0],'gg', 'a'.repeat(257),null,{},1]){
   assert.throws(()=>retainPublicIntegrationResult(d,result([bad])),{message:'LAUNCH_ADVERSE_RESERVATIONS'});
   assert.equal(existsSync(join(d,'integration-result.json')),false);
  }
 }finally{rmSync(d,{recursive:true,force:true});}
});
