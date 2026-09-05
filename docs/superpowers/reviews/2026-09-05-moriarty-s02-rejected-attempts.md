# S02 rejected-attempt unit review

Date: 2026-09-05 UTC. Classification: independent source review and local
experiment record. This is not a cross-provider Council gate or an S02 verdict.

## Source and authorship

Root implemented rejection in `execution.qnt` and authored `rejection_test.qnt`,
`rejection_harness.qnt`, and `rejection_pipeline_test.qnt`. The independent
non-author native GPT-6 Astra reviewer `/root/quint_policy_review` inspected
the source and a subsequent narrow correction. No reviewer source edits or
reviewer runtime tests are claimed.

The initial reviewed execution SHA-256 was
`f5edc49dcba6fa2027ded74eba70e0df4f0e512dfb6f35b55d44ca2d0e9632d7`.
The corrected reviewed execution SHA-256 is
`cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928`.
Harness SHA-256 `eee07af2fd3953b79e7a60ae7a7e385f4e3a408853afd9f986d33b58316096d7`
and pipeline-test SHA-256
`9c44e48e8de818fa6b27b3e6632c97d4fbbdace477c52676d835eb1c3b95d19f`
were unchanged through the correction review.

## Finding and disposition

Low-severity diagnostic finding: an accepted Core projection returned
`UnauthorizedEffect` before a consumed/used/cancelled parent could reach
`ConsumptionConflict`. This did not enable commitment or move funds.

The root-authored `acceptedCoreDoesNotHideConsumptionConflictTest` reproduced
the assertion failure against the initial source. Its exact command, terminal
exit 1, source pins, and output are preserved in
`evidence/s02-model-comparison/rejection/classifier-red.json`. Commit `1a23366`
preserves that source and failing regression. Commit `8d78662` preserves the
earlier disabled API scaffold and its eight failing assertions.

Correction: after recognizing a reported Core rejection, an admissible accepted
projection now reaches the consumption-conflict check. Malformed projections
and projections attached to lifecycle cancellation remain unauthorized.
Tests distinguish unavailable from invalid proof, known from unknown Core
error, and original proposals from substituted signature-proof attempts.

The reviewer returned a clean narrow re-review of the corrected source and
confirmed that the earlier finding was resolved. No other source findings
were reported. Runtime results are separate from this source verdict.

## Scope and limits

Rejected records retain the original attempt, supplied evidence, actual context
at rejection, derived reason, and verification/commit stage. Guarded rejection
changes only the attempt cell. Stage mismatch, absent/executed/rejected attempts,
and still-valid evidence cannot take the rejection transition. Competing stale
commit attempts can retain their own rejection without undoing the winner.

The rejection harness starts from a constructed signed swap context. It explores
missing-proof rejection or verification followed by an anchor change and stale
commit rejection. It does not itself execute signing, installment races, or
recovery. It uses explicit `rejectionInit` and `rejectionStep`; imported ordinary
settlement `init`/`step` are not its simulation entry points. Terminal rejection
is not financial completion, and the harness has no blanket stutter.

Evidence dispositions remain trusted external-verifier abstractions. A retained
`CoreRejected` records a reported known error; no independent Core execution,
cryptographic verification, candidate A–D semantics, exhaustive checking,
Council acceptance, or release claim follows from this unit.

## Final evidence review

Corrected source is committed at `38cf13df8fbc9cfce3dacad093dcc05215fc39f3`;
receipts and manifest are at `4f3bb75`. The independent reviewer checked all
25 manifest pins and confirmed that all 13 pinned Quint sources match that
source commit. Complete terminal receipts support 21 rejection tests, two
pipeline tests, and 1,000 sampled traces: 504 missing-proof and 496 stale-commit
rejections. Eight settlement regressions, 42 adversarial regressions, 286 Python
tests, ten local S01 checks, and standalone typecheck also have successful
terminal receipts. Both RED commits remain available.

Evidence verdict: clean, with no discrepancies. This was a read-only source and
receipt check; the reviewer did not repeat runtime tests. It closes this scoped
unit review only. Stateful installment races and signed recovery are separate
in-progress work, not part of this rejection manifest.
