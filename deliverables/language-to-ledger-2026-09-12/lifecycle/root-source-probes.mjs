import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
const root=process.argv[2];
const evidence=path.dirname(new URL(import.meta.url).pathname);
const expectation=JSON.parse(fs.readFileSync(path.join(evidence,'../design-review/lifecycle-source-expectations.json'),'utf8'));
const source=fs.readFileSync(path.join(root,'experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori'),'utf8');
const work=JSON.parse(fs.readFileSync(path.join(evidence,'root-work-expectations.json'),'utf8'));
assert.equal(createHash('sha256').update(source).digest('hex'),work.sourceSHA256,'Source changed: independently recount work before using this oracle.');
const mod=async name=>import(pathToFileURL(path.join(root,'experiments/moriarty-language/src/successor',name)).href);
const {createFinancialAgreementSourceV5}=await mod('financial-agreement-source-v5.ts');
const {admitFinancialLifecycleStateJSON}=await mod('financial-lifecycle.ts');
const {canonical}=await mod('expression-wire-v1.ts');
const api=createFinancialAgreementSourceV5();
const copy=x=>JSON.parse(JSON.stringify(x));
const seed=copy(expectation.seed);seed.financial.work.remaining='512';
const stages=expectation.stages;
const compiled=api.elaborate(source);assert.equal(compiled.judgmentResult,'SourceElaborated');
assert.deepEqual(compiled.actions.map(a=>a.action),['originate','accrue','repay','settle']);
const argsFor=(stage,extra={})=>{
 const defaults={nominal:stage.action==='originate'?'100':'30',cap:'110',...stage.args,...extra};
 const args={};for(const key of Object.keys(compiled.actions.find(a=>a.action===stage.action).schema.args)){assert.ok(Object.hasOwn(defaults,key),`independently specify argument ${key}`);args[key]=defaults[key];}return args;
};
const invoke=(src,action,pre,fin,args)=>api.evaluate(src,action,canonical({Args:args,Obs:{},Pre:pre,workInitial:fin.work.remaining}),JSON.stringify(fin));
const absent=r=>{assert.equal(r.status,'Rejected');for(const k of ['post','financialPost','effects','descriptors','continuation','continueSuffix','workRemaining'])assert.equal(Object.hasOwn(r,k),false,k);};
const rows=[];const test=(name,fn)=>{try{fn();rows.push({name,pass:true});}catch(e){rows.push({name,pass:false,error:e.stack});}};
const actual=[];let pre=copy(seed.ordinary),fin=copy(seed.financial),sum=0;
for(const [i,stage] of stages.entries())test(`complete source result ${stage.name}`,()=>{
 sum+=work.actions[stage.action].total;
 const expected=copy(stage.expectedResultTemplate);expected.financialPost.work={remaining:String(512-sum),spent:String(17+sum),closureReserve:'16'};expected.workRemaining=expected.financialPost.work.remaining;
 const before=JSON.stringify({pre,fin});const r=invoke(source,stage.action,pre,fin,argsFor(stage));assert.equal(JSON.stringify({pre,fin}),before);assert.deepEqual(r,expected);assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(r.financialPost)).ok,true);actual.push(copy(r));pre=r.post;fin=r.financialPost;
});
assert.equal(actual.length,4,'The complete independent positive trace must pass before negative probes.');
const prior=i=>i===0?{post:copy(seed.ordinary),financialPost:copy(seed.financial)}:copy(actual[i-1]);
for(const c of expectation.negativeCases.filter(c=>c.args))test(c.name,()=>{
 const p=prior(c.predecessor),before=JSON.stringify(p),stage=stages.find(s=>s.action===c.action);const args=argsFor(stage,c.args);const r=invoke(source,c.action,p.post,p.financialPost,args);absent(r);assert.equal(r.code,c.code);assert.deepEqual(invoke(source,c.action,p.post,p.financialPost,args),r);assert.equal(JSON.stringify(p),before);
});
for(const [i,stage] of stages.entries()){
 test(`exact work for ${stage.action}`,()=>{const p=prior(i),W=work.actions[stage.action].total;p.financialPost.work.remaining=String(W);const r=invoke(source,stage.action,p.post,p.financialPost,argsFor(stage));assert.equal(r.status,'FundedExpressionPrepared');assert.deepEqual(r.financialPost.work,{remaining:'0',spent:String(BigInt(p.financialPost.work.spent)+BigInt(W)),closureReserve:'16'});});
 test(`late work exhaustion for ${stage.action}`,()=>{const p=prior(i);p.financialPost.work.remaining=String(work.actions[stage.action].total-1);const r=invoke(source,stage.action,p.post,p.financialPost,argsFor(stage));absent(r);assert.equal(r.code,'WORK_EXHAUSTED');});
}
test('ordinary phase offset survives the complete source trace',()=>{let p={phase:'13',paid:'0'},f=copy(seed.financial);for(const [i,s] of stages.entries()){const r=invoke(source,s.action,p,f,argsFor(s));assert.equal(r.status,'FundedExpressionPrepared');assert.deepEqual(r.post,{phase:String(14+i),paid:stages[i].expectedResultTemplate.post.paid});assert.deepEqual(r.financialPost,actual[i].financialPost);p=r.post;f=r.financialPost;}});
test('late false final settlement rolls back all tentative results',()=>{
 const rx=/post_balance<Cash>\(\s*"Lender"\s*\)\s*==\s*amount<Cash>\(110\)/g;const matches=[...source.matchAll(rx)];assert.ok(matches.length);const m=matches.at(-1);const bad=source.slice(0,m.index)+m[0].replace('(110)','(111)')+source.slice(m.index+m[0].length);const p=prior(3),args=argsFor(stages[3]);const r=invoke(bad,'settle',p.post,p.financialPost,args);absent(r);assert.equal(r.code,'ENSURES_FAILED');assert.equal(r.workUsed,String(work.actions.settle.total));assert.deepEqual(invoke(source,'settle',p.post,p.financialPost,args),actual[3]);
});
test('missing current origination funding rejects',()=>{const bad=source.replace(/emit\s+Transfer\s*\{[^}]*\}\s*;/,'');assert.notEqual(bad,source);const r=invoke(bad,'originate',seed.ordinary,seed.financial,argsFor(stages[0]));absent(r);assert.equal(r.code,'TRANSFER_NOT_IN_STEP');});
test('unrelated obligation and all original histories survive each source step',()=>{
 const old=copy(actual[0].financialPost.obligations[0]);Object.assign(old,{id:'OldLoan',debtor:'OldBorrower',creditor:'OldLender',denomination:'Token',settlementAsset:'Token',originationId:'OldOrigin',originationTransferId:'OldTransfer'});
 let p=copy(seed.ordinary),f=copy(seed.financial);f.obligations=[old];f.usedTransferIds=['OldTransfer'];f.usedOriginationIds=['OldOrigin'];assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(f)).ok,true);
 for(const [i,s] of stages.entries()){const r=invoke(source,s.action,p,f,argsFor(s));assert.equal(r.status,'FundedExpressionPrepared');const expected=copy(actual[i]);expected.financialPost.obligations.unshift(old);expected.financialPost.usedTransferIds.unshift('OldTransfer');expected.financialPost.usedOriginationIds.unshift('OldOrigin');assert.deepEqual(r,expected);p=r.post;f=r.financialPost;}
});
console.log(JSON.stringify({passed:rows.filter(r=>r.pass).length,failed:rows.filter(r=>!r.pass).length,rows}));
process.exitCode=rows.some(r=>!r.pass)?1:0;
