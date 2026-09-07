# Candidate A control-flow and ordered-input review

Date: 2026-09-05
Source: `d3f5dd6e60ca937dfc4840d34204bf00436bdc48`.
Scope: Candidate A plan Task 3 and independent installment reference vectors.
Status: locally tested and source reviewed; S3 experimental branch only.

Repository observation: nonauthor native `quint_policy_review` found no
actionable source findings. The root read the modified interpreter, types,
core tests, and separately authored boundary tests. The fixed 22-call fold
preserves exact output order/counts and distinguishes completion, domain failure,
and bound failure. If preserves absence versus stored zero; When uses the
actual modeled time including deadline equality. Ordered input scanning accepts
the first accepting case, allows later acceptance after a bounds mismatch, and
requires exact deposit identity and quantity before nonpositive-deposit errors.

The input wrapper is a restricted internal API for admitted supplied input on
an unexpired When. Its precondition/domain diagnostics are not Core errors.
Whole-transaction ordering and rollback are still Task 4.

Experiment observation: root reran 52 core tests, four independently authored
boundary tests, and 327 Python tests, all exit 0. Boundary coverage includes
21 actual reductions followed by terminal quiescence on call 22, maximum-deposit
potential preservation, clock retention, and ordered choice scanning. Root
caught a test-construction mistake: a quiescence expectation omitted the final
Close refund of 280 after Pay20. The test was corrected; no product change
was indicated. Its original RED test source was not archived, which the
boundary report explicitly discloses. The primary author's Task 3 RED closure
is archived in full.

The 22 new Python installment tests are independently specified two-When
workload regressions: separate five-unit fills, actual ten/five refunds,
deadline commit/reject pairs, and complete rollback for selected invalid
requests. Nonauthor review is clean. No candidate evaluator, codec, projection,
or authorization implementation is imported by those tests.

No Quint/Python correspondence, exhaustive model checking, candidate authority
integration, full Candidate A completion, Council approval, or S02 acceptance
follows from this unit. Required exact-provider reviews remain open.
Evidence: `evidence/s02-model-comparison/candidate-a-control-flow/manifest.json`.
