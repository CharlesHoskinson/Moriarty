// Materializes retained independent all40 expectations; never calls evaluate/run.
import {readFileSync,writeFileSync} from 'node:fs';
import {N,U,I,B,Bin,Act,Req,Ens,Let,Write,Pre,Arg,Local,inputs,stateX,J,P} from '../../../tests/expression-v1-support.mjs';
const spec=JSON.parse(readFileSync(new URL('../../../spec/successor/expression-cases.json',import.meta.url)));
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

const traces=[];
function add(id,core,p,expected,work='100',source='') {
 traces.push({id,schema:J(p.schema),request:J({contract:'moriarty-expression-contract/1',source,core,Pre:p.Pre,Args:p.Args,Obs:p.Obs,workInitial:work}),expected});
}
const rejected=(code,workUsed='0',nodePath=[],span=P)=>({status:'Rejected',code,span:{...span},nodePath,workUsed});
const prepared=(post,workRemaining,descriptors=[])=>({status:'ExpressionPrepared',post,descriptors,workRemaining});
for(const row of pairs) {
 const positiveId=['And','Or'].includes(row.name)?row.name+'-short-circuit-positive-v1':row.name+'-positive';
 const pe=cases.get(positiveId).expected,ne=cases.get(row.name+'-reject').expected;
 add(positiveId,row.positive,row.p,{judgmentResult:'ExpressionValue',type:row.type,value:row.value,workRemaining:pe.workRemaining});
 add(row.name+'-reject',row.negative,row.p,rejected(ne.code,String(ne.workUsed)));
}
add('ReadLocal-positive',Act(Let('y',U(11)),Ens(Bin('Eq',Local('y'),U(11)))),inputs(),prepared({},'94'));
add('ReadLocal-reject',Act(Let('y',U(11)),Ens(Bin('Eq',Local('z'),U(11)))),inputs(),rejected('TYPE_NAME','0',['1','0','0']));
add('ReadPre-positive',Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','post'),U(11)))),stateX(),prepared({x:'11'},'94'));
add('ReadPre-reject',Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','next'),U(11)))),stateX(),rejected('TYPE_NEXT_READ','0',['1','0','0']));
add('Require-positive',Act(Req(B(true))),inputs(),prepared({},'98'));
add('Require-reject',Act(Req(B(false))),inputs(),rejected('GUARD_FAILED','2',['0']));
add('Let-positive',Act(Let('y',Bin('Add',Pre('x'),U(1))),Ens(Bin('Eq',Local('y'),U(11)))),stateX(),prepared({x:'10'},'92'));
{const p=stateX();p.schema.args.y=['UInt128'];p.Args.y='0';add('Let-reject',Act(Let('y',U(3))),p,rejected('TYPE_DUPLICATE_BINDER','0',['0']));}
add('NextWrite-positive',Act(Let('y',U(11)),Write('x',Local('y'))),stateX(),prepared({x:'11'},'96'));
add('NextWrite-reject',Act(Let('y',U(11)),Write('x',B(true))),stateX(),rejected('TYPE_MISMATCH','0',['1']));
add('Ensure-positive',Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','post'),U(11)))),stateX(),prepared({x:'11'},'94'));
add('Ensure-reject',Act(Write('x',U(11)),Ens(Bin('Eq',Pre('x','post'),U(12)))),stateX(),rejected('ENSURES_FAILED','6',['1']));
{const p=inputs();p.schema.recordTypes.R={amount:['UInt128']};p.schema.operations.O='R';p.schema.args.fields=['Record','R'];p.Args.fields={amount:'3'};
 add('Emit-positive',Act(N('Emit',{operation:'O',fields:Arg('fields')})),p,prepared({},'98',[{operation:'O',fields:{amount:'3'}}]));
 add('Emit-reject',Act(N('Emit',{operation:'Unknown',fields:Arg('fields')})),p,rejected('TYPE_NAME','0',['0']));
 add('rollback-descriptor',Act(N('Emit',{operation:'O',fields:Arg('fields')}),Ens(B(false))),p,rejected('ENSURES_FAILED','4',['1']));
}
for(const value of ['','-','+','00','-0'])add('malformed-decimal-'+JSON.stringify(value),N('LitUInt',{width:'128',value}),inputs(),rejected('INPUT_SCHEMA'));
for(const name of ['','-','1bad'])add('malformed-name-'+JSON.stringify(name),Arg(name),inputs(),rejected('INPUT_SCHEMA'));
add('empty-text',N('LitText',{value:''}),inputs(),{judgmentResult:'ExpressionValue',type:['Text'],value:'',workRemaining:'99'});
add('dead-dynamic-right',Bin('And',B(false),Bin('Eq',Bin('FloorDiv',I(1),I(0)),I(0))),inputs(),{judgmentResult:'ExpressionValue',type:['Bool'],value:false,workRemaining:'0'},'2');
add('selected-right-work',Bin('And',B(true),B(true)),inputs(),rejected('WORK_EXHAUSTED','2',['1']),'2');
add('prefix-rollback-work',Act(Write('x',U(11)),Req(B(true))),stateX(),rejected('WORK_EXHAUSTED','2',['1']),'2');
{const span={kind:'source',start:'2',end:'3'};const n=Bin('FloorDiv',I(1),I(0));n.span=span;add('utf8-parent-span',n,inputs(),rejected('ARITH_DENOMINATOR','3',[],span),'100','éx');}
{const p=inputs();p.schema.recordTypes.R={self:['Option',['Record','R']]};add('schema-cycle',U(1),p,rejected('TYPE_SCHEMA_CYCLE'));}
add('zero-work-root',U(7),inputs(),rejected('WORK_EXHAUSTED'),'0');
add('static-before-zero-work',Bin('Add',U(7),B(true)),inputs(),rejected('TYPE_MISMATCH'),'0');
add('duplicate-write',Act(Write('x',U(11)),Write('x',U(12))),stateX(),rejected('TYPE_DUPLICATE_WRITE','0',['1']));
{const p=stateX();p.schema.fields.x.writeClass='financial';add('financial-write-forbidden',Act(Write('x',U(11))),p,rejected('TYPE_FINANCIAL_WRITE','0',['0']));}
add('ensure-suffix',Act(Ens(B(true)),Req(B(true))),inputs(),rejected('TYPE_STATEMENT_PLACEMENT','0',['1']));
{const p=inputs();p.schema.observations.bad=['UInt64'];p.Obs.bad='-1';add('dead-input-still-admitted',Bin('Or',B(true),B(false)),p,rejected('INPUT_VALUE'));}
for(const depth of [64,65]){
 let n=B(false);for(let i=1;i<depth;i++)n=N('Not',{value:n});
 add('ast-depth-'+depth,n,inputs(),depth===64?{judgmentResult:'ExpressionValue',type:['Bool'],value:true,workRemaining:'36'}:rejected('INPUT_BOUND','0',Array(64).fill('0')));
}
{const n=U(1);n.span={kind:'source',start:'0',end:'4'};add('invalid-span-fallback',n,inputs(),rejected('INPUT_SPAN'),'100','éx');}
// Deep type metadata is byte-bounded, not value/AST nesting. Build canonical
// source text iteratively to avoid imposing JSON.stringify's host recursion cap.
for(const depth of [1500,5947,5948]) {
 const type='["Option",'.repeat(depth)+'["UInt64"]'+']'.repeat(depth);
 const core='{"constructor":"ConstructNone","operands":{"elementType":'+type+'},"span":{"end":"0","kind":"synthetic","start":"0"}}';
 const request='{"Args":{},"Obs":{},"Pre":{},"contract":"moriarty-expression-contract/1","core":'+core+',"source":"","workInitial":"1"}';
 const expected=Buffer.byteLength(core)>65536?rejected('INPUT_BOUND'):{judgmentResult:'ExpressionValue',type:JSON.parse('["Option",'+type+']'),value:[],workRemaining:'0'};
 traces.push({id:'metadata-depth-'+depth,schema:J(inputs().schema),request,expected});
}
// J is the retained iterative production writer only for final serialization;
// expectations above are specified independently, not observed evaluator output.
const {canonical}=await import('../../../src/successor/expression-wire-v1.ts');
writeFileSync(new URL('./expression-v1.json',import.meta.url),canonical({schemaVersion:'moriarty-k-expression-fixtures/1',scope:'source expectations; K execution unperformed',cases:traces})+'\n');
console.log(JSON.stringify({cases:traces.length,constructors:40,expectations:'independent retained specifications + explicit contextual/boundary cases'}));
