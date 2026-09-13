# AFK handover, 2026-09-12 evening

Written by the orchestrating session while the owner is away, under endstop contract
`moriarty-ll-loop-20260912`. The owner asked to finish all six roadmap sections. This
document states honestly which of those can be finished unattended and which cannot.

## The hard stop: the time budget is exhausted

`design-review/admission-accounting-recon.json` reconstructs the resource position and
returns verdict `INSUFFICIENT_FOR_CURRENT_RUNTIME_ACCOUNTING_OR_DISPATCH` with
`runtimeApproval: false`. The master time budget reads:

| Field | Value |
| --- | --- |
| masterUsedSeconds | 210883.05 |
| masterRemainingSeconds | 6.95 |

Just under seven seconds remain. Its `missingEvidence` list states plainly that the
existing host-check charge "already claimed and cannot fund" a single-case reproduction,
and that "Root admission and exact resource amendment for parse04 and macro05 separately"
are required. `financialTotals` reports `globallyComplete: false` and
`paidFeesEstablished: false` across 13 unique allocations and 31 reserved submissions.

No amount of unattended work changes this. A new grant or amendment is the owner's
decision, it needs bounded limits and substantive votes per the package design, and the
design forbids creating accounting from zero or resetting counters. This session did not
and will not fabricate it.

## Therefore

Sections 4, 5.4 through 5.8, and 6 cannot be completed unattended. Each one terminates in
an action that requires owner authority: root admission for K, a funded resource
amendment, an admitted Docker lifetime contract, or a funded Preview transaction that
spends real value on a live network. Section 6.3 closes only after the whole retained
roadmap passes, so it inherits every one of those blocks.

## What was actually closed

Section 0 is closed against a receipt at `scope-reconciliation/receipt-01.json` and `.md`.
All four items were already satisfied in substance; the checkboxes were stale. Evidence:
base main `cd240b8` is an ancestor of head, `ROADMAP.md` records PR1 through PR7 with
result and audit links and names the retained wider gates, `openspec validate --all
--strict` reports 13 passed, and the current plan hash
`3e2895245bcf97729e96f57ad4294b725ebc067cec5096f3d25deb7e1cdfdcd1` carries an independent
review with verdict ACCEPT. Sections 1 through 3 were already merged before this session.

## What is in flight

Three lanes, each in its own worktree, none of them yet verified or accepted:

1. GPT-5.6 Sol on task 5.2, the exit-retaining executor caller, static prover-lifetime
   helper and offline fault matrix. Six files produced so far.
2. An Opus lane on task 5.1, freezing the source, profile, head, authority, state, time,
   failure and fee constraints with real hashes, recording genuine open items rather than
   inventing values.
3. GPT-5.6 Sol on the grounded change proposal that obligation 127 requires as its first
   leg. It is forbidden from running any K binary, because running the frozen case is
   exactly what the obligation prohibits. It produces a proposal for the owner to decide
   on, not a decision.

## What the owner must decide on return

1. Whether to grant a bounded resource amendment, and on what limits. Nothing downstream
   of 5.4 can proceed without it.
2. Whether to accept the grounded K proposal, once lane 3 reports, and whether to grant
   root admission for a single bounded reproduction.
3. Whether the `kore-expand-macros` failure should be accepted as the already documented
   upstream exclusion CLM-0524 rather than treated as a novel defect to repair.
4. Whether to authorise a funded Preview transaction, which is irreversible spending.

## What this session will not do

It will not create accounting from zero, reset a counter, grant itself admission, spend
funds, start an admitted Docker lifetime, tick a checkbox without a receipt, weaken a test,
or rebuild a retained artifact to make a gated suite pass.

---

## Update: task 5.1a is now blocked on a design decision

The constraint freeze for task 5.1 completed. It pinned 118 constraints against real hashes
and recorded 29 as explicit open items, with `complete: false` in the record. It verified
31 path-and-hash pairs with zero mismatches, and it cross-checked two declared hashes
against the real files rather than trusting them. That is good, honest work.

It also raised eight items for reviewer attention, four at high severity. Two of those mean
task 5.1a, "bind those reviewed constraints in the actual lowerer, custody and SDK caller",
cannot be executed as written:

- **There is no slot to bind authority into.** `lenderCapability` is written in the
  constructor and never asserted by any transition circuit, yet the lifecycle's originate
  step is a lender-to-debtor disbursement. The successor lifecycle state has exactly nine
  keys and none of them is a principal, authority, signature or nonce.
- **There is no fee operation to bind fees into.** The v5 profile's operations are exactly
  Transfer, Repay, Originate and Accrue. There is no Fee operation and no outcome-intent
  authority. The requirement that fees "must enter fee-inclusive net goals" has no
  implementable target in the source being lowered; that machinery exists only in the
  legacy evaluator.

Both would require extending the accepted source surface, which changes the language rather
than binding to it. That is an owner decision, not a lane decision, so this session did not
take it.

Two further high-severity items for the owner to read: `design.md` still names profile
version 4 and Core version 3 while the merged tree is at 5 and 4, so the design text was
never revised after origination and accrual landed; and allowances plus the four used-id
replay registers have no on-ledger counterpart, leaving replay defence resting on revision
and kernel cursor only.

A medium item matters for 5.2a specifically: two incompatible numeric universes exist, the
source sequence at rate one tenth and a fixture in USD micro at eight percent over 31/365,
with no chosen target before compilation.

The freeze is authored but NOT approved. Task 5.1 also requires an independent review, and
that review has not run, so 5.1 stays open.

## Revised statement of what is blocked on the owner

1. A bounded resource amendment. The master time budget has about seven seconds left.
2. Root admission for one bounded K reproduction, after reading the grounded proposal.
3. Whether to extend the source profile and lifecycle state with an authority slot and a
   fee operation, which unblocks 5.1a and therefore 5.2a, 5.3 and everything after.
4. Whether the `kore-expand-macros` failure is accepted as documented upstream exclusion
   CLM-0524 rather than a novel defect to repair.
5. Whether to authorise irreversible funded Preview execution.

## Status of task 5.2

An independent audit returned BLOCKED with eleven findings, two of them high and genuinely
wrong rather than stylistic: a caller-requested timeout kill was being reported as a
terminal financial failure, with a test asserting that wrong outcome as correct; and the
success gate was fed invented zeros for a stop, a timer and two persistence flags that were
never observed. A correction round is running. The audit also established that two of the
three scoped requirements are larger than one round, so they are being recorded as open
rather than claimed.

---

## Correction: parse04 and macro05 are two different crashes

The grounded K proposal is written, and it corrects a conflation that this orchestrating
session introduced. The correction matters, so it is recorded here rather than buried.

This session's earlier framing described the retained failure as "a segmentation fault in
`kore-expand-macros`, tracked as macro05 trace106", and told the owner that the crash under
diagnosis was already documented in the project wiki as a known upstream exclusion. That
was imprecise, and only half of it holds.

Against the retained evidence:

- **macro05's own segmentation fault is inside the LLVM interpreter, exit 113.** It is not
  in `kore-expand-macros`.
- **The `kore-expand-macros` segmentation fault, exit 139, belongs to the earlier parse04
  attempt**, which is a different attempt from the retained macro05 failure.

Claim CLM-0524 in `wiki/zkir/midnight-k-tooling.md`, with its primary source in the pinned
`midnightntwrk/k-rust` differential configuration, does corroborate parse04's
`kore-expand-macros` crash as at minimum a documented upstream exclusion class on the
identical pinned K build, version 7.1.337 at commit `4a46d123`. That corroboration does
**not** extend to macro05.

So the earlier suggestion that the owner might simply accept the retained failure as a known
upstream exclusion is not available on this evidence. It would be available for parse04. The
retained failure is macro05, and its cause is still undetermined between stack exhaustion
and semantic evaluation. Both the prior diagnosis and this proposal say the same thing: a
smallest-discriminator diagnostic is needed to tell those apart, and it has not been run.

The proposal was produced without executing any K binary and without running the retained
diagnostic script, which is what obligation 127 forbids. Twenty evidence hashes were
independently recomputed with zero mismatches. It offers four options, an advisory-only
recommendation, and six open questions, the first of which is who authorises the
discriminator diagnostic that nothing in the current budget can fund.

Decision 4 in the list above is therefore restated: the question is not whether to accept an
upstream exclusion, but whether to fund and admit one bounded discriminator run for macro05.

---

## The freeze was independently reviewed: four claims upheld, one alarm withdrawn

A cross-vendor review of the constraint freeze returned BLOCKED. Read that verdict carefully,
because it does not mean the freeze was wrong about the important things.

**Upheld, by the reviewer's own direct file reads rather than by trusting the author.** All
four high-severity findings stand:

- The lender capability is declared and initialised in `custody/loan.compact` but no exported
  circuit takes a lender secret or asserts against it. Only the borrower capability is
  checked. Confirmed at three circuit sites.
- The lifecycle state has exactly nine top-level keys and none is a principal, authority,
  signature or nonce.
- The lifecycle source declares exactly four operations, Transfer, Repay, Originate and
  Accrue. There is no fee operation and no outcome-intent authority.
- The compiled ledger declarations carry no allowance or used-identifier fields even though
  the source state carries allowances and four used-identifier arrays.

So the conclusion that task 5.1a cannot be executed as written is not a false alarm. It is
confirmed by two independent readings. Extending the source surface with an authority slot
and a fee operation remains an owner decision.

The reviewer also checked specifically for the dangerous direction of error, a value invented
or inferred rather than found, and reported none. Every defect it found was over-caution.
Hash integrity was independently recomputed: 205 occurrences across 31 unique files, zero
mismatches.

**Withdrawn.** This session earlier reported a risk that two incompatible numeric universes
were unresolved before compilation. The reviewer found that `design.md` line 52 already
selects the concrete lifecycle case, so that ambiguity claim is refuted and the warning is
withdrawn. Two further open items were also false: real Preview network and participant
identities do exist in the recorded `actual-run` evidence, and other Preview contract
addresses exist beyond the stale local one. Those were recorded as unavailable when the
repository determines them.

A bounded repair round is running to pin what is actually determinable, correct the false
reasons, and narrow two overstated wordings. The four confirmed findings are not being
retracted.

Task 5.1 therefore remains open, now for a precise and much smaller reason than before.

---

# FINAL STATE

The loop stopped here because the dispatchable work list is exhausted, not because of a
timeout or a failure. Everything remaining requires an owner decision.

## Roadmap position

| Section | State |
| --- | --- |
| 0 scope reconciliation | CLOSED against receipt `scope-reconciliation/receipt-01.json` |
| 1 financial postconditions | closed before this session, merged PR5 |
| 2 origination and accrual | closed before this session, merged PR6 |
| 3 complete lifecycle | closed before this session, merged PR7 |
| 5.1 constraint freeze | CLOSED against receipt `preview-constraints/closure-receipt-5.1.json` |
| 5.2 executor and fault matrix | partially delivered, audited APPROVED, two of three requirements open |
| 4 K agreement | proposal written, first of three gate legs only |
| 5.1a, 5.2a, 5.3, 5.4 to 5.8, 6 | blocked |

Two sections closed tonight. Neither was closed by asserting it; both carry a receipt with
recomputed hashes.

## Committed, on lane branches only

Nothing was merged into `feat/session-handoff-2026-09-12` and nothing was pushed. The 33
preserved working-tree changes are untouched. Every commit was checked for attribution
trailers and none exist on any ref.

- `cff5f1e3` executor caller, prover-lifetime model, fault matrix, both audit rounds
- `2122cf35` the constraint freeze with both review rounds
- `b12b3943` the grounded K proposal

## What two independent readings established

Task 5.1a cannot be executed as written. There is no source-level slot to bind authority
into and no fee operation to bind fees into. This is not one model being cautious; the
freeze author found it and a separate cross-vendor reviewer confirmed it by reading the
files itself. Fixing either changes the accepted source surface.

## Two corrections this session made to its own earlier claims

1. It described the retained K failure as a `kore-expand-macros` crash already covered by a
   documented upstream exclusion. Wrong. macro05's fault is in the LLVM interpreter at exit
   113; the `kore-expand-macros` fault at exit 139 belongs to the earlier parse04 attempt,
   and the exclusion covers parse04 only. Accepting the retained failure as a known upstream
   issue is not available on this evidence.
2. It reported a risk that two incompatible numeric universes were unresolved before
   compilation. Refuted: the design already selects the concrete case. Warning withdrawn.

## The five decisions

1. A bounded resource amendment. The master budget reads about seven seconds remaining
   against roughly 58 hours used, with fees not established. Nothing downstream of 5.4
   proceeds without it, and creating accounting from zero is forbidden.
2. Whether to extend the source profile and lifecycle state with an authority slot and a fee
   operation. This unblocks 5.1a and therefore 5.2a, 5.3 and everything after.
3. Whether to fund and admit one bounded smallest-discriminator run for macro05, to separate
   stack exhaustion from semantic evaluation. Neither the prior diagnosis nor the new
   proposal can settle that without it.
4. Whether to accept the six low audit findings on the executor work as written or fix them
   first. The one worth fixing is that an unconditionally false terminal-evidence flag makes
   every clean run report a write failure for a write never attempted.
5. Whether to authorise irreversible funded Preview execution.

## What was never done

No accounting created or reset. No admission granted. No funds spent. No Docker lifetime
started. No network action. No checkbox ticked without a receipt. No test weakened. No
retained artifact rebuilt. No attribution trailer written.
