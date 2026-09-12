import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const candidate=process.argv[2]+'/experiments/moriarty-language';
const baseline='/home/charl/Moriarty/.worktrees/financial-state-reads/experiments/moriarty-language';
const {canonical}=await import(pathToFileURL(baseline+'/src/successor/expression-wire-v1.ts'));
const state=readFileSync(baseline+'/spec/successor/examples/expression-funded-payment.state.json','utf8');
const original=readFileSync(baseline+'/spec/successor/examples/source-defined-payment.mori','utf8');
const snap=JSON.parse(readFileSync(baseline+'/spec/successor/examples/expression-funded-payment.snapshots.json','utf8'));
const variants=[['original',original]];
for(const n of ['post_outstanding','post_principal','post_accrued','post_balance','post_allowance_remaining','post_allowance_spent']){
 variants.push([n+' action',original.replace('action pay(',`action ${n}(`)]);
 variants.push([n+' parameter',original.replace(/\bfirst\b/g,n)]);
 variants.push([n+' local',original.replaceAll('payment',n)]);
 variants.push([n+' state',original.replaceAll('paid',n)]);
}
for(const [label,mutate]of [['missing Repay',s=>s.replace('operation Repay: RepayFields;','')],['duplicate parameter',s=>s.replace('transferId: Text,','transferId: Text, transferId: Text,')],['compound invalid',s=>s.replace('operation Repay: RepayFields;','').replace('transferId: Text,','transferId: Text, transferId: Text,')],['invalid pure call',s=>s.replace('magnitude(nominal)','principal<Cash>("Due100")')]])variants.push([label,mutate(original)]);
let passed=0;const failures=[];
for(const version of[1,2,3]){
 const name=`createFinancialAgreementSourceV${version}`;
 const old=(await import(pathToFileURL(baseline+`/src/successor/financial-agreement-source-v${version}.ts`)))[name]();
 const now=(await import(pathToFileURL(candidate+`/src/successor/financial-agreement-source-v${version}.ts`)))[name]();
 for(const [label,text]of variants){const s=version===1?text:text.replace('source/1',`source/${version}`);for(const method of['check','elaborate','evaluate']){
  const localSnap=structuredClone(snap); if(label.endsWith(' parameter')){localSnap.Args[label.split(' ')[0]]=localSnap.Args.first;delete localSnap.Args.first;} if(label.endsWith(' state')){localSnap.Pre[label.split(' ')[0]]=localSnap.Pre.paid;delete localSnap.Pre.paid;} const action=s.match(/action\s+([A-Za-z][A-Za-z0-9_]*)\(/)?.[1]??'repay';const args=method==='evaluate'?(version!==1?[s,action,canonical(localSnap),state]:[s,canonical(localSnap),state]):[s];
  let expected,actual;try{expected=old[method](...args);}catch(e){expected={threw:e.message};}try{actual=now[method](...args);}catch(e){actual={threw:e.message};}
  try{assert.deepEqual(actual,expected);passed++;}catch{failures.push({version,label,method,expected,actual});}
 }}
}
console.log(JSON.stringify({passed,failed:failures.length,failures},null,2));if(failures.length)process.exitCode=1;
