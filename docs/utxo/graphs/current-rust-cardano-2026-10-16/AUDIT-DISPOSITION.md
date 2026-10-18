# R3 disposition of the independent Muse graph review

Candidate: R2 graph SHA-256 `879d2a8cf4d18827c34fb5fe686726d928db9a28cd72caec13b5e540264c00c1`. R3 preserves those graph bytes exactly. Review: fresh regular `muse-wonder-1.0` at HIGH effort; report SHA-256 `eef0a8232ea7802079a4114cb6c44858bfc94903a559d8478e7339ab7d8ef5f3`; findings SHA-256 `749c9a8bf26fbf09c82faf91e53a0da77238bc40f1c59b3aad0dc655c81df8f7`. The review is conditional on documentation corrections and approves no execution, proof or ledger claim.

| Finding | Disposition |
|---|---|
| F-01: mixed classification keys | **Resolved in documentation.** Consumers must classify with `evidence_type` when present, else `type`. 165 links are type-only. The 3,401 / 216 / 3 counts use that read-time fallback; the graph is unchanged and the structural check says `semanticApproval: false`. |
| F-02: 1,542 / 5,627 extraction counts | **Withdrawn as graph provenance; unverified.** R2 retains no raw extraction JSON/cache or transformation counts that reconcile them with the published graph. R3 makes no deduplication/filtering claim. Published graph counts are 1,356 / 3,620 from the read-only root structural check. |
| F-03: stale R1 run-state strings | **Resolved in documentation only.** The frozen size status, two `RAW_PARTIAL_CAPTURE` link qualifications and R2 pending-review metadata are historical snapshots. Later result review/diagnosis supersedes their run-state wording. Size and chain receipts remain raw and unadopted; the failed run remains unadopted. |
| F-04: source path bases and inferred calls | **Resolved in documentation.** AST links are package-relative to `packageRoot`; file nodes use the `experiments/moriarty-cardano-rust/` prefix from repository root. Name-based INFERRED call links (sample confidence 0.8) can misbind same-name symbols and are navigation hypotheses. |
| F-05: K4H pending text | **Resolved in documentation.** K4H actual-result review is complete and its 107 finite cases, 29/46-slot financial receipts and 72 reviewed public control rows are adopted narrowly for K5 source preparation. Historical package README/inventory pending statements are author-time snapshots. Four-way run review/initial diagnosis are now complete; that separate run remains failed and unadopted. |

## Current time-scoped disposition

The completed four-way run had 129/147 tests pass, 18 fail, OPX2 exit 1, and false T-147, PC4-4 and RANGE-vs1 predictions. The independent Muse result review identifies a harness identifier collision (`vs2_t` equals `c2_hash`) as the common failure cause and proposes a test-only correction. The correction is unrun; fresh Gemini review, resource agreement and root authorization are still required before it can run. The author’s self-review is not acceptance. No graph refresh or source/ledger conclusion follows from this documentation update.

R2 output remains immutable. R3 adds this disposition and time-scoped documentation; all product gaps and proof/ledger obligations stay open.
