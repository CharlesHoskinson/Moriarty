import assert from 'node:assert/strict';
import {createExpressionContractV1} from '../../experiments/moriarty-language/src/successor/expression-v1.ts';
const scalarCmp=(a,b)=>{const x=[...a].map(x=>x.codePointAt(0)),y=[...b].map(x=>x.codePointAt(0));for(let i=0;i<Math.min(x.length,y.length);i++)if(x[i]!==y[i])return x[i]-y[i];return x.length-y.length;};
const J=v=>typeof v!=='object'||v===null?JSON.stringify(v):Array.isArray(v)?'['+v.map(J).join(',')+']':'{'+Object.keys(v).sort(scalarCmp).map(k=>JSON.stringify(k)+':'+J(v[k])).join(',')+'}';
const P={kind:'synthetic',start:'0',end:'0'};
const N=(constructor,operands,span=P)=>({constructor,operands,span});
const B=v=>N('LitBool',{value:v}); const I=v=>N('LitSInt',{value:String(v)});const U=v=>N('LitUInt',{width:'64',value:String(v)});const Bin=(k,a,b)=>N(k,{left:a,right:b});
const base=()=>({units:['A','Z','a'],assets:['A','B'],vaults:['V'],parties:['H'],recordTypes:{R:{a:['UInt64'],z:['Bool']}},enumTypes:{E:['A','Z']},fields:{x:{type:['UInt64'],writeClass:'ordinary'},debt:{type:['UInt64'],writeClass:'financial'}},args:{n:['UInt64']},observations:{ok:['Bool']},operations:{O:'R'}});
const req=(core,extras={})=>({contract:'moriarty-expression-contract/1',source:'',core,Pre:{x:'10',debt:'4'},Args:{n:'8'},Obs:{ok:true},workInitial:'100',...extras});
const ev=(core,extras={},schema=base())=>createExpressionContractV1(J(schema)).evaluate(J(req(core,extras)));
let checks=0;const check=(label,f)=>{f();checks++;};
const value=(k,e,w)=>{assert.equal(k.judgmentResult,'ExpressionValue',J(k));assert.deepEqual(k.value,e);if(w!==undefined)assert.equal(k.workRemaining,String(w));};
const reject=(k,code,work='0',path)=>{assert.equal(k.status,'Rejected',J(k));assert.equal(k.code,code,J(k));assert.equal(k.workUsed,String(work));assert.deepEqual(Object.keys(k).sort(),['code','nodePath','span','status','workUsed']);if(path)assert.deepEqual(k.nodePath,path.map(String));};
const literals=[['LitUInt',{width:'128',value:'340282366920938463463374607431768211455'},'340282366920938463463374607431768211455'],['LitSInt',{value:'-170141183460469231731687303715884105728'},'-170141183460469231731687303715884105728'],['LitBool',{value:true},true],['LitText',{value:'\u0000\b\f\n\r\t\\"😀é'},'\u0000\b\f\n\r\t\\"😀é'],['LitAmount',{asset:'A',value:'9'},'9'],['LitQuantity',{units:[['A','1'],['a','-1']],scale:'2',mantissa:'-7'},'-7'],['LitShares',{vault:'V',holder:'H',value:'3'},'3'],['LitRate',{scale:'18',mantissa:'-2'},'-2'],['LitPrice',{base:'A',quote:'B',scale:'2',mantissa:'12'},'12']];
for(const [k,o,v] of literals)check(k,()=>value(ev(N(k,o)),v,99));
for(const [k,o,v] of [['ReadPre',{view:'pre',field:'x'},'10'],['ReadArg',{name:'n'},'8'],['ReadObs',{name:'ok'},true]])check(k,()=>value(ev(N(k,o)),v,99));
const record=N('ConstructRecord',{recordType:'R',fields:[{name:'z',value:B(true)},{name:'a',value:U(7)}]});
check('record',()=>value(ev(record),{a:'7',z:true},97));
for(const k of ['ProjectField','AccessField'])check(k,()=>value(ev(N(k,{record,field:'a'})),'7',96));
const collection=N('ConstructCollection',{elementType:['UInt64'],capacity:'2',items:[U(5),U(8)]});
check('collection',()=>value(ev(collection),['5','8'],97));
for(const k of ['ProjectIndex','AccessIndex']){check(k,()=>value(ev(N(k,{collection,index:U(1)})),'8',95));check(k+'range',()=>reject(ev(N(k,{collection,index:U(2)})),'INDEX_RANGE',5));}
check('enum',()=>value(ev(N('ConstructEnum',{enumType:'E',member:'Z'})),'Z',99));
check('some',()=>value(ev(N('ConstructSome',{elementType:['Bool'],value:B(false)})),[false],98));
check('none',()=>value(ev(N('ConstructNone',{elementType:['Bool']})),[],99));
check('Not',()=>value(ev(N('Not',{value:B(true)})),false,98));
for(let a=-20;a<=20;a++)for(let b=-20;b<=20;b++)for(const k of ['Add','Sub','Mul','Eq','Lt','Lte','Gt','Gte'])check(k,()=>value(ev(Bin(k,I(a),I(b))),({Add:String(a+b),Sub:String(a-b),Mul:String(a*b),Eq:a===b,Lt:a<b,Lte:a<=b,Gt:a>b,Gte:a>=b})[k],97));
for(let a=-20;a<=20;a++)for(let b=-3;b<=10;b++)for(const k of ['FloorDiv','CeilDiv'])check(k,()=>{const out=ev(Bin(k,I(a),I(b))); if(b<=0)reject(out,'ARITH_DENOMINATOR',3);else value(out,String(k==='FloorDiv'?Math.floor(a/b):Math.ceil(a/b)),97);});
const doom=Bin('Eq',Bin('FloorDiv',I(1),I(0)),I(1));
for(const k of ['And','Or'])for(const a of [false,true])for(const b of [false,true])check(k,()=>value(ev(Bin(k,B(a),B(b))),k==='And'?a&&b:a||b,100-((k==='And'?!a:a)?2:3)));
for(const [k,a] of [['And',false],['Or',true]])check('short circuit exact exhaustion '+k,()=>value(ev(Bin(k,B(a),doom),{workInitial:'2'}),a,0));
for(const [k,a] of [['And',true],['Or',false]]){check('selected right exhausted '+k,()=>reject(ev(Bin(k,B(a),doom),{workInitial:'2'}),'WORK_EXHAUSTED',2,[1]));check('selected right denominator '+k,()=>reject(ev(Bin(k,B(a),doom)),'ARITH_DENOMINATOR',6,[1,0]));}
const act=statements=>({statements,span:P});const stmt=(k,o)=>N(k,o);
const staged=act([stmt('Require',{condition:B(true)}),stmt('Let',{name:'local',value:Bin('Add',N('ReadPre',{view:'pre',field:'x'}),U(1))}),stmt('NextWrite',{field:'x',value:N('ReadLocal',{name:'local'})}),stmt('Emit',{operation:'O',fields:record}),stmt('Ensure',{condition:Bin('Eq',N('ReadPre',{view:'post',field:'x'}),U(11))})]);
check('all statements prepared',()=>{const o=ev(staged);assert.equal(o.status,'ExpressionPrepared',J(o));assert.deepEqual(o.post,{debt:'4',x:'11'});assert.deepEqual(o.descriptors,[{operation:'O',fields:{a:'7',z:true}}]);assert.equal(o.workRemaining,'84');});
check('financial write static before guard',()=>reject(ev(act([N('Require',{condition:B(false)}),N('NextWrite',{field:'debt',value:U(0)})])),'TYPE_FINANCIAL_WRITE',0,[1]));
check('missing unused snapshot before guard',()=>reject(ev(act([N('Require',{condition:B(false)})]),{Obs:{}}),'INPUT_SCHEMA'));
check('domain before work0',()=>reject(ev(B(true),{workInitial:'0',Args:{n:'18446744073709551616'}}),'INPUT_VALUE'));
check('statement static before snapshot',()=>reject(ev(N('Not',{value:U(1)}),{Args:{}}),'TYPE_MISMATCH'));
check('duplicate binder before child',()=>reject(ev(act([N('Let',{name:'n',value:N('ReadLocal',{name:'Absent'})})])),'TYPE_DUPLICATE_BINDER',0,[0]));
check('duplicate record field before child',()=>reject(ev(N('ConstructRecord',{recordType:'R',fields:[{name:'a',value:N('ReadLocal',{name:'bad'})},{name:'a',value:U(0)}]})),'TYPE_DUPLICATE_FIELD'));
check('original selected right span',()=>{const broken=N('FloorDiv',{left:I(1),right:I(0)},{kind:'source',start:'4',end:'8'});const o=ev(Bin('Or',B(false),Bin('Eq',broken,I(0))),{source:'abcdefgh'});reject(o,'ARITH_DENOMINATOR',6,[1,0]);assert.deepEqual(o.span,broken.span);});
check('invalid span synthetic',()=>{const x=I(0);x.span={kind:'source',start:'-1',end:'0'};const o=ev(Bin('Add',I(1),x));reject(o,'INPUT_SPAN',0,[1]);assert.deepEqual(o.span,P);});
for(const bad of ['01','-0','+1','1\n'])check('decimal spelling '+bad,()=>reject(ev(I(bad)),'INPUT_SCHEMA'));
for(const text of ['😀'.repeat(256),'é'.repeat(512)])check('text1024',()=>value(ev(N('LitText',{value:text})),text));
for(const text of ['😀'.repeat(257),'é'.repeat(513)])check('text over1024',()=>reject(ev(N('LitText',{value:text})),'TYPE_LITERAL'));
check('no normalization',()=>value(ev(Bin('Eq',N('LitText',{value:'é'}),N('LitText',{value:'é'}))),false));
check('tuple is no recognized type',()=>{const s=base();s.args.n=['Tuple',['Bool'],['Bool']];reject(ev(B(true),{},s),'INPUT_SCHEMA');});
check('option shape',()=>{const s=base();s.args.n=['Option',['Bool']];reject(ev(B(true),{Args:{n:[true,false]}},s),'INPUT_SCHEMA');});
check('operation cycle despite None',()=>{const s=base();s.recordTypes.R.a=['Option',['Operation','O']];reject(ev(B(true),{},s),'TYPE_SCHEMA_CYCLE');});
check('none Unit rejected',()=>reject(ev(N('ConstructNone',{elementType:['Unit']})),'TYPE_NAME'));
check('exact capacity types',()=>reject(ev(Bin('Eq',N('ConstructCollection',{elementType:['Bool'],capacity:'0',items:[]}),N('ConstructCollection',{elementType:['Bool'],capacity:'1',items:[]}))),'TYPE_MISMATCH'));
check('schema binds only host',()=>{const r=req(B(true));r.schema=base();reject(createExpressionContractV1(J(base())).evaluate(J(r)),'INPUT_SCHEMA');});
check('fresh call ownership',()=>{const c=createExpressionContractV1(J(base()));const r=J(req(staged));const a=c.evaluate(r);a.post.x='999';a.descriptors[0].fields.a='999';assert.equal(c.evaluate(r).post.x,'11');assert.equal(c.evaluate(r).descriptors[0].fields.a,'7');});
check('canonical whitespace',()=>reject(createExpressionContractV1(J(base())).evaluate(' '+J(req(B(true)))),'INPUT_SCHEMA'));
check('duplicate wire key',()=>{let r=J(req(B(true)));r=r.replace('"source":""','"source":"","source":""');reject(createExpressionContractV1(J(base())).evaluate(r),'INPUT_SCHEMA');});
check('surrogate wire',()=>reject(createExpressionContractV1(J(base())).evaluate(J(req(N('LitText',{value:'\ud800'})))),'INPUT_SCHEMA'));
check('129 descriptors bound',()=>reject(ev(act(Array.from({length:129},()=>N('Emit',{operation:'O',fields:record}))),{workInitial:'65536'}),'DESCRIPTOR_BOUND',516,[128]));
check('128 descriptors success',()=>assert.equal(ev(act(Array.from({length:128},()=>N('Emit',{operation:'O',fields:record}))),{workInitial:'65536'}).descriptors.length,128));
const findings=[];
for(const depth of [508,509,600]){let type=['Bool'];for(let i=0;i<depth;i++)type=['Option',type];const core=N('ConstructNone',{elementType:type});const o=ev(core,{workInitial:'1'});findings.push({case:'nested annotation '+depth,coreBytes:Buffer.byteLength(J(core)),valueWBytes:Buffer.byteLength(J({type:['Option',type],value:[]})),expected:'ExpressionValue',actual:o.status??o.judgmentResult,code:o.code});}
console.log(JSON.stringify({independentAssertionsPassed:checks,constructorCoverage:40,findings},null,2));
let seed=0x79a1983;const rand=n=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed%n;};
const tree=(d)=>d===0||rand(4)===0?B(rand(2)===0):rand(3)===0?N('Not',{value:tree(d-1)}):Bin(rand(2)?'And':'Or',tree(d-1),tree(d-1));
function ref(node,work,path=[]){if(work===0)return {error:path,remaining:0};work--;if(node.constructor==='LitBool')return{value:node.operands.value,remaining:work};if(node.constructor==='Not'){const a=ref(node.operands.value,work,[...path,0]);return a.error?a:{value:!a.value,remaining:a.remaining};}const a=ref(node.operands.left,work,[...path,0]);if(a.error)return a;if((node.constructor==='And'&&!a.value)||(node.constructor==='Or'&&a.value))return a;return ref(node.operands.right,a.remaining,[...path,1]);}
let differential=0;
for(let i=0;i<250;i++){const core=tree(6);for(let work=0;work<35;work++){const expected=ref(core,work),actual=ev(core,{workInitial:String(work)});if(expected.error)reject(actual,'WORK_EXHAUSTED',work,expected.error);else value(actual,expected.value,expected.remaining);differential++;}}
for(const [tag,min,max] of [['UInt64',0n,(1n<<64n)-1n],['UInt128',0n,(1n<<128n)-1n],['SInt128',-(1n<<127n),(1n<<127n)-1n]]){
 const lit=v=>tag==='SInt128'?I(v):N('LitUInt',{width:tag.slice(4),value:String(v)});
 for(const [k,a,b] of [['Add',max,1n],['Sub',min,1n],['Mul',max,2n]]){reject(ev(Bin(k,lit(a),lit(b))),'ARITH_RANGE',3);checks++;}
 for(const v of [min,max]){value(ev(lit(v)),String(v));checks++;}
 for(const v of [min-1n,max+1n]){reject(ev(lit(v)),'TYPE_LITERAL');checks++;}
}
console.log(JSON.stringify({finalIndependentAssertionsPassed:checks,booleanReferenceDifferentialCases:differential,total:checks+differential}));
