# FOREMAN_REPORT

- run_id: moriarty-ll-successor-20260913b
- role: implement
- slug: handoff-fable-repair
- branch: foreman/moriarty-ll-successor-20260913b/implement/handoff-fable-repair
- worktree: /home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair
- base_sha: 4c95364353e48540c686219fa5c318313994bda0
- status: completed
- implementer: Claude Fable 5.1 (claude-fable-5-1), medium effort, no delegation
- checker per user routing: GPT-6 high (not run in this lane)
- audit input: /tmp/moriarty-handoff-gpt6-high-20260913.md and .json (findings H1, H2, H3)

## Summary

The two GPT-6 high findings and the H3 cleanup finding are repaired in the
handoff helpers. The audit's exact H1 and H2 reproducers now return the
required outcomes, the executor suite grew from 86 to 101 passing tests, and
all existing language and financial suites still pass. The still-missing
Python loan executor (C1), static PID 1 prover wrapper (C2) and full evidence
path (C3) are untouched and remain task 5.2 blockers.

## Changed files (uncommitted in the worktree)

| Path | Change | SHA-256 after |
| --- | --- | --- |
| experiments/moriarty-midnight-financial/ledger/fault-matrix.mjs | H2: null raw exit is unknown; success requires exact zero exit | 1d6b52248b8b1748e554395cfd22ee0075141de261e082edd5063a7fda4619dc |
| experiments/moriarty-midnight-financial/ledger/fault-matrix.test.mjs | 3 new rows including the H2 reproducer | 3a2fc296104a0928095e5e413e6329353be0fb5a99d6eff00688fb278ef857b6 |
| experiments/moriarty-midnight-financial/ledger/executor-caller.mjs | H1 and H3: one absolute deadline, bounded setup and completion, cancellation, reported cleanup | 110e57cff180d08c1c601eed54a93e3e24554c648cd182f1fa4cac017c2029df |
| experiments/moriarty-midnight-financial/ledger/executor-caller.test.mjs | 12 new tests, 2 existing tests extended for the new diagnostics field | a06651eda8d488e3478fef3dba22be103a30a32095f4f5e2809b672a06a48c59 |

invocation-handoff.mjs and its test are unchanged (digests 2594d799… and
7c7bfed8… match the audit manifest). prover-lifetime.mjs is unchanged.
No plugin Python, hooks, retained artifacts, dependencies, accounting or
resource records were touched. No commit, tag or push was made.

## Findings and repairs

### H2 (fault-matrix.mjs): null raw exit could reach PROCESS_SUCCESS

Before: `classifyDisposition` with `rawExit:null` and every other fact set to
success returned `PROCESS_SUCCESS/0/null`.

Repair: after the REFUSED, startup and evidence/identity gates, a null
observation now takes the same branch as `kind:'unknown'` and returns
`PROCESS_UNKNOWN/3/MAIN_EXIT_UNAVAILABLE`. The success path is additionally
guarded by an explicit `kind==='exit' && code===0` check. REFUSED, STARTUP_INVALID,
EVIDENCE_WRITE_FAILED, EVIDENCE_PREEXISTS and MAIN_OBSERVATION_INVALID keep
their precedence over the null observation. Known nonzero/signal failures keep
PROCESS_FAILED precedence over all later cleanup facts (existing row test).

### H1 (executor-caller.mjs): unbounded persistence and setup waits

Before: `complete()` awaited `handoffSetup` without a bound, so a
never-resolving `appendInvocationEvent` held the executor open after the
outer timeout and abandonment bound. `writeControlRecord`,
`createRuntimeDirectory`, `createInvocationSocket` and `readBootId` were
awaited before any timer existed. A denial append or `confirmCurrentState`
stall inside `runHandoffServer` had the same effect.

Repair:
- One absolute wall-clock schedule per run: `outerMs = start + outerTimeoutMs`,
  `finalMs = outerMs + abandonAfterKillMs` (default 1000 ms). The kill timer and
  abandonment timer are installed relative to that schedule, so pre-spawn setup
  time no longer shifts the child's deadline.
- Pre-spawn phases (`control-record` in the prover path, `runtime-directory`,
  `invocation-socket`, `boot-id`) are awaited against `finalMs`. A stalled phase
  rejects with `EXECUTOR_SETUP_TIMEOUT`, `error.phase` naming the phase and
  `error.outstanding=true` stating that the I/O promise was not cancelled. A
  failed phase rethrows its original error unchanged. No spawn occurs.
- `complete()` waits for the post-spawn setup (parent stat, invocation append,
  handshake) only until `finalMs`. A setup still pending at that bound is
  cancelled for this run, recorded as `startupValid=false` (with
  `startupWriteFailed=true` only when the append phase was the stalled one) and
  reported in `diagnostics.handoff` as `setup:'pending'`, its `phase`, and
  `outstandingWork:true`. No persisted evidence is assumed.
- Cancellation: a late-resolving append checks the cancelled flag and returns
  without calling `runHandoffServer`, so no `connection` listener is attached to
  a cleaned-up server and queued sockets are never resumed. Queued and tracked
  sockets are destroyed at cleanup, so a late denial or payload write inside
  `runHandoffServer` has no live socket to answer.
- The `runHandoffServer` timeout is clamped to
  `min(setupTimeoutMs, finalMs - now)` so its own settle guard fires by the
  run's bound; a stalled `confirmCurrentState` and a stalled denial append are
  both handled by the same bound and both yield `STARTUP_INVALID`.
- Durable append still precedes any handshake activation; the race, boot
  identity and record-before-answer repairs from 5107a6ff are preserved and
  their tests still pass.

### H3 (executor-caller.mjs): swallowed cleanup errors and leaked early failures

Before: completion ran `removeRuntime` inside an empty catch; a spawn throw or
child `error` after runtime allocation rejected without removing the owned
socket/server/directory.

Repair:
- `runPrepared` wraps allocation through completion in one try/catch. On any
  rejection after the runtime directory exists, the executor detaches its
  listeners, destroys its sockets, runs the (bounded) remover and attaches the
  outcome as a non-enumerable `error.runtimeCleanup` on the original error,
  which is rethrown unchanged.
- On normal completion the cleanup outcome is reported in
  `diagnostics.handoff.runtimeCleanup` as `removed`, `failed` (with error class
  and message) or `unresolved` (remover still outstanding at the bound), with
  the owned `runtimeDir` and `socketPath`.
- Listener detachment and socket destruction moved from the default remover into
  the executor-owned cleanup, so an injected remover cannot leave a late
  connection path open.
- `unlink`/`rmdir` ENOENT on the owned paths is tolerated; any other removal
  error is reported, not swallowed.

### Result API change

`result.diagnostics` gained a `handoff` field: `null` without a configured
handoff, otherwise a frozen record
`{setup, phase, appendConfirmed, handshake, denialCode, errorClass, errorMessage, outstandingWork, runtimeCleanup}`.
Two existing tests were extended to assert it.

## Evidence (actual checks, this worktree, Node v24.18.1)

Reproducers run exactly as given in the audit, from the worktree root:

| Reproducer | Before (HEAD 4c953643) | After |
| --- | --- | --- |
| H1 stdin script | `{"state":"still pending after 60ms","kills":1,...}` | returned in 13 ms: `PROCESS_UNKNOWN/3/EVIDENCE_WRITE_FAILED`, `handoff.setup:"pending"`, `phase:"append"`, `outstandingWork:true`, `runtimeCleanup.status:"removed"`, `kills:1` |
| H2 stdin script | `{"status":"PROCESS_SUCCESS","exitCode":0,"failureCode":null}` | `{"status":"PROCESS_UNKNOWN","exitCode":3,"failureCode":"MAIN_EXIT_UNAVAILABLE"}` |

Leak check: the H1 scenario run as a standalone script with the append gate
never released exits with code 0 after 206 ms wall time and
`process.getActiveResourcesInfo()` empty at return, so no timer or socket
handle survives the bounded return.

Tests reproduce the faults before repair: the new/changed test files run
against the HEAD versions of the two source files (copied to
/tmp/moriarty-prerepair-ledger) give 48 pass / 17 fail. The 17 failures are the
12 new executor tests, the 3 new fault-matrix rows, and the 2 existing tests
that now assert the added `diagnostics.handoff` field (those 2 fail on shape,
not on the audited fault).

Suites after repair (all exit 0):

| Command | Tests | Pass | Fail |
| --- | --- | --- | --- |
| npm --prefix experiments/moriarty-midnight-financial run test:executor | 101 | 101 | 0 |
| npm --prefix experiments/moriarty-midnight-financial test | 63 | 63 | 0 |
| … run test:ledger | 625 | 625 | 0 |
| … run test:compiled | 3 | 3 | 0 |
| … run test:compiled-rejection | 11 | 11 | 0 |
| … run test:finalized-state | 27 | 27 | 0 |
| … run test:preview | 89 | 89 | 0 |
| npm --prefix experiments/moriarty-language test | 906 | 906 | 0 |
| npm --prefix experiments/moriarty-language run typecheck | tsc clean | | |

Retained pointers were exported as given (MORIARTY_SWAP_BUILD_RECEIPT,
MORIARTY_CUSTODY_ARTIFACTS); nothing was rebuilt or installed. The
moriarty-language package has no node_modules and its suite ran with the
installed toolchain only.

New regression tests use explicit gate promises, 20 ms outer / 20 ms
abandonment bounds and a 2000 ms watchdog, with a 40 ms late-settle window
after releasing each gate to observe no late listener registration, no late
socket write and destroyed sockets.

Guarded CLI: `cli.py --repo . status --json` ran at start. It reports the
dependent loan-swap dispatch blocked on stale sprint inputs and missing
current accounting; this authorized source repair does not depend on that
dispatch and created no accounting, reservation or resource record. The
attribution grep over `git log --all` matched nothing.

## Remaining limitations

- I/O promises are never cancelled. A stalled append, control write, directory
  or socket creation, boot-id read or remover keeps running after the bounded
  return; the result or error states this (`outstandingWork`, `outstanding`,
  `runtimeCleanup.status:'unresolved'`). A runtime directory whose creation
  resolves after a `runtime-directory` timeout is not removed by this run.
- The unresolved remover wait is `max(finalMs, now + 1000 ms)`, so a stalled
  remover can extend the return by up to one second past outer + abandonment.
- Under a stalled `confirmCurrentState` or denial append, the setup may be
  reported as `pending/handshake/cancelled` or as `settled/denied/HANDOFF_SETUP_TIMEOUT`
  depending on which of two co-scheduled timers fires first; both yield
  `STARTUP_INVALID` and the tests accept either.
- `runHandoffServer` gained no cancellation signal (invocation-handoff.mjs is
  unchanged). A late `confirmCurrentState` resolution can still resolve
  `HANDSHAKE_OK` internally, but the executor has destroyed the socket and
  detached the server, so no bytes reach a peer; the tests assert no payload
  write and no listener registration.
- Deadlines use the wall clock (`Date.now`) for timers; the outer start
  remains the injected monotonic sample in the payload. Wall-clock jumps move
  the enforcement point but cannot extend it past the bound in the process's
  timer scheduling.
- The spec's separately supervised persistence worker for blocking fsync is
  not implemented; the JS caller only bounds its own waits.
- Task 5.2 blockers unchanged (out of scope by instruction): C1 no Python
  loan executor or runner/cmd_run consumer of these helpers; C2 no static PID 1
  prover wrapper; C3 the caller still hardcodes terminal evidence, stop and
  timer facts to false, so PROCESS_SUCCESS cannot be produced from genuine
  evidence. No K, Java, Docker, network, wallet or live financial action was
  run.

## Open questions

None for this lane. The GPT-6 high check of these exact bytes has not been run
and is the next step under the current routing.
