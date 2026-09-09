// Reviewer-authored probes; candidate implementation is never edited.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { prepareSuccessor } from '../../experiments/moriarty-language/src/successor/evaluate.ts';
import { elaborateSuccessorSource } from '../../experiments/moriarty-language/src/successor/elaborate.ts';
const source = readFileSync(new URL('../../experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori', import.meta.url), 'utf8');
const input = JSON.parse(readFileSync(new URL('../../experiments/moriarty-language/spec/successor/examples/funded-partial-payment.invocation.json', import.meta.url), 'utf8'));
const cases = JSON.parse(readFileSync(new URL('./independent-financial-cases.json', import.meta.url))).cases;
const base = () => structuredClone(input);
const run = (s=source, v=base()) => prepareSuccessor(s, typeof v==='string' ? v : JSON.stringify(v));
let count=0;
const check=(label,fn)=>{ fn(); count++; console.log('PASS '+label); };
function reject(s,v,code,index=null) { assert.deepEqual(run(s,v),{status:'Rejected',code,actionIndex:index}); }
const transfer='emit Transfer {id: TransferId("T1"), from: Payer, to: Lender, settlementAsset: Cash, amount: cash};';
const repay='emit Repay {allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"), obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: nominal};';
const program=(body,params='cash: Amount<Cash>, nominal: Debt<Cash>')=>`profile "moriarty-successor-syntax/0"; agreement Audit {unit Cash; party Payer; party Lender; asset Cash: Asset<Cash>; action pay(${params}) {${body}}}`;
check('baseline full result',()=>assert.deepEqual(run(),cases[0].expected));
check('field order produces same Core',()=>assert.deepEqual(elaborateSuccessorSource(program(transfer+repay)),elaborateSuccessorSource(program(transfer.replace('id: TransferId("T1"), from: Payer','from: Payer, id: TransferId("T1")')+repay))));
check('mutating inspected Core cannot change recompilation',()=>{const core=elaborateSuccessorSource(source);core.actions[0].emissions.length=0;assert.deepEqual(run(),cases[0].expected);reject(core,base(),'SOURCE_TYPE');reject(source,{...base(),core},'INVOCATION_SCHEMA');assert.equal(run(JSON.stringify(core)).status,'Rejected');});
check('malformed serialized Core is not source',()=>assert.equal(run('{"schemaVersion":"moriarty-funded-source/0","actions":[]}').status,'Rejected'));
check('objects and proxies trigger no source/invocation getters',()=>{let n=0;const p=new Proxy({}, {get(){n++;throw Error('trap');},ownKeys(){n++;throw Error('trap');}});assert.equal(prepareSuccessor(p,JSON.stringify(base())).code,'SOURCE_TYPE');assert.equal(prepareSuccessor(source,p).code,'INVOCATION_TYPE');assert.equal(n,0);});
check('source selection changes Transfer and debt independently',()=>{const s=program(transfer+repay).replace(/}$/,`action alternate(cash: Amount<Cash>, nominal: Debt<Cash>) {${transfer.replace('amount: cash','amount: amount(40, Cash)')}${repay.replace('nominalAmount: nominal','nominalAmount: debt(20, Cash)')}}}`);const v=base();v.action='alternate';const r=run(s,v);assert.equal(r.status,'Prepared');assert.equal(r.post.balances[0].amount,'60');assert.equal(r.post.obligations[0].principal,'80');});
check('unselected unsupported action rejects',()=>reject(program(transfer+repay).replace(/}$/,'action bad() { requires true; }}'),base(),'UNSUPPORTED_STATEMENT'));
check('reversed emissions reject before any funding',()=>reject(program(repay+transfer),base(),'TRANSFER_NOT_IN_STEP',0));
check('late third emission failure exposes no partial result',()=>reject(program(transfer+repay+repay.replace('Alloc1','Alloc2')),base(),'INSUFFICIENT_UNALLOCATED',2));
check('multiple allocations consume shared Transfer once',()=>{const v=base();v.arguments.cash.value='60';const r=run(program(transfer+repay+repay.replace('Alloc1','Alloc2')),v);assert.equal(r.status,'Prepared');assert.equal(r.post.obligations[0].principal,'40');assert.equal(r.post.balances[0].amount,'40');assert.equal(r.post.allowances[0].spent,'60');assert.deepEqual(r.post.usedTransferIds,['T1']);assert.deepEqual(r.post.usedAllocationIds,['Alloc1','Alloc2']);assert.equal(r.post.work.spent,'3');assert.equal(r.effects.length,3);});
check('every Repay nominal witness is bound after tentative success',()=>{const v=base();v.arguments.cash.value='60';const other={...structuredClone(v.state.obligations[0]),id:'Other',denomination:'USD'};v.state.obligations.push(other);reject(program(transfer+repay+repay.replace('Alloc1','Alloc2').replace('Due100','Other')),v,'NOMINAL_UNIT',2);});
check('distinct nominal unit and asset retain conversion',()=>{const v=base();v.state.obligations[0].denomination='USD';v.arguments.nominal.type.name='USD';v.arguments.cash.value='60';v.state.obligations[0].conversion.mantissa='2';const s=source.replace('unit Cash;','unit Cash; unit USD;').replace('Debt<Cash>','Debt<USD>');const r=run(s,v);assert.equal(r.status,'Prepared');assert.equal(r.post.obligations[0].principal,'70');assert.equal(r.effects[1].settlementAmount,'60');assert.equal(r.effects[1].denomination,'USD');assert.deepEqual(r.post.obligations[0].conversion,v.state.obligations[0].conversion);});
check('dynamic Asset binding rejects Amount with same underlying unit',()=>{const s=source.replace('unit Cash;','unit Cash; asset Other: Asset<Cash>;').replace('nominal: Debt<Cash>)','nominal: Debt<Cash>, token: Asset)').replace('settlementAsset: Cash','settlementAsset: token');const v=base();v.arguments.token={type:{kind:'Asset'},value:'Other'};reject(s,v,'SETTLEMENT_UNIT',0);});
check('kernel error precedes nominal mismatch',()=>{const v=base();v.state.obligations[0].denomination='USD';v.arguments.nominal.value='31';reject(source,v,'INSUFFICIENT_UNALLOCATED',1);});
check('settlement mismatch precedes malformed kernel state',()=>{const v=base();v.state={};reject(source.replace('unit Cash;','unit Cash; asset Other: Asset<Cash>;').replace('settlementAsset: Cash','settlementAsset: Other'),v,'SETTLEMENT_UNIT',0);});
check('unrelated fields arrays order and settled duty retained',()=>{const v=base();const obligation={...structuredClone(v.state.obligations[0]),id:'Retained',debtor:'Third',denomination:'USD',principal:'0',accrued:'0',outstanding:'0',status:'Settled',conversion:{mantissa:'7',scale:'2',rounding:'ceil'}};v.state.obligations.unshift(obligation);v.state.balances.unshift({party:'Third',asset:'Else',amount:'47'});v.state.allowances.unshift({party:'Third',asset:'Else',remaining:'49',spent:'3'});v.state.usedTransferIds=['Older'];v.state.usedAllocationIds=['Prior'];const r=run(source,v);assert.equal(r.status,'Prepared');assert.deepEqual(r.post.obligations[0],obligation);assert.deepEqual(r.post.balances[0],v.state.balances[0]);assert.deepEqual(r.post.allowances[0],v.state.allowances[0]);assert.deepEqual(r.post.usedTransferIds,['Older','T1']);assert.deepEqual(r.post.usedAllocationIds,['Prior','Alloc1']);});
check('malformed unrelated duty rejects whole result',()=>{const v=base();v.state.obligations.push({...v.state.obligations[0],id:'Other',outstanding:'99'});reject(source,v,'INVARIANT');});
check('post-state replay cannot discharge twice',()=>{const v=base();v.state=run().post;reject(source,v,'DUPLICATE',0);});
check('caller JSON remains reusable after late failure',()=>{const v=base();v.arguments.nominal.value='31';const json=JSON.stringify(v);reject(source,json,'INSUFFICIENT_UNALLOCATED',1);assert.equal(JSON.stringify(v),json);assert.deepEqual(run(),cases[0].expected);});
for(const [rounding,cash,settlement,code] of [['none','30',null,'INEXACT_CONVERSION'],['floor','30','15',null],['ceil','30','16',null]])check('conversion '+rounding,()=>{const v=base();v.arguments.cash.value=cash;v.arguments.nominal.value='31';v.state.obligations[0].conversion={mantissa:'5',scale:'1',rounding};if(code)reject(source,v,code,1);else{const r=run(source,v);assert.equal(r.post.obligations[0].principal,'69');assert.equal(r.post.balances[0].amount,'70');assert.equal(r.effects[1].settlementAmount,settlement);}});
check('conversion dust cannot discharge nominal debt',()=>{const v=base();v.arguments.nominal.value='1';v.state.obligations[0].conversion={mantissa:'1',scale:'1',rounding:'floor'};reject(source,v,'DUST',1);});
check('ProRata remains kernel allocation',()=>{const v=base();Object.assign(v.state.obligations[0],{principal:'90',accrued:'10',allocationRule:'ProRata'});const r=run(source,v);assert.equal(r.post.obligations[0].principal,'63');assert.equal(r.post.obligations[0].accrued,'7');});
for(const [label,s,code] of [
 ['empty action',program(''),'SCHEMA'],
 ['missing field',program(transfer.replace('amount: cash','')+repay).replace(', };','};'),'MISSING_FIELD'],
 ['effect type parameter',program(transfer.replace('Transfer {','Transfer<Cash> {')+repay),'UNSUPPORTED_EFFECT'],
 ['unit as term',program(transfer.replace('amount: cash','amount: Missing')+repay),'UNKNOWN_NAME'],
 ['term shadow',program(transfer+repay,'Payer: Amount<Cash>, nominal: Debt<Cash>'),'DUPLICATE_NAME'],
 ['builtin shadow',program(transfer+repay).replace('party Payer;','party UInt;'),'RESERVED_NAME'],
 ['unknown asset unit',source.replace('Asset<Cash>','Asset<Missing>'),'UNSUPPORTED_TYPE'],
 ['bare string',program(transfer.replace('amount: cash','amount: "30"')+repay),'UNSUPPORTED_EXPRESSION'],
 ['unary',program(transfer.replace('amount: cash','amount: not true')+repay),'UNSUPPORTED_EXPRESSION'],
 ['comparison',program(transfer.replace('amount: cash','amount: 1 == 1')+repay),'UNSUPPORTED_EXPRESSION'],
 ['numeric constructor coercion',program(transfer.replace('amount: cash','amount: amount(cash, Cash)')+repay),'UINT128'],
 ['bare UInt cannot be Amount',program(transfer.replace('amount: cash','amount: 30')+repay),'FIELD_TYPE'],
 ['boolean cannot be Amount',program(transfer.replace('amount: cash','amount: true')+repay),'FIELD_TYPE'],
 ['escaped ID newline',source.replace('TransferId("T1")','TransferId("T1\\n")'),'INVALID_IDENTIFIER'],
 ['naked principal subtraction',program('next.principal = nominal - nominal;'),'UNSUPPORTED_STATEMENT'],
 ['trailing input',source+'unit Hidden;','TRAILING_INPUT'],
])check(label,()=>reject(s,base(),code));
for(const [label,mutate,code,index] of [
 ['number amount',v=>v.arguments.cash.value=30,'UINT128',null],
 ['amount newline',v=>v.arguments.cash.value='30\n','UINT128',null],
 ['type extra key',v=>v.arguments.cash.type.extra='ignored','INVOCATION_SCHEMA',null],
 ['state extra key',v=>v.state.unrelated=true,'UNKNOWN_FIELD',null],
 ['state missing key',v=>delete v.state.allowances,'SCHEMA',null],
 ['invalid nested conversion',v=>v.state.obligations[0].conversion.extra=1,'UNKNOWN_FIELD',null],
 ['collection over capacity',v=>v.state.usedAllocationIds=Array.from({length:129},(_,i)=>'A'+i),'CAPACITY',null],
 ['allocation tombstone capacity after Transfer',v=>v.state.usedAllocationIds=Array.from({length:128},(_,i)=>'A'+i),'CAPACITY',1],
 ['insufficient work cannot spend reserve',v=>{v.state.work.remaining='1';v.state.work.closureReserve='1000';},'INSUFFICIENT_WORK',null],
 ['prototype JSON extra key',v=>Object.defineProperty(v,'__proto__',{value:{},enumerable:true}),'INVOCATION_SCHEMA',null],
])check(label,()=>{const v=base();mutate(v);reject(source,v,code,index);});
check('64 parameters admitted and 65 rejected',()=>{const extra=Array.from({length:62},(_,i)=>'arg'+i+': UInt');const s=program(transfer+repay,'cash: Amount<Cash>, nominal: Debt<Cash>,'+extra.join(','));const v=base();extra.forEach((_,i)=>v.arguments['arg'+i]={type:{kind:'UInt'},value:'0'});assert.equal(run(s,v).status,'Prepared');reject(s.replace('arg61: UInt','arg61: UInt, arg62: UInt'),v,'ARITY_BOUND');});
check('128 emissions admitted and 129 rejected',()=>{const body=Array.from({length:128},(_,i)=>transfer.replace('T1','T'+i).replace('amount: cash','amount: amount(1, Cash)')).join('');const v=base();v.state.balances[0].amount='200';v.state.allowances[0].remaining='200';v.state.work.remaining='200';const r=run(program(body),v);assert.equal(r.status,'Prepared');assert.equal(r.effects.length,128);assert.equal(r.post.balances[0].amount,'72');reject(program(body+transfer),v,'EMISSION_BOUND');});
check('exact source byte bound admitted',()=>assert.equal(run(source+' '.repeat(65536-Buffer.byteLength(source))).status,'Prepared'));
check('source multibyte bound rejects before parsing',()=>reject(source+'/*'+'é'.repeat(33000)+'*/',base(),'SOURCE_BOUND'));
check('exact invocation byte bound admitted',()=>{const j=JSON.stringify(base());assert.equal(run(source,j+' '.repeat(65536-Buffer.byteLength(j))).status,'Prepared');reject(source,j+' '.repeat(65537-Buffer.byteLength(j)),'INVOCATION_BOUND');});
check('invocation multibyte bound rejects before JSON',()=>reject(source,'é'.repeat(33000),'INVOCATION_BOUND'));
const dir=mkdtempSync(join(tmpdir(),'moriarty-gpt6-audit-'));
try {
 const cli=new URL('../../experiments/moriarty-language/src/successor/simulate-cli.ts',import.meta.url).pathname;
 const src=join(dir,'case.mori'), inv=join(dir,'case.json');writeFileSync(src,source);writeFileSync(inv,JSON.stringify(base()));
 const invoke=(args)=>{const r=spawnSync(process.execPath,[cli,...args],{encoding:'utf8',timeout:5000});assert.equal(r.error,undefined);assert.equal(r.stderr,'');return {exit:r.status,result:JSON.parse(r.stdout)};};
 check('actual CLI complete fixture matches oracle',()=>assert.deepEqual(invoke(['simulate',src,inv]),{exit:0,result:cases[0].expected}));
 check('actual CLI edited source changes result',()=>{writeFileSync(src,source.replace('to: Lender','to: Payer'));assert.deepEqual(invoke(['simulate',src,inv]),{exit:1,result:{status:'Rejected',code:'SELF_TRANSFER',actionIndex:0}});writeFileSync(src,source);});
 for(const [label,args,code] of [['usage',[],'CLI_USAGE'],['directory',['simulate',dir,inv],'CLI_IO'],['device',['simulate','/dev/null',inv],'CLI_IO'],['missing',['simulate',join(dir,'missing'),inv],'CLI_IO']])check('actual CLI '+label,()=>assert.deepEqual(invoke(args),{exit:1,result:{status:'Rejected',code,actionIndex:null}}));
 check('actual CLI oversized file',()=>{writeFileSync(src,' '.repeat(65537));assert.deepEqual(invoke(['simulate',src,inv]),{exit:1,result:{status:'Rejected',code:'CLI_BOUND',actionIndex:null}});});
 check('actual CLI invalid UTF8',()=>{writeFileSync(src,Buffer.from([0xc0,0xaf]));assert.deepEqual(invoke(['simulate',src,inv]),{exit:1,result:{status:'Rejected',code:'CLI_UTF8',actionIndex:null}});});
} finally {rmSync(dir,{recursive:true,force:true});}
console.log(`${count} fresh reviewer-authored adversarial probes passed`);
