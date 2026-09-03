import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export enum Phase { WaitingAlice = 0,
                    WaitingBob = 1,
                    WaitingDecision = 2,
                    Settled = 3,
                    Refunded = 4
}

export type PartySecret = Uint8Array;

export type Witnesses<PS> = {
  aliceSecret(context: __compactRuntime.WitnessContext<Ledger, PS>): [PS, PartySecret];
  bobSecret(context: __compactRuntime.WitnessContext<Ledger, PS>): [PS, PartySecret];
}

export type ImpureCircuits<PS> = {
  fundAlice(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  fundBob(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  decide(context: __compactRuntime.CircuitContext<PS>, decision_0: bigint): Promise<__compactRuntime.CircuitResults<PS, []>>;
  expire(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
}

export type ProvableCircuits<PS> = {
  fundAlice(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  fundBob(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  decide(context: __compactRuntime.CircuitContext<PS>, decision_0: bigint): Promise<__compactRuntime.CircuitResults<PS, []>>;
  expire(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
}

export type PureCircuits = {
}

export type Circuits<PS> = {
  fundAlice(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  fundBob(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
  decide(context: __compactRuntime.CircuitContext<PS>, decision_0: bigint): Promise<__compactRuntime.CircuitResults<PS, []>>;
  expire(context: __compactRuntime.CircuitContext<PS>): Promise<__compactRuntime.CircuitResults<PS, []>>;
}

export type Ledger = {
  readonly alice: { bytes: Uint8Array };
  readonly bob: { bytes: Uint8Array };
  readonly aliceAuthority: Uint8Array;
  readonly bobAuthority: Uint8Array;
  readonly tokenA: Uint8Array;
  readonly tokenB: Uint8Array;
  readonly amountA: bigint;
  readonly amountB: bigint;
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
               initialAlice_0: { bytes: Uint8Array },
               initialBob_0: { bytes: Uint8Array },
               initialAliceAuthority_0: Uint8Array,
               initialBobAuthority_0: Uint8Array,
               initialTokenA_0: Uint8Array,
               initialTokenB_0: Uint8Array,
               initialAmountA_0: bigint,
               initialAmountB_0: bigint,
               initialDeadline_0: bigint): Promise<__compactRuntime.ConstructorResult<PS>>;
}

export declare function ledger(state: __compactRuntime.StateValue | __compactRuntime.ChargedState): Ledger;
export declare const pureCircuits: PureCircuits;
export declare const expectedVk: Record<string, string>;
