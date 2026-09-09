/**
 * The language: specification layers, pipeline, profiles, declarations and real
 * source. Every sample below is verbatim repository source.
 */

export const SPEC_LAYERS = [
  {
    layer: 'Lexical structure',
    method: 'separate token rules and regular expressions',
    fixes: 'identifiers, literals, whitespace, comments, source locations',
  },
  {
    layer: 'Syntax',
    method: 'EBNF, ISO/IEC 14977',
    fixes: 'valid combinations of declarations, actions and expressions',
  },
  {
    layer: 'Static semantics',
    method: 'typing and scoping judgments',
    fixes: 'name resolution, asset units, resource use, admissible bounds',
  },
  {
    layer: 'Dynamic semantics',
    method: 'executable operational semantics in the K Framework',
    fixes: 'state transitions, financial effects, obligations, rejection',
  },
  {
    layer: 'Correctness claims',
    method: 'explicit properties over those semantics',
    fixes: 'what must be established about agreement, execution and history',
  },
] as const;

/** Direct source-to-circuit-IR generation is deliberately deferred. */
export const PIPELINE = [
  { stage: 'Moriarty source', detail: 'a .mori agreement' },
  {
    stage: 'Typed elaboration',
    detail: 'plus a finite resource and lifetime certificate',
  },
  { stage: 'Canonical Moriarty Core', detail: 'the normative representation' },
  {
    stage: 'Generated Compact',
    detail: 'readable, with a correspondence manifest',
  },
  { stage: 'compactc', detail: 'the supported compiler' },
  {
    stage: 'ZKIR 3',
    detail: 'plus generated bindings and proving and verifier artifacts',
  },
  { stage: 'Midnight ledger and wallet', detail: 'acceptance' },
] as const;

export const ZKIR_DEFERRAL =
  'ZKIR is a typed straight-line circuit IR with guarded impacts and no source-level financial concepts, and it is ledger-coupled and evolving. Generating Compact preserves a reviewable backend artifact and reuses the supported compiler, source maps, runtime bindings and ledger operations.';

/** Every datum carries one of these classifications. */
export const VISIBILITY = ['public', 'private', 'committed', 'revealed'] as const;

export const PROFILES = [
  {
    id: 'moriarty-bounded-atomic/1',
    name: 'Atomic profile',
    has: 'grammar, parser, type checker, canonical encoding, local evaluator and restricted Compact lowering',
    note: 'The loan and swap agreements run under it.',
  },
  {
    id: 'moriarty-successor-syntax/0',
    name: 'Successor profile',
    has: 'separate lexical rules, EBNF grammar, bounded parser, canonical formatter and a read-only command line',
    note: 'The two profiles are not interchangeable.',
  },
] as const;

export const DECLARATIONS = [
  'unit',
  'party',
  'const',
  'state',
  'observation',
  'settlement',
  'status',
  'policy',
  'reserve',
  'effect',
  'action',
] as const;

export const STATEMENTS = ['guard', 'let', 'set', 'emit'] as const;
export const BOUNDS = ['lifetime', 'horizon'] as const;

export interface Sample {
  id: string;
  title: string;
  profile: string;
  source: string;
  /** Why this sample is on the site. */
  point: string;
}

export const SAMPLES: readonly Sample[] = [
  {
    id: 'partial-payment',
    title: 'Paying interest may not touch principal',
    profile: 'moriarty-successor-syntax/0',
    source: `profile "moriarty-successor-syntax/0";

agreement PartialPayment {
  unit USD;
  party borrower;
  party lender;
  state principal: Debt<USD> = debt(100, USD);
  state interest:  Debt<USD> = debt(10, USD);
  action payInterest(payment: Debt<USD>) {
    requires payment > debt(0, USD);
    requires payment <= pre.interest;
    next.interest = pre.interest - payment;
    ensures post.principal == pre.principal;
  }
}`,
    point:
      'Explicit pre- and post-state, and Debt<USD> as a type distinct from a transferable Amount<USD>. The ensures line is the whole thesis in one statement.',
  },
  {
    id: 'funded-payment',
    title: 'A payment is a transfer plus an allocation',
    profile: 'moriarty-successor-syntax/0',
    source: `action pay(cash: Amount<Cash>, nominal: Debt<Cash>) {
  emit Transfer {
    id: TransferId("T1"), from: Payer, to: Lender,
    settlementAsset: Cash, amount: cash
  };
  emit Repay {
    allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"),
    obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: nominal
  };
}`,
    point:
      'The transfer, the allocation and the obligation it discharges are three separately identified objects. Moving cash is not the same event as discharging a debt.',
  },
  {
    id: 'policy-rounding',
    title: 'Rounding is declared, not incidental',
    profile: 'moriarty-bounded-atomic/1',
    source: `policy accrued_interest targets write(accrue, interest_due), effect(accrue, 1, amount) {
  unit USD_micro;
  derivation "notional*8*31/(100*365)";
  rounding floor(accrue, interest_calculated);
  remainder "discard 54/73 micro-USD for this sample only";
  comparison "exact integer sample value; no ACTUS tolerance claim";
  proof "loan_first_period_interest_floor_v1";
}`,
    point:
      'Every policy names its unit, derivation, rounding direction, remainder disposition, comparison basis and proof obligation. Integer division no longer decides who keeps the remainder.',
  },
  {
    id: 'swap-guards',
    title: 'Failures are named, not absorbed',
    profile: 'moriarty-bounded-atomic/1',
    source: `let effective_input   = arg.amount_in * const.fee_numerator;
let numerator         = effective_input * state.reserve_b;
let denominator       = state.reserve_a * const.fee_denominator + effective_input;
let output_calculated = floor_div(numerator, denominator);
guard arg.min_out <= output_calculated,      "minimum output not met";
guard output_calculated < state.reserve_b,   "output would empty reserve";`,
    point:
      'Reserves 1,000,000 A and 2,000,000 B, a 997/1000 fee and 10,000 A in produce exactly 19,743 B. Asking for 19,744 gives a named slippage failure, not a silent adjustment.',
  },
  {
    id: 'closure-guard',
    title: 'Closing an episode does not discharge a debt',
    profile: 'moriarty-bounded-atomic/1',
    source: `guard state.notional == const.expected_outstanding_notional,
      "episode closure cannot discharge remaining notional";`,
    point:
      'A bounded episode ends. The obligation it left behind does not end with it.',
  },
] as const;

/** The authoring and settlement flow the workspace implements. */
export const WORKFLOW = [
  'Import or author agreement',
  'Check types and bounds',
  'Inspect events and simulate',
  'Analyze named properties',
  'Prepare and verify plan',
  'Review and sign intent',
  'Prove authorized transition',
  'Verify statement and proof',
  'Check live ledger state and submit',
] as const;

/** Two rules the workspace enforces. */
export const WORKFLOW_RULES = [
  'Simulation and static analysis are distinct. A successful example does not turn the property list green.',
  'A valid proof does not make a stale predecessor live.',
] as const;
