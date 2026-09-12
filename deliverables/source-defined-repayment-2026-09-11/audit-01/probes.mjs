import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
const root='/home/charl/Moriarty/.worktrees/source-defined-repayment';
const out='/home/charl/Moriarty/deliverables/source-defined-repayment-2026-09-11/audit-01';
const src=root+'/experiments/moriarty-language/src';
const ex=root+'/experiments/moriarty-language/spec/successor/examples';
const {createFinancialAgreementSourceV1}=await import(src+'/successor/financial-agreement-source-v1.ts');
const {createFundedFinancialExpressionSourceV1}=await import(src+'/successor/funded-expression-source-v1.ts');
const {createFinancialExpressionSourceV1}=await import(src+'/successor/financial-expression-source-v1.ts');
const {canonical}=await import(src+'/successor/expression-wire-v1.ts');
const {formatFinancialAgreementSource}=await import(src+'/successor/financial-agreement-source-frontend.ts');
const read=n=>readFileSync(ex+'/'+n,'utf8');
const s=read('source-defined-payment.mori'), snap=read('expression-funded-payment.snapshots.json'), state=read('expression-funded-payment.state.json');
const schema=read('expression-funded-payment.schema.json'), old=read('expression-funded-payment.mori');
const api=createFinancialAgreementSourceV1(); const records=[];
const check=(name,f)=>{try {f();records.push({name,pass:true});}catch(e){records.push({name,pass:false,error:String(e)});}};
const evaluate=(source=s,a=JSON.parse(snap),b=JSON.parse(state))=>api.evaluate(source,canonical(a),JSON.stringify(b));
function rejected(r,code) {assert.equal(r.status,'Rejected',JSON.stringify(r));if(code)assert.equal(r.code,code);for(const k of ['post','financialPost','effects','descriptors','result'])assert.equal(k in r,false);}
const base=evaluate();
check('baseline exact schema/result',()=>{assert.equal(canonical(api.elaborate(s).schema),schema);assert.deepEqual(base,createFundedFinancialExpressionSourceV1(schema).evaluate(old,snap,state));assert.equal(base.financialPost.obligations[0].principal,'70');});
check('actual E+N and exact boundary',()=>{const expr=createFinancialExpressionSourceV1(schema).evaluate(old,snap);const E=100n-BigInt(expr.workRemaining);assert.equal(BigInt(base.financialPost.work.spent),E+2n);const a=JSON.parse(snap),b=JSON.parse(state);a.workInitial=b.work.remaining=String(E+2n);const r=evaluate(s,a,b);assert.equal(r.status,'FundedExpressionPrepared');assert.equal(r.workRemaining,'0');assert.equal(r.financialPost.work.closureReserve,'16');a.workInitial=b.work.remaining=String(E+1n);rejected(evaluate(s,a,b),'INSUFFICIENT_WORK');});
check('short circuit work uses actual reductions',()=>{const source=s.replace('requires pre.due > 0;','requires true or pre.due > 0;');const r=evaluate(source);assert.equal(r.status,'FundedExpressionPrepared');assert.ok(BigInt(api.check(source).staticWorkBound)>100n-BigInt(r.workRemaining)-2n);});
check('interest-first plus complete continuation residual',()=>{const a=JSON.parse(snap),b=JSON.parse(state);a.Args.first='7';a.Args.second='0';b.obligations[0].accrued='10';b.obligations[0].outstanding='110';b.usedAllocationIds=['EarlierA'];b.usedTransferIds=['EarlierT'];b.balances.push({party:'Other',asset:'Cash',amount:'3'});const r=evaluate(s,a,b);assert.equal(r.financialPost.obligations[0].principal,'100');assert.equal(r.financialPost.obligations[0].accrued,'3');a.Pre=r.post;a.Args.allocationId='NextA';a.Args.transferId='NextT';a.workInitial=r.workRemaining;const r2=evaluate(s,a,r.financialPost);assert.equal(r2.financialPost.obligations[0].principal,'96');assert.deepEqual(r2.financialPost.usedTransferIds,['EarlierT','T1','NextT']);assert.equal(r2.financialPost.balances[2].amount,'3');});
for(const [name,mutate,code] of [
['allowance',(a,b)=>b.allowances[0].remaining='29','INSUFFICIENT_ALLOWANCE'],
['duplicate-id',(a,b)=>b.usedTransferIds.push('T1'),'DUPLICATE'],
['denomination',(a,b)=>b.obligations[0].denomination='USD','NOMINAL_UNIT'],
['wrong-creditor',(a,b)=>b.obligations[0].creditor='Other','TRANSFER_MISMATCH'],
['work-mismatch',(a,b)=>a.workInitial='99','WORK_MISMATCH'],
['missing-pre',(a,b)=>delete a.Pre.due,'INPUT_SCHEMA'],
]) check(name+' atomic',()=>{const a=JSON.parse(snap),b=JSON.parse(state);mutate(a,b);rejected(evaluate(s,a,b),code);});
for(const [name,source,code] of [
['guard',s.replace('requires payment > 0;','requires false;'),'GUARD_FAILED'],
['ensure',s.replace('ensures post.paid == pre.paid + payment;','ensures false;'),'ENSURES_FAILED'],
['settlement-witness',s.replace('settlementAsset: "Cash"','settlementAsset: "USD"'),'SETTLEMENT_UNIT'],
['unfunded',s.replace('amount<Cash>(payment)','amount<Cash>(1)'),'INSUFFICIENT_UNALLOCATED'],
['reverse-order',s.replace(/(    emit Transfer[\s\S]+?};)\n(    emit Repay[\s\S]+?};)/, '$2\n$1'),'TRANSFER_NOT_IN_STEP']
])check(name+' atomic',()=>rejected(evaluate(source),code));
check('ordinary financial-looking field cannot replace projection',()=>{const source=s.replace('state paid: UInt128;','state paid: UInt128; state outstanding: UInt128;').replace('next.paid =','next.outstanding = 0; next.paid =').replace('ensures post.paid','ensures post.outstanding == 0; ensures post.paid');const a=JSON.parse(snap);a.Pre.outstanding='999';const r=evaluate(source,a);assert.equal(r.post.outstanding,'0');assert.equal(r.financialPost.obligations[0].outstanding,'70');});
check('owned Core output cannot inject execution',()=>{const elaborated=api.elaborate(s);elaborated.core.statements=[];elaborated.schema.operations={};assert.deepEqual(evaluate(),base);rejected(api.evaluate(elaborated,snap,state),'SOURCE_TYPE');});
check('object transports never invoke getters',()=>{const hostile=new Proxy({}, {get(){throw Error('getter executed');}});rejected(api.evaluate(hostile,snap,state),'SOURCE_TYPE');rejected(api.evaluate(s,hostile,state),'INPUT_SCHEMA');rejected(api.evaluate(s,snap,hostile),'INPUT_SCHEMA');});
check('nested forward references and formatting preserve schema',()=>{const source=s.replace('state paid: UInt128;','state paid: UInt128; record A { b: Option<Record<B>>; } record B { c: Text; }');assert.equal(api.check(source).judgmentResult,'SourceChecked');const formatted=formatFinancialAgreementSource(source);assert.equal(formatFinancialAgreementSource(formatted),formatted);assert.deepEqual(api.elaborate(source).schema,api.elaborate(formatted).schema);});
const diagnosticCases=[['reserved-state',s.replace('state paid: UInt128;','state some: UInt128;')],['reserved-param',s.replace('first: Quantity','some: Quantity')],['operation-cycle',s.replace('payer: Text;','payer: Operation<Repay>;')]];
for(const [name,source] of diagnosticCases){const prefixed='// λ🙂 original offsets\n'+source;writeFileSync(out+'/'+name+'.mori',prefixed);records.push({name,expected:'source span on offending declaration/type',actual:api.check(prefixed),finding:true});}
function cli(name,args,exit=0){const r=spawnSync(process.execPath,[src+'/cli.ts',...args],{encoding:'utf8',timeout:10000});writeFileSync(out+'/'+name+'.stdout',r.stdout);writeFileSync(out+'/'+name+'.stderr',r.stderr);check(name,()=>{assert.equal(r.error,undefined);assert.equal(r.status,exit,r.stderr);if(exit)assert.equal(r.stdout,'');else assert.equal(r.stderr,'');});return r;}
const profile='moriarty-financial-agreement-source/1', file=ex+'/source-defined-payment.mori';
cli('cli-check',['check','--profile',profile,file]);
const formatted=cli('cli-format',['format','--profile',profile,file]);writeFileSync(out+'/formatted.mori',formatted.stdout);
const simulated=cli('cli-simulate',['simulate','--profile',profile,'--snapshots',ex+'/expression-funded-payment.snapshots.json','--repayment-state',ex+'/expression-funded-payment.state.json',file]);check('CLI success equals independent API',()=>assert.deepEqual(JSON.parse(simulated.stdout).result,base));
cli('cli-no-schema',['check','--profile',profile,'--schema',ex+'/expression-funded-payment.schema.json',file],2);
for(const [name] of diagnosticCases)cli('cli-'+name,['check','--profile',profile,out+'/'+name+'.mori'],1);
writeFileSync(out+'/bad-snapshot.json','{');cli('cli-bad-snapshot',['simulate','--profile',profile,'--snapshots',out+'/bad-snapshot.json','--repayment-state',ex+'/expression-funded-payment.state.json',file],1);
writeFileSync(out+'/probe-results.json',JSON.stringify(records,null,2));console.log(JSON.stringify({passed:records.filter(x=>x.pass).length,unexpectedFailures:records.filter(x=>x.pass===false),diagnosticFindings:records.filter(x=>x.finding)},null,2));
