# Moriarty bounded atomic profile: static and dynamic semantics

Status: S2 corrected candidate, specified-only and not frozen or implemented.
Profile `moriarty-bounded-atomic/1`; Core `moriarty-core/1`. The closed wire
records and hash preimages in `typed-schemas.md`, simultaneous limits in
`bounds.json`, and numeric rules in `numeric-profile.json` are normative with
this document. A conflict is an error in the candidate, not a precedence choice.

## Domains, source, and judgments

Let `U = {0..2^128-1}`. A declared nominal unit has a canonical unit vector with
one exponent `1`; UInt128 has the empty vector. A vector has at most eight
nonzero components, each in `[-16,16]`. Stored types are UInt128, Text, and
Amount<X>. Bool and Quantity<q> are expression/local types only. In particular,
a `let` may contain Bool or a nontrivial product/quotient Quantity, and a local
is not a stored result. Bool and Quantity cannot be constants, arguments,
observations, state, `set` results, effect operands, obligations, or receipts.

```text
G |- e : T ! q                  expression and canonical unit vector
G ; S ; L |- statement => S'   typed straight-line statement
P |- source => SourceAST        complete parse, without recovery execution
SourceAST |- check => TypedProgram
TypedProgram |- lower => BoundProgram
M ; genesis ; state ; request |- step => Rejected | Complete
```

There are no user functions, recursion, loops, dynamic evaluation, collections,
or implicit iteration. Recursive-looking expression records are finite bounded
trees, not executable recursion. Value and type names resolve declaration-before-use; a bare
identifier resolves only to an earlier local. Duplicate names reject within each
namespace. Policy target actions and rounding locals are the only forward
metadata references: resolve them over the complete action table in stage 6,
then require all target/value-local constraints before evaluation. Each action has exactly one `actor: Text` parameter. A word that
exactly equals a grammar keyword is a keyword, never an identifier; only `asset`
and `amount` are contextual effect-field labels. `not` binds tighter than a
comparison, so `not a == b` parses `(not a) == b` and normally fails typing.

Source spans are zero-based UTF-8 byte intervals `[start,end)`. Lowering retains
the source hash, version, node IDs, types, unit vectors, resolution targets, and
source references on Core statements and expressions. Generated checks name
their origin spans and generated tag. No compiler may discard or fabricate this
binding. The exact deterministic IDs and schemas are in `typed-schemas.md`.

## Arithmetic and evaluation order

`uint(n)`, `text(s)`, `amount(n,X)`, `true`, and `false` have their explicit
types. Numeric-looking Text is never coerced. `remaining` is UInt128 and reads
the allowance before consumption by the current action.

`+` and `-` require identical numeric types and vectors and retain that vector.
`*` adds exponents. `floor_div(n,d)` subtracts denominator exponents and is the
only division syntax. It evaluates numerator then denominator, rejects zero,
and returns mathematical floor because all values are unsigned. Classification
by the result vector produces UInt128, Amount<X>, or local-only Quantity<q>.

Every literal, operand, and intermediate result must be in U. Multiplication and
addition reject overflow; subtraction rejects underflow. The implementation
must detect overflow before a later division, so `(MAX * 2)` rejects even if a
later quotient would fit. No reassociation, cancellation, widening accepted
semantics, floating point, hidden division, or implicit rounding is permitted.
Host big integers may only detect rejection. `and` and `or` short-circuit left
to right. Comparisons do not chain. Ordering requires equal numeric types and
vectors. Equality requires the same type and, for numerics, vector.

Statements execute against a private working copy in source order. Guards false
reject. A let defines once. A set writes its declared state field and later reads
see the new value. Each action may set a field at most once. Emit captures values
at its position. Rejection exposes no working writes, effects, obligations, or
authority consumption.

## Static financial policy binding

A policy target has one unambiguous identity:

```text
write(actionName,stateFieldName)
effect(actionName,zeroBasedEmitOrdinal,effectFieldLabel)
```

Every Amount-valued `set` occurrence and every Amount-valued field occurrence
in an `emit` has exactly one target in exactly one policy. No other occurrence
may be targeted. A target must resolve to the named action, statement, declared
field, and exact unit. Duplicate, missing, unknown, wrong-kind, wrong-unit, or
out-of-range targets reject. Coverage is occurrence-based: copying, adding,
zeroing, or emitting an Amount still requires an explicit target. Policies do
not attach by similarly spelled names, and there is no default or heuristic
propagation between targets.

During an action, each computed numeric value also carries an internal finite
set of FloorDiv node IDs. Literals, constants, action arguments, observations,
`remaining`, and values read from the committed pre-state start with the empty
set. Arithmetic unions operand sets, and a FloorDiv additionally inserts its own
node ID. Lets retain the set; sets place it in the private working value for the
rest of this action; reads and emits copy it. Provenance is checking metadata and
is not persisted across actions.

`rounding none` requires the target value's set to be empty. `rounding
floor(a,l)` resolves statically to the unique let `l` in action `a`, whose root
expression must be FloorDiv, and dynamically requires the target set to be the
singleton containing that Core node. Multiple floor nodes at one target are
unsupported and reject. This makes rounding propagation through a local, a
same-action state write/read, or further exact arithmetic deterministic. A
missing, ambiguous, future, non-FloorDiv, unreachable, or extra rounding node
rejects without name guessing.

`derivation`, `remainder`, `comparison`, and `proof` are documentary strings.
Their exact bytes and the resolved target/node records are program-hash bound.
The strings are not parsed, compared to source, or accepted as proof predicates;
there is therefore no undefined “stale text” check. The proof string names a
mandatory future claim only. Policy validation establishes binding and declared
rounding provenance, not financial correctness, ACTUS conformance, or a proof.

Both supplied examples explicitly cover every Amount set/effect occurrence.
The swap's `output_calculated` provenance propagates through reserve/trader
writes and the output transfer; its input and closure paths have no FloorDiv.
The loan's accrued interest provenance propagates through `interest_due` and its
DueCreated effect, while the later settlement begins with empty provenance from
committed state and uses `rounding none`.

## Effect schemas and settlement

The fixed source schemas for Transfer, Fee, DueCreated, and DueSettled are exact
in `typed-schemas.md`; arbitrary effect records are not permitted. Emit fields
must occur exactly once in declaration order with exact types. Transfer and Fee
propose asset movements. Due effects update the semantic obligation ledger.
Neither proves actual movement, balance, custody, or ledger acceptance.

A settlement declaration creates a one-to-one nominal-unit/ledger-asset binding.
Its `asset` Text must be nonempty and its quantum must be a positive Amount of
the declared unit. Across a manifest, binding names, units, and asset text values
are each unique. Thus runtime selection is exact, never first-match: an emitted
asset and Amount unit must resolve to the same unique binding or evaluation
rejects `SETTLEMENT_BINDING_MISSING`; multiple candidates reject statically as
`SETTLEMENT_BINDING_AMBIGUOUS`.

Quantum orientation is exact: `quantum.value` nominal subunits equal one ledger
base unit. For nominal `value`, require `value mod quantum.value = 0`, then
`ledgerAmount = floor_div(value,quantum.value)`. Check all operands, quotient,
and the reverse multiplication `ledgerAmount * quantum.value == value` in
UInt128. A remainder rejects `SETTLEMENT_NON_DIVISIBLE`; zero quantum rejects
statically; checked-conversion failure rejects `SETTLEMENT_OVERFLOW`. No rounding
or dust disposal occurs. For quantum 10, nominal 20 converts to ledger 2; nominal
15 rejects rather than converting to 1 or 2.

Every Transfer and Fee requires that exact resolution. DueCreated requires its
denomination Text equal its Amount unit identifier exactly. DueSettled requires
that equality and its asset/unit settlement resolution. The Complete effect
record includes the closed `SettlementResolution`, amount unit, quantum, and
ledger amount; there is no unhashed side map or implicit unit erasure.

For each `(asset,from,to,unit)` group containing DueSettled effects, the checked
sum of their nominal amounts must equal the checked sum of Transfer amounts for
the same asset with `from=debtor` and `to=creditor`, grouped by those parties.
The corresponding checked ledger sums must also be equal. This permits the loan
to combine PR and IP into one Transfer but prevents an obligation record from
asserting settlement without matching movement intent. Unrelated swap Transfers
need no DueSettled record. Fees are not settlement credit and remain included in
gross-debit and net-goal authority checks.

Asset authentication and actual ledger quantization remain external predicates.
A wrapper must verify that binding text denotes the intended ledger asset and
that the finalized movement equals the resolution. This profile only specifies
the value and interface that must be checked.

## Obligation transition and status derivation

The state retains at most 128 `ObligationRecord` values, including settled
identity tombstones. Identity is `(instanceId,dueId)` for the entire nonresetting
genesis lifetime. Effects are processed in source emit order against a private
obligation copy after their typed values and settlements are resolved.

For DueCreated:

1. amount must be positive; denomination must equal the Amount unit identifier;
2. neither Outstanding nor Settled record may already have the due ID;
3. capacity for one retained record must remain; and
4. insert the exact debtor, creditor, denomination, unit, amount, and Outstanding.

Failures are respectively `OBLIGATION_ZERO`, `OBLIGATION_DUPLICATE`, and
`OBLIGATION_CAPACITY`. For DueSettled, exactly one existing due ID must exist.
Debtor, creditor, denomination, and unit must all equal it. Zero rejects. An
amount below the outstanding amount rejects `OBLIGATION_PARTIAL_UNSUPPORTED`;
an amount above rejects `OBLIGATION_EXCESS`; equality changes status to Settled
without deleting the record. Unknown IDs reject `OBLIGATION_UNKNOWN`; an already
Settled record rejects `OBLIGATION_ALREADY_SETTLED`; identity-field mismatch
rejects `OBLIGATION_MISMATCH`. This first atomic profile neither partially settles
nor recreates an ID. A later version may add a remaining-amount field and partial
rules, but cannot reinterpret this profile.

There is exactly one episode status rule and one agreement status rule. Before
evaluation, the supplied status fields must equal deterministic derivation from
the supplied values/obligations or reject `STATUS_MISMATCH`. After all provisional
writes and obligation effects, derive again:

- EpisodeStatus is Closed iff the declared state field equals the declared
  stored literal under exact typed equality; otherwise Open.
- RemainingNotional is the Amount in the declared state field, or the explicit
  NotApplicable variant for `no_remaining_notional`.
- AgreementStatus is Outstanding iff any obligation is Outstanding or a declared
  remaining notional is greater than zero; otherwise NoOutstanding.

Episode Closed therefore never implies Agreement NoOutstanding. In the loan,
accrue creates separate `lam01:period1:PR` (500,000,000) and `...:IP`
(33,972,602) Outstanding records and leaves notional 4,500,000,000 micro-USD.
Settle marks both records Settled, closes the episode, and still derives agreement
Outstanding with remaining notional 4,500,000,000. The tombstones prevent a
duplicate create or settle. In the swap, remaining notional is NotApplicable;
agreement obligation status is independent of whether the epoch is Open/Closed.

## Genesis and lifecycle

Genesis binds the program reference, bounds, initial state, positive lifetime,
exclusive horizon, instance ID, principal/actor bindings, observation-provider
bindings, and mandatory claim root. The initial state has revision 0, remaining
equal to lifetime, no obligations, and deterministically derived statuses. A
continuation, redeployment, new request, or receipt cannot replace genesis or
increase remaining/horizon. Split, join, Pending, and continuation export are
unsupported and cannot reset either bound.

The genesis `requiredClaimRoot` must equal the domain-separated claimRoot of the
manifest's exact ordered ClaimRequirement array. EvaluationInput.claimEvidence supplies one
matching ClaimEvidenceRef per requirement and no extras. The signed authority
contains only the exact ClaimRequirement list and its root, not that evidence. The array must cover
ContractProperty (including every policy proof identifier), IntentRefinement,
TransitionValidity, and PredecessorHistory. External proof verification must bind
each proof/public-input digest to the program, genesis/instance, predecessor set,
authority, observations, before state, action trace, and after state as applicable.
This source profile fixes the acceptance interface and fail-closed requirement;
it supplies no PCD construction or verifier.

Every input state must satisfy checked `revision + remaining == genesis.lifetime`.
Every Complete action decrements remaining exactly once and increments revision
exactly once. Remaining zero rejects before execution; revision overflow rejects.
The horizon is the genesis UInt128 exclusive UTC epoch-second bound. `obs.now`
is explicit and externally authenticated; the evaluator never reads a host
clock. It requires `now < horizon`. The horizon guards repeated in the examples
are intentional redundant developer diagnostics; this mandatory check applies
even if the source guard is absent or drifts.

When a program has a distinct terminal resource-return action separate from its
repeatable financial action, each repeatable action must contain a source guard
reserving at least one allowance (`remaining > 1`). The swap is such a program
and contains the guard. The loan's settle action is the required second step of
its fixed two-step episode, not a separate resource-return cleanup action, so the
reserve rule does not apply. No generic action receives a reset from this rule.

## Authority, observations, and external assumptions

Evaluation receives the exact closed `EvaluationInput` in `typed-schemas.md`.
The compact ProgramRef must equal the recomputed full program reference and state/
genesis bindings. The authenticated principal must equal the signed principal;
exactly one genesis PrincipalBinding must map it to an actor; and that actor must
equal the action's required `actor` Text argument. Thus source guards such as
`arg.actor == const.borrower` constrain a program role, while the mandatory
wrapper binding proves which authenticated principal supplied that actor. Text
equality alone is never authentication.

Every declared observation appears exactly once, with the genesis-bound provider
and correct stored type. `now` is mandatory. The wrapper authenticates observation
evidence and oracle/provider identity. The language neither asserts oracle truth
nor permits an undeclared host read.

ExactPlan authority signs the exact instance, ActionCall, writes, and enriched
effects. IntentRefinement signs the exact instance, allowed actions, recipient/call permissions,
gross debit caps including Fee, minimum net credits after debits and fees,
predecessors, claim requirements, validity, principal, nonce, domain, genesis
hash, input-state hash, and ProgramRef. Proof evidence is not signed; its exact
ProofContext and the acyclic acceptance sequence are in typed-schemas.md. Checks use
ledger amounts after exact quantum conversion. Refunds do not reduce gross debit.
Net goals use checked credits minus debits/fees for the named actor/asset and fail
if the promised minimum is not met. This language has no external-call effect,
so the permitted-call list is empty.

Signature verification, key control, nonce freshness across durable history,
state unconsumed/currentness, predecessor ordering/finality, observation truth,
required contract/intent/transition/history PCD claims bound to the derived
ProofContext, asset authenticity,
balance conservation, and ledger application are external mandatory predicates.
All corresponding trusted checks must be true; missing implementations fail
closed. A hash-linked receipt, local simulation, documentary proof string, or
MockProver result is not PCD. Historical `MORIARTY-SIGN-v1` and
`MORIARTY-OUTCOME-SIGN-v1` bytes/domains remain unchanged and cannot be relabeled
as the new authority variants.

## Deterministic validation and transition

Validation occurs in these stages; diagnostics within one stage sort by
`(primarySpan.startByte,primarySpan.endByte,code)`:

1. UTF-8/source byte validation.
2. Longest-match lexing, exact keyword classification, identifiers, integer and
   JSON-string tokens; comments and plain slash reject.
3. Complete parse; recovery never creates executable AST.
4. Closed declarations, exact effect schemas, uniqueness, counts and AST bounds.
5. Declaration-before-use value/type resolution and deterministic node IDs/spans.
   Policy metadata uses complete-action-table resolution in stage 6.
6. Types, unit vectors, let locality, UInt bounds, policy target/node coverage,
   settlement one-to-one bindings, status rules, and actor parameter.
7. All simultaneous source, AST, typed-program and program-shape bounds.
   Per-category maxima never waive aggregate limits.
8. Deterministic lowering, source maps, canonical SemanticManifest and full
   BoundProgram representability, bounds hash and program hash recomputation.
9. Closed canonical evaluation/genesis/state/action/authority/observation schemas,
   all digest recomputations, and compact ProgramRef equality.
10. Current genesis/state/revision/remaining/horizon plus external authority,
    principal/actor, nonce, validity, observation, and predecessor checks.
    Claim evidence is structurally checked here; verify its ProofContext in stage 13.
11. Sequential guard/let/set/emit execution with checked arithmetic and policy
    provenance in a private working copy.
12. Settlement, obligation, transfer conservation, status, exact-plan or outcome
    refinement, gross/net, resource, and result representability checks.
13. Derive the candidate obligation copy, statuses, values, revision+1, remaining-1,
    stateHash, Complete traceHash and ProofContext without commit. Verify all
    mandatory claims against that context. Commit atomically only after success.
    Any failure returns only Rejected.

Complete means the selected atomic action and signed/refined outcome completed.
It does not mean the full contract is discharged. Pending is not a result variant;
a request needing residual progress rejects `UNSUPPORTED_PENDING` before commit.

## Bounds, examples, and scope

Every applicable bound in `bounds.json` holds simultaneously: source, AST, full
BoundProgram, compact signing statement, state, and result each have their own
byte/depth/node/record/array limits. The full program-hash preimage contains Core
instructions and uses the manifest depth-40 limit. Expanded AST, typed-program
and manifest records have separate finite byte/node limits from signed objects. A signing statement uses only
ProgramRef and depth 16. Neither object embeds the other. Actual aggregate program
nodes/bytes decide representability, so 64 actions each at their local maximum are
not promised to fit the 32,768-node manifest.

The loan example specifies one first LAM period, not ACTUS conformance. Its exact
interest is 2,480,000,000/73 micro-USD, floor 33,972,602 with discarded remainder
54/73; combined settlement is 533,972,602; remaining notional is 4,500,000,000.
The swap specifies only the fixed input vector enforced by expected_output: exact
output 1,994,000,000/100,997, floor 19,743, reserves 1,010,000 and 1,980,257. It
is not a reusable constant-product implementation or Uniswap conformance.

ACTUS calendar/event/convention/signed-value/quantity-price behavior and DeFi
concentrated liquidity, hooks, partial fill, routing, convergence, governance,
and the rest of all 32/72 target rows remain mandatory versioned extensions and
MC07 work. No source example, schema check, arithmetic calculation, or future
review establishes parser/compiler implementation, native proof, PCD, ledger
acceptance, compiler correspondence, safety, or corpus conformance.

Changing numeric width/order/rounding, unit or quantum meaning, codec, bounds,
effect/obligation/status/authority schema, lifecycle, hashing, or lowering creates
a new semantic profile and new program hashes/domains. It invalidates affected
proofs, certificates, theorem instances, vectors, and audits, while historical
artifacts remain bound to their original bytes and predicates.
