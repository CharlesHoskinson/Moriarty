import { createExpressionContractV1 } from '../src/successor/expression-v1.ts';
export const P = {kind:'synthetic',start:'0',end:'0'};
export const J = v => Array.isArray(v) ? '['+v.map(J).join(',')+']' : v !== null && typeof v === 'object'
  ? '{'+Object.keys(v).sort().map(k=>JSON.stringify(k)+':'+J(v[k])).join(',')+'}' : JSON.stringify(v);
export const N = (constructor, operands={}) => ({constructor,operands,span:{...P}});
export const U = (value,width=128) => N('LitUInt',{width:String(width),value:String(value)});
export const I = value => N('LitSInt',{value:String(value)});
export const B = value => N('LitBool',{value});
export const Bin = (kind,left,right) => N(kind,{left,right});
export const Act = (...statements) => ({statements,span:{...P}});
export const Req = condition => N('Require',{condition});
export const Ens = condition => N('Ensure',{condition});
export const Let = (name,value) => N('Let',{name,value});
export const Write = (field,value) => N('NextWrite',{field,value});
export const Pre = (field,view='pre') => N('ReadPre',{field,view});
export const Arg = name => N('ReadArg',{name});
export const Local = name => N('ReadLocal',{name});
export function inputs() { return {schema:{units:['USD','m','s'],assets:['A','B'],vaults:['V'],parties:['H'],recordTypes:{},enumTypes:{},fields:{},args:{},observations:{},operations:{}},Pre:{},Args:{},Obs:{}}; }
export function run(core, p=inputs(), workInitial='100', source='') {
  return createExpressionContractV1(J(p.schema)).evaluate(J({contract:'moriarty-expression-contract/1',source,core,Pre:p.Pre,Args:p.Args,Obs:p.Obs,workInitial:String(workInitial)}));
}
export function stateX() { const p=inputs();p.schema.fields.x={type:['UInt128'],writeClass:'ordinary'};p.Pre.x='10';return p; }
