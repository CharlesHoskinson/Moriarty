# Independent GPT-6 high helper audit

**Verdict: REQUEST_CHANGES.** H2 is fixed and separable; H1/H3 are partially repaired. Task 5.2 remains open.

Candidate: `/home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair` at base `4c95364353e48540c686219fa5c318313994bda0`. All eight audited source/test hashes were identical at start and end.

Independent checks: 101/101 tests passed in 2190 ms; original H1 and H2 reproducers now pass; seven additional probes ran successfully and exposed the failures below. `git diff --check` passed.

## H1a — high, unresolved / new clock regression

Wall-clock rollback extends the new absolute schedule. Timers installed after a rollback calculate a longer duration from Date.now; the monotonic payload sample does not govern enforcement. Location: `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:87`.

Observed (`clock-rollback`): `{"elapsedMs":344,"configuredTotalMs":40,"rollbackMs":300,"kills":1}`.

Minimal correction: Use one monotonic absolute schedule for enforcement and all remaining-time calculations. Keep wall time only for UTC evidence; test a wall-clock jump without changing enforcement.

## H1b — high, unresolved

Completion still grants a fresh 1000 ms result read and cleanup grants max(finalMs, now+1000), contradicting the promised shared final deadline. Location: `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:135 and :234`.

Observed (`fresh-completion-bounds`): `{"elapsedMs":2043,"configuredTotalMs":40,"resultReadFailed":true,"cleanup":"unresolved"}`.

Minimal correction: Cap result read and cleanup to remaining final budget, reserve cleanup time before exhausting that budget, and retain unresolved outcomes when it is exhausted. Do not begin a new optional observation once no budget remains. The existing 2000 ms watchdog against 40 ms limits does not test the claimed limit.

## H1c — high, unresolved late-start guard

Pre-spawn work is allowed to finish inside abandonment after the outer execution deadline, then spawn occurs without checking outer expiry. An immediate zero close clears the zero-delay kill timer and reports deadlineExceeded=false. Location: `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:181-189`.

Observed (`startup-after-outer`): `{"spawnAfterMs":66,"outerMs":20,"kills":0,"deadlineExceeded":false}`.

Minimal correction: Check the monotonic outer execution deadline immediately before spawn, after all awaited setup. Refuse startup after that cutoff; abandonment is a return/cleanup allowance, not a fresh-work allowance. Also perform the existing prover latest-start check immediately before spawn.

## H1d — medium, unresolved cancellation

Cleanup detaches queue/track listeners but not runHandoffServer connection listener or timer. A child error after activation returns cleanup=removed while a later timer starts a denial append. This is newly initiated late work, not merely an already pending I/O completing. Location: `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:126-143; invocation-handoff.mjs:69-96`.

Observed (`late-denial-after-error`): `{"error":"child-error","cleanup":"removed","listenersAtReturn":1,"denialsAtReturn":0,"denialsLater":1}`.

Minimal correction: Give the protocol an explicit abort/dispose operation that sets its settled/cancelled guard, clears timer/replay timers, detaches its own listeners and prevents new persistence/response work; invoke it on every exit before resource cleanup. Preserve already outstanding writes as unresolved.

## H3a — medium, partially repaired but unresolved

makeNonce() runs while evaluating the Object.assign argument before created.server/socketPath are assigned to owned context. If it throws, default cleanup cannot close/unlink the acquired socket. A real AF_UNIX probe leaves the socket listening and reports socketPath=null. Location: `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:183`.

Observed (`nonce-exception-loses-socket`): `{"error":"nonce source unavailable","cleanup":"failed","cleanupError":"ENOTEMPTY","reportedSocketPath":null,"socketListening":true,"socketExists":true}`.

Minimal correction: Assign each acquired resource into the cleanup context immediately, before any later fallible work. Then run nonce generation/listener setup. Add actual early-failure cleanup tests asserting server closed and owned paths absent.

## H3b — medium, new timeout-path resource leak, disclosed but unresolved

A runtime allocation that fulfills after boundedPhase times out is never registered or cleaned up. outstanding=true discloses the promise but leaves a newly created owned directory outside any cleanup owner. Location: `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:180-182`.

Observed (`late-directory`): `{"error":"EXECUTOR_SETUP_TIMEOUT","phase":"runtime-directory","outstanding":true,"reportedCleanup":null,"lateDirectoryExists":true,"spawns":0}`.

Minimal correction: Retain resource ownership across the race: attach late-completion handling to allocation/socket creation that closes/removes resources without starting the next phase, and report explicit unresolved allocation ownership until that cleanup settles. Test delayed settlement, not only never-resolving promises.

The +120 latest-start probe also spawns at the forbidden boundary because the check precedes awaited boot-id. This ordering existed in the base; it is not a new regression and does not demonstrate actual prover startup. Move this existing guard alongside the immediate pre-spawn deadline check.

H2 approval applies only to the two fault-matrix files at the hashes below. Null/unknown observations return MAIN_EXIT_UNAVAILABLE, known failures remain failed, and success requires an observed exact zero.

C1 and C3 remain production blockers. C2 production integration/acceptance is outside this helper audit, but the author report’s literal claim that static PID1 source is absent is stale: current base contains tracked `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c` (604 lines). Its independent source work was not audited here. No K, Docker, network, accounting or financial acceptance was performed.

The author discloses uncancelled I/O, late directory leakage and extra cleanup grace, but those disclosures do not close the named H1/H3 requirements. Its wall-clock and no-late-work claims are contradicted by reproduced behavior. Large-suite results remain author evidence.

Exact reproducers: `node /tmp/moriarty-handoff-fable-audit-probes.mjs`. Source is also embedded in the JSON report; real socket/directory fixtures are removed by the probe.

## Frozen SHA-256 manifest

- `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs`: `110e57cff180d08c1c601eed54a93e3e24554c648cd182f1fa4cac017c2029df`
- `experiments/moriarty-midnight-financial/ledger/executor-caller.test.mjs`: `a06651eda8d488e3478fef3dba22be103a30a32095f4f5e2809b672a06a48c59`
- `experiments/moriarty-midnight-financial/ledger/fault-matrix.mjs`: `1d6b52248b8b1748e554395cfd22ee0075141de261e082edd5063a7fda4619dc`
- `experiments/moriarty-midnight-financial/ledger/fault-matrix.test.mjs`: `3a2fc296104a0928095e5e413e6329353be0fb5a99d6eff00688fb278ef857b6`
- `experiments/moriarty-midnight-financial/ledger/invocation-handoff.mjs`: `2594d799dacb2fc68a75aeec09fda4616ea8024f42fcd26c43809a2dcc78510c`
- `experiments/moriarty-midnight-financial/ledger/invocation-handoff.test.mjs`: `7c7bfed8c3013af39eceadaeae0efb911832e3deece1ce3ca25aa4a9a06be095`
- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.mjs`: `a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb`
- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.test.mjs`: `693f88c809668f0daf513f8b5ed12826d4facfea72a66a9eda160ac373d5e6ed`

Full checks, findings, exact reproducer source, report hashes, limits and scope: `/tmp/moriarty-handoff-fable-audit-gpt6-high-20260913.json`.
