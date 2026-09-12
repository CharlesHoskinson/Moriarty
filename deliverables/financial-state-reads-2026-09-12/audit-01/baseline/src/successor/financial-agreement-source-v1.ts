/** Source-defined financial agreement: declarations compile to the existing schema. */
import { FINANCIAL_AGREEMENT_SOURCE_PROFILE } from './frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V1 } from './financial-expression-v1.ts';
import type { SourceRejected } from './financial-expression-source-v1.ts';
import type { Schema } from './financial-expression-types-v1.ts';
import type { ExpressionSpan } from './expression-wire-v1.ts';
import type { FundedExpressionResult } from './funded-expression-source-v1.ts';
import {
  compileAgreementSource,
  evaluateCompiledAction,
  rejection,
  sourceTransport,
} from './financial-agreement-source-compiler.ts';

export {
  FINANCIAL_AGREEMENT_SOURCE_PROFILE,
  parseSuccessorFinancialAgreementSource as parseFinancialAgreementSource,
} from './frontend.ts';
export { formatSuccessorFinancialAgreementSource as formatFinancialAgreementSource } from './format.ts';

export interface AgreementSourceElaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V1;
  agreement: string;
  action: string;
  staticWorkBound: string;
  schema: Schema;
  core: { statements: object[]; span: ExpressionSpan };
}
export interface AgreementSourceChecked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_PROFILE;
  staticWorkBound: string;
}

function compile(source: string): AgreementSourceElaborated | SourceRejected {
  const compiled = compileAgreementSource(source, FINANCIAL_AGREEMENT_SOURCE_PROFILE);
  if ('status' in compiled) return compiled;
  const only = compiled.actions[0];
  return {
    judgmentResult: 'SourceElaborated',
    sourceProfile: FINANCIAL_AGREEMENT_SOURCE_PROFILE,
    contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    agreement: compiled.agreement,
    action: only.action,
    staticWorkBound: only.staticWorkBound,
    schema: only.schema,
    core: only.core,
  };
}

/** Trusted Σ is derived from source text. Public APIs accept primitive strings only. */
export function createFinancialAgreementSourceV1() {
  return Object.freeze({
    elaborate(source: string): AgreementSourceElaborated | SourceRejected {
      try { sourceTransport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): AgreementSourceChecked | SourceRejected {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return {
          judgmentResult: 'SourceChecked',
          sourceProfile: FINANCIAL_AGREEMENT_SOURCE_PROFILE,
          staticWorkBound: compiled.staticWorkBound,
        };
      } catch (error) { return rejection(error, source); }
    },
    evaluate(source: string, snapshotCanonicalJSON: string, repaymentStateJSON: string): FundedExpressionResult {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return evaluateCompiledAction(source, {
          action: compiled.action,
          schema: compiled.schema,
          core: compiled.core,
          staticWorkBound: compiled.staticWorkBound,
        }, snapshotCanonicalJSON, repaymentStateJSON);
      } catch (error) { return rejection(error, source); }
    },
  });
}
