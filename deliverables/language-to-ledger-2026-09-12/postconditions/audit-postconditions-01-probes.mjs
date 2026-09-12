import assert from 'node:assert/strict';
import fs from 'node:fs';
import {spawnSync} from 'node:child_process';
import {pathToFileURL} from 'node:url';
const root=process.argv[2];
process.chdir(root+'/experiments/moriarty-language');
const load=p=>import(pathToFileURL(process.cwd()+'/'+p));
const {createFinancialAgreementSourceV4}=await load('src/successor/financial-agreement-source-v4.ts');
const {createFinancialExpressionContractV3}=await load('src/successor/financial-expression-v3.ts');
const exports=await load('src/successor/financial-expression-v1.ts');
const {canonical:J}=await load('src/successor/expression-wire-v1.ts');
const src=fs.readFileSync('spec/successor/examples/financial-postconditions-payment.mori','utf8');
const state=JSON.parse(fs.readFileSync('spec/successor/examples/financial-state-payment.state.json','utf8'));
const api=createFinancialAgreementSourceV4();
const snap={Args:{allocationId:'AuditA',transferId:'AuditT'},Obs:{},Pre:{due:'100',paid:'0'},workInitial:'256'};
let assertions=0; const eq=(a,b)=>{assert.deepEqual(a,b);assertions++};
const run=(s=src,st=state,sp=snap)=>api.evaluate(s,'repay_remaining',J(sp),JSON.stringify(st));
const ok=run();eq(ok.status,'FundedExpressionPrepared');eq(ok.financialPost.work,{remaining:'174',spent:'82',closureReserve:'16'});eq(ok.financialPost.obligations[0].outstanding,'0');
for(const [w,code,used] of [[82,undefined,undefined],[81,'WORK_EXHAUSTED','81'],[43,'WORK_EXHAUSTED','43'],[42,'INSUFFICIENT_WORK',undefined],[40,'WORK_EXHAUSTED','40']]) {const st=structuredClone(state);st.work={remaining:String(w),spent:'19',closureReserve:'16'};const r=run(src,st,{...snap,workInitial:String(w)});if(code){eq(r.code,code);if(used)eq(r.workUsed,used);for(const k of ['post','financialPost','effects','descriptors','continueSuffix','workRemaining'])eq(k in r,false);}else{eq(r.financialPost.work,{remaining:'0',spent:'101',closureReserve:'16'});}}
for(const [s,used] of [[src.replace('post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0)','post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(1)'),'54'],[src.replace('post_allowance_spent<Cash>("Payer") == amount<Cash>(100)','post_allowance_spent<Cash>("Payer") == amount<Cash>(99)'),'82']]){const r=run(s);eq(r.code,'ENSURES_FAILED');eq(r.workUsed,used);eq('financialPost' in r,false);eq('effects' in r,false);}
const held=structuredClone(ok);ok.financialPost.obligations[0].outstanding='999';ok.financialPost.work.spent='0';ok.effects.length=0;ok.post.paid='999';eq(run(),held);
const comp=api.elaborate(src).actions.find(a=>a.action==='repay_remaining');const request={contract:exports.FINANCIAL_EXPRESSION_CONTRACT_V3,source:src,core:comp.core,...snap};const core=createFinancialExpressionContractV3(J(comp.schema),JSON.stringify(state));eq(core.evaluate(J(request)),held);for(const k of ['financialPost','continueSuffix','callback'])eq(core.evaluate(J({...request,[k]:{}})).code,'INPUT_SCHEMA');eq(Object.isFrozen(core),true);eq(Object.isFrozen(api),true);eq('evaluateFinancialExpressionV3Prefix' in exports,false);
const missing=createFinancialExpressionContractV3(J(comp.schema));const rejection=missing.evaluate(J(request));rejection.nodePath.push('mutated');eq(missing.evaluate(J(request)).nodePath,[]);eq(Object.isFrozen(rejection.span),true);
const wrong=structuredClone(state);wrong.balances.push({...wrong.balances[0]});eq(run(src,wrong).code,'DUPLICATE');eq(run(src,state,{...snap,workInitial:'255',Pre:{bad:'x'}}),{status:'Rejected',code:'WORK_MISMATCH'});
const nominalSrc=src.replace('let nominal = outstanding<Cash>("Due100");','let nominal = quantity<Units<Cash,1>,0>(100);');const wrongUnit=structuredClone(state);wrongUnit.obligations[0].denomination='Else';eq(run(nominalSrc,wrongUnit).code,'NOMINAL_UNIT');
const short=src.slice(0,src.indexOf('    ensures post_outstanding'))+'    ensures true or post_balance<Cash>("Missing") == amount<Cash>(0);\n  }\n}\n';const st=structuredClone(state);st.work.remaining='52';const sr=run(short,st,{...snap,workInitial:'52'});eq(sr.status,'FundedExpressionPrepared');eq(sr.workRemaining,'0');
const cli=spawnSync(process.execPath,['src/cli.ts','simulate','--profile','moriarty-financial-agreement-source/4','--action','NoSuchAction','--snapshots','spec/successor/examples/financial-state-payment.snapshots.json','--repayment-state','spec/successor/examples/financial-state-payment.state.json','spec/successor/examples/financial-postconditions-payment.mori'],{encoding:'utf8'});eq(cli.status,1);eq(cli.stdout,'');eq(JSON.parse(cli.stderr).code,'SOURCE_ACTION_UNKNOWN');
console.log(JSON.stringify({status:'passed',assertions,cases:['exact budgets and prior spent','first/last ensure rollback','success output mutation isolation','source/Core full equality','closed request injections','frozen APIs/private staging','rejection mutation isolation','complete admission','work mismatch precedence','mapped repayment nominal mismatch','short circuit exact 52','CLI rejection envelope']}));
