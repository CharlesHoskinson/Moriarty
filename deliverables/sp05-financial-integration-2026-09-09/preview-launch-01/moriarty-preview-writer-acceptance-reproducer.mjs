import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {retainPreviewIntegrationResult} from '/home/charl/Moriarty/.worktrees/sp05-preview-owner/experiments/moriarty-midnight-financial/ledger/preview-launch.mjs';
const root='/home/charl/Moriarty/.worktrees/sp05-preview-owner';
const results=[];
for(const value of [true,'PASS',1]){
 const x=JSON.parse(readFileSync(root+'/deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/integration-result.json'));
 x.schema='moriarty.preview-financial-integration/1';x.status='FAILED';x.driver.schema='moriarty.preview-financial-run/1';x.financialComparison.proofAcceptance=value;
 const d=mkdtempSync(join(tmpdir(),'moriarty-preview-gpt6-negative-'));let rejected=false,code;
 try{try{retainPreviewIntegrationResult(d,x);}catch(e){rejected=true;code=e.message;}results.push({inputType:typeof value,input:value,rejected,code,retainedValue:rejected?undefined:JSON.parse(readFileSync(join(d,'integration-result.json'))).financialComparison.proofAcceptance});}finally{rmSync(d,{recursive:true,force:true});}
}
console.log(JSON.stringify({scope:'Controlled public fixture; no live operation',results},null,2));
assert.ok(results.every(r=>r.rejected),'Every truthy non-false nested acceptance value must be rejected');
