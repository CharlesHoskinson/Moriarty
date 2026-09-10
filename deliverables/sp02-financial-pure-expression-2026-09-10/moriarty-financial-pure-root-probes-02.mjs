import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {createFinancialExpressionContractV1} from '/home/charl/Moriarty/.worktrees/sp02-financial-pure/experiments/moriarty-language/src/successor/financial-expression-v1.ts';
const root='/home/charl/Moriarty/.worktrees/sp02-financial-pure/';
const paths=['experiments/moriarty-language/src/successor/financial-expression-v1.ts','experiments/moriarty-language/src/successor/financial-expression-types-v1.ts','experiments/moriarty-language/src/successor/expression-wire-v1.ts'];
const hashes=()=>Object.fromEntries(paths.map(p=>[p,createHash('sha256').update(readFileSync(root+p)).digest('hex')]));const before=hashes();
const J=x=>Array.isArray(x)?'['+x.map(J).join(',')+']':x&&typeof x==='object'?'{'+Object.keys(x).sort().map(k=>JSON.stringify(k)+':'+J(x[k])).join(',')+'}':JSON.stringify(x);
const n=(constructor,operands={})=>({constructor,operands,span:{kind:'synthetic',start:'0',end:'0'}}),u=(value,width=128)=>n('LitUInt',{width:String(width),value:String(value)}),b=value=>n('LitBool',{value});
const schema=()=>({units:['USD'],assets:['A','B'],vaults:['V'],parties:['H'],recordTypes:{},enumTypes:{},variantTypes:{},fields:{},args:{},observations:{},operations:{}});
let rows=[];
function run(id,core,s=schema(),args={},expected,work='100'){
 const got=createFinancialExpressionContractV1(J(s)).evaluate(J({contract:'moriarty-financial-expression-contract/1',source:'',core,Pre:{},Args:args,Obs:{},workInitial:work}));
 try{for(const [k,v] of Object.entries(expected))assert.deepEqual(got[k],v);rows.push({id,status:'PASS'});}catch(e){rows.push({id,status:'FAIL',got,expected});}
}
for(const from of [64,128,256])for(const to of [64,128,256]){
 const max=(1n<<BigInt(from))-1n;run(`width-${from}-${to}`,n('ConvertUInt',{width:String(to),value:u(max,from)}),undefined,{},from<=to?{judgmentResult:'ExpressionValue',type:['UInt'+to],value:String(max),workRemaining:'98'}:{status:'Rejected',code:'ARITH_RANGE',workUsed:'2'});
}
for(const [type,minimum,resultType] of [[['Rate','2'],-(1n<<127n),'UInt128'],[['Quantity',[['USD','1']],'2'],-(1n<<127n),'UInt128'],[['NetAmount','A'],-(1n<<127n),'UInt128'],[['SignedAmount','A'],-(1n<<255n),'UInt256']]){
 const s=schema();s.args.value=type;run('magnitude-min-'+type[0],n('ScalarValue',{component:'magnitude',value:n('ReadArg',{name:'value'})}),s,{value:String(minimum)},{judgmentResult:'ExpressionValue',type:[resultType],value:String(-minimum),workRemaining:'98'});
}
run('unsigned-conversion-is-not-unit-erasure',n('ConvertUInt',{width:'256',value:n('LitAmount',{asset:'A',value:'1'})}),undefined,{}, {status:'Rejected',code:'TYPE_MISMATCH',workUsed:'0'},'0');
run('construction-needs-explicit-width',n('ConstructAmount',{asset:'A',value:u(1,64)}),undefined,{}, {status:'Rejected',code:'TYPE_MISMATCH',workUsed:'0'});
run('shares-max-roundtrip',n('ScalarValue',{component:'quanta',value:n('ConstructShares',{vault:'V',holder:'H',value:u((1n<<128n)-1n)})}),undefined,{}, {judgmentResult:'ExpressionValue',type:['UInt128'],value:String((1n<<128n)-1n),workRemaining:'97'});
const select=(condition,consequent,alternative)=>n('Select',{condition,consequent,alternative});
run('dead-none-not-evaluated',select(b(true),u(7),n('ProjectSome',{value:n('ConstructNone',{elementType:['UInt128']})})),undefined,{}, {judgmentResult:'ExpressionValue',value:'7',workRemaining:'0'},'3');
run('dead-mismatched-amount-static',select(b(true),n('ConstructAmount',{asset:'A',value:u(7)}),n('ConstructAmount',{asset:'B',value:u(7)})),undefined,{}, {status:'Rejected',code:'TYPE_MISMATCH',workUsed:'0'},'0');
const s=schema();s.recordTypes.LeftValue={v:['UInt128']};s.recordTypes.RightValue={v:['UInt128']};s.variantTypes.Choice={Left:['Record','LeftValue'],Right:['Record','RightValue']};
run('variant-same-payload-is-not-same-tag',n('ProjectVariant',{tag:'Right',value:n('ConstructVariant',{family:'Choice',tag:'Left',value:n('ConstructRecord',{recordType:'LeftValue',fields:[{name:'v',value:u(7)}]})})}),s,{}, {status:'Rejected',code:'VARIANT_CASE',workUsed:'4'});
run('none-rejection-after-projection-charge',n('ProjectSome',{value:n('ConstructNone',{elementType:['UInt128']})}),undefined,{}, {status:'Rejected',code:'OPTION_NONE',workUsed:'2'});
const after=hashes();const report={scope:'Root independent pure-expression boundary probes on current author code; no acceptance audit',before,after,stable:JSON.stringify(before)===JSON.stringify(after),rows};writeFileSync('/tmp/moriarty-financial-pure-root-probes-02.json',JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));if(!report.stable||rows.some(r=>r.status==='FAIL'))process.exitCode=1;
