import test from 'node:test';
import assert from 'node:assert/strict';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { parseFinancialExpressionSource, formatFinancialExpressionSource } from '../src/successor/financial-expression-source-frontend.ts';
import { parseExpressionSource } from '../src/successor/expression-source-frontend.ts';
import { parseSuccessorSource, FINANCIAL_GENERIC_PRIMARIES, GENERIC_PRIMARIES } from '../src/successor/frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';
const P='moriarty-financial-expression-source/1';
const schema=patch=>({units:[],assets:[],vaults:[],parties:[],recordTypes:{},enumTypes:{},variantTypes:{},fields:{},args:{},observations:{},operations:{},...patch});
const source=(body,params='')=>`profile "${P}";agreement Demo{action step(${params}){${body}}}`;
const check=(s,patch)=>createFinancialExpressionSourceV1(canonical(schema(patch))).check(s);

test('financial parser has gated punctuation, generic names, arities and unchanged old catalogs',()=>{
  assert.deepEqual(GENERIC_PRIMARIES,['some','none','collection','quantity','record']);
  assert.ok(Object.isFrozen(FINANCIAL_GENERIC_PRIMARIES));
  assert.throws(()=>FINANCIAL_GENERIC_PRIMARIES.push('bad'),TypeError);
  for(const [parse,profile] of [[parseExpressionSource,'moriarty-expression-source/1'],[parseSuccessorSource,'moriarty-successor-syntax/0']]){
    assert.throws(()=>parse(source('let x=true?1:2;').replace(P,profile)),{code:'UNEXPECTED_CHAR'});
  }
  for(const text of ['let x=variant<F.T>(1);','let x=amount<>(1);','let x=true?1;','let x=variant(1);'])
    assert.throws(()=>parseFinancialExpressionSource(source(text)),{code:'UNEXPECTED_TOKEN'});
});

test('financial source enforces lexical metadata roles and profile-local name domains',()=>{
  for(const name of ['amount','shares','variant','project_variant','to_uint','requires'])
    assert.equal(check(source(''),{assets:[name]}).code,'SOURCE_SCHEMA_NAME');
  assert.equal(check(source(''),{variantTypes:{Family:{amount:['UInt128']}}}).code,'SOURCE_SCHEMA_NAME');
  assert.equal(check(source(''),{variantTypes:{Family:{One:['UInt128']}},enumTypes:{Family:['One']}}).code,'SOURCE_SCHEMA_NAME');
  assert.equal(check(source('','arg:UInt256'),{args:{arg:['UInt128']}}).code,'SOURCE_PARAMETER_TYPE');
  assert.equal(check(source('let x=none<AmountProduct<1,A>>();')).code,'SOURCE_TYPE_SHAPE');
  assert.equal(check(source('let x=to_uint<-0>(1);')).code,'SOURCE_TYPE_SHAPE');
});

test('financial schema variants retain distinct payloads, cycles and 128-case bound',()=>{
  const cases=Object.fromEntries(Array.from({length:128},(_,i)=>['Case'+i,['Collection',['UInt128'],String(i)]]));
  assert.equal(check(source('let x=none<Variant<Family>>();'),{variantTypes:{Family:cases}}).judgmentResult,'SourceChecked');
  assert.equal(check(source(''),{variantTypes:{Family:{...cases,Extra:['UInt256']}}}).code,'INPUT_BOUND');
  assert.equal(check(source(''),{variantTypes:{Family:{One:['UInt128'],Two:['UInt128']}}}).code,'INPUT_SCHEMA');
  assert.equal(check(source(''),{variantTypes:{Family:{One:['Variant','Family']}}}).code,'TYPE_SCHEMA_CYCLE');
  assert.equal(check(source(''),{variantTypes:{Family:{}}}).code,'INPUT_BOUND');
});

test('financial collections and action parameters retain 128 and 256 bounds',()=>{
  const s=source(`let x=collection<UInt256,128>(${Array(128).fill('u256(1)').join(',')});`);
  assert.equal(check(s).judgmentResult,'SourceChecked');
  assert.equal(formatFinancialExpressionSource(formatFinancialExpressionSource(s)),formatFinancialExpressionSource(s));
  assert.equal(check(source(`let x=collection<UInt256,128>(${Array(129).fill('u256(1)').join(',')});`)).code,'ARITY_BOUND');
  const entries=Array.from({length:256},(_,i)=>['p'+i,['UInt256']]);
  const params=entries.map(([name])=>name+':UInt256').join(',');
  assert.equal(check(source('',params),{args:Object.fromEntries(entries)}).judgmentResult,'SourceChecked');
  assert.equal(check(source('',params+',extra:UInt256')).code,'ARITY_BOUND');
});

test('financial source conditional depth, tokens and source bounds remain bounded',()=>{
  const tooDeep='true ? 1 : '.repeat(70)+'1';
  assert.equal(check(source(`let x=${tooDeep};`)).code,'NESTING_BOUND');
  assert.equal(check(' '.repeat(65537)).code,'SOURCE_BOUND');
  assert.equal(check(source('let x='+Array(9000).fill('1').join('+')+';')).code,'TOKEN_BOUND');
});
