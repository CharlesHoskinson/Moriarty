import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {evaluate} from '/home/charl/Moriarty/experiments/moriarty-developer-mock/dist/language/core.js';
import {elaborate,exampleSource,defaultAction} from '/home/charl/Moriarty/experiments/moriarty-developer-mock/dist/language/packages.js';
const root='/home/charl/Moriarty';
const sources=['experiments/moriarty-developer-mock/src/language/core.ts','experiments/moriarty-developer-mock/src/language/packages.ts'].map(path=>({path,sha256:crypto.createHash('sha256').update(fs.readFileSync(`${root}/${path}`)).digest('hex')}));
const cases={};
for(const kind of ['loan','swap']){
 const source=JSON.parse(exampleSource(kind));const bundle=elaborate(source);assert.equal(bundle.outcome,'elaborated');
 let state=structuredClone(bundle.initialState);const steps=[];
 for(let i=0;i<2;i++){
  const action=kind==='swap'&&i===1?{name:'close',args:{actor:'provider'}}:defaultAction(bundle,state);
  const result=evaluate(bundle.program,state,action);assert.equal(result.outcome,'evaluated',result.message);
  steps.push({action,result});state=result.after;
 }
 cases[kind]={source,program:bundle.program,initialState:bundle.initialState,steps};
}
assert.equal(cases.loan.steps[1].result.after.values.notional,'4500000000');
assert.equal(cases.swap.steps[0].result.effects[1].fields.amount,'19743');
const output={scope:'Executed legacy R2 financial reference for complete-field/effect comparison with future frontend. Not new DSL execution, conformance, PCD or ledger evidence.',sources,cases};
const dir=`${root}/evidence/moriarty-completion-program-2026-09-07/MC01/implementation-intake`;fs.mkdirSync(dir,{recursive:true});fs.writeFileSync(`${dir}/legacy-reference.json`,JSON.stringify(output,null,2)+'\n');
console.log(JSON.stringify({cases:Object.keys(cases),steps:4,loanFinal:cases.loan.steps[1].result.after,swapFinal:cases.swap.steps[1].result.after,output:`${dir}/legacy-reference.json`}));
