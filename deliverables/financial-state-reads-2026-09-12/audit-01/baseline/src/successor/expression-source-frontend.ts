/** The expression profile has explicit parse/format entry points. The original
 * syntax-only and funded entry points never opt into these semantics. */
export { EXPRESSION_SOURCE_PROFILE, parseSuccessorExpressionSource as parseExpressionSource } from './frontend.ts';
export { formatSuccessorExpressionSource as formatExpressionSource } from './format.ts';
