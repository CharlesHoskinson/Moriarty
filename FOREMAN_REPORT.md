# FOREMAN_REPORT

- run_id: moriarty-release-afk-20260913
- role: implement
- slug: production-executor
- branch: foreman/moriarty-release-afk-20260913/implement/production-executor
- worktree: /home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor
- base_sha: f7f40ce5e945fd9d7d2bd7a3c05ee3798331ff8e
- prior frozen candidate: /tmp/moriarty-production-grok-r1-freeze/manifest.json
- prior sourceMapSha256: 68f879d2897471f6ee69bc7cbda0528baad8ef9991cafaacd4c5e8375114acda
- this candidate sourceMapSha256 (same 43 paths): cd5772c1ad43ba25389cbe68db3540f413420485f4e136bdf103d756fa780c18
- requested model: grok-4.6 high
- observed provider usage: grok-4.6-build (prior canary / independent review; this process has no separate identity field)
- returned identity field in this author process: unavailable
- git mutation: none
- live services/network/wallet/K/proving: none
- canonical accounting/store mutations: none (isolated fixture stores only)
- source approval: not claimed
- live admission: not claimed

## Summary

This author continued the same production-executor capability after GPT-6 Astra high REQUEST_CHANGES on G1-01..G1-13 (inheriting R2/P/TD). Repairs stayed in this worktree. Shared `/home/charl/Moriarty` was not edited. Independent probe trees under `/tmp/moriarty-production-grok-r1-independent-probes/` were not edited or reused as fixture roots.

The isolated path remains `cli.cmd_run` → `runner.execute` → Foreman strong launcher → `loan_executor.main` → AF_UNIX handshake/progress → accounting consumer → timer/prover/unit → collector → cleanup → `moriarty.loan-process-result/1`.

Passing author gates do not waive remaining producer, live-instance, funding, native-proof, or Preview-acceptance gaps. Empirical positive controls do not prove every fault path.

## Model identity

Requested: grok-4.6 at high effort.

Independent review recorded terminal identity `grok-4.6-build`. Outer harness metadata may still say grok-4.5 as a stale default. This author process did not receive a separate model-identity field and does not self-certify.

## Gate

Command: `bash /tmp/moriarty-production-switch-grok-20260913/gate.sh`

Log: `/tmp/moriarty-production-r3-grok-20260913/full-gate-g1.txt` SHA256 `01dde6c8e433c6da64e0e11a830e87d36a0d753f8492b4e1b72c853324eadc03`

Observed:

- Plugin pytest: 376 passed, 2 deselected, 241 subtests, 141.98s
- Wrapper reaper: 2 passed in 1.50s. Child -9, wrapper 124, child -9, test process 0. Original PID-absence assertions intact.
- JS executor: 123 passed, 0 failed, 1867 ms
- GATE_EXIT: 0

The two deselected names are the unchanged wrapper-context split. No other exclusions.

Focused pre-gate (excluding wrapper context): 185 passed, 132 subtests, 129.48s.

## Hook pins

Runtime digest (SHA256 of compact sorted JSON of relative `.py` hashes under `scripts/moriarty_dev`): `94937b8b9e53b9bb1116a7c33377202c35996ea3a44bf5288d03a8e452993b7d`

Four hook command pins in `plugins/moriarty-dev/hooks/hooks.json` were rewritten to that digest. Immutable tree staged at `~/.local/share/moriarty-dev/runtimes/94937b8b9e53b9bb1116a7c33377202c35996ea3a44bf5288d03a8e452993b7d/` without global activation.

## G1 dispositions

| ID | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| G1-01 | CRITICAL | repaired in isolated reader | `_validate_evidence_contents` recomputes startup identity, comparison digest/seed, timer active/NextElapse/Accuracy, classify_main_exit, stop returnCode type, containment contained+commands. Test: `ResultReaderG1.test_g1_01_one_field_negatives_are_rejected` (6 subtests). Result-reader probes, not six CLI launches. |
| G1-02 | HIGH | repaired | REFUSED closes only with acknowledged `contained` progress plus complete containment and no owners. Missing progress is unresolved. Test: `test_g1_02_refused_without_contained_progress_stays_unresolved`. |
| G1-03 | CRITICAL | repaired (supervisor is the bound) | `supervise_absolute` binds CLOCK_MONOTONIC cutoff before financial exec; late entry 64; SIGKILL of the child process group at cutoff returns 137. RuntimeMaxSec is remaining seconds, defence in depth only. Formula counterexample now100.950/cut1606/activation102.050 still yields runtime 1505 and would overrun without the supervisor. Timer NextElapse tolerance 1s and AccuracyUSec ≤1s. Tests: `AbsoluteSupervisor.*`. Synthetic formula is not a historical timing observation. |
| G1-04 | HIGH | repaired in consumer path | Remaining time is computed immediately before lock, write, and rename against the authenticated deadline. Release uses outerStart+cleanupSeconds. Cleanup intent carries `cleanupDeadlineMonotonic`; detached main does not grant a fresh now+bound. Test: `test_g1_04_past_deadline_grants_no_new_write_or_lock_budget`. Full syscall stall matrix was not rerun. |
| G1-05 | CRITICAL | repaired in consumer | SDK closure file must exist and its files map must hash. Parent evidence requires canonical input paths/mode/hashes, snapshot files, and a containment observation file. Finality requires network/protocol/genesis. Pending-use requires two prior allocations, intervening swaps, persistence, finalityRef. Test: `test_g1_05_missing_sdk_closure_file_is_invalid`. Full producer observer is still pending. |
| G1-06 | HIGH | repaired in Python consumer | Python rejects wrong firstAddress, directory overlap, uint128 dustFee. Selected need 0 only after contained loan-initialize mint receipt binds source/build/program/domain/native asset/unit. Absent derivation is unavailable, not an invented positive minimum. Native schema unchanged. Test: `test_g1_06_python_rejects_the_native_malformed_plans`. |
| G1-07 | CRITICAL | repaired in consumer | Master claim requires unique debit IDs, counterpartSha256, 125s financial-preflight debit, and mc02ApplicationMarker. History matches event kind to receipt outcome. records remaining shares those debit/preflight/marker checks. Test: `test_g1_07_missing_preflight_counterpart_or_marker_is_claim_invalid`. Full producer remains pending; consumer evidence is not optional. |
| G1-08 | CRITICAL | repaired | Live `cmd_run` authenticates db_path against `get_db_path(repo)`, ignores caller history_reader, and constructs canonical history internally. Test: `test_g1_08_live_cmd_run_rejects_caller_history_and_alternate_db`. |
| G1-09 | HIGH | repaired | PROCESS_UNKNOWN allows known raw exit with unknown operational code. PROCESS_FAILED no longer requires terminalEvidencePersisted to accept the diagnostic. Unknown startup with a well-formed observed ID is rejected if the result does not claim it; authentic observed IDs are retained on STARTUP_INVALID. Test: `test_g1_09_known_main_with_stop_failure_is_a_validated_unknown`, `test_td_03_unknown_startup_retains_observed_invocation_id`. |
| G1-10 | HIGH | repaired | Bounded carry buffer, complete newline JSON, closed phase order, leftover fragment deny. Test: `test_g1_10_split_coalesced_utf8_oversize_and_foreign_records`. Explains lost ack, not historical child3/status-null. |
| G1-11 | HIGH | repaired | Parser/shape failures at show boundaries become named unknown/refused. Malformed startup after resource start persists a result. Test: `test_g1_11_malformed_startup_show_reaches_durable_result`. |
| G1-12 | HIGH | repaired | Live branch requires exact `complete-i2-source-only`. Historical branch keeps `complete-design-subset-only`. Test: `test_g1_12_live_acceptance_rejects_historical_design_status`. |
| G1-13 | CRITICAL | repaired | Exact LIVE_SCOPE and standalone source/spec/resource tokens ingest as source_review and do not clear operational findings. Historical DESIGN_SCOPE result_review still clears named findings (existing execution tests). Test: `test_g1_13_source_review_does_not_clear_runtime_failures`. |

## TD dispositions

| ID | Disposition | Evidence |
| --- | --- | --- |
| TD-01 | repaired | After-startup parent-loss waits on durable `unit-ownership.json`, not fake unit-file existence. Separate pre-ownership loss (`lose_after=startup`) does not kill an unowned unit and keeps runtime-held ownership. Tests: `test_parent_loss_after_startup_yields_owned_cleanup_and_unknown`, `test_parent_lost_before_unit_ownership_does_not_kill_an_unowned_unit`. Independent post-durable control previously PASSED; this replaces the fake-unit-file barrier. |
| TD-02 | repaired as production timing | Absolute supervisor before financial work. One-second RuntimeMax reserve is not treated as a bound. Independent cleanup +1620, cleanup/result/ack +1740, outer 1800/grace 5/prepaid 1835 unchanged. Tests: `AbsoluteSupervisor.*`. Formula reproduction is synthetic, not historical. |
| TD-03 | repaired | Authentic 32-hex InvocationID retained on STARTUP_INVALID; parent reader accepts matching unknown diagnostics and rejects foreign/nonhex/all-zero/unclaimed IDs. Test: `test_td_03_unknown_startup_retains_observed_invocation_id`. |
| TD-04 | repaired | Same as G1-10. Stdout is never used as containment acknowledgment. |
| TD-05 | bounded worker drain added; no historical timeout causality | `run_bounded` drains the pipe nonblocking during wait. 200000-byte payload completed. Stalled worker killed at bound. Sibling-ancestry rejection now uses a separate process, not a thread across forks. 20k previously succeeded; this does not prove historical CLI timeouts. Tests: `test_td_05_payload_beyond_pipe_capacity_drains_and_stalled_worker_is_killed`, `test_sibling_process_outside_the_launcher_is_denied`. |
| TD-06 | repaired in CLI JSON | Isolated CLI JSON includes `loan.problem`. Test: `test_td_06_cli_json_retains_the_reader_problem`. Fixture-only; no private/real data; no rewritten historical logs. |

## R2 and P dispositions (inherited, still mandatory)

R2-01..R2-13 remain covered by `tests/loan_executor/test_r2_findings.py` plus the G1 reader/accounting/timer tightenings. R2-02 now additionally requires contained progress to close REFUSED. R2-08 NextElapse window is 1s with AccuracyUSec, and offset 4s is refused. R2-05/R2-12 consumer evidence is stricter (SDK file, parent observation, remaining write budgets).

P01..P12 remain covered by loan_executor suites. Live labels stay SP05.3 / i2 / MC02 / `finalized-financial-settlement`. Financial acceptance stays pending.

## Original P/R3 notes that stay true

- Financial plan schema `moriarty.preview-financial-launch/1` is mandatory.
- `grossByLogicalAsset` is a spending ceiling, not funding need.
- Authenticated loan initialize mint may require 0 selected tokens; absent mint derivation is unavailable.
- DUST uses plan `dustFee` with unit SPECK.
- Closed plans and immutable authority were not replaced with a new permission framework.

## Unresolved limitations

- Full accounting producer (predebit, live observer, master mint) remains pending.
- Real instance, funding, native proof, financial acceptance, and Preview readiness remain open.
- Status CLI still reports SP01 stale bindings, missing current-accounting, resource-live-state-unavailable.
- No broad syscall fault matrix was rerun.
- Peripheral/legacy paths were not exhaustively empirically exercised.
- Positive isolated controls do not prove live admission.
- Synthetic formula/probe is not a historical timing observation.
- Independent `/tmp` probe artifacts were not treated as a coherent final candidate.

## Source / test / hook manifest (current bytes)

43-path map SHA256: `cd5772c1ad43ba25389cbe68db3540f413420485f4e136bdf103d756fa780c18`

Changed versus frozen `68f879d2…` (15 files): hooks.json, accounting.py, cli.py, loan_cleanup.py, loan_executor.py, exit_retention.py, records.py, runner.py, store.py, fake_services.py, harness.py, test_executor_phases.py, test_loan_cleanup.py, test_r2_findings.py, test_runner_handoff.py.

New isolated regressions: `plugins/moriarty-dev/tests/loan_executor/test_g1_findings.py` SHA256 `4d939fba33dd4de8378ede58ef60846752c4627aba61c633e6eb22b3d9344b50`.

Current hashes:

```
experiments/moriarty-midnight-financial/ledger/indexed-owner.mjs 7164ccf41de2bd611203a58f73efea440f7e71526b837d2c328d1b46a3345d2d
experiments/moriarty-midnight-financial/ledger/preview-launch.mjs 61e3dc84b495285d8d4ca2460744f802b3123326c601d0e4f86ad5ea585e7dff
experiments/moriarty-midnight-financial/ledger/providers.mjs 40c0a894f8176855ae832837c3447c4d71aa357e6159532c0387a35986017c74
openspec/changes/afk-live-financial-execution/specs/afk-live-accounting/spec.md 429391810e73c3bb40a93253307a0de0ed65f71064049b1744ce8da8070c54dd
openspec/changes/afk-live-financial-execution/specs/afk-live-bindings/spec.md d0f478d511ceebcbe9214ce28cd1e8ba0e31a88a147175b38491594846b6d8e8
openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md f99bae8e639e7955fdb84a6fbf791b71fb5be3914a2df6f9a02da3cc1b2792f5
plugins/moriarty-dev/hooks/hooks.json 729bc77b49eed1a1d083817a2c064e7ac1143a7306d5d702ef5f066b38e5a14f
plugins/moriarty-dev/scripts/moriarty_dev/__init__.py f39f3bfc8060c098f28f8595d295ee282396a8a3f1c233e2295108e2fc128c06
plugins/moriarty-dev/scripts/moriarty_dev/accounting.py 8a510bfb1ff6fc36bb2cbd991efb73e2c2ab94b7acf5e11d927561fd4f32da20
plugins/moriarty-dev/scripts/moriarty_dev/bootstrap.py 957b5baa57c3a930fcb9620b9e073daa9cb3c7116b5f3904562fc4b004aa6009
plugins/moriarty-dev/scripts/moriarty_dev/cli.py d548cedca9f2d30bf98b9ecfc768c56d79103188226de6eae8cd6f66093b3ad8
plugins/moriarty-dev/scripts/moriarty_dev/compatibility.py 2e18c10fb78d3e9fbe1f0ab923d81871428c4aedf7fcc86d89b6ddcebdcd81e9
plugins/moriarty-dev/scripts/moriarty_dev/deployment.py ec0a9c2f29951d2c258a81d132333551922f4567f669aa0fb0bf4af6d49f6a9f
plugins/moriarty-dev/scripts/moriarty_dev/hook.py c185d2f0344599784c28aa52864971e9c336ca905bafeb09ba2a2d2db4e38084
plugins/moriarty-dev/scripts/moriarty_dev/hook_protocol.py 1cf8edcca04cccfc52ec8dad336b4200b96b7b0dbf5e5b6e7073d476443aaba8
plugins/moriarty-dev/scripts/moriarty_dev/loan_cleanup.py be3ee7c69aba32a36716c4c2bbb4bec42dd8e2f2aa9caecb3def60cdbf00d463
plugins/moriarty-dev/scripts/moriarty_dev/loan_executor.py f78e6143c6f4a2668a09c6f8b8b55a49f481b459d11462f06ee1ab405b0fd229
plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/__init__.py 5f5e2408b9f3bf8cc4c00963cd779e178e33a0bd6914163c81b238f92c84d51a
plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/cli.py e2780980962d6c9fe092810ade02c7e6b88ced043f516897c6a72a73f84a4bb8
plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/exit_retention.py ccb093fa2a3d0977c0318523632c716ac485fff842336f78348feae781f94689
plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/launch_contract.py 477dace598365dd530d419289273be2f8caefb8e28106a6c84690012b83b633a
plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/loan_exit_operator.py 61cde6a813c242c1f8bc88fbdfc88448691cd434af9ba9b3e8f0c0e1aa68253b
plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/terminal_predicate.py b040629fd83ba25f385855da42ab0c457bc065b46d9df93f10199b6397ebeb81
plugins/moriarty-dev/scripts/moriarty_dev/notifications.py ec96e339cc5a29e0b611be8b6c2404c09910fa0712d319980dfc9b55ed608e6c
plugins/moriarty-dev/scripts/moriarty_dev/policy.py 88725583618c50684ac00fd7e7219d065a21adc3f8416b1e85f41f10d534c55b
plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.build.md e873d917c4d5a2de4b257bfdf05ef08b54072828dac63347ee2a3fbf99bf1565
plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c d0b08d72d5fa8d1d85a4b1d6f6c43e26b07f4615f0ca0e76126407157cf11691
plugins/moriarty-dev/scripts/moriarty_dev/records.py 26a03e398c8a9d5f5c9c9fd024fee3f9d01e25caa9ebe5b9fbe941f6dcbb03c7
plugins/moriarty-dev/scripts/moriarty_dev/runner.py c3df3fac7630db3ed1d9fbd15f790c88caa467879ce00e2c1e25b2844558814f
plugins/moriarty-dev/scripts/moriarty_dev/store.py 4814fcc6bce6367426e66dc2814216772019c2e5e56d4497c77ef2ac3945e89f
plugins/moriarty-dev/tests/loan_executor/fake_services.py 9462177cc262f8ec0c3c9a78b2f12183f3ba142e0b7ccbfbedce5a52516e493d
plugins/moriarty-dev/tests/loan_executor/harness.py 9224758a075c9c82540652e446065f8dae27c59de7e94e61914487c1869c9b53
plugins/moriarty-dev/tests/loan_executor/test_cli_consumer.py 52780122b3ab6ed4773803164ab8bd35b3acb8ef075faadc3b8f2c02b468bd82
plugins/moriarty-dev/tests/loan_executor/test_consumer_exists.py e37b928b5ebf74f5db39d628d7f5e80bdd36d3ae7406138f7a28012300b0b195
plugins/moriarty-dev/tests/loan_executor/test_executor_phases.py e416d3d5e7257177f7329d5d2c236186d67b61e4e0a0f3062648d254c13335a1
plugins/moriarty-dev/tests/loan_executor/test_installed_collector.py 925f218c0c717a1f0c1fb14b0796d4751a54909f476d4e5d0c056fa7c4f0d3ab
plugins/moriarty-dev/tests/loan_executor/test_loan_cleanup.py fd666fea31829084296ca4fa3dcb0ac45cdcf68420d303ebf712321927ee34dc
plugins/moriarty-dev/tests/loan_executor/test_r2_findings.py ff79e7975f04ae84f2fa0070cab5af02bc44288f8f535d9f12590321ef7a2d06
plugins/moriarty-dev/tests/loan_executor/test_runner_handoff.py 74c8f852494f26e97eb309d9fec61d1e729234abd5ad642827ac6f4d774fa4b5
plugins/moriarty-dev/tests/test_policy.py a9b8f1d15502b9c26dc57b2d5b9895637bf0216571408a32b3181be3ed57a912
plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py 0cf96fb4c8407d0a2a5ad8a197548e5ccace4e4a819de0c81fe73990e452f90b
plugins/moriarty-dev/tests/test_runner.py 21702e33e7a681a9042966f10015ab39b6c797016290d7383cf64b248af4bac4
plugins/moriarty-dev/tests/test_store_connections.py f8ddd4cbfe798a2688545dcbb95e0c1fd11f5135b95db3e1018d542d155f5396
plugins/moriarty-dev/tests/loan_executor/test_g1_findings.py 4d939fba33dd4de8378ede58ef60846752c4627aba61c633e6eb22b3d9344b50
```

This report is isolated source completeness against the G1/TD/R2/P consumer findings as implemented and gated. It is not live admission, not producer completion, and not source approval.
