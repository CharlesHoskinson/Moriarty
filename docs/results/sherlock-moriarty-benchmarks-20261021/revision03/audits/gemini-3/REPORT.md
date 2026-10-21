# Independent Non-Author Benchmark Suite Audit — Revision 03 (`audits03/gemini-3`)

- **Auditor Seat / Model**: `gemini-3` (`gemini-4-argon-high` via AGY, high effort)
- **Routing & Authority**: Independent non-author review under the explicit 3-Gemini + 3-Muse user request and standing Opus 5.5 outage fallback (`AGENTS.md`, `moriarty-dev:develop`, October 18 beta automation override). The three Muse seats returned API 429 (`usage_limit_reached`) with no verdict and remain un-substituted. Prior `audits02` envelopes are preserved (`gemini-1` and `gemini-3` CLI 0 / wrapper `ERROR` with 5 blocking findings each; `gemini-2` `SUCCESS` / `APPROVE_SOURCE_WITH_SCOPES`). Implementation authors (`sol-metrics`, `sol-loop`) hold zero vote.
- **Candidate Cut**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-moriarty-benchmark-suite-20261021/integration03/candidate` bound by `integration03/CANDIDATE-MAP.json` (`a49711eb0086fb516460b2fa02d937e89d2bd173c0f8bd29c5fd2917b54c25e1`, 17,915 bytes): **45 files total** (**14 changed** in Revision 03 across `sol-metrics` [4] and `sol-loop` [10]; **31 unchanged** from Revision 02).
- **Verdict**: **`APPROVE_SOURCE_WITH_SCOPES`** (0 blocking findings).

---

## 1. Candidate Pin & Test Receipt Verification

All 45 files in `integration03/candidate` were inspected against `integration03/CANDIDATE-MAP.json` and `openspec/changes/sherlock-moriarty-benchmarks/traceability.json` (`528ce97cec923188f138282212b1c30d3d10ba1b6dd02ce8d45e65a364f90f21`, 15,239 bytes, pinning the 44 non-self candidate files) via `audits03/gemini-3/probe_rev03.py`:

- **Changed `sol-metrics` files (4)**:
  - `models/sherlock/benchmarks/METRICS.md` (`8826a5105526948f5d7d2cac2a5f12eac27e6d750aa161d56ecd9ca62fc71182`, 15,358 bytes)
  - `models/sherlock/benchmarks/metrics.py` (`f23b98a81fc45797ae0aaa8605194a73a25f5638d9180daf7c3ef0be889f1e03`, 44,148 bytes)
  - `models/sherlock/benchmarks/sample.schema.json` (`56e0e8348ae20c367eef2f8120d293859c245bf7333655dcf2109681ac17973b`, 27,098 bytes)
  - `models/sherlock/benchmarks/tests/test_metrics.py` (`e8e0ea3b3a7bd2b4d38b82cde71e71f283399e852c2570a3869bf6a20006c656`, 37,744 bytes)
- **Changed `sol-loop` files (10)**:
  - `models/sherlock/benchmarks/acceptance.py` (`21681a60e927a27601c050dd3acac9f958e294897d233ca5528afff1b071b7ea`, 52,322 bytes)
  - `models/sherlock/benchmarks/test_acceptance.py` (`160e77eff04541dd522638abe40c50466ac324a7797588c3f405da8d147b23c3`, 37,629 bytes)
  - `models/sherlock/qa/test_benchmark_completion.py` (`1451417f566cc2321d508ccc058fd839ae2ad47ac1e9d5e131f96735747d36c7`, 5,756 bytes)
  - `docs/sherlock/benchmark-howto.md` (`f7860f824cf17bcb867b0f9aeed4349c0caf1a0d4c4103b17c9067091177008b`, 5,775 bytes)
  - `docs/sherlock/benchmark-reference.md` (`6ef877812343e04a916de22fb986264eb456ccebf28b5d9ae393451eec190961`, 14,164 bytes)
  - `docs/sherlock/implementation-loop.md` (`c54a5d8504316fbf97671c6dcf0ec8df75367a488f0ee1c949f86e1d45f08028`, 55,743 bytes)
  - `openspec/changes/sherlock-moriarty-benchmarks/proposal.md` (`c6ee4decda614fba4cd78865fa8b46260ffd48e4cdc9d7668cec7cee986f6527`, 2,112 bytes)
  - `openspec/changes/sherlock-moriarty-benchmarks/specs/benchmarks/spec.md` (`9aab22dec3ec1a3fcb38d678e1fb091ee299aab2a5e73164baf7ab77ca499e35`, 9,896 bytes)
  - `openspec/changes/sherlock-moriarty-benchmarks/tasks.md` (`4f91f1d439e2879ddadc48ee1a6a30f021a8ee393316fb84a66d4a9d48b8b641`, 2,305 bytes)
  - `openspec/changes/sherlock-moriarty-benchmarks/traceability.json` (`528ce97cec923188f138282212b1c30d3d10ba1b6dd02ce8d45e65a364f90f21`, 15,239 bytes)
- **Root Ordinary Test Receipt**: `integration03/root-actual-tests01/RESULT.json` (`54261fd7e268621c9c6b4d2fa5fd8646ab134a6268c8a29cc57c8b872a7dd82f`, 3,893 bytes) and all six raw stderr streams confirm **154/154 passing ordinary synthetic tests** (`48` metrics + `16` workloads + `39` acceptance + `7` checker-seam + `33` existing-check + `11` existing-requirements) with `source45ChangedDuringTests: false`.

---

## 2. Reconciliation of `audits02` Findings (Mandatory vs. Optional Scope)

1. **`REV02-F01` / `R02-F01` (Operation-Token Duplicate Filtering & Per-Profile Minima — RESOLVED)**
   - *Mandatory resolution*: In `acceptance._evaluate` (`acceptance.py` lines 691–712), duplicate `sample_id`, duplicate `(workload_id, receipt_sha256, pointer, observation_id)` occurrences, and reused operation `tokens` are filtered *before* incrementing `row["counts"][checked["status"]]` and *before* appending to the `valid` comparison pool, while `raw_counts` and `raw_samples` preserve all supplied records. In `acceptance.assess` (lines 761–777), each compared cache/environment profile must independently satisfy `case["min_successful_samples"]` in both `baseline` and `current`, and tolerance `min_samples` is enforced as `max(case["min_successful_samples"], rule["min_samples"])`.
   - *Optional / rejected overreach*: No unselected cold/warm profile is fabricated when only one profile is present.

2. **`REV02-F02` / `R02-F02` & `R02-F03` (Independent Prover/Verifier Attribution & All-Member Layer Aggregation — RESOLVED)**
   - *Mandatory resolution*: `_independent_commands` (`acceptance.py` lines 229–243) enforces disjoint reviewed `command_role_values` (`prove` vs. `local_verify`), distinct `(receipt_sha256, pointer)` bindings, and distinct non-empty `execution_id` values for both per-node and terminal roles. `_proof_chain` (line 586) requires receipt attribution across all 19 layer metrics (`witness.*`, `keygen.*`, `prove.*`, `local_verify.*`, `circuit.*`, `proof.*`, `proving_key.*`, `verifying_key.*`, `instance.*`, `artifact.*`). For multi-member layers (`len(members) > 1`), `_layer_aggregation` (lines 430–479) verifies explicit per-metric aggregation recipes across all ordered members: `observed_parallel_span` (`all-member-command-span`) with verified member/window `start`/`end` intervals for `wall_seconds`, `sum` (`cumulative-member-subprocess-cpu`) for `cpu_seconds`, `maximum` (`maximum-member-subprocess-peak-rss`) for `peak_rss_bytes`, `common` for `circuit.*`, `unique-verifying-keys` for `verifying_key.bytes`, and `sum` (`serialized-member-artifacts`) for remaining byte metrics, plus matching `qualified_aggregation_scopes` across comparisons.
   - *Optional / rejected overreach*: Distinct command-result pointers inside one shared JSON bundle are valid (`test_distinct_command_results_in_one_bundle_are_valid`); a blanket ban on shared receipt files was rightly rejected.

3. **`REV02-F03` / `R02-F05` (Frozen Catalog Scalar & Per-Transition State Oracle Bindings — RESOLVED)**
   - *Mandatory resolution*: `_oracle_catalog` (`acceptance.py` lines 199–227) loads the validated catalog's frozen `expected_effects_path` and Fenrir batch corpus, extracts all known scalar leaves via `_known_scalars` (`literal`, `program`, `ordered_transitions`, `work` for `sherlock-linked-two-step`; `balances`, `batches`, `ordered_transaction_ids`, `state`, `totals` for Fenrir), checks exact equality at `catalog_scalar_pointers`, and checks every transition's `pre`/`post` state scalars via `transition_state_pointers` and `state_scalar_fields`. `_proof_chain` (lines 544–546) binds each leaf statement's `pre_state_sha256` and `post_state_sha256` to those exact transition state views. `_context` (lines 167–169) requires distinct, non-overlapping `expected_effect_root_pointers`.
   - *Optional / rejected overreach*: Distinct semantic root fields may legitimately hold equal values, and valid identity programs are not blanket-forbidden when catalog transitions genuinely preserve state.

4. **`REV02-F04` / `R02-F04` (Partial Failure/Censored Lineage & Explicit `comparison_scope` — RESOLVED)**
   - *Mandatory resolution*: For `failure` and `censored` samples, `_sample` (`acceptance.py` lines 618–624) checks `_lineage`, `_observation`, and `_partial_sources` (lines 250–280), verifying producer context and attribution on available `observation`, `receipt_roles`, `proof_nodes`, and `layer_observations` without requiring fictitious successful proof chains or deployment artifacts. Censored observations must bind a non-empty `censoring_reason` via `observation_fields.censoring_reason`. Observation tokens are recorded in `all_tokens` across all statuses so failed/censored records cannot be reused between `baseline` and `current`. In `compare` mode, `assess` (lines 756–760) enforces full `required_metric_ids(case)` measurement coverage while reporting `comparison_scope` (`required_measurements`, `compared_metrics`, `measured_without_tolerance`).
   - *Optional / rejected overreach*: No invented all-93-metric SLA is imposed where tolerances were only declared for a subset.

5. **`REV02-F05` / `R02-F05` (Canonical Observation Key, Deduplicated `n` Comparison & Scoped Verifier Verdicts — RESOLVED)**
   - *Mandatory resolution*: `metrics.py` defines `metric_observation_key(metric)` returning `(receipt_sha256, canonical_pointer)` (lines 408–412) and uses it across `summarize` (line 439), `_exact_statistic` (line 509), and `compare_baseline` reuse checks (line 585). `metric_observation_identity(metric)` (lines 388–406) retains `scale_decimal`, `scope`, and `timer_resolution_seconds` metadata in `observation_identities` without treating rescaling as a new repetition. `compare_baseline` (lines 590–591) checks `min(old['n'], new['n']) >= rule['min_samples']` on deduplicated counts so retained descriptive duplicates neither inflate `n` nor poison comparisons that already meet `min_samples`. `collect_sample` (lines 672–691) requires `True` or a non-empty semantic status string in `terminal.success_values` (rejecting numeric exit `0`), checks nested verifier verdicts at the explicitly selected `terminal.pointer`, and retains root `verifies: false` dominance.
   - *Optional / rejected overreach*: Unrelated nested `verifies: false` fields (e.g., negative controls or historical entries) are not blanket-scanned when `terminal.pointer` selects the actual verdict.

---

## 3. Scopes, Boundaries & Non-Claims

- **Evidence Consistency Only**: Passing the 154 synthetic unit/seam tests and `probe_rev03.py` establishes suite-code and receipt-consistency behavior only. Zero native builds, Halo2/ZKIRv3 proofs, independent cryptographic verifications, or Midnight stagenet transactions have been executed.
- **Unchanged Project Gates**: Native authority, resource caps, operation counters, C4/R2/60460, C13/stopped-amendment dispositions, the four permanent lanes (Clean, Quint, K, handwritten circuits), and the fixed 35 UIREQ / 25 refinement / 68 scenario inventories remain unchanged and open.
