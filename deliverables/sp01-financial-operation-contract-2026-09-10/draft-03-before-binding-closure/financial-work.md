# Deterministic financial helper work — proposed draft03

This is logical semantic work, not CPU time, proof rows, gas or a measured runtime
limit. It replaces the earlier undefined helper-cost term. Work is computed by
executing the fixed procedures below, never accepted from an operand, proof
sidecar total or caller annotation. The source/profile commits to this table.
All participating values already have finite type/shape bounds.

The action begins with the selected instance's authenticated remaining ordinary
work. Each expression/financial operation/helper entry debits its specified cost
before doing that step; insufficient work returns WORK_EXHAUSTED immediately and
discards the whole candidate. Final stored work is initial minus actual charged
steps. Recovery work is unchanged unless a separately specified recovery action
explicitly spends it. No failed preparation replenishes either budget.

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
all release tickets consumed, all required receipt allocations complete and all
claim/duty invariants. Canonicalize complete post and effects and check independent
byte/node/depth bounds. These traversals and helper calls are charged. Final Ensure
expressions execute on the complete merged ordinary/financial post; their rejection
still discards every change. Bound-check/effect-finalization ordering must match
this schedule in the evaluator/K implementation.

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

Ordinary maximum65,536 still applies. If this reconstructed upper bound exceeds
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
