/** Source-defined financial agreement with multiple named actions. */
import { FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE } from './frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V1 } from './financial-expression-v1.ts';
import type { SourceRejected } from './financial-expression-source-v1.ts';
import type { Schema } from './financial-expression-types-v1.ts';
import type { ExpressionSpan } from './expression-wire-v1.ts';
import type { FundedExpressionResult } from './funded-expression-source-v1.ts';
import {
  compileAgreementSource,
  evaluateCompiledAction,
  rejection,
  selectCompiledAction,
  sourceTransport,
} from './financial-agreement-source-compiler.ts';

export {
  FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
  parseSuccessorFinancialAgreementSourceV2 as parseFinancialAgreementSourceV2,
} from './frontend.ts';
export { formatSuccessorFinancialAgreementSourceV2 as formatFinancialAgreementSourceV2 } from './format.ts';

export interface AgreementSourceV2ActionArtifact {
  action: string;
  schema: Schema;
  core: { statements: object[]; span: ExpressionSpan };
  staticWorkBound: string;
}

export interface AgreementSourceV2Elaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V1;
  agreement: string;
  actions: AgreementSourceV2ActionArtifact[];
}

export interface AgreementSourceV2Checked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE;
  actions: Array<{ action: string; staticWorkBound: string }>;
}

function compile(source: string): AgreementSourceV2Elaborated | SourceRejected {
  const compiled = compileAgreementSource(source, FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE);
  if ('status' in compiled) return compiled;
  return {
    judgmentResult: 'SourceElaborated',
    sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
    contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    agreement: compiled.agreement,
    actions: compiled.actions,
  };
}

/** Trusted Σ is derived from source text. Public APIs accept primitive strings only. */
export function createFinancialAgreementSourceV2() {
  return Object.freeze({
    elaborate(source: string): AgreementSourceV2Elaborated | SourceRejected {
      try { sourceTransport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): AgreementSourceV2Checked | SourceRejected {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return {
          judgmentResult: 'SourceChecked',
          sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
          actions: compiled.actions.map((item) => ({
            action: item.action,
            staticWorkBound: item.staticWorkBound,
          })),
        };
      } catch (error) { return rejection(error, source); }
    },
    evaluate(
      source: string,
      actionName: string,
      snapshotCanonicalJSON: string,
      repaymentStateJSON: string,
    ): FundedExpressionResult {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        const selected = selectCompiledAction(compiled.actions, actionName);
        return evaluateCompiledAction(source, selected, snapshotCanonicalJSON, repaymentStateJSON);
      } catch (error) { return rejection(error, source); }
    },
  });
}
