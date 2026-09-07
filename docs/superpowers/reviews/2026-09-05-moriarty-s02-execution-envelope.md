# S02 common execution-envelope review

Source candidate `46fe589`, evidence `0d5926b`, base `d7bf934`.
Independent reviewer: `/root/quint_policy_review`, native GPT-6 Astra, high
reasoning. Adversarial tests were authored separately by
`/root/execution_adversarial_tests`; that author is not this reviewer.
Neither activity is the required three-vendor Council acceptance gate.

## Source verdict

Clean for the declared envelope scope; no new findings. Commit rechecks the
verified operation against the actual current context. Retained signed parents,
residual guards, and revisions restrict consumed-authority reuse. Money,
authority consumption, parent accounting, candidate state, and attempt status
update in one guarded state assignment. Per-operation records retain full
prepared context and evidence bindings.

The separate test author found that effects-free cancellation could admit an
invented accepted Core projection. The controller preserved the failing test at
`523ee14` and corrected the guard at `46fe589`. Cancellation now requires
`NoCoreProjection`; other operations retain their existing projection rules.

The reviewer ran a focused TypeScript expression: first-fill verification
accepted its constructed parent fixture, and output-context validation returned
true for constructed first-fill, initial-cancellation, and second-fill commits.
These are helper-level experiments, not complete parent lifecycle traces.

## Evidence verdict

Clean. All forty-seven manifest pins match; pinned Quint sources match `46fe589`.
Raw receipts support eight pipeline tests, forty-two adversarial tests, and one
thousand sampled traces, with 494 after-resolution and 506 before-resolution
completions. Foundation, Python, and local S01 regression counts also match.
No routine suite was rerun by the reviewer.

The controller initially guessed the test author's RED command. The exact
command was corrected from the complete report, with the reporting error
disclosed and history preserved. The author's initial standalone typecheck lost
its terminal result; no success is claimed for it. The final scoped Rust test
run typechecked its imports and passed. Controller typechecking is separately
recorded. Neither reporting limitation is hidden by a green status.

## Limits and next work

The sampled harness begins with prefunded swap escrow but unsigned policies and
executes actual symbolic check/sign/propose/verify/commit transitions. It does
not execute funding or a candidate A–D interpreter. `EvidenceValid` is a trusted
external-verifier result, not a proof of cryptographic correctness or actual
candidate effect derivation. Candidate adapters must establish that separately.

Generic parent commit helpers exist, but stateful first/second-fill paths,
competing cancellation attempts, classified rejection, and recovery traces are
still required. Constructed parent fixtures do not satisfy those obligations.
Enabledness and completion evidence currently covers the prefunded swap
settlement harness only. No Core correspondence, model-checked A–D selection,
Council gate, S02 gate, or XML release gate follows from this unit.
