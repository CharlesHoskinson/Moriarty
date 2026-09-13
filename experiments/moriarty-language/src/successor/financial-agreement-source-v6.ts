/** Explicit source /6 factory. No caller-supplied Core or implicit migration. */
import { FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE } from './financial-agreement-source-v6-frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V5, createFinancialExpressionContractV5 } from './financial-expression-v5.ts';
import { compileAgreementSource, rejection, selectCompiledAction, sourceTransport, snapshots } from './financial-agreement-source-compiler.ts';
import { canonical } from './expression-wire-v1.ts';
export {FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE, parseFinancialAgreementSourceV6, formatFinancialAgreementSourceV6} from './financial-agreement-source-v6-frontend.ts';
export function createFinancialAgreementSourceV6() {
  function compile(source:string) {
    sourceTransport(source);
    const compiled=compileAgreementSource(source,FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE);
    if('status' in compiled)return compiled;
    return {judgmentResult:'SourceElaborated' as const,sourceProfile:FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE,
      contract:FINANCIAL_EXPRESSION_CONTRACT_V5,...compiled};
  }
  return Object.freeze({
    elaborate(source:string) {try{return compile(source);}catch(error){return rejection(error,source);}},
    check(source:string) {
      try {const c=compile(source);if('status' in c)return c;
        return {judgmentResult:'SourceChecked' as const,sourceProfile:FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE,
          actions:c.actions.map(a=>({action:a.action,staticWorkBound:a.staticWorkBound}))};
      }catch(error){return rejection(error,source);}
    },
    evaluate(source:string,actionName:string,snapshotCanonicalJSON:string,financialPreStateJSON:string) {
      try {const c=compile(source);if('status' in c)return c;
        const action=selectCompiledAction(c.actions,actionName),input=snapshots(snapshotCanonicalJSON);
        return createFinancialExpressionContractV5(canonical(action.schema),financialPreStateJSON).evaluate(canonical({
          contract:FINANCIAL_EXPRESSION_CONTRACT_V5,source,core:action.core,...input}));
      }catch(error){return rejection(error,source);}
    },
  });
}
