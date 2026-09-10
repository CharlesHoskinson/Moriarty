import assert from 'node:assert/strict';
import {createExpressionContractV1} from '../../experiments/moriarty-language/src/successor/expression-v1.ts';
const J=v=>Array.isArray(v)?'['+v.map(J).join(',')+']':v!==null&&typeof v==='object'?'{'+Object.keys(v).sort().map(k=>JSON.stringify(k)+':'+J(v[k])).join(',')+'}':JSON.stringify(v);
const P={kind:'synthetic',start:'0',end:'0'};
const N=(constructor,operands={})=>({constructor,operands,span:P});
const base=()=>({units:['U'],assets:['A','B'],vaults:['V'],parties:['H'],recordTypes:{R:{}},enumTypes:{E:['M']},fields:{},args:{},observations:{},operations:{O:'R'}});
const req=(core,work='1',Args={},source='')=>J({Args,Obs:{},Pre:{},contract:'moriarty-expression-contract/1',core:'CORE',source,workInitial:work}).replace('"CORE"',core);
const core=t=>J(N('ConstructNone',{elementType:'TYPE'})).replace('"TYPE"',t);
const option=(n,t='["Bool"]')=>'["Option",'.repeat(n)+t+']'.repeat(n);
const mixed=(n,t='["Bool"]',cap='128')=>{for(let i=0;i<n;i++)t=i%2?'["Option",'+t+']':'["Collection",'+t+','+JSON.stringify(cap)+']';return t;};
let checks=0;const groups={};
function check(label,actual,expected){checks++;groups[label]=(groups[label]??0)+1;assert.equal(actual,expected,label);}
function run(t,work='1',s=base()){return createExpressionContractV1(J(s)).evaluate(req(core(t),work));}
function rejection(label,r,code){check(label,r.code,code);check(label,r.workUsed,'0');}
const start=performance.now();
for(const n of [0,1,60,508,509,512,1000,2500,4000,5900]){
 const r=run(option(n));check('Option accepted',r.judgmentResult,'ExpressionValue');check('Option empty',r.value.length,0);check('Option work',r.workRemaining,'0');
 let t=r.type;for(let i=0;i<n+1;i++){assert.equal(t[0],'Option');t=t[1];}assert.deepEqual(t,['Bool']);
 rejection('Option no work',run(option(n),'0'),'WORK_EXHAUSTED');
}
const leaves=[['Bool'],['Text'],['SInt128'],['UInt64'],['UInt128'],['Amount','A'],['Shares','V','H'],['Rate','18'],['Price','A','B','0'],['Quantity',[['U','-128']],'18'],['Record','R'],['Enum','E'],['Operation','O']];
for(const leaf of leaves)for(const depth of [509,1900,3000]){
 const t=mixed(depth,J(leaf));if(Buffer.byteLength(core(t))>65536)continue;
 const r=run(t);check('mixed resolved leaf',r.judgmentResult,'ExpressionValue');check('mixed value',r.value.length,0);
}
for(const [t,code] of [[['Unit'],'TYPE_NAME'],[['Record','Missing'],'TYPE_NAME'],[['Amount','Missing'],'TYPE_NAME'],[['Shares','V','Missing'],'TYPE_NAME'],[['Rate','19'],'TYPE_LITERAL'],[['Price','A','A','0'],'TYPE_LITERAL'],[['Quantity',[['U','0']],'0'],'TYPE_LITERAL'],[['Quantity',[['Missing','1']],'0'],'TYPE_NAME'],[['Unknown'],'INPUT_SCHEMA'],[['Bool','extra'],'INPUT_SCHEMA']]) rejection('deep leaf rejection',run(mixed(1500,J(t)),'0'),code);
for(const cap of ['-1','129'])rejection('capacity range',run(mixed(1500,'["Bool"]',cap),'0'),'TYPE_COLLECTION_BOUND');
for(const cap of ['00','-0','+1','1\n'])rejection('capacity spelling',run(mixed(1500,'["Bool"]',cap),'0'),'INPUT_SCHEMA');
rejection('inner nominal before capacity',run('["Collection",'+mixed(1500,'["Record","Missing"]')+',"129"]','0'),'TYPE_NAME');
rejection('shape before nominal',run('["Collection",'+mixed(1500,'["Record","Missing"]')+',"00"]','0'),'INPUT_SCHEMA');
let lo=0;while(Buffer.byteLength(core(option(lo+1)))<=65536)lo++;
check('max Option depth',lo,5947);check('max Option accepted',run(option(lo)).judgmentResult,'ExpressionValue');rejection('next Option Core byte bound',run(option(lo+1)),'INPUT_BOUND');
const api=createExpressionContractV1(J(base()));
for(const t of [option(2000).replace('["Bool"]','[ "Bool"]'),option(2000).replace('Bool','\\u0042ool'),option(2000).replace('["Bool"]','["Bool",null]'),option(2000).replace('["Bool"]','["Bool",1]'),option(2000).replace('["Bool"]','["\\ud800"]')])rejection('noncanonical deep',api.evaluate(req(core(t))),'INPUT_SCHEMA');
rejection('duplicate envelope',api.evaluate(req(core(option(2000))).replace('"Args":{}','"Args":{},"Args":{}')),'INPUT_SCHEMA');
for(const n of [60,61,2000]){
 const s=J(base()).replace('"args":{}','"args":{"x":'+option(n)+'}');
 const r=createExpressionContractV1(s).evaluate(req(J(N('LitBool',{value:true})),'1',{x:[]}));
 if(n===60)check('schema depth64',r.judgmentResult,'ExpressionValue');else rejection('schema depth bound',r,'INPUT_BOUND');
}
for(const cycle of [false,true]){
 const s=base();s.recordTypes={};for(let i=0;i<100;i++)s.recordTypes['R'+String(i).padStart(3,'0')]={f:i===99?(cycle?['Operation','O']:['Bool']):['Option',['Collection',['Record','R'+String(i+1).padStart(3,'0')],'0']]};s.operations.O='R000';
 const r=createExpressionContractV1(J(s)).evaluate(req(core('["Record","R000"]')));
 if(cycle)rejection('mixed schema cycle',r,'TYPE_SCHEMA_CYCLE');else check('acyclic nominal chain',r.judgmentResult,'ExpressionValue');
}
for(const n of [63,64,65]){
 let c=N('LitBool',{value:true});for(let i=1;i<n;i++)c=N('Not',{value:c});const r=api.evaluate(req(J(c),'65'));
 if(n<=64)check('Core depth accepted',r.judgmentResult,'ExpressionValue');else rejection('Core depth65',r,'INPUT_BOUND');
}
for(const work of ['65536','65537','-1','0','00']){
 const r=api.evaluate(req(J(N('LitBool',{value:true})),work));
 if(work==='65536')check('work upper',r.workRemaining,'65535');else rejection('work bounds',r,work==='0'?'WORK_EXHAUSTED':work==='00'?'INPUT_SCHEMA':'INPUT_BOUND');
}
for(const source of ['a'.repeat(65536),'a'.repeat(65537),'😀'.repeat(16384),'😀'.repeat(16384)+'a']){
 const r=api.evaluate(req(J(N('LitBool',{value:true})),'1',{},source));if(Buffer.byteLength(source)<=65536)check('source UTF8 exact',r.judgmentResult,'ExpressionValue');else rejection('source UTF8 over',r,'INPUT_BOUND');
}
// Independently solve the standalone W boundary for a 64-item Text collection.
const s=base();s.args={a:['Text'],b:['Text']};const items=Array.from({length:64},(_,i)=>N('ReadArg',{name:i===63?'b':'a'}));
const c=J(N('ConstructCollection',{elementType:['Text'],capacity:'64',items}));
const wlen=n=>Buffer.byteLength(J({type:['Collection',['Text'],'64'],value:[...Array(63).fill('a'.repeat(1024)),'b'.repeat(n)]}));
const exact=65536-wlen(0);assert.ok(exact>=0&&exact<1024);
for(const n of [exact,exact+1]){const r=createExpressionContractV1(J(s)).evaluate(req(c,'65',{a:'a'.repeat(1024),b:'b'.repeat(n)}));if(n===exact){check('W exact65536',r.judgmentResult,'ExpressionValue');check('W work',r.workRemaining,'0');}else{check('W65537',r.code,'VALUE_BOUND');check('W consumed work',r.workUsed,'65');}}
// Snapshot value-node aggregate separately from JSON bytes and metadata.
const sn=base();sn.args={x:['Collection',['Collection',['Bool'],'128'],'32']};
for(const total of [4096,4097]){const x=Array.from({length:32},()=>Array(127).fill(true));let remove=4098-total;while(remove--)x.at(-1).pop();const r=createExpressionContractV1(J(sn)).evaluate(req(J(N('LitBool',{value:true})),'1',{x}));if(total===4096)check('snapshot4096',r.judgmentResult,'ExpressionValue');else rejection('snapshot4097',r,'INPUT_BOUND');}
// Deep type equality through a Some constructor and nominally distinct leaves.
for(const different of [false,true]){
 const c=J(N('ConstructSome',{elementType:'ANNOTATION',value:'VALUE'})).replace('"ANNOTATION"',option(2400,'["Record","R"]')).replace('"VALUE"',core(option(2399,different?'["Enum","E"]':'["Record","R"]')));
 const r=api.evaluate(req(c,'2'));if(different)rejection('deep exact type mismatch',r,'TYPE_MISMATCH');else{check('deep exact type agreement',r.judgmentResult,'ExpressionValue');assert.deepEqual(r.value,[[]]);check('deep Some work',r.workRemaining,'0');}
}
// A compact acyclic schema may describe an overdeep value through named records.
// The public API must return its specified bound failure before reduction.
{
 const s=base();s.recordTypes={};s.operations={};s.args={x:['Record','R00']};
 for(let i=0;i<30;i++){let t=i===29?['Bool']:['Record','R'+String(i+1).padStart(2,'0')];for(let k=0;k<58;k++)t=['Option',t];s.recordTypes['R'+String(i).padStart(2,'0')]={f:t};}
 let v='true';for(let i=0;i<30;i++)v='{"f":'+'['.repeat(58)+v+']'.repeat(58)+'}';
 const r=req(J(N('LitBool',{value:true}))).replace('"Args":{}','"Args":{"x":'+v+'}');
 rejection('expanded acyclic snapshot depth',createExpressionContractV1(J(s)).evaluate(r),'INPUT_BOUND');
}
console.log(JSON.stringify({status:'PASS',checks,groups,maxOptionDepth:lo,exactWLastTextBytes:exact,durationMs:performance.now()-start}));
