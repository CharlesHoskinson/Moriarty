# Moriarty unified semantics — design proposal

Date: 2026-09-06. Status: S2, proposed for user review; not a selected architecture,
implemented language or proved theorem. This is the concrete design output of
the approved [design sprint](../plans/2026-09-06-actus-defi-pcd-replanning.md).
Sources and coverage: [target study](../../research/2026-09-06-actus-defi-design-study.md)
and [PCD research](../../research/2026-09-06-pcd-bounded-dsl.md).
All definitions below are proposals unless explicitly called source observations.

## Recommended architecture

Use a **typed agreement language that elaborates into a small, bounded
transition language**. Financial packages define ACTUS and DeFi behavior using
shared arithmetic, time, state and effect facilities. An intent selects and
authorizes bounded work on those agreements. Every accepted state change
requires a proof of that work and its compliant predecessor history.

This makes agreements and transactions parts of one semantic model. The
agreement defines permitted behavior; an intent authorizes a particular use;
the proof binds the resulting state and effects. A solver can search for an
acceptable plan outside the language, but cannot enlarge the authorized relation.

Moriarty source and Midnight's Compact are different languages. Compact/ZKIR is
the initial compilation target to investigate. This proposal does not assume
that Compact exposes a suitable recursive verifier, or select a proof system.

## Three alternatives tested against the same requirements

| Alternative | Loan representation | AMM representation | Assessment |
| --- | --- | --- | --- |
| A. Transition language as the developer language | Explicit schedule cursor, financial state and PR/IP handlers | Explicit reserve transition and effect list | Uniform and directly related to a proof relation; exposes too much bookkeeping to every agreement author. Useful as the intermediate Core. |
| B. Finite contract combinators | Expand or fold scheduled `When/Pay/If` continuations with shared accrual expressions | A finite interaction tree containing reserve updates and swap calculations | Familiar Marlowe-style structure, with a natural reduction measure. Stateful AMMs, partial fills and event composition require considerable additional machinery; finite expansion can grow with horizon and choices. |
| C. Typed agreements over bounded transitions — recommended | A typed LAM package supplies calendar/event/state rules; settlement handlers operate on resulting dues | A typed pool package supplies reserve/share/fee rules; swap intents authorize bounded transitions | Shared semantics with useful domain authoring. Adds elaboration and package-correctness obligations, which must remain explicit and independently checkable. |

Schematic expressions of the same two examples, not parser syntax:

```text
A: step(loanState, nextEvent, observations) -> newState + Due
   step(poolState, SwapExactIn(dx, minOut), authorization) -> newState + Transfers

B: boundedSchedule(terms, horizon).fold(When(event, accrueAndPay, timeout))
   boundedChoices(maxSwaps, When(swapInput, reserveAndPay, closePool))

C: agreement Loan = Actus.LAM(terms, numericProfile, bounds)
   agreement Pool = Exchange.ConstantProduct(assets, feePolicy, bounds)
   intent Execute(agreement, action, constraints, expiry)
```

This is an expressivity/design comparison, not an executed size or equivalence
benchmark. The original assignment's agreement-Core option maps to B; its
intent-Core option informs A's authorization envelope; its two-calculus option
informs C's explicit elaboration relation. A Compact-library-only route remains
a backend baseline: it would need the same formal relation, package rules and
developer contract, so existing Compact syntax alone does not satisfy this task.

## Why the targets require these facilities

| Facility | Smallest motivating behavior | Proposed home |
| --- | --- | --- |
| Bounded typed numbers and explicit rounded division | AMM output and lending share conversions differ by rounding direction | Core integer operations; typed numeric libraries |
| Ordered time and event data | ACTUS `lam01` accrues on old principal before PR reduces it; IP then discharges accrued interest | Core finite data; calendar, schedule and event-order libraries |
| Bounded first-order iteration | Scalar/array schedules; exact integer square root; Newton calculations with an explicit stop flag | Core bounded fold; algorithm and calendar libraries |
| Persistent typed state | Outstanding notional, AMM reserves, partial fills, exercise state | Core transition input/output |
| Obligations distinct from transfers | A contractual payoff can become due without having been paid | Agreement/settlement libraries over typed Core effects |
| Authority and authenticated observations | Mandates, rate resets, guarantee triggers, outcome resolution | Core checked inputs/effects plus explicit capability profiles |
| Atomic composition with identity | ACTUS child swap legs; a DeFi action consuming independently held assets | Core transaction composition and proof statements |
| Bounded lifecycle accounting | Open-ended accounts, rolling derivatives and repeated swaps | Core resource rules with package-specific closure semantics |

No `LAM`, `AMM`, `Option` or family tag is a Core constructor. An operation only
enters the Core when its semantics cannot be expressed with existing finite
data and bounded operations without losing the required behavior. The matrix
records a design obligation, not proof that these facilities already cover every
target.

## Core types and evaluation

Proposed value forms:

```text
T ::= Bool | UInt<w> | SInt<w> | Bytes<n> | Enum<finite cases>
    | Record<finite fields> | Sum<finite alternatives>
    | Vector<T,n> | BoundedMap<Key,T,n>
```

Widths and capacities are positive compile-time literals after package
specialization. Keys have a canonical total order; duplicate map keys are
rejected. Length-delimited encodings distinguish empty values, padding and absent
fields. Text, addresses and asset identifiers have bounded canonical encodings.
An unbounded host string or integer cannot enter through an opaque escape hatch.

Expressions contain typed literals, variables, record/sum operations, conditionals,
checked integer arithmetic, comparisons and statically bounded folds. Functions
are first-order with an acyclic call graph. There is no general recursion,
unbounded loop, dynamic code loading or unconstrained host callback in evaluation.

Arithmetic errors are defined outcomes: overflow, division by zero, invalid
conversion and exhausted capacity reject the transition. There is no silent
wraparound, truncation or partial state commit. Intermediate widths are checked
separately from result widths. A finite `fold<N>` may carry a `done` flag and
thereafter perform the identity step; failure to meet a required convergence
condition by N is a rejection, not an approximate success.

Typed library wrappers distinguish `Amount<asset,scale>`, `Rate<scale>`,
`Price<base,quote,scale>`, `Shares<pool>` and calendar times. Addition requires
compatible units. Multiplication/division must specify intermediate precision,
result scale and rounding. In particular, unsigned floor division from DeFi
must not silently become signed truncation for negative ACTUS cash flows.

**Numeric decision still requiring evidence:** there is no universal decimal
scale or rounding mode selected for all ACTUS fields. Import decimal strings
and JSON numeric lexemes losslessly. Each package's numeric profile must define
every rounding point; all-field fixture comparison must validate it. Exact
bounded rational intermediates and explicit quantization are candidates, not a
claim that the published finite decimals equal the mathematical formula exactly.
An unproved field tolerance cannot hide this obligation.

## Agreement state, events and settlement

A specialized agreement is the finite tuple:

```text
Agreement = (stateType, actionType, Init, Step, propertyManifest, bounds)
Step : State × Action × Observations × AuthorityEvidence
       -> Reject(reason) | Accept(State', Effects, ResourceUse)
```

For fixed inputs the result is deterministic. Public ledger state, private state
commitments and disclosed outputs are explicitly identified; privacy is not
inferred from the presence of a proof. Asset movement is expressed only through
the returned effect envelope.

Scheduled ACTUS events and user actions use the same transition machinery.
The package defines schedule generation, calculation/payment dates, same-time
event priority, child identity and final tie-break rules. The author cannot
reorder mandatory events by supplying a convenient list. A transition checks
that it processes the next eligible event or a permitted bounded prefix, and
cannot skip a due mandatory event to execute a later dependent action. A supplied
schedule or proof of its construction must be checked against the bound terms;
an unchecked off-chain schedule is not authoritative.

Observed events carry stable IDs, source/subject, occurrence time, observation
time and bounded payloads. The package specifies which time orders them and how
they interact with scheduled events. Missing required observations, duplicate
events or ambiguous ordering reject. Calendar conventions and time-zone rules
are versioned package semantics, rather than host defaults.

**Cash flow is not settlement.** `AccrueDue` records an obligation with debtor,
creditor, denomination, amount, event identity and remaining amount. `Transfer` records
an authorized actual asset movement. An ACTUS payoff can create a due and update
the reference financial state; it must not claim that money moved. Settlement
consumes or reduces that due against matching transfer evidence. Contractual
notional and unpaid dues remain separate so reducing scheduled notional cannot
erase an unpaid receivable. Default, arrears interest and recoveries require
explicit package rules; this proposal does not invent them from an ACTUS trace.

A currency denomination such as ACTUS `USD` is not automatically a ledger token
identity. A settlement policy binds the actual asset, denomination conversion,
observation assumptions, token quantum, rounding and treatment of residual dust.
The proof checks that policy when discharging a due. It cannot assume that any
token called USD has the same issuer, backing or value. The initial mock displays
the settlement mapping as unconfigured until those inputs are supplied.

Other effects are bounded mint/burn, authority update and disclosure records,
each requiring its declared policy. All fees and change are included in the
authorized effect envelope. If the target computes a ledger fee later, the
intent and final acceptance must check its asset, payer and cap; the integration
must prove that this permitted variability does not admit extra application
effects.

## Composition and finite lifecycle

A transaction names bounded sets of writable predecessor states and read-only
observations. Each writable input is consumed once; its identity and revision
are bound to the output. Two writes to the same input conflict and reject.
Read-only references have a separate freshness policy and cannot masquerade as
consumable assets. All child transitions and effects either commit atomically
under the target's ledger model or reject together. Cross-chain operations do
not inherit that atomicity; they require explicit staged settlement and finality
capabilities outside the local transaction.

Every deployed instance has a finite lifecycle budget and finite state capacity.
Each accepted write consumes a step allowance; event batches also consume their
actual event allowances. Revisions cannot wrap. Splitting an instance allocates
remaining allowances to children without increasing their sum; joining consumes
each input once and cannot restore spent allowance. The design proof must define
a global decreasing measure over a bounded composition, including its pending
events and children.

An open-ended pool or account is deployed in an explicit bounded epoch. At its
boundary it admits only the declared closure/settlement behavior. Renewal is a
new authorized contract with a separately checked migration relation, not an
implicit reset that extends the old lifetime guarantee. Processing arbitrarily
many separately created contracts does not give any one contract an unbounded
execution guarantee.

Closure needs reserved capacity for refund/settlement actions. A user cannot
spend the last allowance on an ordinary action and thereby bypass promised
closure. The compiler must check the package's bound on closure work. Whether
closure eventually happens still depends on any stated clock, funding and
participant assumptions; termination of evaluation is not participant liveness.

## Bound profile and the precise finite claim

Each instantiated program supplies a closed profile B:

| Bound | Required check |
| --- | --- |
| Numeric widths/scales and intermediate widths | Every operator has defined checked arithmetic and unit conversion. |
| Program size, call depth and fold iterations | Expanded code/circuit work has a static upper bound. |
| State fields, map/vector sizes, byte lengths | Every represented state and input domain is finite. |
| Calendar horizon, generated/observed events and children | Schedule generation and composition have finite output capacity. |
| Per-transaction actions, observations, writable inputs/outputs, proof fan-in | A transaction has finite semantic and verification work. |
| Lifecycle steps/events, revisions and closure reserve | No continuation, split, join or renewal silently restores the certified budget. |
| Cryptographic transcript depth and verifier resources | Supported by the selected construction and deployment profile, not inferred from a fold. |

Proposed language theorem: for a well-typed closed program P and any admissible
encoded input under B, evaluation terminates within a computable bound and
returns one defined result. Proposed contract theorem: a named invariant holds
initially and is preserved by every accepted step under the manifest's
assumptions, including composition and closure. Neither theorem is proved by
this document. Finite state does not promise tractable exhaustive enumeration.

## Properties to prove for the first packages

These are proposed predicates, not passing results. Quantification ranges over
all well-typed admissible inputs under the declared profile, not just examples.

| Property | Precise obligation |
| --- | --- |
| Asset accounting | For each concrete asset and account, the new balance equals the old balance plus the complete authorized incoming/minted amounts minus outgoing/burned/fee amounts. Aggregate preservation permits only declared mint/burn authorities. |
| Due accounting | For each oriented due, issued amount equals settled amount plus outstanding amount plus explicitly authorized write-off amount. A scheduled notional update alone cannot increase settled amount or erase a due. |
| Event identity/order | Every applied event has a unique package/instance/child-qualified identity and the next eligible ordering key; rejected or replayed events do not advance the state. |
| LAM accrual | The package's declared year-fraction, rate and interest-base rule computes period accrual before the principal update when required by the event rule; settlement uses the resulting recorded due. Numeric rounding is part of this predicate. |
| AMM exact input | For the supported constant-product swap, positive bounded reserves/input and nonzero denominator imply the specified floor-division output and new reserves; output satisfies the signed minimum. LP behavior requires its own additional predicates. |
| Authority and effects | Every committed application effect is permitted by the checked program and actual signed intent under the current authority policy; there are no additional recipients, assets, approvals or disclosures. |
| Finite lifecycle and closure | Each ordinary transition decreases its allocated measure while preserving sufficient capacity for the declared closure path; splits/joins preserve the global resource accounting rule. Reachable closure is conditional on the stated funding/clock/actor assumptions. |

Source-to-Core elaboration, Core-to-proof-relation encoding and
Core-to-Compact/ZKIR effect correspondence are separate obligations for these
same predicates. The target study's source discrepancies must be resolved before
a package certificate can claim the corresponding ACTUS behavior.

## Contract certificates and transaction PCD

The language has two connected assurance objects:

1. **Contract certificate:** binds the exact program, package lock, bounds,
   assumptions and required properties to a checkable proof of initialization
   and preservation, or another explicitly justified finite verification method.
2. **Transaction proof:** establishes the authorized transition and its
   compliant predecessor history for that certified program/domain.

Required properties with `unknown`, `unavailable` or a counterexample prevent
certified deployment. Optional properties remain visibly optional. A property
manifest is a statement of obligations, not evidence that they hold. A trusted
attestation must be labeled as such and cannot silently replace a checked proof.

The exact way a property proof checker is connected to the target is unresolved.
The design requires either an admissible checker/verifier at the acceptance
boundary or an explicitly governed registration mechanism with its trust
assumptions disclosed. Merely placing the hash of an unchecked certificate in
genesis is insufficient. No such checker is implemented here.

The proposed public transaction statement binds:

```text
domain = network, deployment, semanticsVersion, verifierProfile
program = normalizedCoreHash, packageLockHash, boundsHash,
          propertyManifestHash, validatedCertificateReference
composition = compositionProgramHash, compositionCertificateReference,
              bounded instance-to-program/policy/bounds/certificate map
inputs = ordered writable predecessor commitments and identities,
         read-only observation commitments, intentDigest, authorizationPolicy
outputs = resulting state commitments, effectEnvelopeCommitment,
          consumption identifiers, resource accounting
```

A local compliance predicate checks admissible genesis or valid input messages,
program/domain consistency, typed inputs, observation policy, authorization,
the financial transition, exact outputs/effects and decreasing resource budget.
PCD prover/verifier machinery links incoming messages to their proofs. The
predicate's semantics and the recursive proof construction are separate
definitions with a required correspondence argument.

For a single agreement, `program` identifies its specialized transition program.
For a cross-agreement transaction, it identifies a certified composition program;
the bounded composition map binds every input and output instance to its own
program, numeric policy, budget and checked contract certificate. The composition
program has a finite declared set of admissible child program identities and
checks their transitions plus the cross-agreement asset/authority invariants.
Individual child certificates do not establish those joint invariants. Ordinary
continuations preserve their instance's program identity; migration to a new
identity requires a separately authorized and certified migration rule. A join
cannot silently reinterpret one program's state as another's.

Genesis cannot choose arbitrary state merely because there are no predecessors.
Splits bind each output position to one checked output vector; joins authenticate
each predecessor and prohibit duplicate input identities. The final proof
acceptance algorithm must verify the completed claim, including any accumulator
decider or compression step required by its construction.

Application acceptance also checks live ledger consumption, network/deployment
identity and the applicable finality policy. PCD can establish a compliant
history while two conflicting branches remain individually valid. Global
exclusive spending comes from the anchored consumption rule, not from claiming
that recursive proofs make forks impossible. Oracle signatures similarly
establish the declared attestation, not the economic truth of its contents.

## Proof topology and backend decision

| Route | Fit | Unresolved gate |
| --- | --- | --- |
| Sequential IVC over a serialized state commitment | One advancing state machine; simpler initial topology | Multiple independently created predecessors need an explicit authenticated import/join relation and consumption accounting. |
| Bounded multi-input PCD | Naturally represents transactions joining and splitting state/asset claims | Concrete construction, extraction/soundness assumptions, fan-in/depth and final verifier cost. |
| Midnight-native proof composition | Closest to the actual deployment stack | Which verifier and public-input interfaces are accessible from the pinned Compact/ZKIR/ledger path. |

Recommendation: specify bounded multi-input transaction semantics now, and test
the smallest target-consumable realization before choosing an implementation
route. A serialized realization is acceptable only if it preserves those
semantics. A hash chain or mocked certificate is not an alternative PCD backend.

## Two concrete walkthroughs

**ACTUS LAM source observation.** The pinned `actus-tests-lam.json/lam01` begins
with principal 5,000 and nominal rate 0.08. On 2013-02-01, PR returns principal
cash flow 500, remaining notional 4,500 and accrued interest
33.972602739726; IP then returns that interest and resets accrued interest.
The exact simple-interest expression for the first period is
`5000 × 8 × 31 / (100 × 365)`, which is not exactly that finite decimal.

Proposed workflow: import terms losslessly, show both events in order, generate
separate principal/interest dues, then authorize settlement. Reordering to
calculate that period on the reduced principal is a semantic defect. Displaying
the payoff as already paid without settlement evidence is a separate defect.
The numeric profile and comparator must resolve the decimal representation
before declaring fixture compatibility.

**AMM illustrative arithmetic, not an executed protocol test.** For reserves
`x=1,000,000`, `y=2,000,000`, input `dx=10,000` and a 997/1000 fee multiplier,
the proposed exact-input package computes
`out = floor(dx × 997 × y / (x × 1000 + dx × 997)) = 19,743`.
The checked new reserves are 1,010,000 and 1,980,257. A signed `minOut=19,700`
permits that result; `minOut=19,744` rejects. Ceiling division would pay one
extra unit and must fail the package relation. This example exercises swap
semantics only; LP mint/burn and protocol fees remain separate required package
operations identified by the target matrix.

Both workflows use the same finite inputs, state transition, authority/effects
and PCD interface. Their financial formulas remain versioned package semantics.
Neither example is presented as a working language implementation.

## Review decision and next executable work

Review the architecture, dues/settlement distinction, explicit finite epochs,
contract-certificate requirement and multi-input proof semantics together with
the [developer interface proposal](2026-09-06-moriarty-developer-interface-design.md).
Numeric profiles, ACTUS source gaps and the real target verifier remain named
obligations, not hidden defaults. Adoption of this design does not close them.

The first implementation proposal should cover one loan event/settlement pair
and one AMM swap through the same interpreter and typed effect representation,
then a minimal real proof consumed at the actual target boundary. Specify its
commands, inputs, resource ceilings and rejection controls before execution.
No old A4/A5 campaign is a prerequisite merely because work was invested in it.
