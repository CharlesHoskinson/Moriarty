# Moriarty bounded atomic profile: static and dynamic semantics

Status: S2 provisional corrected candidate, specified-only and not frozen or implemented.
Fresh independent Fable and GPT-6 audits of these exact bytes remain required.
Profile `moriarty-bounded-atomic/1`; Core `moriarty-core/1`. The closed wire
records and hash preimages in `typed-schemas.md`, simultaneous limits in
`bounds.json`, and numeric rules in `numeric-profile.json` are normative with
this document. A conflict is an error in the candidate, not a precedence choice.

## Domains, source, and judgments

Let `U = {0..2^128-1}`. A declared nominal unit has a canonical unit vector with
one exponent `1`; UInt128 has the empty vector. Bool and Text have exactly `[]`
as nonnumeric unit metadata; their explicit type distinguishes them from UInt128. A vector has at most eight
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
namespace. The separate namespaces are units, constants, state fields,
observations, settlement bindings, field policies, effect kinds, actions, and
(each separately per action) parameters and locals. Cross-namespace name
collisions are permitted; prefixed references select the namespace.
Policy target actions, rounding locals, and reserve action/closure names are forward
metadata references: resolve them over the complete action table in stage 6,
then require all target/value-local constraints before evaluation. A policy may
precede or follow the action it covers. Its unit reference still requires a
previous unit declaration. Settlement bindings may precede or follow the actions
that use them, but their quantum unit must precede the settlement declaration.
Status-rule state references require previously declared state fields;
ReserveDecl and action-target/rounding metadata are position-independent.
Effect schemas still precede their emits. Each action has exactly one `actor: Text` parameter. A word that
exactly equals a grammar keyword is a keyword, never an identifier; only `asset`
and `amount` are contextual effect-field labels. `not` binds tighter than a
comparison, so `not a == b` parses `(not a) == b` and normally fails typing.

Source spans are zero-based UTF-8 byte intervals `[start,end)`. Lowering retains
the source hash, version, node IDs, types, unit vectors, resolution targets, and
source references on Core statements and expressions. In moriarty-core/1 every
sourceRef uses Source and exactly one span; other generated tags are reserved for
MC02 and reject here. No compiler may discard or fabricate this binding. Exact
token-boundary spans, structural parenthesis erasure and deterministic IDs are
specified in `typed-schemas.md`; merely in-range spans are insufficient.

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
The delta arrays capture immutable snapshots at each successful event: DueCreated
appends an Outstanding copy to created; DueSettled appends the updated Settled
copy to settled. Creating, transferring and settling one due in the same action
therefore retains both snapshots and a final Settled tombstone. It never rewrites
the created snapshot to the final status.

For DueCreated:

1. amount must be positive; denomination must equal the Amount unit identifier;
2. neither Outstanding nor Settled record may already have the due ID;
3. capacity for one retained record must remain; and
4. insert the exact debtor, creditor, denomination, unit, amount, and Outstanding.

Zero amount rejects `OBLIGATION_ZERO`; denomination/unit disagreement rejects
`OBLIGATION_MISMATCH`; duplicate identity rejects `OBLIGATION_DUPLICATE`; capacity
failure rejects `OBLIGATION_CAPACITY`. For DueSettled, exactly one existing due ID must exist.
Debtor, creditor, denomination, and unit must all equal it. Zero rejects
`OBLIGATION_ZERO`. An
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
manifest's exact ordered ClaimRequirement array. Post-trace
ProofAcceptanceInput.claimEvidence supplies one matching ClaimEvidenceRef per
requirement and no extras. The signed authority
contains only the exact ClaimRequirement list and its root, not that evidence. The array must cover
ContractProperty (including every policy proof identifier), IntentRefinement,
TransitionValidity, and PredecessorHistory. External proof verification must bind
each proof/public-input digest to the program, genesis/instance, predecessor set,
authority, observations, before state, exact actionHash, action trace, and after state
through the exact ProofContext and its committed objects.
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

Closure reserve is explicit, decidable, and hash-bound. Source
`reserve swap for close;` lowers to `ReserveRule {action:"swap",closure:"close"}`.
Stage 6 checks every reserve declaration as follows, rejecting `RESERVE_RULE`:

1. Both names resolve to existing, distinct actions in the complete action table.
2. Each reserved action occurs in at most one ReserveRule. A named closure cannot
   itself occur as any ReserveRule.action. Multiple reserved actions may share
   one closure.
3. The reserved action contains a top-level Guard whose condition is exactly
   `Gt(Remaining, Literal(UIntLiteral token "1"))` after structural parenthesis
   erasure. Spans and the guard message do not affect this shape test. Equivalent
   arithmetic or Boolean reformulations do not satisfy it. Other guards remain
   permitted, and no instruction-position inference is used.

No rule is inferred from an action name, emitted resources, repeatability, or
example identity. A program without reserve declarations incurs no reserve check;
the loan has none and receives no implicit exception. The swap explicitly declares
the rule. It blocks that action when remaining is one, preserving the allowance
for the named closure. It proves neither closure feasibility nor that the closure
returns resources, and cannot reset lifetime or horizon. Keys, guards, balances,
mandatory proofs and aggregate result limits can still prevent closure.

## Binding equalities

Stage 9 uses canonical byte equality for records/arrays and exact scalar equality.
The independently supplied BoundProgram is rechecked and its ProgramRef recomputed.
These predicates are mandatory; the code in parentheses rejects a mismatch:

- input.program equals that recomputed ProgramRef; manifest.bounds equals
  input.program.bounds and the actual bounds registry reference (`PROGRAM_BINDING`).
- genesis.body.program equals input.program (`GENESIS_PROGRAM_BINDING`).
- genesis.body.initialState equals manifest.initialState, preserving initial
  literals and declaration order (`GENESIS_INITIAL_STATE_BINDING`).
- genesis.body.lifetime equals manifest.lifetime (`GENESIS_LIFETIME_BINDING`).
- genesis.body.horizon equals manifest.horizon (`GENESIS_HORIZON_BINDING`).
- genesis.body.bounds equals input.program.bounds (`GENESIS_BOUNDS_BINDING`).
- state.body.programHash equals input.program.programHash (`STATE_PROGRAM_BINDING`).
- state.body.genesisHash equals genesis.genesisHash (`STATE_GENESIS_BINDING`).
- state.body.instanceId equals genesis.body.instanceId (`INSTANCE_BINDING`).
- authority.statement.program equals input.program (`AUTHORITY_PROGRAM_BINDING`);
  its domain, genesisHash, instanceId and beforeStateHash equal genesis.body.domain,
  genesis.genesisHash, genesis.body.instanceId and state.stateHash respectively
  (`AUTHORITY_CONTEXT_BINDING`).
- authority.statement.requiredClaims equals manifest.requiredClaims, and both its
  requiredClaimRoot and genesis.body.requiredClaimRoot equal the recomputed
  manifest claimRoot (`CLAIM_ROOT_BINDING`).
- authority.statement.predecessors equals exactly `[state.stateHash]`
  (`PREDECESSOR_BINDING`).
- In ExactPlan mode, authority.statement.action equals input.action
  (`EXACT_ACTION_BINDING`). In IntentRefinement mode, input.action.name occurs in
  allowedActions (`INTENT_ACTION_FORBIDDEN`); every allowed name is declared.

All supplied digest wrappers are independently recomputed (`DIGEST_MISMATCH`).
Action name, argument names/order/types, state values/schema and observation
names/order/types must match the manifest (`INPUT_SCHEMA`). State obligations
have unique due IDs, positive correctly denominated amounts and declared units
(`INPUT_SCHEMA`). At revision zero, state values equal manifest.initialState,
remaining equals lifetime, obligations are empty and statuses are derived exactly
(`GENESIS_STATE_BINDING`); later states require authenticated currentness/history.
Stage 10 also checks checked revision + remaining == genesis lifetime, the
exclusive horizon, valid interval, all prechecks and the principal-to-actor rule.

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
Only the principal-bound actor A may appear in any cap or goal; other actors
reject `INTENT_ACTOR_SCOPE`. Every Transfer or Fee with from == A requires a cap
for (A,asset), even for zero amount, or rejects `INTENT_DEBIT_UNCAPPED`. Checked
sums of those ledger amounts must not exceed the corresponding caps
(`INTENT_DEBIT_CAP`). Refunds never reduce these gross sums. For each goal,
credits sum Transfer/Fee with to == A and debits sum Transfer/Fee with from == A
for that asset. A self-transfer contributes to both sums. DueCreated and
DueSettled contribute to neither sum, avoiding double-counting settlement records.
A missing debit or credit sum is zero. The goal passes exactly when
`checked(credits) >= checked(debits + minimumLedgerAmount)`; failure rejects
`INTENT_NET_GOAL`. Every sum and addition is UInt128 in ledger units after quantum
conversion; overflow rejects `INTENT_ARITHMETIC_OVERFLOW`. All Transfer/Fee
recipients, including nonprincipal transfers, must appear in permittedRecipients
(`INTENT_RECIPIENT_FORBIDDEN`). This language has no external-call effect, so a
nonempty permittedCalls list rejects `INTENT_CALL_UNSUPPORTED`.

Signature verification, key control, nonce freshness across durable history,
state unconsumed/currentness, predecessor ordering/finality, observation truth,
required contract/intent/transition/history PCD claims bound to the derived
ProofContext, asset authenticity,
balance conservation, and ledger application are external mandatory predicates.
All pre-execution checks and post-trace backend verdicts must be true in their
respective phases; missing implementations fail closed. A hash-linked receipt, local simulation, documentary proof string, or
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
   settlement one-to-one bindings, status rules, actor parameter and explicit
   ReserveRule validation.
7. All simultaneous source, AST, typed-program and program-shape bounds.
   Per-category maxima never waive aggregate limits.
8. Deterministic lowering, source maps, canonical SemanticManifest and full
   BoundProgram representability, bounds hash and program hash recomputation.
9. Closed canonical evaluation/genesis/state/action/authority/observation schemas,
   all digest recomputations, and every listed Binding equality.
10. Current genesis/state/revision/remaining/horizon plus external authority,
    principal/actor, nonce, validity, observation, and predecessor checks.
    EvaluationInput carries prechecks only; no claim evidence exists in this phase.
11. Sequential guard/let/set/emit execution with checked arithmetic and policy
    provenance in a private working copy.
12. Resolve settlement, process obligations and transfer conservation, derive
    candidate statuses/values/revision+1/remaining-1, and check exact-plan or
    outcome refinement, gross/net, resources and full result representability.
13. Finalize candidate stateHash, recomputed actionHash, Complete traceHash and
    ProofContext without commit. Receive the separate ProofAcceptanceInput,
    resolve bounded proof bytes and verify all mandatory claims against that exact
    context. Accept only the matching trusted ProofAcceptanceVerdict and recheck
    durable currentness/nonce atomically with commit. Any failure returns Rejected.
    The numbered two-phase protocol and implementing API are in typed-schemas.md.

Complete means the selected atomic action and signed/refined outcome completed.
It does not mean the full contract is discharged. Pending is not a result variant;
a request needing residual progress rejects `UNSUPPORTED_PENDING` before commit.

## Closed diagnostics

This is the closed DiagnosticCode registry. No other code is legal in this profile.
Each code has exactly the stage below; a condition described elsewhere maps to
this table even where its prose does not repeat the code. Malformed data stops
before dependent semantic checks. Validate stages in numeric order and stop at
the first failing stage; for execution-dependent stages, stop at the first failing
operation in the specified evaluation/effect order. Where independent errors at
that point coexist, choose the smallest `(primarySpan.startByte,primarySpan.endByte,code)`.
Return exactly one Diagnostic (therefore the rejection itself remains bounded).
Use the failing source expression/statement/declaration span when available, or
`{startByte:"0",endByte:"0"}` for input/backend/global errors without a source node.
`relatedSpans` is `[]`; `message` equals the code, except GUARD_FAILED uses the
exact source guard message. Parsing/checking returns this same diagnostic directly.

| Code | Stage | Rejection condition |
|---|---:|---|
| SOURCE_ENCODING | 1 | Invalid UTF-8 or source byte cap exceeded. |
| LEXICAL_TOKEN | 2 | Invalid token, forbidden source comment/slash, noncanonical decimal token, invalid JSON string, surrogate, identifier or language-text lexical limit, forbidden/reserved identifier use. |
| PARSE_ERROR | 3 | Source does not fully parse under grammar.ebnf, including missing or trailing tokens. |
| DECLARATION_SCHEMA | 4 | Missing action, invalid header profile, missing/extra/malformed declaration or fixed effect schema/field order. |
| DUPLICATE_NAME | 4 | Duplicate name in a declaration/parameter/local namespace, duplicate effect kind or field, or duplicate write to a field in an action. |
| AST_BOUNDS | 4 | SourceAST exceeds any encoding bound, including declarations > 128. |
| NAME_RESOLUTION | 5 | Unknown or premature value/type/unit reference or a bare identifier not resolving to a prior local. |
| TYPE_MISMATCH | 6 | Operator, literal, initializer, set, argument declaration, effect field, stored/local type, or Bool/Text unit metadata violates exact typing. |
| UINT_RANGE | 6 | Source integer outside UInt128, or lifetime is zero. |
| UNIT_VECTOR | 6 | Invalid numeric unit vector/classification, component count or exponent bound. |
| POLICY_TARGET | 6 | Duplicate, absent, unknown, wrong-kind, wrong-unit, out-of-range or nonfinancial policy target; uncovered Amount occurrence. |
| POLICY_ROUNDING | 6 | Rounding reference is unknown, wrong-action, ambiguous, unknown, wrong-action, ambiguous, non-let or non-FloorDiv-root reference, or its root is not FloorDiv. |
| SETTLEMENT_DECLARATION | 6 | Empty asset, nonpositive quantum, quantum/declared-unit mismatch or malformed settlement declaration. |
| SETTLEMENT_BINDING_AMBIGUOUS | 6 | Duplicate settlement name, unit or asset prevents one-to-one binding. |
| STATUS_RULE | 6 | Missing/duplicate status rule or rule field/literal has wrong type or missing declaration. |
| ACTOR_PARAMETER | 6 | Action lacks exactly one actor:Text parameter. |
| OBSERVATION_SCHEMA | 6 | Source lacks now:UInt128 or declares now at another type. |
| RESERVE_RULE | 6 | Reserve declaration violates any of the three explicit reserve checks. |
| PROGRAM_BOUNDS | 7 | Any simultaneous typed-program or program-shape bound fails; includes expression/instruction/count limits. |
| SOURCE_MAP | 8 | AST span, typed annotation, node ID, resolution, Core tree, generatedTag or sourceRef differs from deterministic source lowering. |
| PROGRAM_ENCODING | 8 | BoundProgram/manifest is noncanonical, malformed, out of aggregate bounds, or differs from deterministic lowering/hash. |
| INPUT_SCHEMA | 9 | Noncanonical or malformed evaluation/genesis/state/action/authority/observation input, unknown fields, wrong argument/value schema/order, unsupported authority mode, invalid ordered/unique arrays or observation/principal binding schema. |
| INPUT_BOUNDS | 9 | Any signing/evaluation encoding or input record/count bound fails. |
| DIGEST_MISMATCH | 9 | Supplied program/source/bounds/genesis/state or other digest fails its registered recomputation. |
| PROGRAM_BINDING | 9 | Input ProgramRef or manifest bounds differs from recomputed program/registry. |
| GENESIS_PROGRAM_BINDING | 9 | Genesis ProgramRef differs from input.program. |
| GENESIS_INITIAL_STATE_BINDING | 9 | Genesis initialState differs from manifest.initialState. |
| GENESIS_LIFETIME_BINDING | 9 | Genesis lifetime differs from manifest.lifetime. |
| GENESIS_HORIZON_BINDING | 9 | Genesis horizon differs from manifest.horizon. |
| GENESIS_BOUNDS_BINDING | 9 | Genesis bounds differs from program.bounds. |
| STATE_PROGRAM_BINDING | 9 | State programHash differs from program.programHash. |
| STATE_GENESIS_BINDING | 9 | State genesisHash differs from genesis.genesisHash. |
| INSTANCE_BINDING | 9 | State instanceId differs from genesis instanceId. |
| AUTHORITY_PROGRAM_BINDING | 9 | Signed ProgramRef differs from input.program. |
| AUTHORITY_CONTEXT_BINDING | 9 | Signed domain/genesis/instance/before-state binding differs from actual input. |
| CLAIM_ROOT_BINDING | 9 | Signed requirements or signed/genesis claim root differs from generated manifest claims/root. |
| PREDECESSOR_BINDING | 9 | Signed predecessor array is not exactly the singleton current before-state hash. |
| EXACT_ACTION_BINDING | 9 | Signed ExactPlan ActionCall differs from EvaluationInput.action. |
| INTENT_ACTION_FORBIDDEN | 9 | Outcome allowedActions contains an undeclared action or omits the selected action. |
| GENESIS_STATE_BINDING | 9 | Revision-zero state differs from the prescribed genesis values, allowance, empty obligations or derived status. |
| GENESIS_UNAUTHENTICATED | 10 | Trusted genesisValid is false or unavailable. |
| SIGNATURE_INVALID | 10 | Trusted signatureValid is false/unavailable, including unsupported external algorithm. |
| NONCE_STALE | 10 | Trusted pre-execution nonceFresh is false/unavailable. |
| STATE_NOT_CURRENT | 10 | Trusted pre-execution stateCurrentAndUnconsumed is false/unavailable. |
| OBSERVATION_UNAUTHENTICATED | 10 | Trusted observationsAuthentic is false/unavailable or supplied provider differs from genesis binding. |
| PREDECESSOR_UNAUTHENTICATED | 10 | Trusted predecessorSetValid is false/unavailable. |
| PRINCIPAL_BINDING | 10 | Authenticated principal differs from signed principal, has no unique genesis actor, or that actor differs from arg.actor. |
| LIFETIME_INVARIANT | 10 | Checked revision + remaining overflows or differs from genesis.lifetime. |
| LIFETIME_EXHAUSTED | 10 | remaining == 0. |
| HORIZON_EXPIRED | 10 | Authenticated now >= genesis.horizon. |
| VALIDITY_INTERVAL | 10 | notBefore <= now < notAfterExclusive <= genesis.horizon fails. |
| STATUS_MISMATCH | 10 | Supplied non-genesis status/notional fields differ from derivation from values and obligations. |
| ARITHMETIC_OVERFLOW | 11 | Checked expression add/multiply or another numeric expression result exceeds UInt128. |
| ARITHMETIC_UNDERFLOW | 11 | Expression subtraction would be negative. |
| DIVISION_BY_ZERO | 11 | FloorDiv denominator equals zero. |
| GUARD_FAILED | 11 | Evaluated guard condition is false. |
| POLICY_PROVENANCE | 11 | An Amount write/emit value has nonempty provenance for rounding none or other than the exact singleton for rounding floor. |
| SETTLEMENT_BINDING_MISSING | 12 | Transfer/Fee/DueSettled asset and amount unit do not resolve to the same unique binding. |
| SETTLEMENT_NON_DIVISIBLE | 12 | Nominal amount is not exactly divisible by quantum. |
| SETTLEMENT_OVERFLOW | 12 | Checked settlement conversion or inverse multiplication fails UInt128. |
| OBLIGATION_ZERO | 12 | DueCreated or DueSettled amount is zero. |
| OBLIGATION_DUPLICATE | 12 | DueCreated reuses any retained due ID, including a settled tombstone. |
| OBLIGATION_CAPACITY | 12 | DueCreated would exceed retained obligation capacity. |
| OBLIGATION_UNKNOWN | 12 | DueSettled has no matching retained due ID. |
| OBLIGATION_ALREADY_SETTLED | 12 | DueSettled names an already settled record. |
| OBLIGATION_MISMATCH | 12 | Due denomination differs from amount unit, or settlement debtor/creditor/denomination/unit differs from retained identity. |
| OBLIGATION_PARTIAL_UNSUPPORTED | 12 | Positive settlement amount is below outstanding amount. |
| OBLIGATION_EXCESS | 12 | Settlement amount exceeds outstanding amount. |
| OBLIGATION_CONSERVATION | 12 | DueSettled nominal or ledger group sums differ from corresponding Transfer sums. |
| OBLIGATION_SUM_OVERFLOW | 12 | Checked obligation/Transfer conservation group sum exceeds UInt128. |
| EXACT_PLAN_MISMATCH | 12 | Produced write projection/effects differ in length, order or canonical value from signed exact arrays. |
| INTENT_ACTOR_SCOPE | 12 | Any outcome cap or net goal names an actor other than the principal-bound actor. |
| INTENT_DEBIT_UNCAPPED | 12 | Any principal outgoing Transfer/Fee lacks its actor/asset cap. |
| INTENT_DEBIT_CAP | 12 | Checked principal gross ledger debits exceed a matching cap. |
| INTENT_NET_GOAL | 12 | Checked credits < checked(debits + minimumLedgerAmount) for a goal. |
| INTENT_ARITHMETIC_OVERFLOW | 12 | Any intent ledger sum or debits + minimum exceeds UInt128. |
| INTENT_RECIPIENT_FORBIDDEN | 12 | Any Transfer/Fee recipient is absent from permittedRecipients. |
| INTENT_CALL_UNSUPPORTED | 12 | Outcome permittedCalls is nonempty. |
| REVISION_OVERFLOW | 12 | Incrementing revision by one exceeds UInt128. |
| RESOURCE_BOUNDS | 12 | Executed instruction, expression, effect or other runtime count exceeds its bound. |
| RESULT_BOUNDS | 12 | Candidate state or complete result wrapper violates its applicable encoding/size limit. |
| UNSUPPORTED_PENDING | 12 | Requested result requires Pending, split/join, continuation export or residual progress in this atomic profile. |
| PROOF_SCHEMA | 13 | ProofAcceptanceInput/Verdict is malformed, noncanonical or exceeds proofAcceptanceEncoding. |
| PROOF_REQUIREMENTS | 13 | Evidence/proof arrays omit, duplicate, add, reorder or mismatch required claim kind/ID. |
| PROOF_RESOLUTION | 13 | Resolver unavailable, empty or invalid handle, immutable resolution/byte-length mismatch or any resolved proof count/byte bound fails. |
| PROOF_DIGEST | 13 | ClaimEvidenceRef.proofDigest differs from SHA256 of corresponding resolved proof bytes. |
| PROOF_CONTEXT | 13 | Supplied context, actionHash, publicInputDigest or verdict context hash differs from the exact locally derived context. |
| PROOF_INVALID | 13 | Trusted backend unavailable, unknown claim, verification failure, untrusted verdict or either verdict boolean false. |
| COMMIT_CONFLICT | 13 | Candidate differs from local derivation, or durable state/nonce currentness changed before atomic consumption. |

## Bounds, examples, and scope

Every applicable bound in `bounds.json` holds simultaneously: source, AST, full
BoundProgram, compact signing statement, state, and result each have their own
byte/depth/node/record/array limits. The full program-hash preimage contains Core
instructions and uses the manifest depth-40 limit. Expanded AST, typed-program
and manifest records have separate finite byte/node limits from signed objects. A signing statement uses only
ProgramRef and depth 16. Neither object embeds the other. Actual aggregate program
nodes/bytes decide representability, so 64 actions each at their local maximum are
not promised to fit the 32,768-node manifest. SourceAST.declarations has an
explicit effective total cap of 128, including reserve declarations, even when
all category limits fit. ResultEncoding applies to the entire Complete wrapper:
a representable state may have no representable transition or closure because
its result exceeds that cap. This is an accepted profile limitation; reject
RESULT_BOUNDS without consumption. Neither finite syntax nor closure reserve
provides a blanket transition or closure liveness guarantee.

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


## Count scopes and parser implementation

In bounds.json programShape, policyTargets is the total number of target entries
across all field policies; effectFields applies separately to each effect schema.
argumentFieldsPerEntrypoint, instructionsPerEntrypoint, localsPerEntrypoint,
expressionNodesPerEntrypoint and effectsPerEntrypoint apply to each action.
expressionDepthRootOne applies to each expression tree. All declaration-category
counts and reserveRules are program totals. obligationRecordsIncludingSettled
applies to each state; expressionUnitComponents and absoluteUnitExponent apply
to each unit vector/component; predecessorFanIn applies to each transition.
Parenthesis nesting has no separate limit: the finite source byte cap bounds it.
A conforming parser must handle profile-valid redundant parentheses without an
implementation-specific recursion-depth rejection.

Outcome intent debit caps do not bound new nominal obligations. DueCreated and
DueSettled are excluded from ledger debit/credit sums; allowedActions and mandatory
contract/refinement proofs still govern those transitions. Applications requiring
an explicit signed nominal-debt cap need a future profile extension.

Freeze status is an external approval record bound to exact file digests. Do not
edit status strings inside bounds.json to record approval; its exact bytes are
part of boundsHash. Historical candidate status text remains its provenance.
