/** Source-defined financial agreement with kernel-backed financial reads. */
import { FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE } from './frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V2 } from './financial-expression-v1.ts';
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
  FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
  parseSuccessorFinancialAgreementSourceV3 as parseFinancialAgreementSourceV3,
} from './frontend.ts';
export { formatSuccessorFinancialAgreementSourceV3 as formatFinancialAgreementSourceV3 } from './format.ts';

export interface AgreementSourceV3ActionArtifact {
  action: string;
  schema: Schema;
  core: { statements: object[]; span: ExpressionSpan };
  staticWorkBound: string;
}

export interface AgreementSourceV3Elaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V2;
  agreement: string;
  actions: AgreementSourceV3ActionArtifact[];
}

export interface AgreementSourceV3Checked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE;
  actions: Array<{ action: string; staticWorkBound: string }>;
}

function compile(source: string): AgreementSourceV3Elaborated | SourceRejected {
  const compiled = compileAgreementSource(source, FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE);
  if ('status' in compiled) return compiled;
  return {
    judgmentResult: 'SourceElaborated',
    sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
    contract: FINANCIAL_EXPRESSION_CONTRACT_V2,
    agreement: compiled.agreement,
    actions: compiled.actions,
  };
}

/** Trusted Σ is derived from source text. Public APIs accept primitive strings only. */
export function createFinancialAgreementSourceV3() {
  return Object.freeze({
    elaborate(source: string): AgreementSourceV3Elaborated | SourceRejected {
      try { sourceTransport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): AgreementSourceV3Checked | SourceRejected {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return {
          judgmentResult: 'SourceChecked',
          sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
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
        return evaluateCompiledAction(
          source,
          selected,
          snapshotCanonicalJSON,
          repaymentStateJSON,
          FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
        );
      } catch (error) { return rejection(error, source); }
    },
  });
}
