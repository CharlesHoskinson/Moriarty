import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

const api = await import('../src/successor/evaluate.ts').catch(e => {
  if (e.code !== 'ERR_MODULE_NOT_FOUND') throw e;
  return {};
});
const compiler = await import('../src/successor/elaborate.ts').catch(e => {
  if (e.code !== 'ERR_MODULE_NOT_FOUND') throw e;
  return {};
});
const cases = JSON.parse(readFileSync(new URL('../../../deliverables/source-core-repayment-2026-09-09/independent-financial-cases.json', import.meta.url))).cases;
const source = `profile "moriarty-successor-syntax/0";
agreement Loan {
 unit Cash; party Payer; party Lender; asset Cash: Asset<Cash>;
 action pay(cash: Amount<Cash>, nominal: Debt<Cash>) {
 emit Transfer {id: TransferId("T1"), from: Payer, to: Lender, settlementAsset: Cash, amount: cash};
 emit Repay {allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"), obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: nominal};
 }
}`;
function invocation(c = cases[0]) {
 return {schemaVersion:'moriarty-funded-source/0', action:'pay', arguments:{cash:{type:{kind:'Amount',name:'Cash'},value:c.input.actions[0].amount},nominal:{type:{kind:'Debt',name:'Cash'},value:c.input.actions[1].nominalAmount}}, state:structuredClone(c.input.state)};
}
function run(s = source, i = invocation()) {
 assert.equal(typeof api.prepareSuccessor, 'function', 'source preparation API exists');
 return api.prepareSuccessor(s, typeof i === 'string' ? i : JSON.stringify(i));
}
function rejects(s, i, code) {
 const r = run(s,i); assert.equal(r.status,'Rejected');
 assert.equal('post' in r,false); assert.equal('effects' in r,false);
 if (code) assert.equal(r.code,code);
 return r;
}
for (const c of cases) test(`independent full financial result: ${c.id}`, () => assert.deepEqual(run(source,invocation(c)),c.expected));
test('Core is deterministic serializable and cannot be executed as source', () => {
 assert.equal(typeof compiler.elaborateSuccessorSource,'function');
 const core = compiler.elaborateSuccessorSource(source);
 assert.deepEqual(JSON.parse(JSON.stringify(core)), compiler.elaborateSuccessorSource(source));
 rejects(core,invocation(),'SOURCE_TYPE'); rejects(JSON.stringify(core),invocation());
});
test('payment arguments change both debt and cash', () => {
 const i=invocation(); i.arguments.cash.value='20'; i.arguments.nominal.value='20';
 const r=run(source,i); assert.equal(r.post.balances[0].amount,'80'); assert.equal(r.post.obligations[0].principal,'80');
});
test('complete unrelated duties, terms, order and tombstones survive', () => {
 const i=invocation(); const extra={...structuredClone(i.state.obligations[0]),id:'Other'};
 i.state.obligations.unshift(extra); i.state.usedTransferIds.push('Earlier'); i.state.usedAllocationIds.push('Prior');
 const r=run(source,i); assert.deepEqual(r.post.obligations[0],extra);
 assert.deepEqual(r.post.usedTransferIds,['Earlier','T1']); assert.deepEqual(r.post.usedAllocationIds,['Prior','Alloc1']);
});
test('runtime nominal denomination is checked', () => { const i=invocation(); i.state.obligations[0].denomination='USD'; rejects(source,i,'NOMINAL_UNIT'); });
test('runtime settlement asset is checked', () => {
 const s=source.replace('unit Cash;', 'unit Cash; asset Other: Asset<Cash>;').replace('settlementAsset: Cash, amount','settlementAsset: Other, amount');
 rejects(s,invocation(),'SETTLEMENT_UNIT');
});
test('retained conversion is honored', () => {
 const i=invocation(); i.state.obligations[0].conversion.mantissa='2'; i.arguments.cash.value='60';
 const r=run(source,i); assert.equal(r.status,'Prepared'); assert.equal(r.post.balances[0].amount,'40');
 assert.equal(r.post.obligations[0].principal,'70'); assert.equal(r.effects[1].settlementAmount,'60');
 assert.equal(r.post.obligations[0].conversion.mantissa,'2');
});
for (const [name,change,code] of [
 ['zero',i=>i.arguments.nominal.value='0','ZERO_AMOUNT'],
 ['overpayment',i=>i.arguments.nominal.value='101','EXCEEDS_OUTSTANDING'],
 ['balance',i=>i.state.balances[0].amount='1','INSUFFICIENT_BALANCE'],
 ['allowance',i=>i.state.allowances[0].remaining='1','INSUFFICIENT_ALLOWANCE'],
 ['work',i=>i.state.work.remaining='1','INSUFFICIENT_WORK'],
 ['transfer reuse',i=>i.state.usedTransferIds.push('T1'),'DUPLICATE'],
 ['allocation reuse',i=>i.state.usedAllocationIds.push('Alloc1'),'DUPLICATE'],
 ['insufficient funding',i=>i.arguments.cash.value='20','INSUFFICIENT_UNALLOCATED'],
]) test(`atomic financial rejection: ${name}`,()=>{const i=invocation();change(i); rejects(source,i,code);});
test('Repay without Transfer cannot discharge debt', () => rejects(source.replace(/emit Transfer[^;]+;/,''),invocation(),'TRANSFER_NOT_IN_STEP'));
for (const [name,s] of [
 ['state',source.replace('action pay','state p: Debt<Cash> = debt(100, Cash); action pay')],
 ['next',source.replace('emit Transfer','next.principal = nominal; emit Transfer')],
 ['const',source.replace('action pay','const n: UInt = 1; action pay')],
 ['requires',source.replace('emit Transfer','requires true; emit Transfer')],
 ['let',source.replace('emit Transfer','let n = 1; emit Transfer')],
 ['ensures',source.replace('\n }','\n ensures true; }')],
 ['arithmetic',source.replace('amount: cash','amount: cash + cash')],
 ['projection',source.replace('amount: cash','amount: pre.cash')],
 ['unknown reference',source.replace('amount: cash','amount: absent')],
 ['duplicate party',source.replace('party Payer;','party Payer; party Payer;')],
 ['duplicate unit',source.replace('unit Cash;','unit Cash; unit Cash;')],
 ['duplicate param',source.replace('nominal: Debt<Cash>','cash: Debt<Cash>')],
 ['unsupported type',source.replace('Debt<Cash>','Debt<Unknown>')],
 ['wrong field type',source.replace('payer: Payer','payer: nominal')],
 ['unknown field',source.replace('amount: cash','extra: cash, amount: cash')],
 ['duplicate field',source.replace('amount: cash','amount: cash, amount: cash')],
 ['parameter type confusion',source.replace('nominalAmount: nominal','nominalAmount: cash')],
]) test(`closed source subset rejects ${name}`,()=>rejects(s,invocation()));
for (const [name,change] of [
 ['unknown argument',i=>i.arguments.extra=i.arguments.cash],
 ['missing argument',i=>delete i.arguments.nominal],
 ['wrong unit',i=>i.arguments.nominal.type.name='USD'],
 ['wrong type',i=>i.arguments.nominal.type.kind='Amount'],
 ['coercion',i=>i.arguments.nominal.value=30],
 ['overflow',i=>i.arguments.nominal.value='340282366920938463463374607431768211456'],
 ['negative',i=>i.arguments.nominal.value='-1'],
 ['unknown invocation field',i=>i.extra=true],
 ['unknown typed field',i=>i.arguments.nominal.extra=true],
 ['unknown state field',i=>i.state.extra=true],
 ['unknown action',i=>i.action='missing'],
 ['prototype key',i=>Object.defineProperty(i.arguments,'__proto__',{value:{},enumerable:true})],
]) test(`closed invocation rejects ${name}`,()=>{const i=invocation();change(i);rejects(source,i);});
test('hostile objects are rejected without executing accessors',()=>{
 let touched=false; const hostile={get schemaVersion(){touched=true;throw Error('getter');},toString(){touched=true;throw Error('coercion');}};
 assert.equal(api.prepareSuccessor(source,hostile).code,'INVOCATION_TYPE');
 assert.equal(api.prepareSuccessor(hostile,JSON.stringify(invocation())).code,'SOURCE_TYPE'); assert.equal(touched,false);
});
test('CLI reads actual source and invocation and rejects with nonzero status',()=>{
 const base=new URL('../',import.meta.url); const cli=new URL('src/successor/simulate-cli.ts',base);
 const args=[cli.pathname,'simulate',new URL('spec/successor/examples/funded-partial-payment.mori',base).pathname,new URL('spec/successor/examples/funded-partial-payment.invocation.json',base).pathname];
 const good=spawnSync(process.execPath,args,{encoding:'utf8'}); assert.equal(good.status,0,good.stderr); assert.equal(JSON.parse(good.stdout).post.obligations[0].principal,'70');
 const bad=spawnSync(process.execPath,[cli.pathname,'simulate','/missing/source','/missing/invocation'],{encoding:'utf8'}); assert.notEqual(bad.status,0); assert.equal(JSON.parse(bad.stdout).status,'Rejected');
});
test('explicit nominal and settlement literals remain distinct',()=>{
 const s=source.replace('amount: cash','amount: amount(40, Cash)').replace('nominalAmount: nominal','nominalAmount: debt(30, Cash)');
 const r=run(s); assert.equal(r.post.balances[0].amount,'60'); assert.equal(r.post.obligations[0].principal,'70');
});
test('UInt, Bool, parties, assets and effect IDs have closed argument types',()=>{
 const s=source.replace('nominal: Debt<Cash>)','nominal: Debt<Cash>, count: UInt, flag: Bool, who: Party, token: Asset, tid: TransferId, aid: AllocationId, oid: ObligationId)')
 .replaceAll('TransferId("T1")','tid').replace('AllocationId("Alloc1")','aid').replace('ObligationId("Due100")','oid')
 .replace('from: Payer','from: who').replace('payer: Payer','payer: who').replace('settlementAsset: Cash','settlementAsset: token');
 const i=invocation(); for (const [name,kind,value] of [['count','UInt','1'],['flag','Bool',true],['who','Party','Payer'],['token','Asset','Cash'],['tid','TransferId','T1'],['aid','AllocationId','Alloc1'],['oid','ObligationId','Due100']]) i.arguments[name]={type:{kind},value};
 assert.deepEqual(run(s,i),cases[0].expected);
 i.arguments.flag.value='true'; rejects(s,i,'ARGUMENT_VALUE');
 i.arguments.flag.value=true; i.arguments.who.value='Unknown'; rejects(s,i,'UNKNOWN_PARTY');
});
test('all actions are statically checked and selected action is respected',()=>{
 const extra='action smaller(cash: Amount<Cash>, nominal: Debt<Cash>) { emit Transfer {id: TransferId("T1"), from: Payer, to: Lender, settlementAsset: Cash, amount: amount(20, Cash)}; emit Repay {allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"), obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: debt(20, Cash)}; }';
 const s=source.replace(/}\s*$/,extra+'}'); const i=invocation(); i.action='smaller';
 assert.equal(run(s,i).post.obligations[0].principal,'80');
 rejects(s.replace('amount(20, Cash)','arbitrary(20)'),invocation(),'UNSUPPORTED_EXPRESSION');
});
test('bounded source and invocation fail before parsing oversized content',()=>{
 rejects(' '.repeat(65537),invocation(),'SOURCE_BOUND');
 rejects(source,' '.repeat(65537),'INVOCATION_BOUND');
 rejects(source,'{','INVOCATION_JSON');
});
test('intermediate conversion overflow is retained even if division could shrink it',()=>{
 const i=invocation(); i.state.obligations[0].conversion={mantissa:'340282366920938463463374607431768211455',scale:'18',rounding:'floor'};
 rejects(source,i,'OVERFLOW');
});
