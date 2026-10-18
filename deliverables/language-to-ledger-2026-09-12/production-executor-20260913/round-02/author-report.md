# FOREMAN_REPORT

- run_id: moriarty-release-afk-20260913
- role: implement
- slug: production-executor
- branch: foreman/moriarty-release-afk-20260913/implement/production-executor
- worktree: /home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor
- base_sha: f7f40ce5e945fd9d7d2bd7a3c05ee3798331ff8e
- author: Fable 5.1 (claude-fable-5-1), medium effort, two bounded author rounds in one session; no delegation, no Git mutation
- status: complete (isolated callable source readiness only; live admission, producer and instance remain pending)

## Summary

The installed Python production consumer now exists and runs end to end on isolated fixtures through real validators: `cli.py::cmd_run` -> `runner.execute` (executor profile) -> pinned Foreman launcher (strong containment) -> installed `loan_executor.main --plan --sha256 --admission` -> real AF_UNIX invocation handshake -> `accounting.validate_executor_context` -> prover control and container pre-validation -> timer arming -> single Docker start and financial unit launch -> installed `loan_exit_retention` collector -> identity-safe `loan_cleanup` containment -> durable `moriarty.loan-process-result/1` -> runner result read -> reservation/history mapping. Every service, Docker and wallet I/O is a fake transport in tests; every security predicate, parser, collector, persistence ordering and classification is production code.

No production readiness, live admission, grant, store adoption, Docker, systemd, wallet or Preview acceptance is claimed. The full accounting producer (predebit gate, preflight reservation, charged observer, debit commit, grant application) remains a named pending dependency. Only the specified runtime-claim tail (master one-shot claim, correspondence and wallet-ownership handover under the held OS lock) is implemented in the runner; no code path can create current authority from nothing.

## Delivered scope (P01-P12 mapping)

| Family | Where verified | Result |
| --- | --- | --- |
| P01 import/help inert; invalid inputs reject before any mutation; one literal substitution | test_executor_phases::InputRejection, CompletePath::test_runtime_plan_substitutes_only_the_derived_deadline | pass |
| P02 real cmd_run/runner/script with real AF_UNIX channel | test_cli_consumer::test_p09 (real Foreman strong launcher), test_runner_handoff::test_complete_handoff | pass; launches exactly once |
| P03 forged nonce, wrong authority, stale store event, sibling/wrong launcher, second handshake, alternate/restored SQLite | test_runner_handoff (nonce, second handshake, sibling ancestry, stale event, boot mismatch), AuthorityChain::test_alternate_or_restored_sqlite | pass; no launch, debit stays claimed |
| P04 parent loss before/after startup; hung store append, writer/fsync | ParentLoss (3 tests), Handoff::test_parent_loss_after_startup (runner SIGKILLed in a subprocess), Handoff::test_failed_or_hung_invocation_append | pass; before startup refuses, after startup owned cleanup + PARENT_LOST, hung workers killed at bound |
| P05 entry +50/+120, short block, clock jumps, observation age 119/121 | DeadlineSchedule (4 tests), FaultTable::test_collection_cutoff | pass; absolute cutoffs unchanged, no refresh/refund |
| P06 exact running polls, exit 0, 1..255, signal 1..64, malformed/unloaded/transitional/default/mismatched | TerminalPredicate::test_closed_classification_table (all 255 + 64 codes), FaultTable (nonzero/signal/unknown/show failure) | pass; wrapper/cleanup 0 cannot promote a main failure |
| P07 every write/fsync boundary, preexisting artifacts | FaultTable::test_terminal_write_failures_and_preexisting_artifacts (10 artifacts), test_stop_failure, hung writer | pass; no evidence-driven stop before durable terminal, no retry of the exclusive path |
| P08 prover config/control/prestart failures, start then receipt loss, replacement generation, delayed wrapper entry | FaultTable::test_prover_prevalidation_failures, docker start failure, ownership receipt loss, replacement generation; CompletePath::test_prover_control (real static wrapper honours the written control and refuses late entry with 64) | pass |
| P09 child 0 + valid PROCESS_SUCCESS | test_cli_consumer::test_p09 | CLI 0, finished reservation, digests in history, pending/false |
| P10 child 1 + PROCESS_FAILED + containment; child 2 + REFUSED | test_cli_consumer::test_p10 (two tests), policy denial stays exit 2 | CLI 4, failed reservation, distinct refused_before_start event |
| P11 child 3; child 0 with missing/forged result; exit/status mismatch; known failure with unresolved containment | test_cli_consumer::test_p11 (four tests) | CLI 4 runner_unresolved / loan_failure_with_unresolved_ownership, reservation active, ownership runtime-held |
| P12 authentic fixture accepted; wrong projection/vote/history/context/claim denied; own prepaid equality; contender; interrupted generation | AuthorityChain (8 tests) | pass; S=G accepted for own invocation, c never subtracted twice |

## Findings

1. **First-round gate failures (8) were all reproduced and corrected without deleting tests.** Retained log: /tmp/moriarty-production-round1-evidence/partial-full-gate.log (8 failed, 310 passed, 2 deselected, 195 subtests, 39.19 s). Causes and repairs: (a) collection-cutoff test injected a late entry instead of a mid-collection clock jump, so the executor correctly refused at +120; the harness now jumps the injected clock at the first terminal observation and its injected sleep advances that clock. (b) The collector persisted through its own `durable_exclusive_save`, so injected terminal/stop write failures never reached it; `retain_exit_evidence` now accepts a `save` I/O callable (ordering and exclusivity stay inside the collector) and the executor routes it through the injected supervised writer. (c) Three AuthorityChain tests reserved the same charge twice per test (UNIQUE runner_claims) or mutated immutable files before the handover; restructured to one reservation per test and mutation after payload creation. (d) The cleanup test relied on a show-timeout flag that now applies only to terminal shows; a separate identity-show fault flag was added to the fake. (e) A preexisting collector artifact is refused by plan validation as a reused evidence directory; the test now asserts that refusal.
2. **Second-round defects found by the new end-to-end tests.** The runner's private channel directory was named `runtime-*`, colliding with the executor's `runtime-financial-plan.json` prefix; renamed to `channel-*` and the plan's reuse check permits only that 0700 same-owner directory. AF_UNIX addresses exceeded 108 bytes under deep evidence directories; both sides bind/connect through a directory descriptor (`/proc/self/fd/N/channel.sock`) while the socket file stays inside the evidence tree. Custom test children must do the same. The runner waited on its listener after a child exited without connecting; it now drops the listener once the child is gone. A REFUSED result written before the exchange completed carries null invocation-bound identities; the runner accepts that only for status REFUSED with exit 2, and a failed durable invocation append forces `unresolved` regardless of the child's refusal.
3. **Vote identity routing.** The accounting spec names `claude-opus-5` and `gpt-6-astra`. Per the independent review notes, `accounting.VOTE_VENDORS` now admits two distinct identities from distinct vendors from a closed table that includes the historical pair and the current route (`claude-fable-5-1`, `gpt-6`, `gpt-6-high`). Historical votes are not relabelled; routing alone grants nothing. Tested: same-vendor pair and duplicated identity are refused, the current-route pair is admitted.
4. **Store integrity scare was a probe artifact.** During round one a probe process kept an unclosed `sqlite3.connect` (a `with` block does not close), and its later garbage collection released the WAL locks mid-run, so its own re-read reported "malformed". Fresh-process integrity checks after every CLI case pass; the harness now uses `closing()`. Not a product defect, but a real hazard for any harness holding connections across a run.
5. **Producer boundary is explicit.** `accounting.py` implements the verifier and the runtime-claim tail only. `runtime_handover` performs the specified one-shot transition (master row `unclaimed -> claimed` with the runtime reservation, correspondence reservation/master digest update, ownership `preflight-held -> runtime-held` under the held wallet OS lock) on records that must already exist; a second handover, a claimed row, a contender reservation or a master/correspondence mismatch refuses. Predebit gate, preflight reservation, funding observer, debit commit and grant-application marker are not implemented (see gaps).
6. **Persistence workers.** `run_bounded` forks a worker and SIGKILLs it at its bound; used for the runner's invocation append and every executor artifact write (exclusive, no-follow, file plus directory fsync). A killed worker is never durable; the test injects a stalled fsync and observes EVIDENCE_WRITE_FAILED within the bound. One pytest DeprecationWarning remains where a test uses a thread before fork (sibling-intruder test only).

## Evidence

Commands run from `plugins/moriarty-dev` unless noted; this shell itself runs inside Foreman strong containment (namespace PID 1 is the Foreman Node launcher), which is the known context for the two wrapper tests.

- Failing baseline before any repair: `python3 -m pytest -q tests/loan_executor/test_consumer_exists.py` -> 8 failed, 1 passed (retained at /tmp/moriarty-exec-failing-before.txt).
- Root baseline reference (independent notes): 243 passed / 109 subtests at f7f40ce5.
- Round-two full suite (block 1): `python3 -m pytest -q --no-header -p no:cacheprovider --deselect <two wrapper tests>` -> `2 failed, 335 passed, 1 warning, 197 subtests passed in 79.49s`. The two failures are exactly `test_control_deadline_kills_hanging_child_and_is_independent` and `test_dead_external_launcher_cannot_disable_wrapper_deadline` (the deselect was not honoured by this pytest invocation; both ran and failed with the documented zombie-adoption symptom). Log: /tmp/moriarty-production-round2-full-gate.log (copy in /tmp/moriarty-production-round2-evidence/).
- Round-two block 2: `python3 /tmp/moriarty-wrapper-test-reaper.py` (unchanged tests, no signals, adopted children reaped) -> `2 passed in 1.58s`, retained statuses child -9, orphan wrapper 124, child -9, test process 0. Log: /tmp/moriarty-production-round2-wrapper-reaper.log.
- Loan executor suites alone: `python3 -m pytest -q tests/loan_executor/` -> `94 passed, 1 warning, 88 subtests passed in 54.35s`. Log: /tmp/moriarty-production-round2-evidence/loan-executor-suites.log.
- Legacy runner/CLI regressions: tests/test_runner.py, tests/test_execution.py, tests/test_hooks.py, tests/test_host_adapter.py, tests/test_compatibility.py all pass inside the full run.
- Real strong launcher transport: `ForemanTransport::test_pinned_strong_launcher_inherits_the_channel_environment` passes (launcher sha 63b934958c6dd883a4420d1e33a48338366e58223929c23b47873a199a9ec322); test_cli_consumer runs the actual launcher for every CLI case; peer ancestry walks host PIDs to the launcher and a same-UID intruder outside the launcher tree is denied with PEER_ANCESTRY.
- Hook digest: all four occurrences in `plugins/moriarty-dev/hooks/hooks.json` replaced `0b40d8c3…4acb3` -> `a89ec2be27a453a8166036430e0bc8a303ddef2a3b68c23603e56e060bdfdc14` (compact sorted JSON over 21 relative `.py` paths). Runtime tree staged at `~/.local/share/moriarty-dev/runtimes/a89ec2be…dfdc14/` (full plugin layout, no `__pycache__`); staged inventory digest recomputed equal to the pin; the existing `0b40d8c3…` runtime was not touched; no global hook activation or other global config change.
- No JS interface changed; no JS suite was run for this candidate. No Docker, systemd, wallet, network, K, proving or live accounting action was executed; the canonical store received only the startup `status` administrative observation.

## Manifest (SHA-256)

Candidate files:

- plugins/moriarty-dev/scripts/moriarty_dev/accounting.py e0555c61f0242e18ab169403ed4155c1dbb414f3644cc0e4c886ea96005bf20d
- plugins/moriarty-dev/scripts/moriarty_dev/cli.py e34bb145fc5920e42ed5dd1f5048ca2bd8f01938c75c67e9bab991398cf135a6
- plugins/moriarty-dev/scripts/moriarty_dev/loan_cleanup.py 10f52fffbb9f6f2861a9202bd59e58bada5dbd4de9b1cb0a6c8fdf4ccb6b2dd9
- plugins/moriarty-dev/scripts/moriarty_dev/loan_executor.py db3130d9c42c75fbe0756cea291b97dc2a7911d00ba5ea7da6f679ef5b5765ed
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/__init__.py 5f5e2408b9f3bf8cc4c00963cd779e178e33a0bd6914163c81b238f92c84d51a
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/cli.py e2780980962d6c9fe092810ade02c7e6b88ced043f516897c6a72a73f84a4bb8
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/exit_retention.py d450b5ec39096cc07dd6153f2838e59831e4481944c2586ce4bac87d02ad26a7
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/launch_contract.py 2fd62de5b5d8c52747c148fccd483bfddbd84fe86b2c59179c63e1837000bc1a
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/loan_exit_operator.py 61cde6a813c242c1f8bc88fbdfc88448691cd434af9ba9b3e8f0c0e1aa68253b
- plugins/moriarty-dev/scripts/moriarty_dev/loan_exit_retention/terminal_predicate.py 77695bfc972c3b943e6a5ea0d4134411d0a42595df43b81bb973528417d442bd
- plugins/moriarty-dev/scripts/moriarty_dev/records.py bc1f10db3c4d78881bdd41d8598e92de6f6434c76ecaf7f1cdf16f48aee692e5
- plugins/moriarty-dev/scripts/moriarty_dev/runner.py 45ce20172ec94c48ebad120518833cbf1dba072ab16f29b63c901783e405cc3b
- plugins/moriarty-dev/scripts/moriarty_dev/store.py cceeda31a926170dee626f0a800c834ae30a0f0bad227791ca82c22b272aa4c4
- plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c d0b08d72d5fa8d1d85a4b1d6f6c43e26b07f4615f0ca0e76126407157cf11691 (unchanged, reused)
- plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.build.md e873d917c4d5a2de4b257bfdf05ef08b54072828dac63347ee2a3fbf99bf1565 (unchanged)
- plugins/moriarty-dev/hooks/hooks.json 4c53ce6d8dbcf99b5c835516128dc0106f27aaa8fb5ca2017703859d32c248c9
- plugins/moriarty-dev/tests/loan_executor/harness.py 3a132e239c412213ac48ab0b1e408ca11f63c8c5328ebdb104a09308b7b1fde0
- plugins/moriarty-dev/tests/loan_executor/fake_services.py 6c35347c5bff84ee77c26458d3c96fddac7f85ca0c953ec6e8eda1b0ffb2862e
- plugins/moriarty-dev/tests/loan_executor/test_consumer_exists.py e37b928b5ebf74f5db39d628d7f5e80bdd36d3ae7406138f7a28012300b0b195
- plugins/moriarty-dev/tests/loan_executor/test_installed_collector.py de4a7a5a2989232847f08a9fe0d522f507bcb784040a2f7c3bd0165d2be2d8f8
- plugins/moriarty-dev/tests/loan_executor/test_loan_cleanup.py 029d97c4f7409bedcd028f3633873fd7d111033d6f48aafcc2b10d5294dfd5fb
- plugins/moriarty-dev/tests/loan_executor/test_executor_phases.py 7ddba651b5c98154fa708bacfcb859ea46583ebf06abce6ebec7e30374d20f8d
- plugins/moriarty-dev/tests/loan_executor/test_runner_handoff.py be027648a8b0a2f2f98a071e0d2b9b13e363a225260d25feed78ac0021d1adff
- plugins/moriarty-dev/tests/loan_executor/test_cli_consumer.py 52780122b3ab6ed4773803164ab8bd35b3acb8ef075faadc3b8f2c02b468bd82
- plugins/moriarty-dev/tests/test_runner.py 21702e33e7a681a9042966f10015ab39b6c797016290d7383cf64b248af4bac4 (unchanged)
- plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py 0cf96fb4c8407d0a2a5ad8a197548e5ccace4e4a819de0c81fe73990e452f90b (unchanged)

Normative inputs: afk-loan-executor spec f99bae8e639e7955fdb84a6fbf791b71fb5be3914a2df6f9a02da3cc1b2792f5; afk-live-accounting 429391810e73c3bb40a93253307a0de0ed65f71064049b1744ce8da8070c54dd; afk-live-bindings d0f478d511ceebcbe9214ce28cd1e8ba0e31a88a147175b38491594846b6d8e8.

External pinned sources: foreman-launch.js 63b934958c6dd883a4420d1e33a48338366e58223929c23b47873a199a9ec322; launcher/src/platform.ts 8762290917352f97c13df8b22541e60eb9b8a128d3bbd0f577732357cbd42382; launcher/src/services.ts 00f33d68bb9361efdddbe3019ad86e77f77084810028250f055fe4c72f800e93; /tmp/moriarty-wrapper-test-reaper.py 9d3dd9f42f4cc57da1b13eb7fd130ac6791009bd9baea12387e93a3e71e4b8fe; /tmp/moriarty-wrapper-context-probe.py 2c97da91958a561c5cff5ab25bd15ffd7e92a488a0267ddca10c58089fcc638b; /tmp/moriarty-production-launcher-channel-probe.py cce68b65a389415a6dd89af75a09192d29d731f1162dab3c208ba2237143b504. Round-one 19-file snapshot /tmp/moriarty-production-round1-evidence/source-manifest.json is superseded by this manifest and left untouched.

## Limitations

- Isolated tests only. Fake `systemctl`, `systemd-run` and `docker` executables/state machines stand in for the user manager and Docker; no live unit, timer, container, wallet, network or canonical store was touched. Process success in these tests establishes nothing about Docker, Preview or financial settlement.
- The fixture registers the executor under the existing SP01 loan `verify` action of campaign `sp01-loan-swap-grok-01`, with the fixture's own amendment as the campaign's `resourceAmendment`. The intended live action `sp05-loan-execute-once` is not registered; that registration, the concrete amendment, projection, votes, context, history/token adoption and observation are separately reviewed instance operations.
- Container configuration checks are against a fake `docker inspect` document shaped from the spec (created/never-started, restart=no, no PidMode/privilege/CapAdd, memory, exactly two read-only mounts, wrapper entrypoint with control path). Real image/entrypoint inspection is item 3 work.
- Under this ForemanStrong shell the two C wrapper deadline/orphan tests fail by zombie adoption exactly as diagnosed; they pass unchanged under the scoped no-signal reaper. No assertion was weakened; the reaper is not a source change.
- `tests/test_records.py` cannot be collected on its own (pre-existing sys.path ordering); it passes inside the full run. Not changed.
- The `--deselect` node ids were ignored by this pytest invocation; the full-suite log therefore shows the two wrapper failures rather than 2 deselected. Counts: 335 passed, 2 failed, 197 subtests; plus the reaper block 2 passed.
- One DeprecationWarning (fork after threads) comes from the sibling-intruder test's helper thread; product code forks only from single-threaded processes.
- History verification requires representation and consistency of retained failed/unknown outcomes; it does not itself count attempts, which stays with the existing policy reader.
- A refusal that happens before the invocation exchange carries null runnerDigest/chargeId/reservationId/invocationSha256 in its REFUSED result; the runner accepts that only for exit 2 and status REFUSED.

## Remaining concrete gaps (all blocking live admission)

1. Full accounting producer per afk-live-accounting: authenticated discovery before the history reader, non-granting predebit gate, `store.reserve_preflight` and occupied-primary handover, grant first-application marker and quota-preserving master writer with exact Decimal C/D arithmetic (reconciliation says C=7, D=1 for the current master a12ca70b…), canonical debit-intent counterparts, 125-second charged nonpersisting funding observer (`ledger/observe-preview-funding.mjs`, absent) with independent parent evidence, current observation/correspondence production, provider reservation/submission guard binding. Independent cases A01-A11 in /tmp/moriarty-accounting-reconciliation-gpt6-20260913.json are specified, not executed.
2. Instance evidence: concrete grant G/N and bounded deadline, exact projection/votes from the current route, immutable execution context, canonical store token adoption against the recorded inventory (the live store has no token), history/pending-use reconciliation of loan01 -> swap01 -> loan-exit01 -> swap-exit01 and the three unresolved local `submitting` allocations, never-started prover container with the static wrapper as PID 1, `sp05-loan-execute-once` registration and successor SP01 binding for the two stale commitments.
3. Live reachability under strong mode (user manager, Docker), real container config inspection, and actual Docker then Preview result audits.
4. Hook activation: the new digest is pinned in source and the exact runtime tree is staged; activation of changed hooks is a root decision and was not performed.

## Open questions

- Whether root wants the current-route vote pair (`claude-fable-5-1` + `gpt-6`/`gpt-6-high`) expressed in the accounting spec text as well as in `accounting.VOTE_VENDORS`; the spec still names the historical pair only.
- Whether an early REFUSED result with null invocation identities should close the reservation as failed (current behaviour) or stay unresolved; the append-failure case already stays unresolved.
