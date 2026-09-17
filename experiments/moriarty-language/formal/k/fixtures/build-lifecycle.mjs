import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import assert from 'node:assert/strict';
import { runLifecycleCorpus, compareLifecycleResult } from '../lifecycle-corpus.mjs';
import { createFinancialAgreementSourceV5 } from '../../../src/successor/financial-agreement-source-v5.ts';
import { createFinancialExpressionContractV4, FINANCIAL_EXPRESSION_CONTRACT_V4 } from '../../../src/successor/financial-expression-v1.ts';
import { canonical } from '../../../src/successor/expression-wire-v1.ts';

// Expected results below use frozen independent states, explicit arithmetic and
// constructor counting. Evaluator outputs are observations, never the oracle.
const copy = structuredClone;
const language = createFinancialAgreementSourceV5();
const corpus = await runLifecycleCorpus();
assert.equal(corpus.ok, true, JSON.stringify(corpus.failures));
const source = readFileSync(new URL('../../../spec/successor/examples/loan-lifecycle.mori', import.meta.url), 'utf8');
const header = source.slice(0, source.indexOf('  action originate('));
const cases = [];
const controls = [];
const sourceRoutes = new Map();
const U128 = (1n << 128n) - 1n, U64 = (1n << 64n) - 1n, M = (1n << 127n) - 1n;
const seed = JSON.parse(corpus.stages[0].source.input.financialPreState);
const stages = corpus.stages.map(x => copy(x.expected));
const fail = (code, actionIndex = 0) => ({ status: 'Rejected', code, actionIndex });
const adapter = code => ({ status: 'Rejected', code });
const packetOf = input => ({ schema: input.schema, request: input.request, financialPreState: input.financialPreState });

function add(id, requirementIds, packet, expected, derivation, extra = {}) {
  const observed = createFinancialExpressionContractV4(packet.schema, packet.financialPreState).evaluate(packet.request);
  const comparison = compareLifecycleResult(observed, expected);
  const route=sourceRoutes.get(packet.request);
  const sourceObserved=route ? language.evaluate(route.source,'probe',route.snapshot,packet.financialPreState) : undefined;
  const sourceMatches=route ? compareLifecycleResult(sourceObserved,expected) : undefined;
  cases.push({ id, requirementIds, packet, expected, derivation, observedCore: observed,
    expectedMatchesCore: comparison.ok, differences: comparison.differences,
    ...(route ? {observedSource:sourceObserved,expectedMatchesSource:sourceMatches.ok,sourceDifferences:sourceMatches.differences} : {}), ...extra });
  return cases.at(-1);
}
for (const [i, stage] of corpus.stages.entries()) add(`chain-${stage.action}`, [], packetOf(stage.core.input), stage.expected,
  'Frozen independent complete lifecycle template and hand-counted work [99,65,88,86].',
  { chain: { id: 'loan-lifecycle', step: i, previous: i ? `chain-${corpus.stages[i - 1].action}` : null }, action: stage.action });

const reused = {
  'duplicate-accrual': 'period-id-priority', 'repeated-period': 'period-order-priority',
  'early-period': 'period-time', 'incurred-cap-after-repay': 'cap-after-partial',
  'settled-source-guard': 'settled-source', 'late-false-ensure': 'late-ensure',
  'malformed-unrelated-allowance': 'unrelated-invalid', 'work-mismatch': 'work-mismatch',
};
for (const row of corpus.cases) {
  add(row.name, reused[row.name] ? [reused[row.name]] : [], packetOf(row.core.input), row.expected,
    'PR8 independent rejection oracle; expression diagnostics derived from source statement offsets and constructor counts.');
  if (row.core.retry) {
    const expected = copy(corpus.stages.find(s => s.action === row.retry).expected);
    if (row.name === 'insufficient-funding') {
      expected.financialPost.usedTransferIds=['D1','PF']; expected.financialPost.usedAllocationIds=['RF'];
      expected.effects[0].id='PF'; expected.effects[1].transferId='PF'; expected.effects[1].allocationId='RF';
    }
    add(`${row.name}-retry`, reused[row.name] ? [reused[row.name]] : [], packetOf(row.core.retry.input), expected,
      'Frozen independent lifecycle template with explicit RF/PF identity substitution when applicable.');
  }
}

function build(body, state, { declarations = '', prefix = '', suffix = '', work, schemaEdit, coreEdit } = {}) {
  const text = header + declarations + `  action probe() {\n${prefix}\n${body}\n${suffix}\n  }\n}\n`;
  const compiled = language.elaborate(text);
  assert.equal(compiled.judgmentResult, 'SourceElaborated', JSON.stringify(compiled));
  const a = compiled.actions[0];
  const schema = copy(a.schema), core = copy(a.core);
  schemaEdit?.(schema); coreEdit?.(core);
  const packet={ schema: canonical(schema), request: canonical({ contract: FINANCIAL_EXPRESSION_CONTRACT_V4,
    source: text, core, Pre: { paid: '0', phase: '0' }, Args: {}, Obs: {}, workInitial: work ?? state.work.remaining }),
    financialPreState: JSON.stringify(state) };
  if(!schemaEdit&&!coreEdit)sourceRoutes.set(packet.request,{source:text,snapshot:canonical({Pre:{paid:'0',phase:'0'},Args:{},Obs:{},workInitial:work??state.work.remaining})});
  return packet;
}
const txt = JSON.stringify;
const transfer = (id = 'T1', from = 'Lender', to = 'Borrower', amount = '100', asset = 'Cash') =>
  `emit Transfer { id: ${txt(id)}, from: ${txt(from)}, to: ${txt(to)}, settlementAsset: ${txt(asset)}, transferAmount: amount<Cash>(${amount}) };`;
const repay = (id = 'R1', tid = 'T1', nominal = '1', obligation = 'Loan1') =>
  `emit Repay { allocationId: ${txt(id)}, transferId: ${txt(tid)}, obligationId: ${txt(obligation)}, payer: "Borrower", nominalAmount: quantity<Units<Cash,1>,0>(${nominal}) };`;
const accrue = (id = 'A1', period = '1', time = '1060', obligation = 'Loan1') =>
  `emit Accrue { accrualId: ${txt(id)}, obligationId: ${txt(obligation)}, periodIndex: u64(${period}), observedTime: u64(${time}) };`;
const originate = ({ nominal = '100', cap = '110', id = 'Loan1', origin = 'O1', tid = 'T1', numerator = '1', denominator = '10', rounding = 'floor', start = '1000', duration = '60', mantissa = '1' } = {}) =>
  `emit Originate { obligationId: ${txt(id)}, transferId: ${txt(tid)}, originationId: ${txt(origin)}, debtor: "Borrower", creditor: "Lender", nominalAmount: quantity<Units<Cash,1>,0>(${nominal}), denomination: "Cash", settlementAsset: "Cash", conversion: record<ConversionFields>{mantissa:${mantissa},scale:0,rounding:"none"}, allocationRule:"AccrualFirst", accrualTerms:record<AccrualTermsFields>{numerator:${numerator},denominator:${denominator},rounding:${txt(rounding)},periodSeconds:u64(${duration}),firstPeriodStart:u64(${start})}, nominalLiabilityCap:quantity<Units<Cash,1>,0>(${cap}) };`;
function negative(id, req, body, state, expected, options = {}) {
  return add(id, [req], build(body, state, options), expected, options.derivation ?? 'Explicit canonical contract guard order; supplied state is an independently constructed fixture.');
}
function nodes(x) { if (!x || typeof x !== 'object') return 0; return (typeof x.constructor === 'string' ? 1 : 0) + Object.values(x).reduce((n, v) => n + nodes(v), 0); }
function successful(packet, postState, effects) {
  const r = JSON.parse(packet.request), expected = copy(postState);
  const count = nodes(r.core) + effects.length;
  // Action containers have no constructor and are not charged.
  expected.work = { remaining: String(BigInt(JSON.parse(packet.financialPreState).work.remaining) - BigInt(count)),
    spent: String(BigInt(JSON.parse(packet.financialPreState).work.spent) + BigInt(count)), closureReserve: JSON.parse(packet.financialPreState).work.closureReserve };
  return { status: 'FundedExpressionPrepared', post: r.Pre, financialPost: expected, effects, workRemaining: expected.work.remaining };
}
function independentlyOriginated({ principal = '100', cap = '110', numerator = '1', denominator = '10', rounding = 'floor', start = '1000', duration = '60' } = {}) {
  const f = copy(stages[0].financialPost), o = f.obligations[0];
  Object.assign(o, { principal, outstanding: principal, initialPrincipal: principal, liabilityIncurred: principal, nominalLiabilityCap: cap,
    accrualTerms: { numerator, denominator, rounding, firstPeriodStart: start, periodSeconds: duration }, nextAccrualAt: String(BigInt(start) + BigInt(duration)) });
  f.work = copy(seed.work);
  f.balances[1].amount = String(BigInt(principal) + 10n);
  f.allowances[0].spent = principal;
  return f;
}
function accrualExpected(state, id, period, observedTime, interest) {
  const f = copy(state), o = f.obligations[0], old = copy(o), i = BigInt(interest);
  Object.assign(o, { accrued: String(BigInt(o.accrued) + i), outstanding: String(BigInt(o.outstanding) + i),
    liabilityIncurred: String(BigInt(o.liabilityIncurred) + i), lastAccruedPeriod: period,
    nextAccrualAt: String(BigInt(o.accrualTerms.firstPeriodStart) + (BigInt(period) + 1n) * BigInt(o.accrualTerms.periodSeconds)) });
  f.usedAccrualIds.push(id);
  const effect = { kind: 'Accrual', accrualId: id, obligationId: o.id, debtor: o.debtor, creditor: o.creditor, denomination: o.denomination,
    periodIndex: period, observedTime, eligibleAt: old.nextAccrualAt, nextAccrualAt: o.nextAccrualAt, principalBasis: o.principal,
    numerator: o.accrualTerms.numerator, denominator: o.accrualTerms.denominator, rounding: o.accrualTerms.rounding,
    interestAmount: String(interest), previousAccrued: old.accrued, accrued: o.accrued, previousOutstanding: old.outstanding,
    outstanding: o.outstanding, previousLiabilityIncurred: old.liabilityIncurred, liabilityIncurred: o.liabilityIncurred, nominalLiabilityCap: o.nominalLiabilityCap };
  return { state: f, effect };
}
function originExpected(packet,initial,config={}) {
 const principal=config.principal??'100',cap=config.cap??'110';
 const f=copy(initial),o=copy(stages[0].financialPost.obligations[0]),effect=copy(stages[0].effects[1]);
 const terms={numerator:config.numerator??'1',denominator:config.denominator??'10',rounding:config.rounding??'floor',periodSeconds:'60',firstPeriodStart:'1000'};
 Object.assign(o,{id:config.id??'Loan1',originationId:config.origin??'O1',principal,outstanding:principal,initialPrincipal:principal,liabilityIncurred:principal,nominalLiabilityCap:cap,accrualTerms:terms,originationTransferId:'T1'});
 Object.assign(effect,{obligationId:o.id,originationId:o.originationId,nominalAmount:principal,settlementAmount:principal,liabilityIncurred:principal,nominalLiabilityCap:cap,accrualTerms:copy(terms),transferId:'T1'});
 f.balances[0].amount=String(BigInt(f.balances[0].amount)-BigInt(principal));f.balances[1].amount=String(BigInt(f.balances[1].amount)+BigInt(principal));
 f.allowances[0].remaining=String(BigInt(f.allowances[0].remaining)-BigInt(principal));f.allowances[0].spent=String(BigInt(f.allowances[0].spent)+BigInt(principal));
 f.obligations.push(o);f.usedTransferIds.push('T1');f.usedOriginationIds.push(o.originationId);
 return successful(packet,f,[{kind:'Transfer',id:'T1',from:'Lender',to:'Borrower',asset:'Cash',amount:principal},effect]);
}
function repay30Expected(packet,pre) {
 const f=copy(pre),o=f.obligations[0];
 f.balances[0].amount=String(BigInt(f.balances[0].amount)+30n);f.balances[1].amount=String(BigInt(f.balances[1].amount)-30n);
 f.allowances[1].remaining=String(BigInt(f.allowances[1].remaining)-30n);f.allowances[1].spent=String(BigInt(f.allowances[1].spent)+30n);
 Object.assign(o,{principal:'80',accrued:'0',outstanding:'80'});f.usedTransferIds.push('P1');f.usedAllocationIds.push('R1');
 return successful(packet,f,copy(stages[2].effects));
}
for(const [name,config,interest] of [
 ['floor',{principal:'101',cap:'112'},'10'],['ceil',{principal:'101',cap:'112',rounding:'ceil'},'11'],
 ['zero',{numerator:'0'},'0'],['cap118',{cap:'118'},'10'],
]) {
 const requirement=name==='cap118'?'cap-independent-alternate':name==='zero'?'zero-interest':'round-'+name;
 const initial=copy(seed),principal=config.principal??'100';initial.balances[0].amount=principal;initial.allowances[0].remaining=principal;
 const p=build(transfer('T1','Lender','Borrower',principal)+originate({nominal:principal,...config}),initial);
 const origin=originExpected(p,initial,config),chainId='alternate-'+name;
 add(chainId+'-origin',[requirement],p,origin,'Independent funded-origin template with explicit principal/cap/rate substitution; complete current-invocation transfer and provenance.',{chain:{id:chainId,step:0,previous:null}});
 const q=build(accrue(),origin.financialPost),a=accrualExpected(origin.financialPost,'A1','1','1060',interest),accrued=successful(q,a.state,[a.effect]);
 add(chainId+'-accrue',[requirement],q,accrued,`Independent principal/rate calculation gives interest${interest}; consume actual originated predecessor.`,{chain:{id:chainId,step:1,previous:chainId+'-origin'}});
 if(name==='zero')negative(chainId+'-repeat',requirement,accrue(),accrued.financialPost,fail('DUPLICATE'));
 if(name==='cap118'){
  const r=build(transfer('P1','Borrower','Lender','30')+repay('R1','P1','30'),accrued.financialPost),paid=repay30Expected(r,accrued.financialPost);
  add(chainId+'-repay',[requirement],r,paid,'30repayment discharges10accrued+20principal; incurred110/cap118 preserved.',{chain:{id:chainId,step:2,previous:chainId+'-accrue'}});
  const s=build(accrue('A2','2','1120'),paid.financialPost),b=accrualExpected(paid.financialPost,'A2','2','1120','8');
  add(chainId+'-second-accrue',[requirement],s,successful(s,b.state,[b.effect]),'Currentprincipal80*1/10=8, incurred110+8=118 at pre-existing cap118.',{chain:{id:chainId,step:3,previous:chainId+'-repay'}});
 }
}
for (const [id, config, interest] of [
  ['round-floor', { principal: '101', cap: '112' }, '10'],
  ['round-ceil', { principal: '101', cap: '112', rounding: 'ceil' }, '11'],
  ['zero-interest', { numerator: '0' }, '0'],
]) {
  const f = independentlyOriginated(config), p = build(accrue(), f), x = accrualExpected(f, 'A1', '1', '1060', interest);
  add(id, [id], p, successful(p, x.state, [x.effect]), `Independent principal*rate arithmetic: ${config.principal ?? '100'} * ${config.numerator ?? '1'} / 10 -> ${interest}; explicit rounding, full fields copied from independent predecessor.`);
}
{
  const f = independentlyOriginated({ cap: '120' }), p = build(accrue('A1','1','1120') + accrue('A2','2','1120'), f);
  const a = accrualExpected(f,'A1','1','1120','10'), b = accrualExpected(a.state,'A2','2','1120','10');
  add('late-explicit', ['late-explicit'], p, successful(p,b.state,[a.effect,b.effect]), 'Two explicit periods each accrue100/10=10; unchanged principal; cursor2 and next1180.');
  negative('late-skip', 'late-explicit', accrue('A2','2','1120'), f, fail('PERIOD_SEQUENCE'));
}
{
  const f = copy(stages[2].financialPost); f.obligations[0].nominalLiabilityCap = '118';
  const p = build(accrue('A2','2','1120'), f), x = accrualExpected(f,'A2','2','1120','8');
  add('cap-independent-alternate',['cap-independent-alternate'],p,successful(p,x.state,[x.effect]),
    'Independent alternate predecessor with cap118, principal80, incurred110: interest8 -> debt88/incurred118. This fixture is constructed, not a live-state cap mutation.');
}
negative('funding-missing','funding-missing',originate({mantissa:U128.toString()}),seed,fail('TRANSFER_NOT_IN_STEP'));
negative('funding-future','funding-missing',originate()+transfer(),seed,fail('TRANSFER_NOT_IN_STEP'));
negative('funding-prior-invocation','funding-missing',originate({id:'Loan2',origin:'O2',tid:'D1'}),stages[0].financialPost,fail('TRANSFER_NOT_IN_STEP'));
negative('funding-direction','funding-direction',transfer('T1','Lender','Other')+originate(),seed,fail('TRANSFER_MISMATCH',1));
for (const n of ['99','101']) { const f=copy(seed);f.balances[0].amount='101';f.allowances[0].remaining='101';negative('funding-amount-'+n,'funding-amount',transfer('T1','Lender','Borrower',n)+originate(),f,fail('TRANSFER_AMOUNT_MISMATCH',1)); }
negative('funding-reuse','funding-reuse',transfer()+originate()+originate({id:'Loan2',origin:'O2'}),seed,fail('TRANSFER_ALREADY_ALLOCATED',2));
negative('funding-partial-reuse','funding-reuse',transfer('T1','Borrower','Lender','100')+repay('R1','T1','30')+
 originate({id:'Loan2',origin:'O2'}).replace('debtor: "Borrower", creditor: "Lender"','debtor: "Lender", creditor: "Borrower"'),stages[1].financialPost,fail('TRANSFER_ALREADY_ALLOCATED',2));
negative('origin-duplicate','origin-duplicate',originate(),stages[0].financialPost,fail('DUPLICATE'));
negative('origin-duplicate-obligation','origin-duplicate',originate({origin:'O2'}),stages[0].financialPost,fail('DUPLICATE'));
negative('origin-duplicate-origin-id','origin-duplicate',originate({id:'Loan2'}),stages[0].financialPost,fail('DUPLICATE'));
negative('settled-accrue','settled-accrue',accrue(),stages[3].financialPost,fail('NOT_OUTSTANDING'));
negative('settled-repay-kernel','settled-repay-kernel',repay(),stages[3].financialPost,fail('NOT_OUTSTANDING'));
negative('product-overflow','product-overflow',accrue(),independentlyOriginated({principal:'2',cap:'10',numerator:U128.toString(),denominator:U128.toString()}),fail('OVERFLOW'));
negative('boundary-before-cap','boundary-before-cap',accrue('A1','1',U64.toString()),independentlyOriginated({cap:'100',start:(U64-1n).toString(),duration:'1'}),fail('OVERFLOW'));
negative('nominal-range','nominal-range',accrue(),independentlyOriginated({principal:M.toString(),cap:M.toString(),numerator:'1',denominator:M.toString()}),fail('NOMINAL_RANGE'));
negative('closed-admission-denominator','closed-admission',accrue(),independentlyOriginated({denominator:'0'}),fail('INVARIANT',null));
negative('closed-admission-rounding','closed-admission',accrue(),independentlyOriginated({rounding:'nearest'}),fail('SCHEMA',null));
negative('nominal-cap-input-range','nominal-range',accrue(),independentlyOriginated({cap:(M+1n).toString()}),fail('INVALID_AMOUNT',null));
negative('duplicate-before-period','period-id-priority',accrue('A1','3','0'),stages[1].financialPost,fail('DUPLICATE'));
for(const period of ['0','1','3'])negative('period-before-time-'+period,'period-order-priority',accrue('A2',period,'0'),stages[1].financialPost,fail('PERIOD_SEQUENCE'));
{
 const f=copy(seed);f.balances[2].unexpected='x';negative('closed-admission-extra','closed-admission',transfer(),f,fail('UNKNOWN_FIELD',null));
 const g=copy(seed);g.balances[2].party='Other\n';negative('closed-admission-newline','closed-admission',transfer(),g,fail('INVALID_IDENTIFIER',null));
 const h=copy(seed);h.balances[2].amount=7;negative('closed-admission-number','closed-admission',transfer(),h,fail('INVALID_AMOUNT',null));
}
negative('empty-before-suffix','empty-before-suffix','',seed,adapter('EMPTY_BATCH'),{suffix:'ensures false;'});
negative('kernel-before-ensure','kernel-before-ensure',transfer('T1','Lender','Borrower','101'),seed,fail('INSUFFICIENT_BALANCE'),{suffix:'ensures false;'});
negative('units','units',originate().replace('denomination: "Cash"','denomination: "Other"'),seed,adapter('NOMINAL_UNIT'));
{
 const f=independentlyOriginated();f.obligations[0].denomination='Other';
 negative('units-post-kernel','units',accrue(),f,adapter('NOMINAL_UNIT'),{suffix:'ensures false;'});
 const g=copy(seed);g.allowances[2].remaining=U128.toString();
 negative('unrelated-invalid-before-guard','unrelated-invalid',transfer(),g,fail('INVARIANT',null),{prefix:'requires false;'});
}

function exprExpected(packet, code, expression, path, workUsed) {
  const text=JSON.parse(packet.request).source, start=text.indexOf(expression);
  assert.notEqual(start,-1);
  return {status:'Rejected',code,span:{kind:'source',start:String(Buffer.byteLength(text.slice(0,start))),end:String(Buffer.byteLength(text.slice(0,start+expression.length)))},nodePath:path,workUsed:String(workUsed)};
}
{
 const getter='outstanding<Cash>("Loan1")';
 const p=build(transfer(),seed,{prefix:`let x = ${getter};`,coreEdit:c=>{c.statements[0].operands.value.constructor="ReadPostOutstanding";}});
 add('post-scope',['post-scope'],p,exprExpected(p,'TYPE_POST_SCOPE',getter,['0','0'],0),'Getter outside Ensure: static failure at first Let child, zero runtime work.');
 const dead=build(transfer(),seed,{prefix:`requires false and ${getter} == quantity<Units<Cash,1>,0>(100);`,coreEdit:c=>{
  const visit=x=>{if(!x||typeof x!=='object')return;if(x.constructor==='ReadOutstanding')x.constructor='ReadPostOutstanding';for(const v of Object.values(x))visit(v);};visit(c);
 }});
 add('post-scope-dead-branch',['post-scope'],dead,exprExpected(dead,'TYPE_POST_SCOPE',getter,['0','0','1','0'],0),'Static checking visits And right branch even though false left prevents runtime evaluation.');
 for(const [id,packet,path] of [['source-post-scope',p,['0','0']],['source-post-scope-dead',dead,['0','0','1','0']]]){
  const input=JSON.parse(packet.request).source.replace(getter,'post_'+getter),start=input.indexOf('post_'+getter);
  const expected={status:'Rejected',code:'TYPE_POST_SCOPE',span:{kind:'source',start:String(start),end:String(start+getter.length+5)},nodePath:path,workUsed:'0'};
  const observed=language.check(input);assert.deepEqual(observed,expected);
  controls.push({id,requirementIds:['post-scope'],kind:'source-only-static',source:input,expected,observed});
 }
 {
  const invalid=header.replace('transferAmount: Amount<Cash>;','transferAmount: UInt128;')+' action probe() {}\n}\n';
  const field='transferAmount: UInt128;',start=invalid.indexOf(field),observed=language.check(invalid);
  const expected={status:'Rejected',code:'OPERATION_BINDING',span:{kind:'source',start:String(start),end:String(start+field.length)},nodePath:[],workUsed:'0'};
  assert.deepEqual(observed,expected);
  controls.push({id:'source-protected-type',requirementIds:['units'],kind:'source-only-static',source:invalid,
   expected,observed,derivation:'ProtectedTransfer requires Amount<Cash>; Source frontend rejects altered UInt128 binding before Core generation; declaration span derived from source text.'});
 }
 const g='outstanding<Cash>("Loan1")', suffix=`ensures ${g} == quantity<Units<Cash,1>,0>(100);`;
 const q=build(transfer()+originate(),seed,{suffix});
 const qr=JSON.parse(q.request), prefixNodes=qr.core.statements.slice(0,-1).reduce((n,x)=>n+nodes(x),0);
 add('pre-versus-post-missing',['pre-versus-post'],q,exprExpected(q,'MISSING_OBLIGATION',g,['2','0','0'],prefixNodes+2+4),
   'PRE remains empty after Originate; suffix visits Ensure,Eq,ReadOutstanding,LitText (4) after2kernel actions.');
}
for(const [i,stage] of corpus.stages.entries()) {
 const W=[99,65,88,86][i],prefix=[36,10,35,33][i],N=[2,1,2,2][i],firstEnsure=[4,2,7,7][i];
 for(const remaining of [W,W-1,prefix+N-1,prefix+N]) {
  const p=copy(packetOf(stage.core.input)),r=JSON.parse(p.request),f=JSON.parse(p.financialPreState);
  r.workInitial=String(remaining);f.work={remaining:String(remaining),spent:'17',closureReserve:'16'};p.request=canonical(r);p.financialPreState=JSON.stringify(f);
  let expected;
  if(remaining===W){expected=copy(stage.expected);expected.financialPost.work={remaining:'0',spent:String(17+W),closureReserve:'16'};expected.workRemaining='0';}
  else if(remaining===prefix+N-1)expected=fail('INSUFFICIENT_WORK',null);
  else {
   const last=r.core.statements.length-1,statement=remaining===W-1?last:firstEnsure;
   const node=remaining===W-1?r.core.statements[last].operands.condition.operands.right.operands.value:r.core.statements[firstEnsure];
   expected={status:'Rejected',code:'WORK_EXHAUSTED',span:node.span,nodePath:remaining===W-1?[String(statement),'0','1','0']:[String(statement)],workUsed:String(remaining)};
  }
  add(`frozen-${stage.action}-work-${remaining}`,[remaining===W||remaining===W-1?'work-exact':remaining===prefix+N-1?'work-kernel':'work-suffix'],p,expected,
   `Frozen independent action cost${W}=prefix${prefix}+kernel${N}+suffix${W-prefix-N}; exhausted site is firstEnsure or finalEq right ConstructAmount child literal from Core structure, never evaluator output.`);
 }
}
{
 // One Emit Accrue:6 expression nodes,1kernel; Ensure true adds2.
 const f=independentlyOriginated(); const body=accrue();
 for(const remaining of [9,8,6,7]) {
  const s=copy(f);s.work={remaining:String(remaining),spent:'17',closureReserve:'16'};
  const p=build(body,s,{suffix:'ensures true;'});let expected;
  if(remaining===9){const a=accrualExpected(s,'A1','1','1060','10');expected=successful(p,a.state,[a.effect]);}
  else if(remaining===6)expected=fail('INSUFFICIENT_WORK',null);
  else if(remaining===7)expected=exprExpected(p,'WORK_EXHAUSTED','ensures true;',['1'],7);
  else expected=exprExpected(p,'WORK_EXHAUSTED','true',['1','0'],8);
  add('work-'+remaining,[remaining===9||remaining===8?'work-exact':remaining===6?'work-kernel':'work-suffix'],p,expected,
    'Hand tally: Emit+ConstructRecord+4literal=6; kernel1; Ensure+Bool2; preserve prior spent17 and reserve16.');
 }
}
function transferExpected(packet,state,id='T1',amount='1') {
 const f=copy(state),n=BigInt(amount);
 f.balances[0].amount=String(BigInt(f.balances[0].amount)-n);f.balances[1].amount=String(BigInt(f.balances[1].amount)+n);
 f.allowances[0].remaining=String(BigInt(f.allowances[0].remaining)-n);f.allowances[0].spent=String(BigInt(f.allowances[0].spent)+n);f.usedTransferIds.push(id);
 return successful(packet,f,[{kind:'Transfer',id,from:'Lender',to:'Borrower',asset:'Cash',amount}]);
}
{
 const f=copy(seed);f.usedTransferIds=Array.from({length:127},(_,i)=>'Old'+i);
 const p=build(transfer('T1','Lender','Borrower','1'),f);
 const expected=transferExpected(p,f);
 add('history-127-to128',['history-limits'],p,expected,'127 old transfer tombstones plus one valid transfer ->128; all original array ordering retained.');
 negative('history-129','history-limits',transfer('T2','Lender','Borrower','1'),expected.financialPost,fail('CAPACITY'));
 const g=copy(seed);g.balances[0].amount=U128.toString();g.allowances[0].remaining=U128.toString();
 const q=build(transfer('T1','Lender','Borrower','1'),g);
 add('amount-full-u128',['amount-domain'],q,transferExpected(q,g),'FullUInt128 input balances/allowance, one-unit debit; quantity domain is separate.');
 const h=independentlyOriginated({principal:M.toString(),cap:M.toString(),numerator:'0'}),r=build(accrue(),h),a=accrualExpected(h,'A1','1','1060','0');
 add('amount-signed-limit',['amount-domain'],r,successful(r,a.state,[a.effect]),'Maximum supported nominal Quantity accepts zero-rate accrual; fullUInt128 money remains distinct.');
}
{
 const f=independentlyOriginated({principal:'2',cap:'10',numerator:U128.toString(),denominator:U128.toString()});
 const other=copy(f.obligations[0]);Object.assign(other,{id:'OtherLoan',originationId:'OX',originationTransferId:'DX',principal:'0',accrued:'0',outstanding:'0',initialPrincipal:'1',liabilityIncurred:'1',nominalLiabilityCap:'1',status:'Settled',lastAccruedPeriod:'128',nextAccrualAt:'8740'});
 other.accrualTerms={numerator:'0',denominator:'1',rounding:'floor',firstPeriodStart:'1000',periodSeconds:'60'};
 f.obligations.push(other);f.usedOriginationIds.push('OX');f.usedTransferIds.push('DX');f.usedAccrualIds=Array.from({length:128},(_,i)=>'OldA'+i);
 negative('unrelated-history-capacity','unrelated-history-capacity',accrue(),f,fail('OVERFLOW'),{derivation:'Independent unrelated settled obligation has128periods and128matching accrual tombstones; target product2*UInt128Max overflows before full history capacity.'});
}
{
 const f=copy(seed);
 const template=copy(stages[3].financialPost.obligations[0]);
 // Fill with independent, valid settled obligations; their retained origin IDs
 // also populate both provenance arrays. Grow a byte-bound fixture, not results.
 for(let i=0;i<100;i++) {
  const o=copy(template);Object.assign(o,{id:'OldLoan'+i,originationId:'OldO'+i,originationTransferId:'OldT'+i,lastAccruedPeriod:'0',nextAccrualAt:'1060',liabilityIncurred:'100',nominalLiabilityCap:'100'});
  f.obligations.push(o);f.usedOriginationIds.push(o.originationId);f.usedTransferIds.push(o.originationTransferId);
  if(JSON.stringify(f).length>64000)break;
 }
 // Balance rows permit fine-grained padding without fabricated liability.
 while(JSON.stringify(f).length<64750 && f.balances.length<128)f.balances.push({party:'Padding'+f.balances.length+'x'.repeat(40),asset:'PaddingAsset',amount:'0'});
 for(const b of f.balances.slice(3)){const n=Math.min(64-b.party.length,64870-JSON.stringify(f).length);if(n>0)b.party+='x'.repeat(n);}
 const p=build(transfer()+originate({id:'NewLoan',origin:'NewO'+'x'.repeat(60)}),f);
 negative('result-bytes','result-bytes',transfer()+originate({id:'NewLoan',origin:'NewO'+'x'.repeat(60)}),f,fail('RESULT_BOUND',null),
   {derivation:`Independent valid padded predecessor (${JSON.stringify(f).length}ASCII bytes); new origin adds persistent debt/provenance beyond65536, after kernel input still fits.`});
 const g=copy(f);g.work.spent='67';
 for(const b of g.balances.slice(3)){const n=Math.min(b.party.length-8,JSON.stringify(g).length-64838);if(n>0)b.party=b.party.slice(0,-n);}
 assert.equal(JSON.stringify(g).length,64838);
 const boundaryPacket=build(transfer()+originate({id:'NewLoan',origin:'NewO'+'x'.repeat(60)}),g);
 const boundaryExpected=originExpected(boundaryPacket,g,{id:'NewLoan',origin:'NewO'+'x'.repeat(60)});
 assert.equal(JSON.stringify(boundaryExpected.financialPost).length,65536);
 add('result-bytes-exact-kernel',['result-bytes'],boundaryPacket,boundaryExpected,'Independent complete oracle has exactly65536ASCII bytes after kernel, spent99; this control establishes kernel result fits before suffix-width test.');
 negative('result-bytes-suffix-width','result-bytes',transfer()+originate({id:'NewLoan',origin:'NewO'+'x'.repeat(60)}),g,fail('RESULT_BOUND',null),
  {suffix:'ensures true;',derivation:'Independently padded64838-byte predecessor plus698bytes of persistent origin/state changes reaches65536 after prefix30+kernel2; prior spent67 becomes99. Two-node suffix makes spent101, adding one byte after kernel publication ->RESULT_BOUND.'});
}
{
 const mutations=['debt-split','allowance','work','cursor','effect','history'];
 const base=copy(stages[2]);
 for(const kind of mutations){const m=copy(base);
  if(kind==='debt-split'){m.financialPost.obligations[0].principal='79';m.financialPost.obligations[0].accrued='1';}
  if(kind==='allowance')m.financialPost.allowances[1].spent='29';
  if(kind==='work')m.financialPost.work.spent='268';
  if(kind==='cursor')m.financialPost.obligations[0].lastAccruedPeriod='0';
  if(kind==='effect')m.effects[1].payer='Other';
  if(kind==='history')m.financialPost.usedTransferIds.reverse();
  const result=compareLifecycleResult(m,base);assert.equal(result.ok,false);
  controls.push({id:'comparator-'+kind,requirementIds:['comparator-corruption'],kind:'offline-comparator',expected:{ok:false},observed:result});
 }
 const historical=JSON.parse(readFileSync(new URL('./expression-v1.json',import.meta.url),'utf8'));
 const list=Array.isArray(historical)?historical:historical.cases;
 controls.push({id:'metadata-5947',requirementIds:['metadata-5947'],kind:'expression-suite',fixture:'fixtures/expression-v1.json',
   cases:list.filter(x=>/metadata-depth-594[78]/.test(x.id)).map(x=>x.id),
   status:'requires-actual-expression-suite-result',expected:'5947metadata/Core65536 accepts;5948/Core65547 INPUT_BOUND. Never substitute lifecycle TYPE_ACTION_REQUIRED.'});
}

// The remaining boundaries are populated below; no absent requirement is called executed.
const required = ['round-floor','round-ceil','zero-interest','late-explicit','cap-after-partial','cap-independent-alternate','funding-missing','funding-direction','funding-amount','funding-reuse','origin-duplicate','period-id-priority','period-order-priority','period-time','settled-accrue','settled-source','settled-repay-kernel','product-overflow','boundary-before-cap','nominal-range','unrelated-history-capacity','closed-admission','unrelated-invalid','post-scope','pre-versus-post','units','empty-before-suffix','late-ensure','kernel-before-ensure','work-exact','work-kernel','work-suffix','work-mismatch','result-bytes','history-limits','amount-domain','metadata-5947','comparator-corruption'];
const covered = new Set([...cases,...controls].flatMap(c=>c.requirementIds));
const report={schemaVersion:'moriarty-lifecycle-k-fixtures/1',cases,controls,coverage:required.map(id=>({id,status:cases.some(c=>c.requirementIds.includes(id))?'concrete-core-fixtures':controls.some(c=>c.requirementIds.includes(id))?'external-control':'specified-only',caseIds:cases.filter(c=>c.requirementIds.includes(id)).map(c=>c.id),controlIds:controls.filter(c=>c.requirementIds.includes(id)).map(c=>c.id)})),
  derivationLimits:['Core observations are retained separately from independent expected results.','Generated fixtures do not establish native K execution.',
    'Coverage instantiates38historical requirement IDs with explicit variants; it is finite differential conformance, not exhaustive input-domain proof.',
    'Alternate floor/ceil/zero/cap118 chains have explicit origin packets; standalone boundary predecessors remain independently constructed valid states.',
    'post-scope Core packets mutate an elaborated PRE getter retaining source site; separate actual Source-only rejection controls cover invalid financial POST syntax.',
    'result-bytes includes both kernel publication and suffix-spent decimal-width growth.',
    'metadata execution is delegated to the113case expression suite; comparator mutations are offline observations.']};
writeFileSync(new URL('./lifecycle-v1.json',import.meta.url),JSON.stringify(report,null,2)+'\n');
const mismatches=cases.filter(c=>!c.expectedMatchesCore||c.expectedMatchesSource===false).map(c=>({id:c.id,expected:c.expected,observedStatus:c.observedCore.status,observedCode:c.observedCore.code,differences:c.differences,sourceDifferences:c.sourceDifferences}));
console.log(JSON.stringify({cases:cases.length,covered:covered.size,missing:required.filter(id=>!covered.has(id)),mismatches},null,2));
process.exitCode=mismatches.length?1:0;
