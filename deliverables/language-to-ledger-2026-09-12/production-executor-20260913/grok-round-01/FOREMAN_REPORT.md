# FOREMAN_REPORT

- run_id: moriarty-release-afk-20260913
- role: implement
- slug: production-executor
- branch: foreman/moriarty-release-afk-20260913/implement/production-executor
- worktree: /home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor
- base_sha: f7f40ce5e945fd9d7d2bd7a3c05ee3798331ff8e
- freeze: /tmp/moriarty-production-switch-grok-20260913/fable-partial/manifest.json
- freeze sourceMapSha256: 3d0c44518b6417be186ff4d0429026f23eea20b26de610061bc8eae248b9946d
- requested model: grok-4.6 high
- observed provider usage: grok-4.6-build from /tmp/moriarty-production-switch-grok-20260913/canary-receipt.json
- returned identity field in this author process: unavailable
- status: isolated source complete. Live admission, producer, and instance remain pending.
- git mutation: none

## Summary

This author continued the Fable partial production executor. Valid Fable source stayed in place. R3 accounting and timer expectations were applied. The isolated path is `cmd_run` to `runner.execute` to installed `loan_executor.main` to collector, cleanup, result, and history.

The financial plan schema `moriarty.preview-financial-launch/1` is mandatory. `grossByLogicalAsset` is a spending ceiling. The loan initialize path admits selected-asset need 0. DUST uses the plan `dustFee` ceiling.

No live admission, producer, Docker daemon, systemd user manager, wallet, network, or Preview settlement is claimed.

## Model identity

Requested setting: grok-4.6 at high effort.

Canary receipt observed provider usage `grok-4.6-build` for requested `grok-4.6`. This author process did not receive a separate model-identity field. This report does not self-certify a hidden identity.

## R2 findings

| ID | Outcome | Proof |
| --- | --- | --- |
| R2-01 | verified | Result reader requires unique evidence, strict scalar types, and closed status combinations. Empty evidence cannot succeed. Test: `ResultReader.test_r2_01_success_requires_real_evidence_and_strict_scalars`. |
| R2-02 | verified | Null-identity REFUSED stays unresolved. Test: `test_r2_02_null_identity_refusal_stays_unresolved`. |
| R2-03 | verified | Failed durable release keeps occupancy and CLI 4. Test: `OwnershipRelease.test_release_refusal_keeps_reservation_active_and_cli_four`. |
| R2-04 | verified | Failed comparison or timer-armed write refuses before start. Test: `RequiredWrites.test_missing_required_setup_evidence_refuses_before_start`. |
| R2-05 | verified | Parent evidence, pending-use content, selected asset/unit, mint-path need 0, and DUST fee ceiling are checked. Gross ceiling is not the funding need. Tests: `AccountingEvidence.test_r2_05_observation_semantics` and `test_r2_05_history_contents_and_grant_invariants`. |
| R2-06 | verified | Observation age is rechecked at financial startup. Test: `test_r2_06_freshness_rechecked_at_financial_startup`. |
| R2-07 | verified | Cleanup matches receipt identities and refuses replacements. Tests: `CleanupIdentity.test_r2_07_*`. |
| R2-08 | verified | Timer Unit and NextElapseUSecMonotonic are checked against the absolute cutoff. Shared cleanup budget does not restart. Tests: `test_r2_08_*`. |
| R2-09 | verified | Nonregular result path is opened no-follow and nonblocking. The FIFO case runs in a 5s child. Test: `test_r2_09_nonregular_result_path_is_bounded_unresolved`. |
| R2-10 | verified | Full prover argv and mounts are pinned before start. Test: `test_r2_10_prover_command_is_pinned_by_reviewed_authority`. |
| R2-11 | verified | Invalid running rows are terminal unknown. Collector writes RETENTION_FAILED. Test: `test_r2_11_*`. Parent reader accepts PROCESS_UNKNOWN when terminal identity is invalid. |
| R2-12 | verified | Budget lock serializes handover. Supervised writes are bounded. Test: `test_r2_12_budget_lock_and_supervised_master_writes`. |
| R2-13 | verified | Isolated `cmd_run` uses SP05.3, i2, MC02, and finalized-financial-settlement. Near misses reject. Live relabel of verify/review/report cannot skip primary or retry. Tests: `LiveAdmission.*`. Financial acceptance stays pending. |

## P01-P12

P01-P12 remain covered by the existing loan_executor suites plus the R2 regressions. The live fixture is now the default isolated profile. Historical `sp01-verify` stays available.

## R3 repairs in this author turn

1. Financial plan is mandatory at parent and child validators.
2. Selected loan token need is 0 for the initialize mint path. DUST available must meet `dustFee`.
3. `processExit` must be exact int 0. Float 0 is invalid.
4. Pending-use content must show verified-success, class pending-use, and no competing spend.
5. Live profile kinds keep history, primary, and retry checks.
6. FIFO reader runs in a short child.
7. RuntimeMaxSec keeps one spare second inside the absolute bootstrap cutoff.
8. Unknown results without a loaded invocation id do not demand startup identity match.

## Gate

Command: `bash /tmp/moriarty-production-switch-grok-20260913/gate.sh`

Log: `/tmp/moriarty-production-r3-grok-20260913/full-gate-2.txt`

Results:

- Plugin pytest: 356 passed, 2 deselected, 7 warnings, 235 subtests, 136.10s
- Wrapper reaper: 2 passed in 1.58s. Child -9, wrapper 124, child -9, test 0.
- JS executor: 123 passed, 0 failed, 1915 ms
- GATE_EXIT: 0

The two deselected names are the wrapper-context tests. The reaper ran those two unchanged.

Focused R2 log: `/tmp/moriarty-production-r3-grok-20260913/focused-r2-after-repair.txt` (21 passed, then FIFO added). Later combined CLI/handoff/R2: `/tmp/moriarty-production-r3-grok-20260913/cli-handoff-r2.txt` (40 passed).

Earlier incomplete red hangs are not this gate. First gate attempt failed two load-sensitive cases. Those are repaired. The second gate passed.

## Hooks

Python tree digest: `15c309a7f2654e56d570bd90ee340f0ff617232a253ad5d7d8f0265ab7f67965`

All four pins in `plugins/moriarty-dev/hooks/hooks.json` match. File count is 21.

Staged runtime: `~/.local/share/moriarty-dev/runtimes/15c309a7f2654e56d570bd90ee340f0ff617232a253ad5d7d8f0265ab7f67965/`

Staged digest equals the pin. Earlier runtimes were not overwritten. Hooks were not activated globally.

## Manifest (SHA-256)

- plugins/moriarty-dev/scripts/moriarty_dev/accounting.py d49f98dade8e322f1a3781e9eb992ff45fb2cb7b9e844e7fb59ab69441915411
- plugins/moriarty-dev/scripts/moriarty_dev/cli.py 7442f13f7d38c399ab018933d6950b1db371e76b1fa3fe2df312f3a8961eedd4
- plugins/moriarty-dev/scripts/moriarty_dev/loan_cleanup.py dba2f29e201aa82fe3e594b092f4ed21694fe2debe8beaf0ec80595ffa6df0bc
- plugins/moriarty-dev/scripts/moriarty_dev/loan_executor.py d547298eba9c94d6bf698963fa506978d73698f93ded96fa59062c0de27bef84
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/__init__.py 5f5e2408b9f3bf8cc4c00963cd779e178e33a0bd6914163c81b238f92c84d51a
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/cli.py e2780980962d6c9fe092810ade02c7e6b88ced043f516897c6a72a73f84a4bb8
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/exit_retention.py f7d6c6676a91f6b1e2f9dad65be0141bc96f3cbecf67e3153cfdccc76c93d54b
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/launch_contract.py 477dace598365dd530d419289273be2f8caefb8e28106a6c84690012b83b633a
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/loan_exit_operator.py 61cde6a813c242c1f8bc88fbdfc88448691cd434af9ba9b3e8f0c0e1aa68253b
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/terminal_predicate.py b040629fd83ba25f385855da42ab0c457bc065b46d9df93f10199b6397ebeb81
- plugins/moriarty-dev/scripts/moriarty_dev/records.py fc16c294a8cee323683e584f7e4edcfc4391747054644afd330d999e8ee0dfce
- plugins/moriarty-dev/scripts/moriarty_dev/runner.py d920be70ffcc0220a732001ee2d55fd81843bae468dd25da0512e62df18925c6
- plugins/moriarty-dev/scripts/moriarty_dev/store.py 834881df85a26d84d578e2e055aaa8f33eef85e2daf9c85ea1f773c022b1fa8e
- plugins/moriarty-dev/scripts/moriarty_dev/policy.py 88725583618c50684ac00fd7e7219d065a21adc3f8416b1e85f41f10d534c55b
- plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c d0b08d72d5fa8d1d85a4b1d6f6c43e26b07f4615f0ca0e76126407157cf11691
- plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.build.md e873d917c4d5a2de4b257bfdf05ef08b54072828dac63347ee2a3fbf99bf1565
- plugins/moriarty-dev/hooks/hooks.json 0fc311652db1e14a27336788a1a36875283f2c2c2485f78b6c4c247de8bee2b6
- plugins/moriarty-dev/tests/loan_executor/harness.py 5ce36a46467a49239ee38fbde56f8b1642d8e51bb0969bacc8d667c8a2ab3fc6
- plugins/moriarty-dev/tests/loan_executor/fake_services.py 059059c96e44656b89e310fb9bf0b7c8cf0d4f1a91c5f1eb2245a5219fa96ce7
- plugins/moriarty-dev/tests/loan_executor/test_r2_findings.py 6972f233a0f849a2424856e556c73489cb56d4e5f10918df0a5dc86d52f1c82a
- plugins/moriarty-dev/tests/loan_executor/test_cli_consumer.py 52780122b3ab6ed4773803164ab8bd35b3acb8ef075faadc3b8f2c02b468bd82
- plugins/moriarty-dev/tests/loan_executor/test_runner_handoff.py 12cea91e25478a5d9b4b496e6d3b353f4e6d430ce499722adfaa32fb7676380d
- plugins/moriarty-dev/tests/loan_executor/test_executor_phases.py 42ffd55bdbd960277e402190f1bc6e34112450863a7780649d8e1d906fe96866
- plugins/moriarty-dev/tests/loan_executor/test_installed_collector.py 925f218c0c717a1f0c1fb14b0796d4751a54909f476d4e5d0c056fa7c4f0d3ab
- plugins/moriarty-dev/tests/loan_executor/test_loan_cleanup.py 2ee92abb1fddda12b5a52030b193a84e7f403f7ea24c353c1afb800ae7e7f2b5
- plugins/moriarty-dev/tests/loan_executor/test_consumer_exists.py e37b928b5ebf74f5db39d628d7f5e80bdd36d3ae7406138f7a28012300b0b195
- plugins/moriarty-dev/tests/test_runner.py 21702e33e7a681a9042966f10015ab39b6c797016290d7383cf64b248af4bac4
- plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py 0cf96fb4c8407d0a2a5ad8a197548e5ccace4e4a819de0c81fe73990e452f90b

Normative specs unchanged: afk-loan-executor f99bae8e639e7955fdb84a6fbf791b71fb5be3914a2df6f9a02da3cc1b2792f5, afk-live-accounting 429391810e73c3bb40a93253307a0de0ed65f71064049b1744ce8da8070c54dd, afk-live-bindings d0f478d511ceebcbe9214ce28cd1e8ba0e31a88a147175b38491594846b6d8e8.

## Limitations

Isolated tests only. Fake systemctl, systemd-run, and docker stand in for the user manager and Docker. Process success here does not establish Docker, Preview, or financial settlement.

The verifier accepts a complete isolated producer-shaped receipt. It does not produce grants, predebit, charged observer, or live instance evidence.

Parent-evidence and pending-use layouts are consumer interpretations of required facts. They are not new fields on closed context, observation, correspondence, or financial-plan schemas.

Indexed-owner conversion is checked as closed plan role hashes and distinct actors. The JS codec is not invoked from the Python verifier.

Startup status still reports binding-input-stale, candidate-input-stale, current-accounting-missing, and resource-live-state-unavailable.

## Unresolved blockers for live admission

1. Full accounting producer: predebit gate, preflight reserve, charged observer, debit commit, grant application.
2. Concrete instance evidence: grant G/N, votes, context, token adoption, history of live allocations, never-started prover container.
3. Live reachability under strong mode for user manager and Docker, then Preview result audit.
4. Hook activation of digest 15c309a7... is a root decision.

Exact-source isolated readiness is not release success and is not real financial process success.
