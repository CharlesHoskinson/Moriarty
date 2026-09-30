/// <reference types="node" />
/** Closed proposed image records. Runtime admission also rejects extra fields. */
export type IdSort = 'agreement' | 'domain' | 'asset' | 'action' | 'coreProgram' | 'signer' | 'account' | 'obligation';
export type Id<S extends IdSort> = Readonly<{sort: S; value: string}>;
/** Canonical unsigned decimal text; exact domain checked at runtime. */
export type Decimal = string;
export type Kind = 'Transfer' | 'Repay';
export interface SourceDefinition {
  sourceVersion: 6; profile: 'moriarty-financial-agreement-source/6'; wireProfile: 1;
  agreementInstanceId: Id<'agreement'>; domain: Id<'domain'>; asset: Id<'asset'>;
  scale: Decimal; selectedActionId: Id<'action'>; operationKind: Kind;
}
export interface CorePackage {
  coreVersion: 5; coreProfile: 'moriarty-core/5'; sourceVersion: 6; intentSchema: 'moriarty-intent/3'; wireProfile: 1;
  coreProgramId: Id<'coreProgram'>; operationKind: Kind;
  lowerEntry: 'parseAndLowerSource6'; prepareEntry: 'prepareMil4S0'; wrapperEntry: 'prepareSource6S0Unqualified';
  fileCount: 3; files: [{role: 1; bytes: Uint8Array}, {role: 2; bytes: Uint8Array}, {role: 3; bytes: Uint8Array}];
}
export type Operation = {kind: 'Transfer'; owner: Id<'account'>; recipient: Id<'account'>;
  feeRecipient: Id<'account'>; amount: Decimal; fee: Decimal} |
  {kind: 'Repay'; obligationId: Id<'obligation'>; payer: Id<'account'>; amount: Decimal;
    allocation: 'AccrualFirst'; conversion: 'identity'};
export interface PolicyTerms {
  signer: Id<'signer'>; keyRef: string; validFrom: Decimal; validUntil: Decimal;
  grossCap: Decimal; feeCap: Decimal; netFloor: Decimal; failureRelation: 'success_only';
  supplyChanges: 'empty'; observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty';
  retainedDuties: 'empty'; delegation: 'none'; recovery: 'none'; operation: Operation;
}
export interface Policy extends PolicyTerms {
  sourceVersion: 6; coreVersion: 5; wireProfile: 1; agreementInstanceId: Id<'agreement'>;
  domain: Id<'domain'>; asset: Id<'asset'>; scale: Decimal; actionId: Id<'action'>;
  coreProgramId: Id<'coreProgram'>; operationKind: Kind; sourceHash: string; coreHash: string;
}
export interface Image {purpose: 1 | 3 | 4; payload: Buffer; preimage: Buffer; digest: string}
export const SOURCE_PROFILE: 'moriarty-financial-agreement-source/6';
export class ImageCodecError extends Error {field: string; scope: 'local-image-domain'}
export function id<S extends IdSort>(sort: S, value: string): Id<S>;
export function encodeImage(purpose: 1, value: SourceDefinition): Image;
export function encodeImage(purpose: 3, value: CorePackage): Image;
export function encodeImage(purpose: 4, value: Policy): Image;
export function produceImages(definition: SourceDefinition, corePackage: CorePackage, terms: PolicyTerms):
  {source: Image; core: Image; policy: Image; policyValue: Policy};
export interface ContentComparison {matches: boolean; digest: string; claimedDigest: string;
  scope: 'content-only'; authenticated: false}
export function compareContent(purpose: 1, value: SourceDefinition, claimedDigest: string): ContentComparison;
export function compareContent(purpose: 3, value: CorePackage, claimedDigest: string): ContentComparison;
export function compareContent(purpose: 4, value: Policy, claimedDigest: string): ContentComparison;
