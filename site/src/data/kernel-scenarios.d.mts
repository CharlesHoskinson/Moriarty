/** Types for the educational kernel scenario model. See kernel-scenarios.mjs. */

export interface Asset {
  readonly symbol: string;
  readonly domain: string;
  readonly issuer: string;
  readonly reference: string;
  readonly kind: string;
}

export interface Policy {
  readonly id: string;
  readonly owner: string;
  readonly provider: string;
  readonly recipient: string;
  readonly spend: Asset;
  readonly receive: Asset;
  readonly grossCap: number;
  readonly feeCap: number;
  readonly minReceive: number;
  readonly fills: number;
  readonly fillPrincipal: number;
  readonly fillFee: number;
  readonly fillReceive: number;
  readonly openingCustody: number;
  readonly conditions: readonly string[];
  readonly attestationIssuer: string;
  readonly attestationFreshnessEpochs: number;
}

export interface Candidate {
  readonly id: string;
  readonly label: string;
  readonly summary: string;
  readonly recipient: string;
  readonly principalPerFill: number;
  readonly feePerFill: number;
  readonly receivePerFill: number;
  readonly fills: number;
}

export interface Check {
  readonly id: string;
  readonly name: string;
  readonly met: boolean;
  readonly detail: string;
}

export interface CandidateVerdict {
  readonly candidateId: string;
  readonly accepted: boolean;
  readonly checks: readonly Check[];
}

export interface SolverKind {
  readonly id: 'human' | 'ai';
  readonly label: string;
}

export interface EvidenceFixture {
  readonly id: string;
  readonly label: string;
  readonly issuer: string | null;
  readonly ageEpochs: number | null;
  readonly supported: boolean;
}

export type EvidenceStatus = 'met' | 'unmet' | 'unknown' | 'unsupported';

export interface EvidenceResult {
  readonly status: EvidenceStatus;
  readonly reason: string;
}

export interface Observation {
  readonly id: 'late-success' | 'auth-failure' | 'still-unknown';
  readonly label: string;
}

export interface Account {
  readonly grossDebit: number;
  readonly fees: number;
  readonly reserved: number;
  readonly confirmedReceipt: number;
  readonly custody: number;
}

export interface Duty {
  readonly id: string;
  readonly bearer: string;
  readonly duty: string;
  readonly discharge: string;
  readonly status: 'accepted' | 'pending' | 'discharged';
}

export interface Attempt {
  readonly id: string;
  readonly status: 'submitted' | 'timed-out' | 'unknown' | 'resolved';
  readonly fill: number;
  readonly outcome?: 'late-success' | 'auth-failure';
  /** Set on an observed success: whether the fill was accepted as an agreement stage. */
  readonly accepted?: boolean;
}

export type Responsibility = 'owner' | 'moriarty' | 'kernel' | 'midnight' | 'external' | 'solver';

export interface AccountDelta {
  readonly grossDebit?: number;
  readonly fees?: number;
  readonly reserved?: number;
  readonly confirmedReceipt?: number;
  readonly custody?: number;
}

export interface EventRecord {
  readonly n: number;
  readonly type: EventType;
  readonly title: string;
  readonly responsibility: Responsibility;
  readonly meaning: string;
  readonly delta: AccountDelta | null;
  readonly account: Account;
}

export type Phase = 'uncommitted' | 'committed' | 'in-flight' | 'observed' | 'complete';

export interface ScenarioState {
  readonly policy: Policy;
  readonly phase: Phase;
  readonly candidateId: string | null;
  readonly candidateVerdict: CandidateVerdict | null;
  readonly solverKind: 'human' | 'ai';
  readonly evidenceId: string;
  readonly conditions: Readonly<Record<ConditionId, boolean>>;
  readonly account: Account;
  readonly fillsDone: number;
  readonly attempt: Attempt | null;
  readonly consumed: readonly string[];
  readonly duties: readonly Duty[];
  readonly events: readonly EventRecord[];
  readonly lastNote: string | null;
  readonly lastRejected: { readonly type: EventType; readonly reason: string } | null;
}

export type ConditionId = 'provider-signature' | 'recipient-acceptance';

export interface Condition {
  readonly id: ConditionId;
  readonly label: string;
}

export type EventType =
  | 'select-candidate'
  | 'set-solver-kind'
  | 'present-evidence'
  | 'set-condition'
  | 'provider-accepts-duties'
  | 'commit-candidate'
  | 'finalize-fill'
  | 'reserve-fill'
  | 'timeout'
  | 'observe'
  | 'accept-observed-fill'
  | 'duplicate-observation'
  | 'second-solver-reserve'
  | 'replay-fill'
  | 'premature-refund'
  | 'reset';

export type ScenarioEvent =
  | { type: 'select-candidate'; candidateId: string }
  | { type: 'set-solver-kind'; kind: 'human' | 'ai' }
  | { type: 'present-evidence'; evidenceId: string }
  | { type: 'set-condition'; condition: ConditionId; present: boolean }
  | { type: 'provider-accepts-duties' }
  | { type: 'commit-candidate' }
  | { type: 'finalize-fill' }
  | { type: 'reserve-fill' }
  | { type: 'timeout' }
  | { type: 'observe'; outcome: Observation['id'] }
  | { type: 'accept-observed-fill' }
  | { type: 'duplicate-observation' }
  | { type: 'second-solver-reserve' }
  | { type: 'replay-fill' }
  | { type: 'premature-refund' }
  | { type: 'reset' };

export interface Derived {
  readonly exposure: number;
  readonly grossRemaining: number;
  readonly feeRemaining: number;
  readonly feeEncumbered: number;
  readonly feeFree: number;
  readonly goalMet: boolean;
  readonly complete: boolean;
  readonly openDuties: readonly Duty[];
}

export const ASSETS: { readonly A: Asset; readonly B: Asset };
export const POLICY: Policy;
export const CANDIDATES: readonly Candidate[];
export const SOLVER_KINDS: readonly SolverKind[];
export const EVIDENCE: readonly EvidenceFixture[];
export const OBSERVATIONS: readonly Observation[];
export const CONDITIONS: readonly Condition[];
export const EVENT_TYPES: readonly EventType[];
export const PRESETS: { readonly success: ScenarioEvent[]; readonly failure: ScenarioEvent[]; readonly unknown: ScenarioEvent[] };

export function checkCandidate(candidate: Candidate, policy?: Policy): CandidateVerdict;
export function evaluateEvidence(evidence: EvidenceFixture, policy?: Policy): EvidenceResult;
export function initialState(policy?: Policy, note?: string | null): ScenarioState;
export function apply(state: ScenarioState, event: ScenarioEvent): ScenarioState;
export function derive(state: ScenarioState): Derived;
export function availableEvents(state: ScenarioState): readonly EventType[];
export function run(events: readonly ScenarioEvent[], state?: ScenarioState): ScenarioState;
export function fmt(hundredths: number): string;
