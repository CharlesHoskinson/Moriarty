# Tasks and evidence

Checkboxes record this proposal's observed evidence, not full Sherlock acceptance. Fresh returned independent review and integration remain required for changed final bytes.

- [x] DWCF-001: Implement shared capability facts and additive dispatch reporting, including refresh after reconciliation. Evidence: `implementation/readiness/verification.json` and `preflight-verification.json`, capability/preflight qualification regressions (96 focused readiness tests on the latter cut).
- [x] DWCF-002: Compute fingerprints once per assessment; reuse exact FileRefs locally; cache requirement bytes/digests; detect intervening changes. Evidence: 95 focused regressions plus two requirement-cache tests under `implementation/readiness/`.
- [x] DWCF-003/004: Implement mandatory execution history and fixed versioned handkey selection, launch evidence and result collection. Evidence: `implementation/verifier/RESULT.json`, 210-test overlapping cohort; harmless verifier fixtures only.
- [x] Model the readiness distinction independently in additive Quint files. Evidence: `validation/quint/REPORT.md`, 14 directed tests, 1,000 samples, and the expected naive-readiness counterexample. This is model evidence only.
- [x] DWCF-005: Draft the source-currentness/lifecycle wiki review, correcting old-W absence claims and VK-versus-PK confusion. Evidence: `wiki-llm/doctorwatson-workflow-review-2026-10-22.md`.
- [x] DWCF-006/007: Implement and test ordered graph-refresh → wiki-append → cleanup finalization, stable cycle identity and idempotent resume. Owner: cycle-finalization implementation owner; require real entrypoint and exact receipts.
- [x] DWCF-008: Test retained-root ancestry and open PR head/base protection, branch/worktree inactivity, exact-tip compare-and-swap, active/dirty/untracked/ignored/user-data preservation, ambiguity refusal and post-snapshot races using disposable repositories.
- [x] DWCF-009: Test graph, append and cleanup failures, durable partial results, incomplete-cycle reporting and safe retry/resume behavior.
- [x] DWCF-010: Run affected local CI on the integrated final tree; retain exact command exits and evidence locators. Add no hosted checks.
- [x] Obtain fresh non-author review of the exact complete candidate and resolve substantive findings without author self-certification.
- [ ] Record actual merged commit separately from tests and review. Execute finalization only within the authorized selection and retain the terminal receipt; do not infer it from source or fixtures.

Cleanup revision02 is independently reviewed: 31 author checks plus five focused independent regressions passed. Stable operation identity and actual retirement receipts survive fresh inventory and inspection. The final integrated factory run passed 264 tests. Actual graph/wiki finalization succeeded with an explicit empty retirement selection after the earlier live inspection refused missing stale refs. No Git retirement is claimed. See [portable results](../../../docs/audits/2026-10-22/LOCAL-RESULTS.json).

No task here closes the native parent proof, wallet uncertainty, Build9/MP2/D, full39 interpreter map, exact-intent rollup/range statement, full language requirements or stagenet acceptance.
