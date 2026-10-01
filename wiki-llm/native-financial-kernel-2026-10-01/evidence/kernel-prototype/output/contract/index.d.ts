import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export type Secp256k1EcdsaSignature = { r: bigint; s: bigint };

export type Witnesses<PS> = {
}

export type ImpureCircuits<PS> = {
  pay(context: __compactRuntime.CircuitContext<PS>,
      message_0: Uint8Array,
      sig_0: Secp256k1EcdsaSignature): Promise<__compactRuntime.CircuitResults<PS, Uint8Array>>;
}

export type ProvableCircuits<PS> = {
  pay(context: __compactRuntime.CircuitContext<PS>,
      message_0: Uint8Array,
      sig_0: Secp256k1EcdsaSignature): Promise<__compactRuntime.CircuitResults<PS, Uint8Array>>;
}

export type PureCircuits = {
  signatureIsCanonical(sig_0: Secp256k1EcdsaSignature): [];
  verifyOwner(message_0: Uint8Array,
              sig_0: Secp256k1EcdsaSignature,
              pk_0: __compactRuntime.Secp256k1Point): Uint8Array;
}

export type Circuits<PS> = {
  signatureIsCanonical(context: __compactRuntime.CircuitContext<PS>,
                       sig_0: Secp256k1EcdsaSignature): Promise<__compactRuntime.CircuitResults<PS, []>>;
  verifyOwner(context: __compactRuntime.CircuitContext<PS>,
              message_0: Uint8Array,
              sig_0: Secp256k1EcdsaSignature,
              pk_0: __compactRuntime.Secp256k1Point): Promise<__compactRuntime.CircuitResults<PS, Uint8Array>>;
  pay(context: __compactRuntime.CircuitContext<PS>,
      message_0: Uint8Array,
      sig_0: Secp256k1EcdsaSignature): Promise<__compactRuntime.CircuitResults<PS, Uint8Array>>;
}

export type Ledger = {
  readonly ownerKey: __compactRuntime.Secp256k1Point;
  readonly sourceDigest: Uint8Array;
  readonly ownerProgramDigest: Uint8Array;
  readonly usedNonce: boolean;
  readonly head: Uint8Array;
  readonly revision: bigint;
  readonly trustedRound: bigint;
  readonly ownerBalance: bigint;
  readonly recipientBalance: bigint;
  readonly feeBalance: bigint;
  readonly workRemaining: bigint;
  readonly workSpent: bigint;
  readonly allowanceRemaining: bigint;
  readonly allowanceSpent: bigint;
  readonly assetColor: Uint8Array;
  readonly recipientAddress: { bytes: Uint8Array };
  readonly feeAddress: { bytes: Uint8Array };
  readonly lastDebit: bigint;
  readonly lastRecipientCredit: bigint;
  readonly lastFeeCredit: bigint;
  readonly lastAllowanceUse: bigint;
}

export declare class Contract<PS = any, W extends Witnesses<PS> = Witnesses<PS>> {
  witnesses: W;
  circuits: Circuits<PS>;
  impureCircuits: ImpureCircuits<PS>;
  provableCircuits: ProvableCircuits<PS>;
  constructor(witnesses: W);
  initialState(context: __compactRuntime.ConstructorContext<PS>,
               pk_0: __compactRuntime.Secp256k1Point,
               color_0: Uint8Array,
               recipient_0: { bytes: Uint8Array },
               fee_0: { bytes: Uint8Array },
               funding_0: bigint,
               allowance_0: bigint,
               work_0: bigint,
               round_0: bigint): Promise<__compactRuntime.ConstructorResult<PS>>;
}

export declare function ledger(state: __compactRuntime.StateValue | __compactRuntime.ChargedState): Ledger;
export declare const pureCircuits: PureCircuits;
export declare const expectedVk: Record<string, string>;
export declare const circuitSignatures: __compactRuntime.CircuitSignatures;
export declare const declaredInterfaces: __compactRuntime.DeclaredInterfaces;
