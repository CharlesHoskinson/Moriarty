/** Inspectable Core for the bounded funded-source profile. Core is not an execution input. */
export const FUNDED_SOURCE_VERSION = 'moriarty-funded-source/0';
export type CoreType =
  | { kind: 'Debt' | 'Amount'; name: string }
  | { kind: 'UInt' | 'Bool' | 'Party' | 'Asset' | 'TransferId' | 'AllocationId' | 'ObligationId' };
export interface CoreValue { type: CoreType; value: string | boolean }
export type CoreExpression =
  | { tag: 'Literal'; type: CoreType; value: string | boolean }
  | { tag: 'Parameter'; type: CoreType; name: string };
export interface CoreEmission {
  kind: 'Transfer' | 'Repay';
  fields: { name: string; expression: CoreExpression }[];
}
export interface CoreAction {
  name: string;
  parameters: { name: string; type: CoreType }[];
  emissions: CoreEmission[];
}
export interface FundedCore {
  schemaVersion: typeof FUNDED_SOURCE_VERSION;
  agreement: string;
  units: string[];
  parties: string[];
  assets: { name: string; unit: string }[];
  actions: CoreAction[];
}
export class FundedSourceError extends Error {
  readonly code: string;
  constructor(code: string) { super(code); this.name = 'FundedSourceError'; this.code = code; }
}
export function fail(code: string): never { throw new FundedSourceError(code); }
export function sameType(a: CoreType, b: CoreType): boolean {
  return a.kind === b.kind && ('name' in a ? 'name' in b && a.name === b.name : !('name' in b));
}
export function identifier(value: unknown): value is string {
  return typeof value === 'string' && /^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(value) && !/[\r\n]/.test(value);
}
export function uint128(value: unknown): value is string {
  return typeof value === 'string' && value.length <= 39 && /^(0|[1-9][0-9]*)$/.test(value)
    && !/[\r\n]/.test(value) && BigInt(value) <= 340282366920938463463374607431768211455n;
}
