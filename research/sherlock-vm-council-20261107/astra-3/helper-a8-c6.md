A8 needs a call against the replacement deployment and current keys. The dispatch receipt attempt exhausted its wall budget during **nonproving SDK preprocessing**. Its checker does not perform synthesis, keygen, proving, or native verification (`crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:1`). The timeout therefore supplies no native proving-performance measurement and demonstrates no verifier failure.

Paths below are relative to `/home/hoskinson/Projects/Moriarty`, inspected at `8fd34ed794eda7186c32495f715609ec54a9a297`. No builds, tests, or transactions were run. Referenced raw traces, `target/` artifacts, and the dispatch receipt directory are absent here. Retained run records, prose reports, and source facts are distinguished below.

1. Runner and terminology

`scripts/sherlock-next:2` delegates to `doctorwatson next`. That command reads stored results (`models/doctorwatson/factory/cli.py:23`). It selects the first predicate without a recorded pass (`models/doctorwatson/factory/status.py:34`). It does not establish freshness against current keys.

The stored first failure is A8 (`qa/sherlock/status.json:27`). A5–A7 retain October 25 passes despite the replacement A4 deployment (`qa/sherlock/status.json:358`). The handoff requires re-earning that chain (`docs/sherlock/HANDOFF-2026-10-26.md:7`).

Predicate C6 already passes (`qa/sherlock/status.json:340`). “C6 dispatch” instead names the physical representation work under U023. U023 blocks predicate C4 (`qa/sherlock/predicates.yaml:522`).

2. A8: evidence and recovery trap

The records distinguish historical success from the current obligation:

| Event | Retained outcome | Citation |
| --- | --- | --- |
| First October 25 submission | Exit 1, 460.951 seconds | `qa/sherlock/runs.jsonl:92` |
| Retried submission | Exit 0, 475.682 seconds, 14,999,293,952 bytes peak RSS | `qa/sherlock/runs.jsonl:93` |
| Subsequent A8 check | Exit 0 | `qa/sherlock/runs.jsonl:94` |
| Replacement-chain A4 | Exit 0 | `qa/sherlock/runs.jsonl:332` |
| October 26 A8 check | Exit 1 | `qa/sherlock/runs.jsonl:391` |

The old call used `ac93b80e…55388b`, block 1014763 (`docs/sherlock/stagenet-deployment.md:188`). Current pins identify `2391d62f…e91ff3f` (`crates/mori-recursion/tests/fixtures/stagenet-verifier.json:2`). The redeployment record gives block 1032519 and changed parent/decider keys (`docs/sherlock/stagenet-deployment.md:282`). The parent agent's November 7 read-only probe reports only that deployment and zero calls. This agrees with recovery after host loss, not an architectural rejection.

The first submission's retry hypothesis identifies incompatible `ChargedState` classes. Current code converts through serialized `ledger.ContractState` before `updateIndex` (`tools/sherlock-sdk/call-current.mjs:155`). That supports the repair mechanism, although the raw failure trace is absent.

A concrete recovery trap remains: `call-current.mjs` defaults to old `deploy-run3/receipt.json`, then compares its address with current pins (`tools/sherlock-sdk/call-current.mjs:24`). The handoff supplies replacement `deploy-run6` through `MORIARTY_SHERLOCK_DEPLOYMENT` (`docs/sherlock/HANDOFF-2026-10-26.md:24`). Preserve current cryptographic pins when resolving this mismatch.

A8 itself accepts an indexed `ContractCall` without verifying its proof or effects (`scripts/qa/check-a8.sh:29`). Stronger checks exist in preparation, native pre-balance/final validation, and complete readback (`tools/sherlock-sdk/call-current.mjs:121`, `:168`, `:199`, `:226`). Preserve them.

3. Dispatch: observed stop and telemetry uncertainty

Commit `44fa11bccfbdd7b930c7a11077074e3b60bff608` records the wall-cap handoff. Its retained attempt is `20261026T105549Z-U023-dispatch-receipt`, head `c87397f8`, command `cargo test --release --locked -p mori-backend --lib c6_dispatch -- --test-threads=1` (`qa/sherlock/runs.jsonl:394`). It records 16,000.324 seconds, wrapper exit 4, child exit 137, and `capExceeded: "wall"`.

The wrapper kills the job at the wall limit and maps budget termination to exit 4 (`models/doctorwatson/factory/heavy.py:187`, `:442`). Child exit 137 therefore does not independently establish OOM. The prior 30 GB OOM appears in the retry hypothesis, without a separate retained dispatch run record.

The handoff reports mutation-test success, unfinished factorial, and roughly 50 GB RSS (`docs/sherlock/HANDOFF-2026-10-26.md:15`). The run record instead reports 86,470,656 bytes. This discrepancy prevents reliable memory sizing from that record.

Source-supported explanations remain hypotheses. RSS polling occurs only with a DoctorWatson RSS cap (`models/doctorwatson/factory/heavy.py:190`). A wall-only invocation supplies no RSS cap even when an external cgroup limits memory (`:416`). The fallback uses direct-child `wait4` accounting (`:215`). Polling also includes only the original process group (`:41`), whereas cleanup uses broader session/marker matching (`:78`). The original invocation and process tree are unavailable. This logic also exists at run head `c87397f8`.

MemoryHigh throttling, nested cgroups, and sccache hangs are reported environmental observations (`wiki-llm/doctorwatson-improvement-log.md:2333`). None establishes a constraint defect.

4. Source-supported cost hypothesis

The test retains a full circuit per rule, then checks every supported rule on every sixteenth state, starting at state zero (`crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:65`, `:109`). Each check constructs statement vectors and invokes `ProofPreimage::check` (`crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:266`).

Each rule emits shared facts before its selected path (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:1315`). These include a program walk and three checked-value readers (`:630`, `:731`). Each reader adds another complete program walk and data canonicality (`crates/mori-backend/src/sherlock_machine/c6_checked_value_read.rs:363`). Witness gates do not prevent instruction emission. Construction also serializes the full IR through JSON and reloads it (`crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:223`).

Large repeated circuits plus retention across rules are the strongest cost hypothesis. Phase timing must establish their contributions. The negative helper also collapses every checker error into rejection (`crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:85`). Preserve error details to distinguish constraint rejection from backend or shape failure.

5. Dispatch alone cannot close current U023

The inventory already lists dispatch as receipted (`crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:550`), but no receipt directory exists. The visible consistency test compares that list with disk files (`crates/mori-core/tests/sherlock_native_inventory.rs:175`). The holdout requires a committed receipt, matching successful run, and trace (`qa/holdout/lib/uireq.py:1025`, `:1042`). A timeout cannot satisfy it.

The handoff's “Scenario 1 should then pass” exceeds current acceptance code. The holdout also requires complete exercised-rule coverage, fragment receipts, and recursive integration (`qa/holdout/lib/uireq.py:1220`). Inventory source marks C6 wiring false (`crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:805`). Tests expect nine factorial gaps and sixteen fold gaps (`crates/mori-core/tests/sherlock_native_inventory.rs:162`). These are source expectations, not fresh test results. A C6 child adapter exists but explicitly retains parent-integration obligations (`crates/mori-backend/src/sherlock_c6_child.rs:39`).

6. Minimal ordered slices

1. Freeze address, keys, source, SDK versions, deployment receipt, and evidence availability. Reconcile the unsupported dispatch receipt claim.
2. Restore A5–A7 and preparation against explicit deploy-run6. Stop at the first identity, runtime, or call-vector failure.
3. Complete one current A8 proof, native validations, submission, and independent readback. Keep A9 rejection deployments separate.
4. Validate dispatch telemetry on one state and rule. Record PID/group/session, cgroup limits, CPU time, memory peak, and phase times.
5. Add one competing rule and one corrupted witness. Require semantic rejection with preserved error details.
6. If cache retention dominates, replay the same matrix in rule-major batches with few live circuits. Preserve all coverage obligations.
7. Complete factorial/fold dispatch runs with measured caps. Only successful retained results justify the nonproving receipt.
8. Close missing fragment coverage and one real dispatch-plus-step proof before recursive qualification. Use `entry 7` before fact(3).

A8 recovery can proceed independently of dispatch profiling. Cut broad `c6_` sweeps, rollup work, and larger programs from initial diagnosis. Preserve final U023 coverage, native verification, authenticated transitions, and complete effects checks.
