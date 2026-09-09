# RP01 draft-02 reviewer disposition

This is the author's response to the retained Opus CHANGES_REQUESTED review, not an independent approval. The raw Opus result and existing `gpt6-review.json` / `gpt6-review.md` are historical reviews of draft-01 only. They do not certify the changed candidate. Fresh independent review is required. No RP01 gate, language profile or SP05 candidate is frozen or accepted by this revision.

Before edits, the three reviewed files were copied byte-for-byte to `draft-01/`; their hashes still match the raw Opus candidate pins. The initial financial state, every primary step (including exact before/after records and effects), policy, resource bounds and all nine source records remain structurally identical. No economic amount or authorization has been added.

## Findings and decisions

**F1 — disagree with the proposed subtraction; clarify and test the actual representation.** `work.remaining` is ordinary work only. The separate `closureReserve` is additive, so the original budget is 12 ordinary + 2 reserve = 14 total. Requiring remaining >= reserve or subtracting reserve from remaining would implement a different budget convention. Before changing the checker, the same ten events with initialOrdinaryWork 10 and every supplied ordinary remaining value reduced by two passed with ordinary remaining 0 and closure reserve 2. That is preservation of the separate reserve, not spending it. This is now an executable positive boundary control. The ordinary-shortfall negative control still rejects the first two-event step when ordinary remaining is one despite a separate reserve of two. DERIVATION now states these quantities explicitly. No change to the arithmetic work gate is warranted on the supplied representation; independent reviewers should assess this factual disagreement.

**F2 — accept the scope ambiguity; defer complete funded closure as explicit required work.** This fixed three-step partial case does not demonstrate the full funded-zero boundary. Renamed schema/status and a distinct PARTIAL_RECOVERY_SCOPE rejection prevent its positive-debt domain from being misrepresented as a general rule forbidding complete repayment. A scope control exercises the retained zero-duty/Discharged representation as excluded here; it is not a purported funded execution. New task RP01-LOSS-FULL-RECOVERY remains required: establish actual additional 450 cash and fresh bounded authority, retain a discharged zero-duty record, reconcile allowance and loss history, and distinguish funded closure from erasure. No such additional funding exists in these facts: Borrower ends with 45 and its debit grant is exhausted. Owners are SP01 policy, SP02 representation, SP03 executable transition, SP07 source-policy binding and SP09 proof obligations. This disposition does not narrow the full RP01 roadmap or claim to close the reviewer's lifecycle concern.

**F3 — accept and add direct discriminating controls.** Added direct NO_DEBT_ERASURE, LOSS_CONSERVATION, SOURCE_PIN, PRE_STATE, IMPAIRMENT_ACCOUNT, SHARE_SUPPLY, RETAIN_RECORDS, CLAIM_LOSS_ALLOCATION, SCOPE and VERIFICATION_WORK corruptions. Two independent boundary-total corruptions exercise the same ASSET_CONSERVATION helper called on calculated transitions. They distinguish conservation from exact post-state matching. The harness checks input immutability after success and every rejection, including these boundary controls. DERIVATION identifies checks without dedicated direct controls; this is not exhaustive branch coverage.

**F4 — accept the reproduced authority defect and scope correction.** Before the fix, changing RecoveryGrant.asset to Collateral in the initial state and all six before/after snapshots was accepted; likewise changing its recipient to FeeCollector was accepted. These were internally consistent snapshot mutations, so exact state comparison did not catch them. New regressions reproduced the failure before implementation (`MUTATION_repayment-grant-wrong-asset: ACCEPTED`). Repay now requires the actual grant denomination/payee/duty to match the duty, and matches the executed funding transfer against that grant's denomination/payee. Both mutations now reject AUTHORITY. Renamed schema/status separately disclose the checker is a single illustrative fixed partial case under chosen generic policy, not parameterized protocol coverage.

**F5 — accept.** The table now says “Current net loss allocation.” allocatedNetLoss is net of recovery and can decrease; impairmentExpense, feeExpense and recoveryGain retain the separate cumulative event quantities. Full snapshots retain the allocation sequence. No historical loss is silently removed.

**F6 — accept.** Added the missing comma between omitted write footprint and observation capacity exhaustion.

**F7 — retain disclosure.** Read footprints include every pre-state leaf used by complete-state validation. They are not a data-dependent optimization or a parallel/reordering argument.

**F8 — retain explicit policy-review ownership.** Gross impairment 600 excludes Pool liquidation fee 10; pre-recovery loss therefore becomes 610. Recovery gain 150 reverses allowance while residual recourse remains Defaulted. These are consistent chosen assumptions, still requiring SP01/SP07 policy review, not accounting or protocol authority established by this checker.

## Verification

Run from the SP05 worktree root:

```sh
node deliverables/rp01-loss-allocation-2026-09-09/check-case.mjs
```

Observed PASS: three primary steps; 34 rejected mutations; two direct conservation controls; one positive ordinary-work boundary; one explicit domain exclusion; input immutability checked on successful validation and every rejection. Primary residuals remain funded recovery 550, borrower duty 450, impairment allowance 450, current net loss 460, Pool cash 640, ordinary remaining 2 and separate reserve 2. Logical verification work is 35/64. The checker verifies the nine pinned local sources. No network, compiler, proof, wallet or generated-language execution was used.

The archive preserves original bytes, including original relative source lookup. To execute an archived checker, restore its original directory context in a separate scratch copy; moving it one level deeper changes its relative ROOT. The archive is not an independently runnable new candidate.

## File pins

### Historical draft-01 (reviewed)

| File | SHA-256 |
| --- | --- |
| `case.json` | `97e4cbf714e8490c0a26b7bd1badf392f8b2246c4c2be530349758343b7287f2` |
| `DERIVATION.md` | `231943d241ec7813849ed463e0251eb980860d8d0a3ee1a8ceeb3552268ab35c` |
| `check-case.mjs` | `ed0cb754480763af4283f0fef76d2d28b1811086d71ade0b4486e0f956f75b69` |

### Changed candidate (requires fresh review)

| File | SHA-256 |
| --- | --- |
| `case.json` | `74e3a5506a775c260ed5471a273343bb0a8b840c3f628a7a83b4ca818a2edd93` |
| `DERIVATION.md` | `4a86e4ac3b0394f8746c5efa8cbedc4950181fbc5b8ed44abf5edda1b172db0a` |
| `check-case.mjs` | `38c985ce97cb53f4d86fbeb7148f7b6dde3e932b50ad05d98f79b0af169af16b` |

