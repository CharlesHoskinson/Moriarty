import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {N,U,B,Bin,Act,Req,Ens,Let,Write,Pre,Arg,Local,inputs,stateX,run,J} from './expression-v1-support.mjs';

const specified=JSON.parse(readFileSync(new URL('../spec/successor/expression-cases.json',import.meta.url))).combinedCases;
const covered=new Set();
function example(id,fn){covered.add(id);test(id+': actual execution',fn);}
function rejection(actual,code,work){assert.equal(actual.status,'Rejected');assert.equal(actual.code,code);assert.equal(actual.workUsed,String(work));assert.equal('post' in actual,false);assert.equal('descriptors' in actual,false);}
const staged=end=>Act(Let('y',Bin('Add',Pre('x'),U(1))),Write('x',Local('y')),Ens(Bin('Eq',Pre('x','post'),U(end))));
example('staged-x10-to11',()=>assert.deepEqual(run(staged(11),stateX()),{status:'ExpressionPrepared',post:{x:'11'},descriptors:[],workRemaining:'90'}));
example('staged-ensures12',()=>rejection(run(staged(12),stateX()),'ENSURES_FAILED',10));
example('duplicate-write',()=>rejection(run(Act(Write('x',U(11)),Write('x',U(12))),stateX()),'TYPE_DUPLICATE_WRITE',0));
example('next-read',()=>rejection(run(Act(Write('x',Pre('x','next'))),stateX()),'TYPE_NEXT_READ',0));
example('post-outside-ensure',()=>rejection(run(Act(Let('y',Pre('x','post'))),stateX()),'TYPE_POST_SCOPE',0));
example('two-runtime-failures',()=>{const r=run(Bin('Add',Bin('FloorDiv',U(1),U(0)),Bin('Sub',U(0),U(1))));rejection(r,'ARITH_DENOMINATOR',4);assert.deepEqual(r.nodePath,['0']);});
example('static-before-guard',()=>rejection(run(Act(Req(B(false)),Write('x',B(true))),stateX()),'TYPE_MISMATCH',0));
example('work-exhaustion',()=>{const r=run(Bin('Add',U(2),U(3)),inputs(),2);rejection(r,'WORK_EXHAUSTED',2);assert.deepEqual(r.nodePath,['1']);});
const q=(unit,power,scale,mantissa)=>N('LitQuantity',{units:[[unit,String(power)]],scale:String(scale),mantissa:String(mantissa)});
example('quantity-mul',()=>assert.deepEqual(run(Bin('Mul',q('m',1,1,15),q('s',-1,1,20))),{judgmentResult:'ExpressionValue',type:['Quantity',[['m','1'],['s','-1']],'2'],value:'300',workRemaining:'97'}));
example('quantity-div',()=>assert.deepEqual(run(Bin('FloorDiv',q('m',1,2,123),q('s',1,1,20))),{judgmentResult:'ExpressionValue',type:['Quantity',[['m','1'],['s','-1']],'1'],value:'6',workRemaining:'97'}));
example('quantity-negative-scale',()=>rejection(run(Bin('FloorDiv',q('m',1,0,3),q('s',1,1,20))),'TYPE_QUANTITY_DOMAIN',0));
example('financial-write',()=>{const p=inputs();p.schema.fields.debt={type:['UInt128'],writeClass:'financial'};p.Pre.debt='9';p.schema.args.zeroDebt=['UInt128'];p.Args.zeroDebt='0';rejection(run(Act(Write('debt',Arg('zeroDebt'))),p),'TYPE_FINANCIAL_WRITE',0);});
function matrix(value=String(2n**64n-1n)){return Array.from({length:128},()=>Array.from({length:16},()=>value));}
const matrixType=['Collection',['Collection',['UInt64'],'16'],'128'];
example('finite-record-bound',()=>{const p=inputs();p.schema.recordTypes.R={a:matrixType,b:matrixType};p.schema.args.r=matrixType;p.Args.r=matrix('0');assert.equal(Buffer.byteLength(J(p.Args)),8455);const r=run(N('ConstructRecord',{recordType:'R',fields:[{name:'a',value:Arg('r')},{name:'b',value:Arg('r')}]}),p);rejection(r,'VALUE_BOUND',3);});
example('unit-not-a-value',()=>rejection(run(N('ConstructNone',{elementType:['Unit']})),'TYPE_NAME',0));
example('max-u64-matrix-encoding',()=>{const p=inputs();p.schema.args.r=matrixType;p.Args.r=matrix();assert.equal(Buffer.byteLength(J(p.Args)),47367);assert.equal(Buffer.byteLength(J({type:matrixType,value:p.Args.r})),47430);assert.equal(run(Act(),p).status,'ExpressionPrepared');});
function byteBoundary(last){const p=inputs();p.schema.args.a=['Collection',['Text'],'128'];p.schema.args.b=['Collection',['Text'],'128'];p.Args.a=Array(32).fill('x'.repeat(1024));p.Args.b=[...Array(31).fill('x'.repeat(1024)),'x'.repeat(last)];return p;}
example('byte-boundary-65536',()=>{const p=byteBoundary(819);assert.equal(Buffer.byteLength(J(p.Args)),65536);assert.equal(run(Act(),p).status,'ExpressionPrepared');});
example('byte-boundary-65537',()=>{const p=byteBoundary(820);assert.equal(Buffer.byteLength(J(p.Args)),65537);rejection(run(Act(),p),'INPUT_BOUND',0);});
example('undeclared-unit',()=>rejection(run(q('Ghost',1,0,1)),'TYPE_NAME',0));
example('recursive-operation-schema',()=>{const p=inputs();p.schema.recordTypes.R={child:['Option',['Operation','O']]};p.schema.operations.O='R';rejection(run(Act(),p),'TYPE_SCHEMA_CYCLE',0);});
test('all active combined derivations have executed counterparts',()=>assert.deepEqual(covered,new Set(specified.map(c=>c.id))));
test('the historical twentieth strict case has a new explicit short-circuit observation',()=>{
 const bad=Bin('Eq',Bin('FloorDiv',U(1),U(0)),U(0));
 assert.deepEqual(run(Bin('And',B(false),bad)),{judgmentResult:'ExpressionValue',type:['Bool'],value:false,workRemaining:'98'});
});
