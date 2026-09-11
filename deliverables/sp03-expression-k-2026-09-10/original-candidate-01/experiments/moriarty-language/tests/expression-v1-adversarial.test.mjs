import assert from 'node:assert/strict';
import test from 'node:test';
import {createExpressionContractV1} from '../src/successor/expression-v1.ts';
import {P,J,N,U,B,Bin,Act,Req,Ens,Let,Write,Pre,Arg,inputs,stateX,run} from './expression-v1-support.mjs';

const request=(core=B(true),p=inputs())=>({contract:'moriarty-expression-contract/1',source:'',core,Pre:p.Pre,Args:p.Args,Obs:p.Obs,workInitial:'100'});
function raw(text,p=inputs()){return createExpressionContractV1(J(p.schema)).evaluate(text);}
function rejected(result,code){assert.equal(result.status,'Rejected');assert.equal(result.code,code);assert.equal(result.workUsed,'0');assert.equal('post' in result,false);}
test('JSON duplicate keys reject even when their final values agree',()=>{const text=J(request());rejected(raw(text.replace('"workInitial":"100"','"workInitial":"100","workInitial":"100"')),'INPUT_SCHEMA');});
test('noncanonical whitespace and key order reject',()=>{rejected(raw(' '+J(request())),'INPUT_SCHEMA');rejected(raw(JSON.stringify(request())),'INPUT_SCHEMA');});
test('JSON numbers, null and unpaired surrogates reject',()=>{for(const v of [1,null,'\ud800']) rejected(raw(J(request(N('LitText',{value:v})))),'INPUT_SCHEMA');});
test('integer metadata rejects negative zero, leading zeros, plus and terminal line separators',()=>{for(const v of ['-0','00','+1','1\r','1\n','1\u2028','1\u2029']) rejected(run(N('LitUInt',{width:'128',value:v})),'INPUT_SCHEMA');});
test('identifiers reject final carriage returns and Unicode line separators',()=>{for(const v of ['x\r','x\u2028','x\u2029']) rejected(run(Arg(v)),'INPUT_SCHEMA');});
test('unknown type tags and undeclared well-shaped names have different phases',()=>{rejected(run(N('ConstructNone',{elementType:['FutureUInt256']})),'INPUT_SCHEMA');rejected(run(N('ConstructNone',{elementType:['Record','Missing']})),'TYPE_NAME');});
test('schema definition traversal follows ASCII identifier order',()=>{const p=inputs();p.schema.recordTypes.A={x:['Record','Missing']};p.schema.recordTypes.a={x:['Rate','19']};rejected(run(Act(),p),'TYPE_NAME');});
test('no supplied schema or financial write-class override is admitted in a request',()=>{const p=stateX();p.schema.fields.x.writeClass='financial';const r=request(Act(Write('x',U(0))),p);r.schema={};rejected(raw(J(r),p),'INPUT_SCHEMA');rejected(run(Act(Write('x',U(0))),p),'TYPE_FINANCIAL_WRITE');});
test('reject before touching a live object or accessor request',()=>{let read=false;const evil={get source(){read=true;throw Error('accessed');}};rejected(raw(evil),'INPUT_SCHEMA');assert.equal(read,false);});
test('a stale registered funded version cannot opt into expression semantics',()=>{const r=request();r.contract='moriarty-funded-source/0';rejected(raw(J(r)),'INPUT_SCHEMA');});
test('canonical minimal escapes are required',()=>{const text=J(request(B(true)));rejected(raw(text.replace('LitBool','Lit\\u0042ool')),'INPUT_SCHEMA');});
test('later missing snapshot values precede earlier runtime guards',()=>{const p=inputs();p.schema.args.x=['Bool'];rejected(run(Act(Req(B(false))),p),'INPUT_SCHEMA');});
test('snapshot shape errors precede bounds/domain errors within the same snapshot',()=>{const p=inputs();p.schema.args.a=['UInt128'];p.schema.args.z=['Bool'];p.Args={a:String(2n**128n),z:'not bool'};rejected(run(Act(),p),'INPUT_SCHEMA');});
test('all static action errors precede missing snapshot admission',()=>{const p=stateX();delete p.Pre.x;rejected(run(Act(Req(B(false)),Write('x',B(true))),p),'TYPE_MISMATCH');});
test('supply checked nominal snapshots; no structural equality grants a write',()=>{const p=inputs();p.schema.fields.debt={type:['Record','Debt'],writeClass:'financial'};p.schema.recordTypes.Debt={amount:['UInt128']};p.Pre.debt={amount:'9'};const r=run(Act(Ens(Bin('Eq',Pre('debt'),Pre('debt')))),p);assert.equal(r.status,'ExpressionPrepared');assert.deepEqual(r.post,p.Pre);});
test('short-circuiting cannot hide an effectful nested statement',()=>rejected(run(Bin('Or',B(true),Req(B(true)))),'TYPE_STATEMENT_PLACEMENT'));
test('Ensure suffix placement is checked before reduction',()=>rejected(run(Act(Ens(B(true)),Req(B(true)))),'TYPE_STATEMENT_PLACEMENT'));
test('self reference is not introduced before binding',()=>rejected(run(Act(Let('fresh',N('ReadLocal',{name:'fresh'})))),'TYPE_NAME'));
test('descriptor count is an actual runtime bound with rollback',()=>{const p=inputs();p.schema.recordTypes.R={};p.schema.operations.O='R';p.schema.args.f=['Record','R'];p.Args.f={};const emit=()=>N('Emit',{operation:'O',fields:Arg('f')});const ok=run(Act(...Array.from({length:128},emit)),p,1000);assert.equal(ok.descriptors.length,128);const bad=run(Act(...Array.from({length:129},emit)),p,1000);assert.equal(bad.code,'DESCRIPTOR_BOUND');assert.equal(bad.workUsed,'258');assert.equal('descriptors' in bad,false);});
test('result mutation cannot affect subsequent evaluations',()=>{const p=stateX();const evaluator=createExpressionContractV1(J(p.schema));const r=J(request(Act(),p));const a=evaluator.evaluate(r);a.post.x='0';assert.equal(evaluator.evaluate(r).post.x,'10');});
test('source, Core and snapshots retain separate simultaneous byte bounds',()=>{const p=inputs();const r=request(Act(),p);r.source='x'.repeat(65536);assert.equal(raw(J(r),p).status,'ExpressionPrepared');r.source+='x';rejected(raw(J(r),p),'INPUT_BOUND');});
test('deep non-object request is INPUT_SCHEMA without host stack overflow',()=>rejected(raw('['.repeat(10000)+'true'+']'.repeat(10000)),'INPUT_SCHEMA'));

test('statement256 is legal and257 rejects before any reduction',()=>{
 assert.equal(run(Act(...Array.from({length:256},()=>Req(B(true)))),inputs(),1000).status,'ExpressionPrepared');
 rejected(run(Act(...Array.from({length:257},()=>Req(B(true)))),inputs(),1000),'INPUT_BOUND');
});
test('record64 fields is legal and65 rejects schema admission',()=>{
 const p=inputs();p.schema.recordTypes.R=Object.fromEntries(Array.from({length:64},(_,i)=>['f'+i,['Bool']]));
 const fields=Object.keys(p.schema.recordTypes.R).map(name=>({name,value:B(true)}));
 assert.equal(run(N('ConstructRecord',{recordType:'R',fields}),p).judgmentResult,'ExpressionValue');
 p.schema.recordTypes.R.extra=['Bool'];rejected(run(B(true),p),'INPUT_BOUND');
});
test('Text literal1024 versus1025 and supplied Text1025 retain different failure phases',()=>{
 assert.equal(run(N('LitText',{value:'x'.repeat(1024)})).judgmentResult,'ExpressionValue');
 rejected(run(N('LitText',{value:'x'.repeat(1025)})),'TYPE_LITERAL');
 const p=inputs();p.schema.args.text=['Text'];p.Args.text='x'.repeat(1025);rejected(run(B(true),p),'INPUT_VALUE');
});
test('collection capacity128 is legal;129 is not silently widened',()=>{
 assert.equal(run(N('ConstructCollection',{elementType:['Bool'],capacity:'128',items:Array.from({length:128},()=>B(true))}),inputs(),1000).judgmentResult,'ExpressionValue');
 rejected(run(N('ConstructCollection',{elementType:['Bool'],capacity:'129',items:[]})),'TYPE_COLLECTION_BOUND');
});
function recordChain(length){
 const p=inputs();let value={};
 for(let i=length-1;i>=0;i--){p.schema.recordTypes['R'+i]=i===length-1?{}:{child:['Record','R'+(i+1)]};if(i<length-1)value={child:value};}
 p.schema.args.root=['Record','R0'];p.Args.root=value;return p;
}
test('snapshot outer node counts toward value depth64',()=>{
 assert.equal(run(Act(),recordChain(63)).status,'ExpressionPrepared');
 rejected(run(Act(),recordChain(64)),'INPUT_BOUND');
});
test('constructed depth64 succeeds and depth65 rejects after entered children',()=>{
 const p=recordChain(63),inner=N('ConstructSome',{elementType:['Record','R0'],value:Arg('root')});
 assert.equal(run(inner,p).judgmentResult,'ExpressionValue');
 const outer=N('ConstructSome',{elementType:['Option',['Record','R0']],value:inner});
 const r=run(outer,p);assert.equal(r.code,'VALUE_BOUND');assert.equal(r.workUsed,'3');assert.deepEqual(r.nodePath,[]);
});
test('schema JSON-tree node bound is independent of record field bounds',()=>{
 const p=inputs();for(let i=0;i<32;i++)p.schema.recordTypes['R'+i]=Object.fromEntries(Array.from({length:64},(_,j)=>['f'+j,['Bool']]));
 assert.ok(Buffer.byteLength(J(p.schema))<65536);rejected(run(Act(),p),'INPUT_BOUND');
});
test('Quantity normalized scale and exponent bounds reject before work',()=>{
 const p=inputs();const quantity=(units,scale)=>N('LitQuantity',{units,scale:String(scale),mantissa:'1'});
 rejected(run(Bin('Mul',quantity([['m','127']],0),quantity([['m','1']],0)),p),'TYPE_QUANTITY_DOMAIN');
 rejected(run(Bin('Mul',quantity([],18),quantity([],1)),p),'TYPE_QUANTITY_DOMAIN');
 const r=run(Bin('FloorDiv',quantity([['m','1']],1),quantity([['m','1']],1)),p);
 assert.deepEqual(r.type,['Quantity',[],'0']);assert.equal(r.value,'1');
});
test('selected arithmetic overflow is charged while skipped overflow is not entered',()=>{
 const bad=Bin('Eq',Bin('Mul',U(2n**64n-1n,64),U(2,64)),U(0,64));
 assert.equal(run(Bin('Or',B(true),bad)).workRemaining,'98');
 const r=run(Bin('Or',B(false),bad));assert.equal(r.code,'ARITH_RANGE');assert.equal(r.workUsed,'6');assert.deepEqual(r.nodePath,['1','0']);
});
test('empty collection access rejects at the projection after both children',()=>{
 const list=N('ConstructCollection',{elementType:['UInt128'],capacity:'0',items:[]});
 const r=run(N('AccessIndex',{collection:list,index:U(0,64)}));assert.equal(r.code,'INDEX_RANGE');assert.equal(r.workUsed,'3');
});
test('ASCII-valid prototype-like identifiers stay ordinary own properties',()=>{
 const p=inputs();p.schema.args.constructor=['Bool'];p.Args.constructor=true;assert.equal(run(Arg('constructor'),p).value,true);
 rejected(run(Arg('toString')),'TYPE_NAME');
});
test('work65536 is legal and a caller cannot raise or negate that allowance',()=>{
 assert.equal(run(B(true),inputs(),65536).workRemaining,'65535');
 rejected(run(B(true),inputs(),65537),'INPUT_BOUND');rejected(run(B(true),inputs(),-1),'INPUT_BOUND');
});
test('integer endpoints and Unicode Text bytes are checked without wrapping or character counting',()=>{
 assert.equal(run(U(2n**128n-1n)).value,String(2n**128n-1n));
 rejected(run(U(2n**128n)),'TYPE_LITERAL');
 assert.equal(run(N('LitSInt',{value:String(-(2n**127n))})).value,String(-(2n**127n)));
 rejected(run(N('LitSInt',{value:String(-(2n**127n)-1n)})),'TYPE_LITERAL');
 assert.equal(run(N('LitText',{value:'é'.repeat(512)})).value,'é'.repeat(512));
 rejected(run(N('LitText',{value:'é'.repeat(513)})),'TYPE_LITERAL');
});
