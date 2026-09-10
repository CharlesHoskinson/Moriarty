import * as original from './expression-v1-support.mjs';
import {createFinancialExpressionContractV1} from '../src/successor/financial-expression-v1.ts';
export {N,U,I,B,Bin,Act,Req,Ens,Let,Write,Pre,Arg,Local} from './expression-v1-support.mjs';
export function inputs(){const p=original.inputs();p.schema.variantTypes={};return p;}
export function stateX(){const p=original.stateX();p.schema.variantTypes={};return p;}
export function run(core,p=inputs(),workInitial='100',source=''){
 return createFinancialExpressionContractV1(original.J(p.schema)).evaluate(original.J({contract:'moriarty-financial-expression-contract/1',source,core,Pre:p.Pre,Args:p.Args,Obs:p.Obs,workInitial:String(workInitial)}));
}
