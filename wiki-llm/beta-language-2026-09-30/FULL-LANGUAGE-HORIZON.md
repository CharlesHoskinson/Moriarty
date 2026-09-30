# Moriarty full language horizon 0.1

**Status:** specified programmer interface; grammar, financial type checker,
profile evaluator, certificates, kernel qualification and ledger commitment are
**not implemented by this document**. All `moriarty-horizon/0.1` code below is
proposed syntax. It is distinct from the beta brace-record registry and from the
existing local Source/6–Core/5 S0 baseline explained in
[PROGRAMMER-MOCKUP](PROGRAMMER-MOCKUP.md). No beta parser is expected to accept
these structs, transitions, functions or match expressions.

This repair responds to [audit04 H1/H2/M1/M2](audits/04-language-horizon.md) and the
[original requirements](../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md).
Existing reviews remain reviews of their recorded candidate, not approvals of
this new proposal. Every financial example is **specified only**; the missing
implementation and correspondence remain **open**. Existing S0 is **local** only.

## 1. How to read the proposed source

An agreement fixes terms and participants. An intent authorizes outcomes and
limits, rather than arbitrary host computation. A typed hole exposes the finite
choices a solver may fill. A stage relates authenticated `pre` cells to derived
`post` cells in one domain; Core derives the complete ordered effect vector.
An episode relates successive accepted stages and duties across domains.

The proposed contract DSL uses `struct`, tagged `enum`, bounded `fn`, `library`,
`agreement`, `intent`, `stage`, `episode`, `pre`, `post`, `derive`, `authority`,
`evidence`, and exhaustive `match`. Named constructor arguments are mandatory in
financial calls. `=` is immutable binding or a post-state equation; `==` is a
predicate. A `stage` is a relation, not imperative code that writes ledger cells.
Imports name typed profile interfaces with pinned program and policy references;
there is no host `call`, string-evaluated relation or effectful expression.

Source blocks in sections 2–4 define a shared proposed `Horizon` prelude. Family
agreements import it. Profile interfaces and equations adjacent to each family
are part of that library's proposed specification, not assumed existing code.
Identifiers passed to `id`, `program`, `policy`, `key` and `verifier` are nominal claims;
only verified evidence may bind them to bytes, ledger objects or keys.

### Values and identity

```mori
profile "moriarty-horizon/0.1";
library Horizon {
  type UInt128 = bounded_uint(bits: 128);
  type Nominal127 = bounded_uint(bits: 127);
  type Int128 = bounded_int(bits: 128); // -2^127 .. 2^127-1
  nominal type AgreementId; nominal type EpisodeId; nominal type StageId;
  nominal type ActionId; nominal type ProgramId; nominal type PolicyId;
  nominal type ClaimId<D>; nominal type AttemptId<D>; nominal type NativeEffectId<D>;
  nominal type PartyId; nominal type KeyRef<D>; nominal type Nonce<D>;
  nominal type Head<D>; nominal type Digest; nominal type ObjectId<D>;
  nominal type Domain; nominal type Account<D>; nominal type PoolId<D>;
  nominal type InstrumentId<D>; nominal type ShareClassId<D>;
  nominal type Asset<D, Representation, Scale>; // Scale is an integer 0..18
  enum Representation { Canonical, Wrapped(bridge: ProgramId, origin: AssetId) }
  struct Qty<A> { atoms: UInt128 }
  struct Delta<A> { atoms: Int128 } // signed change, not spendable custody
  struct Position<I> { contracts: Int128 } // liability/payoff exposure, not Qty
  struct Shares<S> { atoms: UInt128 } // cannot be passed to Debit<Asset>
  struct Price<Base, Quote, Scale> { mantissa: UInt128 }
  struct Round<D> { value: UInt128 }
  struct Instant<C> { ticks: UInt128 } // clock identity C, no cross-clock cast
  struct Window<C> { from: Instant<C>, through: Instant<C> }
  enum Rounding { Floor(beneficiary: PartyId), Ceil(beneficiary: PartyId), Exact }
  struct Conversion<A,B> { price: Price<B,A,6>, rounding: Rounding }
  struct Certified<T> { value: T, certificate: ArithmeticProof<T>, program: ProgramId }
}
```

`Price<Base,Quote,S>` is **Base units per Quote unit**, preserving U0. The
option's `Price<USD,GOLD,2>(10000)` means 100 USD/GOLD. Swapping these type
arguments fails `H_PRICE_DIRECTION`; a reciprocal needs an explicit certified
conversion and rounding beneficiary. `Round<Foreign>` cannot satisfy a
`Round<Preview>` window (`H_CLOCK_DOMAIN`). `Shares<S>` cannot substitute for
`Balance<USD>` (`H_RESOURCE_KIND`). Same symbol/address bytes on another domain
or representation never imply the same asset/account.

`qty`, exact asset-suffix literals, `atoms`, scalar `+ - *`, `min/max`, `delta`
and `round` are closed pure constructors/operators. Every intermediate checks
its numeric range; negative quantities and unsigned underflow reject formation.
For S0-compatible fields, `narrow127` accepts `0..2^127−1`; the next integer
rejects `H_NOMINAL_RANGE`. Balance/counter storage may reach `2^128−1`.
Delta/Position sign does not authorize a debit or extinguish a liability.
`mul_div` and conversion are outside the simple predicate profile: a bounded
arithmetic program must specify numerator, denominator≠0, intermediate width,
rounding and proof relation. `Certified<T>` is evidence to verify, never a trusted
solver value. Price/clock/calendar/advanced arithmetic remain open.

### Pure functions and ordinary exhaustive match

```mori
library PureIntent {
  enum ReceiptCoverage<A> { Met(received: Qty<A>), Short(missing: Qty<A>) }
  fn coverage<A>(received: Qty<A>, floor: Qty<A>) -> ReceiptCoverage<A>
    bounded_work(6) {
    match received >= floor {
      true => Met(received: received);
      false => Short(missing: floor-received);
    }
  }
  fn missing<A>(received: Qty<A>, floor: Qty<A>) -> Qty<A>
    bounded_work(12) {
    let result = coverage(received: received,floor: floor);
    match result {
      Met(received) => atoms(asset: A,value: 0);
      Short(missing) => missing;
    }
  }
}
const preview_gap: Qty<GOLD> = PureIntent.missing(received: 0.899 GOLD,floor: 0.900 GOLD);
// Result 0.001 GOLD, a diagnostic value; it does not relax the signed floor.
```

`>=` is a same-asset pure comparison; `coverage` guards unsigned subtraction.
Both matches exhaust their finite tags, and the reusable `missing` call is total,
nonrecursive and contains at most twelve abstract expression steps including
its callee. No path runs a loop, reads state, verifies evidence or emits effects.
Its elaboration work bound is distinct from financial allowance/work counters.
The typed source/totality specification and executable tests remain open; this
example is an ordinary pure function proposal rather than a relation signature.

### Typed cells, rights and derived footprints

```mori
library Horizon {
  struct Balance<A> { owner: Account<domain(A)>, amount: Qty<A> }
  struct Custody<A> { object: ObjectId<domain(A)>, amount: Qty<A> }
  struct Allowance<A> { owner: Account<domain(A)>, remaining: Qty<A>, spent: Qty<A> }
  struct Work<D> { remaining: UInt128, spent: UInt128 }
  struct History<D> { head: Head<D>, predecessor: Head<D>, consumed: Set<Nonce<D>,256> }
  struct PolicyState { epoch: UInt128, commitment: Digest, revocation_epoch: UInt128 }
  struct Obligation<A> {
    debtor: Account<domain(A)>, creditor: Account<domain(A)>, principal: Qty<A>,
    accrued: Qty<A>, outstanding: Qty<A>, status: DebtStatus
  }
  enum DebtStatus { Outstanding, Defaulted, Settled }
  struct Encumbrance<A> { claim: ClaimId<domain(A)>, locked: Qty<A>, rank: UInt128 }
  struct Locks<A> { custody: Qty<A>, claims: List<Encumbrance<A>,32> }
  struct Duty<A> { claim: ClaimId<domain(A)>, owed: Qty<A>, terms: Digest,
                   continuation: Set<ActionId,16>, recovery: Set<ActionId,8> }
  struct Cell<D,T> { key: CellKey<D,T> } // reference, not a stored value
  struct Read<D,T> { cell: Cell<D,T>, value: T, head: Head<D> }
  struct Write<D,T> { cell: Cell<D,T>, value: T }
  struct Footprint<D> { reads: Set<CellKey<D>,64>, writes: Set<CellKey<D>,64> }
  enum Effect<D> {
    Debit<A>(owner: Account<D>, amount: Qty<A>), Credit<A>(owner: Account<D>, amount: Qty<A>),
    DebitCustody<A>(cell: Cell<D,Custody<A>>, object: ObjectId<D>,
      authority: CustodyCapability<D,A>, amount: Qty<A>),
    SetCell<T>(cell: Cell<D,T>, value: T), Mint<S>(owner: Account<D>, amount: Shares<S>),
    Burn<S>(owner: Account<D>, amount: Shares<S>), MintAsset<A>(owner: Account<D>, amount: Qty<A>),
    BurnAsset<A>(owner: Account<D>, amount: Qty<A>),
    BurnReservation<S>(claim: ClaimId<D>, amount: Shares<S>),
    UseAllowance<A>(owner: Account<D>, gross: Qty<A>), UseReplay(nonce: Nonce<D>),
    AdvanceHead(from: Head<D>, to: Head<D>)
  }
  struct Derived<D> { ordered: List<Effect<D>,64>, post: List<Write<D>,64>,
                      footprint: Footprint<D>, duties: List<DutyRef,16> }
}
```

Cell constructor names below (`balance`, `pool`, `debt`, `locks`, `withdrawal`,
etc.) are typed `CellKey` constructors owned by the indicated profile, with
nominal object/asset/account arguments. A `pre` entry binds a `Read` of that type;
`post { entry = value; }` names the same cell and type. `post unchanged entry`
expressly frames a read-only cell. Omitted cells outside derived writes are
unchanged. Required cells cannot be silently initialized.

Every mutating **economic ledger stage** includes the common control reads `Allowance` for each
signed debtor/asset, `Work`, `History`, `PolicyState` and grant/revocation state;
common writes consume the actual **gross**, one work unit per accepted stage,
nonce and head. Read-only oracle selection consumes no financial allowance.
An episode has an additional typed `EpisodeJournal` read/write on each economically affected
domain. Observational actions use only the separately bounded service journal;
they inherit none of these ledger writes, nonces or head consumption. These are mandatory expansion rules, shown once to keep examples short.
Every example's `pre/post` displays its financial cells; inspection must display
both these and the common control cells.

A selected profile derives ordered economic effects followed by allowance,
replay and head effects. Its derived read/write key sets must equal the authored
footprint, not just be subsets. Missing, extra or misspelled Fee/reserve/custody
cells reject `H_FOOTPRINT_MISMATCH` or formation `H_UNKNOWN_CELL`. Mutating
profiles may not hide fee recipients, supply, debt or loss writes. Derived post
values must satisfy all equations and unchanged-cell framing. Hashing an authored
footprint is not proof of this equality.

## 2. Signed intent, completion, authority and failure contracts

```mori
library Horizon {
  struct AssetBudget<A> { gross: Qty<A>, fees: Qty<A> }
  enum ReceiptGate { Milestone(stage: StageId), Terminal(branch: OutcomeTag) }
  enum ReceiptScope<D,A> {
    StageDelivery(stage: StageId, claim: ClaimRef),
    ReservedCustody(credit_stage: StageId, claim: ClaimRef, terminal_stage: StageId)
  }
  struct ReceiptRequirement<D,A> {
    gate: ReceiptGate, recipient: Account<D>, asset: A, scope: ReceiptScope<D,A>,
    floor: Qty<A>, deductions: Set<DeliveryChargeSelector<D,A>,16>
  }
  struct AssetBudgetEntry { subject: BudgetSubject, asset: AssetId, bounds: AssetBudget<asset> }
  struct Bounds { assets: List<AssetBudgetEntry,8>, work: UInt128, stages: UInt128 }
  struct ProgramRef<P> { id: ProgramId, source: Digest, policy: Digest }
  struct EvidencePolicy<T> { verifier: VerifierId<T>, source: SourceId<T>,
                            maximum_age: UInt128, finality: Finality }
  enum Finality { Finalized, Confirmations(count: UInt128) }
  struct Finite<T,N> { choices: List<T,N> }
  struct Interval<A> { minimum: Qty<A>, maximum: Qty<A>, step: Qty<A> }
  struct Hole<T,N> { id: HoleId, allowed: Finite<T,N> }
  struct QuantityHole<A,N> { id: HoleId, allowed: Interval<A>, maximum_choices: N }
  struct Completion<P> { values: ClosedHoleRecord<P>, proofs: RequiredCertificates<P> }
  struct Signers<D> { keys: List<KeyRef<D>,8>, threshold: UInt128 }
  enum Role { Issue, Fill, Enforce, Amend, Recover, Sign, Veto, Mint, Burn, Slash }
  struct Grant<D> {
    parent: IntentRef, holder: PartyId, roles: Set<Role,12>,
    actions: Set<ActionId,32>, domain: D, expiry: Round<D>, key_epoch: UInt128,
    revocation_epoch: UInt128, allowance: Bounds, signers: Signers<D>
  }
  struct SignedIntent<P> {
    agreement: AgreementId, policy: PolicyId, policy_digest: Digest,
    selected: List<ProgramRefEntry,32>, issuer: PartyId, signers: SignerSets,
    nonce: NonceSet, validity: DomainWindows, heads: HeadSchema,
    fixed: FixedTerms<P>, holes: ClosedHoleSchema<P>, bounds: Bounds,
    receipts: List<ReceiptRequirementEntry,16>, renewal: Option<RenewalPolicy>,
    observational_scopes: List<ObservationalRuleEntry,16>,
    parties: RoleBindings, grants: List<GrantRef,16>,
    outcomes: OutcomeRelation<P>, evidence: EvidencePolicies<P>, disclosures: DisclosurePolicy
  }
  enum FirstFailure {
    Stage(code: StageError), Intent(code: IntentError), Effect(code: EffectError),
    Authority(code: AuthorityError), History(code: HistoryError), Failure(code: FailureError)
  }
  enum StepOutcome<D> {
    Settled(effects: Derived<D>, evidence: LedgerAcceptance<D>),
    Rejected(first: FirstFailure, effects: Empty, new_duties: Empty),
    Pending(claim: ClaimRef, duties: List<DutyRef,16>, reservation: ReservationRef),
    Recovered(claim: ClaimRef, effects: Derived<D>, evidence: RecoveryEvidence<D>)
  }
  struct OutcomeRelation<P> { alternatives: ClosedTaggedRelations<P> }
  struct Retained<P> { effects: AcceptedHistory<P>, duties: List<DutyRef,16> }
}
```

All schema fields are source-fixed except explicitly declared holes and provider
facts. Hole enumeration and arithmetic work have finite counters; a quantity
hole needs `step>0` and `1+(maximum-minimum)/step ≤ maximum_choices`. No hole
changes recipients, asset identity, program, policy, grant or signed alternative.
`refines(intent,completion,pre,derived,outcome)` requires **all** source terms,
finite choices, complete footprint, authority, per-asset cumulative bounds,
evidence and outcome branch to hold. Failure is not an alternate evaluator.

For each budget subject S and asset A, `G_(S,A)` is cumulative authorized
spending from that subject, and `F_(S,A)` its cumulative fees, across **every**
accepted/retained effect under the same logical claim. On every prefix require
`G_(S,A)≤gross_(S,A)` and `F_(S,A)≤fees_(S,A)`. Credits, mint receipts, refunds,
branch selection and attempt changes never reduce these counters. A budget
without `subject` names the source-fixed initiating owner. Custody debits have
separate source-bound resource capabilities/bounds and are not a second wallet
debit. `bounds` contains no receipt floor and no branch override.

`receipts` is a separate, source-signed list. A Milestone requirement is checked
at its named accepted stage and its certificate remains an historical fact; it
is not a promise to keep the asset forever. A Terminal requirement is checked
only when its exact signed outcome tag is selected. A different terminal branch
cannot waive a matching requirement or inherit an unrelated branch's floor.
Every eligible requirement must pass; no constructor name implicitly supplies,
replaces or disables a receipt obligation.

For StageDelivery, net is the sum of fresh Credit/MintAsset effects to the exact
recipient/asset in that stage/claim, less **all** fees and debits charged to that
delivery. Deduction selectors are signed, exhaustive bindings to ordered effects
or their bounded delivery legs; the complete-effect checker rejects an omitted
charge. Another-asset fee remains in its cumulative fee budget; subtracting it
from this receipt needs an explicit signed certified conversion. Existing wallet
balances never count. A later independently authorized loan repayment or coin
burn is not a deduction from an already certified origination/mint milestone.
For ReservedCustody, net is fresh claim-bound credits/mints less every accepted
spend from that reserved claim through the named terminal stage. Qualification
must prove the surviving amount is spendable by the named recipient and no
unresolved send can consume it. Unrelated old balances cannot fill a shortfall.
These are receipt scopes, not whole-episode profit. A whole-life cash delta would
need a distinct signed Delta-valued predicate and is not silently inferred here.

Grant allowance and work never exceed parent residual bounds. Changing an
attempt ID cannot reset counters, residual duties or claim identity.

Formation errors have no Core term. Proposed semantic failures retain the S0
judgment order `Stage → Intent → Effect → Authority → History → Failure`; each
profile declares ordered error constructors within each judgment below. The
first failing predicate determines `FirstFailure`; a rejected stage publishes
no new financial effect/post-head/duty. Previously accepted episode stages remain
visible. Missing evidence may yield `Pending` only where the signed parent
already permits it and preserves its reservation and duty; it cannot turn an
invalid stage into accepted partial execution. Unknown finality and timeout
retain their duties. A terminal recovery branch must prove its own complete
relation before publishing effects.

Authorities are verified grants, not a name for trust. Issue signs the exact
parent intent; Fill only proposes permitted holes; Enforce verifies every
relation; Sign binds the exact payload/domain/recipient/value/fee/nonce/key epoch;
Amend/Veto/Recover require their separate granted scope and signers. Expiry,
revocation epoch and policy epoch are same-head authenticated facts. Multi-signer
thresholds are bounded, with distinct keys and no duplicated votes. Revocation
of future authority does not discharge already created duties.

### Signed continuation heads and conjunctive authorization

```mori
library Horizon {
  enum HeadRule<D> {
    Initial(anchor: Head<D>),
    PreviousAcceptedHead(agreement: AgreementId, claim: ClaimRef,
      predecessor: StageId, fact: AcceptedEconomicFactSelector<D>)
  }
  struct StageHead<D> { stage: StageId, rule: HeadRule<D> }
  struct HeadSchema { stages: List<StageHeadEntry,32>, unrelated_progress: RequiresFreshAuthorization }
  struct DeferredFact<T> { supplied: Option<RawEvidence<T>> }
  enum HeadFacts<D> {
    Economic(rule: HeadRule<D>, binding: DeferredFact<ResolvedHeadBinding<D>>),
    Observational(rule: ObservationalRule<D>, checkpoint: DeferredFact<ObservationAtCheckpoint<D>>)
  }
  struct ObservationalRule<D> {
    action: ActionId, query: QueryId<D>, selector: SignedSelector,
    checkpoints: SelectedVerifierFinalizedCheckpoints<D>,
    original_facts: Set<AcceptedEconomicFactSelectorEntry,8>, // selectors, not assertions of acceptance
    service_work: UInt128, ledger_effects: Empty, ledger_controls: Empty
  }
  struct AuthorizationConjunction<D,P> {
    parent: DeferredFact<IntentSignatureSet<P>>, grant: DeferredFact<GrantAuthorizationSignatures<D>>,
    resource: DeferredFact<ResourceCapabilitySignatures<D>>, native: DeferredFact<NativePayloadSignatures<D>>,
    epoch: DeferredFact<KeyEpochBinding<D>>, revocation: DeferredFact<RevocationAtHead<D>>,
    expiry: DeferredFact<GrantExpiryFacts<D>>, heads: HeadFacts<D>
  }
  struct RenewalPolicy {
    actors: SignerSets, permitted: Set<ActionId,8>,
    preserve: OriginalClaimTermsBudgetsReceiptsAndHistory,
    validity: FreshSourceSignedWindow, head_change: FreshSourceSignedAnchor
  }
}
```

Each action/query has exactly one source-signed observational rule; duplicate
action/query entries reject formation. Its selector fixes the verifier, domain,
paired claim and finite attempt coverage, not a caller-selected convenient head.
Original-fact selectors permit a deferred optional original Claim edge: absence
is neither an accepted edge nor final nonreceipt.

Every accepted **economic ledger stage** resolves exactly one signed HeadRule. Initial uses its exact
anchor. PreviousAcceptedHead uses the authenticated successor of the **named
accepted economic predecessor**, in the same agreement, claim and domain. The
fact includes its StageId, parent digest, complete accepted effects and head
edge. It cannot select the latest convenient head. A rejected stage has no edge;
Kernel observation/service-journal records do not provide economic predecessors.
The ledger must compare resolved head to current head and consume nonce/head
atomically. An unrelated intervening head makes the stage stale; continuation
requires fresh source authorization preserving the original promise and counters.
Completion and AttemptId never choose or replace the head. Reconciliation may
qualify an original finalized fact but cannot send/mint it again or replace its
original pre/post edge with a current unrelated one.

Authorization is a conjunction, not a choice of signatures. The issuer's parent
envelope signature authorizes the economic promise and grant references. Each
grant's exact referenced authorization statement (parent digest, domain, roles,
actions, residual limits, expiry, key/revocation epoch and resource capability)
requires all its declared signers. A resource authority separately signs the
mint/custody/recovery capability statement. Every native payload requires all
signers selected by the grant and adapter capability, over exact decoded
recipient/value/fee/domain/nonce/payload digest. These statements have distinct
types; a signature on one cannot substitute for another. Required signature sets, key epochs, revocations, scope, expiry and the applicable
head/checkpoint binding must all hold together. Their raw facts are collected
without early authorization rejection; the shared judgment schedule below
qualifies each fact at its assigned judgment. Additional resource cosignatures narrow execution; they do not
change or replace the issuer's parent signature or raise any allowance.

If the parent authorization window expires, Intent Validity fails unless an
earlier Stage predicate fails. If the parent window is still valid but the grant
expires, Authority ExpiredAuthorization fails only after Stage, Intent and Effect
pass. Neither case publishes new effects; the original reservation and duty remain
Pending. A renewal is a **new signed authority**, not a completion
field or inferred extension: permitted actors sign a new window, current keys,
revocation facts and (if needed) a fresh explicit head anchor. It must reference
the original parent digest/economic claim, immutable terms/programs/policies,
branch receipts, every accepted effect and already spent gross/fees/work. The renewal authenticates the original envelope and accepted facts at their
original authorization times; it does not require an expired old grant to become
currently valid. The new continuation envelope supplies current window/key/head
authority, while the original promise supplies immutable economic obligations.
Only remaining original economic authority is available. It cannot create a new logical claim,
reset counters, change recipients/floors, or discharge a duty without its original
qualified outcome. A duty's terms may remain epoch3 while authority uses newly
qualified epoch4 keys; both bindings are checked explicitly. Without those new
signatures, late proof remains evidence for reconciliation and never authority
to refund. Availability and canonical renewal encoding/proofs remain open.

## 3. Kernel interface: exact authority and evidence at every call

The optional kernel implements concrete side effects and returns receipts. Core
owns financial acceptance. The [kernel recommendation](../kernel-api-recommendation-2026-09-29.md)
is itself a proposal; the following interfaces do not claim adopted APIs.
There is one Moriarty authority envelope; canonical signing bytes remain an
M4-C1/C5 obligation, not a new mockup-specific codec.

```mori
library Horizon {
  nominal type QueryId<D>;
  enum ScopeIdentity<D> {
    Economic(claim: ClaimRef, stage: StageId),
    Query(query: QueryId<D>, selector_digest: Digest)
  }
  struct ScopeRequest<D,P> { intent: SignedIntent<P>, grant: GrantRef<D>,
    action: ActionRef<D,P>, identity: ScopeIdentity<D>, attempt: DurableAttempt<D>,
    facts: AuthorizationConjunction<D,P> }
  enum ScopeQualificationKind { Planning, Signing, NativeComplete, Observational, Recovery }
  // ScheduledScopeQualification is opaque verifier output, never caller-constructible;
  // each Kernel method checks its required kind and exact still-qualified premises.
  struct KernelScope<D,P> { intent: SignedIntent<P>, grant: VerifiedGrant<D>,
    identity: ScopeIdentity<D>, attempt: AttemptId<D>, qualification: ScheduledScopeQualification<D,P> }
  fn scopeFor<D,P>(stage: ActionRef<D,P>, intent: SignedIntent<P>, grant: GrantRef<D>,
    identity: ScopeIdentity<D>, attempt: DurableAttempt<D>,
    evidence: AuthorizationConjunction<D,P>) -> ScopeRequest<D,P>;
  struct EvidenceBundle<D,P> {
    intent_signature: IntentSignature<P>, selected_program: ProgramBinding<P>,
    policy: PolicyBinding, snapshot: DeferredFact<SnapshotAtHead<D>>,
    reads: DeferredFact<AuthenticatedReads<D,P>>, observations: ProfileEvidence<P>,
    revocations: DeferredFact<RevocationAtHead<D>>, predecessor: HeadFacts<D>,
    authorization: AuthorizationConjunction<D,P>
  }
  struct NativeCall<D,P> { codec: NativeCodecId<D>, payload: Bytes<4096>,
                          decoded: DecodedPlan<D,P>, digest: Digest }
  struct ServiceQueryFact<D> { query: QueryId<D>, attempt: AttemptId<D>,
    parent_digest: Digest, selector_digest: Digest, receipt_digest: Digest }
  struct ServiceQueryJournal<D> { facts: List<ServiceQueryFact<D>,256>,
    attempts: List<QueryAttemptFact<D>,8>, work_spent: UInt128,
    authentication: DeferredFact<ServiceJournalBinding<D>> }
  struct Receipt<D,T> { identity: ScopeIdentity<D>, intent_digest: Digest,
    grant: GrantRef<D>, attempt: AttemptId<D>, fact: T }
  enum SendFact<D> { NotSent, Submitted(native: NativeEffectId<D>), Unknown }
  enum ObserveFact<D,P> { Final(evidence: LedgerEvidence<D,P>), Pending,
                          Conflicting(evidence: ConflictEvidence<D,P>) }
  enum RecoveryFact<D,P> { Qualified(evidence: RecoveryEvidence<D,P>), Unresolved }
  interface Kernel {
    validatePlan<D,P>(scope: KernelScope<D,P>, plan: Completion<P>,
      prepared: CorePrepared<D,P>, evidence: EvidenceBundle<D,P>) -> Receipt<D,ValidatedPlan<P>>;
    reserve<D,P>(scope: KernelScope<D,P>, plan: ValidatedPlan<P>,
      evidence: ReservationEvidence<D>) -> Receipt<D,Reservation<D>>;
    buildCall<D,P>(scope: KernelScope<D,P>, plan: ValidatedPlan<P>,
      evidence: AdapterBinding<D,P>) -> Receipt<D,NativeCall<D,P>>;
    authorizeSign<D,P>(scope: KernelScope<D,P>, call: NativeCall<D,P>,
      evidence: PayloadCorrespondence<D,P>) -> Receipt<D,PayloadSignature<D>>;
    dispatch<D,P>(scope: KernelScope<D,P>, call: NativeCall<D,P>,
      evidence: SignedReservedCall<D,P>) -> Receipt<D,SendFact<D>>;
    observe<D,P>(scope: KernelScope<D,P>, selector: ObservationSelector<D,P>,
      evidence: ProviderBinding<D,P>) -> Receipt<D,ObserveFact<D,P>>;
    reconcile<D,P>(scope: KernelScope<D,P>, history: AcceptedHistory<P>,
      evidence: DeliveryCountEvidence<D,P>) -> Receipt<D,ObserveFact<D,P>>;
    recover<D,P>(scope: KernelScope<D,P>, branch: RecoveryBranch<P>,
      evidence: RecoveryEvidence<D,P>) -> Receipt<D,RecoveryFact<D,P>>;
  }
}
```

For EvidenceBundle, `predecessor` uses exactly the same Economic/Observational
HeadFacts variant as authorization. Economic snapshots/reads remain deferred until
their Stage checks; observation uses its signed query checkpoint, with no fictional
economic predecessor. The required evidence variant is fixed by the signed action.

`Grant`, certificate, signature and evidence constructors above are typed
requirements, not public constructors that can forge verification. Each profile-specific proof/evidence name is an opaque parameterized type with
the binding predicates stated in its profile section. A literal nominal name
selects a requirement, not a proof constructor. A provider returns raw evidence; a selected verifier establishes these qualified types.
Each family names its `authority` and `evidence` inputs. Its `kernel protocol`
clause applies this finite typed call sequence, with that exact authority:

```mori
protocol OneDomain<D,P>(stage: Stage<D,P>, request: ScopeRequest<D,P>,
                        e: EvidenceBundle<D,P>) {
  let preparation = Core.prepare(stage: stage, completion: stage.completion,
    scope_request: request, evidence: e); // shared Stage→Intent→Effect→Authority→History→Failure
  let (candidate,scope) = preparation.readyOrPublishFirstRejection();
  let checked = Kernel.validatePlan(scope: scope, plan: stage.completion,
    prepared: candidate, evidence: e);
  let reserved = Kernel.reserve(scope: scope, plan: checked.fact,
    evidence: verifyReservation(scope: scope, snapshot: candidate.authenticated_snapshot, bounds: scope.intent.bounds));
  let call = Kernel.buildCall(scope: scope, plan: checked.fact,
    evidence: verifyAdapter(scope: scope, selected: e.selected_program));
  let signingCheck = Core.qualifySigning(stage: stage, candidate: candidate, scope: scope,
    call: call.fact, fresh_evidence: e); // shared schedule, native signature remains pending output
  let signing = signingCheck.readyOrPublishFirstRejection();
  let sig = Kernel.authorizeSign(scope: signing.scope, call: call.fact,
    evidence: signing.payload_correspondence);
  let nativeCheck = Core.qualifyNative(stage: stage, candidate: candidate, scope: signing.scope,
    call: call.fact, signature: sig.fact, reservation: reserved.fact, fresh_evidence: e);
  let native = nativeCheck.readyOrPublishFirstRejection(); // same ordered judgments, all native premises now present
  let sent = Kernel.dispatch(scope: native.scope, call: call.fact,
    evidence: native.signed_reserved_call);
  let observed = Kernel.observe(scope: native.scope, selector: effectSelector(stage: stage, send: sent.fact),
    evidence: verifyProvider(scope: native.scope, policies: scope.intent.evidence));
  match observed.fact {
    Final(evidence) => Core.accept(stage: stage, completion: stage.completion,
      authority: native.scope.grant, evidence: qualify(scope: native.scope, bundle: e, ledger: evidence));
    Pending => Pending(claim: economicClaim(scope.identity), duties: stage.signed_pending, reservation: reserved.fact);
    Conflicting(evidence) => Pending(claim: economicClaim(scope.identity), duties: stage.signed_pending,
      reservation: reserved.fact); // conflict is retained evidence, not success
  }
}
```

### Shared scheduled validation, including deferred authority facts

`scopeFor` is a total bounded collector over well-formed typed inputs. It records
parent/grant/action/identity/attempt and raw evidence; it does not qualify a grant,
compare expiry/head, return FirstFailure, or call a kernel. Malformed syntax,
unknown fields/tags and resource bounds are formation errors outside Core.
For a well-formed request, missing/invalid facts are retained in DeferredFact and
passed to Core; no caller may supply a trusted verification flag.

One scheduler is used by direct Core.prepare, OneDomain, observe, reconcile and
recover. It visits Stage→Intent→Effect→Authority→History→Failure, with each
profile's ordered predicates inside that judgment:

| Judgment | Checks in the proposed common schedule |
| --- | --- |
| Stage | Versions/profile/typed cell shape, selected program/policy and authenticated snapshot; observation request schema/provider/current authority-clock, or returned checkpoint proof in the response term; exact parent and action/claim/query structural binding. Missing required Stage proof fails here. |
| Intent | Signed fixed terms, parent validity, hole/refinement scope, recipients/assets and cumulative gross/fee caps; for observations, the signed selector and zero-effect promise. |
| Effect | Checked arithmetic, full proposed effects/post/footprint/receipt/duty relation; for observational actions, exactly no economic/ledger-control mutation and correct bounded fact query. |
| Authority | Exact grant/resource signature sets, roles/scopes, current allowance/work, key epochs/revocation and grant expiry. Raw native signatures are checked here once their exact native payload exists. |
| History | Economic resolved HeadRule/current-head/replay/atomic reservation facts; observational signed checkpoint/query/attempt/service-work journal binding with **no economic head consumption**. |
| Failure | Selected signed outcome and retained-duty/qualified recovery relation; earlier accepted history remains visible. |

Cryptographic parent/source/snapshot facts needed to identify the Stage are
qualified at Stage; grant authorization predicates remain Authority checks.
An authenticated grant's expired date is not a formation or Stage error.
Native-payload signatures cannot exist before native bytes are built: preparation
may return a candidate and qualified **planning** scope with that named premise
still pending. It cannot return a dispatch-authorized scope or ledger success.
After buildCall, the exact decoded payload, fresh authority facts and signatures
reenter the same schedule. Native signing requires the exact parent/grant/resource signing authority,
payload correspondence, expiry/revocation and history qualified by the shared
schedule. The requested native signature is an explicit pending output, never an
assumed verified fact. Dispatch requires its returned exact native signatures to
pass Core.qualifyNative in the same schedule, conjunctively with all the other
authority facts. Neither planning nor signing alone establishes dispatch authority. Unknown/absent facts are never treated as true.
Any new mutation invalidates the corresponding previous qualification and
reschedules it; no client entry point reports a later failure before an earlier
failed predicate. The schedule's cost/operational proof remains open.

`readyOrPublishFirstRejection` unwraps only Core's scheduled preparation result.
On rejection it publishes null new effects/post/head/duties and stops before
kernel calls; it is not scopeFor.requireOk. On preparation it yields the derived
candidate and qualified planning scope plus explicit remaining native/finality/
atomic-commit premises. Kernel validatePlan checks adapter feasibility against
that candidate and may narrow it, never replace Core's economic relation.
`verifyReservation` atomically reserves the exact residual allowance, work and
claim attempt; `verifyAdapter` binds code/policy/codec; `verifyPayload` independently
decodes and compares all signed fixed/hole fields. These helpers collect/verify
facts for the common schedule; none overrides its first-failure observation.
`bindSignedReservation` binds signature, reservation and exact native bytes to
one identity/attempt. `qualify` supplies successor/finality/compare-and-consume
facts for Core.accept under that same complete relation. Receipts supply facts,
not caller-constructed qualification.

`kernel OneDomain(authority:G,evidence:E)` builds ScopeRequest via scopeFor and
passes it to OneDomain. `kernel recover(authority:G,evidence:E)` first calls the
same Core.prepare schedule on its signed economic recovery stage/request. Its
Planning output does **not** enter Kernel.recover. Core.qualifyRecovery reruns the
shared schedule on that exact candidate, signed branch and complete recovery
evidence, returning an opaque Recovery scope for the proof service described below.
The subsequent financial writes use OneDomain and its separate Signing and
NativeComplete qualification. Observation sugar (`kernel observe`
or `kernel reconcile`) instead calls Core.prepareObservation on the signed
ObservationalRule/request/evidence, with the same ordered judgments above and
empty financial/control effects. Only its qualified observational scope may
enter Kernel.observe/reconcile. Read-only query qualification is never economic
acceptance or permission to spend the corresponding claim.

Every Kernel method retains its typed KernelScope parameter. This scope is
created only by the scheduler, not by scopeFor or by a caller grant. Its opaque
qualification has a checked kind: validatePlan/reserve/buildCall require Planning;
authorizeSign requires Signing after exact payload correspondence is scheduled;
dispatch requires NativeComplete; a Query observe/reconcile requires Observational;
recover requires Recovery. Economic post-send observation retains NativeComplete
and its source-signed effect selector, rather than masquerading as a free Query.
Dispatch
requires a native-complete qualification; observation requires the signed
checkpoint/query qualification; recover requires the signed economic branch
qualification. The exact scope identity, parent digest, grant and durable
attempt remain in every receipt. No premature `.requireOk()` is used on collected
authority facts. A kernel refusal preserves the direct Midnight path and does
not reinterpret the signed outcome.

All source, types, expressions, enumerations, cells, certificates and protocol
steps are bounded. Proposed initial limits reuse beta's 65536 source bytes,
8192 tokens/nodes, depth64, declarations256, fields/arguments64, identifiers64,
strings1024, expression edges32768. Horizon additionally bounds stages32,
assets8, locks32, signatures8, outcomes16, holes16/choices64 per hole,
effects64/reads64/writes64 per stage, proof bytes65536, journal entries256.
Exceeding any limit rejects formation with `H_RESOURCE_BOUND`, with no partial
artifact. A journal cannot discard unsettled duties at its bound: further
continuation requires a separately proved bounded history commitment.

## 4. Common agreement context and transition notation

```mori
context Examples {
  domain Preview = domain(id: "Midnight", chain: "midnight", network: "preview");
  domain Foreign = domain(id: "Foreign", chain: "example", network: "test");
  party Owner = party(id: "owner-party"); party Creditor = party(id: "creditor-party");
  party Reserve = party(id: "reserve-party"); party Quorum = party(id: "quorum-party");
  party Keeper = party(id: "keeper-party"); party Provider = party(id: "provider-party");
  account Alice: Account<Preview> = account(domain: Preview, id: "Alice", party: Owner);
  account Bob: Account<Preview> = account(domain: Preview, id: "Bob", party: Creditor);
  account Treasury: Account<Preview> = account(domain: Preview, id: "Treasury", party: Reserve);
  account AliceF: Account<Foreign> = account(domain: Foreign, id: "AliceF", party: Owner);
  account TreasuryF: Account<Foreign> = account(domain: Foreign, id: "TreasuryF", party: Reserve);
  asset USD = asset(domain: Preview, id: "USD", representation: Canonical, scale: 2);
  asset GOLD = asset(domain: Preview, id: "GOLD", representation: Canonical, scale: 3);
  asset MUSD = asset(domain: Preview, id: "MUSD", representation: Canonical, scale: 2);
  asset WUSD = asset(domain: Foreign, id: "WUSD",
    representation: Wrapped(bridge: id("BridgeV1"), origin: USD), scale: 2);
  asset GOLD_F = asset(domain: Foreign, id: "GOLD_F", representation: Canonical, scale: 3);
  clock PreviewClock = ledger_round(domain: Preview);
  clock ForeignClock = ledger_round(domain: Foreign);
  policy Terms = policy(id: "Terms", epoch: 3, commitment: digest("terms-v3"));
  key k1: KeyRef<Preview> = key(domain: Preview,id: "governor1");
  key k2: KeyRef<Preview> = key(domain: Preview,id: "governor2");
  key k3: KeyRef<Preview> = key(domain: Preview,id: "governor3");
  key v1: KeyRef<Preview> = key(domain: Preview,id: "veto1");
  key v2: KeyRef<Preview> = key(domain: Preview,id: "veto2");
  key v3: KeyRef<Preview> = key(domain: Preview,id: "veto3");
  key OwnerPreviewKey: KeyRef<Preview> = key(domain: Preview,id: "owner-preview");
  key ReservePreviewKey: KeyRef<Preview> = key(domain: Preview,id: "reserve-preview");
  key OwnerForeignKey: KeyRef<Foreign> = key(domain: Foreign,id: "owner-foreign");
  key ReserveForeignKey: KeyRef<Foreign> = key(domain: Foreign,id: "reserve-foreign");
  key_binding Owner = { Preview: (OwnerPreviewKey,3), Foreign: (OwnerForeignKey,3) };
  key_binding Reserve = { Preview: (ReservePreviewKey,3), Foreign: (ReserveForeignKey,3) };
  const FeedA: SourceId<Price<USD,GOLD,2>> = source("FeedA");
  const FeedB: SourceId<Price<USD,GOLD,2>> = source("FeedB");
  const FeedC: SourceId<Price<USD,GOLD,2>> = source("FeedC");
  const WindowP = window(clock: PreviewClock, from: 100, through: 300);
  const WindowF = window(clock: ForeignClock, from: 100, through: 400);
  const Feed = EvidencePolicy<Price<USD,GOLD,2>>(
    verifier: verifier("GoldFeedV1"), source: source("GoldFeed"),
    maximum_age: 5, finality: Finalized);
}
```

Every agreement below imports `Horizon` and `Examples`. Its header declares a
nominal agreement ID, policy Terms, validity WindowP, a pinned profile
`ProgramRef<P>(id,source_digest,policy_digest)`, and fresh action-specific
nonces/pre-heads. `header(profile: P, id: X, actions: [A...])` is a proposed
source abbreviation with this **explicit expansion contract**: source-fixed
agreement X, Terms epoch3/digest terms-v3, WindowP, selected P/source `P-v1`/policy
terms-v3, nonce `(X,A,1)`, key epoch3, initial pre-head supplied as a fixed authored
`Head<Preview>(X,0)`, continuation heads bound by the signed
`PreviousAcceptedHead` rule, disclosures None, renewal None, observational_scopes empty unless explicitly
declared, stage limit equal to action count. Only economic ledger stages receive
a StageHead; declared observation actions use their signed ObservationalRule. Receipt
requirements are always the explicit `receipts` list in each intent; no header
default converts a budget into a receipt promise.
The continuation rule names the exact accepted predecessor in this agreement
and domain; it cannot select an unrelated or stale head. This symbolic binding
is a proposed episode feature beyond Source/6's single fixed `pre_head`, with
a separate history/refinement proof obligation. These opaque IDs are fixtures;
they do not define canonical signed bytes. A
real author supplies actual head/key bindings. Every action in the list must
have an explicit grant, allowed terms, budget and outcome. No unnamed stage can
be authorized by this abbreviation. Foreign stages state their separate header.

`authority { A: grant(...) }` expands the full Grant fields in section 2:
parent is this intent; domains/roles/holder/actions are given; expiry is the
corresponding Window endpoint; key/revocation epoch3; allowance cannot exceed
parent bounds; signer threshold is shown. The authenticated grant and all its
fields are required evidence. `owners([Owner])` means the single distinct key
bound to Owner in the stage's domain/key epoch, threshold1; `owners([Owner,Creditor])` means both distinct bound
keys are required (threshold equals list length); `quorum(keys:[k1,k2,k3],threshold:2)` means exactly
those source-bound key refs, with no duplicated keys. Finite Fill grants carry
zero debit authority unless separately granted.

A stage's `operation` is a closed typed profile constructor, `derive` names its
pinned pure/certified relation and `post` states its equations. `effects` names
the exact ordered economic effect constructors before the common control suffix.
`writes(pre.x,...)` and `reads(pre.x,...)` are typed footprint constructors,
not strings. `evidence` binds the named profile facts to the same stage snapshot.
`kernel OneDomain(authority: G, evidence: E)` expands to section 3's protocol
with ScopeRequest from `scopeFor(stage: currentStageRef, intent: thisStage.parent,
grant: G, identity: thisStage.signedIdentity, attempt: journal.currentDurableAttempt,
evidence: E.authorization)`. This collection does not return a semantic failure;
Core.prepare qualifies it in the shared schedule. OneDomain requires Economic
identity. Ordinary economic identities derive from fixed agreement/episode/
object/StageId; bridge/composition use the fixed Pair. Observation identities are
Query with a source-fixed query/selector and optional paired economic subject,
never Economic merely because their evidence concerns custody. Unknown authority
facts are deferred until their judgment; no caller manufactures VerifiedGrant.

`failures` lists ordered Stage/Intent/Effect/... profile predicates in addition
to common qualification/scope/authority/history/failure checks. `reject(first)`
has no published stage effects. `pending(duty)` is an explicitly signed branch.

## 5. AMMs and exchanges

**Desired outcome:** exchange exactly 100 USD plus at most 0.30 USD fee for at
least 0.900 GOLD to Alice. LP issuance/redemption is a different operation on a
share resource. **Status:** all source/profile/evidence below specified; execution,
invariant proof and custody/capability qualification open.

Proposed profile types and equations:

```mori
library AMM {
  struct PoolState<A,B> { reserve_a: Qty<A>, reserve_b: Qty<B>,
                         custody_a: Qty<A>, custody_b: Qty<B>, fee_bps: UInt128 }
  struct LPState<S> { supply: Shares<S>, owner: Shares<S> }
  struct SwapPlan<A,B> { output: Qty<B>, fee: Qty<A>, proof: AMMInvariantProof<A,B> }
  // ExactInput: x=input, f=fee, y=output; product is checked/certified at 256 bits.
  // ra'=ra+x; rb'=rb-y; custody mirrors reserve changes; fee is outside reserve.
  // require f=ceil(x*fee_bps/10000), (ra+x)*(rb-y)>=ra*rb, y>=floor,
  //        y<=rb, gross=x+f, conservation incl Treasury fee and Alice net receipt.
  relation ExactInput<A,B>(pre: PoolState<A,B>, input: Qty<A>, plan: SwapPlan<A,B>)
    -> Certified<PoolState<A,B>>;
  // Mint: qa, qb deposited simultaneously; s=floor(min(qa*S/ra,qb*S/rb));
  // excess/dust remains in pool for Reserve. Initial liquidity is separate profile.
  relation MintLP<A,B,S>(pre: PoolState<A,B>, lp: LPState<S>, a: Qty<A>, b: Qty<B>)
    -> Certified<(PoolState<A,B>,LPState<S>)>;
  // Redeem: burn s; qa=floor(ra*s/S), qb=floor(rb*s/S); remaining dust to Reserve.
  relation RedeemLP<A,B,S>(pre: PoolState<A,B>, lp: LPState<S>, burn: Shares<S>)
    -> Certified<(PoolState<A,B>,LPState<S>,Qty<A>,Qty<B>)>;
}
```

```mori
agreement Exchange {
  header(profile: AMM, id: Exchange, actions: [Swap,MintLP,RedeemLP]);
  pool Spot: PoolId<Preview> = id("Spot");
  share_class LP: ShareClassId<Preview> = id("SpotLP");
  intent Trade {
    fixed { owner: Alice; recipient: Alice; fee_to: Treasury; pool: Spot;
            input: 100.00 USD; output_asset: GOLD; net_floor: 0.900 GOLD; }
    holes { fee: QuantityHole<USD,31>(id: Fee, allowed: interval(0.00 USD,0.30 USD,0.01 USD));
            output: QuantityHole<GOLD,64>(id: Output, allowed: interval(0.900 GOLD,0.963 GOLD,0.001 GOLD)); }
    bounds { USD: budget(gross: 100.30 USD, fees: 0.30 USD);
             GOLD: budget(gross: 0.000 GOLD, fees: 0.000 GOLD); work: 1; }
    receipts [receipt(gate: Terminal(settled),recipient: Alice,asset: GOLD,
      scope: StageDelivery(Swap,claim(Exchange)),floor: 0.900 GOLD,deductions: allDeliveryCharges(Swap,GOLD))];
    authority { Swap: grant(holder: Owner, roles: [Enforce,Sign], signers: owners([Owner]));
                Fill: grant(holder: Keeper, roles: [Fill], signers: owners([Owner])); }
    evidence { pool: PoolAtHead(Spot); arithmetic: AMMInvariantProof(Spot);
               selected: program(AMM); custody: PoolCustody(Spot); }
    outcomes { settled: NetReceipt(owner: Alice, asset: GOLD, minimum: 0.900 GOLD);
               rejected: reject(first); pending: None; }
  }
  stage Swap on Preview uses Trade {
    operation amm.swap_exact_input(pool: Spot, owner: Alice, input: 100.00 USD,
      output_asset: GOLD, net_floor: 0.900 GOLD, fee_cap: 0.30 USD);
    completion { output: Trade.holes.output; fee: Trade.holes.fee; proof: AMMInvariantProof(Spot); }
    pre { owner_usd: balance(Alice,USD); owner_gold: balance(Alice,GOLD);
          fee: balance(Treasury,USD); pool: pool(Spot,USD,GOLD); }
    derive AMM.ExactInput(pre.pool, Trade.fixed.input, completion) as next;
    post { owner_usd = pre.owner_usd - 100.00 USD - completion.fee;
           owner_gold = pre.owner_gold + completion.output; fee = pre.fee + completion.fee;
           pool = next.value; }
    effects [Debit(Alice,100.00 USD+completion.fee), Credit(Spot,100.00 USD),
             Credit(Treasury,completion.fee), Debit(Spot,completion.output), Credit(Alice,completion.output),
             SetCell(pre.pool,next.value)]; // pool ID maps to its typed custody account
    footprint reads(pre.owner_usd,pre.owner_gold,pre.fee,pre.pool)
              writes(pre.owner_usd,pre.owner_gold,pre.fee,pre.pool);
    authority Trade.Swap; evidence Trade.evidence;
    kernel OneDomain(authority: Trade.Swap, evidence: Trade.evidence);
    failures Stage[PoolCustodyMismatch] Intent[OutputFloor,FeeCap]
             Effect[Invariant,ArithmeticRange,CompleteEffects]; outcome reject(first);
  }
  intent Liquidity {
    fixed { owner: Alice; pool: Spot; share_class: LP; deposit_a: 10.00 USD;
            deposit_b: 0.100 GOLD; burn: shares(LP,100); rounding: Floor(Reserve); }
    holes {}; bounds { USD: budget(gross: 10.00 USD,fees: 0.00 USD);
      GOLD: budget(gross: 0.100 GOLD,fees: 0.000 GOLD); work: 2; }
    receipts []; // share entitlements are bound by MintLP/RedeemLP relations, not cash receipt floors
    authority { MintLP: grant(holder: Owner, roles: [Sign,Mint], signers: owners([Owner]));
      RedeemLP: grant(holder: Owner, roles: [Sign,Burn], signers: owners([Owner])); }
    evidence { pool: PoolAtHead(Spot); custody: PoolCustody(Spot); shares: LPSupplyAtHead(LP);
      capability: LPAuthority(LP); arithmetic: LPConversionProof(Spot,LP); }
    outcomes { settled: LiquidityConserved(pool: Spot, shares: LP); rejected: reject(first); pending: None; }
  }
  stage MintLP on Preview uses Liquidity {
    operation amm.mint(pool: Spot, owner: Alice, deposit_a: 10.00 USD, deposit_b: 0.100 GOLD);
    pre { a: balance(Alice,USD); b: balance(Alice,GOLD); pool: pool(Spot,USD,GOLD); lp: lp(LP,Alice); }
    derive AMM.MintLP(pre.pool,pre.lp,10.00 USD,0.100 GOLD) as next;
    post { a=pre.a-10.00 USD; b=pre.b-0.100 GOLD; pool=next.value.0; lp=next.value.1; }
    effects [Debit(Alice,10.00 USD),Debit(Alice,0.100 GOLD),Credit(Spot,10.00 USD),
      Credit(Spot,0.100 GOLD),Mint(Alice,next.value.1.owner-pre.lp.owner),SetCell(pre.pool,next.value.0)];
    footprint reads(pre.a,pre.b,pre.pool,pre.lp) writes(pre.a,pre.b,pre.pool,pre.lp);
    authority Liquidity.MintLP; evidence Liquidity.evidence;
    kernel OneDomain(authority: Liquidity.MintLP,evidence: Liquidity.evidence);
    failures Effect[ZeroSupply,ShareRounding,CompleteEffects]; outcome reject(first);
  }
  stage RedeemLP on Preview uses Liquidity {
    operation amm.redeem(pool: Spot, owner: Alice, share_atoms: 100);
    pre { a: balance(Alice,USD); b: balance(Alice,GOLD); pool: pool(Spot,USD,GOLD); lp: lp(LP,Alice); }
    derive AMM.RedeemLP(pre.pool,pre.lp,shares(LP,100)) as next;
    post { a=pre.a+next.value.2; b=pre.b+next.value.3; pool=next.value.0; lp=next.value.1; }
    effects [Burn(Alice,shares(LP,100)),Debit(Spot,next.value.2),Credit(Alice,next.value.2),
      Debit(Spot,next.value.3),Credit(Alice,next.value.3),SetCell(pre.pool,next.value.0)];
    footprint reads(pre.a,pre.b,pre.pool,pre.lp) writes(pre.a,pre.b,pre.pool,pre.lp);
    authority Liquidity.RedeemLP; evidence Liquidity.evidence;
    kernel OneDomain(authority: Liquidity.RedeemLP,evidence: Liquidity.evidence);
    failures Effect[ShareBalance,ReserveRange,CompleteEffects]; outcome reject(first);
  }
}
```

`Credit(Spot,...)`/`Debit(Spot,...)` are profile sugar for Spot's source-bound
custody account, never permission to cast PoolId to Account. Pool/LP fields are
unique typed cells; reserve changes and custody transfers must agree without
double-counting supply. Zero economic lines are omitted by the relation, not a
solver choice. Routes require a new finite route profile with intermediate
recipients, all pools/fees and per-asset cumulative bounds. Multi-party clearing
also requires signed party liabilities and netting/novation rules; neither is
established by this single-pool example.

## 6. Lending and borrowing

**Desired outcome:** Bob funds a 500 USD loan secured by 1 GOLD; Alice's payments
reach the bound creditor; liquidation leaves unpaid debt visible. **Status:**
origination, roll-forward, locks/liquidation specified; their financial checks,
certification and ledger path open. Funded repayment's existing S0 relation is
local, while this new notation remains specified.

```mori
library Lending {
  struct LoanTerms<A,B> { principal: Qty<A>, collateral: Qty<B>, creditor: Account<domain(A)>,
    coupon: Price<A,A,2>, accrual_from: Round<domain(A)>, accrual_through: Round<domain(A)>,
    collateral_ratio_bps: UInt128, rounding: Rounding }
  // Originate: debit creditor/credit debtor principal; create debt(principal,0,principal);
  // lock collateral under Loan claim. Sum(all locks including other protocols)<=custody.
  relation Originate<A,B>(terms: LoanTerms<A,B>, debt: Option<Obligation<A>>, locks: Locks<B>)
    -> Certified<(Obligation<A>,Locks<B>)>;
  // Fixed bounded coupon at round200: accrued'=accrued+ceil(principal*1/100).
  // Roll cursor100->200 exactly once; outstanding'=principal+accrued'; no capitalized principal.
  relation RollForward<A>(debt: Obligation<A>, cursor: Round<domain(A)>, through: Round<domain(A)>)
    -> Certified<(Obligation<A>,Round<domain(A)>)>;
  // Repay: da=min(amount,accrued), dp=amount-da; debt(p-dp,a-da,outstanding-amount).
  // Require full funded creditor credit; status Settled iff outstanding'=0.
  relation FundedRepay<A>(debt: Obligation<A>, amount: Qty<A>) -> Obligation<A>;
  // Liquidate only with authenticated unhealthy trigger and current rolled debt.
  // Buyer pays r<=outstanding to bound creditor; transfers locked collateral to buyer;
  // remaining debt=max(outstanding-r,0), Defaulted if >0; residual creditor duty survives.
  relation Liquidate<A,B>(debt: Obligation<A>, locks: Locks<B>, sale: Qty<A>,
    price: Price<A,B,2>, trigger: LiquidationTrigger) -> Certified<(Obligation<A>,Locks<B>,Duty<A>)>;
}

agreement SecuredLoan {
  header(profile: Lending, id: SecuredLoan, actions: [Originate,Roll,Repay,Liquidate]);
  obligation Loan: ObjectId<Preview> = id("Loan");
  intent LoanPromise {
    fixed { debtor: Alice; creditor: Bob; buyer: Treasury; obligation: Loan;
      principal: 500.00 USD; collateral: 1.000 GOLD; collateral_object: id("LoanPledge"); ratio_bps: 15000;
      roll_from: round(Preview,100); roll_through: round(Preview,200);
      coupon: price<USD,USD,2>(1); payment: 30.00 USD; sale: 400.00 USD;
      loss_priority: [LoanCollateral,ResidualCreditorClaim]; }
    holes {}; bounds { Bob_USD: budget(subject: Bob,gross: 500.00 USD,fees: 0.00 USD);
      Alice_USD: budget(subject: Alice,gross: 30.00 USD,fees: 0.00 USD);
      Treasury_USD: budget(subject: Treasury,gross: 400.00 USD,fees: 0.00 USD);
      Alice_GOLD: budget(subject: Alice,gross: 1.000 GOLD,fees: 0.000 GOLD); work: 4; }
    receipts [receipt(gate: Milestone(Originate),recipient: Alice,asset: USD,
      scope: StageDelivery(Originate,claim(Loan)),floor: 500.00 USD,deductions: allDeliveryCharges(Originate,USD))];
    authority { Originate: grant(holder: Owner,roles: [Sign,Enforce],signers: owners([Owner,Creditor]));
      Roll: grant(holder: Keeper,roles: [Enforce],signers: owners([Owner,Creditor]));
      Repay: grant(holder: Owner,roles: [Sign],signers: owners([Owner]));
      Liquidate: grant(holder: Keeper,roles: [Enforce,Sign],signers: owners([Reserve,Creditor])); }
    evidence { debt: ObligationAtHead(Loan); aggregate: AllEncumbrances(Alice,GOLD);
      price: Feed; trigger: LiquidationPolicy(Terms); arithmetic: LoanArithmetic(Lending);
      custody: PledgedCustodyCapability(Loan,id("LoanPledge"),Alice,GOLD);
      funding: BalanceAtHead(Bob,USD); buyer: BalanceAtHead(Treasury,USD); }
    outcomes { settled: DebtDischarged(Loan);
      continuing: Duty(claim: claim(Loan),owed: debt(Loan).outstanding,terms: Terms.commitment,
        continuation: [Roll,Repay,Liquidate],recovery: []);
      rejected: reject(first); pending: KeepDebtAndLocks(Loan); }
  }
  stage Originate on Preview uses LoanPromise {
    operation lending.originate(obligation: Loan,debtor: Alice,creditor: Bob,
      principal: 500.00 USD,collateral: 1.000 GOLD);
    pre { lender: balance(Bob,USD); debtor: balance(Alice,USD); locks: locks(Alice,GOLD);
      debt: optional_debt(Loan,USD); duty: optional_duty(Loan,USD); }
    derive Lending.Originate(LoanPromise.fixed,pre.debt,pre.locks) as next;
    post { lender=pre.lender-500.00 USD; debtor=pre.debtor+500.00 USD;
      debt=Some(next.value.0); locks=next.value.1; duty=Some(debtDuty(next.value.0)); }
    effects [Debit(Bob,500.00 USD),Credit(Alice,500.00 USD),SetCell(pre.locks,post.locks),
      SetCell(pre.debt,post.debt),SetCell(pre.duty,post.duty)];
    footprint reads(pre.lender,pre.debtor,pre.locks,pre.debt,pre.duty)
      writes(pre.lender,pre.debtor,pre.locks,pre.debt,pre.duty);
    authority LoanPromise.Originate; evidence LoanPromise.evidence;
    kernel OneDomain(authority: LoanPromise.Originate,evidence: LoanPromise.evidence);
    failures Stage[DebtExists,MissingAggregateLocks] Intent[CollateralRatio]
      Effect[FundingRange,AggregateEncumbrance]; outcome reject(first);
  }
  stage Roll on Preview uses LoanPromise {
    operation lending.roll_forward(obligation: Loan,through: round(Preview,200));
    pre { debt: debt(Loan,USD); cursor: accrual_cursor(Loan,PreviewClock); duty: duty(Loan,USD); }
    derive Lending.RollForward(pre.debt,pre.cursor,round(Preview,200)) as next;
    post { debt=next.value.0; cursor=next.value.1; duty=debtDuty(post.debt); }
    effects [SetCell(pre.debt,post.debt),SetCell(pre.cursor,post.cursor),SetCell(pre.duty,post.duty)];
    footprint reads(pre.debt,pre.cursor,pre.duty) writes(pre.debt,pre.cursor,pre.duty);
    authority LoanPromise.Roll; evidence LoanPromise.evidence;
    kernel OneDomain(authority: LoanPromise.Roll,evidence: LoanPromise.evidence);
    failures Intent[AccrualWindow] Effect[DuplicateAccrual,ArithmeticRange]; outcome reject(first);
  }
  stage Repay on Preview uses LoanPromise {
    operation repay(obligation: Loan,payer: Alice,amount: 30.00 USD);
    pre { payer: balance(Alice,USD); creditor: balance(Bob,USD); debt: debt(Loan,USD); duty: duty(Loan,USD); }
    derive Lending.FundedRepay(pre.debt,30.00 USD) as next;
    post { payer=pre.payer-30.00 USD; creditor=pre.creditor+30.00 USD;
      debt=next; duty=debtDuty(next); }
    effects [Debit(Alice,30.00 USD),Credit(Bob,30.00 USD),SetCell(pre.debt,next),SetCell(pre.duty,post.duty)];
    footprint reads(pre.payer,pre.creditor,pre.debt,pre.duty) writes(pre.payer,pre.creditor,pre.debt,pre.duty);
    authority LoanPromise.Repay; evidence LoanPromise.evidence;
    kernel OneDomain(authority: LoanPromise.Repay,evidence: LoanPromise.evidence);
    failures Stage[CreditorBinding] Intent[PaymentScope] Effect[Overpayment,MissingCreditorCredit];
    outcome reject(first);
  }
  stage Liquidate on Preview uses LoanPromise {
    operation lending.liquidate(obligation: Loan);
    pre { buyer: balance(Treasury,USD); creditor: balance(Bob,USD); buyer_gold: balance(Treasury,GOLD);
      custody: custody_object(id("LoanPledge"),Alice,GOLD); free_gold: balance(Alice,GOLD);
      locks: locks(Alice,GOLD); debt: debt(Loan,USD); duty: duty(Loan,USD); }
    derive Lending.Liquidate(pre.debt,pre.locks,400.00 USD,evidence.price,evidence.trigger) as next;
    post { buyer=pre.buyer-400.00 USD; creditor=pre.creditor+400.00 USD;
      buyer_gold=pre.buyer_gold+1.000 GOLD; custody=pre.custody-1.000 GOLD;
      locks=next.value.1; debt=next.value.0; duty=next.value.2; unchanged free_gold; }
    effects [Debit(Treasury,400.00 USD),Credit(Bob,400.00 USD),
      DebitCustody(cell: pre.custody.cell,object: id("LoanPledge"),authority: evidence.custody,amount: 1.000 GOLD),
      Credit(Treasury,1.000 GOLD),SetCell(pre.locks,post.locks),SetCell(pre.debt,post.debt),SetCell(pre.duty,post.duty)];
    footprint reads(pre.buyer,pre.creditor,pre.buyer_gold,pre.custody,pre.free_gold,pre.locks,pre.debt,pre.duty)
      writes(pre.buyer,pre.creditor,pre.buyer_gold,pre.custody,pre.locks,pre.debt,pre.duty);
    authority LoanPromise.Liquidate; evidence LoanPromise.evidence;
    kernel OneDomain(authority: LoanPromise.Liquidate,evidence: LoanPromise.evidence);
    failures Stage[ObservationStale,UnrolledDebt] Intent[HealthyLoan,SaleScope]
      Effect[LockPriority,CompleteEffects]; outcome reject(first);
  }
  episode LoanLife uses LoanPromise { stages: [Originate,Roll,Repay,Liquidate];
    links: accepted_head_chain; terminal: DebtDischarged(Loan); residual: LoanPromise.outcomes.continuing; }
}
```

`debtDuty` is the closed constructor preserving the original creditor, claim and
Terms digest, with owed equal to outstanding; zero debt produces no duty.
`DebitCustody` consumes the named pledged object under its capability; it cannot
resolve to `balance(Alice,GOLD)`. The explicit free_gold read is framed unchanged,
and only the pledged custody cell is in the write set. This debit charges Alice's
signed collateral-spend bound once; no second ordinary Account Debit exists.
Lock moves do not create cash debits twice: origination encumbers existing custody,
liquidation transfers it once. With custody2 GOLD and another claim0.5 GOLD,
locking1 succeeds; custody1.4 must reject AggregateEncumbrance. A single 1% roll
makes debt505 USD, repayment30 leaves475, and sale400 leaves a Defaulted75 USD
creditor claim. No timeout, auction fee or caller impairment erases that 75.
A bankruptcy/default discharge requires a further signed loss relation and
qualified evidence, not a zero-out assignment. Loan refinancing/netting and
calendar-driven ACTUS accrual are additional profiles.

## 7. Stablecoins and synthetic assets

**Desired outcome:** deposit backing to mint MUSD, then redeem/burn it or claim
under an authenticated emergency settlement rate. **Status:** all interfaces and
relations specified; peg verifier, mint/burn capability, supply correspondence
and emergency commitment open.

```mori
library Stablecoin {
  struct StableState<A,B> { supply: Qty<A>, backing: Qty<B>, debt: Qty<A>, mode: StableMode }
  enum StableMode { Live, Shutdown(rate: Price<B,A,6>) }
  struct Redemption<A,B> { claimant: Account<domain(A)>, burn: Qty<A>, owed: Qty<B>, paid: Qty<B> }
  // Mint: supply'=supply+m, debt'=debt+m, backing'=backing+c;
  // c>=ceil(m*Price<B,A> * signed haircut); deposit and mint atomic.
  relation Mint<A,B>(state: StableState<A,B>, mint: Qty<A>, backing: Qty<B>, peg: Price<B,A,6>)
    -> Certified<StableState<A,B>>;
  // Redeem: burn m<=holder balance; release r<=backing and r>=signed minimum;
  // supply'=supply-m, debt'=debt-m, backing'=backing-r, disclosed reserve dust retained.
  relation Redeem<A,B>(state: StableState<A,B>, burn: Qty<A>, minimum: Qty<B>)
    -> Certified<(StableState<A,B>,Qty<B>)>;
  // Shutdown freezes rate=backing/supply (B per A), floor claimant amounts;
  // emergency_settle burns once, fixes claim, pays claim and discharges exact owed amount.
  relation Freeze<A,B>(state: StableState<A,B>, claims: List<Redemption<A,B>,16>,
    emergency: EmergencyPolicyEvidence) -> Certified<StableState<A,B>>;
  relation Emergency<A,B>(state: StableState<A,B>, claim: Option<Redemption<A,B>>,
    claimant: Account<domain(A)>, burn: Qty<A>)
    -> Certified<(StableState<A,B>,Redemption<A,B>,Qty<B>)>;
}

agreement BackedMoney {
  header(profile: Stablecoin,id: BackedMoney,actions: [Mint,Redeem,Shutdown,Emergency]);
  instrument Stable: InstrumentId<Preview> = id("Stable");
  intent CoinPromise {
    fixed { instrument: Stable; holder: Alice; mint: 100.00 MUSD; deposit: 0.200 GOLD;
      burn: 100.00 MUSD; minimum: 0.190 GOLD; peg_direction: Price<GOLD,MUSD,6>;
      rounding: Floor(Reserve); emergency_priority: ProRataAfterSeniorBackingClaims; }
    holes {}; bounds { GOLD: budget(gross: 0.200 GOLD,fees: 0.000 GOLD);
      MUSD: budget(gross: 100.00 MUSD,fees: 0.00 MUSD); work: 4; }
    receipts [receipt(gate: Milestone(Mint),recipient: Alice,asset: MUSD,
        scope: StageDelivery(Mint,claim(Stable)),floor: 100.00 MUSD,deductions: allDeliveryCharges(Mint,MUSD)),
      receipt(gate: Terminal(Redeemed),recipient: Alice,asset: GOLD,
        scope: StageDelivery(Redeem,claim(Stable)),floor: 0.190 GOLD,deductions: allDeliveryCharges(Redeem,GOLD)),
      receipt(gate: Terminal(EmergencySettled),recipient: Alice,asset: GOLD,
        scope: StageDelivery(Emergency,claim(Stable)),floor: 0.190 GOLD,deductions: allDeliveryCharges(Emergency,GOLD))];
    authority { Mint: grant(holder: Owner,roles: [Sign,Mint],signers: owners([Owner,Reserve]));
      Redeem: grant(holder: Owner,roles: [Sign,Burn],signers: owners([Owner,Reserve]));
      Shutdown: grant(holder: Quorum,roles: [Enforce,Amend],signers: quorum([k1,k2,k3],2));
      Emergency: grant(holder: Owner,roles: [Sign,Burn],signers: owners([Owner,Reserve])); }
    evidence { state: StableAtHead(Stable); capability: SupplyCapability(Stable);
      peg: EvidencePolicy<Price<GOLD,MUSD,6>>(verifier: verifier("PegV1"),source: source("Peg"),maximum_age: 5,finality: Finalized);
      emergency: EmergencyPolicyAtHead(Terms); arithmetic: BackingProof(Stable); }
    outcomes { settled: choice(Redeemed(Stable),EmergencySettled(Stable));
      require: SupplyDebtBackingConserved(Stable); rejected: reject(first);
      pending: RedemptionDuty(claimant: Alice,instrument: Stable,terms: Terms.commitment); }
  }
  stage Mint on Preview uses CoinPromise {
    operation stablecoin.mint(instrument: Stable,owner: Alice,supply: 100.00 MUSD,backing: 0.200 GOLD);
    pre { gold: balance(Alice,GOLD); coin: balance(Alice,MUSD); state: stable(Stable,MUSD,GOLD); }
    derive Stablecoin.Mint(pre.state,100.00 MUSD,0.200 GOLD,evidence.peg) as next;
    post { gold=pre.gold-0.200 GOLD; coin=pre.coin+100.00 MUSD; state=next.value; }
    effects [Debit(Alice,0.200 GOLD),Credit(Stable,0.200 GOLD),MintAsset(Alice,100.00 MUSD),SetCell(pre.state,next.value)];
    footprint reads(pre.gold,pre.coin,pre.state) writes(pre.gold,pre.coin,pre.state);
    authority CoinPromise.Mint; evidence CoinPromise.evidence;
    kernel OneDomain(authority: CoinPromise.Mint,evidence: CoinPromise.evidence);
    failures Stage[ObservationStale,ShutdownActive] Effect[BackingRatio,SupplyDebtMismatch]; outcome reject(first);
  }
  stage Redeem on Preview uses CoinPromise {
    operation stablecoin.redeem(instrument: Stable,owner: Alice,burn: 100.00 MUSD,minimum_backing: 0.190 GOLD);
    pre { gold: balance(Alice,GOLD); coin: balance(Alice,MUSD); state: stable(Stable,MUSD,GOLD); }
    derive Stablecoin.Redeem(pre.state,100.00 MUSD,0.190 GOLD) as next;
    post { coin=pre.coin-100.00 MUSD; gold=pre.gold+next.value.1; state=next.value.0; }
    effects [BurnAsset(Alice,100.00 MUSD),Debit(Stable,next.value.1),Credit(Alice,next.value.1),SetCell(pre.state,post.state)];
    footprint reads(pre.gold,pre.coin,pre.state) writes(pre.gold,pre.coin,pre.state);
    authority CoinPromise.Redeem; evidence CoinPromise.evidence;
    kernel OneDomain(authority: CoinPromise.Redeem,evidence: CoinPromise.evidence);
    failures Intent[MinimumBacking] Effect[InsufficientBacking,DebtDischargeMismatch]; outcome reject(first);
  }
  stage Shutdown on Preview uses CoinPromise {
    operation Stablecoin.freeze(instrument: Stable); // horizon-only; no beta registry entry
    pre { state: stable(Stable,MUSD,GOLD); claims: redemption_claims(Stable,MUSD,GOLD); }
    derive Stablecoin.Freeze(pre.state,pre.claims,evidence.emergency) as next;
    post { state=next.value; unchanged claims; }
    effects [SetCell(pre.state,post.state)];
    footprint reads(pre.state,pre.claims) writes(pre.state);
    authority CoinPromise.Shutdown; evidence CoinPromise.evidence;
    kernel OneDomain(authority: CoinPromise.Shutdown,evidence: CoinPromise.evidence);
    failures Stage[EmergencyPolicyMissing] Effect[ClaimPreservation,ZeroSupplyRate]; outcome reject(first);
  }
  stage Emergency on Preview uses CoinPromise {
    operation stablecoin.emergency_settle(instrument: Stable,owner: Alice,burn: 100.00 MUSD);
    pre { state: stable(Stable,MUSD,GOLD); claim: optional_redemption(Stable,Alice);
      coin: balance(Alice,MUSD); gold: balance(Alice,GOLD); duty: redemption_duty(Stable,Alice); }
    derive Stablecoin.Emergency(pre.state,pre.claim,Alice,100.00 MUSD) as next;
    post { state=next.value.0; claim=Some(next.value.1); coin=pre.coin-100.00 MUSD;
      gold=pre.gold+next.value.2; duty=remainingRedemption(next.value.1); }
    effects [BurnAsset(Alice,100.00 MUSD),Debit(Stable,next.value.2),Credit(Alice,next.value.2),
      SetCell(pre.state,post.state),SetCell(pre.claim,post.claim),SetCell(pre.duty,post.duty)];
    footprint reads(pre.state,pre.claim,pre.coin,pre.gold,pre.duty) writes(pre.state,pre.claim,pre.coin,pre.gold,pre.duty);
    authority CoinPromise.Emergency; evidence CoinPromise.evidence;
    kernel OneDomain(authority: CoinPromise.Emergency,evidence: CoinPromise.evidence);
    failures Stage[NotShutdown] Intent[ClaimOwner] Effect[DoubleBurn,ClaimDischargeMismatch]; outcome reject(first);
  }
  episode CoinLife uses CoinPromise { stages: [Mint,choice(Redeem,sequence(Shutdown,Emergency))];
    links: accepted_head_chain; residual: CoinPromise.outcomes.pending;
    terminal: match terminalStage<Redeem,Emergency> {
      Redeem => Redeemed(Stable); Emergency => EmergencySettled(Stable);
    } }
}
```

`Stablecoin.Freeze` has typed inputs StableState/claim list/emergency authority and
returns the same supply/debt/backing with `mode=Shutdown(certified fixed rate)`;
all existing claims preserve their terms and priority. `remainingRedemption`
returns a duty for `owed-paid`, or None when zero. Freeze is horizon-only, not
an extra beta-registry call. Stable's custody identity is bound by its capability.
The displayed emergency branch requires `pre.claim=None`, creates its frozen-rate
claim by burning100 MUSD, records the paid amount and discharges it in the same
atomic stage. An existing unpaid burned claim uses a separate zero-burn payment
constructor under the same priority/rate policy; that constructor is open and
not claimed by this new-claim example. The emergency branch burns and pays once; it cannot repeat a prior redemption.
If the fixed pro-rata backing is below 0.190 GOLD, settlement is allowed only by
an explicitly signed EmergencyHaircut alternative with its rate/priority/net
floor; this example has no such relaxation and remains Pending with its claim.
A timestamp or policy string cannot trigger shutdown or forgive backing debt.

## 8. Derivatives

**Desired outcome:** a fully collateralized 1 GOLD cash-settled call, strike
100 USD/GOLD, fixing at round150; Alice pays premium5 USD, Bob reserves500 USD.
**Status:** all lifecycle constructs specified; observation qualification,
certified payoff, exercise-right consumption and reserve settlement open.

```mori
library Option {
  enum OptionPhase { Unfunded, Funded, Fixed, Exercised, Settled }
  struct OptionState<A,B> { holder: Account<domain(A)>, writer: Account<domain(A)>,
    position: Position<InstrumentId>, strike: Price<A,B,2>, notional: Qty<B>,
    reserve: Qty<A>, premium_paid: Qty<A>, fixing: Option<Price<A,B,2>>, phase: OptionPhase }
  // Fund: premium holder->writer, collateral writer->instrument custody, create duty.
  // Fix: selected finalized source at exact round, record value/unit/provenance once.
  // Exercise: consume holder right once; payoff=max((fixing-strike)*notional,0), floor to Reserve.
  // Settle: credit payoff to holder and all residual reserve to writer atomically,
  // reserve'=0, phase=Settled, discharge cash-settlement duty; payoff<=reserve.
  relation Fund<A,B>(state: OptionState<A,B>, premium: Qty<A>, collateral: Qty<A>) -> OptionState<A,B>;
  relation Fix<A,B>(state: OptionState<A,B>, observation: FinalObservation<Price<A,B,2>>) -> OptionState<A,B>;
  relation Exercise<A,B>(state: OptionState<A,B>) -> Certified<(OptionState<A,B>,Qty<A>)>;
  relation Settle<A,B>(state: OptionState<A,B>, payoff: Certified<Qty<A>>) -> OptionState<A,B>;
}

agreement CashCall {
  header(profile: Option,id: CashCall,actions: [Fund,Fix,Exercise,Settle]);
  instrument Call: InstrumentId<Preview> = id("Call");
  observation GoldFixing150: SubjectId<Price<USD,GOLD,2>> = subject(asset: GOLD,round: round(Preview,150));
  intent CallPromise {
    fixed { instrument: Call; holder: Alice; writer: Bob; notional: 1.000 GOLD;
      position: position(Call,1); strike: price<USD,GOLD,2>(10000);
      fixing_round: round(Preview,150); premium: 5.00 USD; collateral: 500.00 USD;
      maximum_payoff: 500.00 USD; rounding: Floor(Reserve); }
    holes {}; bounds { Alice_USD: budget(subject: Alice,gross: 5.00 USD,fees: 0.00 USD);
      Bob_USD: budget(subject: Bob,gross: 500.00 USD,fees: 0.00 USD); work: 4; }
    receipts []; // terminal OptionPaid binds the certified payoff and residual reserve exactly
    authority { Fund: grant(holder: Owner,roles: [Sign],signers: owners([Owner,Creditor]));
      Fix: grant(holder: Provider,roles: [Enforce],signers: owners([Owner,Creditor]));
      Exercise: grant(holder: Owner,roles: [Sign],signers: owners([Owner]));
      Settle: grant(holder: Keeper,roles: [Enforce],signers: owners([Owner,Creditor])); }
    evidence { state: OptionAtHead(Call); fixing: Feed; reserve: InstrumentCustody(Call,USD);
      arithmetic: PayoffProof(Call); selected: program(Option); }
    outcomes { settled: OptionPaid(holder: Alice,writer: Bob,instrument: Call);
      rejected: reject(first); pending: AwaitFixingOrSettlement(instrument: Call,
        reserve: 500.00 USD,continuation: [Fix,Exercise,Settle],recovery: []); }
  }
  stage Fund on Preview uses CallPromise {
    operation Option.fund(instrument: Call,premium: 5.00 USD,collateral: 500.00 USD); // horizon-only
    pre { holder: balance(Alice,USD); writer: balance(Bob,USD); state: option(Call,USD,GOLD); duty: option_duty(Call,USD); }
    derive Option.Fund(pre.state,5.00 USD,500.00 USD) as next;
    post { holder=pre.holder-5.00 USD; writer=pre.writer+5.00 USD-500.00 USD;
      state=next; duty=cashSettlementDuty(Call,CallPromise); }
    effects [Debit(Alice,5.00 USD),Credit(Bob,5.00 USD),Debit(Bob,500.00 USD),Credit(Call,500.00 USD),
      SetCell(pre.state,next),SetCell(pre.duty,post.duty)];
    footprint reads(pre.holder,pre.writer,pre.state,pre.duty) writes(pre.holder,pre.writer,pre.state,pre.duty);
    authority CallPromise.Fund; evidence CallPromise.evidence;
    kernel OneDomain(authority: CallPromise.Fund,evidence: CallPromise.evidence);
    failures Intent[AlreadyFunded] Effect[ReserveRange,PremiumNotPaid]; outcome reject(first);
  }
  stage Fix on Preview uses CallPromise {
    operation option.fix(instrument: Call,observation: GoldFixing150);
    pre { state: option(Call,USD,GOLD); duty: option_duty(Call,USD); }
    completion { fixing: selectEvidence(policy: Feed,subject: GOLD,round: round(Preview,150)); }
    derive match completion.fixing {
      Final(obs) => Option.Fix(pre.state,obs);
      Missing => pending(CallPromise.outcomes.pending);
      Stale(obs) => reject(Stage(ObservationStale));
      Disputed(conflict) => pending(CallPromise.outcomes.pending);
    } as next;
    post { state=next; unchanged duty; }
    effects [SetCell(pre.state,next)]; footprint reads(pre.state,pre.duty) writes(pre.state);
    authority CallPromise.Fix; evidence CallPromise.evidence;
    kernel OneDomain(authority: CallPromise.Fix,evidence: CallPromise.evidence);
    failures Stage[ObservationUnit,ObservationSource,ObservationRound,ObservationStale]
      Intent[FixingAlreadySet]; outcome reject(first);
  }
  stage Exercise on Preview uses CallPromise {
    operation option.exercise(instrument: Call,holder: Alice);
    pre { state: option(Call,USD,GOLD); payoff: option_payoff(Call,USD); duty: option_duty(Call,USD); }
    derive Option.Exercise(pre.state) as next;
    post { state=next.value.0; payoff=next.value.1; duty=cashSettlementDuty(Call,CallPromise); }
    effects [SetCell(pre.state,post.state),SetCell(pre.payoff,post.payoff),SetCell(pre.duty,post.duty)];
    footprint reads(pre.state,pre.payoff,pre.duty) writes(pre.state,pre.payoff,pre.duty);
    authority CallPromise.Exercise; evidence CallPromise.evidence;
    kernel OneDomain(authority: CallPromise.Exercise,evidence: CallPromise.evidence);
    failures Intent[HolderRight,ExerciseWindow] Effect[PayoffRange,ReserveInsufficient];
    outcome reject(first); pending CallPromise.outcomes.pending;
  }
  stage Settle on Preview uses CallPromise {
    operation option.settle(instrument: Call);
    pre { holder: balance(Alice,USD); writer: balance(Bob,USD); state: option(Call,USD,GOLD);
      payoff: option_payoff(Call,USD); duty: option_duty(Call,USD); }
    derive Option.Settle(pre.state,certify(pre.payoff,evidence.arithmetic)) as next;
    post { holder=pre.holder+pre.payoff; writer=pre.writer+pre.state.reserve-pre.payoff;
      state=next; duty=None; unchanged payoff; }
    effects [Debit(Call,pre.payoff),Credit(Alice,pre.payoff),Debit(Call,pre.state.reserve-pre.payoff),
      Credit(Bob,pre.state.reserve-pre.payoff),SetCell(pre.state,next),SetCell(pre.duty,None)];
    footprint reads(pre.holder,pre.writer,pre.state,pre.payoff,pre.duty)
      writes(pre.holder,pre.writer,pre.state,pre.duty);
    authority CallPromise.Settle; evidence CallPromise.evidence;
    kernel OneDomain(authority: CallPromise.Settle,evidence: CallPromise.evidence);
    failures Intent[NotExercised] Effect[PayoffMismatch,ReserveConservation,DutyDischarge]; outcome reject(first);
  }
  episode OptionLife uses CallPromise { stages: [Fund,Fix,Exercise,Settle];
    links: accepted_head_chain; residual: CallPromise.outcomes.pending; terminal: CallPromise.outcomes.settled; }
}
```

`pending/reject` in a `derive match` exits that branch before any post/effect is
formed. Retry Fix uses authenticated permissible evidence and the same funded
instrument/duty, not a second premium/reserve debit. At fixing140 USD/GOLD,
payoff40 USD goes to Alice and residual460 USD to Bob; premium remains a separate
5 USD outlay, so Alice's profit before other costs is35 USD. Missing/disputed
fixing retains500 USD reserve and the exercise/settlement duty. A perpetual
requires additional signed positions, funding clock/cashflow, margin/netting,
liquidation and loss priorities; this call supplies none of them.

## 9. Oracles and observations

**Desired outcome:** select a qualified typed fixing, distinguish absence,
staleness and dispute, and expose the assumptions of aggregation.
**Status:** typed observation/query/aggregation interfaces specified; verifier,
provider independence and authentication open; no oracle financial execution local.

```mori
library Oracle {
  struct FinalObservation<T> { subject: SubjectId<T>, value: T, unit: Unit<T>,
    source: SourceId<T>, observed: Round<Preview>, received: Round<Preview>,
    finality: Finality, provenance: ProvenanceProof<T> }
  enum Observation<T> { Final(value: FinalObservation<T>), Missing,
    Stale(value: FinalObservation<T>), Disputed(conflict: ConflictEvidence<Preview,T>) }
  struct AggregatePolicy<T> { sources: Finite<SourceId<T>,3>, quorum: UInt128,
    independence: IndependencePolicyId, combine: Median<T>, rounding: Rounding }
  relation Select<T>(candidate: Observation<T>, policy: EvidencePolicy<T>,
    now: Round<Preview>, requested: Round<Preview>) -> Observation<T>;
  // Require source/unit/subject/finality, observed<=now, now-observed<=maximum_age,
  // and exact requested fixing round. Median requires >=2 distinct authenticated
  // sources from the fixed three, with independent-source proof under selected policy.
}

agreement ReadGold {
  header(profile: Oracle,id: ReadGold,actions: [Read]);
  observation GoldFixing150: SubjectId<Price<USD,GOLD,2>> = subject(asset: GOLD,round: round(Preview,150));
  intent ReadPromise {
    fixed { subject: GoldFixing150; unit: unit<Price<USD,GOLD,2>>(); requested: round(Preview,150); }
    holes {}; bounds { USD: budget(gross: 0.00 USD,fees: 0.00 USD); work: 1; }
    observational_scopes { Read: observational_rule(action: Read,query: id("ReadGoldQuery150"),
      selector: fixing_selector(GoldFixing150,Feed),checkpoints: selected_finalized(Preview,Feed.verifier),
      original_facts: [],service_work: 1,ledger_effects: [],ledger_controls: []); }
    receipts []; // typed value observation is not a cash receipt
    authority { Read: grant(holder: Provider,roles: [Enforce],signers: owners([Owner])); }
    evidence { observation: Feed; current_round: LedgerClock(Preview);
      aggregate: AggregatePolicy(sources: [FeedA,FeedB,FeedC],quorum: 2,
        independence: id("IndependentOperatorsV1"),combine: Median,rounding: Floor(Reserve)); }
    outcomes { settled: QualifiedValue(GoldFixing150); rejected: reject(first);
      pending: AwaitObservation(subject: GoldFixing150,continuation: [Read],recovery: []); }
  }
  observation_action Read on Preview uses ReadPromise {
    rule: ReadPromise.observational_scopes.Read;
    identity: Query(query: id("ReadGoldQuery150"),selector_digest: digest_of(rule.selector));
    operation oracle.select(observation: GoldFixing150);
    pre { observation: observation(GoldFixing150); now: current_round(PreviewClock); policy: feed_policy(Feed); }
    derive Oracle.Select(pre.observation,Feed,pre.now,round(Preview,150)) as selected;
    post { unchanged observation; unchanged now; unchanged policy; }
    effects []; footprint reads(pre.observation,pre.now,pre.policy) writes();
    authority ReadPromise.Read; evidence ReadPromise.evidence;
    kernel observe(authority: ReadPromise.Read,evidence: ReadPromise.evidence);
    outcome match selected {
      Final(obs) => QualifiedValue(value: obs.value,evidence: obs.provenance);
      Missing => pending(ReadPromise.outcomes.pending);
      Stale(obs) => reject(Stage(ObservationStale));
      Disputed(conflict) => pending(ReadPromise.outcomes.pending);
    }
  }
}
```

A concrete raw candidate is `FinalObservation(subject: GoldFixing150,
value: price<USD,GOLD,2>(14000), unit: unit<Price<USD,GOLD,2>>(), source: Feed.source,
observed: round(Preview,150), received: round(Preview,151), finality: Finalized,
provenance: provider_evidence)`. Its value/time are **claims** until qualification.
At now151, age1 meets max5; at now156 it is Stale. A disputed150 price retains
the dependent option's duty; ReadGold by itself has no financial custody or new
ledger duty. `Read` declares `identity: Query(query: ReadGoldQuery150,selector_digest:
digest_of(ReadPromise.observational_scopes.Read.selector))`; ReadGoldQuery150 is a source-fixed QueryId<Preview>.
`kernel observe` expands to `scopeFor(stage: Read, intent: ReadPromise,
grant: ReadPromise.Read, identity: Read.signedIdentity,
attempt: queryJournal.currentDurableAttempt, evidence: ReadPromise.evidence.authorization)`
followed by `Core.prepareObservation(action: Read,request: request,
evidence: ReadPromise.evidence)` and, only on its scheduled qualified result,
`Kernel.observe(scope: preparation.observationalScope,
selector: GoldFixing150,evidence: verifyProvider(preparation.observationalScope,ReadPromise.evidence))`.
Query is a read-only operation identity, not an economic custody claim. The
selector digest binds unit/source/subject/requested round, and the attempt binds
the actual provider request. Query and attempt identities must match the receipt;
two queries sharing a grant cannot exchange observations or provenance. Aggregation is a different
certified Oracle.Aggregate relation; the policy record alone does not compute
or establish an independent-source median. No timestamp authenticates a feed.

## 10. Governance

**Desired outcome:** queue epoch4 policy, allow a quorum veto until179, enact
no earlier than180, and preserve every already signed obligation's terms.
**Status:** library process/effect contracts specified; quorum/revocation,
policy authentication and obligation-preservation proof open.

```mori
library Governance {
  enum QueueStatus { Empty, Queued, Vetoed, Enacted }
  struct Amendment { next_epoch: UInt128, next_commitment: Digest,
    earliest: Round<Preview>, veto_through: Round<Preview>, status: QueueStatus }
  struct DutiesSnapshot { commitments: List<Digest,16>, total_liabilities: LiabilityVector }
  relation Queue(current: PolicyState, amendment: Amendment, evidence: QuorumEvidence) -> Amendment;
  relation Veto(amendment: Amendment, now: Round<Preview>, evidence: VetoEvidence) -> Amendment;
  relation Execute(current: PolicyState, amendment: Amendment, now: Round<Preview>,
    duties: DutiesSnapshot, proof: DutyPreservationProof) -> (PolicyState,Amendment);
  // Queue requires next_epoch=current.epoch+1, authenticated threshold, ordered clock.
  // Veto requires now<=179 and source-authorized veto quorum; Queued->Vetoed.
  // Execute requires now>=180, Queued, final no-veto fact; policy->epoch4,
  // new commitments use new terms, each existing duty keeps its previous digest/liability.
}

agreement PolicyChange {
  header(profile: Governance,id: PolicyChange,actions: [Queue,Veto,Execute]);
  intent AmendmentPromise {
    fixed { policy: Terms; next_epoch: 4; next_commitment: digest("terms-v4");
      earliest: round(Preview,180); veto_through: round(Preview,179); preserve_existing: true; }
    holes {}; bounds { USD: budget(gross: 0.00 USD,fees: 0.00 USD); work: 2; }
    receipts []; // policy transition preserves liabilities rather than promising cash
    authority { Queue: grant(holder: Quorum,roles: [Amend,Sign],signers: quorum([k1,k2,k3],2));
      Veto: grant(holder: Quorum,roles: [Veto,Sign],signers: quorum([v1,v2,v3],2));
      Execute: grant(holder: Keeper,roles: [Enforce],signers: quorum([k1,k2,k3],2)); }
    evidence { policy: PolicyAtHead(Terms); quorum: DistinctSignaturesAtEpoch(3);
      clock: LedgerClock(Preview); veto: FinalVetoJournal(Terms); duties: DutiesAtHead(Terms);
      preservation: DutyPreservationProof(Terms); }
    outcomes { settled: choice(Enacted(epoch: 4),Vetoed(epoch: 3)); rejected: reject(first);
      pending: AwaitTimelock(policy: Terms,continuation: [Veto,Execute],recovery: []); }
  }
  stage Queue on Preview uses AmendmentPromise {
    operation governance.queue(policy: Terms,next_epoch: 4);
    pre { policy: policy(Terms); queue: amendment_queue(Terms); duties: signed_duties(Terms); }
    derive Governance.Queue(pre.policy,AmendmentPromise.fixed,evidence.quorum) as next;
    post { queue=next; unchanged policy; unchanged duties; }
    effects [SetCell(pre.queue,next)]; footprint reads(pre.policy,pre.queue,pre.duties) writes(pre.queue);
    authority AmendmentPromise.Queue; evidence AmendmentPromise.evidence;
    kernel OneDomain(authority: AmendmentPromise.Queue,evidence: AmendmentPromise.evidence);
    failures Authority[GovernanceQuorum,RevokedKey] Effect[DutyPreservation]; outcome reject(first);
  }
  stage Veto on Preview uses AmendmentPromise {
    operation governance.veto(policy: Terms);
    pre { queue: amendment_queue(Terms); now: current_round(PreviewClock); }
    derive Governance.Veto(pre.queue,pre.now,evidence.veto) as next;
    post { queue=next; unchanged now; }
    effects [SetCell(pre.queue,next)]; footprint reads(pre.queue,pre.now) writes(pre.queue);
    authority AmendmentPromise.Veto; evidence AmendmentPromise.evidence;
    kernel OneDomain(authority: AmendmentPromise.Veto,evidence: AmendmentPromise.evidence);
    failures Intent[VetoWindow] Authority[VetoQuorum]; outcome reject(first);
  }
  stage Execute on Preview uses AmendmentPromise {
    operation governance.execute(policy: Terms);
    pre { policy: policy(Terms); queue: amendment_queue(Terms); now: current_round(PreviewClock);
      duties: signed_duties(Terms); }
    derive Governance.Execute(pre.policy,pre.queue,pre.now,pre.duties,evidence.preservation) as next;
    post { policy=next.0; queue=next.1; unchanged now; unchanged duties; }
    effects [SetCell(pre.policy,post.policy),SetCell(pre.queue,post.queue)];
    footprint reads(pre.policy,pre.queue,pre.now,pre.duties) writes(pre.policy,pre.queue);
    authority AmendmentPromise.Execute; evidence AmendmentPromise.evidence;
    kernel OneDomain(authority: AmendmentPromise.Execute,evidence: AmendmentPromise.evidence);
    failures Stage[NoFinalVetoEvidence] Intent[Timelock,GovernanceVeto]
      Effect[EpochSequence,DutyPreservation]; outcome reject(first); pending AmendmentPromise.outcomes.pending;
  }
  episode AmendmentLife uses AmendmentPromise { stages: [Queue,choice(Veto,Execute)];
    links: accepted_head_chain; residual: AmendmentPromise.outcomes.pending; terminal: AmendmentPromise.outcomes.settled; }
}
```

Queue/Veto/Execute use the same deferred scope collection and shared scheduled
Core.prepare as section3; invalid epoch/effects precede an expired grant, while
invalid Stage bindings still precede both. No quorum/expiry preflight overrides
that order. Queue and a later Execute are ledger-linked stages, not one atomic waiting
transaction. Enact changes which policy authorizes new commitments; it does
not rewrite a stored debt, redemption or withdrawal duty. A revocation proof
narrows future grants; separate recovery authority may still be needed to fulfill
existing duties. The public compiler requires no governance quorum or maintainer
permission: these are financial agreement roles, not developer admission rules.

## 11. Bridges and cross-domain settlement

**Desired outcome:** escrow100 USD on Preview and obtain100 WUSD for AliceF on
Foreign, or a qualified source refund under the same economic claim.
**Status:** separate stages/representation/recovery interface specified;
foreign verification, finality, replay/nullifier and conservation proofs open.

```mori
library Bridge {
  nominal type EconomicClaimId;
  struct ClaimPair { economic: EconomicClaimId, source: ClaimId<Preview>, destination: ClaimId<Foreign> }
  enum SourceClaimStatus { Unspent, Escrowed, Refunded, Delivered }
  enum DestinationClaimStatus { Unclaimed, Received, Returned }
  struct EscrowState<A> { owner: Account<Preview>, amount: Qty<A>, status: SourceClaimStatus, pair: ClaimPair }
  struct ReceiptState<B> { recipient: Account<Foreign>, amount: Qty<B>, status: DestinationClaimStatus, pair: ClaimPair }
  struct RepresentationProof<A,B> { origin: A, destination: B, ratio: Price<B,A,0>,
    source_final: FinalEscrowEvidence<A>, mint_authority: SupplyCapability<B> }
  struct NonreceiptProof { pair: ClaimPair, finalized_count: UInt128,
    cutoff: FinalCheckpoint<Foreign>, excludes_later_delivery: ExclusionProof<Foreign> }
  enum ReceiptKnowledge { FinalReceipt, QualifiedNonreceipt, Unknown, Conflict }
  relation Escrow<A>(source: EscrowState<A>, amount: Qty<A>, pair: ClaimPair) -> EscrowState<A>;
  relation Claim<A,B>(source: FinalEscrowEvidence<A>, receipt: ReceiptState<B>,
    conversion: RepresentationProof<A,B>) -> ReceiptState<B>;
  relation Recover<A>(source: EscrowState<A>, nonreceipt: NonreceiptProof) -> EscrowState<A>;
  relation Classify(source: FinalEscrowEvidence, count: DeliveryCountEvidence,
    nullifier: PairedNullifierState, checkpoint: FinalCheckpoint<Foreign>) -> ReceiptKnowledge;
  // Escrow: Unspent->Escrowed; custody100 and claim duty created atomically.
  // Claim: finalized escrow/representation1:1 -> mint100 WUSD, consume paired nullifier once;
  // source escrow stays backing until explicit delivery/reconciliation accounting.
  // Recover: count=0 AND final nonreceipt/exclusion proof -> release100 USD once;
  // destination future Claim must be excluded/revoked under the same pair before release.
}

agreement BridgeTransfer {
  header(profile: Bridge,id: BridgeTransfer,actions: [Escrow,Claim,Reconcile,Recover]);
  foreign_header(domain: Foreign,validity: WindowF,keys: [OwnerForeignKey],
    pre_head: head(Foreign,"BridgeTransferClaim0"),nonces: [Claim1]);
  const Pair = ClaimPair(economic: id("BridgeEconomic1"),source: id("BridgeSource1"),destination: id("BridgeDest1"));
  intent BridgePromise {
    fixed { pair: Pair; source_owner: Alice; destination_recipient: AliceF;
      origin: USD; destination: WUSD; amount: 100.00 USD; net: 100.00 WUSD;
      representation: price<WUSD,USD,0>(1); expiry: round(Foreign,300); }
    holes {}; bounds { USD: budget(subject: Alice,gross: 100.00 USD,fees: 0.00 USD);
      WUSD: budget(subject: AliceF,gross: 0.00 WUSD,fees: 0.00 WUSD); work: 4; }
    observational_scopes { Reconcile: observational_rule(action: Reconcile,
      query: id("BridgeReconcile1"),selector: paired_delivery_selector(Pair,maximum_attempts: 8),
      checkpoints: selected_finalized(Foreign,BridgePromise.evidence.destination),
      original_facts: [exactAcceptedFact(Preview,Pair,Escrow),exactAcceptedFact(Foreign,Pair,Claim)],
      service_work: 2,ledger_effects: [],ledger_controls: []); }
    receipts [receipt(gate: Terminal(delivered),recipient: AliceF,asset: WUSD,
        scope: ReservedCustody(Claim,Pair,Claim),floor: 100.00 WUSD,deductions: allClaimSpends(Pair,WUSD)),
      receipt(gate: Terminal(refund),recipient: Alice,asset: USD,
        scope: StageDelivery(Recover,Pair),floor: 100.00 USD,deductions: allDeliveryCharges(Recover,USD))];
    authority { Escrow: grant(holder: Owner,roles: [Sign],signers: owners([Owner]));
      Claim: grant(domain: Foreign,holder: Owner,roles: [Sign,Mint],signers: owners([Owner,Reserve]));
      Reconcile: grant(domain: Foreign,holder: Keeper,roles: [Enforce],signers: owners([Owner]));
      Recover: grant(holder: Owner,roles: [Recover,Sign],signers: owners([Owner,Reserve])); }
    evidence { source: FinalEscrowEvidence(Pair); destination: ForeignVerifier(Foreign,Bridge);
      representation: RepresentationProof(USD,WUSD); replay: PairedNullifier(Pair);
      nonreceipt: NonreceiptProof(Pair); supply: SupplyCapability(WUSD); }
    outcomes { delivered: FinalReceipt(pair: Pair,recipient: AliceF,net: 100.00 WUSD);
      unknown: Pending(pair: Pair,duty: ClaimOrReconcile(Pair),reservation: SourceEscrow(Pair));
      refund: QualifiedSourceRefund(pair: Pair,recipient: Alice,amount: 100.00 USD);
      rejected: reject(first); }
  }
  stage Escrow on Preview uses BridgePromise {
    operation bridge.escrow(owner: Alice,asset: USD,amount: 100.00 USD,claim: Pair.source);
    pre { owner: balance(Alice,USD); escrow: escrow(Pair.source,USD); duty: bridge_duty(Pair); }
    derive Bridge.Escrow(pre.escrow,100.00 USD,Pair) as next;
    post { owner=pre.owner-100.00 USD; escrow=next; duty=ClaimOrReconcile(Pair); }
    effects [Debit(Alice,100.00 USD),Credit(Pair.source,100.00 USD),SetCell(pre.escrow,next),SetCell(pre.duty,post.duty)];
    footprint reads(pre.owner,pre.escrow,pre.duty) writes(pre.owner,pre.escrow,pre.duty);
    authority BridgePromise.Escrow; evidence BridgePromise.evidence;
    kernel OneDomain(authority: BridgePromise.Escrow,evidence: BridgePromise.evidence);
    failures Intent[ClaimPairBinding] Effect[EscrowConservation]; outcome reject(first);
  }
  stage Claim on Foreign uses BridgePromise {
    operation bridge.claim(recipient: AliceF,asset: WUSD,amount: 100.00 WUSD,claim: Pair.destination);
    pre { recipient: balance(AliceF,WUSD); receipt: receipt(Pair.destination,WUSD); nullifier: nullifier(Pair.destination); }
    derive Bridge.Claim(evidence.source,pre.receipt,evidence.representation) as next;
    post { recipient=pre.recipient+100.00 WUSD; receipt=next; nullifier=Consumed(Pair); }
    effects [MintAsset(AliceF,100.00 WUSD),SetCell(pre.receipt,next),SetCell(pre.nullifier,post.nullifier)];
    footprint reads(pre.recipient,pre.receipt,pre.nullifier) writes(pre.recipient,pre.receipt,pre.nullifier);
    authority BridgePromise.Claim; evidence BridgePromise.evidence;
    kernel OneDomain(authority: BridgePromise.Claim,evidence: BridgePromise.evidence);
    failures Stage[ForeignProof,SourceFinality] Intent[Recipient,RepresentationDirection]
      History[NullifierConsumed]; outcome reject(first); pending BridgePromise.outcomes.unknown;
  }
  observation_action Reconcile on Foreign uses BridgePromise {
    rule: BridgePromise.observational_scopes.Reconcile;
    identity: Query(query: id("BridgeReconcile1"),selector_digest: digest_of(rule.selector));
    evidence_reads [source_final_escrow(Pair.source),foreign_delivery_count(Pair.destination,8),
      foreign_nullifier(Pair.destination),foreign_exclusion_checkpoint(Pair.destination),
      original_claim_edge(Pair.destination,Claim)];
    economic_effects []; ledger_writes []; ledger_control_effects [];
    service_journal append_query_fact(identity,attempt,raw_receipt);
    authority BridgePromise.Reconcile; evidence BridgePromise.evidence;
    kernel reconcile(authority: BridgePromise.Reconcile,evidence: reconcileEvidenceBundle(
      parent: BridgePromise,delivery: deferredDeliveryEvidence(Pair),authorization: BridgePromise.evidence.authorization));
    failures Stage[ObservationCheckpoint,IncompleteAttemptCoverage] Authority[ObservationWork,ExpiredAuthorization]
      History[QueryAttemptBinding]; outcome observation_only;
  }
  stage Recover on Preview uses BridgePromise {
    operation bridge.recover(owner: Alice,asset: USD,claim: Pair.source);
    pre { owner: balance(Alice,USD); escrow: escrow(Pair.source,USD); duty: bridge_duty(Pair); }
    derive Bridge.Recover(pre.escrow,evidence.nonreceipt) as next;
    post { owner=pre.owner+100.00 USD; escrow=next; duty=None; }
    effects [Debit(Pair.source,100.00 USD),Credit(Alice,100.00 USD),SetCell(pre.escrow,next),SetCell(pre.duty,None)];
    footprint reads(pre.owner,pre.escrow,pre.duty) writes(pre.owner,pre.escrow,pre.duty);
    authority BridgePromise.Recover; evidence BridgePromise.evidence;
    kernel recover(authority: BridgePromise.Recover,evidence: BridgePromise.evidence);
    failures Stage[NonreceiptUnqualified,LateDeliveryNotExcluded] Intent[RecoveryBranch]
      Effect[RefundConservation]; outcome reject(first); pending BridgePromise.outcomes.unknown;
  }
  episode BridgeLife uses BridgePromise { stages: [Escrow,choice(Claim,Recover)];
    links: foreign_evidence(Pair); terminal: choice(BridgePromise.outcomes.delivered,BridgePromise.outcomes.refund);
    on_unknown: observe_reconcile(action: Reconcile,parent: BridgePromise,
      grant: BridgePromise.Reconcile,rule: BridgePromise.observational_scopes.Reconcile,
      attempt: currentDurableQueryAttempt(),evidence: reconcileEvidenceBundle(BridgePromise,Pair));
    on_timeout: BridgePromise.outcomes.unknown;
  }
}
```

Reconcile's signed finite evidence-read set is source escrow proof, foreign
complete delivery count across at most8 attempts, paired nullifier/exclusion
checkpoint and the **original** Claim edge if one exists. It is an observational
action: it never SetCells an economic/ledger journal, consumes no ledger nonce,
allowance or head, and cannot become Swap's predecessor. Service fact records
are separately typed ServiceQueryFact records and spend only the declared
observation work quota/attempt identity in its service journal. Their existence proves no economic effect. Each dispatched query attempt spends
one unit of the signed observation quota and the parent's cumulative work budget,
using an authenticated durable service journal reservation/deduplication; it
never resets those counters on a new attempt or writes ledger Work/Replay/Head
cells. No further query is authorized when the declared2-unit quota is exhausted.
Service recording and economic acceptance remain separate facts/proofs. An expired observation grant needs newly signed
observation authority before another query; financial duties remain unchanged.

`observe_reconcile` first forms ScopeRequest with Query identity from the source
rule and exact parent/grant/durable query attempt. It runs Core.prepareObservation
on the request term using the shared judgment order; only its qualified query
scope enters Kernel.reconcile. The returned proof/receipt is then processed by
Core.qualifyObservation under the same ordered response judgments. This is not
scopeFor authorization preflight. The request's Stage checks schema, selected
provider and authenticated current authority-clock facts; returned checkpoint/
count/edge proofs are explicit response premises, not assumed request facts.
Both terms publish no financial effects. Query permission does not qualify the
future response, and a service receipt cannot provide a trusted success flag.

If Claim was never accepted, observations yield Unknown unless they establish
qualified finalized complete nonreceipt plus exclusion of all later delivery;
zero count alone cannot release escrow. If Claim accepted f0→f1 but its response
was lost, qualified evidence recovers that exact mint/edge under original-time
authorization. It does not send/mint/advance history again. A separate Core
qualification of the original economic Claim fact may then link it to the
accepted episode history; Swap still resolves that Claim's original f1. A queried
final checkpoint fx is permitted observation data only when the selected verifier
binds domain/Pair/query/coverage; it cannot replace the economic successor f1.
Unrelated/misbound checkpoint evidence rejects; unrelated real progress from f1
to fx may be observed but requires fresh economic authorization before Swap.

`kernel recover` uses the scheduled **economic** scope, full signed branch and
qualified nonreceipt bundle before OneDomain may perform financial recovery; it retains the
original duty/reservation if any earlier judgment fails. Unknown/conflicting
receipt never supplies qualified recovery evidence. Expiry may trigger a request
for newly authorized observation/reconciliation; it cannot authorize an expired
query or supply NonreceiptProof. A ClaimId is not an AttemptId;
changing attempt never creates a second escrow/claim or permits another mint.
The source and destination are never a global atomic transaction.

## 12. Staking, restaking and yield

**Desired outcome:** deposit100 USD for vault shares, account for authenticated
reward/slash, reserve100 shares for unbond, and withdraw their remaining backing
after round200. **Status:** shares, priorities and all transitions specified;
reward/slash verification, competing custody claims and certified redemption open.

```mori
library Staking {
  struct VaultState<A,S> { backing: Qty<A>, supply: Shares<S>, reward_index: UInt128,
    slash_cursor: UInt128, locks: Locks<A> }
  enum WithdrawalStatus { Pending, MatureUncommitted, Paid }
  struct Withdrawal<A,S> { owner: Account<domain(A)>, shares: Shares<S>, status: WithdrawalStatus,
    owed: Qty<A>, maturity: Round<domain(A)>, lock_rank: UInt128, tier: RiskTierId, paid: Qty<A> }
  // Deposit: s=floor(deposit*supply/backing), backing+=deposit, supply+=s;
  // dust benefits Reserve, nonzero existing supply/backing required in this profile.
  relation Deposit<A,S>(vault: VaultState<A,S>, deposit: Qty<A>,
    book: RiskTierBook<A,S>, owner: Account<domain(A)>,
    withdrawals: List<Withdrawal<A,S>,16>, duties: List<Duty<A>,16>)
    -> Certified<(VaultState<A,S>,Shares<S>,RiskTierBook<A,S>)>;
  // Restricted Deposit: require no Pending/MatureUncommitted claims, no current
  // withdrawal duties, and no active withdrawal book entries under complete same-head
  // registry authentication. A zero owed unburned claim is still active. Paid-only
  // entries have reserved shares0/owed0, no duty, and remain unchanged.
  // Reward: actual funded reward enters backing, supply unchanged, index moves once.
  relation Reward<A,S>(vault: VaultState<A,S>, reward: Qty<A>, event: RewardEvidence,
    book: RiskTierBook<A,S>,withdrawals: List<Withdrawal<A,S>,16>,duties: List<Duty<A>,16>)
    -> Certified<(VaultState<A,S>,RiskTierBook<A,S>)>;
  // Slash: evidence-bound debit from backing to signed slash beneficiary; supply unchanged;
  // All active pending/mature-uncommitted withdrawals and free shares in VaultEquity
  // share one proportional risk tier. External slash-claim ranks select eligible
  // collateral/beneficiary; they never prioritize losses between same-tier holders.
  // Pending AND mature-but-uncommitted withdrawals remain slashable; paid claims do not.
  relation Slash<A,S>(vault: VaultState<A,S>, withdrawals: List<Withdrawal<A,S>,16>,
    loss: Qty<A>, event: SlashEvidence, book: RiskTierBook<A,S>)
    -> Certified<(VaultState<A,S>,List<Withdrawal<A,S>,16>,RiskTierBook<A,S>)>;
  // Unbond transfers owner shares into reservation, not burn; quote=floor(backing*s/supply).
  relation Unbond<A,S>(vault: VaultState<A,S>, shares: Shares<S>, maturity: Round<domain(A)>,
    book: RiskTierBook<A,S>) -> Certified<(Withdrawal<A,S>,RiskTierBook<A,S>)>;
  // Withdraw consumes the exact latest certified tier-book owed (not a new loss policy),
  // requires maturity/final slash cursor/priority;
  // burn reserved shares, debit backing, credit owner, paid=owed, discharge duty atomically.
  relation Withdraw<A,S>(vault: VaultState<A,S>, claim: Withdrawal<A,S>, now: Round<domain(A)>, book: RiskTierBook<A,S>)
    -> Certified<(VaultState<A,S>,Withdrawal<A,S>,Qty<A>,RiskTierBook<A,S>)>;
}

agreement YieldVault {
  header(profile: Staking,id: YieldVault,actions: [Deposit,Reward,Unbond,Slash,Withdraw]);
  share_class VaultShares: ShareClassId<Preview> = id("VaultShares");
  instrument Vault: InstrumentId<Preview> = id("Vault");
  intent YieldPromise {
    fixed { owner: Alice; share_class: VaultShares; vault: Vault; deposit: 100.00 USD;
      reward: 110.00 USD; loss: 121.00 USD; slash_to: Bob; unbond: shares(VaultShares,100);
      maturity: round(Preview,200); rounding: Floor(Reserve);
      slash_policy: SameTierProportional(tier: VaultEquity,
        participants: [PendingWithdrawal,MatureUncommittedWithdrawal,FreeShares],
        rounding: Floor(Reserve),paid_claims: Excluded);
      external_lock_priority: [SeniorRestakingSlash,JuniorRestakingSlash];
      protected_senior_backing: 0.00 USD;
      slashable_until: AcceptedWithdrawal; }
    holes {}; bounds { Alice_USD: budget(subject: Alice,gross: 100.00 USD,fees: 0.00 USD);
      Treasury_USD: budget(subject: Treasury,gross: 110.00 USD,fees: 0.00 USD);
      Vault_USD: budget(subject: Vault,gross: 221.00 USD,fees: 0.00 USD); work: 5; }
    receipts []; // Withdrawn requires exact certified post-loss owed payment, with no fixed profit floor
    authority { Deposit: grant(holder: Owner,roles: [Sign,Mint],signers: owners([Owner,Reserve]));
      Reward: grant(holder: Reserve,roles: [Sign],signers: owners([Reserve]));
      Unbond: grant(holder: Owner,roles: [Sign],signers: owners([Owner]));
      Slash: grant(holder: Keeper,roles: [Slash,Enforce],signers: owners([Reserve,Creditor]));
      Withdraw: grant(holder: Owner,roles: [Sign,Burn],signers: owners([Owner,Reserve])); }
    evidence { vault: VaultAtHead(Vault); shares: ShareSupplyAtHead(VaultShares);
      capability: ShareCapability(VaultShares); reward: FinalRewardEvent(Vault);
      slash: FinalSlashEvent(Vault); priority: AllRestakingLocks(Vault); clock: LedgerClock(Preview);
      arithmetic: VaultArithmetic(Vault); registry: CompleteWithdrawalRegistryAtHead(Vault,maximum: 16);
      duty_registry: CompleteWithdrawalDutyRegistryAtHead(Vault,maximum: 16); }
    outcomes { settled: Withdrawn(owner: Alice,claim: claim("Unbond1")); rejected: reject(first);
      pending: WithdrawalDuty(claim: claim("Unbond1"),terms: Terms.commitment,
        continuation: [Slash,Withdraw],recovery: []); }
  }
  stage Deposit on Preview uses YieldPromise {
    operation staking.deposit(owner: Alice,shares: VaultShares,backing: 100.00 USD);
    pre { owner: balance(Alice,USD); shares: shares(VaultShares,Alice); vault: vault(Vault,USD,VaultShares);
      book: risk_tier_book(Vault,VaultEquity); withdrawals: withdrawals(Vault,USD,VaultShares);
      duties: withdrawal_duties(Vault,USD); }
    derive Staking.Deposit(pre.vault,100.00 USD,pre.book,Alice,pre.withdrawals,pre.duties) as next;
    post { owner=pre.owner-100.00 USD; shares=pre.shares+next.value.1; vault=next.value.0; book=next.value.2; unchanged withdrawals; unchanged duties; }
    effects [Debit(Alice,100.00 USD),Credit(Vault,100.00 USD),Mint(Alice,next.value.1),SetCell(pre.vault,post.vault),SetCell(pre.book,post.book)];
    footprint reads(pre.owner,pre.shares,pre.vault,pre.book,pre.withdrawals,pre.duties) writes(pre.owner,pre.shares,pre.vault,pre.book);
    authority YieldPromise.Deposit; evidence YieldPromise.evidence;
    kernel OneDomain(authority: YieldPromise.Deposit,evidence: YieldPromise.evidence);
    failures Stage[WithdrawalRegistryProof] Effect[ActiveWithdrawalUnsupported,ZeroSupply,ShareRounding,BackingConservation]; outcome reject(first);
  }
  stage Reward on Preview uses YieldPromise {
    operation staking.reward(shares: VaultShares,amount: 110.00 USD);
    pre { payer: balance(Treasury,USD); vault: vault(Vault,USD,VaultShares);
      book: risk_tier_book(Vault,VaultEquity); withdrawals: withdrawals(Vault,USD,VaultShares);
      duties: withdrawal_duties(Vault,USD); }
    derive Staking.Reward(pre.vault,110.00 USD,evidence.reward,pre.book,pre.withdrawals,pre.duties) as next;
    post { payer=pre.payer-110.00 USD; vault=next.value.0; book=next.value.1; unchanged withdrawals; unchanged duties; }
    effects [Debit(Treasury,110.00 USD),Credit(Vault,110.00 USD),SetCell(pre.vault,post.vault),SetCell(pre.book,post.book)];
    footprint reads(pre.payer,pre.vault,pre.book,pre.withdrawals,pre.duties) writes(pre.payer,pre.vault,pre.book);
    authority YieldPromise.Reward; evidence YieldPromise.evidence;
    kernel OneDomain(authority: YieldPromise.Reward,evidence: YieldPromise.evidence);
    failures Stage[RewardEvidence,WithdrawalRegistryProof] Effect[ActiveWithdrawalUnsupported,UnfundedReward,DuplicateRewardIndex]; outcome reject(first);
  }
  stage Unbond on Preview uses YieldPromise {
    operation staking.unbond(owner: Alice,shares: VaultShares,share_atoms: 100);
    pre { owner: shares(VaultShares,Alice); vault: vault(Vault,USD,VaultShares);
      book: risk_tier_book(Vault,VaultEquity); claim: optional_withdrawal("Unbond1",USD,VaultShares); duty: withdrawal_duty("Unbond1",USD); }
    derive Staking.Unbond(pre.vault,shares(VaultShares,100),round(Preview,200),pre.book) as next;
    post { owner=pre.owner-shares(VaultShares,100); claim=Some(next.value.0);
      book=next.value.1; duty=withdrawalDuty(next.value.0); unchanged vault; }
    effects [SetCell(pre.owner,post.owner),SetCell(pre.claim,post.claim),SetCell(pre.book,post.book),SetCell(pre.duty,post.duty)];
    footprint reads(pre.owner,pre.vault,pre.book,pre.claim,pre.duty) writes(pre.owner,pre.book,pre.claim,pre.duty);
    authority YieldPromise.Unbond; evidence YieldPromise.evidence;
    kernel OneDomain(authority: YieldPromise.Unbond,evidence: YieldPromise.evidence);
    failures Intent[DuplicateUnbond] Effect[ShareBalance,ReservationConservation]; outcome reject(first);
  }
  stage Slash on Preview uses YieldPromise {
    operation staking.slash(shares: VaultShares,amount: 121.00 USD);
    pre { vault: vault(Vault,USD,VaultShares); claims: withdrawals(Vault,USD,VaultShares);
      book: risk_tier_book(Vault,VaultEquity); beneficiary: balance(Bob,USD); duties: withdrawal_duties(Vault,USD); }
    derive Staking.Slash(pre.vault,pre.claims,121.00 USD,evidence.slash,pre.book) as next;
    post { vault=next.value.0; claims=next.value.1; book=next.value.2; beneficiary=pre.beneficiary+121.00 USD;
      duties=withdrawalDuties(post.claims); }
    effects [Debit(Vault,121.00 USD),Credit(Bob,121.00 USD),SetCell(pre.vault,post.vault),
      SetCell(pre.claims,post.claims),SetCell(pre.book,post.book),SetCell(pre.duties,post.duties)];
    footprint reads(pre.vault,pre.claims,pre.book,pre.beneficiary,pre.duties) writes(pre.vault,pre.claims,pre.book,pre.beneficiary,pre.duties);
    authority YieldPromise.Slash; evidence YieldPromise.evidence;
    kernel OneDomain(authority: YieldPromise.Slash,evidence: YieldPromise.evidence);
    failures Stage[SlashEvidence] Effect[DuplicateSlash,CompetingLockPriority,LossConservation]; outcome reject(first);
  }
  stage Withdraw on Preview uses YieldPromise {
    operation staking.withdraw(owner: Alice,shares: VaultShares,claim: claim("Unbond1"));
    pre { owner: balance(Alice,USD); vault: vault(Vault,USD,VaultShares);
      claim: withdrawal("Unbond1",USD,VaultShares); book: risk_tier_book(Vault,VaultEquity);
      duty: withdrawal_duty("Unbond1",USD); now: current_round(PreviewClock); }
    derive Staking.Withdraw(pre.vault,pre.claim,pre.now,pre.book) as next;
    post { owner=pre.owner+next.value.2; vault=next.value.0; claim=next.value.1; book=next.value.3; duty=None; unchanged now; }
    effects [BurnReservation(claim: claim("Unbond1"),amount: pre.claim.shares),Debit(Vault,next.value.2),Credit(Alice,next.value.2),
      SetCell(pre.vault,post.vault),SetCell(pre.claim,post.claim),SetCell(pre.book,post.book),SetCell(pre.duty,None)];
    footprint reads(pre.owner,pre.vault,pre.claim,pre.book,pre.duty,pre.now) writes(pre.owner,pre.vault,pre.claim,pre.book,pre.duty);
    authority YieldPromise.Withdraw; evidence YieldPromise.evidence;
    kernel OneDomain(authority: YieldPromise.Withdraw,evidence: YieldPromise.evidence);
    failures Stage[SlashCursorUnfinalized] Intent[WithdrawalMaturity]
      Effect[Priority,ClaimConservation,AlreadyPaid]; outcome reject(first); pending YieldPromise.outcomes.pending;
  }
  episode YieldLife uses YieldPromise { stages: [Deposit,Reward,Unbond,Slash,Withdraw];
    links: accepted_head_chain; residual: YieldPromise.outcomes.pending; terminal: YieldPromise.outcomes.settled; }
}
```

### Exact proportional slash relation

```mori
library Staking {
  nominal type RiskTierId;
  struct TierHolding<S> { owner: PartyId, shares: Shares<S>, claim: Option<ClaimRef>,
    state: HolderState, lock_rank: UInt128, owed_atoms: UInt128, paid_atoms: UInt128 }
  enum HolderState { Free, Pending, MatureUncommitted, Paid }
  struct RiskTierBook<A,S> { tier: RiskTierId, backing: Qty<A>, protected_senior: Qty<A>,
    holdings: List<TierHolding<S>,32>, dust: Qty<A>, cursor: UInt128 }
  // Let B be active VaultEquity backing (not protected senior backing), S active supply,
  // L the authenticated loss funded by that tier, 0<=L<=B; require every active holding
  // counted exactly once, sum(active share weights)=S and no Paid holding has active shares.
  // B'=B-L, S'=S; for each active holding with weight s_i,
  // owed_i'=floor(B'*s_i/S) at asset atom precision;
  // dust'=B'-sum(owed_i'), reserved for Reserve and unspendable by another claimant.
  // Pending/Mature claim cells and duties take their exact book entry owed_i'; Free
  // entitlements are updated in book; Paid entries/paid wallet balances remain unchanged.
  // Require vault.slash_cursor=book.cursor; consume the exact slash event once
  // and increment both cursors c->c+1; preserve all claim IDs,
  // terms, maturity, share weights and lock ranks. Retain duty until accepted Withdraw.
}
```

Deposit reads the finite complete withdrawal registry and current-duty registry
(max16 entries each) plus tier book at the same authenticated head. Its financial
read footprint is owner balance, owner shares, vault, book, withdrawals and duties;
its write footprint is owner balance/shares, vault and book. Withdrawals/duties
are explicitly framed unchanged because the supported domain has none active.
Stage requires complete membership/completeness proofs of both source-bound
registry cells, with the certified book naming exactly their active entries; partial enumeration, missing registry or a bound violation rejects,
never an invisible dynamic scan or default empty list. Effect checks the explicit
NoActiveWithdrawals predicate before recomputation. This is a financial profile
restriction, with no maintainer/developer admission permission involved.

A Pending or MatureUncommitted claim, any undischarged withdrawal duty, or an
active book withdrawal entry rejects ActiveWithdrawalUnsupported atomically;
zero owed but unburned/unsettled claims still reject. Paid-only entries with
shares0/owed0 and no remaining duty are allowed and preserved. The same finite
registry reads/precondition apply to Reward. Supporting deposits/rewards while
withdrawals are active requires a future complete bounded claim/duty update
relation and exact footprint, not partial book edits or a weakened safety bound.

Within that supported domain, Deposit adds the minted free-share entry and
recomputes the certified tier book.
Reward updates its backing/entitlements and index; this displayed restricted
Reward requires no active withdrawals and occurs before Unbond. A reward with
pending claims needs a further explicit claim/duty update profile, not partial
book edits. Unbond moves the same shares from a free entry to a pending entry
with the new claim, preserving total share weight/backing. Slash updates all
active entitlements, claim cells and duties together. Withdraw marks that entry
Paid with active shares zero and records the paid amount, updates book backing
and total active share weight, burns supply and discharges the matching duty
atomically. The book never becomes a stale independent copy of supply/custody.

The signed `SameTierProportional` policy places all active share holders and
withdrawals in VaultEquity. A withdrawal's lock_rank does **not** make it a
first-loss holder; it identifies the external encumbrance ordering. The separate
external_lock_priority chooses the eligible restaking claim/beneficiary and
capacity before L is authorized; an attempt to consume a lower-priority lock
while a higher-priority encumbrance reserves that capacity rejects
CompetingLockPriority. The accepted loss is then allocated to the entire exposed
tier exactly once. Different external events must use the remaining custody,
current cursor/head and cumulative slash bounds, never independent copies of it.

Protected senior backing is a distinct verified reserve excluded from B and
from slash eligibility. YieldPromise fixes it to zero. If a further profile
introduces protected senior claims P>0, it must expose P in the same-head book
and prove custody_total=B+P; the ratio for active equity is `(B-L)/B`, not
`(custody_total-L)/custody_total`, while senior P stays unchanged. A proposed
loss exceeding eligible B rejects rather than invading protected backing.
Another holder risk tier needs a separately signed allocation rule; this example
cannot infer one from lock_rank or a generic priority list.

Withdraw must consume the latest certified owed entry/cursor, confirm all claims
and reserve/dust accounting agree, then burn only that claim's reserved shares
and pay exactly owed. A zero-valued post-loss claim is retained until an accepted
zero-payment/burn/discharge transition; it is never silently dropped. Maturity
changes eligibility to withdraw but not risk allocation. The same allocation
applies until acceptance; an already paid claim is excluded from subsequent
losses and cannot be clawed back by this policy.

`BurnReservation` is a typed share effect consuming the Unbond claim's reserved
shares and reducing total supply; it cannot burn cash or somebody's free shares.
`withdrawalDuty`/`withdrawalDuties` preserve claim IDs, maturity, external lock
rank, risk tier and Terms digest, with owed equal to the exact latest certified
tier-book claim value. An initial vault
1000 USD/1000 shares yields100 new shares for100 USD. Reward110 gives1210 USD
backing/1100 shares; unbond100 initially quotes110 USD. Slash121 (10%) before
maturity leaves1089 backing and the claim99 USD. Withdraw burns100 reserved
shares, pays99, leaves990 backing/1000 shares and no claim duty. With no reward,
a 110 USD slash of1100 backing would instead leave that100-share claim90 USD.
The signed policy chooses pending and mature uncommitted withdrawals to absorb
proportional loss; a pending withdrawal cannot race a higher-priority slash by
using stale state. Double-restaking custody requires the aggregate Locks relation;
not every numeric claim is independently spendable.

## 13. Composed bridge plus AMM episode: one complete signed promise

**Desired outcome:** escrow100 USD, receive100 WUSD, spend at most100 WUSD
including AMM fee, and deliver at least0.900 GOLD_F to **AliceF**. The owner
also signs two different recovery alternatives: qualified100 USD origin refund,
or100 WUSD retained in AliceF's destination custody after an atomic swap
rejection. They are distinct terminal branches with distinct assets/net floors.
**Status:** all source bindings/refinement/continuation interfaces proposed;
no implemented full episode grammar, verified bridge, AMM or persisted duty claim.

The source recipient is Alice, destination recipient AliceF; the optional kernel
cannot substitute another. Source escrow and destination receipt are ledger
accepted independently. Final delivery must refine the same parent Promise at
every step. Program/schema definitions from AMM and Bridge are imported as typed
profiles; the concrete stages below instantiate them under this parent, rather
than reuse BridgeTransfer's independent intent or authority.

```mori
library Composed {
  enum EpisodePhase { Draft, Escrowed, ReceiptUnknown, Received, SwapRejected,
                      Delivered, OriginRefunded, DestinationReturned }
  struct EpisodeJournal { pair: Bridge.ClaimPair, intent: IntentRef, phase: EpisodePhase,
    accepted: List<AcceptedStageFact,32>, attempts: List<AttemptFact,8>,
    gross: AssetVector<UInt128,8>, fees: AssetVector<UInt128,8>, duties: List<DutyRef,16> }
  enum Finish {
    Delivered(recipient: Account<Foreign>, net: Qty<GOLD_F>),
    OriginRefund(recipient: Account<Preview>, net: Qty<USD>),
    DestinationReturn(recipient: Account<Foreign>, net: Qty<WUSD>)
  }
  relation Refine(parent: SignedIntent<BridgeSwap>, pre: EpisodeJournal,
    completion: Completion<BridgeSwap>, next: DerivedStageFact, branch: Option<Finish>)
    -> EpisodeJournal;
  // Append one verified stage fact, preserve Pair and parent digest; add gross/fees
  // without netting credits/refunds; enforce counters on every prefix;
  // branch=None carries residual duty; terminal branch must satisfy its exact receipts.
  // Resolve parent.heads[next.StageId] using the exact accepted economic fact;
  // compare and consume that head atomically; preserve the original edge on reconciliation.
}

agreement BridgeAndSwap {
  import Horizon; import Examples; import AMM; import Bridge; import Composed;
  agreement_id id("BridgeAndSwap");
  episode_id id("BridgeSwapEpisode1");
  pool DestinationPool: PoolId<Foreign> = id("ForeignSpot");
  const Pair = Bridge.ClaimPair(economic: id("BridgeSwapEconomic1"),
    source: id("BridgeSwapSource1"),destination: id("BridgeSwapDest1"));
  intent Promise: SignedIntent<BridgeSwap> {
    agreement: id("BridgeAndSwap"); issuer: Owner;
    policy: Terms.id; policy_digest: Terms.commitment;
    selected: [program<Bridge>(id: "BridgeV1",source: digest("bridge-v1"),policy: Terms.commitment),
      program<AMM>(id: "ForeignAMMV1",source: digest("amm-v1"),policy: digest("foreign-pool-fee-v1")),
      program<Composed>(id: "BridgeSwapV1",source: digest("compose-v1"),policy: Terms.commitment)];
    signers: { Preview: Signers(keys: [OwnerPreviewKey],threshold: 1),
      Foreign: Signers(keys: [OwnerForeignKey],threshold: 1) };
    nonce: { Preview: nonce(Preview,"episode1-source"), Foreign: nonce(Foreign,"episode1-destination") };
    validity: { Preview: WindowP, Foreign: WindowF };
    heads: { stages: [
      stage_head(Escrow,Initial(anchor: head(Preview,"p0"))),
      stage_head(Claim,Initial(anchor: head(Foreign,"f0"))),
      stage_head(Swap,PreviousAcceptedHead(agreement: id("BridgeAndSwap"),claim: Pair,
        predecessor: Claim,fact: exactAcceptedFact(Foreign,Pair,Claim))),
      stage_head(OriginRefund,PreviousAcceptedHead(agreement: id("BridgeAndSwap"),claim: Pair,
        predecessor: Escrow,fact: exactAcceptedFact(Preview,Pair,Escrow))),
      stage_head(DestinationReturn,PreviousAcceptedHead(agreement: id("BridgeAndSwap"),claim: Pair,
        predecessor: Claim,fact: exactAcceptedFact(Foreign,Pair,Claim)))],
      unrelated_progress: RequiresFreshAuthorization };
    observational_scopes: [observational_rule(action: Reconcile,query: id("BridgeSwapReconcile1"),
      selector: paired_delivery_selector(Pair,maximum_attempts: 8),
      checkpoints: selected_finalized(Foreign,verifier("ForeignFinalityV1")),
      original_facts: [exactAcceptedFact(Preview,Pair,Escrow),exactAcceptedFact(Foreign,Pair,Claim)],
      service_work: 2,ledger_effects: [],ledger_controls: [])];
    fixed: { pair: Pair, source_owner: Alice, recipient: AliceF, origin_asset: USD,
      receipt_asset: WUSD, output_asset: GOLD_F, pool: DestinationPool,
      escrow: 100.00 USD, source_fee: 0.20 USD, source_fee_to: Treasury,
      destination_input: 99.70 WUSD, destination_fee_to: TreasuryF,
      final_net: 0.900 GOLD_F, representation: price<WUSD,USD,0>(1),
      recovery_origin: Alice, recovery_destination: AliceF, timeout: round(Foreign,300) };
    holes: { fee: QuantityHole<WUSD,31>(id: SwapFee,allowed: interval(0.00 WUSD,0.30 WUSD,0.01 WUSD)),
      output: QuantityHole<GOLD_F,64>(id: SwapOutput,allowed: interval(0.900 GOLD_F,0.963 GOLD_F,0.001 GOLD_F)) };
    bounds: { assets: [budget(subject: Alice,asset: USD,gross: 100.20 USD,fees: 0.20 USD),
      budget(subject: AliceF,asset: WUSD,gross: 100.00 WUSD,fees: 0.30 WUSD),
      budget(subject: AliceF,asset: GOLD_F,gross: 0.000 GOLD_F,fees: 0.000 GOLD_F)],
      work: 8, stages: 6 };
    receipts: [receipt(gate: Terminal(Delivered),recipient: AliceF,asset: GOLD_F,
        scope: StageDelivery(Swap,Pair),floor: 0.900 GOLD_F,deductions: allDeliveryCharges(Swap,GOLD_F)),
      receipt(gate: Terminal(OriginRefund),recipient: Alice,asset: USD,
        scope: StageDelivery(OriginRefund,Pair),floor: 100.00 USD,deductions: allDeliveryCharges(OriginRefund,USD)),
      receipt(gate: Terminal(DestinationReturn),recipient: AliceF,asset: WUSD,
        scope: ReservedCustody(Claim,Pair,DestinationReturn),floor: 100.00 WUSD,deductions: allClaimSpends(Pair,WUSD))];
    parties: { Issue: Owner, Fill: Keeper, Enforce: Keeper, Sign: Owner,
      Amend: None, Veto: None, Recover: Owner };
    grants: [EscrowGrant,ClaimGrant,SwapGrant,ObserveGrant,RecoverGrant,ReturnGrant,FillGrant];
    evidence: { source: verifier("MidnightFinalEscrowV1"), foreign: verifier("ForeignFinalityV1"),
      representation: verifier("USDToWUSD1to1V1"), selected: verifier("ExactProgramPolicyV1"),
      pool: verifier("ForeignPoolAtHeadV1"), arithmetic: verifier("AMMInvariantV1"),
      nonreceipt: verifier("FinalNonreceiptAndExclusionV1"), journal: verifier("EpisodeLinkV1") };
    outcomes: {
      Delivered: relation(recipient: AliceF,asset: GOLD_F,receipt_gate: Terminal(Delivered),
        require: [FinalClaim(Pair),AcceptedSwap(Pair),DischargedDuty(Pair)]),
      OriginRefund: relation(recipient: Alice,asset: USD,receipt_gate: Terminal(OriginRefund),
        require: [QualifiedNonreceipt(Pair),ExcludeLateClaim(Pair),SourceReleasedOnce(Pair)]),
      DestinationReturn: relation(recipient: AliceF,asset: WUSD,receipt_gate: Terminal(DestinationReturn),
        require: [FinalClaim(Pair),AtomicSwapRejected(Pair),NoSwapDebit(Pair),DestinationDutyDischarged(Pair)]),
      Pending: relation(retained: AcceptedHistory(Pair),duties: [CompleteOrReconcile(Pair)],
        reservation: RemainingClaimCustody(Pair)),
      Rejected: reject(first)
    };
    renewal: Some(RenewalPolicy(actors: {Preview: Signers([OwnerPreviewKey,ReservePreviewKey],2),
      Foreign: Signers([OwnerForeignKey,ReserveForeignKey],2)},
      permitted: [Claim,Swap,OriginRefund,DestinationReturn,Reconcile],
      preserve: OriginalClaimTermsBudgetsReceiptsAndHistory,
      validity: FreshSourceSignedWindow,head_change: FreshSourceSignedAnchor));
    disclosures: None;
  }
  authority {
    EscrowGrant = grant(parent: Promise,domain: Preview,holder: Owner,roles: [Sign,Enforce],actions: [Escrow],
      expiry: round(Preview,300),key_epoch: 3,revocation_epoch: 3,allowance: Promise.bounds,signers: owners([Owner]));
    ClaimGrant = grant(parent: Promise,domain: Foreign,holder: Owner,roles: [Sign,Mint],actions: [Claim],
      expiry: round(Foreign,400),key_epoch: 3,revocation_epoch: 3,allowance: Promise.bounds,signers: Signers(keys: [OwnerForeignKey,ReserveForeignKey],threshold: 2));
    SwapGrant = grant(parent: Promise,domain: Foreign,holder: Owner,roles: [Sign,Enforce],actions: [Swap],
      expiry: round(Foreign,400),key_epoch: 3,revocation_epoch: 3,allowance: Promise.bounds,signers: owners([Owner]));
    ObserveGrant = grant(parent: Promise,domain: Foreign,holder: Keeper,roles: [Enforce],actions: [Reconcile],
      expiry: round(Foreign,400),key_epoch: 3,revocation_epoch: 3,allowance: zeroDebit(work: 2),signers: owners([Owner]));
    RecoverGrant = grant(parent: Promise,domain: Preview,holder: Owner,roles: [Recover,Sign],actions: [OriginRefund],
      expiry: round(Preview,300),key_epoch: 3,revocation_epoch: 3,allowance: remainingBounds(Promise),signers: Signers(keys: [OwnerPreviewKey,ReservePreviewKey],threshold: 2));
    ReturnGrant = grant(parent: Promise,domain: Foreign,holder: Owner,roles: [Recover,Sign],actions: [DestinationReturn],
      expiry: round(Foreign,400),key_epoch: 3,revocation_epoch: 3,allowance: zeroDebit(work: 1),signers: owners([Owner]));
    FillGrant = grant(parent: Promise,domain: Foreign,holder: Keeper,roles: [Fill],actions: [Swap],
      expiry: round(Foreign,400),key_epoch: 3,revocation_epoch: 3,allowance: zeroDebit(work: 1),signers: owners([Owner]));
  }
  stage Escrow on Preview uses Promise {
    operation bridge.escrow(owner: Alice,asset: USD,amount: 100.00 USD,claim: Pair.source);
    pre { owner: balance(Alice,USD); fee: balance(Treasury,USD); escrow: escrow(Pair.source,USD);
      journal: episode_journal(Preview,Pair); }
    derive Bridge.Escrow(pre.escrow,100.00 USD,Pair) as next;
    post { owner=pre.owner-100.20 USD; fee=pre.fee+0.20 USD; escrow=next;
      journal=Composed.Refine(Promise,pre.journal,emptyCompletion(),acceptedStage(),None); }
    effects [Debit(Alice,100.20 USD),Credit(Pair.source,100.00 USD),Credit(Treasury,0.20 USD),
      SetCell(pre.escrow,next),SetCell(pre.journal,post.journal)];
    footprint reads(pre.owner,pre.fee,pre.escrow,pre.journal) writes(pre.owner,pre.fee,pre.escrow,pre.journal);
    authority EscrowGrant; evidence sourceBundle(Promise,Pair);
    kernel OneDomain(authority: EscrowGrant,evidence: sourceBundle(Promise,Pair));
    failures Intent[SourceRecipient,SourceFeeCap] Effect[EscrowConservation,CumulativeGross];
    outcome reject(first); pending Promise.outcomes.Pending;
  }
  stage Claim on Foreign uses Promise {
    operation bridge.claim(recipient: AliceF,asset: WUSD,amount: 100.00 WUSD,claim: Pair.destination);
    pre { owner: balance(AliceF,WUSD); receipt: receipt(Pair.destination,WUSD);
      nullifier: nullifier(Pair.destination); journal: episode_journal(Foreign,Pair); }
    derive Bridge.Claim(evidence.source,pre.receipt,evidence.representation) as next;
    post { owner=pre.owner+100.00 WUSD; receipt=next; nullifier=Consumed(Pair);
      journal=Composed.Refine(Promise,pre.journal,emptyCompletion(),acceptedStage(),None); }
    effects [MintAsset(AliceF,100.00 WUSD),SetCell(pre.receipt,next),SetCell(pre.nullifier,post.nullifier),SetCell(pre.journal,post.journal)];
    footprint reads(pre.owner,pre.receipt,pre.nullifier,pre.journal) writes(pre.owner,pre.receipt,pre.nullifier,pre.journal);
    authority ClaimGrant; evidence foreignBundle(Promise,Pair);
    kernel OneDomain(authority: ClaimGrant,evidence: foreignBundle(Promise,Pair));
    failures Stage[ForeignProof,SourceFinality] Intent[Recipient,AssetRepresentation]
      History[NullifierConsumed]; outcome reject(first); pending Promise.outcomes.Pending;
  }
  stage Swap on Foreign uses Promise {
    operation amm.swap_exact_input(pool: DestinationPool,owner: AliceF,input: 99.70 WUSD,
      output_asset: GOLD_F,receipt_gate: Terminal(Delivered),fee_cap: 0.30 WUSD);
    completion { fee: Promise.holes.fee; output: Promise.holes.output; proof: AMMInvariantProof(DestinationPool); }
    pre { input: balance(AliceF,WUSD); output: balance(AliceF,GOLD_F); fee: balance(TreasuryF,WUSD);
      pool: pool(DestinationPool,WUSD,GOLD_F); receipt: receipt(Pair.destination,WUSD);
      journal: episode_journal(Foreign,Pair); }
    derive AMM.ExactInput(pre.pool,99.70 WUSD,completion) as next;
    post { input=pre.input-99.70 WUSD-completion.fee; output=pre.output+completion.output;
      fee=pre.fee+completion.fee; pool=next.value; unchanged receipt;
      journal=Composed.Refine(Promise,pre.journal,completion,acceptedStage(),
        Some(Delivered(recipient: AliceF,net: completion.output))); }
    effects [Debit(AliceF,99.70 WUSD+completion.fee),Credit(DestinationPool,99.70 WUSD),
      Credit(TreasuryF,completion.fee),Debit(DestinationPool,completion.output),Credit(AliceF,completion.output),
      SetCell(pre.pool,next.value),SetCell(pre.journal,post.journal)];
    footprint reads(pre.input,pre.output,pre.fee,pre.pool,pre.receipt,pre.journal)
      writes(pre.input,pre.output,pre.fee,pre.pool,pre.journal);
    authority SwapGrant; evidence poolBundle(Promise,Pair,DestinationPool);
    kernel OneDomain(authority: SwapGrant,evidence: poolBundle(Promise,Pair,DestinationPool));
    failures Stage[ReceiptUnqualified] Intent[Recipient,OutputFloor,FeeCap]
      Effect[Invariant,CumulativeGross,CumulativeFees,CompleteEffects];
    outcome reject(first); pending Promise.outcomes.Pending;
  }
  stage OriginRefund on Preview uses Promise {
    operation bridge.recover(owner: Alice,asset: USD,claim: Pair.source);
    pre { owner: balance(Alice,USD); escrow: escrow(Pair.source,USD); journal: episode_journal(Preview,Pair); }
    derive Bridge.Recover(pre.escrow,evidence.nonreceipt) as next;
    post { owner=pre.owner+100.00 USD; escrow=next;
      journal=Composed.Refine(Promise,pre.journal,emptyCompletion(),acceptedStage(),
        Some(OriginRefund(recipient: Alice,net: 100.00 USD))); }
    effects [Debit(Pair.source,100.00 USD),Credit(Alice,100.00 USD),SetCell(pre.escrow,next),SetCell(pre.journal,post.journal)];
    footprint reads(pre.owner,pre.escrow,pre.journal) writes(pre.owner,pre.escrow,pre.journal);
    authority RecoverGrant; evidence recoveryBundle(Promise,Pair);
    kernel recover(authority: RecoverGrant,evidence: recoveryBundle(Promise,Pair));
    failures Stage[NonreceiptUnqualified,LateDeliveryNotExcluded] Failure[UnsignedRecoveryBranch];
    outcome reject(first); pending Promise.outcomes.Pending;
  }
  stage DestinationReturn on Foreign uses Promise {
    operation Composed.return_received(pair: Pair,recipient: AliceF,asset: WUSD,amount: 100.00 WUSD); // horizon-only
    pre { owner: balance(AliceF,WUSD); receipt: receipt(Pair.destination,WUSD); journal: episode_journal(Foreign,Pair); }
    derive Composed.ReturnReceived(Promise,pre.owner,pre.receipt,pre.journal,evidence.swap_rejection) as next;
    post { unchanged owner; receipt=next.receipt; journal=next.journal; }
    effects [SetCell(pre.receipt,post.receipt),SetCell(pre.journal,post.journal)];
    footprint reads(pre.owner,pre.receipt,pre.journal) writes(pre.receipt,pre.journal);
    authority ReturnGrant; evidence returnBundle(Promise,Pair);
    kernel recover(authority: ReturnGrant,evidence: returnBundle(Promise,Pair));
    failures Stage[ReceiptUnqualified,SwapRejectionUnqualified] Effect[MissingDestinationCustody]
      Failure[UnsignedRecoveryBranch]; outcome reject(first); pending Promise.outcomes.Pending;
  }
  observation_action Reconcile on Foreign uses Promise {
    rule: Promise.observational_scopes.Reconcile;
    identity: Query(query: id("BridgeSwapReconcile1"),selector_digest: digest_of(rule.selector));
    evidence_reads [source_final_escrow(Pair.source),foreign_delivery_count(Pair.destination,8),
      foreign_nullifier(Pair.destination),foreign_exclusion_checkpoint(Pair.destination),
      original_claim_edge(Pair.destination,Claim)];
    economic_effects []; ledger_writes []; ledger_control_effects [];
    service_journal append_query_fact(identity,attempt,raw_receipt);
    authority ObserveGrant; evidence reconcileBundle(Promise,Pair);
    kernel reconcile(authority: ObserveGrant,evidence: reconcileBundle(Promise,Pair));
    failures Stage[ObservationCheckpoint,IncompleteAttemptCoverage] Authority[ObservationWork,ExpiredAuthorization]
      History[QueryAttemptBinding]; outcome observation_only;
  }
  episode BridgeThenSwap uses Promise {
    pair: Pair; attempts: per_domain(maximum: 8); stages: [Escrow,Claim,Swap,OriginRefund,DestinationReturn];
    start: Escrow;
    after Escrow: Claim;
    after Claim: match stageOutcome(Claim) {
      Settled => Swap;
      Rejected(first) => Pending(Promise.outcomes.Pending);
      Pending => Pending(Promise.outcomes.Pending);
    }
    on_reconciled: match qualifiedReceipt(Pair) {
      FinalReceipt => qualify_prior_claim_then(Swap);
      QualifiedNonreceipt => OriginRefund;
      Unknown => Pending(Promise.outcomes.Pending);
      Conflict => Pending(Promise.outcomes.Pending);
    }
    after Swap: match stageOutcome(Swap) {
      Settled => Finish(Promise.outcomes.Delivered);
      Rejected(first) => DestinationReturn;
      Pending => Pending(Promise.outcomes.Pending);
    }
    on_timeout: observe_reconcile(action: Reconcile,parent: Promise,grant: ObserveGrant,
      rule: Promise.observational_scopes.Reconcile,attempt: currentDurableQueryAttempt(),
      evidence: reconcileBundle(Promise,Pair));
    on_unknown: keep(Promise.outcomes.Pending);
    terminal: [Promise.outcomes.Delivered,Promise.outcomes.OriginRefund,Promise.outcomes.DestinationReturn];
  }
}
```

Promise.signers requires only OwnerPreviewKey/OwnerForeignKey on the parent
envelope. ClaimGrant additionally requires OwnerForeignKey **and**
ReserveForeignKey on the exact grant authorization and native mint payload;
ReserveForeignKey also verifies the separate WUSD mint capability. RecoverGrant
requires OwnerPreviewKey **and** ReservePreviewKey on its grant authorization
and native refund payload; ReservePreviewKey also binds escrow release authority.
These additional signatures are conjunctive resource constraints, not signatures
that replace Owner's envelope or enlarge its signed promise. Each key is bound
to its explicit domain, epoch3 and same-head revocation state. Every OneDomain, observe, reconcile and recover request collects these typed
conjunctive facts for the common schedule. An observational scope has no native
payload/signature obligation because it cannot sign or dispatch a financial
call; all actual native sign/dispatch scopes require that conjunction in full.

After Preview300, an otherwise qualified OriginRefund cannot execute under
RecoverGrant; after Foreign400, DestinationReturn cannot execute under
ReturnGrant. Their original duties/reservations remain Pending. Promise.renewal
permits a newly signed continuation for the listed existing actions, with both
Owner and Reserve in that domain, an explicitly new validity/head anchor and
all immutable claims/receipts/budgets/history preserved. A renewed grant may use
currently qualified replacement keys at epoch4, but cannot treat original
spent work/fees/gross as zero or authorize a second Escrow. Lack of renewal means
no execution, even when the recovery proof itself is qualified.

`sourceBundle`, `foreignBundle`, `poolBundle`, `recoveryBundle` and `returnBundle`
are closed constructors of `EvidenceBundle<Domain,Profile>`: all share exact
parent signature, selected program/policy, same-head reads, revocations,
predecessor/successor and atomic-consumption facts. Source adds source balance,
escrow/custody and fee policy; Foreign adds source finality, representation,
mint authority and paired nullifier; Pool adds final receipt and certified
invariant/fee; Recovery adds finalized complete delivery count and future
exclusion; Return adds final receipt, authenticated no-debit swap rejection,
unchanged AliceF custody and duty-discharge relation. None chooses a verifier
outside Promise.evidence. `ReturnReceived` accepts only Received/SwapRejected,
receipt100 WUSD credited to AliceF still present/reserved, and no unresolved swap
send; it performs no origin refund/mint/new debit, discharges the destination
completion duty and retains the origin backing obligation. Receipt Returned here
means terminal custody return to its authorized owner, not bridge token burning.

`acceptedStage` is the verifier-derived current stage fact **before** journal
annotation; `Refine` annotates that fact and cannot certify itself. `qualify_prior_claim_then` verifies the already observed Claim effect under the
original stage/pair and links it to the journal; it does not send or mint again.
`currentAttempt` is a distinct domain-bound send ID from the durable journal. Its parent claim,
intent digest and accumulated counters cannot change. Common administrative
nonces are `(Promise nonce,StageId)`; retry uses the same economic StageId and
claim, with a new AttemptId and compare-and-consume preventing duplicate economic
acceptance. Reconcile uses the signed observational rule and finite evidence-read set from
section11, a Query identity, and section3's deferred/scheduled scope interface.
Its service journal supplies no economic predecessor or control effect.

Destination custody is reserved for this claim at receipt; AliceF cannot spend
it concurrently outside the episode. Core includes that reservation/allowance
cell in the common footprint. If the AMM succeeds with fee0.30 and output0.900,
source G_USD=100.20/F_USD=0.20; destination G_WUSD=100.00/F_WUSD=0.30;
net_GOLD_F=0.900. Source refund does not erase G_USD=100.20 or the retained0.20
source fee. The signed OriginRefund branch promises100 USD, not100.20; the fee
loss is explicit. DestinationReturn promises100 WUSD and keeps100 USD backing
escrowed; it is not an origin refund or satisfaction of Delivered's GOLD floor.

| Mutation / event | Exact controlling source relation | Result and duty |
| --- | --- | --- |
| Destination recipient changed to another account | Promise.fixed.recipient; Claim/Swap grants and signed Delivered recipient | Intent Recipient rejection, no stage effects; existing escrow/receipt duty survives. |
| AMM adds fee0.31 WUSD | Fee hole max0.30, WUSD gross100.00/fees0.30; fee must equal pinned pool rule | Hole formation/refinement failure, or Intent FeeCap before commitment; retain received100 WUSD. |
| Retry after Unknown allocates a new economic pair | Promise.fixed.pair; Refine preserves pair/parent; paired nullifier across all attempts | Intent ClaimPairBinding/History NullifierConsumed; old reservation/duty not reset. |
| Timeout without final nonreceipt/exclusion | OriginRefund require QualifiedNonreceipt+ExcludeLateClaim | Pending, preserve source escrow and CompleteOrReconcile duty. |
| Valid claim, atomic slippage rejection | DestinationReturn require FinalClaim+AtomicSwapRejected+NoSwapDebit | May discharge completion via signed100 WUSD return; origin backing remains. |
| Swap send is Unknown, then caller requests destination return | Return requires authenticated no-debit rejection and no unresolved send | Pending; reserve destination custody against possible late debit. |
| Wrapped funds returned but caller declares OriginRefund | OriginRefund assetUSD and recipientAlice versus DestinationReturn assetWUSD/recipientAliceF | Failure UnsignedRecoveryBranch; no cross-asset substitution. |
| A second successful Claim after changing AttemptId | Same pair/source economic ID and destination nullifier | History NullifierConsumed; no second mint, G/fees/work never reset. |

There is no global rollback. A rejected Swap publishes no new swap effects, while
accepted source escrow/fee and destination receipt remain in the parent history.
All terminal alternatives require the whole signed relation and their own
qualified evidence; no generic fallback waives its terms.

## 14. Construct reference, mappings and coverage

The following status/evidence table covers the syntax visible above. `Specified`
identifies written interface coverage, not acceptance of the grammar. `Open`
identifies the corresponding implementation/proof gap. The existing S0 row is
the only local execution baseline of this mockup; new beta status needs measured
results recorded separately.

| Construct group | Proposed meaning / Core or kernel mapping | Proposal status | Existing evidence / missing predicate |
| --- | --- | --- | --- |
| profile/library/import/context/header/agreement; nominal IDs/id | Versioned bounded declarations and pinned interface reuse; AgreementId/ProgramRef | specified | [Source/6](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md) local closed syntax differs; horizon total elaboration open. |
| asset/domain/account/party/pool/instrument/share_class/obligation | Nominal financial identity and role/custody references | specified | [beta design](BETA-DESIGN.md); domain/representation/capability authentication open. |
| UInt128/Nominal127/Int128/Qty/Delta/Position/Shares/Price/Conversion | Checked numeric categories, units and conversion/rounding direction | specified | S0 [numeric contract](../../experiments/moriarty-language/formal/mil4/s0-implementation-contract.md) local; horizon signed arithmetic/dimensional proofs open. |
| const/fn/bounded_work/struct/enum/generic/List/Set/Option/Result/tuple/match | Pure total bounded computation and exhaustive typed alternatives | specified | General libraries/ADTs are not beta grammar; termination/exhaustiveness/elaboration proofs open. |
| rounds/clock/Instant/Window/Round | Nominal clocks/windows; only same-domain comparison | specified | S0 rounds local; clock conversion/calendars/authenticated time open. |
| Floor/Ceil/Exact/mul_div/Certified/certify/relation/derive | Selected typed arithmetic/effect relation; proof-bearing advanced profile | specified | S0 no division/rounding; arithmetic certificate verification and native correspondence open. |
| intent/fixed/holes/Completion/Hole/QuantityHole/Finite/Interval | Fixed promise and finite solver alternatives; signed-template refinement | specified | S0 source-fixed action local; general completion/refinement open. |
| Bounds/budget/remainingBounds/zeroDebit/AssetVector/ReceiptRequirement/ReceiptGate/ReceiptScope | Cumulative per-subject/per-asset gross/fees/work; separate typed receipt gates/scopes | specified | S0 local caps/counters; cumulative history bounds open. |
| Grant/Signers/authority/owners/quorum/Role/AuthorizationConjunction/RenewalPolicy | Issue, Fill, Enforce, Amend, Recover, Sign, Veto, Mint/Burn/Slash and narrowing grants | specified | S0 signer/allowance local checks; signatures/revocation/quorum proof open. |
| Cell/pre/post/unchanged/read/write/Footprint | Same-head typed cells, exact complete derived footprint and framing | specified | S0 local complete vector; general profile footprint and authenticated reads open. |
| Encumbrance/Locks/Duty/Retained/RiskTierBook/TierHolding | Claims/ranks, aggregate custody, surviving liabilities and continuation terms | specified | S0 repayment obligation local; persistent cross-stage custody/duties open. |
| Effect/Debit/DebitCustody/Credit/SetCell/Mint/Burn/MintAsset/BurnAsset/BurnReservation | Ordered complete economic effects; Mint/Burn are resource-specific supply changes | specified | S0 transfer/funded repay [Core/5](../../experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts) local; other operations unsupported. |
| stage/operation/footprint/failures/outcome/reject/pending/HeadRule/HeadSchema | One-domain relation, first-failure publication contract, permitted nonterminal state | specified | Existing S0 terminal success/local atomic rejection; accepted failure/duty semantics open. |
| episode/links/after/start/terminal/on_timeout/on_unknown | Ledger-linked stages and explicit qualified continuation, no global rollback | specified | [MIL/4 semantics](../../experiments/moriarty-language/formal/mil4/semantics-contract.md); implementation/K/Quint continuation open. |
| EconomicClaimId/ClaimPair/AttemptId/NativeEffectId/Journal | Logical effect separate from send/native effect, durable unknown and counters | specified | [kernel recommendation](../kernel-api-recommendation-2026-09-29.md); append-only qualification/replay/retry proof open. |
| Observation/FinalObservation/Unit/EvidencePolicy/Provenance/Finality | Value, dimensional unit, subject, source, observed/current time and verifier assumptions | specified | Source/6 observations empty; qualified oracles/foreign evidence open. |
| ScopeRequest/KernelScope/ScopeIdentity/QueryId/scopeFor/DeferredFact/ObservationalRule/ServiceQueryJournal/EvidenceBundle/Receipt/NativeCall/effectSelector/verify helpers | Typed side-effect authority and evidence interfaces | specified | [kernel proposal](../kernel-api-recommendation-2026-09-29.md), not implemented qualification. |
| validatePlan/reserve/buildCall/authorizeSign/dispatch/observe/reconcile/recover | Distinct phase facts under the original signed authority | specified | Service acknowledgements are neither proof nor ledger acceptance; full adapter/codec/evidence correspondence open. |
| Source/6 transfer/repay preparation | Existing local comparison target only | local | [wrapper](../../experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts) retains four unverified bindings; [Core](../../experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts) retains four external premises. |

`Credit(ResourceId,...)`/`Debit(ResourceId,...)` sugar always resolves a
source-bound custody account proved by the resource capability. It does not cast
types or grant debit authority. `Mint`/`Burn` update both owner/reserved shares
and total supply; SetCell of a pool/vault mirrors those changes without a second
mint. `Balance` post arithmetic above abbreviates changing its `amount` field,
while preserving owner and cell identity. Profile constructors such as
`debtDuty`, `cashSettlementDuty`, `remainingRedemption`, `withdrawalDuty`,
`FinalClaim`, `ExcludeLateClaim` and `NoSwapDebit` are closed typed constructors
or predicates with the fields/equations specified in their family section. They
are not English labels evaluated by a runtime. An unknown constructor, enum tag,
argument or field fails formation. All symbolic keys and enum priority labels
must resolve in the pinned profile interface.

### Typed operation and beta-registry mapping

The horizon interface for each operation takes the fixed role/asset/quantity
arguments shown in source plus its typed pre-state/evidence; it returns a
`Derived<D>` with the relation and failures named in its section. This table
makes the remaining lifecycle calls visible. Registry names indicate planned
beta recognition, not acceptance of horizon statements or financial relations.
The exact beta argument schema remains the checker/design's own closed contract;
there is no automatic general-contract lowering.

| Horizon operation | Typed resource/value arguments | Derived relation | Planned beta name |
| --- | --- | --- | --- |
| Swap | PoolId<D>, Account<D>, Qty<A>, Asset<B>, Qty<B>, Qty<A> | AMM.ExactInput | `amm.swap_exact_input` |
| LP deposit | PoolId<D>, Account<D>, Qty<A>, Qty<B> | AMM.MintLP | `amm.mint` |
| LP redeem | PoolId<D>, Account<D>, Shares<S> / checked share_atoms | AMM.RedeemLP | `amm.redeem` |
| Originate | ObjectId<D>, debtor/creditor Account<D>, Qty<A>, Qty<B> | Lending.Originate | `lending.originate` |
| Roll | ObjectId<D>, Round<D> | Lending.RollForward | `lending.roll_forward` |
| Funded payment | ObjectId<D>, Account<D>, Qty<A> | Lending.FundedRepay / S0 AccrualFirst | `repay` (S0 only) |
| Liquidate | ObjectId<D>, pinned buyer/price/sale/trigger in parent | Lending.Liquidate | `lending.liquidate` |
| Stable mint | InstrumentId<D>, Account<D>, Qty<MUSD>, Qty<GOLD> | Stablecoin.Mint | `stablecoin.mint` |
| Burn/redeem | InstrumentId<D>, Account<D>, Qty<MUSD>, Qty<GOLD> | Stablecoin.Redeem | `stablecoin.redeem` |
| Shutdown | InstrumentId<D>, claim list, emergency policy | Stablecoin.Freeze | horizon-only, not registered |
| Emergency payment | InstrumentId<D>, Account<D>, Qty<MUSD>, qualified claim | Stablecoin.Emergency | `stablecoin.emergency_settle` |
| Premium/reserve | InstrumentId<D>, Qty<USD>, Qty<USD> | Option.Fund | horizon-only, not registered |
| Fix | InstrumentId<D>, SubjectId<Price<A,B,S>> | Option.Fix | `option.fix` |
| Exercise | InstrumentId<D>, Account<D> | Option.Exercise | `option.exercise` |
| Option payment | InstrumentId<D>, certified payoff | Option.Settle | `option.settle` |
| Read fixing | SubjectId<Price<A,B,S>>, selected EvidencePolicy | Oracle.Select | `oracle.select` |
| Queue/veto/enact | PolicyId, UInt128 epoch, signed timelock/quorum | Governance.Queue/Veto/Execute | `governance.queue/veto/execute` |
| Escrow | Account<Preview>, Asset<Preview>, Qty<A>, ClaimId<Preview> | Bridge.Escrow | `bridge.escrow` |
| Claim | Account<Foreign>, Asset<Foreign>, Qty<B>, ClaimId<Foreign> | Bridge.Claim | `bridge.claim` |
| Qualified refund | Account<Preview>, Asset<Preview>, ClaimId<Preview> | Bridge.Recover | `bridge.recover` |
| Reconcile / destination return | ClaimPair, typed journal, qualified send/effect/nonreceipt facts | Bridge.Classify / Composed.ReturnReceived | horizon/kernel-only, not registered |
| Stake deposit | Account<D>, ShareClassId<D>, Qty<A> | Staking.Deposit | `staking.deposit` |
| Funded reward / slash | ShareClassId<D>, Qty<A>, selected event/rank policy | Staking.Reward / Slash | `staking.reward/slash` |
| Unbond / withdraw | Account<D>, ShareClassId<D>, Shares<S> / ClaimId<D> | Staking.Unbond / Withdraw | `staking.unbond/withdraw` |

### Eight-family lifecycle coverage

Every “shown” item is source-level **specified** coverage; none establishes
implemented finance. The profile equations above are proposed semantic rules.
The linked evidence supplies existing requirements/specifications, not proof
that these horizon examples execute.

| Family | Source shown | Hard failure/continuation shown | Status / repository evidence |
| --- | --- | --- | --- |
| AMMs and exchanges | Swap, fee/floor, reserve/custody/invariant, LP mint and redeem | Missing fee cell/incorrect vector; invariant/floor rejection, no new effects | specified/open; [MIL/2 categories](../../deliverables/mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md), §5. |
| Lending and borrowing | Originate, aggregate locks, single roll-forward, funded accrued-first repay, liquidation | Overencumbrance; 75 USD residual debt/default after sale | specified/open except existing S0 local relation; [S0 contract](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md), §6. |
| Stablecoins and synthetic assets | Backed mint, burn/redeem, typed peg, shutdown, emergency settlement | Existing claim preservation; below-floor emergency remains Pending | specified/open; [requirements](../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md), §7. |
| Derivatives | Premium/reserve funding, fixing, exercise, payoff and terminal settlement | Missing/disputed fixing retains500 USD and exercise/settlement duty | specified/open; [category coverage](../../deliverables/mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md), §8. |
| Oracles and observations | Typed price/value/unit/source/subject/time/finality/provenance; selection | now156 stale; dispute Pending; aggregation quorum/independence assumptions explicit | specified/open; [beta types](BETA-DESIGN.md), §9. |
| Governance | Queue, veto, timelock, execute; multi-signer authority | Veto/missing final veto evidence; signed duties keep old digest/liability | specified/open; [product contract](../../docs/MORIARTY-PRODUCT-CONTRACT.md), §10. |
| Bridges and cross-domain settlement | Escrow/claim, paired IDs, foreign proof, reconcile and qualified refund | Unknown/timeouts retain reservation; excludes late delivery before refund | specified/open; [kernel recommendation](../kernel-api-recommendation-2026-09-29.md), §11. |
| Staking, restaking and yield | Deposit/share conversion, funded reward, slash/competing priority, unbond and withdraw | 100-share pending claim reduced to99 USD; maturity/priority/finality required | specified/open; [category coverage](../../deliverables/mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md), §12. |
| Cross-family composition | BridgeThenSwap parent bounds, stages, finite holes and three terminal promises | Recipient/fee/attempt substitutions, Unknown and asset-specific recovery discriminators | specified/open; [audit04 H2](audits/04-language-horizon.md), §13. |

### Surface-requirement coverage and authority split

| Requirement area | Typed source location | Author / solver / provider / Core / kernel split | Status |
| --- | --- | --- | --- |
| Identity/assets | §1/4 nominal IDs, representations, scale; §13 ClaimPair | Author fixes identities; provider binds ledger/capability; Core rejects domain/kind changes | specified; binding open |
| Values/computation | §1 quantities/deltas/positions/prices/clocks, §5/8/12 rounding | Author selects bounded arithmetic; solver supplies required proof; verifier qualifies; Core checks relation | specified; numeric/certified correspondence open |
| Agreement/intent/holes | §2 signed template, §5/13 finite choices and budgets | Author signs fixed terms; Fill supplies only holes; Core refines complete promise | specified; exact signing codec/refinement open |
| State/time/footprints | §1 typed pre/post, common controls, each stage footprint | Provider authenticates complete same-head read set; Core derives effects/post/frame; kernel atomically commits | specified; framing/qualification open |
| Authority | §2 roles/grants/expiry/revocation, §10 quorum, §13 narrowing | Source names all roles; provider proves key/policy epoch; Core enforces residual authority | specified; authentication/linear consumption open |
| Evidence | §3 bundle/pipeline, §9 observation, §11 nonreceipt | Author pins policies; provider supplies raw proofs; selected verifier qualifies; no solver Boolean substitutes | specified; every external premise open |
| Outcomes/failure | §2 ordered judgments/outcomes, each family failures, §13 terminal alternatives | Core derives first failure and no new effects; episode retains previous accepted effects/duties | S0 local rejection baseline; horizon specified/open |
| Composition/libraries | §1/4 imported typed profiles, §6–13 ledger links | Author bounds interfaces and signs alternatives; solver cannot add protocols; optional kernel carries allowed external effects | specified; cross-layer/episode proofs open |

## 15. Recommended choices and remaining formal obligations

These recommendations expose unresolved choices; they are not a broader grammar
freeze. Source annotations `horizon-only`, pinned profile references and typed
certificate/evidence requirements keep their open status visible.

| Choice | Recommended disposition | Required formal/empirical obligation |
| --- | --- | --- |
| Price direction / reciprocity | Preserve Base-per-Quote; require explicit reciprocal conversion and beneficiary | Elaboration rejects reversed dimensions; prove numeric/rounding correspondence and no implicit reciprocals. |
| Simple predicates vs arithmetic | Closed checked pure operators for simple profile; selected total certified programs for mul/div, interest/share/payoff | Prove termination, widths, certificate soundness and native equality; uncertified arithmetic cannot set success. |
| Shares, rights and custody | Separate Qty/Delta/Position/Shares; consume shares/claims under capability; retain duties explicitly | Prove no-copy/no-double-spend and no unauthorized drop, aggregate encumbrance conservation and liability persistence. |
| Completion | Closed finite typed holes; fixed recipients/assets/program/policy; no record escape | Prove signed-template refinement, origin preservation, total bounded elaboration and complete effects/framing. |
| Canonical authority | One language-signed envelope; grants narrow parent; native payload codec separately pinned | M4-C1/C5 exact bytes/vectors, distinct-key signature thresholds, revocation/expiry/history proof; no new kernel signature meaning. |
| Failure/duty semantics | Preserve ordered first failure; atomic stage rejection; explicit signed Pending/recovery branches | K/Quint same outcome projection, retained-effect/cumulative-bound predicates, no failure-path waiver or timeout discharge. |
| Episodes / retries | Domain-bound claim/attempt/native IDs, append-only fact journal, compare-and-consume | Prove history links, no duplicate economic effect across attempts, unknown-finality reservation and late-delivery exclusion. |
| Governance | Library process plus typed effects/preservation; public compiler permissionless | Prove amendment preserves already signed terms/liabilities and old recovery duties; no maintainer gate in developer flow. |
| General libraries/ADTs | Bounded pure functions and exhaustive match; interface pins instead of unrestricted host import | Grammar/type soundness, total evaluation, profile monomorphization/work bounds and observational elaboration equivalence. |
| Next executable slice | Measured beta S0 authoring/lowering to existing Source/6/Core/5; next financial profile should expose swap full footprint | Independent transfer/repay command comparisons, range/first-failure/effect tests; later swap native/ledger evidence separate. |

K must model typed stage preparation/admission, exact effect order, first failure,
framing, cumulative authority and residual duty. Quint must model stage/episode
sign-submit-observe grain, reservation, replay, unknown/late delivery and qualified
recovery, with independent expected outcomes. Neither model's syntax/typecheck
establishes the correspondence itself. Elaboration must preserve origins and the
entire observation (formation versus Core rejection, first judgment/code,
null publication or complete effects/post/work/head/duties). Authentication must
bind exact signed bytes, program/policy/scale/agreement, complete same-head cells,
observations and predecessor/successor. Atomic ledger compare-and-consume must
publish all authorized effects, allowances, replay/history and surviving duties
together. W-D0–W-D4, M4-C1–C5, native proof and U-roadmap financial qualification
remain open until their own evidence is produced.

## 16. Focused v2 review repair and independent arithmetic discriminators

These are proposed dispositions of [audit04 v2](audits/04-horizon-v2.md) and
[audit05 v2](audits/05-horizon-v2.md), which reviewed the original hashes recorded
in their reports. Their dissent remains preserved. The equations and tables
below are source-level discriminators and independently evaluated integer
arithmetic; no horizon compiler, simulator, certificate verifier, K/Quint model
or ledger path ran. Fresh exact-byte concurrence remains pending.

| Finding | Before | Proposed repair / source discriminator |
| --- | --- | --- |
| 04 F1 / 05 H1: slash allocation | Claim-first priority but proportional99 USD witness | §12 signs SameTierProportional; external lock rank is separate, and certified book supplies all owed/claim/duty updates. The0 USD claim-first certificate rejects PolicyAllocationMismatch. |
| 04 F2 / 05 H2: net scope | Untagged budget net ambiguously applied to entire lifecycle and all branches | §2 removes net from Bounds; source-signed ReceiptRequirement has exact gate/recipient/asset/scope/deductions. §6 milestone500, §7 mint100/redemption0.190, §13 three terminal receipt gates. Gross/fees/work never reset. |
| 05 H3: continuation head | Composed p0/f0 anchors with no per-stage successor rule | §13 signs Initial for Escrow/Claim and exact PreviousAcceptedHead for Swap/OriginRefund/DestinationReturn; §2 rejects unrelated current heads and requires explicit fresh authorization. |
| 05 M1: kernel scope | Grant passed where KernelScope was required | §3/4 scopeFor collects parent/grant refs, Economic/Query identity, durable attempt and deferred authorization facts; scheduled Core qualification supplies KernelScope; §9/query and §11/13 reconcile/recover expansions now construct that exact scope. |
| 05 M2: signatures/expiry | Owner-only envelope and extra Reserve grant signers lacked conjunctive meaning | §2/4/13 distinguish parent/grant/resource/native signatures and explicit per-domain keys; expiry/revocation checks are conjunctive; late proof cannot execute without newly signed preserving authority. |
| 05 M3: liquidation cell | Account Debit(Alice,GOLD) with only pledged-custody write | §1/6 use DebitCustody targeting LoanPledge under exact capability, frame free balance unchanged, and remove only Loan lock. One asset transfer, no duplicate wallet debit. |
| 05 M4: pure functions/checklist | Function signatures only; checked boxes could imply contract acceptance | §1 gives bounded pure function body, reusable call and ordinary exhaustive ADT match; checklist and §14 explicitly describe proposal coverage, with all execution/proof/review gates open. |

### Slash before/after: one signed loss rule

All amounts here are USD; share weights are distinct from money. The scalar
calculations use integer cents, with floor only where stated.

| Literal pre-state / event | Rejected old interpretation | Expected under signed SameTierProportional |
| --- | --- | --- |
| B1210, S1100; pending100 shares owed110; free1000 shares owed1100; L121 | Claim-first: pending0/free1089 | B1089/S1100; pending99/free990; loss11+110=121; cursor7→8; pending duty99 survives until Withdraw. |
| Two pending claims:100 shares rank10 owed110;200 shares rank20 owed220; free800 shares owed880; B1210/S1100/L121 | Different lock ranks used to exhaust one holder before another | Post99/198/792, losses11/22/88; sum1089; ranks/IDs unchanged; both duties retained; beneficiary receives121; cursor7→8. |
| Same second claim matures before slash but is uncommitted | Maturity removes its exposure without accepted payment | Same99/198/792; maturity allows later withdrawal but cannot change allocation. |
| Add a separate Paid record with active shares0, owed0, already-paid55; active book still B1210/S1100 | Slash claws back paid55 or includes those inactive shares | Same active values; Paid record and paid wallet55 unchanged, no restored duty, cursor7→8. |
| B101, S3, one pending share and two free shares; L1 | Over-round credits to claimants | B100; pending33.33/free66.66, dust0.01 belongs to Reserve; sum including dust100. |
| Protected senior P100 and active equity B1210 (total custody1310), same active shares/L121 | Use total custody1310 as the holder denominator | Senior100 unchanged; equity1089; pending99/free990. Proportion uses B1210 only. |
| A different profile reports total1210 with senior100, hence B1110/S1100; L121 | Apply the prior99 witness despite a different exposed B | New B989;100-share claim floor89.90, free1000-share claim floor899.09, dust0.01; protected100 unchanged. This needs its own signed profile/receipt terms. |
| Slash event already consumed at cursor8, or lower-priority external lock lacks eligible capacity | Replay event or seize reserved higher-priority collateral | History/Effect rejection; no new debit, no second cursor increment, all previous claims/duties retained. |

The displayed YieldLife uses the first row. All zero-valued economic effect
lines are omitted; a zero claim is discharged only by its complete accepted
burn/book/duty relation. Deposit100 into1000/1000 mints100
shares; funded reward110 makes1210/1100; Unbond records110; Slash121 allocates11
to that claim and110 to free holders; Withdraw pays99/burns100, leaving990/1000.
Its cursor remains8 and its paid entry has active shares0/owed0/paid99. The
current book, not a fresh common ratio or inferred priority, determines payment.

### Receipt floors before/after: milestones and terminal branches

| Literal trace | Ambiguous old lifetime-net result | Required signed scoped result |
| --- | --- | --- |
| Loan origination credits Alice500; later repayment debits30 | Whole-life cash delta470 would fail untagged500 | Milestone Originate fresh net500 passes; later Alice cash delta470 is permitted; payment still must fund bound creditor30 and cumulative Alice gross30 remains consumed. |
| Coin deposits0.200 GOLD, mints100 MUSD, burns100 and redeems0.190 GOLD | Whole-life MUSD0 / GOLD−0.010 would fail untagged100/0.190 | Mint milestone100 passes; Terminal Redeemed stage receipt0.190 passes; terminal MUSD0 and whole-life GOLD delta−0.010 are not promised profits. G_GOLD0.200/G_MUSD100 and fees0 remain recorded. |
| Same coin path through shutdown/new emergency claim, with authenticated release0.190 | Generic terminal could ignore either floor | Only EmergencySettled receipt requirement applies,0.190 minimum; Mint milestone proof remains required; ordinary Redeemed gate is inapplicable, not overridden. |
| Delivered swap: new GOLD_F credit0.900 after all output charges | Could count a preexisting wallet balance | Terminal Delivered measures only new Swap/Pair delivery;0.900 passes,0.899 fails regardless of old balance. Source G_USD100.20/F_USD0.20 and destination G_WUSD/F_WUSD remain cumulative. |
| Qualified OriginRefund: new source USD100; no GOLD_F receipt | Untagged GOLD0.900 rejects legitimate refund, or gets silently waived | Only Terminal OriginRefund USD100 floor applies; Delivered not selected. Retained source fee0.20 and gross100.20 remain; whole-life source cash delta−0.20 is explicit. |
| Final Claim100 WUSD; atomic Swap rejection with no debit; DestinationReturn | Could substitute wrapped return for USD refund | Terminal DestinationReturn reserves new claim-bound100 WUSD for AliceF; no GOLD/USD receipt is asserted; source backing100 stays escrowed and fee0.20 stays spent. |
| Prior wallet USD1000, fresh scoped credit480; promised floor500 | Wallet1480 incorrectly satisfies floor | Fresh delivery480 fails; old1000 contributes zero. For an isolated legal-charge measurement probe, credit520 less charged20=500 passes; credit500 less20=480 fails. Actual LoanPromise fee cap0 would also reject any such20 fee. |
| Claim received only90 WUSD but AliceF already owns1000 WUSD | Old destination custody fills the deficit | ReservedCustody net90 fails100; unrelated old1000 never satisfies Pair's return. Unknown possible late swap debit also prevents a terminal custody proof. |

A matching receipt cannot be checked from an ending wallet balance alone. Its
accepted effect/claim provenance, complete delivery charges and exact signed gate
are required. These clarified scopes preserve every positive receipt floor and
every cumulative cap; they replace the inconsistent whole-life interpretation.

### Continuation, scope, authority and custody discriminators

| Literal fact / mutation | Signed check | Expected observation |
| --- | --- | --- |
| Escrow p0→p1; Claim f0→f1; Swap proposes f1→f2 | Swap's HeadRule names exact accepted Claim successor | Resolve f1 and consume it atomically; no substitution of f0. |
| Qualified OriginRefund after Escrow p0→p1 | OriginRefund HeadRule names Escrow successor | Resolve p1; require nonreceipt/exclusion and current recovery authority; no use of p0. |
| Unrelated foreign ledger change f1→fx before Swap | RequiresFreshAuthorization, current head must equal resolved f1 | History stale, Pending duty retained; no completion-selected fx. Fresh preserving source authorization must explicitly bind fx. |
| Forged predecessor from another agreement/pair, though its head text equals f1 | PreviousAcceptedHead binds agreement, Pair, StageId, effects and evidence edge | Stage/history binding rejection; matching text is insufficient. |
| Claim send Unknown then original f0→f1 mint finalized | Exact original economic fact and paired nullifier, not last response | Qualify/link that original fact once; Swap resolves f1; no second mint. If head is now unrelated fx, require fresh authorization before Swap. |
| Retry changes AttemptId but keeps Claim StageId/Pair | scope identity and replay/nullifier/history fixed; only send-attempt fact changes | Receipt must match that attempt; duplicate economic acceptance rejects; all bounds/history remain. |
| Two oracle queries share Read grant; queryA requests150, queryB151 | QueryId+selector digest+parent/grant/domain/attempt all bind receipt | Cross-query or cross-attempt receipt rejects; a grant alone is not KernelScope or VerifiedGrant. |
| Owner signs parent, ReserveForeign signature absent on Claim grant/mint/native payload | AuthorizationConjunction requires distinct exact signature types and Foreign epoch3 capability | Authority/signature qualification fails; no grant/capability signature can replace parent authorization. |
| Prior Stage/Intent/Effect pass; right keys but expired/revoked grant or wrong grant domain/key epoch | All signature, expiry, revocation, domain/epoch checks must hold | Reject Authority; proof or receipt success cannot override it. |
| Refund proof arrives at Preview301 or return proof at Foreign401 | Old grant expiry300/400 and source-signed renewal policy | With a still-valid parent and earlier judgments passing, Authority rejects the old grant. If the parent window also expired, Intent Validity fails first. Duties/custody remain; new preserving authority permits only remaining original outcomes, with no extension/reset inferred. |
| Alice free GOLD5; LoanPledge custody2; locks Loan1/Other0.5; buyer GOLD0; debt475; sale400 | DebitCustody selects LoanPledge and Loan lock; free cell framed unchanged | Free5 unchanged; custody1; Other lock0.5 retained; buyer GOLD1; creditor receives400; default debt/duty75 survives; one custody transfer only. |
| Replace DebitCustody with Account Debit(Alice,GOLD) or omit custody write | Typed effect/capability and exact authored/derived footprint equality | Resource/Effect mismatch, no published stage effects; cannot debit both free balance and pledged custody. |
| PureIntent.missing(0.899 GOLD,0.900 GOLD) | Total same-asset comparison/guarded subtraction/exhaustive ADT match, bounded_work12 | Diagnostic0.001 GOLD only; no floor mutation, effect, authentication or ledger success. |

The typed scope/authorization/head rules above remain proposals awaiting grammar,
elaboration, signature/certificate soundness and formal/native correspondence.
Integer arithmetic checks establish only these literal worked values.

### v4 focused dispositions of the v3 reviews

The immutable [audit04 v3](audits/04-horizon-v3.md) and
[audit05 v3](audits/05-horizon-v3.md) reviewed mockup hash
`16d419cb4e8f26571d9aab79406e5e8864e7db623028b98929274c727e5d601e`
and horizon hash
`f5c947822879f55d25a06efc48eec77eb82149c9cadb262b31e6274f683db488`.
Their positive scoped decisions remain attached to those inputs. The following
three repairs are proposals requiring fresh review; these literal discriminators
are not executable horizon tests or proofs.

| Finding | Concrete reproduction before repair | Focused proposed disposition |
| --- | --- | --- |
| 04 v3 M1: Reconcile head/scope | Reconcile used Economic identity and a ledger journal write, but Promise.heads supplied no Reconcile StageHead; its write could conflict with Swap's Claim successor. | §§2/3/9/11/13 declare signed ObservationalRule, Query identity, deferred checkpoint facts and bounded ServiceQueryJournal. Reconcile has no economic/control effects, ledger writes, nonce or head consumption. The original Claim edge alone may supply Swap's predecessor. |
| 05 v3 H1: first-failure order | scopeFor checked grant/expiry with requireOk before Core.prepare; an expired grant could report Authority while the same term already failed Intent cap or Effect overflow. | §§2/3/10/11/13 collect total ScopeRequest/DeferredFact, then use one Stage→Intent→Effect→Authority→History→Failure scheduler. Only scheduled qualification constructs KernelScope. Native signatures remain explicit output premises until exact-payload requalification before dispatch. |
| 05 v3 M1: Deposit admitted active claims | B1001/S100, pending10 shares owed100.10; deposit100 mints9, book would become B1101/S109 and claim owed101.00 while its old duty stayed100.10. | §12 restricts Deposit and Reward to NoActiveWithdrawals, authenticated through complete finite claim/duty registry cells and book coherence. Pending/mature/zero-owed unburned claims reject Effect ActiveWithdrawalUnsupported before any money/book change. Paid-only history is preserved. |

#### Reconciliation: observational checkpoint versus economic successor

Use the literal trace Escrow p0→p1 and Claim sent with proposed f0→f1 and
response Unknown. Both §11's BridgeReconcile1 and §13's
BridgeSwapReconcile1 use their own exact signed QueryId/selector, selected
foreign finality verifier, original Escrow/Claim selectors, at most8 covered
send attempts and2 service-work units. The query request qualifies only request
premises; the returned checkpoint/count/edge evidence is independently scheduled
as a response. Neither term publishes economic effects.

| Returned evidence / attempted mutation | Required outcome under the signed observational rule |
| --- | --- |
| Claim never accepted; count0 at checkpoint fx without final exclusion | Observation remains Unknown; escrow100, pending duty and all economic heads/counters survive. Neither zero nor timeout authorizes refund. |
| Claim never accepted; qualified finalized complete nonreceipt plus exclusion of every later delivery | Service journal records the scoped proof. A separately scheduled/currently authorized OriginRefund may use it, resolves Escrow successor p1, and must satisfy its own USD100 receipt. Observation itself refunds nothing. |
| Claim accepted f0→f1, but response lost; original accepted edge and exact mint qualified | Service journal records that original fact, with no mint or ledger advance. The original economic Claim may be linked once under its original-time authorization; Swap's rule still resolves f1. |
| Query finalized checkpoint is fx after unrelated economic progress f1→fx | The selected verifier may qualify fx as observation data, but Swap cannot substitute fx for f1. Stale economic continuation rejects; a fresh preserving economic authorization is required. |
| Caller changes QueryId, selector, verifier, Pair or original-fact identity | Exact signed bindings reject at the first applicable Stage/Intent/History predicate, before a service fact is qualified; no economic predecessor can be fabricated. |
| Third query attempt after2 authorized units, or retry reuses a consumed durable query attempt | Authority work or History deduplication rejects as scheduled. Prior service work remains spent; no new ledger head/nonce/effect and no counter reset. |

The observation journal records facts only. Its authenticated reservation spends
service quota and the parent's cumulative work exactly once per dispatched query
attempt, without touching ledger Work/Replay/Head cells. Query authority cannot
supply economic spend/custody authority. Renewal of expired observation authority
preserves query identity, original facts and consumed counters; a duty remains
pending if renewed authority is unavailable.

#### First failure: deferred expiry combined with an earlier defect

For an isolated proposed semantic discriminator, fix well-formed parent validity
[250,400], authenticated current round301, and grant expiry300. All omitted
predicates pass; the parent is valid so its expiry does not obscure the grant
case. UInt128 amounts use the declared asset atoms. Raw evidence that the grant
expired is collected, not replaced by a verified flag or an early exception.

| Literal defects in the same term | Scheduled first failure | Published new state |
| --- | --- | --- |
| Proposed gross debit2 atoms exceeds signed cap1; grant expired | Intent gross-cap failure | None: no effects, post, head, allowance/work consumption or new duties. |
| Gross debit1/cap1 is valid; recipient balance UInt128.max plus credit1; grant expired | Effect overflow | None; earlier accepted stages and duties remain. |
| Add invalid selected Stage binding to either row above | Stage binding failure | None; later cap/effect/expiry defects cannot replace it. |
| Stage, Intent and Effect valid; only grant expired | Authority ExpiredAuthorization | None; original reservation/duty remains pending. |
| Parent window also ends300; grant expired at301 | Intent Validity, provided Stage passes | None; expiry is not mislabeled solely as an Authority failure. |

The expected first judgments follow the declared order, not an executed evaluator.
Direct Core preparation, OneDomain, governance and economic recovery use the
same schedule. Query request/response terms use that same order over their own
signed zero-effect observation relation; they cannot borrow an economic stage's
success flag. Formation errors have no Core term. After native bytes/signatures
exist, Core.qualifyNative reschedules fresh facts before dispatch: a failed
parent, resource cosignature, expiry, revocation or history predicate cannot be
hidden by a successful signing-service response. Planning/signing receipts are
not final acceptance or ledger effects.

#### Deposit: arithmetic before/after and atomic rejection

All currency values below are USD with integer cents. In the old admitted case,
`minted = floor(10000*100/100100) = 9` shares. The pending claim's original
entitlement is `floor(100100*10/100) = 10010` cents (100.10). Updating only the book
would give `floor(110100*10/109) = 10100` cents (101.00), a90-cent discrepancy
with the unchanged claim/duty. A book rewrite is therefore unsupported here.

The exact financial reads are owner balance, owner shares, vault, tier book,
complete withdrawals registry (maximum16), complete current withdrawal-duty
registry (maximum16), all at the same authenticated head. Completeness and
book/registry coherence are Stage premises. The explicit NoActiveWithdrawals
Effect predicate runs before price/share/book recomputation; this is a financial
restriction. There is no maintainer gate or dynamic unbounded claim scan.

| Literal pre-state / probe | Proposed result and unchanged cells |
| --- | --- |
| B1001/S100, Pending10 owed100.10 with matching current duty; deposit100 | Effect ActiveWithdrawalUnsupported. B stays1001/S100; pending owed/duty100.10, owner balance/shares and book unchanged; no effects/control consumption. |
| Same claim MatureUncommitted, or owed0 but shares10 unburned | Same atomic rejection; maturity and zero liability do not discharge the claim. |
| No active claims/duties/book entries; B1001/S100 all free; deposit100 | Supported relation mints9, B1101/S109; existing free100 shares owed1010.09, new free9 owed90.90, reserve dust0.01; sum1101. No withdrawal liability is updated. |
| Same supported input with only historical Paid55 record, active shares0/owed0 and no current duty | Same9 minted and B1101/S109; paid55 record and its wallet payment unchanged. It creates no renewed claim or duty. |
| Empty supplied claim list but missing completeness proof, or omitted current-duty registry | Stage WithdrawalRegistryProof; no financial/control effects. An unproved empty list is not NoActiveWithdrawals. |
| More than16 claims/duties needed, or partial list offered to fit16 | Formation resource-bound error for oversized data, or Stage completeness failure for partial data; no accepted truncated footprint or hidden scan. |

Reward uses the same finite registry guard. Supporting active claims in Deposit
or Reward requires a future complete bounded claim/duty update relation; no
positive floor, cumulative bound or existing duty is loosened by this restriction.

## Requirements checklist

Checked boxes mean **visible proposed source coverage only**. They do not mean
financial consistency accepted by reviewers, parser/typechecker acceptance,
certification, authenticated execution or any discharged proof obligation.
The v2 and v3 findings have proposed dispositions and discriminators in §16; fresh review of these bytes is pending.

- [x] One explicitly versioned proposed horizon, separate from specified beta registry and local Source/6/Core/5; no new implementation/proof/ledger claim (§1; PROGRAMMER-MOCKUP).
- [x] Programmer syntax tour covers declarations/types/expressions/agreement/actions/stages/outcomes/errors; reusable profile/ADT/match notation (§1–4).
- [x] Typed identities, quantities/deltas/positions/shares/prices/clocks/rounding/conversion and arithmetic boundary (§1/4/8).
- [x] Typed signed limits, finite holes/completion, validity/nonce/policy, roles/grants/quorum/expiry/revocation/refinement (§2/5/10/13).
- [x] Proposed financial transitions display typed pre/post cells, declared/derived footprint requirements, economic effects, authority, evidence, ordered failures and outcome (§5–13); mandatory common control footprint stated (§1).
- [x] Kernel's eight methods have typed authority/evidence inputs; per-stage bindings and common call pipeline are explicit (§3/5–13).
- [x] All eight family lifecycles include omitted roll-forward, premium/fix/settlement, shutdown/emergency payment, reward/slash/unbond/withdraw, veto and qualified bridge recovery (§5–12).
- [x] Proposed composed episode displays a signed parent promise and continuation-head schema, asset-specific cumulative gross/fees and branch/milestone receipt scopes, program/policy/recipient, claims/attempts, all terminal alternatives and residual duties (§13).
- [x] Side-by-side independent transfer and repayment trace includes source/scenario/Core/kernel facts/effects/post/commit-or-reject; every Source/6 field has an origin mapping (PROGRAMMER-MOCKUP).
- [x] Construct reference and eight-family/surface check matrices distinguish local/specification/open status and link repository evidence (§14).
- [x] Explicit choices, smallest next executable profile and K/Quint/elaboration/authentication/atomic-consumption obligations preserved (§15).
- [ ] Independent concurrence on these repaired exact bytes; original v2 reviews remain dissent on their frozen input and are not reused as approval.
- [ ] Horizon grammar/type checking/execution, financial certificates, authenticated kernel/ledger continuation and cross-layer/native proofs: **open**, beyond this design artifact.


### Recovery qualification and financial execution (v5 clarification)

`Core.qualifyRecovery(stage,candidate,planning_scope,branch,recovery_evidence,
fresh_evidence)` is a bounded scheduled entry point. It rechecks the complete
Stage→Intent→Effect→Authority→History→Failure relation, including the signed
branch's recipients/assets/net terms, derived complete effects/post, current
head/replay/attempt, expiry/revocation and profile-specific qualified evidence.
For OriginRefund that evidence includes finalized complete nonreceipt and
exclusion of future delivery; DestinationReturn instead requires the original
qualified receipt, destination custody and qualified swap rejection. No branch
may substitute the other's asset, evidence or outcome. On success it yields
`RecoveryPrepared { candidate: CorePrepared<D,P>, scope: KernelScope<D,P>,
branch: RecoveryBranch<P>, evidence: RecoveryEvidence<D,P> }` whose opaque
qualification kind is Recovery. On failure it publishes the first scheduled
rejection with null new effects/post/head/duties. Caller flags and Planning
scopes cannot construct this value.

**Recovery qualifies a proof service, not a refund dispatch.** Kernel.recover
checks that Recovery scope and exact branch/evidence identity, then returns
Qualified bound evidence or Unresolved. It performs no ledger/control/economic
writes, consumes no economic head or replay key, and cannot release custody,
reservations or duties. Qualified is not an accepted terminal financial outcome.
Any bounded service record remains separate from economic accepted history.
Native payload correspondence/signatures/finality/atomic commit remain explicit
pending premises even when the recovery proof service is qualified.

The `kernel recover` sugar expands to this finite sequence:

```mori
protocol RecoverOneDomain<D,P>(stage: Stage<D,P>, request: ScopeRequest<D,P>,
  branch: RecoveryBranch<P>, e: EvidenceBundle<D,P>, r: RecoveryEvidence<D,P>) {
  let initial = Core.prepare(stage: stage, completion: stage.completion,
    scope_request: request, evidence: e).readyOrPublishFirstRejection();
  let recovery = Core.qualifyRecovery(stage: stage, candidate: initial.candidate,
    planning_scope: initial.scope, branch: branch, recovery_evidence: r,
    fresh_evidence: e).readyOrPublishFirstRejection();
  let fact = Kernel.recover(scope: recovery.scope, branch: recovery.branch,
    evidence: recovery.evidence);
  match fact.fact {
    Qualified(boundEvidence) => OneDomain(stage: stage, request: request,
      e: e.withRecoveryEvidence(boundEvidence)),
    Unresolved => Pending(claim: economicClaim(request.identity),
      duties: stage.signed_pending, reservation: existingReservation(request.identity))
  }
}
```

`withRecoveryEvidence` binds the verified fact to the existing signed parent,
branch, stage, candidate and attempt; it cannot change any signed term or return
trusted success flags. OneDomain starts the shared schedule again with fresh
facts: proof-service success is never reused as a stale financial authorization.
It obtains Planning for validation/reservation/buildCall, Signing for authorizeSign,
NativeComplete for dispatch, then actual ledger evidence for Core.accept. A stale
head, expired grant or missing native signature rejects at its scheduled position;
no recovery service receipt overrides it. Only accepted complete financial effects
release the appropriate signed duties/reservation. Unresolved retains them.

Discriminators: passing Planning directly to Kernel.recover is a kind error;
qualified nonreceipt plus an expired recovery grant fails Authority; expired grant
plus a bad signed cap fails Intent first; Recovery plus no payload signature can
return a proof fact but never dispatch; proof-service Qualified followed by a head
change fails the fresh OneDomain History check and publishes no financial effects.
All of this remains specified-only, with qualification/proof/adapter implementation
and financial/native correspondence open.

