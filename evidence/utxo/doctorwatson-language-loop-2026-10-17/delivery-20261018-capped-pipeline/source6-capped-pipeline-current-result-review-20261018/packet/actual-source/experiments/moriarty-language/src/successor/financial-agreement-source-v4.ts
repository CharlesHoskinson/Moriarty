/** Source-defined financial agreement with kernel-backed financial postconditions. */
import { FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE } from './frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V3 } from './financial-expression-v1.ts';
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
  FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE,
  parseSuccessorFinancialAgreementSourceV4 as parseFinancialAgreementSourceV4,
} from './frontend.ts';
export { formatSuccessorFinancialAgreementSourceV4 as formatFinancialAgreementSourceV4 } from './format.ts';

export interface AgreementSourceV4ActionArtifact {
  action: string;
  schema: Schema;
  core: { statements: object[]; span: ExpressionSpan };
  staticWorkBound: string;
}

export interface AgreementSourceV4Elaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V3;
  agreement: string;
  actions: AgreementSourceV4ActionArtifact[];
}

export interface AgreementSourceV4Checked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE;
  actions: Array<{ action: string; staticWorkBound: string }>;
}

function compile(source: string): AgreementSourceV4Elaborated | SourceRejected {
  const compiled = compileAgreementSource(source, FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE);
  if ('status' in compiled) return compiled;
  return {
    judgmentResult: 'SourceElaborated',
    sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE,
    contract: FINANCIAL_EXPRESSION_CONTRACT_V3,
    agreement: compiled.agreement,
    actions: compiled.actions,
  };
}

/** Trusted Σ is derived from source text. Public APIs accept primitive strings only. */
export function createFinancialAgreementSourceV4() {
  return Object.freeze({
    elaborate(source: string): AgreementSourceV4Elaborated | SourceRejected {
      try { sourceTransport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): AgreementSourceV4Checked | SourceRejected {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return {
          judgmentResult: 'SourceChecked',
          sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE,
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
          FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE,
        );
      } catch (error) { return rejection(error, source); }
    },
  });
}
