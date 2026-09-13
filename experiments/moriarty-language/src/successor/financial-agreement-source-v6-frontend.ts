/** /6 keeps /5 grammar. Only the profile token changes in the private parser
 * input. Its byte width and every source span stay unchanged. The compiler and
 * digest binding retain the original /6 source, including strings and comments. */
import { parseSuccessorFinancialAgreementSourceV5, SuccessorSyntaxError,
  FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE } from './frontend.ts';
import { formatSuccessorFinancialAgreementSourceV5 } from './format.ts';
export const FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE = 'moriarty-financial-agreement-source/6';
function parserSource(source: string): string {
  try { parseSuccessorFinancialAgreementSourceV5(source); }
  catch(error) {
    if (!(error instanceof SuccessorSyntaxError) || error.code !== 'PROFILE_MISMATCH') throw error;
    // Lexer spans are UTF-8 byte offsets. Only an exact profile token is rewritten.
    const input=new TextEncoder().encode(source);
    const token=new TextDecoder().decode(input.slice(error.start,error.end));
    if(token!==JSON.stringify(FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE)) throw error;
    const replacement=new TextEncoder().encode(JSON.stringify(FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE));
    input.set(replacement,error.start);
    return new TextDecoder().decode(input);
  }
  throw new SuccessorSyntaxError('PROFILE_MISMATCH','profile must be '+FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE);
}
export function parseFinancialAgreementSourceV6(source: string) {
  const program=parseSuccessorFinancialAgreementSourceV5(parserSource(source));
  program.profile.value=FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE;
  program.profile.raw=JSON.stringify(FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE);
  return program;
}
export function formatFinancialAgreementSourceV6(source: string): string {
  const formatted=formatSuccessorFinancialAgreementSourceV5(parserSource(source));
  return formatted.replace(JSON.stringify(FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE),JSON.stringify(FINANCIAL_AGREEMENT_SOURCE_V6_PROFILE));
}
