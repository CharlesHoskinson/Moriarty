import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {createFinancialExpressionSourceV1} from '../../../experiments/moriarty-language/src/successor/financial-expression-source-v1.ts';
import {formatFinancialExpressionSource,parseFinancialExpressionSource} from '../../../experiments/moriarty-language/src/successor/financial-expression-source-frontend.ts';
import {canonical} from '../../../experiments/moriarty-language/src/successor/expression-wire-v1.ts';
const profile='moriarty-financial-expression-source/1';
const schema={units:[],assets:['A','B'],vaults:[],parties:[],recordTypes:{Row:{}},enumTypes:{},variantTypes:{Choice:{Cash:['Amount','A'],Number:['UInt128']}},fields:{n:{type:['UInt128'],writeClass:'ordinary'}},args:{},observations:{},operations:{Notice:'Row'}};
const api=createFinancialExpressionSourceV1(canonical(schema));
const src=body=>`profile "${profile}";agreement Probe{action step(){${body}}}`;
const snap=work=>canonical({Pre:{n:'0'},Args:{},Obs:{},workInitial:String(work)});
const stripped=v=>Array.isArray(v)?v.map(stripped):v&&typeof v==='object'?Object.fromEntries(Object.entries(v).filter(([k])=>k!=='span').map(([k,w])=>[k,stripped(w)])):v;
const results=[];
function check(name,f){f();results.push(name);}
check('conditional generated AST/Core/result roundtrips',()=>{
 const bools=['true','false','not false','true or false','true and false','(false ? true : false)'];
 const ints=['1','2+3','true ? 4 : 5','false ? 6 : 7'];
 for(const b of bools)for(const x of ints)for(const y of ints){
  const s=src(`next.n=(${b}) ? (${x}) : (${y});`),f=formatFinancialExpressionSource(s);
  assert.equal(formatFinancialExpressionSource(f),f);
  assert.deepEqual(stripped(parseFinancialExpressionSource(s)),stripped(parseFinancialExpressionSource(f)));
  const e=api.elaborate(s);assert.equal(e.judgmentResult,'SourceElaborated');
  assert.deepEqual(stripped(e.core),stripped(api.elaborate(f).core));
  assert.deepEqual(api.evaluate(s,snap(100)),api.evaluate(f,snap(100)));
 }
});
check('unselected case failure skipped; selected rollback exact span and work',()=>{
 const bad='quanta(project_variant<Cash>(variant<Choice,Number>(3)))';
 const s=src(`let label="é😀";next.n=1;emit Notice {};let x=false ? 7 : ${bad};`);
 const r=api.evaluate(s,snap(100));
 assert.equal(r.code,'VARIANT_CASE');assert.equal(r.workUsed,'13');assert.deepEqual(r.nodePath,['3','0','2','0']);
 assert.equal('post' in r,false);assert.equal('descriptors' in r,false);
 const needle='project_variant<Cash>(variant<Choice,Number>(3))',i=s.indexOf(needle);
 assert.deepEqual(r.span,{kind:'source',start:String(Buffer.byteLength(s.slice(0,i))),end:String(Buffer.byteLength(s.slice(0,i+needle.length)))});
 const yes=s.replace('let x=false','let x=true');assert.equal(api.evaluate(yes,snap(100)).status,'ExpressionPrepared');
});
check('both Select arms typed before malformed typed snapshots',()=>{
 const s=src('let x=true ? 7 : project_variant<Missing>(variant<Choice,Number>(3));');
 const r=api.evaluate(s,canonical({Pre:{},Args:{wrong:true},Obs:{},workInitial:'0'}));
 assert.equal(r.code,'TYPE_NAME');assert.equal(r.workUsed,'0');
});
check('scaled cancellation retains literal node restriction',()=>{
 for(const divisor of ['u64(10)','to_uint<128>(10)','d']){
  const r=api.check(src(`let d=10;let x=floor_div(amount(5,A)*rate(-5,1),${divisor});`));
  assert.equal(r.code,'TYPE_SCALE_DIVISOR');
 }
 assert.equal(api.check(src('let x=floor_div(amount(5,A)*rate(-5,1),u128(10));')).judgmentResult,'SourceChecked');
});
check('per-call owned rejection and result structures',()=>{
 const s=src('next.n=1;');const r=api.evaluate(s,snap(3));r.post.n='999';
 assert.equal(api.evaluate(s,snap(3)).post.n,'1');
 const bad=src('let x=some_value(none<UInt128>());');const err=api.evaluate(bad,snap(10));err.nodePath.push('999');err.span.start='999';
 assert.notEqual(api.evaluate(bad,snap(10)).span.start,'999');
});
check('CLI published fixture, unknown-profile precedence and no writes',()=>{
 const base='experiments/moriarty-language/spec/successor/examples/financial-vault-quote';
 const paths=[base+'.mori',base+'.schema.json'];const hashes=paths.map(p=>createHash('sha256').update(readFileSync(p)).digest('hex'));
 const run=a=>spawnSync(process.execPath,['experiments/moriarty-language/src/cli.ts',...a],{encoding:'utf8',timeout:5000});
 let r=run(['check','--profile',profile,'--schema',paths[1],paths[0]]);assert.equal(r.status,0);assert.equal(r.stderr,'');
 const language=createFinancialExpressionSourceV1(readFileSync(paths[1],'utf8'));
 assert.equal(r.stdout,JSON.stringify(language.check(readFileSync(paths[0],'utf8')))+'\n');
 r=run(['format','--profile','invalid','/dev/null']);assert.equal(r.status,2);assert.equal(r.stdout,'');assert.equal(JSON.parse(r.stderr).code,'CLI_PROFILE');
 r=run(['format','--profile',profile,'/dev/null']);assert.equal(r.status,2);assert.equal(JSON.parse(r.stderr).code,'CLI_IO');
 assert.deepEqual(paths.map(p=>createHash('sha256').update(readFileSync(p)).digest('hex')),hashes);
});
console.log(JSON.stringify({passed:true,groups:results.length,generatedRoundtrips:96,checks:results},null,2));
