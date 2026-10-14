# Deterministic financial helper work — proposed correction04

This is logical semantic work, not CPU time, proof rows, gas or a measured runtime
limit. It replaces the earlier undefined helper-cost term. Work is computed by
executing the fixed procedures below, never accepted from an operand, proof
sidecar total or caller annotation. The source/profile commits to this table.
All participating values already have finite type/shape bounds.

The action begins with the selected instance's authenticated kernel-envelope
Ordinary or Recovery work account, as derived below. Each expression/financial operation/helper entry debits its specified cost
before doing that step; insufficient work returns WORK_EXHAUSTED immediately and
discards the whole candidate. Final stored envelope work is initial minus actual charged
steps. Business Instance has no work fields. Select the exact account through profile.resourceAccounts[C.action] before
body evaluation. Every actual charge debits that account; the other stays unchanged.
Classification does not grant action/mode eligibility or a fresh allocation. No failed preparation replenishes either budget.

## Primitive schedule

| Primitive | Exact charge and behavior |
| --- | --- |
| Enter a Core node |1, once; Boolean /1 evaluates only selected children, but static maximum counts all nodes. |
| Enter one financial operation |1 before identity lookup. |
| Read/write a known scalar record field |1 per actual field read or tentative field assignment; unchanged framed fields are not rewritten. |
| Destruct/reconstruct a typed numeric/Variant/Option value |1 per layer visited/constructed; mismatched variant or absent required Some rejects at that visit. |
| Compare scalar integers, test enum/tag, checked add/subtract/multiply/divide, bounds test |1 per explicitly performed comparison/arithmetic/check; intermediate arithmetic is bounded512 bits, result must fit its declared type. Primitive mathematical input bounds make this finite; it is not a hidden arbitrary-precision or product-rule callback. |
| Compare two identifier/hash/key byte strings |1 + byteLength(left)+byteLength(right), always scan the complete bounded strings even if a mismatch is early. No locale/case folding. |
| Visit an entry of an explicit collection loop |1 per entry, plus all called primitives below. The loop's list and order must be specified; no uncharged filter/search. |
| Append a scalar/reference/list element |1 for append after checking capacity; constructing/copying a compound additionally charges one per value node copied. Shared host pointers do not avoid semantic copy cost. |
| Canonical encode a bounded value |1 + mathematicalValueNodes + canonicalUTF8Bytes. This is the exact candidate02/extension encoding; bounds checked before use. |
| SHA256 of an already encoded bounded byte string |1 + input byte length. This defines logical cost for this fixed hash primitive, not cryptographic proof complexity. |
| Return success/reject from a helper |No additional charge; all actual entered/visited steps remain counted. |

Exact canonical input bytes and typed node counts are derived, not caller fields.
Rule-definition/Core schema admission is bounded static validation and has no
semantic ordinary charge; conservative static admission still includes all
possible action/rule/helper work described below. No expensive proof work is
justified by that static distinction.

## Lookup and set discipline

Every financial table is in canonical ascending composite-key order with no
duplicate key. Lookup does **one complete left-to-right scan**, charges loop visit
and full key comparison for every entry, and records at most one match. Missing
required or duplicate match rejects after that complete scan. Thus lookup cost
cannot be reduced by moving an entry earlier. Optional missing balance lookup
returns typed zero only where the consumer explicitly permits creation.

Insertion performs the same complete scan, rejects an existing key for a fresh
identity, checks capacity, constructs the new record and appends/reorders by a
stable insertion pass: scan old entries once, compare new key with each, append
each old record exactly once and new record at its position. Each visit, compare
and copied value node is charged. Update uses one lookup and writes only named
changed fields. No host map cache changes these logical costs. A profile may
later prove an equivalent cheaper schedule only through explicit versioned review.

## Fixed helper algorithms

`available(owner,resource)` scans balances for the exact key, then scans all
encumbrances and live action-local release tickets. For each matching owner/
resource, checked-add reserved quantity; verify reserved<=owned, then subtract.
Pledge and settlement hold count once each; freeze is policy, not another reserve.
A source-specific release is consumed by the exact permitted leg and remains
unavailable to other legs until then. All scans and arithmetic charge by table.

`move` performs, in order: field/quantity/holder extraction; positive/distinct-
party tests; required authority/policy lookups and finite rule evaluation;
available/reservation checks; source/destination balance lookups and checked
sub/add; matching counter lookup; checked prospective gross/fee/eligible counters
and cap comparisons; then writes balances/counters/reservations, constructs receipt
and movement/delta records. Zero legs are omitted by the calling consumer after
one zero comparison, not passed to move. Per-leg funding then cap order is fixed;
a later leg does not reorder an earlier failure. All action changes still rollback.

`allocate` scans uses in literal order, checks exact purpose/class, positive
quantity and duplicate receipt references through a scan of preceding uses.
For each use it scans the receipt table, validates origin/source/recipient/resource,
checked-adds allocated quantity and rejects exceeding receipt quantity. Accumulate
and finally compare the exact required total; only then install the tentative
allocations. A caller classification cannot override the consumer's required
purpose or a receipt's class. Maximum uses16, receipts at most128; both loop
bounds and their nested product contribute to static work.

`claimContinuity` scans every explicit map entry and then every affected old claim
in sorted ID order. Reconstruct allocations per old claim with checked sum; require
exact remaining quantity, matching units/priority/beneficiary unless scoped change
is specifically granted, and complete successor/discharge/payment disposition.
Scan all backing references to reject double reservation. Every new claim must
have a fresh ID, named obligor and explicit source rule, not just a result flag.
The operation supplies the finite affected-old-claim set from its fixed footprint;
a caller cannot omit one to lower cost. Whole final-state validation additionally
checks every claim/reference and reservation, independent of supplied map entries.

`finishOperationEffect` builds deltas from first-touched fields in their observed
order. It checks that every changed state object is either named by the consumer
or rejects frame mismatch. It canonicalizes/hashes the actual argument record and
changed before/after records, retains their preimages in complete pre/post state,
and appends one typed effect. No state change can exist only in a caller digest.

`validateFinalState` scans all fixed state collections and all nested bounded
fields in schema order, checking the full financial refinements and referential
integrity. Every named-reference resolution uses the lookup above. It then checks
all release tickets and declared Mint→DebtCreate links consumed, all required
receipt allocations complete and all
claim/duty invariants. These traversals and helper calls are charged. Final Ensure expressions then
execute in source order on the complete merged ordinary/financial post; their
rejection still discards every change. Finally canonicalize the stable business post/effects
and use the exact accounting epilogue below. Ensure has no type/selector path to
that final resource envelope. Charge that traversal too before returning the result. This ordering agrees with rule-binding-and-admission.md;
work exhaustion at any step discards the candidate.

## Conservative bound and reconstruction

The conservative financial bound is **input-dependent**, reconstructed after the
bounded body has produced descriptors but before any financial mutation. Its
inputs are exact committed pre-state/argument/observation sizes, complete rule
syntax and the finite descriptor list. For each table lookup use its pre-state
length plus every possible insertion by preceding descriptors, capped by the
registered capacity. Include every potential movement/claim/receipt leg and full
rule syntax even when a Boolean branch may be skipped. This is a derivation from
known input shape and fixed consumer footprints, never a caller work count.

Canonical-byte bounds use exact unchanged input subtrees plus per-type maximum
encoded sizes for fields each consumer may modify, and the complete declared
shape of possible new records. Numeric maximum length follows its fixed bit
width; ID/text copies use their actual already-bound source values. If a rule can
produce a Text/collection whose size is not statically exact, use its type's full
bound. There is no optimistic post-state size assumption. This yields a finite
upper bound even when an eventual guard rejects. Include every hash/encode/lookup
and final-state traversal scheduled above.

The selected original ordinary or recovery account maximum65,536 still applies. If this reconstructed upper bound exceeds
remaining work, reject WORK_BOUND before financial reduction. A profile with
large legal storage capacity may still accept a small concrete state/action;
blindly charging every empty table at global capacity would otherwise reject
all profiles before they could operate. Large concrete instances may reject;
they do not gain a budget reset. Static source/rule admission separately bounds
syntax and every possible growth, and no native/proof resource admission follows
from these logical bounds. This is a proposed extension to the cost contract
that requires evaluator/K correspondence and independent bound tests.

For selected concrete inputs, actual work is the sum of the charged primitives
above. Every loop visit decreases a finite remaining-list measure or enters a
bounded child; every successful primitive consumes positive work except return.
No source recursion or unspecified convergence loop exists. The accepted compiler
must produce the same trace/count from the same source/Core/state; an independent
checker/K relation must reconstruct it. This document and a host-reported total
are not that proof. Source tests must include omitted helper visits, changed
receipt class/purpose, repeated lookup entries, skipped Boolean operands and
state-size boundaries, alongside meaningful full positive histories.

## Correction04 finite record construction schedule

Each constructor/update role in field-generation-04.json runs its fields in
lexical schema order. A typed-field source performs its stated record field read
and a typed value copy. A literal performs a typed reconstruction. A fixed equation
performs its written scalar operations left-to-right, range checks immediately
after each written checked operation, and the named helper calls below; repeated
operands are read again unless an explicitly named local intermediate retains
them. Every new field assignment costs1; unchanged update fields are framed and
cost no write/copy. A whole record copy is not a zero-cost host alias.

- quantity/resource/holder extraction scans variant tags in canonical tag order,
  charging one tag test per visited alternative through the selected tag. It then
  charges one payload projection, one scalar extraction for units, and one typed
  static resource/holder reconstruction when that result is requested. It is a
  closed finite kernel destructor, not a new user numeric cast. inverse quantity
  checks the exact registered resource/holder, scans tags likewise, bounds-checks
  n, constructs Amount/Shares once and boxes the variant once.
- MovementPolicy family selection scans all policies, then all movementBindings
  in the selected policy, then that rule family's operation entries, with full
  key comparisons; a missing/duplicate row rejects. CurrentMovement construction
  uses its eight actual fields, charged once each. Repeat on every nonzero leg.
  Do not cache a previous quote/input shape or omit repeated ruleUses entries.
- Release scans the actual lock, linked claim and, for Debt origin, linked debt;
  SettlementHold constructs its specified0 values. Pledge additionally scans the
  bound observation and checks every ReleaseValuation field against actual lock/
  remaining quantity/liability units before exposing valueAfter. Construct the
  seven CurrentRelease fields and run the actual family entry.
- Reward enrollment scans the selected vault's complete rewardDefinitions and
  rewards table for each actually changed share owner in source-then-recipient
  order. Compare exact holder/vault/id. Check endpoint on each existing match;
  initialize each missing listed record with G8 and its Duty. Construct/update
  their effects exactly as any other changed record. No promise is created for
  unmatched holders. Rebase also checks its exact accrual endpoint.
- Claim handling computes hasFuture by its single closed origin dispatch and
  required full owner lookup. It scans the complete encumbrance table for each
  source claim. Every referenced owner/backing update and Conditional zero carry
  is visited and charged. Zero currently owed does not skip a future obligation.
- Genesis scans the immutable registry and exact input list, compares every input
  identity with all preceding identities to reject duplicates, and for each
  distinct owner/resource scans the complete input list to sum units. Registry
  copying, balance/supply insertion, named reward enrollment, exact seed counter
  construction and all capacities are charged. This deliberately uses bounded
  scans, not unspecified hash-map performance.
- Migrate visits every value node in schema order. Each ResourceId/index dispatch
  scans the complete alias list; each claim-reference field scans the complete
  claim map. It reconstructs all changed typed payloads, sorts with the stable
  insertion pass already specified, and validates the whole destination state.
  Hash/history origin fields are copied unchanged, never interpreted as names.

Each equation helper's finite lists are the typed input/current state lists or
fixed field/variant rosters. No caller supplies loop counts or generated records.
Static bounds include all potentially visited entries and inserted-record sizes;
actual charge follows the executed visits, including selected Core nodes.

## Exact accounting epilogue, without a fixed point

After Ensure, the business post/effects are immutable. Derive an output template
from those exact values, the predecessor, and the actual sorted resource-envelope
instance IDs/unchanged entries. For the selected resource entry's two final work
values and FinancialPrepared's two matching remaining values, use UInt64.max as
the template value. This template is only for this explicitly fixed logical
accounting charge and conservative final capacity check. It does not replace
actual selected-node charges or change the canonical wire encoding. Each such
slot therefore contributes exactly20 decimal bytes regardless of its eventual
value; every non-work field contributes its actual bytes/nodes.

Let B,N be that template's exact canonical byte/node counts and I the number of
envelope entries. Charge E=12+(1+N+B)+(1+B)+I. The terms are twelve fixed scalar
reads/checks/subtractions/assignments, one canonical encode, one SHA256-length
charge, and one resource entry visit per entry. This is the complete primitive
finishAccounting, replacing any recursive attempt to charge for its own changing
output digit lengths. It makes no host-time or proof-row assertion. The twelve
steps are: read ordinary allocation; read ordinary used; test used<=allocation;
subtract; read recovery allocation; read recovery used; test used<=allocation;
subtract; test ordinary result bound; test recovery result bound; assign selected
ordinary; assign selected recovery. E itself is debited to the already selected
ordinary/recovery execution account before those subtractions. Bound computation
uses checked UInt64 arithmetic; overflow or insufficient account rejects.

The retained actual result contains the computed numbers, not UInt64.max. The
maximum-width template is a deterministic exact charge by definition for this
epilogue, and a conservative capacity test; actual output bytes cannot exceed it.
No financial rule/Ensure can inspect its pending values because the G10 type
boundary excludes the envelope. All body/rule/Ensure Core nodes retain actual /1
entry charging. No later resource deduction is allowed after this primitive;
ledger/proof correspondence checks the resulting exact envelope independently.
A backend that uses a different allocation account or count must reject rather
than report a forged matching business post as a valid full transition.
