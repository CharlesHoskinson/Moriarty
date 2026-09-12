/** Source-defined financial agreement with origination and accrual. */
import { FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE } from './frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V4 } from './financial-expression-v1.ts';
import type { SourceRejected } from './financial-expression-source-v1.ts';
import type { Schema } from './financial-expression-types-v1.ts';
import type { ExpressionSpan } from './expression-wire-v1.ts';
import type { LifecycleExpressionResult } from './funded-expression-source-v1.ts';
import {
  compileAgreementSource,
  evaluateCompiledAction,
  rejection,
  selectCompiledAction,
  sourceTransport,
} from './financial-agreement-source-compiler.ts';

export {
  FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
  parseSuccessorFinancialAgreementSourceV5 as parseFinancialAgreementSourceV5,
} from './frontend.ts';
export { formatSuccessorFinancialAgreementSourceV5 as formatFinancialAgreementSourceV5 } from './format.ts';

export interface AgreementSourceV5ActionArtifact {
  action: string;
  schema: Schema;
  core: { statements: object[]; span: ExpressionSpan };
  staticWorkBound: string;
}

export interface AgreementSourceV5Elaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V4;
  agreement: string;
  actions: AgreementSourceV5ActionArtifact[];
}

export interface AgreementSourceV5Checked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE;
  actions: Array<{ action: string; staticWorkBound: string }>;
}

function compile(source: string): AgreementSourceV5Elaborated | SourceRejected {
  const compiled = compileAgreementSource(source, FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE);
  if ('status' in compiled) return compiled;
  return {
    judgmentResult: 'SourceElaborated',
    sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
    contract: FINANCIAL_EXPRESSION_CONTRACT_V4,
    agreement: compiled.agreement,
    actions: compiled.actions,
  };
}

/** Trusted Σ is derived from source text. Public APIs accept primitive strings only. */
export function createFinancialAgreementSourceV5() {
  return Object.freeze({
    elaborate(source: string): AgreementSourceV5Elaborated | SourceRejected {
      try { sourceTransport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): AgreementSourceV5Checked | SourceRejected {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return {
          judgmentResult: 'SourceChecked',
          sourceProfile: FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
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
      financialPreStateJSON: string,
    ): LifecycleExpressionResult {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        const selected = selectCompiledAction(compiled.actions, actionName);
        return evaluateCompiledAction(
          source,
          selected,
          snapshotCanonicalJSON,
          financialPreStateJSON,
          FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
        );
      } catch (error) { return rejection(error, source); }
    },
  });
}
