/** Public source /4 exhaustive effect consumer. Closed Transfer and Repayment only. */
import { createFinancialAgreementSourceV4 } from './financial-agreement-source-v4.ts';

const result = createFinancialAgreementSourceV4().evaluate('', '', '', '');
if (result.status === 'FundedExpressionPrepared') {
  for (const effect of result.effects) {
    switch (effect.kind) {
      case 'Transfer':
        break;
      case 'Repayment':
        break;
      default: {
        const impossible: never = effect;
        void impossible;
      }
    }
  }
}
