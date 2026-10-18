# One-case K amendment feasibility

**Conditionally feasible as a newly reviewed, isolated resource grant using existing legacy reader fields. It is not a reconstructed global ledger, an adopted grant, or dispatch approval.** No runtime accounting or code was changed. The companion JSON specifies proposed fields and exact Decimal arithmetic; unresolved candidate/action/digest placeholders deliberately prevent treating it as an executable candidate.

The first reconstruction remains correct: historical rows plus a new file path cannot establish complete current master/package usage. A case projection is permissible only if new substantive authority expressly defines the independent grant, closes old credit, preserves unresolved liabilities and binds the one invocation. Reader shape acceptance alone cannot provide that authority. My provisional “old rows plus 60” formulation lacked explicit historical credit closure and is superseded by the closure 7/adjustment 1 construction below.

## Normative basis and its limit

`raw/assignments/moriarty-autonomous-execution-2026-09-07.md` expressly permits bounded successor allocations and direct supervision, retains historical limits/charges, and says exhaustion alone is not a reason to ask the user. `moriarty-twelve-sprint-afk-execution-2026-09-07.md` delegates design/resource choices to two substantive agreeing votes. Current user-selected identities replace stale historical routing, not the need for actual votes.

`openspec/changes/afk-live-financial-execution/specs/afk-live-accounting/spec.md:53–83` supplies an explicit conservative successor principle: unknown historical costs stay unknown in immutable dispositions; a two-vote amendment may close old residual credit with a reservation and create exclusively new bounded envelope credit; no lifetime-global total is claimed. It requires conservative closure 7 for the inspected fractional remainder and separately approved nonspendable adjustment 1. It also says: “If an unresolved old cost could still consume this active envelope, deny launch until explicitly dispositioned.”

That specification's authenticated producer and MC02 owner projection apply to financial settlement and are not an existing K implementation. This proposal does not invoke or relax them. A concrete K amendment must explicitly adopt the successor disposition under the delegated authority, using legacy MC01 reader semantics. SP03's actual entry gate allows MC01 among its owners, so preserving that owner is possible without relabeling charges. All source/stage/prerequisite/history gates still apply.

## Exact proposed resource disposition

Choose G=60 executable seconds: one krun with 20-second child timeout, zero compiles/retries; adapter and diagnostics have 5 seconds of additional headroom within a 25-second runner timeout; grace 5 and the existing 30-second startup/cleanup allowance make the advance charge 60. Freeze and review the actual adapter/containment/argv/artifact first. The adapter must retain raw exit 139/113 and return an exact agreed behavioral assertion code in 2–123 with committed stdout digest; it must not convert a crash to success exit 0. These numbers do not grant author/reviewer work outside that invocation.

Preserve the original master, all 405 debit rows, all 8 reserve rows, planning/overhead, package owner/limit and old dispatch counters. Append only:

- A 7-second `reserved` row with exact fields `id,seconds,basis,sourceSha256,amendmentSha256`, basis `historical-credit-closure-not-paid-cost`.
- One 60-second exact `charges` row bound to the new K action/candidate/runner digest.
- One new `successorEnvelopes` entry with allocation 60, remainingReservedSeconds 0 and no author dispatch grant. Preserve exhausted old envelopes.

The separately voted master increase is 61: G 60 plus D 1 nonspendable closure adjustment. Retain package_limit_seconds 117710; its known remainder already covers the prepaid 60 but contributes no authority to the new case. Resource amendment `additionalSeconds=60` gives the executable allocation; existing `resources` records closure 7, adjustment 1, and exact disposition/vote references. Avoid a second binding allocation that counts 60 again.

Using original JSON numeric tokens:

```
U =210883.0473305040214; M =210890
C =ceil(max(0,M-U)) =7
D =max(0,ceil(U+C)-M) =1
M' =M+D+G =210951
U' =U+C+60 =210950.0473305040214
master slack =0.9526694959786; package slack =6201.9526694959786
new grant 60 =exact prepaid charge 60 +case envelope 0
```

Neither C nor D measures unknown paid usage. They close represented credit without a rounding refund. Known later K allocations, 212 inspected sibling invocations, original 113/2512 commitments, 31 known financial reservations and all older unquantified histories remain preserved by exact references to the reconstruction and original records. They are not silently dropped from a claimed global sum: **no global sum is claimed**.

The new case-specific `currentAccounting` reference must include `passBinding`; that binding supplies the exact campaign scope, worktree, parentCandidate, selected new envelope and `resources.verificationCommands.reproduce=60`. The existing reader computes:

```
min(master slack, package slack, allocation 60, envelope 0) -prepaid 0 =0
```

Thus the action cannot use old master/package slack. A positive-cost unprepaid action sees zero envelope credit and denies. Exact existing debit validation requires one row matching charge/action/candidate/runner; SQLite's unique runner claim prevents reuse. Exhausted author dispatch counters remain unchanged because legacy `reproduce` is not an implementation kind. One-case execution capacity comes from the new bound and unique charge, not that exemption. No existing loan campaign may consume this case projection.

## Required amendment text and stop condition

The exact reviewed disposition must substantively say:

> This amendment grants a new isolated 60-second K reproduction allocation and separately approves 1 second of nonspendable closure adjustment. It reserves 7 seconds to close represented historical credit, without asserting payment. All original charges, limits, failures, consumed invocations, financial reservations and identified or unknown later liabilities remain preserved; this amendment neither retrospectively approves old use nor refunds, resets or extinguishes it. The new grant cannot fund old liabilities, and old residual credit cannot fund this case. The numeric projection covers the pinned retained baseline plus this amendment only and is not a lifetime-global usage statement. Exactly one frozen prepaid invocation may claim the grant; failure, nonlaunch or success grants no retry or refund. Old/global admission remains unavailable.

A reviewed disposition is required for unestablished original-limit ancestry and later direct amendments; do not invent retrospective majority votes. The active case authority must be independent of unresolved historical costs in fact, not merely in metadata. If the applicable authority still imposes a single global cap whose unknown historical use must subtract from G, the construction is **insufficient**: existing schema has no symbolic unknown-cost arithmetic and envelope 0 cannot cure a global-cap violation. Deny that interpretation until explicitly resolved; do not assign a number to the unknown.

No new permission question follows from stale caps alone. The next action is to freeze this concrete disposition with the exact one-case candidate and obtain the selected substantive votes. The present artifact is feasibility analysis, not one of those votes.

Concrete adoption requires independent validation of the exact projection, not only its old-reader shape: recompute C/D/G from original numeric tokens; compare all original debit/reservation rows byte-for-byte; verify the known/unknown history dispositions, substantive exact votes, current selected envelope and source/artifact/action/runner bindings; establish the one-use charge is unclaimed. The financial specification explicitly rejects omitted charges, guessed costs and missing exact projection even where the old reader accepts shape. Its principle constrains this proposed K adoption too: referenced later histories must be positively preserved and dispositioned, not silently omitted. If a reviewer cannot validate that narrower authority and conservation, deny adoption.
