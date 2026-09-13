import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export type Witnesses<PS> = {
}

export type ImpureCircuits<PS> = {
  originate(context: __compactRuntime.CircuitContext<PS>,
            borrowerSecret_0: Uint8Array,
            lenderSecret_0: Uint8Array,
            expectedProgram_0: Uint8Array,
            expectedNetwork_0: Uint8Array,
            expectedRevision_0: bigint,
            expectedActor_0: bigint,
            expectedDebtor_0: bigint,
            privateFeeAmount_0: bigint,
            now_0: bigint): __compactRuntime.CircuitResults<PS, []>;
}

export type ProvableCircuits<PS> = {
  originate(context: __compactRuntime.CircuitContext<PS>,
            borrowerSecret_0: Uint8Array,
            lenderSecret_0: Uint8Array,
            expectedProgram_0: Uint8Array,
            expectedNetwork_0: Uint8Array,
            expectedRevision_0: bigint,
            expectedActor_0: bigint,
            expectedDebtor_0: bigint,
            privateFeeAmount_0: bigint,
            now_0: bigint): __compactRuntime.CircuitResults<PS, []>;
}

export type PureCircuits = {
}

export type Circuits<PS> = {
  originate(context: __compactRuntime.CircuitContext<PS>,
            borrowerSecret_0: Uint8Array,
            lenderSecret_0: Uint8Array,
            expectedProgram_0: Uint8Array,
            expectedNetwork_0: Uint8Array,
            expectedRevision_0: bigint,
            expectedActor_0: bigint,
            expectedDebtor_0: bigint,
            privateFeeAmount_0: bigint,
            now_0: bigint): __compactRuntime.CircuitResults<PS, []>;
}

export type Ledger = {
  readonly programDigest: Uint8Array;
  readonly sourceDigest: Uint8Array;
  readonly coreDigest: Uint8Array;
  readonly networkTag: Uint8Array;
  readonly borrowerCapability: Uint8Array;
  readonly lenderCapability: Uint8Array;
  readonly borrowerAddress: { bytes: Uint8Array };
  readonly lenderAddress: { bytes: Uint8Array };
  readonly cashColor: Uint8Array;
  readonly revision: bigint;
  readonly originated: boolean;
  readonly borrowerGrossCap: bigint;
  readonly lenderGrossCap: bigint;
  readonly borrowerHasNetGoal: boolean;
  readonly lenderHasNetGoal: boolean;
  readonly borrowerMinimumNetCredit: bigint;
  readonly lenderMinimumNetCredit: bigint;
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
               borrowerSecret_0: Uint8Array,
               lenderSecret_0: Uint8Array,
               borrowerPayout_0: { bytes: Uint8Array },
               lenderPayout_0: { bytes: Uint8Array },
               expectedProgram_0: Uint8Array,
               expectedNetwork_0: Uint8Array,
               settlementColor_0: Uint8Array,
               debtorCap_0: bigint,
               creditorCap_0: bigint,
               debtorHasGoal_0: boolean,
               creditorHasGoal_0: boolean,
               debtorMinimum_0: bigint,
               creditorMinimum_0: bigint): __compactRuntime.ConstructorResult<PS>;
}

export declare function ledger(state: __compactRuntime.StateValue | __compactRuntime.ChargedState): Ledger;
export declare const pureCircuits: PureCircuits;
