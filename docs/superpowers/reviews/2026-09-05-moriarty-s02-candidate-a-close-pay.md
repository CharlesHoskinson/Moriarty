# Candidate A finite types and Close/Pay review

Date: 2026-09-05
Scope: S02 Candidate A plan Tasks 1–2 and independent frozen Python reference vectors.
Status: locally tested and source reviewed; experimental branch only.

## Review outcome

Repository observation: the nonauthor native reviewers `s02_recovery_review`
and `quint_policy_review` reported no actionable findings. The root also read
all four new Quint files and the new Python test file. These are scoped native
reviews, not the requested three-provider Council gate.

The finite carriers constrain syntax, ordered cases, state maps, numeric values,
and strictly decreasing node ranks. The literal canonical swap table matches
the unchanged Python reference. Close refunds the first positive account in
canonical order. Pay preserves exact paid quantity, warning, continuation,
state, and reduction count semantics. Unsupported If/When branches remain
explicitly unavailable. Malformed-domain calls remain distinct from Core errors.

Experiment observation: root reran the 26 scoped Quint tests with Rust seed 42,
exit 0. Root reran the full Python suite: 305 tests, exit 0. The 19 new Python
vectors compare complete frozen-reference records, including warning rollback,
first-accepting ordered cases, missing choice versus stored zero, the deadline
commit/reject pair, and the pre-input payment prefix needed for deposit effect
ordering. They import no Candidate A evaluator, codec, or projection.
Terminal outputs are preserved in the adjacent unit evidence directory.

## Deliberate limitations and next obligations

- No If/When evaluator, input application, or transaction interpreter yet.
- No Quint/Python correspondence, exhaustive model check, or authority integration.
- The Python vectors are selected reference regressions, not the complete planned
  inventory and not a behavioral RED for the new Quint interpreter.
- The development-only diagnostics are not Core errors or semantic outcomes;
  Task 3 must eliminate unavailable evaluation for every admitted constructor.
- Candidate A, alternatives B–D, S02 acceptance, required Council reviews,
  and all downstream XML release gates remain open.

Evidence: `evidence/s02-model-comparison/candidate-a-close-pay/manifest.json`.
