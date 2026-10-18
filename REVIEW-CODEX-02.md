# Codex re-review: Preview constraint freeze, repair round 1

Reviewer: Codex GPT-5.6 Sol, high reasoning, read-only sandbox.
Worktree: `/home/charl/Moriarty-wt-moriarty-ll-loop-20260912-plan-preview-constraint-freeze`
Task: `openspec/changes/language-to-ledger-lifecycle/tasks.md` 5.1, repair round 1.
Prior review: `REVIEW-CODEX-01.json` / `REVIEW-CODEX-01.md`, verdict BLOCKED.
Repair account under test: `FOREMAN_REPORT.md`.

## Verdict: APPROVED

All five repair-round findings are genuinely cleared against primary sources,
not merely claimed: the network/identity, numeric-target, and
contract-address open items are now honestly pinned or honestly narrowed, and
the revision-assertion and principal-slot wordings are accurate rather than
merely softer. The four high-severity findings that were never in dispute
(unasserted `lenderCapability`, no source-level authority slot, no `Fee`
operation in the lowered profile, and allowances/replay registers with no
on-ledger counterpart) all still stand, worded consistently and unweakened.
No newly invented value, overstated qualifier, or open item closed by bare
assertion was found. The tree was not modified during this audit.

## Method

Codex GPT-5.6 Sol was run cold (no memory of the prior session) in a
read-only, ephemeral sandbox `cd`-ed into the worktree, and instructed to
recover the prior verdict from `REVIEW-CODEX-01.json`/`.md`, treat
`FOREMAN_REPORT.md` as an unverified claim, and independently re-derive every
answer from primary sources rather than trust the author's account. The
orchestrating agent additionally re-read `design.md` and
`loan-lifecycle.mori` directly, outside of Codex, to spot-check the
highest-risk item (the numeric-target pin) given the architect's explicit
warning that a wrong pin there would be worse than the original
over-caution.

## Prior findings and disposition

### 1. REVISION-ASSERTION (medium) -- cleared

`loan.compact:57-69` confirms `initialize` has no `expectedRevision`
parameter and no `REVISION_MISMATCH` assert; `:74` and `:101` contain the
assert for `accrue` and `settle`. `freeze-01.json:295` and
`freeze-01.md:98-102` now say "accrue and settle only" instead of "every
state-changing transition."

### 2. NETWORK-IDENTITY (high) -- cleared

The network tag `3c096de2...`, `firstAddress`, `secondAddress`, and the
bech32 wallet address are pinned at `freeze-01.json:255` and are genuinely
present in `plan.json`, `public-preflight.json`, and three `stage-deploy.json`
records. The role mapping is supported by `integrate-preview.mjs:103-112`
and the `loan.compact`/`swap.compact` constructor argument order. The
narrowed residual open item (`lifecycleRunReuse`) is honest: no seed,
wallet-state, or role-secret file exists in the checkout.

### 3. NUMERIC-TARGET (high) -- cleared

This item carried the highest risk, since a wrong pin here would be worse
than the withdrawn ambiguity claim it replaces. Independently re-read
`design.md` line 52 and `loan-lifecycle.mori` lines 1-177 directly, not only
via Codex. `design.md`'s stated case (lender 100, borrower reserve 10,
originate 100, accrue 10, repay 30 interest-first, computed settle 80, debt
sequence 0 to 100 to 110 to 80 to 0, final lender 110/borrower 0) is
reproduced exactly in the freeze pin at `freeze-01.json:1523-1556`. The
corroborating concrete fields not stated in `design.md` itself -- unit
`Cash`, numerator 1/denominator 10, `periodSeconds` 60, `firstPeriodStart`
1000, `allocationRule` `AccrualFirst`, `nominalLiabilityCap` 110 -- are
exactly what `loan-lifecycle.mori` lines 1-85 declare, and its `ensures`
clauses at lines 91, 109, 112, 140, 142, 170, and 172 confirm the outstanding
sequence 100, 110, 80, 0 and final balances 110/0. The pin correctly cites
`design.md` as `source` for the case-selection and `loan-lifecycle.mori`/
`loan-lifecycle.state.json` as `corroboration` for the concrete parameters,
rather than misattributing the concrete parameters to `design.md`. The
residual scale question is genuinely folded into
`state.fieldMapping.sourceToLedger` at `freeze-01.json:1656-1666` (Cash
quantum 1 vs `USD_TEST_ASSET`/`USD_micro`), not deleted.

### 4. CONTRACT-ADDRESS (medium) -- cleared

The corrected reason at `freeze-01.json:490` names real Preview and local
addresses (`ffedd46f...`, `323d43b9...`, `87affdd9...`, `36e4a923...`, and the
stale `ba4c8088...` in `stale-loan-plan.mjs`), each genuinely present in the
cited `stage-deploy.json` records, while the target lifecycle deployment
address stays open because no lifecycle contract is deployed anywhere in the
tree.

### 5. PRINCIPAL-SLOT (medium) -- cleared

`financial-lifecycle.ts` confirms `LifecycleObligation.principal`,
`accrued`, and `outstanding` are real nested fields and `OBLIGATION_KEYS`
lists them, while the nine top-level `STATE_KEYS` contain no `authority`,
`signature`, or `nonce` field (zero case-insensitive occurrences of those
three words in the whole file). The open item was renamed and narrowed
accordingly and no longer claims "no principal slot at all."

## Four high-severity findings -- confirmed still standing, unweakened

- **HIGH-LENDER-CAP**: `loan.compact:14,41,50` declares and initializes
  `lenderCapability`; exported `initialize`/`accrue`/`settle` at `:57`,
  `:71`, `:98` accept only `borrowerSecret`; no exported transition accepts
  `lenderSecret`.
- **HIGH-NO-AUTHORITY-SLOT**: `financial-lifecycle.ts`'s nine top-level
  fields contain no authority, signature, or nonce slot. This is the
  narrower, corrected version of the claim (excluding the refuted
  "no-principal-at-all" sub-claim) and is the version now carried in
  `freeze-01.json` and `reviewerAttention` RA-2.
- **HIGH-NO-FEE-OP**: `loan-lifecycle.mori:51-54` declares only `Transfer`,
  `Repay`, `Originate`, `Accrue`; no `Fee` operation exists in the successor
  profile being lowered, so the fee-inclusive net-goal rule cannot be
  expressed by it.
- **HIGH-ALLOWANCE-REPLAY**: `financial-lifecycle.ts` carries `allowances`
  and four used-identifier registers with no on-ledger counterpart in
  `loan.compact`. Both open items, `state.allowances.ledgerRepresentation`
  and `state.replayHistory.ledgerRepresentation`, remain present and
  unweakened.

## New findings

None. No newly invented or inferred value, no pin whose qualification
overstates its source, and no open item closed by assertion rather than
evidence was found. `openItems.items` in `freeze-01.json` is consistent with
the six per-family `open` arrays; the JSON and Markdown companion files do
not contradict each other on any of the six re-checked items.

## Verification

The tree was not mutated during this audit: a `find -newer` scan across the
whole worktree, anchored against `FOREMAN_REPORT.md`, returned only
`FOREMAN_REPORT.json`, whose timestamp (18:41) predates this audit run and
was already newer than `FOREMAN_REPORT.md` (18:40) before the audit started.
The Codex sandbox was `read-only`, `--ephemeral`, `--skip-git-repo-check`,
`--cd`-ed into the worktree, model `gpt-5.6-sol`, reasoning effort `high`.
The orchestrating agent additionally re-read `design.md` and
`loan-lifecycle.mori` directly (not via Codex) to confirm the numeric-target
pin independently, given its status as the highest-risk item.

## Note on the counts the orchestrator already verified

Not repeated here: 232 embedded path/hash occurrences across 37 unique
files, zero mismatches, as stated in the task brief. `complete` remains
`false`, `openItems.count` is 27, pinned count is 121. A `complete: false`
freeze with truthful open items is an acceptable deliverable for task 5.1;
approval here does not require every value to be pinned.
