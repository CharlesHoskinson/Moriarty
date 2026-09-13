# FOREMAN_REPORT

- run_id: moriarty-ll-successor-20260913b
- role: implement
- slug: handoff-fable-repair
- branch: foreman/moriarty-ll-successor-20260913b/implement/handoff-fable-repair
- worktree: /home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair
- base_sha: 4c95364353e48540c686219fa5c318313994bda0
- status: completed (second correction round; awaiting GPT-6 high check)
- implementer: Claude Fable 5.1 (claude-fable-5-1), medium effort, no delegation
- checker per user routing: GPT-6 high (not run in this lane)
- inputs: first audit /tmp/moriarty-handoff-gpt6-high-20260913.{md,json} (H1, H2, H3, C1, C2, C3); second audit /tmp/moriarty-handoff-fable-audit-gpt6-high-20260913.{md,json} (H1a, H1b, H1c, H1d, H3a, H3b, L1) with probes /tmp/moriarty-handoff-fable-audit-probes.mjs
- interruption: the first attempt at this round was killed externally (queue 1655, 18:26:44 UTC) after the protocol abort edit had been written to disk; this round resumed from those bytes without discarding them.

## Summary

The six concrete H1/H3 findings and the L1 ordering observation from the
second GPT-6 high audit are repaired in the JS helpers. All seven audit probes
now give the required outcomes on the final bytes. The approved H2 fix is
preserved byte-for-byte. The executor suite grew from 101 to 115 passing tests
and every existing financial and language suite passes. The earlier report's
claims that wall-clock changes could not extend the bound, that no late work
could start after return, and that the static PID 1 wrapper source was absent
were false or stale and are corrected below. C1 and C3 remain open; C2
production integration is outside this helper repair.

## Changed files (uncommitted in the worktree)

| Path | Change | SHA-256 after |
| --- | --- | --- |
| ledger/fault-matrix.mjs | unchanged since H2 approval | 1d6b52248b8b1748e554395cfd22ee0075141de261e082edd5063a7fda4619dc |
| ledger/fault-matrix.test.mjs | unchanged since H2 approval | 3a2fc296104a0928095e5e413e6329353be0fb5a99d6eff00688fb278ef857b6 |
| ledger/executor-caller.mjs | H1a, H1b, H1c, H1d, H3a, H3b, L1 | a4b54aad06d74f6b286f3c1463d359fb0bda40626c277ef91f9784c912c9f1dc |
| ledger/executor-caller.test.mjs | regression block rewritten: 22 tests, configured-limit timing | 019723faffdf41e2d6be8c74c2c8806f113378fe8a11f6cbb9c069be40319223 |
| ledger/invocation-handoff.mjs | H1d: optional AbortSignal disposes the protocol | b47797546c5982a6f8f1b5b05d5e320fd9f410ab83148a2d1130bd42ff205519 |
| ledger/invocation-handoff.test.mjs | 4 abort tests | 70f2b1cd1eabe05d18331d36420415d45c56700305a305bcc89dd623da338f5f |

Paths are under experiments/moriarty-midnight-financial/. prover-lifetime.mjs
and prover-lifetime.test.mjs are unchanged (a046a77a…, 693f88c8…). Both
fault-matrix hashes equal the second audit's approved manifest. No plugin
Python, C, hooks, retained artifacts, dependencies, accounting or resource
records were touched. No commit, tag, stash or push was made.

## Repairs by finding

### H1a: monotonic common deadline

Enforcement now uses one schedule from `performance.now()` per run, created
before any awaited work: `outerMs = start + outerTimeoutMs`,
`finalMs = outerMs + abandonAfterKillMs`, and `settleMs = finalMs -
cleanupReserveMs` where the reserve is `min(250 ms, floor(abandon / 2))`.
Every timer and remaining-time calculation uses this schedule. `Date.now` is no
longer imported for enforcement, and the wall-clock helper `beforeDeadline`
is no longer used by the executor. The payload's outer start remains the
injected monotonic sample. Probe `clock-rollback` (300 ms rollback during the
boot-id read, 40 ms budget) now returns in 32 ms.

### H1b: result read and cleanup bounded to remaining budget with a reserve

The setup wait in completion ends at `settleMs`. The optional durable read
gets `min(1000 ms, remaining to settleMs)` and is not started when that is
under two milliseconds (timer granularity, documented in code); the result
reports `resultReadSkipped:true` in that case. Cleanup ends at `finalMs`, so it
always keeps the reserved margin, and a remover still running at the bound is
reported `unresolved`. The abandonment timer fires at `settleMs`, not
`finalMs`, so the reserve is available on the abandonment path too. Probe
`fresh-completion-bounds` (never-resolving append, read and remover, 40 ms
budget) now returns in 41 ms with `cleanup:"unresolved"`.

### H1c: no fresh work after the outer deadline

Pre-spawn phases (`control-record`, `runtime-directory`, `invocation-socket`,
`boot-id`) are bounded to `outerMs`, not `finalMs`. Immediately before spawn,
after every await, the executor rechecks `outerMs` and rejects with
`EXECUTOR_LATE_START` (phase `spawn`, `outstanding:false`) if it has passed;
the prover latest-start check (`preSpawn`) runs after that recheck, directly
before spawn (this also closes L1). Post-spawn setup stops advancing at
`outerMs`: a late append does not activate the handshake, and the handshake
timeout is clamped to the time left to `outerMs`. Probe `startup-after-outer`
now rejects at 20 ms with phase `boot-id` and zero spawns. Probe
`latest-start-check-before-boot` now rejects `PROVER_LIFETIME_LATE_ENTRY`
with zero spawns.

### H1d: explicit protocol abort

`runHandoffServer` accepts an optional `signal` (AbortSignal, validated).
Abort sets the settled and aborted guards, clears the setup timer and every
replay timer, detaches the `connection` listener and the abort listener,
destroys the active socket, and resolves `HANDSHAKE_ABORTED` if not already
settled. After abort no denial record or response starts: `recordAndDeny`
returns without calling the recorder, the replay handler returns, and a
connection arriving on the still-attached server is destroyed. A recorder call
already in flight is left outstanding and is never reported as complete. The
executor owns one AbortController per run and aborts it as the first step of
every cleanup, before detaching its own listeners and calling the remover.
Probe `late-denial-after-error` now shows zero listeners at return and zero
denials later.

### H3a: ownership before fallible work

The runtime context is created as soon as the directory exists. The acquired
server and socket path are assigned to that context immediately after the
socket phase resolves, before the connection queue is installed and before
nonce generation. A nonce failure therefore closes the server and unlinks the
socket and directory. Probe `nonce-exception-loses-socket` (real AF_UNIX
socket) now reports `cleanup:"removed"`, server not listening, socket file
absent.

### H3b: late allocation ownership

`boundedPhase` accepts a late-fulfilment handler for the directory and socket
phases. When a phase times out, the handler is attached to the still-pending
promise; if the allocation later fulfils, the resource is wrapped in its own
runtime context and removed with the run's remover without any further setup.
The rejection carries `lateAllocation:{phase, settled}` where `settled` is a
promise resolving to the late cleanup outcome (`removed`, `failed` with the
error, or `rejected` when the allocation itself failed). It resolves only if
the allocation ever fulfils, which is the truthful state of an uncancelled
promise. Probe `late-directory` now reports the late directory removed
(`lateAllocation.status:"removed"`, `lateDirectoryExists:false`).

### Result and error API

`result.diagnostics` now has `resultReadSkipped` and `handoff` (`null`
without a handoff, else `{setup, phase, appendConfirmed, handshake, denialCode,
errorClass, errorMessage, outstandingWork, runtimeCleanup}`). Rejections after
runtime allocation carry a non-enumerable `runtimeCleanup`; setup timeouts
carry `phase`, `outstanding` and, for allocation phases, `lateAllocation`.

## Evidence (actual checks in this worktree, Node v24.18.1)

Audit probes, exact file `/tmp/moriarty-handoff-fable-audit-probes.mjs`:

| Probe | Second-audit observation (before) | This worktree, final bytes |
| --- | --- | --- |
| clock-rollback | elapsed 344 ms for 40 ms budget | elapsed 32 ms, kills 1 |
| startup-after-outer | spawn at 66 ms, outer 20 ms | rejects `EXECUTOR_SETUP_TIMEOUT` phase `boot-id` at 20 ms, spawns 0 |
| fresh-completion-bounds | elapsed 2043 ms for 40 ms | elapsed 41 ms, read failed and skipped, cleanup unresolved |
| nonce-exception-loses-socket | cleanup failed ENOTEMPTY, socket listening | cleanup removed, server closed, socket absent |
| late-directory | late directory exists, no owner | lateAllocation removed, directory absent |
| late-denial-after-error | 1 listener at return, 1 denial later | 0 listeners, 0 denials |
| latest-start-check-before-boot | spawned at the +120 boundary | rejects `PROVER_LIFETIME_LATE_ENTRY`, spawns 0 |

The unmodified probe file exits 1 after probe 2 because that probe does not
catch the rejection that is now the required outcome. A copy with try/catch
around probes 2, 5 and 7 (`/tmp/moriarty-handoff-fable-audit-probes-wrapped.mjs`,
otherwise identical) runs all seven and exits 0; the table above is from that
run and from the unmodified file's first probe.

Before-repair state was also recorded by running the unmodified probe file
against the audited bytes before any edit this round: all seven observations
matched the audit (344 ms, 66 ms spawn, 2046 ms, ENOTEMPTY, late directory
present, 1 listener and 1 late denial, late-boundary spawn).

New tests against base sources: the ledger directory copied to
/tmp/moriarty-prerepair-ledger with executor-caller.mjs and
invocation-handoff.mjs at HEAD 4c953643, run with a 3 s per-test timeout
because the base executor hangs on the stall tests: 82 tests, 54 pass, 17 fail,
11 cancelled by timeout. Every new H1/H3/L1/abort test fails or hangs there;
the two existing tests that fail there do so on the added diagnostics fields.

Suites on the final bytes (all exit 0):

| Command | Tests | Pass | Fail |
| --- | --- | --- | --- |
| npm --prefix experiments/moriarty-midnight-financial run test:executor | 115 | 115 | 0 |
| same, repeated 3 further times | 115 each | 115 each | 0 |
| … test | 63 | 63 | 0 |
| … run test:ledger (includes the executor files) | 639 | 639 | 0 |
| … run test:compiled | 3 | 3 | 0 |
| … run test:compiled-rejection | 11 | 11 | 0 |
| … run test:finalized-state | 27 | 27 | 0 |
| … run test:preview | 89 | 89 | 0 |
| npm --prefix experiments/moriarty-language test | 906 | 906 | 0 |
| npm --prefix experiments/moriarty-language run typecheck | tsc clean | | |

Leak check: the first-audit H1 scenario as a standalone script with the
append gate never released returns in 10 ms, exits 0 in 105 ms wall time, and
`process.getActiveResourcesInfo()` is empty at return.

Existing C wrapper test (untouched plugin code, run for completeness):
`python3 -m pytest -q plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py`
gives 22 passed, 2 failed. Both failures are `assert_pid_gone` on a killed
child whose `/proc/<pid>` entry remains on this host
(`test_control_deadline_kills_hanging_child_and_is_independent`,
`test_dead_external_launcher_cannot_disable_wrapper_deadline`). This lane did
not modify the C source or that test and did not diagnose the cause; it may be
host or reaper specific and is reported as observed, not as a verdict.

Timing test design: bounds are 40 ms outer and 40 ms abandonment; assertions
compare measured `performance.now()` elapsed time against the configured
limit plus a 50 ms scheduling allowance. The allowance is justified in the test
file: libuv timers have 1 ms granularity and observed slip on this host is
single-digit milliseconds, so 50 ms is an order of magnitude above the slip and
below every configured bound it guards. Gates are explicit promises released
after return, followed by a 40 ms window to observe no late listener, write or
denial. The 2 s watchdogs from the previous round are removed.

Other checks: `git diff --check` clean; guarded CLI status was loaded at the
start of the session (dependent loan-swap dispatch blocked on stale sprint
inputs and missing accounting; unrelated to this source repair); the
attribution grep over `git log --all` matches nothing; retained pointers
exported, nothing rebuilt or installed.

## Corrections to the previous report

- "Wall-clock jumps move the enforcement point but cannot extend it past the
  bound" was false; probe `clock-rollback` showed 344 ms. Fixed by H1a.
- "no late listener registration" and "no bytes reach a peer" understated the
  hole: the protocol's own timer could start a new denial append after return.
  Fixed by H1d.
- "no static PID 1 prover wrapper" was stale. The base contains tracked
  `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c` (604 lines,
  commit 31b59ce9) with `prover_lifetime.build.md` and
  `plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py`. Its production
  integration and acceptance are not established by this lane.
- The 1000 ms result grace and the `max(finalMs, now + 1000)` cleanup wait
  contradicted the claimed shared deadline. Fixed by H1b.

## Retained findings from both audits

- H2 (first audit): fixed and approved at the hashes above; unchanged this round.
- H1, H3 (first audit): the specific defects were the subject of round one;
  their residuals H1a, H1b, H1c, H1d, H3a, H3b are repaired this round as above.
- L1 (second audit): closed by the pre-spawn ordering change.
- C1: open, outside repair. No Python loan executor, no runner/cmd_run consumer
  of these helpers, no authenticated production handoff.
- C2: the C wrapper source and its tests exist in the base; production
  integration, namespace and live lifetime acceptance are not established here.
- C3: open, outside repair. The caller still hardcodes terminal evidence, stop
  and timer facts to false, so `PROCESS_SUCCESS` cannot be produced from
  genuine evidence; no separately supervised persistence worker exists.

## Remaining limitations

- I/O promises are never cancelled. A stalled append, control write,
  allocation, boot-id read, durable read or remover keeps running after the
  bounded return and is reported as outstanding, skipped or unresolved. A
  synchronous blocking fsync inside such a call is not preemptible by this
  code; the spec's supervised persistence worker is not implemented.
- Late allocation cleanup is best-effort ownership after return: it uses the
  run's remover, reports through `lateAllocation.settled`, and cannot settle
  if the allocation never fulfils.
- Under a stalled `confirmCurrentState` or denial append, the setup record may
  read `pending/handshake/cancelled`, `settled/denied/HANDOFF_SETUP_TIMEOUT`
  or `settled/cancelled` depending on timer order; all yield `STARTUP_INVALID`
  and the tests accept each.
- The reserve is capped at 250 ms and at half the abandonment budget; a
  remover needing longer is reported `unresolved`, not waited for.
- The schedule is per process (`performance.now()`); it is not shared with the
  child or with the durable outer start in the payload.
- No K, Java, Docker, network, wallet or live financial action; source-only
  offline tests. Large-suite results are this lane's evidence only.

## Open questions

None for this lane. Next step under the current routing: GPT-6 high check of
these exact bytes.
