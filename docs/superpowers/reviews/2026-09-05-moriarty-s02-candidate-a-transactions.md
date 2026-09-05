# Candidate A transaction evaluation review

Date: 2026-09-05
Source: `ea35cada6bf84a016ec568834242628ee8ccfa76`.
Scope: Candidate A plan Task 4.
Status: locally tested and source reviewed; S3 experimental branch only.

Repository observation: independent native `s02_recovery_review` found no
actionable findings. Root inspected the full transaction delta and all 23 added
tests. The implementation preserves frozen source order: finite admission,
original-clock rejection, time update, pre-reduction, NoInput/When/Close
branches, ordered input application, full ORIGINAL rollback on Core error,
post-reduction, and ordered concatenation of outputs and actual reduction counts.

All six frozen Core errors remain distinct from finite-domain, reduction-bound,
and input-precondition diagnostics. Every Core rejection restores original
continuation, accounts, choices and minimum time, with empty payments/warnings
and zero reductions. Accepted deposit without a payment remains accepted;
effect extraction must still record that deposit in the next unit.

Experiment observation: root reran 75 core tests, four independent retained
boundary tests, and 327 Python tests, each exit 0. The 75 tests contain 52
retained tests and 23 new transaction tests. A 200-case admitted literal
program/time/input matrix requires genuine TransactionComputedA outcomes,
including legitimate rejected results, rather than tolerating diagnostics.

Tests include speculative warning/payment rollback, no-input rollback after
pre-reduction, deadline NoInput commitment versus supplied-input rollback,
ordered pre/post payments and warnings, and acceptance after actual reductions.
Primary-author typed behavioral RED, complete six-file source closure, GREEN
and independent root terminal receipts are preserved separately.

This is not independent Quint/Python correspondence or exhaustive verification.
No stateful Candidate A trace, neutral effect extraction, authority integration,
Council acceptance, candidate selection, or S02 gate follows yet.
Evidence: `evidence/s02-model-comparison/candidate-a-transactions/manifest.json`.
