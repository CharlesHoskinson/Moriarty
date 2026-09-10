import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { formatFinancialExpressionSource, parseFinancialExpressionSource } from '../src/successor/financial-expression-source-frontend.ts';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import { createFinancialExpressionContractV1 } from '../src/successor/financial-expression-v1.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = 'moriarty-financial-expression-source/1';
const schema = (patch = {}) => ({ units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {}, variantTypes: {}, fields: {}, args: {}, observations: {}, operations: {}, ...patch });
const source = (body, params = '') => `profile "${P}"; agreement Demo { action step(${params}) { ${body} } }`;
const input = (workInitial = '1000', Pre = {}, Args = {}, Obs = {}) => canonical({ Pre, Args, Obs, workInitial });
const api = patch => createFinancialExpressionSourceV1(canonical(schema(patch)));
const strip = value => Array.isArray(value) ? value.map(strip) : value && typeof value === 'object'
  ? Object.fromEntries(Object.entries(value).filter(([key]) => key !== 'span').map(([key, child]) => [key, strip(child)])) : value;
const nominal = { assets: ['A','B'], vaults: ['Vault'], parties: ['Alice'],
  variantTypes: { Quote: { Cash: ['Amount','A'], Shares: ['Shares','Vault','Alice'] } } };
function assigned(expr, type, expected, patch = {}) {
  const language = api({ ...patch, fields: { out: { type, writeClass: 'ordinary' } } });
  const s = source(`next.out = ${expr};`);
  const result = language.evaluate(s, input('1000', { out: expected }));
  assert.equal(result.status, 'ExpressionPrepared', JSON.stringify(result));
  assert.deepEqual(result.post.out, expected);
  return language.elaborate(s);
}
function rejected(expr, code, patch = {}, work = '1000') {
  const result = api(patch).evaluate(source(`let value = ${expr};`), input(work));
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, code, JSON.stringify(result));
  return result;
}

test('financial source literals and dynamic constructors keep distinct nodes and charges', () => {
  for (const [expr, constructor, nodes] of [['amount(5,A)','LitAmount',2], ['amount<A>(5)','ConstructAmount',3],
    ['shares(5,Vault,Alice)','LitShares',2], ['shares<Vault,Alice>(5)','ConstructShares',3],
    ['u256(5)','LitUInt',2], ['to_uint<256>(5)','ConvertUInt',3], ['to_uint<128>(5)','ConvertUInt',3]]) {
    const s = source(`let x=${expr};`), language = api(nominal), e = language.elaborate(s);
    assert.equal(e.judgmentResult, 'SourceElaborated', JSON.stringify(e));
    assert.equal(e.core.statements[0].operands.value.constructor, constructor);
    assert.equal(e.staticWorkBound, String(nodes));
    assert.equal(language.evaluate(s, input(String(nodes))).workRemaining, '0');
  }
  for (const expr of ['amount(1+2,A)','shares(1+2,Vault,Alice)','u256(1+2)']) rejected(expr,'SOURCE_LITERAL_SHAPE',nominal);
  for (const expr of ['amount<A>(u64(1))','shares<Vault,Alice>(u256(1))','to_uint<256>(amount(1,A))','to_uint<128>(-1)']) rejected(expr,'TYPE_MISMATCH',nominal);
  rejected('to_uint<17>(1)','TYPE_LITERAL');
});

test('financial source UInt256 boundaries and explicit narrowing retain exact errors', () => {
  const max = (2n**256n-1n).toString();
  assigned(`u256(${max})`, ['UInt256'], max);
  assigned('to_uint<256>(quanta(amount(7,A)))', ['UInt256'], '7', nominal);
  rejected(`u256(${2n**256n})`,'TYPE_LITERAL');
  rejected(`u256(${max}) + u256(1)`,'ARITH_RANGE');
  rejected(`to_uint<128>(u256(${2n**128n}))`,'ARITH_RANGE');
  rejected('u256(1) + 1','TYPE_MISMATCH');
  assigned('to_uint<64>(to_uint<256>(u64(7)))', ['UInt64'], '7');
});

test('financial dynamic amount/share endpoints and child-entry work remain explicit',()=>{
  for(const value of ['0',String(2n**128n-1n)]){
    assigned(`amount<A>(${value})`,['Amount','A'],value,nominal);
    assigned(`shares<Vault,Alice>(${value})`,['Shares','Vault','Alice'],value,nominal);
  }
  const result=rejected('amount<A>(7)','WORK_EXHAUSTED',nominal,'2');
  assert.deepEqual(result.nodePath,['0','0','0']);assert.equal(result.workUsed,'2');
  rejected(`shares<Vault,Alice>(${2n**128n-1n}+1)`,'ARITH_RANGE',nominal);
  rejected('amount<Unknown>(7)','TYPE_NAME',nominal);
  rejected('shares<Vault,Unknown>(7)','TYPE_NAME',nominal);
});

test('financial direct Emit preserves a conditional record operand and selected work',()=>{
  const language=api({recordTypes:{Row:{n:['UInt128']}},operations:{Notice:'Row'},args:{a:['Record','Row'],b:['Record','Row']}});
  const s=source('emit Notice true ? a : b;','a:Record<Row>,b:Record<Row>');
  const e=language.elaborate(s);assert.equal(e.staticWorkBound,'5');
  assert.equal(e.core.statements[0].operands.fields.constructor,'Select');
  assert.deepEqual(language.evaluate(s,input('4',{}, {a:{n:'1'},b:{n:'2'}})),{status:'ExpressionPrepared',post:{},descriptors:[{operation:'Notice',fields:{n:'1'}}],workRemaining:'0'});
});

test('financial source lazy Select checks both arms and charges only selected nodes', () => {
  const language = api();
  const yes = source('let x=true ? 7 : floor_div(1,0);');
  assert.equal(language.check(yes).staticWorkBound, '7');
  assert.equal(language.evaluate(yes,input('4')).workRemaining, '0');
  const no = source('let x=false ? 7 : floor_div(1,0);');
  const result = language.evaluate(no,input('10'));
  assert.equal(result.code,'ARITH_DENOMINATOR');
  assert.deepEqual(result.nodePath,['0','0','2']);
  assert.equal(result.workUsed,'6');
  const exhausted = language.evaluate(yes,input('3'));
  assert.equal(exhausted.code,'WORK_EXHAUSTED');
  assert.deepEqual(exhausted.nodePath,['0','0','1']);
  rejected('true ? 7 : u64(7)','TYPE_MISMATCH');
  rejected('true ? amount(1,A) : amount(1,B)','TYPE_MISMATCH',nominal);
  rejected('1 ? 7 : 8','TYPE_MISMATCH');
  rejected('(floor_div(1,0) == 1) ? 7 : 8','ARITH_DENOMINATOR');
});

test('financial source variants/options project exact payloads and reject wrong cases', () => {
  assigned('variant<Quote,Cash>(amount<A>(7))',['Variant','Quote'],{tag:'Cash',value:'7'},nominal);
  assigned('quanta(project_variant<Cash>(variant<Quote,Cash>(amount<A>(7))))',['UInt128'],'7',nominal);
  assigned('some_value(some<UInt256>(u256(7)))',['UInt256'],'7');
  rejected('some_value(none<UInt128>())','OPTION_NONE');
  rejected('project_variant<Cash>(variant<Quote,Shares>(shares<Vault,Alice>(7)))','VARIANT_CASE',nominal);
  rejected('variant<Quote,Unknown>(7)','TYPE_NAME',nominal);
  rejected('variant<Missing,Cash>(7)','TYPE_NAME',nominal);
  rejected('variant<Quote,Cash>(7)','TYPE_MISMATCH',nominal);
  rejected('project_variant<Cash>(7)','TYPE_MISMATCH',nominal);
  assert.equal(api().evaluate(source('requires none<UInt128>() != none<UInt128>() and some_value(none<UInt128>()) > 0;'),input()).code,'GUARD_FAILED');
});

test('financial source indexed arithmetic uses exact price/rate and literal scale rules', () => {
  assigned('floor_div(amount(10000,A)*u128(997)*amount(2000000,B), amount(1000000,A)*u128(1000)+amount(10000,A)*u128(997))',['Amount','B'],'19743',nominal);
  assigned('ceil_div(amount(10000,A)*price(19743,B,A,4),u128(10000))',['Amount','B'],'19743',nominal);
  assigned('floor_div(amount(5,A)*rate(-5,1),10)',['SignedAmount','A'],'-3',nominal);
  assigned('ceil_div(amount(5,A)*rate(-5,1),10)',['SignedAmount','A'],'-2',nominal);
  rejected('floor_div(amount(5,A)*rate(-5,1),to_uint<128>(10))','TYPE_SCALE_DIVISOR',nominal);
  rejected('amount(5,A)*price(5,A,B,1)','TYPE_MISMATCH',nominal);
  assigned('amount(5,B)*amount(3,A)',['AmountProduct','A','B'],'15',nominal);
});

test('financial source formatter preserves conditional AST association and exact Core', () => {
  for(const expr of ['(true ? false : true) ? 1 : 2','true ? false ? 1 : 2 : 3',
    'true ? 1 : false ? 2 : 3','false or true ? 1 : 2',
    '(true ? record<Row>{n:1} : record<Row>{n:2}).n']) {
    const s=source(`let value=${expr};`), formatted=formatFinancialExpressionSource(s);
    assert.equal(formatFinancialExpressionSource(formatted),formatted);
    assert.deepEqual(strip(parseFinancialExpressionSource(s)),strip(parseFinancialExpressionSource(formatted)));
    const language=api({recordTypes:{Row:{n:['UInt128']}}});
    assert.deepEqual(strip(language.elaborate(s).core),strip(language.elaborate(formatted).core));
    assert.equal(language.evaluate(s,input()).status,'ExpressionPrepared');
  }
});

test('financial source names are profile-local and original source/1 stays unchanged', () => {
  const oldSchema=schema(); delete oldSchema.variantTypes;
  const oldSource=source('let u256=7;let variant=8;requires u256 < variant;').replace(P,'moriarty-expression-source/1');
  assert.equal(createExpressionSourceV1(canonical(oldSchema)).check(oldSource).judgmentResult,'SourceChecked');
  for(const name of ['u256','variant','UInt256','magnitude']) assert.equal(api().check(source(`let ${name}=7;`)).code,'SOURCE_RESERVED_NAME');
  assert.equal(api().check(oldSource).code,'PROFILE_MISMATCH');
  assert.equal(createExpressionSourceV1(canonical(oldSchema)).check(source('let x=u256(7);')).code,'PROFILE_MISMATCH');
});

test('financial runtime additive check never evaluates or requires typed snapshots', () => {
  const s=source('let x=true ? 1 : floor_div(1,0);'), language=api();
  const elaborated=language.elaborate(s);
  const runtime=createFinancialExpressionContractV1(canonical(schema()));
  const request=canonical({contract:'moriarty-financial-expression-contract/1',source:s,core:elaborated.core,Pre:{unexpected:true},Args:{},Obs:{},workInitial:'0'});
  assert.deepEqual(runtime.check(request),{judgmentResult:'ExpressionChecked'});
  assert.equal(runtime.evaluate(request).code,'INPUT_SCHEMA');
});

test('one actual .mori fixture elaborates and evaluates all48 constructors', () => {
  const s=readFileSync(new URL('../spec/successor/examples/financial-all48.mori',import.meta.url),'utf8');
  const language=api({assets:['A','B'],parties:['Holder'],vaults:['Vault'],units:['USD'],
    variantTypes:{Quote:{Cash:['Amount','A'],Shares:['Shares','Vault','Holder']}},
    recordTypes:{Row:{n:['UInt64']}},enumTypes:{Mode:['Closed','Open']},args:{input:['UInt64'],row:['Record','Row']},
    observations:{ready:['Bool']},fields:{counter:{type:['UInt64'],writeClass:'ordinary'}},operations:{Notice:'Row'}});
  const e=language.elaborate(s); assert.equal(e.judgmentResult,'SourceElaborated',JSON.stringify(e));
  const seen=new Set();
  const visit=v=>{if(v&&typeof v==='object'){if(Object.hasOwn(v,'constructor'))seen.add(v.constructor);Object.values(v).forEach(visit);}};
  visit(e.core);
  assert.deepEqual([...seen].sort(),['LitUInt','LitSInt','LitBool','LitText','LitAmount','LitQuantity','LitShares','LitRate','LitPrice',
    'ReadLocal','ReadArg','ReadObs','ReadPre','ProjectField','AccessField','ProjectIndex','AccessIndex','ConstructRecord','ConstructEnum',
    'ConstructSome','ConstructNone','ConstructCollection','Add','Sub','Mul','FloorDiv','CeilDiv','Eq','Lt','Lte','Gt','Gte','Not','And','Or',
    'Require','Let','NextWrite','Ensure','Emit','ConstructAmount','ConstructShares','ConstructVariant','ProjectVariant','ProjectSome',
    'ConvertUInt','ScalarValue','Select'].sort());
  const result=language.evaluate(s,input('1000',{counter:'10'},{input:'2',row:{n:'42'}},{ready:true}));
  assert.equal(result.status,'ExpressionPrepared',JSON.stringify(result));
  assert.deepEqual(result.post,{counter:'12'});
  assert.deepEqual(result.descriptors,[{operation:'Notice',fields:{n:'42'}}]);
});

test('financial source exposes every new indexed type recursively without host aliases', () => {
  const rows=[['UInt256',['UInt256'],'7'],['Variant<Quote>',['Variant','Quote'],{tag:'Cash',value:'7'}],
    ['AmountProduct<A,B>',['AmountProduct','A','B'],'7'],['ScaledAmount<A,2>',['ScaledAmount','A','2'],'7'],
    ['SignedScaledAmount<A,2>',['SignedScaledAmount','A','2'],'-7'],['SignedAmount<A>',['SignedAmount','A'],'-7'],
    ['NetAmount<A>',['NetAmount','A'],'-7']];
  for(const [spelling,type,value] of rows){
    const language=api({...nominal,args:{arg:type},recordTypes:{Row:{v:type}},fields:{out:{type:['Collection',['Option',type],'1'],writeClass:'ordinary'}}});
    const s=source(`let r=record<Row>{v:arg};next.out=collection<Option<${spelling}>,1>(some<${spelling}>(r.v));`,`arg:${spelling}`);
    const result=language.evaluate(s,input('1000',{out:[]},{arg:value}));
    assert.equal(result.status,'ExpressionPrepared',JSON.stringify(result));
    assert.deepEqual(result.post.out,[[value]]);
  }
  const wrong=api({...nominal,args:{arg:['AmountProduct','B','A']}}).check(source('let x=arg;','arg:AmountProduct<B,A>'));
  assert.equal(wrong.code,'TYPE_LITERAL');
});

test('financial source scalar projections follow the complete closed overload table', () => {
  const rows=[['quanta','Amount<A>',['Amount','A'],'7',['UInt128'],'7'],
    ['quanta','Shares<Vault,Alice>',['Shares','Vault','Alice'],'7',['UInt128'],'7'],
    ['mantissa','Price<A,B,2>',['Price','A','B','2'],'7',['UInt128'],'7'],
    ['mantissa','Rate<2>',['Rate','2'],'-7',['SInt128'],'-7'],
    ['mantissa','Quantity<Units,2>',['Quantity',[],'2'],'-7',['SInt128'],'-7'],
    ...[['Rate<2>',['Rate','2']],['Quantity<Units,2>',['Quantity',[],'2']],['SignedAmount<A>',['SignedAmount','A']],['NetAmount<A>',['NetAmount','A']]]
      .map(([spelling,type])=>['is_negative',spelling,type,'-7',['Bool'],true]),
    ...[['Rate<2>',['Rate','2'],-(2n**127n)],['Quantity<Units,2>',['Quantity',[],'2'],-(2n**127n)],['NetAmount<A>',['NetAmount','A'],-(2n**128n-1n)]]
      .map(([spelling,type,value])=>['magnitude',spelling,type,String(value),['UInt128'],String(-value)]),
    ['magnitude','SignedAmount<A>',['SignedAmount','A'],String(-(2n**255n)),['UInt256'],String(2n**255n)]];
  for(const [fn,spelling,type,value,resultType,expected] of rows){
    const language=api({...nominal,args:{arg:type},fields:{out:{type:resultType,writeClass:'ordinary'}}});
    const result=language.evaluate(source(`next.out=${fn}(arg);`,`arg:${spelling}`),input('3',{out:expected},{arg:value}));
    assert.equal(result.status,'ExpressionPrepared',JSON.stringify(result));
    assert.equal(result.post.out,expected); assert.equal(result.workRemaining,'0');
  }
  for(const expr of ['quanta(7)','mantissa(amount(7,A))','magnitude(price(7,A,B,2))','is_negative(u256(0))']) rejected(expr,'TYPE_MISMATCH',nominal);
  const s=source('let x=magnitude(arg);','arg:SignedScaledAmount<A,2>');
  assert.equal(api({...nominal,args:{arg:['SignedScaledAmount','A','2']}}).check(s).code,'TYPE_MISMATCH');
});

test('financial source selected failure rolls back prior writes and descriptors with real byte paths', () => {
  const patch={recordTypes:{Row:{}},operations:{Notice:'Row'},fields:{n:{type:['UInt128'],writeClass:'ordinary'},funds:{type:['UInt128'],writeClass:'financial'}}};
  const s=source('let label="é😀";next.n=1;emit Notice {};let failed=true ? some_value(none<UInt128>()) : 7;');
  const result=api(patch).evaluate(s,input('100',{n:'0',funds:'8'}));
  const text='some_value(none<UInt128>())', start=s.indexOf(text);
  assert.deepEqual(result,{status:'Rejected',code:'OPTION_NONE',span:{kind:'source',start:String(Buffer.byteLength(s.slice(0,start))),end:String(Buffer.byteLength(s.slice(0,start+text.length)))},nodePath:['3','0','1'],workUsed:'11'});
  assert.equal(api(patch).evaluate(source('next.n=1;emit Notice {};ensures false;'),input('100',{n:'0',funds:'8'})).code,'ENSURES_FAILED');
  assert.equal(api(patch).check(source('next.funds=1;')).code,'TYPE_FINANCIAL_WRITE');
});

test('financial source exact new-node spans, metadata operands and arity failures', () => {
  const rows=[['amount<A>(5)','ConstructAmount',{asset:'A'}],['shares<Vault,Alice>(5)','ConstructShares',{vault:'Vault',holder:'Alice'}],
    ['variant<Quote,Cash>(amount(5,A))','ConstructVariant',{family:'Quote',tag:'Cash'}],
    ['project_variant<Cash>(variant<Quote,Cash>(amount(5,A)))','ProjectVariant',{tag:'Cash'}],
    ['some_value(some<UInt128>(5))','ProjectSome',{}],['to_uint<256>(5)','ConvertUInt',{width:'256'}],
    ['is_negative(rate(-1,0))','ScalarValue',{component:'negative'}],['true ? 5 : 6','Select',{}]];
  for(const [expr,tag,metadata] of rows){
    const s=source(`let label="é😀";let x=${expr};`),e=api(nominal).elaborate(s),n=e.core.statements[1].operands.value,start=s.indexOf(expr);
    assert.equal(n.constructor,tag);for(const [k,v] of Object.entries(metadata))assert.equal(n.operands[k],v);
    assert.deepEqual(n.span,{kind:'source',start:String(Buffer.byteLength(s.slice(0,start))),end:String(Buffer.byteLength(s.slice(0,start+expr.length)))});
  }
  for(const expr of ['amount<A,B>(5)','shares<Vault>(5)','variant<Quote>(5)','some_value()','to_uint<256>(1,2)']) rejected(expr,'SOURCE_ARITY',nominal);
  for(const expr of ['amount<1>(5)','to_uint<UInt256>(5)','variant<Quote,Record<Row>>(5)']) rejected(expr,'SOURCE_TYPE_SHAPE',nominal);
  assert.equal(api(nominal).check(source('let x=variant<Quote.Cash>(5);')).code,'UNEXPECTED_TOKEN');
});

test('financial source transport/ownership and phase priorities preserve source/1 contract', () => {
  const language=api(),s=source('let x=true ? 7 : floor_div(1,0);');
  const first=language.elaborate(s);first.core.statements[0].operands.value.constructor='bad';
  assert.equal(language.evaluate(s,input('4')).status,'ExpressionPrepared');
  assert.equal(language.evaluate('invalid',' '.repeat(2000001)).code,'INPUT_BOUND');
  assert.equal(language.evaluate('invalid',input()).code,'UNEXPECTED_TOKEN');
  let touched=false;const hostile={get value(){touched=true;throw Error('read');}};
  assert.equal(language.check(hostile).code,'SOURCE_TYPE');
  assert.equal(language.evaluate(s,hostile).code,'INPUT_SCHEMA');assert.equal(touched,false);
  assert.equal(language.evaluate(source('let x=some_value(none<UInt128>());'),input('100',{}, {extra:'1'})).code,'INPUT_SCHEMA');
  for(const method of ['elaborate','check','evaluate']){
    const bad=source('let x=access_field(unknown(),"bad field");');
    const result=method==='evaluate'?language.evaluate(bad,input()):language[method](bad);
    assert.equal(result.code,'SOURCE_CALL');assert.equal(result.workUsed,'0');
  }
});

const vaultOracle=JSON.parse(readFileSync(new URL('../../../deliverables/sp02-financial-pure-expression-2026-09-10/inputs/independent-vault-arithmetic-01.json',import.meta.url),'utf8'));
for(const row of vaultOracle.cases) test(`financial source independent vault equation: ${row.id}`,()=>{
  const shares=['deposit','withdraw'].includes(row.operation),ceil=['mint','withdraw'].includes(row.operation);
  const numerator=shares?'supply':'valuation',denominator=shares?'valuation':'supply';
  const s=source(`requires supply > 0;requires valuation > 0;next.out=to_uint<128>(${ceil?'ceil_div':'floor_div'}(to_uint<256>(q)*to_uint<256>(${numerator}),to_uint<256>(${denominator})));`,'q:UInt128,supply:UInt128,valuation:UInt128');
  const language=api({args:{q:['UInt128'],supply:['UInt128'],valuation:['UInt128']},fields:{out:{type:['UInt128'],writeClass:'ordinary'}}});
  assert.equal(language.check(s).staticWorkBound,'18');
  const result=language.evaluate(s,input('18',{out:'0'},{q:row.quantity,supply:row.supply,valuation:row.valuation}));
  if(row.expect.output!==undefined){assert.equal(result.status,'ExpressionPrepared',JSON.stringify(result));assert.equal(result.post.out,row.expect.output);assert.equal(result.workRemaining,'0');}
  else {
    const code={RESULT_UINT128_BOUND:'ARITH_RANGE',INPUT_UINT128_BOUND:'INPUT_VALUE',OUTSIDE_INITIALIZED_SUBRULE:'GUARD_FAILED'}[row.expect.rejection];
    assert.ok(code);assert.equal(result.status,'Rejected');assert.equal(result.code,code,JSON.stringify(result));
  }
});
