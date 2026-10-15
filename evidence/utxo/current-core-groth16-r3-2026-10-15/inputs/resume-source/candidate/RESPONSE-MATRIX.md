# Response matrix: review `a0d1725f…` → review-repair successor

Every finding and required action of `current-core-groth16-compiler-independent-source-review-20261015/REPORT.md` (SHA-256 `a0d1725fc6ad473f3aa9b94409c7894961c536de9a852f8d2ec4b6ccbe69455f`) has exactly one disposition below.

- **CHANGED** names the exact successor files. Their unified diffs against the origin bytes are in `DIFFS/` (listed in `DELTA.json`), and the verbatim origin bytes are in `history/origin-r2/`.
- **OPEN** means no byte change claims to resolve the finding. It stays a required gap.
- **RETAINED** means the bytes are unchanged and the finding's required action is already satisfied or is only a label.

Evidence is source-inspected and lexical only (`REPAIR-STATIC-CHECKS.json`, checks C1–C12). Nothing was compiled or run.

## Findings

| ID | Severity | Disposition | Exact bytes / evidence | What is NOT claimed |
|---|---|---|---|---|
| **F-01** test glob ambiguity | High (execution blocker) | **CHANGED** | `tests/common/mod.rs:76` now reads `pub fn compile_ok(`. The 24 bare calls `compile(` → `compile_ok(` span all 7 test files: accounting 1, arithmetic 1, duties_frame 5, generic_pipeline 10, propose 3, refusals 2, tamper 2. `duties_frame.rs:158` now reads `mori_r1cs::compile::<ark_bls12_381::Fr>(admitted, …)`. Applied mechanically by `repair-tools/f01_rename.py`, which refuses any unexpected line count. C1: `common::*` and `mori_r1cs::*` export no common name. C2: no bare `compile(` remains. C3: after undoing the rename, every inherited test line equals the origin except the enumerated F-11 lines, and no assertion line changed, so the frozen oracle outcomes are unchanged. | That the tests compile or pass. E3b is a prediction until an authorized run. |
| F-02 external duties | Medium (coverage) | **OPEN** | Refusal kept unchanged (`plan.rs:322-334`, `Unsupported{"external duty"}`). A dynamic duty-identity cell is required future work. | No external-duty support. |
| F-03 conservative bound | Low | **OPEN** (accepted as conservative) | `plan.rs` bound unchanged; the REFUSALS bound row is unchanged. | No completeness for Core-admissible scenarios the bound refuses. |
| **F-04** balance-row presence mislabelled | Medium | **CHANGED (relabel to target premise; not fixed)** | `src/coverage.rs`: new `Support::TargetPremise`. The REFUSALS row "mori::core::balance (missing explicit balance)" moves from `Constant` to `TargetPremise`, with text citing M-14. The `Scenario.balances` field row names the premise. `src/adapter.rs` TARGET_OBLIGATIONS gains a "Balance-row existence (premise, review F-04)" entry. `COVERAGE.md` changes its field table, refusal semantics ("except one premise row") and target-obligation section. C7 checks this. The Core premise was re-read at `runtime.rs:609-627`. | The relation still admits a 0 pre-balance for a missing row. M-14 stays expected SATISFIABLE. A presence bit is only proposed (`DUTY-FRAME-PROPOSAL.md` §3). |
| F-05 unregistered-duty frame, extra rows | Medium | **OPEN; labels CHANGED** | `src/adapter.rs` TARGET_OBLIGATIONS "Frame completeness" now states that FrameCells covers balance cells only, that unregistered incoming duties cannot be represented, and that no P3 or ABI mapping may claim a complete frame. `src/coverage.rs` `Scenario.liabilities` row and `COVERAGE.md` say the same. A precise extension is proposed separately in `DUTY-FRAME-PROPOSAL.md` §2 and §4 and is not implemented. | No duty-frame option, no extra-row exclusion, no complete P3 frame or ABI coverage. |
| **F-06** toolchain never selected | Medium (request) | **CHANGED (request)** | `CORRECTED-EXECUTION-REQUEST.json` `toolchain`: the reviewer's `selection` and `control` are carried verbatim. `repair_author_rehash_read_only` holds cargo `e951141c…`, rustc `f3834d26…` and the `librustc_driver` hash, all equal to the reviewer's. The observed rustup `settings.toml` (default `stable`) is recorded, and no `rust-toolchain*` exists on layout ancestors. PC-2a requires these hashes in the authorization. | No binary executed; the version strings are unobserved. |
| **F-07** name-level lock predicate | Low (request) | **CHANGED (request)** | `lock_identity`: the 102 reviewed registry rows are pinned by `LOCK-UNION-ANALYSIS.json` `328d157e…`, and the registry-subset digest is recorded. The predicate is checksum-level via the unchanged `lock_subset_check.py` `a3215bd0…`. E1hv adds the proposed `repair-tools/lock_pair_compare.py`; its synthetic controls returned SAME→0 and DIFFER→3. | No lock generated or resolved. Staying inside the union is unproven until E1/E1h. |
| **F-08** caller-supplied build digests | Low | **CHANGED (request); source unchanged** | `CORRECTED-EXECUTION-REQUEST.json` `identity_binding` separates what is **already bound in-circuit** (header constants, incl. `compiler_build_digest` over the two caller-supplied digests) from **proposed catalogue/admission metadata** (lock digest, registry-subset digest, toolchain hashes, observed r1cs digest). E0 must recompute the BUILD-MANIFEST and closure digests. `BUILD-IDENTITY-DELTA.json` tracks the build change `2c943b51…` → `4e3ba2e2…3b47`. It also records that the reviewer harness's hard-coded `2c943b51` is stale for the successor: outcomes are unaffected, identity columns are not evidence. | `CompilerBuild` is unchanged: the compiler still does not verify its digests, and no catalogue schema is adopted. |
| F-09 UNVERIFIED-LIB facts confirmed | Info | **RETAINED** | The `r1cs.rs` and `circuit.rs` doc labels are unchanged (C5/C6 keep synthesis bytes identical). The reviewer's confirmation (review §5) is recorded here for the listed facts only. | Compilation is still unobserved. Labels are retired only after an authorized E3a. |
| F-10 Emit target policy not wired | Info | **OPEN; doc CHANGED** | `src/adapter.rs` doc on `initial_ledger_target_admits` now states that it is not called by `compile`, by `StatementAdapter::admits` or by any path, so a future target adapter must wire it under review. `COVERAGE.md` refusal row updated. `ABI_V1_MAPPING_REQUIREMENTS` is unchanged and already lists it. Emit stays lowered. | No enforcement exists; the policy is declared only. |
| **F-11** overstated test name | Low | **CHANGED** | In `tests/generic_pipeline.rs`, the test is renamed `compile_source_is_deterministic_and_never_reads_program_names`, its comment disclaims "only path", and the module doc names both public paths (`compile_source`; `admit_program` + `compile`), both through Core `validate`. C4 checks this. Test assertions are unchanged (C3). | — |
| **F-12** malformed `Assignment` can panic | Low (robustness) | **CHANGED** | `src/circuit.rs`: `CompiledRelation::circuit` calls the new `check_assignment_lengths`, which compares inputs, pre/post states, cells and duties, effects and the public vector length. `src/error.rs`: new `CompileError::AssignmentShape(String)` with a `Display` arm. New `tests/assignment_shape.rs` has 2 tests and rows R12-0…R12-9. Expectations were frozen first (`REPAIR-EXPECTATIONS/FREEZE.json`, 2026-10-15T13:33:19Z, aggregate `d11a426f…`); they are repair-author, not independent. C6: `compile`, `compile_source`, `probe`, `synthesize` and their helpers are byte-identical, so no constraint changed. C11 checks the wiring. | That the new tests compile or pass. Value-level checks stay with the constraints by design. |
| F-13 DirectFieldV0 re-wrappable | Info | **RETAINED** | `adapter.rs` module docs and variant doc are unchanged: test-only, no claimant/outref binding, never a claim ABI. The request's `never_additional` repeats it. | No claim ABI. |
| F-14 no rejection relation | Info | **OPEN** | Declared in COVERAGE.md and REFUSALS (unchanged). | No refusal-code differential in-circuit. |

## Votes and amendments (detail in `REPAIR-VOTES.md`)

| Item | Repair-author vote | Count on identical text |
|---|---|---|
| C4 reviewer amendment | AGREE (literal); implemented as a relabel | 2 (reviewer, repair author) |
| C9 reviewer amendment | AGREE (literal) + additive clause (metadata, not in-circuit) | 2; clause 1 |
| C15 reviewer amendment | AGREE (literal) | 2 |
| PC-1 | AMEND (BUILD-MANIFEST invariant cannot hold) | literal 1; PC-1r 1 |
| PC-2 | AGREE (literal) + PC-2a | 2; PC-2a 1 |
| PC-3 | AGREE (literal) + PC-3a | 2; PC-3a 1 |
| PC-4 | AMEND (citation erratum §9 → §8/§10) | literal 1; PC-4r 1 |
| Request blocks copied by value (`isolated_layout`, `toolchain.selection`, `toolchain.control`, `environment`, `sandbox`, `never`, `interpretation`, E1, E1v, E1h, E2, E3a, E3c, scoring) | AGREE (literal) | 2 each |
| E0, E1hv, E3b, E4, E5, PC-0, `inherited_counters`, `lock_identity`, `identity_binding` | new or amended text | 1 (repair author) |
| D-13 value | ABSTAIN | split preserved |
| P3-1 … P3-13 | no vote | All disagreements preserved. V6 `f3ae8556` P3-7 CHANGE (all-account authority, thread-advance authority, signed intent) is **not** overruled by the compiler review's AGREE on the external-account wording. Reconciliation deferred to the source owner |

## User mandates checked

| Mandate | Disposition |
|---|---|
| DirectFieldV0 test-only, no claimant/outref binding, never a claim ABI | RETAINED unchanged; `never_additional` in the request |
| No new ABI v1 mapping while its full review is pending | No mapping added. `ClaimantBoundAbiV1Pending` still refuses. `ABI_V1_MAPPING_REQUIREMENTS` is byte-unchanged |
| Emit lowered; initial-ledger whole-program refusal must be wired by a future adapter | Emit lowering unchanged. Non-wiring now stated in the source doc and COVERAGE (F-10) |
| Do not change arithmetic, evaluation, memo, cost or circuit constraints | `gadgets.rs`, `lower.rs`, `plan.rs`, `statement.rs`, `r1cs.rs`, `witness.rs` byte-identical (C5); synthesis functions byte-identical (C6) |
| Frozen reviewer 61 vectors / 9 static / 26 hostile and two post-freeze addenda used unchanged | Hashes re-verified (`INPUT-VERIFICATION.json`); not copied or edited; the harness is not edited and not claimed to have run |
| Fresh exact full-candidate/delta Opus audit before any E0–E5 | PC-0 in the request |
| Old freezes, expected packets and counters retained | `EXPECTED-PACKET/` (FREEZE, ADDENDUM-1-FREEZE) and `fixtures/` byte-identical. Origin `EXECUTION-REQUEST.json`, `FAILURES.json`, `STATIC-CHECKS.json` and `DESIGN-VOTES.md` byte-identical at the root. Replaced files kept verbatim in `history/origin-r2/`. Counters in the request's `inherited_counters` are unchanged at zero runs |
