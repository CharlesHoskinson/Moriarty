import test from 'node:test';
import assert from 'node:assert/strict';
import {createFinancialExpressionContractV1} from '../src/successor/financial-expression-v1.ts';
import {N,U,B,Bin,J,inputs} from './expression-v1-support.mjs';
const p=()=>{const x=inputs();x.schema.variantTypes={};return x;};
function run(core,x=p(),work=100){return createFinancialExpressionContractV1(J(x.schema)).evaluate(J({contract:'moriarty-financial-expression-contract/1',source:'',core,Pre:x.Pre,Args:x.Args,Obs:x.Obs,workInitial:String(work)}));}
test('explicit UInt256 conversion evaluates a full-domain UInt128 product',()=>{
 const max=(2n**128n-1n).toString();const wide=v=>N('ConvertUInt',{width:'256',value:U(v)});
 const result=run(N('ConvertUInt',{width:'128',value:Bin('FloorDiv',Bin('Mul',wide(max),wide(max)),wide(max))}));
 assert.equal(result.judgmentResult,'ExpressionValue');assert.equal(result.value,max);assert.deepEqual(result.type,['UInt128']);assert.equal(result.workRemaining,'91');
});
test('Select uses actual selected-node work while checking both branches',()=>{
 const r=run(N('Select',{condition:B(true),consequent:U(7),alternative:Bin('FloorDiv',U(1),U(0))}));
 assert.equal(r.value,'7');assert.equal(r.workRemaining,'97');
});
const amount=(asset,v)=>N('LitAmount',{asset,value:String(v)});
const convert=(width,value)=>N('ConvertUInt',{width:String(width),value});
const scalar=(component,value)=>N('ScalarValue',{component,value});
const arg=name=>N('ReadArg',{name});
const expectValue=(core,value,type,x=p(),work=100)=>{const r=run(core,x,work);assert.equal(r.judgmentResult,'ExpressionValue',JSON.stringify(r));assert.deepEqual(r.type,type);assert.deepEqual(r.value,value);return r;};
const expectFailure=(core,code,x=p(),work=100)=>{const r=run(core,x,work);assert.equal(r.status,'Rejected',JSON.stringify(r));assert.equal(r.code,code);assert.equal('post' in r,false);return r;};

test('ConstructAmount and ConstructShares require exact UInt128 and fixed identities',()=>{
 const max=(2n**128n-1n).toString();expectValue(N('ConstructAmount',{asset:'A',value:U(max)}),max,['Amount','A']);
 expectValue(N('ConstructShares',{vault:'V',holder:'H',value:U(5)}),'5',['Shares','V','H']);
 expectFailure(N('ConstructAmount',{asset:'Unknown',value:U(1)}),'TYPE_NAME');
 expectFailure(N('ConstructShares',{vault:'V',holder:'Other',value:U(1)}),'TYPE_NAME');
 expectFailure(N('ConstructAmount',{asset:'A',value:U(1,256)}),'TYPE_MISMATCH');
 expectFailure(N('ConstructShares',{vault:'V',holder:'H',value:amount('A',1)}),'TYPE_MISMATCH');
});
test('Variant introduction and exact projection retain payload asset and node work',()=>{
 const x=p();x.schema.variantTypes.F={First:['Amount','A'],Second:['Amount','B']};
 const v=N('ConstructVariant',{family:'F',tag:'First',value:amount('A',7)});
 expectValue(v,{tag:'First',value:'7'},['Variant','F'],x);
 assert.equal(expectValue(N('ProjectVariant',{tag:'First',value:v}),'7',['Amount','A'],x).workRemaining,'97');
 const failure=expectFailure(N('ProjectVariant',{tag:'Second',value:v}),'VARIANT_CASE',x);assert.equal(failure.workUsed,'3');
 expectFailure(N('ConstructVariant',{family:'F',tag:'Second',value:amount('A',7)}),'TYPE_MISMATCH',x);
 expectFailure(N('ProjectVariant',{tag:'Absent',value:v}),'TYPE_NAME',x);
 expectFailure(convert(256,v),'TYPE_MISMATCH',x);
});
test('Variant values and schemas reject wrong cases, duplicate payloads and cycles',()=>{
 const x=p();x.schema.variantTypes.F={First:['Amount','A']};x.schema.args.v=['Variant','F'];x.Args.v={tag:'Absent',value:'2'};
 expectFailure(arg('v'),'INPUT_SCHEMA',x);
 const duplicate=p();duplicate.schema.variantTypes.F={First:['UInt128'],Second:['UInt128']};expectFailure(U(1),'INPUT_SCHEMA',duplicate);
 const cyclic=p();cyclic.schema.variantTypes.F={Loop:['Record','R']};cyclic.schema.recordTypes.R={v:['Variant','F']};expectFailure(U(1),'TYPE_SCHEMA_CYCLE',cyclic);
});
test('ProjectSome distinguishes None and preserves child failure and work',()=>{
 const some=N('ConstructSome',{elementType:['Bool'],value:B(false)});expectValue(N('ProjectSome',{value:some}),false,['Bool']);
 assert.equal(expectFailure(N('ProjectSome',{value:N('ConstructNone',{elementType:['UInt128']})}),'OPTION_NONE').workUsed,'2');
 expectFailure(N('ProjectSome',{value:U(1)}),'TYPE_MISMATCH');
 const r=expectFailure(N('ProjectSome',{value:some}),'WORK_EXHAUSTED',p(),1);assert.deepEqual(r.nodePath,['0']);assert.equal(r.workUsed,'1');
});
test('UInt256 boundaries and explicit narrowing have distinct static and dynamic errors',()=>{
 const max=2n**256n-1n;expectValue(U(max,256),String(max),['UInt256']);expectFailure(U(max+1n,256),'TYPE_LITERAL');
 expectValue(convert(64,U(2n**64n-1n,256)),String(2n**64n-1n),['UInt64']);
 assert.equal(expectFailure(convert(64,U(2n**64n,256)),'ARITH_RANGE').workUsed,'2');
 expectFailure(convert(32,U(1)),'TYPE_LITERAL');expectFailure(convert(256,amount('A',1)),'TYPE_MISMATCH');
 expectFailure(Bin('Add',U(1,128),U(1,256)),'TYPE_MISMATCH');expectFailure(Bin('Mul',U(max,256),U(2,256)),'ARITH_RANGE');
});
test('ScalarValue has only the approved component/type overloads',()=>{
 expectValue(scalar('quanta',amount('A',9)),'9',['UInt128']);
 expectValue(scalar('mantissa',N('LitRate',{scale:'2',mantissa:'-5'})),'-5',['SInt128']);
 expectValue(scalar('negative',N('LitRate',{scale:'2',mantissa:'-5'})),true,['Bool']);
 expectValue(scalar('magnitude',N('LitRate',{scale:'0',mantissa:String(-(2n**127n))})),String(2n**127n),['UInt128']);
 expectFailure(scalar('quanta',U(9)),'TYPE_MISMATCH');expectFailure(scalar('unknown',U(9)),'TYPE_LITERAL');
 const x=p();x.schema.args.s=['SignedAmount','A'];x.Args.s=String(-(2n**255n));expectValue(scalar('magnitude',arg('s')),String(2n**255n),['UInt256'],x);
});
test('NetAmount exact symmetric domain does not narrow to signed128',()=>{
 const x=p();x.schema.args.net=['NetAmount','A'];
 for(const value of [2n**128n-1n,-(2n**128n-1n)]){x.Args.net=String(value);expectValue(arg('net'),String(value),['NetAmount','A'],x);expectValue(scalar('magnitude',arg('net')),String(2n**128n-1n),['UInt128'],x);}
 x.Args.net=String(2n**128n);expectFailure(arg('net'),'INPUT_VALUE',x);
});
test('Amount products preserve dimensions and allow a full UInt128-square quotient',()=>{
 const max=2n**128n-1n;const product=Bin('Mul',amount('B',max),amount('A',max));
 expectValue(product,String(max*max),['AmountProduct','A','B']);
 expectValue(Bin('FloorDiv',product,amount('A',max)),String(max),['Amount','B']);
 expectValue(Bin('FloorDiv',Bin('Mul',amount('A',4),amount('A',5)),amount('A',2)),'10',['Amount','A']);
 expectFailure(Bin('Mul',amount('A',max),U(2)),'ARITH_RANGE');
 expectFailure(Bin('Mul',amount('A',3),U(2,64)),'TYPE_MISMATCH');
 const x=p();x.schema.assets.push('C');expectFailure(Bin('FloorDiv',product,amount('C',1)),'TYPE_MISMATCH',x);
});
test('AMM expression retains floor19743 and ceil19744 without a financial effect',()=>{
 const adjusted=Bin('Mul',amount('A',10000),U(997));const numerator=Bin('Mul',adjusted,amount('B',2000000));const denominator=Bin('Add',Bin('Mul',amount('A',1000000),U(1000)),adjusted);
 expectValue(Bin('FloorDiv',numerator,denominator),'19743',['Amount','B']);expectValue(Bin('CeilDiv',numerator,denominator),'19744',['Amount','B']);
});
test('Price direction and literal scale divisor are statically bound',()=>{
 const price=N('LitPrice',{base:'B',quote:'A',scale:'4',mantissa:'19743'});const product=Bin('Mul',amount('A',10000),price);
 expectValue(product,'197430000',['ScaledAmount','B','4']);expectValue(Bin('FloorDiv',product,U(10000)),'19743',['Amount','B']);
 expectFailure(Bin('Mul',amount('B',10000),price),'TYPE_MISMATCH');expectFailure(Bin('FloorDiv',product,U(1000)),'TYPE_SCALE_DIVISOR');
 const x=p();x.schema.args.divisor=['UInt128'];x.Args.divisor='10000';expectFailure(Bin('FloorDiv',product,arg('divisor')),'TYPE_SCALE_DIVISOR',x);
});
test('negative rates use signed256 and Euclidean rounding, not unsigned cash',()=>{
 const product=Bin('Mul',amount('A',5),N('LitRate',{scale:'1',mantissa:'-5'}));
 expectValue(product,'-25',['SignedScaledAmount','A','1']);expectValue(Bin('FloorDiv',product,U(10)),'-3',['SignedAmount','A']);expectValue(Bin('CeilDiv',product,U(10)),'-2',['SignedAmount','A']);
 expectFailure(convert(128,Bin('CeilDiv',product,U(10))),'TYPE_MISMATCH');
});
test('Select preserves skipped dynamic errors, but rejects skipped static errors and unit mismatch',()=>{
 const none=N('ProjectSome',{value:N('ConstructNone',{elementType:['UInt128']})});
 expectValue(N('Select',{condition:B(true),consequent:U(7),alternative:none}),'7',['UInt128']);
 assert.equal(expectFailure(N('Select',{condition:B(false),consequent:U(7),alternative:none}),'OPTION_NONE').workUsed,'4');
 assert.equal(expectFailure(N('Select',{condition:B(true),consequent:U(7),alternative:arg('missing')}),'TYPE_NAME').workUsed,'0');
 expectFailure(N('Select',{condition:B(true),consequent:amount('A',1),alternative:amount('B',1)}),'TYPE_MISMATCH');
 const r=expectFailure(N('Select',{condition:B(true),consequent:U(7),alternative:U(9)}),'WORK_EXHAUSTED',p(),2);assert.deepEqual(r.nodePath,['1']);
});
test('new failures retain actual parent or child source spans',()=>{
 const core=convert(64,U(2n**64n,256));core.span={kind:'source',start:'0',end:'4'};core.operands.value.span={kind:'source',start:'1',end:'3'};
 const x=p();const request=work=>J({contract:'moriarty-financial-expression-contract/1',source:'abcd',core,Pre:x.Pre,Args:x.Args,Obs:x.Obs,workInitial:String(work)});
 const api=createFinancialExpressionContractV1(J(x.schema));let r=api.evaluate(request(10));assert.equal(r.code,'ARITH_RANGE');assert.deepEqual(r.span,core.span);r=api.evaluate(request(1));assert.equal(r.code,'WORK_EXHAUSTED');assert.deepEqual(r.span,core.operands.value.span);
});
test('new schema validates every named type and simultaneous global bounds',()=>{
 const names=p();names.schema.recordTypes=Object.fromEntries(Array.from({length:257},(_,i)=>['R'+i,{}]));expectFailure(U(1),'INPUT_BOUND',names);
 const unknown=p();unknown.schema.variantTypes.F={Bad:['Record','Missing']};expectFailure(U(1),'TYPE_NAME',unknown);
 const excessive=p();excessive.schema.args=Object.fromEntries(Array.from({length:2100},(_,i)=>['Arg'+i,['UInt128']]));expectFailure(U(1),'INPUT_BOUND',excessive);
});
