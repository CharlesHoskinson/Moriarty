import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export enum Phase { Waiting = 0, Funded = 1, Released = 2, Refunded = 3 }

export type PartySecret = Uint8Array;

export type Witnesses<PS> = {
  partySecret(context: __compactRuntime.WitnessContext<Ledger, PS>): [PS, PartySecret];
}

export type ImpureCircuits<PS> = {
  fund(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  release(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  refundAfterTimeout(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
}

export type ProvableCircuits<PS> = {
  fund(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  release(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  refundAfterTimeout(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
}

export type PureCircuits = {
}

export type Circuits<PS> = {
  fund(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  release(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  refundAfterTimeout(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
}

export type Ledger = {
  readonly buyer: { bytes: Uint8Array };
  readonly seller: { bytes: Uint8Array };
  readonly buyerAuthority: Uint8Array;
  readonly tokenColor: Uint8Array;
  readonly amount: bigint;
  readonly deadline: bigint;
  readonly phase: Phase;
}

export type ContractReferenceLocations = any;

export declare const contractReferenceLocations : ContractReferenceLocations;

export declare class Contract<PS = any, W extends Witnesses<PS> = Witnesses<PS>> {
  witnesses: W;
  circuits: Circuits<PS>;
  impureCircuits: ImpureCircuits<PS>;
  provableCircuits: ProvableCircuits<PS>;
  constructor(witnesses: W);
  initialState(context: __compactRuntime.ConstructorContext<PS>,
               initialBuyer_0: { bytes: Uint8Array },
               initialSeller_0: { bytes: Uint8Array },
               initialBuyerAuthority_0: Uint8Array,
               initialTokenColor_0: Uint8Array,
               initialAmount_0: bigint,
               initialDeadline_0: bigint): Promise<__compactRuntime.ConstructorResult<PS>>;
}

export declare function ledger(state: __compactRuntime.StateValue | __compactRuntime.ChargedState): Ledger;
export declare const pureCircuits: PureCircuits;
export declare const expectedVk: Record<string, string>;
