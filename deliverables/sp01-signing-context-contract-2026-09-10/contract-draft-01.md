# Signing and contextual history contract draft

Status: proposed SP01.3 source contract; independently expected examples only.
No signature, wire compatibility, evaluator, proof or ledger acceptance is
established. This draft does not approve archived candidate04 or close full RP01.
The source manifest identifies exact inspected inputs and the proposed expression
dependency. Financial-operation record layouts and equations remain owned by
`deliverables/sp01-financial-operation-contract-2026-09-10/`.

## Source defects and intended corrections

The archived review `SP01/independent-review-round-08/successor/gpt6-semantic-partial-review.md`
controls these dispositions. Its 965 passing checks did not establish contextual
acceptance. The archived generator, schema and test are retained evidence only.

| Defect | Required contract change | Independent discriminator |
| --- | --- | --- |
| C04-F2 | Resolve metadata through the declared named argument/type, never its array index | Reorder existing debt_id and qty bindings; obligation identity stays Loan01 |
| C04-F3 | Separate loan and swap genesis, retain originating authorization and exact funding/debt inputs | Reject undeclared loan field, missing funding or absent originating document |
| C04-F4 | Derive cancellation deficit from current same-asset history and reserve enforceable backing | Valid deficit30/reserve30 versus reserve29; fill/cancel race |
| C04-F5 | Derive work from complete selected Core and operation trace, not certificate totals; partition after charge | Sum omitted node, zero-priced op, cloned residual, valid split/join |
| C04-F6 | Compute ordered predicates over a fully linked history; expected labels are not inputs to validation | Remove mutation and obtain positive result; rebuild dependent commitments for semantic mutations |

## 1. Proposed dependency and representation boundary

Use the current proposed expression `representation.md` mathematical J/tree,
finite acyclic types, schema, Args/Obs/Pre/Post and emitted Operation descriptors.
`ExpressionPrepared` is preparation only. It neither authorizes financial effects
nor admits a transaction. Its published bounds are separate component bounds,
not a claimed 65,536-byte aggregate signing-document limit.

Signing requires a separately reviewed closed envelope and byte/node/list bounds
for the whole document, claims, predecessors, observation sidecars and history
witness. Those numeric bounds, signature suite, digest algorithm/domain strings,
native correspondence and implemented canonical codec remain OPEN. No `/0`
archived domain is adopted by this draft. Unknown extensions/keys, duplicate keys,
invalid scalar/domain values and noncanonical bytes must reject before hashing.
A future envelope must use an acyclic commitment graph; do not commit a successor
state to an acceptance that itself commits that same successor state.

Proposed dependency is pinned source candidate02, not a statement that its
101 files constitute a financial or signing profile. A changed dependency digest
reopens affected representation/signing review. Source/Core correspondence is an
additional obligation; a source hash and action-name list do not supply a body.

## 2. Authority identities and display

Define distinct identities for (a) originating signed authority, (b) concrete
selected plan, (c) predecessor state/consumption identity, (d) prepared result,
(e) successor, and (f) accepted transition. Never substitute one for another.

Every originating authority binds: mode; exact schema/spec/profile/policy and
program/source/Core identities; network and deployment; instance and genesis;
signer/principal/key authority and nonce; validity/activation policy; required
ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance
claims; permitted observations/anchors/freshness and truth assumptions; allowed
operations/assets/recipients/calls; separate gross, fee and nominal-debt limits;
net goals and their timing; partial-fill/cancellation rules; locks, duties,
continuation/recovery ownership; lifetime work and verification bounds.
Every relevant omitted/empty permission means no permission, not unrestricted.

Exact-plan authorization binds the actual action body and typed named arguments,
exact predecessor states, observations or their explicit admitted constraints,
all expected writes/effects and residual obligations. The selected plan must be
that exact plan, with all committed values equal. Any change to a committed plan
field requires new authorization; semantically similar route substitutions are
not exact-plan execution.

Outcome-intent authorization binds hard predicates and the allowed predecessor
constraint; a solver-selected concrete plan has its own identity. Acceptance
binds BOTH the unchanged intent identity and the selected plan identity, selected
current predecessors, observations, complete effects, liabilities and work.
A different admissible route may change the selected-plan identity without
changing the originating intent. Soft ranking is not a hard condition and does
not prove optimality. Pinned constraints require exact state equality;
admitted-context constraints require the named instance, policy and event-order
conditions plus currentness, not arbitrary history selection.

The originating authority is immutable across residuals. A new exact-plan
signature for a continuation can authorize that concrete step but cannot replace
or replenish the original lifetime authority. Distinct new funding/authority
must have an explicit authorized extension transition, never a nonce reset.

Display must be generated from parsed canonical signed bytes and the exact typed
schema. Resolve an argument's metadata by enclosing binding name and declared
type/operation signature, with unique declared names; path index only locates the
leaf. Reordered named bindings change paths/bytes and hence exact-plan identity,
but must not turn ObligationId into generic Text. If an eventual wire requires
canonical argument ordering, reject the reordered wire at that earlier boundary;
the metadata resolver still cannot depend on position. Executable expression-list
order remains the expression contract's lexical order.

Render all leaves, including empty prohibitions, actual indexed assets, gross
limits, fees, net predicate (Exact versus AtLeast), debt creation/accrual/exposure,
recipients, locks, residual rights and assumptions. Missing, extra, changed or
mislabelled leaves reject; non-display UI caches cannot authorize bytes. Preserve
the inspected price unit orientation rather than guessing from the variable name.
MC08 owns implemented parsing/rendering/signature interaction and workflow tests.

## 3. Funded genesis and complete contextual inputs

A genesis fixture contains its own declaration, full selected action/Core body,
unborn state, original exact-plan document, funding/creation authorizations and
inputs, resulting state/effects and residuals. Loan and swap have distinct
instances/declarations; a common profile does not make their field sets equal.
Every declared field is initialized once with the declared type; no undeclared
field occurs. Every financial collection is explicit, including empty balances,
debts, shares, requests, messages, claims, rewards, locks and duties.

For every asset, admitted input value plus explicitly authorized issuance equals
resulting balances plus external outputs plus burns/fees as separately classified.
No input is consumed twice. External funds in a source fixture are named assumed
inputs with owners, asset, amount and unique consumption references; they are not
verified ledger UTXOs. Arbitrary creation of an initial balance is not Genesis.

Nominal debt is not a token. Debt creation requires the debtor/creditor identities,
denomination, principal/interest conventions, signed creation/exposure authority
and named obligation identity. The funded loan below explicitly pairs the advance
with creation; the general language must also support separately specified legal
novation/credit models only through their own authorized rules. Do not assume all
debt equals a token transfer or treat a write-off as cash. Genesis funding and debt
creation may be separate linked transitions; retain both if so. Never invent a
missing authorization preimage in a generator and omit it from retained inputs.

## 4. Cumulative financial and work invariants

For each authority lineage, actor and asset, derive actual debit, fee and receipt
counters from complete effects. Refunds add balances but never subtract historical
gross debit. Gross excludes fees only if the profile explicitly charges fees in
the separate fee counter; both ceilings remain enforced. No fee can disappear
between the two classes. Residual limits equal original limits minus the relevant
cumulative consumption; an independently signed child cannot copy original caps.

A receiving-asset goal uses eligible cumulative receipts minus fees in that same
asset. This is NOT the actor's full balance delta: full delta also includes asset
outflows, refunds, lock changes and other classified effects. Refunds do not count
as trade proceeds unless the signed goal expressly includes them. For zero
receipts and fee30, receiving net=-30. Never aggregate different assets or assume
that unspecified fee units equal token/SPECK units. Exact19743 rejects19744;
AtLeast19743 permits19744 if every other predicate holds.

Debt outstanding = created + accrued - repaid - writtenOff - credited, indexed
by obligation/creditor/debtor/denomination, with checked nonnegative arithmetic.
Capitalization reclassifies components without double-counting debt. New nominal
creation, accrued additions and outstanding exposure need distinct stated signed
limits. Repayment does not erase cumulative created/accrued history or replenish
a lifetime addition ceiling. Write-off/credit require their own authorization.
Bound exhaustion leaves the obligation live; episode closure is not discharge.

Derive expression work from the complete selected Core under the expression
contract, and operation work from the exact emitted descriptor trace and a frozen
financial price table. The original certificate is at most a witness to this
calculation; omitted/unknown/underpriced components reject. Sidecar work is derived
from exact bytes and the reviewed schedule; admission/verification work and
ordinary/recovery transition reserves must not be conflated or double-counted.
The final combined schedule is OPEN with the financial operation bindings.

The histories below use explicit illustrative charges to test arithmetic only;
they do not reinstate candidate04's one-unit operation prices as current policy.
For a transition charge c: sum(child remaining)+retired remainder=parent remaining-c.
Recovery reserves obey the same conservation separately. Retired work cannot be
recovered by migration. On join, sum only distinct live siblings of compatible
origin/policy and subtract the join charge. Cumulative ledgers merge shared
ancestry once plus disjoint branch deltas; summing entire branch histories would
double-count the prefix. Duties/backing resources cannot appear in two spendable
children. RecoveryRight aliases a reserve; it is not another reserve.

## 5. Cancellation, currentness and migration

Fill and cancellation both consume the same current request/predecessor identity
when based on the same state. After one accepts, the other rejects currentness,
even with a valid signature and internally correct effect calculation. A new
cancellation can operate on the winner's successor only if its exact request
state and residual authority still permit cancellation. Deadline expiry is an
observation, not automatic debt/request settlement.

For cancellation with an unmet receiving goal, deficit=max(0,goal-cumulativeNet)
in the same asset at the selected CURRENT state. Under the explicit escrow-backed
policy in these examples, create a surviving duty from the authorized payer to
the goal recipient and reserve that exact amount from a named free balance.
A balance figure alone is not backing: ownership, permitted recipient, authority,
existing encumbrances, reservation identity and non-reuse must all be checked.
No reservation may back two duties or remain simultaneously freely spendable.
Reject insufficient/missing/wrong-asset/unauthorized backing; do not make the
recipient the debtor. Other credit-bearing backing needs its own financial
contract, not an implicit substitute for escrow. Goal copies in authority,
residual and history must agree. Cancellation of pending request input and
survival of already-funded output entitlement are distinct; exact record/status
bindings belong to the financial draft's request contract.

Currentness is an acceptance-layer atomic predicate on live predecessor and
consumption identities. Genesis nonce/instance identity must also be unique.
Migration consumes the current predecessor, preserves originating authority,
genesis lineage, financial counters/debts/duties and remaining work after its
charge, and creates a distinct successor. It cannot clear spent identities,
reset counters, widen permission or revive a cancelled/consumed request. An
old-spec signature can be cryptographically valid while activation/revocation
policy rejects it. Time windows are internally consistent [start,end) and checked
against the specified authenticated/assumed observation. Registry truth and
currentness are environmental inputs in these source examples, not proofs.

## 6. Required ordered validation and negative isolation

Proposed diagnostic order: (1) finite shape/canonical/domain checks;
(2) complete preimages, DAG links and source/Core/profile identity;
(3) signature authority, target and spec/key activation;
(4) declared action, argument/observation admission and predecessor binding;
(5) uniqueness/currentness and compatible lineage;
(6) derived work and authority bounds;
(7) complete financial transition, frames and conservation;
(8) intent/net/residual/backing obligations;
(9) all four mandatory claim validations and atomic commit.
This is a proposed diagnostic order, not the existing runtime's implementation.
Cheap bounds precede expensive checks. No partial effect publishes on failure.

Each eventual negative fixture must first have an independently valid sibling.
Rebuild every dependent plan/document/state/prepared/successor/acceptance digest
when changing a semantic input, and verify all predicates before the target.
For a hash test, deliberately do not rebuild exactly the targeted digest. For a
signature test, distinguish stale signature from freshly authorized invalid
semantics. Unsigned tests supply an explicit assumed-authority context; they
cannot claim signature acceptance. First failure is computed from data; expected
labels and unreachable-stage lists are assertion outputs, never validator inputs.
Removing the mutation must recover the positive sibling. Extra corruption may
fail earlier and must not be counted as evidence of the intended late predicate.

## 7. Remaining implementation and full-RP01 gaps

This draft and arithmetic checks are specified/checked expectations only. Still
required: closed bounded signing schema/codec and actual signature domains;
all38 operation/type/descriptor/effect fields and prices from the financial owner;
full source/Core preimages; independently implemented linked history validator;
funded genesis and complete positive/negative serialized fixtures; real signature
and display binding tests; evaluator/K correspondence; mandatory native claims
and ledger currentness; composition proofs and all challenge-map rows.
The complete RP01 map, eight intents, held-outs, five composition operators and
asset-study obligations remain open. No operation layout, current schema version
or old archived fixture is accepted merely by this draft.
