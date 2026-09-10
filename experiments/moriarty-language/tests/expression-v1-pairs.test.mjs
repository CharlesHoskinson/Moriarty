import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import {N,U,I,B,Bin,Act,Req,Ens,Let,Write,Pre,Arg,Local,inputs,stateX,run} from './expression-v1-support.mjs';

const spec=JSON.parse(readFileSync(new URL('../spec/successor/expression-cases.json',import.meta.url)));
const cases=new Map(spec.cases.map(c=>[c.id,c]));
const pairs=[];
function pair(name,positive,negative,value,type,p=inputs()) { pairs.push({name,positive,negative,value,type,p}); }
const amount=(asset,value)=>N('LitAmount',{asset,value:String(value)});
const quantity=(units,scale,mantissa)=>N('LitQuantity',{units,scale:String(scale),mantissa:String(mantissa)});
pair('LitUInt',U(7,64),U(2n**64n,64),'7',['UInt64']);
pair('LitSInt',I(-5),I(2n**127n),'-5',['SInt128']);
pair('LitBool',B(true),B(1),true,['Bool']);
pair('LitText',N('LitText',{value:'é'}),N('LitText',{value:'x'.repeat(1025)}),'é',['Text']);
pair('LitAmount',amount('A',30),amount('A',-1),'30',['Amount','A']);
pair('LitQuantity',quantity([['USD','1']],2,-123),quantity([['USD','1'],['USD','1']],2,-123),'-123',['Quantity',[['USD','1']],'2']);
pair('LitShares',N('LitShares',{vault:'V',holder:'H',value:'4'}),N('LitShares',{vault:'V',holder:'H',value:'-1'}),'4',['Shares','V','H']);
pair('LitRate',N('LitRate',{scale:'2',mantissa:'-5'}),N('LitRate',{scale:'19',mantissa:'5'}),'-5',['Rate','2']);
pair('LitPrice',N('LitPrice',{base:'B',quote:'A',scale:'4',mantissa:'19743'}),N('LitPrice',{base:'A',quote:'A',scale:'4',mantissa:'19743'}),'19743',['Price','B','A','4']);
{
 const p=inputs();p.schema.args.q=['UInt128'];p.Args.q='7';pair('ReadArg',Arg('q'),Arg('z'),'7',['UInt128'],p);
 const o=inputs();o.schema.observations.now=['UInt64'];o.Obs.now='100';pair('ReadObs',N('ReadObs',{name:'now'}),N('ReadObs',{name:'other'}),'100',['UInt64'],o);
}
for(const kind of ['ProjectField','AccessField']) {
 const p=inputs();p.schema.recordTypes.R={x:['UInt128']};p.schema.args.r=['Record','R'];p.Args.r={x:'9'};
 pair(kind,N(kind,{record:Arg('r'),field:'x'}),N(kind,{record:Arg('r'),field:'y'}),'9',['UInt128'],p);
}
for(const kind of ['ProjectIndex','AccessIndex']) {
 const p=inputs();p.schema.args.xs=['Collection',['UInt128'],'3'];p.Args.xs=['4','9'];
 pair(kind,N(kind,{collection:Arg('xs'),index:U(1,64)}),N(kind,{collection:Arg('xs'),index:U(2,64)}),'9',['UInt128'],p);
}
{
 const p=inputs();p.schema.recordTypes.R={x:['UInt128'],y:['Bool']};
 pair('ConstructRecord',N('ConstructRecord',{recordType:'R',fields:[{name:'y',value:B(true)},{name:'x',value:U(9)}]}),N('ConstructRecord',{recordType:'R',fields:[{name:'x',value:U(9)},{name:'x',value:U(9)}]}),{x:'9',y:true},['Record','R'],p);
 const e=inputs();e.schema.enumTypes.E=['Closed','Open'];pair('ConstructEnum',N('ConstructEnum',{enumType:'E',member:'Open'}),N('ConstructEnum',{enumType:'E',member:'Other'}),'Open',['Enum','E'],e);
}
pair('ConstructSome',N('ConstructSome',{elementType:['UInt128'],value:U(4)}),N('ConstructSome',{elementType:['UInt128'],value:B(true)}),['4'],['Option',['UInt128']]);
pair('ConstructNone',N('ConstructNone',{elementType:['UInt128']}),N('ConstructNone',{elementType:['Record','AbsentType']}),[],['Option',['UInt128']]);
pair('ConstructCollection',N('ConstructCollection',{elementType:['UInt128'],capacity:'2',items:[U(4),U(9)]}),N('ConstructCollection',{elementType:['UInt128'],capacity:'1',items:[U(4),U(9)]}),['4','9'],['Collection',['UInt128'],'2']);
pair('Add',Bin('Add',amount('A',10),amount('A',3)),Bin('Add',amount('A',10),amount('B',3)),'13',['Amount','A']);
pair('Sub',Bin('Sub',I(2),I(5)),Bin('Sub',U(0),U(1)),'-3',['SInt128']);
pair('Mul',Bin('Mul',U(7,64),U(6,64)),Bin('Mul',U(2n**64n-1n,64),U(2,64)),'42',['UInt64']);
pair('FloorDiv',Bin('FloorDiv',I(-5),I(2)),Bin('FloorDiv',I(-5),I(0)),'-3',['SInt128']);
pair('CeilDiv',Bin('CeilDiv',I(-5),I(2)),Bin('CeilDiv',I(-5),I(-2)),'-2',['SInt128']);
for(const [kind,value] of [['Eq',false],['Lt',true],['Lte',true],['Gt',false],['Gte',false]]) pair(kind,Bin(kind,U(2),U(3)),Bin(kind,U(2,64),U(3)),value,['Bool']);
pair('Not',N('Not',{value:B(false)}),N('Not',{value:U(0)}),true,['Bool']);
pair('And',Bin('And',B(false),B(true)),Bin('And',B(false),U(0)),false,['Bool']);
pair('Or',Bin('Or',B(true),B(false)),Bin('Or',B(true),U(0)),true,['Bool']);

for(const row of pairs) {
 const positiveId=['And','Or'].includes(row.name)?row.name+'-short-circuit-positive-v1':row.name+'-positive';
 test(positiveId+' executes the specified value/type/work',()=>{
  const expected=cases.get(positiveId).expected;
  assert.deepEqual(run(row.positive,row.p),{judgmentResult:'ExpressionValue',type:row.type,value:row.value,workRemaining:expected.workRemaining});
 });
 test(row.name+'-reject executes the specified phase and error',()=>{
  const expected=cases.get(row.name+'-reject').expected,actual=run(row.negative,row.p);
  assert.equal(actual.status,'Rejected');assert.equal(actual.code,expected.code);assert.equal(actual.workUsed,String(expected.workUsed));
  assert.equal('post' in actual,false);assert.equal('descriptors' in actual,false);
 });
}

// The remaining seven constructors' source cases assume existing frames. Their
// test inputs establish those frames through real statements, with explicit
// prefix/wrapper work; no API accepts forged locals or staged writes.
const contextual=new Set(['ReadLocal','ReadPre','Require','Let','NextWrite','Ensure','Emit']);
test('all forty positive/rejection pairs have an executable materialization',()=>{
 assert.deepEqual(new Set([...pairs.map(p=>p.name),...contextual]),new Set(spec.cases.map(c=>c.constructor)));
});
test('ReadLocal-positive: preceding Let supplies immutable y=11',()=>{
 const actual=run(Act(Let('y',U(11)),Ens(Bin('Eq',Local('y'),U(11)))));
 assert.equal(actual.status,'ExpressionPrepared');assert.equal(actual.workRemaining,'94');
});
test('ReadLocal-reject: missing z after a valid earlier binder rejects statically',()=>{
 const actual=run(Act(Let('y',U(11)),Ens(Bin('Eq',Local('z'),U(11)))));
 assert.equal(actual.code,'TYPE_NAME');assert.equal(actual.workUsed,'0');assert.deepEqual(actual.nodePath,['1','0','0']);
});
test('ReadPre-positive: post read sees staged x11 only inside Ensure',()=>{
 const actual=run(Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','post'),U(11)))),stateX());
 assert.deepEqual(actual.post,{x:'11'});assert.equal(actual.workRemaining,'94');
});
test('ReadPre-reject: next view remains statically forbidden',()=>{
 const actual=run(Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','next'),U(11)))),stateX());
 assert.equal(actual.code,'TYPE_NEXT_READ');assert.equal(actual.workUsed,'0');
});
test('Require-positive: true consumes two nodes and changes no frame',()=>{
 const actual=run(Act(Req(B(true))));assert.equal(actual.status,'ExpressionPrepared');assert.equal(actual.workRemaining,'98');assert.deepEqual(actual.post,{});
});
test('Require-reject: false consumes two nodes then rejects',()=>{
 const actual=run(Act(Req(B(false))));assert.equal(actual.code,'GUARD_FAILED');assert.equal(actual.workUsed,'2');
});
test('Let-positive: binding is evaluated against pre then observed by Ensure',()=>{
 const actual=run(Act(Let('y',Bin('Add',Pre('x'),U(1))),Ens(Bin('Eq',Local('y'),U(11)))),stateX());
 assert.equal(actual.status,'ExpressionPrepared');assert.equal(actual.workRemaining,'92');assert.deepEqual(actual.post,{x:'10'});
});
test('Let-reject: argument collision prevents reduction',()=>{
 const p=stateX();p.schema.args.y=['UInt128'];p.Args.y='0';const actual=run(Act(Let('y',U(3))),p);
 assert.equal(actual.code,'TYPE_DUPLICATE_BINDER');assert.equal(actual.workUsed,'0');
});
test('NextWrite-positive: prefix Let2 then NextWrite2 stages x11',()=>{
 const actual=run(Act(Let('y',U(11)),Write('x',Local('y'))),stateX());assert.deepEqual(actual.post,{x:'11'});assert.equal(actual.workRemaining,'96');
});
test('NextWrite-reject: complete action typing rejects Bool before prefix work',()=>{
 const actual=run(Act(Let('y',U(11)),Write('x',B(true))),stateX());assert.equal(actual.code,'TYPE_MISMATCH');assert.equal(actual.workUsed,'0');
});
test('Ensure-positive: prefix NextWrite2 then Ensure4 checks x11',()=>{
 const actual=run(Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','post'),U(11)))),stateX());assert.equal(actual.status,'ExpressionPrepared');assert.equal(actual.workRemaining,'94');
});
test('Ensure-reject: work includes staging but no staged state escapes',()=>{
 const actual=run(Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','post'),U(12)))),stateX());assert.equal(actual.code,'ENSURES_FAILED');assert.equal(actual.workUsed,'6');assert.equal('post' in actual,false);
});
function operationInputs(){const p=inputs();p.schema.recordTypes.R={amount:['UInt128']};p.schema.operations.O='R';p.schema.args.fields=['Record','R'];p.Args.fields={amount:'3'};return p;}
test('Emit-positive: append a descriptor, with no financial effects',()=>{
 const actual=run(Act(N('Emit',{operation:'O',fields:Arg('fields')})),operationInputs());
 assert.deepEqual(actual,{status:'ExpressionPrepared',post:{},descriptors:[{operation:'O',fields:{amount:'3'}}],workRemaining:'98'});
});
test('Emit-reject: unknown operation rejects before reduction',()=>{
 const actual=run(Act(N('Emit',{operation:'Unknown',fields:Arg('fields')})),operationInputs());assert.equal(actual.code,'TYPE_NAME');assert.equal(actual.workUsed,'0');
});
