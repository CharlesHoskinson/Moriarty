import fs from 'node:fs';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
const root=process.argv[2],here=path.dirname(new URL(import.meta.url).pathname);
const oracle=JSON.parse(fs.readFileSync(path.join(here,'../design-review/lifecycle-source-expectations.json'),'utf8'));
const work=JSON.parse(fs.readFileSync(path.join(here,'root-work-expectations.json'),'utf8'));
const example=path.join(root,'experiments/moriarty-language/examples/loan-lifecycle.mjs');
const imported=await import(pathToFileURL(example).href);const result=imported.runLoanLifecycleDemo();
assert.equal(result.checked.judgmentResult,'SourceChecked');assert.equal(result.calls.length,6);
const expected=oracle.stages.map(s=>{const e=structuredClone(s.expectedResultTemplate),w=work.actions[s.action];e.financialPost.work={remaining:w.remaining,spent:w.spent,closureReserve:'16'};e.workRemaining=w.remaining;return e;});
const seed=structuredClone(oracle.seed);seed.financial.work.remaining='512';
const prevs=[{post:seed.ordinary,financialPost:seed.financial},expected[0],expected[1],expected[1],expected[2],expected[3]];
const actions=['originate','accrue','accrue','repay','settle','settle'];
for(const [i,call] of result.calls.entries()){
 assert.equal(call.action,actions[i]);const snapshot=JSON.parse(call.snapshotJSON);assert.deepEqual(snapshot.Pre,prevs[i].post);assert.equal(snapshot.workInitial,prevs[i].financialPost.work.remaining);assert.deepEqual(snapshot.Obs,{});assert.deepEqual(JSON.parse(call.financialJSON),prevs[i].financialPost);assert.deepEqual(call.snapshot,snapshot);assert.deepEqual(call.financialPre,prevs[i].financialPost);
}
for(const [i,callIndex] of [0,1,3,4].entries())assert.deepEqual(result.calls[callIndex].result,expected[i]);
for(const [i,code] of [[2,'DUPLICATE'],[5,'GUARD_FAILED']]){const r=result.calls[i].result;assert.equal(r.status,'Rejected');assert.equal(r.code,code);for(const k of ['post','financialPost','effects','workRemaining'])assert.equal(Object.hasOwn(r,k),false);}
for(const i of [4,5])assert.deepEqual(Object.keys(result.calls[i].snapshot.Args).sort(),['allocationId','transferId']);
const direct=spawnSync(process.execPath,[example],{cwd:root,encoding:'utf8'});assert.equal(direct.status,0,direct.stderr);assert.deepEqual(JSON.parse(direct.stdout),result);
const importOnly=spawnSync(process.execPath,['--input-type=module','-e',`await import(${JSON.stringify(pathToFileURL(example).href)})`],{cwd:root,encoding:'utf8'});assert.equal(importOnly.status,0,importOnly.stderr);assert.equal(importOnly.stdout,'');
console.log(JSON.stringify({passed:true,actualCallsVerified:6,completeResultsVerified:4,actualRejectionsVerified:2,settlementComputedInSource:true,standaloneOutputMatched:true,importSilent:true,finalDebt:result.settled.financialPost.obligations[0].outstanding,finalWork:result.settled.financialPost.work}));
