# K10: current-proof evaluation through the compiled C2 claim wrapper

This proposed publication records one completed local evaluation of the **already compiled, applied B9 C2 PlutusV3 mint wrapper** using the genuine public CURRENT Rust K6 invoice proof. The [root adoption][adoption] closes only `compiledC2WrapperPrerequisiteClosed`: one positive context returned unit and six frozen mutations returned semantic `EvaluationFailure`. It establishes a finite compiled-wrapper milestone; ledger and Preprod acceptance remain open.

**Publication review is pending.** The [fresh actual-result review][actual] supports that narrow result. Its [recommendation][recommendation] leaves public-record export and public-input publication flags false. This new package requires a fresh Muse publication audit before root integration. Nothing here creates execution, retry, signing or ledger authority.

The [native receipt][receipt], [outer capture][capture] and fourteen raw stdout/stderr files preserve the actual seven cases. T1 returned exit 0, empty stderr and `(con unit ())`. T2–T7 returned exit 1, empty stdout and the strictly parsed EvaluationFailure error/cost form.

| Case | Frozen change | Observed result | Local VM CPU / memory |
| --- | --- | --- | --- |
| T1 | Genuine public proof, coherent issue context | Unit, exit 0 | 2,222,340,191 / 205,312 |
| T2 | N1 statement/body digest mutation | EvaluationFailure, exit 1 | 2,220,212,191 / 192,012 |
| T3 | N2 statement/context digest mutation | EvaluationFailure, exit 1 | 2,220,212,191 / 192,012 |
| T4 | N3 proof A/C swap | EvaluationFailure, exit 1 | 2,220,212,191 / 192,012 |
| T5 | Mint quantity two | EvaluationFailure, exit 1 | 26,983,518 / 85,187 |
| T6 | Claim token-name bit flip | EvaluationFailure, exit 1 | 58,371,081 / 127,229 |
| T7 | Envelope VK-digest bit flip | EvaluationFailure, exit 1 | 74,419,397 / 170,473 |

The guard names in the records are **source-expected** from the unchanged [`claim.issue`][claim] conjunct order, positive control and minimally changed cases. Runtime guard instrumentation was absent; typed decoding can also cause EvaluationFailure. These six refusals are neither six distinct cryptographic theorems nor compiled Bool-false results. The same public proof was reused.

The captured wrapper has **2,770 CBOR bytes**, a `590acf` byte-string prefix and **2,767 inner flat bytes**. Its recorded applied policy/script hash is `6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1`, using `blake2b-224(0x03 || 2770 CBOR bytes)`. The inner flat bytes alone are not that hash input. See the [script hex][script], [generated manifest][generated], [public proof/VK expectations][expectations] and [C2 source][c2]. The reused public VK is 488 bytes and proof 192 bytes. Tool identity is release-declared Aiken v1.1.24 provenance; this record does not establish an independently signed binary or a reproducible source build. No executable tool body is included.

The completed attempt recorded **7 launches, 7 returns and 7 validations; 1 attempt, 0 retries**. Outer wall time was `0.8483442220021971` seconds and waited-children CPU `0.570947` seconds. Transitive native CPU was not measured, and native-executable-start counts were not independently measured. The CPU/memory figures above use local `ExBudget::default()` VM accounting. They do not qualify ledger ExUnits, transaction fees, min-ADA, collateral or transaction fit. The run generated no proof or setup and performed no signing, key access, network request or ledger transaction.

The source history is deliberately preserved. The [R1 source review][r1] (`f7caa24e…`) called the candidate ready, but [root reproduction][defects] found an eagerly evaluated mapping default, missing post-run custody of nine staged inputs, and a returned call lost from accounting when validation failed. R1 was never executed or authorized. R2 repaired those three points and received a [fresh full source review][r2] (`a67387df…`). The [five ordinary helper outcomes][helpers] cover staged-file integrity only; they do not test the entire controller or proof system. The [materialized operator][operator] is included as archived source, not a portable run recipe.

The [source decision][source-authority] adopted 13 literal author-FOR plus fresh-source-AGREE positions before the historical AUTH. That tally is separate from the actual review, whose positions array is empty despite a request for 13 repeats. There are no invented actual-review votes. The older demand for two fresh full source audits was explicitly qualified by root; the incorrect R1 approval was not counted toward R2. Failures, ABSTAIN records, stale source comments and source-time pending flags remain byte-identical. In particular, `COMPLETED_AWAITING_FRESH_ACTUAL_RESULT_AUDIT` in the captured receipt and the capture's false review flag are historical observations; the later companion adoption closes only the scoped actual K10 predicate.

The author/reviewer records disclose the explicit user Muse fallback after the Opus timeout. Fresh non-author Muse Wonder 1.0 review provides same-family independence, not cross-provider independence. HIGH was requested; actual provider effort was not independently observed. Generic native-roundtrip or authority-ready flags in raw reviews are narrowed by the root adoption and do not imply general native acceptance or new authority. The [AUTH][auth] is historical and [spent][spent]. Public compilation, proving and deployment remain permissionless; internal experiment review is not public developer admission.

The claimant `555…`, outref `2222…`, settle-A2 context, zero fee and unbound 2030 expiry are caller-fabricated premises. Issuance checks a proof against an asserted digest; settlement must separately recompute objective identities from authenticated facts. These evaluations establish no genuine ledger facts, intent signature, actual real-context proof, compiled VI/four-piece atomic settlement or signed on-chain transaction. Class-0 single-party setup remains forgeable and VK-to-matrix/constraint correspondence remains unproved, as qualified by the adopted review.

All 20 UTxO families; ACTUS 277 fixtures, 18 types and 32 taxonomy items; DeFi 72 and general financial obligations; real recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod qualification remain open. No whole-product compatibility percentage is claimed. The remaining path includes a first qualified live public Preprod observation, a fresh real-wallet/thread-context proof, a genuine secp256k1 intent signature, compiled VI plus four pieces with atomic settlement, and actual ledger acceptance under separately reviewed scopes. This capsule grants no authority for those steps.

[PUBLICATION.json](PUBLICATION.json) records scope, read coverage and pending review. [COPY-MANIFEST.json](COPY-MANIFEST.json) is the complete inventory: original path, publication-relative path, byte count and SHA-256 for every copied payload, with a lossless source map. All 327 frozen public inputs are mirrored under `payload/inputs/`, preserving source provenance rather than depending on external historical capsules. Eight actual-review outputs, the [input freeze][freeze] and later root adoption bring the copied total to **337 payloads / 5,125,461 bytes**. Four packaging documents bring the staged file count to **341**. Source commit `752c7aa7a07523a7bc5bf929fcadc094815c48e4` is checkout context; out-of-repository evidence is pinned independently by the freeze. Original absolute references are archival provenance, not permission to access referenced paths.

Every copied payload was checked against its byte/hash pin and copied unchanged. Packaging semantic inspection focused on adoption, decisive reports, raw case logs and selected structured receipt fields; it was not a fresh review of all 327 source files. The original auditor's [read coverage][review-coverage] is retained separately. No native tests or existing integration helpers were executed to prepare this package.

[adoption]: root/K10-ACTUAL-ADOPTION.json
[actual]: reviews/actual/REPORT.md
[recommendation]: reviews/actual/ADOPTION-RECOMMENDATION.json
[review-coverage]: reviews/actual/READ-COVERAGE.json
[freeze]: reviews/actual/ROOT-INPUT-FREEZE.json
[receipt]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-local-20261016/receipts/RESULT.json
[capture]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-root-capture-20261016/RESULT.json
[script]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-public-inputs-generated-20261016/script.hex
[generated]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-public-inputs-generated-20261016/manifest.json
[expectations]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-primary-hash-input-adapter-r2-candidate-20261016/EXPECTATIONS.json
[operator]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-operator-materialized-20261016/runner/operator.py
[source-authority]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-root-records-20261016/SOURCE-ADOPTION-AND-ONE-RUN-AUTHORITY.json
[auth]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-root-records-20261016/AUTH.json
[spent]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-root-records-20261016/AUTH-SPENT.json
[r1]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-compiled-wrapper-operator-independent-muse-source-review-20261016/REPORT.md
[defects]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-native-operator-root-source-reproductions-20261016/REPRODUCTION.json
[r2]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-native-operator-r2-independent-muse-source-review-20261016/REPORT.md
[helpers]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-r2-ordinary-helper-regressions-20261016/RESULT.json
[c2]: payload/inputs/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement/validators/c2.ak
[claim]: payload/inputs/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement/lib/pvc2/claim.ak
