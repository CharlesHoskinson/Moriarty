# Candidate A projection and extraction review

Date: 2026-09-05
Source: `d4a714ce6013e8237ff9ef07b6b661c52a421e8c`.
Scope: the neutral projection/extraction portion of Candidate A plan Task 5.
Status: locally tested and source reviewed; experimental branch only.

Repository observation: root and nonauthor native `s02_recovery_review`
inspected the new projection and tests. No actionable findings were reported.
Carrier translation preserves the full program-bound continuation, numeric time,
existing choice keys and NoInt/zero distinction, every result field, and exact
deposit/choice identities. It is deliberately nonvalidating: malformed maps and
arbitrary carrier fields are copied, not certified or normalized.

Extraction recomputes the actual transaction, requires complete supplied-result
equality including rejected cases, and only then returns effects. Actual
rejection has no committed effects. Actual acceptance preserves pre-input
payment debits, accepted deposit credit, then post-input payment debits.
Mismatch/domain/bound/precondition diagnostics never become Core outcomes.

Experiment observation: independent root execution passed all 26 projection
tests, exit 0, Rust seed 42. Author RED has 22 assertion failures and four
scaffold passes; GREEN has 26 passing and unchanged-source Core regression
has 75 passing. The author preserved both seven-file source closures and exact
chunked command receipts. Root's receipt is separate.

Tests distinguish deposit without payment, immediate refund, Pay1/deposit5/Pay5,
speculative rollback, timeout precedence, altered results, complete program
identity, and nonvalidating malformed carrier translation.

This review does not certify stateful workload sampling, independent
correspondence, authority integration, cryptography, Council acceptance, or
Candidate A/S02 completion. Re-evaluating the producer is not an independent
semantic oracle. The frozen-Python checker remains required.
Evidence: `evidence/s02-model-comparison/candidate-a-projection/manifest.json`.
