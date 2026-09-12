/** Controlled financial source lowering; callers of the source API supply text only. */
import { createSourceLowering } from './expression-source-lower.ts';
import { sourceExtension } from './financial-expression-source-types.ts';
import type { Schema } from './financial-expression-types-v1.ts';
export function createFinancialSourceLowering(schema: Schema, parameters: Set<string>) {
  return createSourceLowering(schema, parameters, sourceExtension(schema));
}
