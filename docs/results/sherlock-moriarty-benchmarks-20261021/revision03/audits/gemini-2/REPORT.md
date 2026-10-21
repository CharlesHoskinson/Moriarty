# Independent Non-Author Benchmark Audit (`revision03` — Seat `gemini-2`)

- **Verdict**: `APPROVE_SOURCE_WITH_SCOPES` (`0` blocking findings, `2` optional low-severity observations)
- **Reviewer**: `gemini-2` (`gemini-4-argon-high` via AGY; user-requested 3-Gemini benchmark audit roster and standing Opus 5.5 outage fallback; the 3 Muse seats returned API 429 with no verdict and are not substituted)
- **Candidate Root**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-moriarty-benchmark-suite-20261021/integration03/candidate`
- **Authoritative Map**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-moriarty-benchmark-suite-20261021/integration03/CANDIDATE-MAP.json` (`45` files total: `14` changed in `revision03` across `sol-metrics` [4] and `sol-loop` [10]; `31` byte-identical to `integration02/candidate`)
- **Execution Boundary**: Ordinary synthetic consistency/collector/acceptance verification only (`154/154` unit tests passing in `root-actual-tests01/RESULT.json`: `48` metrics, `16` workloads, `39` acceptance, `7` checker-seam, `33` existing-check, `11` existing-requirements). No native `ledger-9` proving, circuit compilation, or Midnight stagenet transaction execution has occurred (`nativeExecuted: false`).

## 1. Machine-Verified Candidate Pins (`revision03`)

All 45 candidate files were read in full and machine-verified by `audits03/gemini-2/probe_revision03.py` (`probe_results.json`) against `integration03/CANDIDATE-MAP.json`, `integration02/candidate`, and the 44 cross-lane `sourcePins` in `openspec/changes/sherlock-moriarty-benchmarks/traceability.json` (`15239` bytes, SHA-256 `528ce97cec923188f138282212b1c30d3d10ba1b6dd02ce8d45e65a364f90f21`):

- `models/sherlock/benchmarks/metrics.py`: `44148` bytes, SHA-256 `f23b98a81fc45797ae0aaa8605194a73a25f5638d9180daf7c3ef0be889f1e03`
- `models/sherlock/benchmarks/sample.schema.json`: `27098` bytes, SHA-256 `56e0e8348ae20c367eef2f8120d293859c245bf7333655dcf2109681ac17973b`
- `models/sherlock/benchmarks/tests/test_metrics.py`: `37744` bytes, SHA-256 `e8e0ea3b3a7bd2b4d38b82cde71e71f283399e852c2570a3869bf6a20006c656`
- `models/sherlock/benchmarks/METRICS.md`: `15358` bytes, SHA-256 `8826a5105526948f5d7d2cac2a5f12eac27e6d750aa161d56ecd9ca62fc71182`
- `models/sherlock/benchmarks/acceptance.py`: `52322` bytes, SHA-256 `21681a60e927a27601c050dd3acac9f958e294897d233ca5528afff1b071b7ea`
- `models/sherlock/benchmarks/test_acceptance.py`: `37629` bytes, SHA-256 `160e77eff04541dd522638abe40c50466ac324a7797588c3f405da8d147b23c3`
- `models/sherlock/qa/test_benchmark_completion.py`: `5756` bytes, SHA-256 `1451417f566cc2321d508ccc058fd839ae2ad47ac1e9d5e131f96735747d36c7`
- Unchanged core anchors: `models/sherlock/benchmarks/workloads/generate.py` (`30045` bytes, `65cf4856bdff1b185b792df576a57471173d1fa7e580d58f38a814526f14b1dc`), `models/sherlock/benchmarks/workloads/catalog.json` (`59505` bytes, `6d6f7ae8708d2910dfce6a1187297cc463b177afcc889b2da6db2c66a1e3c9b4`), and `models/sherlock/qa/check.py` (`89149` bytes, `c22b024c5e845cb4bea1cf6898c3f429569324dc18fc4dc72b023cdf35637846`).

## 2. Reconciliation of All 10 `audits02` Findings (`R02-F01`..`05`, `REV02-F01`..`05`)

1. **`R02-F01` (Duplicate observation filtering vs `compare_baseline` poisoning) — RESOLVED**:
   - In `models/sherlock/benchmarks/acceptance.py:688-712`, `_evaluate` increments `row["duplicate_observations"]` and executes `continue` before `valid.append(checked)` whenever `sample_id` is reused, `(workload_id, observation.receipt_sha256, observation.pointer, observation_id)` repeats, or a successful sample's operation `tokens` intersect `all_tokens`. Token-deduplicated wrappers never enter `validated` or `old_samples`.
   - In `models/sherlock/benchmarks/metrics.py:590-591`, `compare_baseline` gates sufficiency on unique observations (`min(old['n'], new['n']) >= rule['min_samples']`) and zero `success_missing`/`success_unavailable`/`success_censored` without failing when descriptive duplicates (`duplicate_observations > 0`) are retained in `summarize` (`test_metrics.py:470-482`, `test_acceptance.py:395-413`).
2. **`R02-F02` & `REV02-F03` (Frozen catalog scalar oracle, per-transition state bindings, and root pointer distinctness) — RESOLVED WITH CONTRACT QUALIFICATION**:
   - `acceptance.py:165-227` (`_oracle_catalog`) extracts all known non-unavailable scalar leaves (`_known_scalars`) from the frozen catalog expected document (`literal`, `program`, `ordered_transitions`, `work` for `sherlock-linked-two-step`; `balances`, `batches`, `ordered_transaction_ids`, `state`, `totals` for Fenrir) and requires 1-to-1 value equality at distinct JSON pointers in `context["catalog_scalar_pointers"]`.
   - It further validates every ordered transition's `pre`/`post` state scalars via `context["transition_state_pointers"]` and `context["state_scalar_fields"]`, enforces step-to-step state continuity (`index == 0 or pre_state == previous_post`), and binds every leaf proof node's `pre_state_sha256` and `post_state_sha256` to `canonical_digest(_pointer(effects, pointers[boundary]))` (`acceptance.py:544-546`).
   - *Contract Qualification*: `expected_effect_root_pointers` must use distinct, non-overlapping JSON pointers (`acceptance.py:168-169`), while root or state *values* may legitimately coincide for valid identity or zero-net-change transitions (`test_acceptance.py:344-359`).
3. **`R02-F03` (Independent `prove` vs `local_verify` execution bindings) — RESOLVED WITH CONTRACT QUALIFICATION**:
   - `acceptance.py:229-243, 522-523, 641-643` (`_independent_commands`) requires disjoint `command_role_values["prove"]` and `["local_verify"]`, distinct `{receipt_sha256, pointer}` bindings (`roles["prove"] != roles["local_verify"]`), distinct `execution_id` values (`commands[0] != commands[1]`), and role-appropriate `mode` values.
   - *Contract Qualification*: `prove` and `local_verify` may legitimately share a JSON bundle receipt file provided they resolve through distinct JSON pointers (e.g., `/runs/prove` vs `/runs/local_verify`) with distinct `execution_id`s and modes (`test_acceptance.py:325-342`).
4. **`R02-F04` (`comparison_scope` reporting and per-profile repetition minimums) — RESOLVED**:
   - `acceptance.py:749-763` preserves full mandatory metric coverage (`metric_coverage(sample, required_metric_ids(case))`) and populates `report["comparison_scope"]` per workload with `required_measurements`, `compared_metrics`, and `measured_without_tolerance`, avoiding an invented 93-metric SLA.
   - `acceptance.py:761-777` raises each rule's `min_samples` to `max(case["min_successful_samples"], rule["min_samples"])` and checks that minimum within every compared `(workload, evidence, identity, cache, cache_identity, aggregation_scopes)` profile group (`test_acceptance.py:414-439`).
5. **`R02-F05` (Semantic verifier verdict pointer vs numeric exit codes and negative controls) — RESOLVED WITH CONTRACT QUALIFICATION**:
   - `metrics.py:672-673` restricts `terminal["success_values"]` to boolean `True` or non-empty strings (rejecting numeric `0` and boolean `False`) and evaluates the selected `terminal["pointer"]` directly.
   - *Contract Qualification*: Unrelated negative-control fields containing `verifies: false` elsewhere in a receipt do not invalidate a sample whose selected `terminal["pointer"]` resolves to `True` (`test_metrics.py:483-497`).
6. **`REV02-F01` (Multi-node layer aggregation soundness, wall-span vs sum, RSS scope, and phase attribution) — RESOLVED**:
   - `acceptance.py:430-479, 577-596` requires `extension["layer_observations"][layer]` whenever `len(members) > 1` and enforces exact member order, contributor sources within each member's `receipt_roles[phase]`, `observed_parallel_span` (`all-member-command-span` with `start == min(starts)`, `end == max(ends)`, and `last - first == value`) for `wall_seconds`, `sum` (`cumulative-member-subprocess-cpu`) for `cpu_seconds`, `maximum` (`maximum-member-subprocess-peak-rss`) for `peak_rss_bytes`, `common` for `circuit.*`, `unique-verifying-keys` for `verifying_key.bytes`, and `sum` (`serialized-member-artifacts`) for other byte metrics.
   - `acceptance.py:586-591` enforces layer receipt attribution across `witness.`, `keygen.`, `prove.`, `local_verify.`, `circuit.`, `proof.`, `proving_key.`, `verifying_key.`, `instance.`, and `artifact.` (`test_acceptance.py:360-373, 442-447`).
7. **`REV02-F02` (Per-node `operation_id` and `circuit_sha256` proof-graph bindings) — RESOLVED**:
   - `acceptance.py:26-28, 489-530` includes `operation_id` and `circuit_sha256` in `NODE_FIELDS` and verifies both inside every node's `prove` and `local_verify` receipt (`test_acceptance.py:360-373`).
8. **`REV02-F04` (Partial failure/censored producer lineage without fictitious success artifacts) — RESOLVED**:
   - `acceptance.py:250-280, 622-624` validates non-success samples through `_partial_sources`, requiring available measured/censored metrics to come from context-verified observation/role/node/layer receipts and requiring a non-empty `censoring_reason` when `status == "censored"`, without demanding unobserved `proof_nodes` or deployment receipts. Returning `{observation_token}` prevents reusing failed/censored observations across baseline and current runs (`test_acceptance.py:374-394, 449-458`).
9. **`REV02-F05` (`metric_observation_key` scale-collision vs shared-receipt distinct pointers) — RESOLVED**:
   - `metrics.py:388-411, 439, 509, 585` defines `metric_observation_key(metric)` as `(receipt_sha256, canonical_pointer)` across `summarize`, `_exact_statistic`, and `compare_baseline`, while retaining `scale`, `scope`, and `timer_resolution_seconds` in `metric_observation_identity(metric)`. Rescaling the same receipt+pointer is caught as a duplicate observation, whereas distinct pointers in a shared receipt bundle remain distinct (`test_metrics.py:498-512`).

## 3. Optional Non-Blocking Observations (`mandatory: false`)

- **`OPT-R03-01` (`models/sherlock/benchmarks/acceptance.py:261-269`)**: In `_partial_sources`, when a failed/censored sample optionally supplies `proof_nodes` with `receipt_roles`, line 268 indexes `policy["proof_receipt_fields"]["prove"][key]` rather than `policy["proof_receipt_fields"][role][key]`. Non-blocking because partial samples never count toward `min_successful_samples` or statistical baseline comparisons.
- **`OPT-R03-02` (`models/sherlock/benchmarks/acceptance.py:570-579`)**: `_layer_aggregation` is called when `len(members) > 1`; if an optional `layer_observations[layer]` receipt is attached to a single-member layer (`len(members) == 1`), it is added to `candidates` without invoking `_layer_aggregation`. Non-blocking because single-member layers are already bound to their sole member's `circuit_k` and artifact byte lengths at lines 580-595.

## 4. Scope Boundary & Next Dependency

This approval covers the `revision03` benchmark suite source, schemas, deterministic workload generator/catalog, DoctorWatson `--repo`/`--benchmark-*` CLI integration, and OpenSpec traceability. Actual baseline establishment remains gated on executing the real `ledger-9` native prover/verifier and Midnight stagenet recursive rollup transactions.
