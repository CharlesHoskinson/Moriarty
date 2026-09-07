import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export type MulHint = { aLo: bigint;
                        aHi: bigint;
                        bLo: bigint;
                        bHi: bigint;
                        lo: bigint;
                        carry: bigint
                      };

export type DivisionHint = { q: bigint; r: bigint; product: MulHint };

export type KernelState = { f0: bigint;
                            f1: bigint;
                            f2: bigint;
                            f3: bigint;
                            f4: bigint;
                            f5: bigint;
                            f6: bigint
                          };

export type KernelObservations = { o0: bigint };

export type Arguments0 = { a0: bigint;
                           a1: bigint;
                           a2: bigint;
                           a3: bigint;
                           a4: bigint;
                           a5: bigint
                         };

export type Hints0 = { h0: MulHint; h1: MulHint; h2: MulHint; h3: DivisionHint
                     };

export type Effect0_0 = { v0: bigint; v1: bigint; v2: bigint; v3: bigint };

export type Effect0_1 = { v0: bigint; v1: bigint; v2: bigint; v3: bigint };

export type Result0 = { after: KernelState;
                        remaining: bigint;
                        revision: bigint;
                        effect0: Effect0_0;
                        effect1: Effect0_1
                      };

export type Arguments1 = { a0: bigint };

export type Hints1 = { unused: bigint };

export type Effect1_0 = { v0: bigint; v1: bigint; v2: bigint; v3: bigint };

export type Effect1_1 = { v0: bigint; v1: bigint; v2: bigint; v3: bigint };

export type Result1 = { after: KernelState;
                        remaining: bigint;
                        revision: bigint;
                        effect0: Effect1_0;
                        effect1: Effect1_1
                      };

export type Witnesses<PS> = {
}

export type ImpureCircuits<PS> = {
}

export type ProvableCircuits<PS> = {
}

export type PureCircuits = {
  checkedAdd(a_0: bigint, b_0: bigint): bigint;
  checkedSub(a_0: bigint, b_0: bigint): bigint;
  checkedMul(a_0: bigint, b_0: bigint, h_0: MulHint): bigint;
  checkedDiv(n_0: bigint, d_0: bigint, q_0: bigint, r_0: bigint, h_0: MulHint): bigint;
  transition0(before_0: KernelState,
              args_0: Arguments0,
              observations_0: KernelObservations,
              remaining_0: bigint,
              revision_0: bigint,
              hints_0: Hints0): Result0;
  transition1(before_0: KernelState,
              args_0: Arguments1,
              observations_0: KernelObservations,
              remaining_0: bigint,
              revision_0: bigint,
              hints_0: Hints1): Result1;
}

export type Circuits<PS> = {
  checkedAdd(context: __compactRuntime.CircuitContext<PS>,
             a_0: bigint,
             b_0: bigint): __compactRuntime.CircuitResults<PS, bigint>;
  checkedSub(context: __compactRuntime.CircuitContext<PS>,
             a_0: bigint,
             b_0: bigint): __compactRuntime.CircuitResults<PS, bigint>;
  checkedMul(context: __compactRuntime.CircuitContext<PS>,
             a_0: bigint,
             b_0: bigint,
             h_0: MulHint): __compactRuntime.CircuitResults<PS, bigint>;
  checkedDiv(context: __compactRuntime.CircuitContext<PS>,
             n_0: bigint,
             d_0: bigint,
             q_0: bigint,
             r_0: bigint,
             h_0: MulHint): __compactRuntime.CircuitResults<PS, bigint>;
  transition0(context: __compactRuntime.CircuitContext<PS>,
              before_0: KernelState,
              args_0: Arguments0,
              observations_0: KernelObservations,
              remaining_0: bigint,
              revision_0: bigint,
              hints_0: Hints0): __compactRuntime.CircuitResults<PS, Result0>;
  transition1(context: __compactRuntime.CircuitContext<PS>,
              before_0: KernelState,
              args_0: Arguments1,
              observations_0: KernelObservations,
              remaining_0: bigint,
              revision_0: bigint,
              hints_0: Hints1): __compactRuntime.CircuitResults<PS, Result1>;
}

export type Ledger = {
}

export type ContractReferenceLocations = any;

export declare const contractReferenceLocations : ContractReferenceLocations;

export declare class Contract<PS = any, W extends Witnesses<PS> = Witnesses<PS>> {
  witnesses: W;
  circuits: Circuits<PS>;
  impureCircuits: ImpureCircuits<PS>;
  provableCircuits: ProvableCircuits<PS>;
  constructor(witnesses: W);
  initialState(context: __compactRuntime.ConstructorContext<PS>): __compactRuntime.ConstructorResult<PS>;
}

export declare function ledger(state: __compactRuntime.StateValue | __compactRuntime.ChargedState): Ledger;
export declare const pureCircuits: PureCircuits;
