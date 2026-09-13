# FOREMAN_REPORT — correction round (AUDIT-OPUS-01 BLOCKED → corrected)

Worktree: `/home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-scope-reconciliation`
Base SHA: `056e662254c38936758af9c6f72c019705a1ff09` (unchanged; nothing committed this round)
Prior verdict: **BLOCKED** (`AUDIT-OPUS-01.md` / `.json`)
Implementer this round: Codex (`gpt-5.6-sol`, `model_reasoning_effort=high`), independently reviewed
and re-verified by the supervising agent outside Codex's own sandbox.

This report replaces the previous `FOREMAN_REPORT.md`/`.json` in full. It addresses every
finding in `AUDIT-OPUS-01.md` individually: fixed, de-scoped, or disputed.

## Findings disposition

### F1 (blocking) — outer-timeout kill reported as a known main failure — **FIXED**

`executor-caller.mjs` now tracks a `killRequested` provenance flag. It is set only when
`deps.killGroup` is *called and does not throw*. Any subsequent `close` observation, once
`killRequested` is true, is fed to `classifyDisposition` as `rawExit:{kind:'unknown',code:null}`
rather than the real signal — because after issuing a `SIGKILL`, this caller has no way to prove
whether a later close reflects that kill or a coincidental natural exit, and the spec (lines 68,
72, 90, 132) forbids treating a self-inflicted kill as a retained known main failure. The **real**
observed raw exit (e.g. `{kind:'signal',code:9}`) is still retained and returned in
`result.rawExit` for diagnostics/evidence — nothing about the audit-praised raw-exit-retention
placement changed.

If `killGroup` itself throws (kill denied/failed), `killRequested` stays `false`, and a later
natural close is still treated as a genuinely retained observation. A new test,
*"a failed kill request does not hide a later retained natural main failure"*, proves
`PROCESS_FAILED/1/MAIN_EXIT_NONZERO` still surfaces correctly in that case — the fix does not
launder every post-timeout close, only the ones the caller's own successful kill can plausibly
explain.

`fault-matrix.mjs`'s precedence order was **not** touched, per the audit's own finding that the
matrix is not at fault. The test previously at `executor-caller.test.mjs:77-83` (asserting
`PROCESS_FAILED/1/MAIN_SIGNAL` and titled as if this were correct behavior) is retitled
*"outer timeout retains its close signal but classifies the caller kill as unknown"* and now
asserts `PROCESS_UNKNOWN/3/MAIN_EXIT_UNAVAILABLE` (the new F8 row — see below) while still
asserting exactly one `killGroup` call and that `result.rawExit` still reflects the real signal.

### F2 (blocking) — success gate fed invented facts — **FIXED**

The fault-matrix input built in `executor-caller.mjs` no longer hardcodes
`stopReturnCode:0`, `timerCancelReturnCode:0`, `timerCancelReceiptPersisted:true`, nor derives
`terminalEvidencePersisted`/`stopReceiptPersisted` from "exit kind known and durable read
succeeded." Corrected values, none invented:

| Field | Before | After |
| --- | --- | --- |
| `terminalEvidencePersisted` | `terminalKnown&&!resultReadFailed` | `false` (always — never observed) |
| `stopReturnCode` | `0` | `null` |
| `stopErrorClass` | caller's own kill error class | `null` (see F8) |
| `stopReceiptPersisted` | `terminalKnown&&!resultReadFailed` | `false` (always — never observed) |
| `timerCancelReturnCode` | `0` | `null` |
| `timerCancelReceiptPersisted` | `true` | `false` (always — never observed) |

**`PROCESS_SUCCESS` is no longer reachable from `executor-caller.mjs`.** Because
`terminalEvidencePersisted` is unconditionally `false`, and `fault-matrix.mjs`'s existing,
unchanged precedence order requires it `true` before it will even look at the stop/timer/
containment gates, every path through this caller now bottoms out at `PROCESS_UNKNOWN`
(`EVIDENCE_WRITE_FAILED`, `MAIN_EXIT_UNAVAILABLE`, or `CONTAINMENT_UNRESOLVED` depending on the
case) or `PROCESS_FAILED` for a genuinely retained known main failure. This is correct and
intended, not a gap papered over: no new production code was added anywhere to synthesize these
facts, and `fault-matrix.mjs` itself remains fully capable of producing `PROCESS_SUCCESS` given a
complete, honestly-observed input from a caller that actually runs the systemd collector/stop/
timer machinery the spec describes (lines 74-79) — that caller does not exist yet.

### F3 — unvalidated child-authored containment object — **FIXED**

`containment()` now throws `EXECUTOR_DURABLE_RESULT_SCHEMA` unless the durable result's `schema`
field is exactly `moriarty.loan-process-result/1` (previously any schema was accepted, and the
test fixture used the wrong schema `moriarty.local-financial-integration/1` — now corrected). It
also throws `EXECUTOR_DURABLE_RESULT_STATUS` unless the document's own `status` field agrees with
the actually observed `rawExit` (`exit`/`0` → `'PASS'`; `exit`-nonzero or `signal` → `'FAIL'`;
anything else rejected). Both failures are treated as an unresolved durable read, never as
containment. A new test, *"schema-invalid or exit-inconsistent durable results cannot establish
containment,"* covers both cases.

### F5 — `requireControlRecord:false` as a weaker fallback — **FIXED**

`runProverBoundedExecutor` now unconditionally requires `writeControlRecord` to be a function and
unconditionally `await`s it with **no** try/catch swallow before spawn — a thrown error always
propagates and aborts before any spawn, regardless of the legacy `requireControlRecord` flag's
value. The flag is still accepted (type-checked) for backward compatibility but no longer changes
this behavior. The two tests that asserted `PROCESS_SUCCESS` with no writer, or after a failed
write, are rewritten to assert rejection with **no spawn call** in both cases, matching the
pre-existing (unchanged) "required control persistence failure prevents every child spawn" test.

### F6 — unbounded wait after the deadline kill — **FIXED**

A second bounded timer (`ABANDON_AFTER_KILL_MS`, default 1000ms, overridable per call via the new
optional `abandonAfterKillMs` run option) starts right after the outer-timeout kill attempt. If
the child has not closed or errored by then, the promise is forced to settle with
`rawExit:{kind:'unknown',code:null}` instead of hanging forever. All timers are cleared on every
settlement path through one `clearTimers()` helper. New test: *"a child that never closes after a
failed deadline kill settles at the abandonment bound."*

### F8 — missing `MAIN_EXIT_UNAVAILABLE` row; `stopErrorClass` misuse — **FIXED**

`fault-matrix.mjs` gained one additive row, inserted immediately after the `knownMainFailure`
check and before the `terminalEvidencePersisted` check: an unknown/null `rawExit` with no
higher-precedence disqualifier now classifies as `PROCESS_UNKNOWN/3/MAIN_EXIT_UNAVAILABLE` (spec
line 66), instead of falling through to the unrelated `EVIDENCE_WRITE_FAILED`. Every pre-existing
row/precedence test still passes unchanged; one new row test was added. Separately,
`executor-caller.mjs` no longer loads `stopErrorClass` from the caller's own `killGroup` error
class — it is always `null` now (per F2); the kill's own error class is surfaced only in
`diagnostics.killErrorClass` for observability.

### F9 — tautological assertion — **FIXED**

The dead `let cleanupCalls=0; …; assert.equal(cleanupCalls,0)` in `fault-matrix.test.mjs` is
deleted. The test's two real assertions (source-purity regex, unresolved-evidence classification)
are unchanged.

### F10 — full environment inheritance — **FIXED (within this round's honest scope)**

`spawn` now receives an explicit, validated `env` object (a new optional run-option field, a flat
string-to-string map), defaulting to `{}` when the caller supplies none — never `process.env` and
never `undefined` (which Node treats as full parent inheritance). This does **not** implement the
full `MORIARTY_INVOCATION_SOCKET`/`MORIARTY_INVOCATION_NONCE` handoff — that remains F4, explicitly
out of scope — but it removes the specific defect named (spawning with no `env` key at all means
silent, unexamined full inheritance) and gives a future caller a place to put those two variables
once the handoff protocol exists.

### F11 — untested `error` path; unguarded `classifyRawExit` — **FIXED**

New test *"child error rejects with the original spawn error"* covers the `child.once('error')`
path. The `close` handler's synchronous body (including the `classifyRawExit` call) is now wrapped
in `try`/`catch`, so a malformed `(status,signal)` pair rejects the promise with the classifier's
real error message instead of throwing uncaught. New test: *"malformed close payload rejects
instead of escaping the promise."*

### F12 / package wiring — suite outside every aggregate script — **FIXED**

`package.json`'s `test:ledger` script now also runs `ledger/prover-lifetime.test.mjs`,
`ledger/executor-caller.test.mjs` and `ledger/fault-matrix.test.mjs` (appended; the existing
25-file list is unchanged in order). `test:executor` is retained unchanged as an independently
invocable alias. Dependency count remains zero.

### F4 — concrete one-shot runner invocation handoff — **DE-SCOPED, recorded OPEN, not implemented**

The AF_UNIX socket, mode-0700 runtime directory / mode-0600 socket, 256-bit nonce, explicit
env-only `MORIARTY_INVOCATION_SOCKET`/`MORIARTY_INVOCATION_NONCE` pinning, constant-time nonce/
authority comparison, socket peer UID check, `/proc` PID-ancestry verification with retained start
ticks, exactly-one-handshake enforcement, pre-handshake durable invocation-event append ("a failed
append yields no handshake"), and EOF-before-setup/EOF-after-startup semantics (spec.md lines
29-40) are **not present** anywhere in this diff and were not attempted this round. **This
corrects the previous round's report**, which listed this requirement under "implemented" while
disclosing the omission elsewhere — the audit (its F4) is right that this was too generous. The
requirement stays **open**, not met, not partially satisfied by the argv-delivery test. The seven
denial cases in the spec's "Actual runner-to-script protocol" scenario (forged nonce, wrong
authority/reservation, stale store event, different launcher PID, replay, boot mismatch, parent
disappearance) remain entirely unexercised and cannot be exercised without the channel.

### F7 — static PID-1 C wrapper — **DE-SCOPED, recorded OPEN, not implemented**

The compiled-artifact half of "Prover lifetime established before daemon startup" (spec.md lines
100-127: one small import-free static PID-1 executable at
`plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c`, reproducible build instructions, and
offline tests that compile that actual wrapper and drive controlled child programs that exit,
hang, fork, change process group and ignore signals, plus parent-death/reap/signal-forwarding
behavior) does not exist anywhere in the tree and was not attempted this round.
`ledger/prover-lifetime.mjs` and its test suite were **not modified** in this correction round —
the audit found no defect there, and this report does not claim any new coverage for it. It
remains, correctly, a pure JS model of the wrapper's arithmetic and control-record byte format
only: a correct, well-tested model, but not the compiled executable the spec requires, and not
established process-level behavior.

## PROCESS_SUCCESS reachability — stated plainly

**No, `PROCESS_SUCCESS` is not reachable from `executor-caller.mjs`** (neither
`runExitRetainingExecutor` nor `runProverBoundedExecutor`) after the F2 fix, in any test in the
corrected suite or in principle, because `terminalEvidencePersisted` is unconditionally `false` and
the matrix's own (unchanged) precedence order requires that field `true` before it will consider
anything past it. This is the correct and intended state of this caller in this round.
`fault-matrix.mjs` itself is untouched in its ability to produce `PROCESS_SUCCESS` given a complete
input from a future, more complete caller.

## Evidence

- `git log -1`: `056e662254c38936758af9c6f72c019705a1ff09` before and after (unchanged; nothing
  committed). `git status --porcelain` digest before and after this round:
  `03a8f85bdb467441a804a2a86dd9ff65f034c2b058fc49522d65f11192074998` (identical — same path/status
  set both times; only contents within already-untracked/already-modified paths changed).
  `unauthorized_git_activity: false`.
- Verification re-run independently, **outside Codex's own sandbox** (Codex's sandboxed run hit an
  `EPERM` on a nested `spawnSync`/FIFO test inside the pre-existing, untouched
  `launch-local.test.mjs`, unrelated to this diff — the same known sandbox artifact called out in
  the correction brief):

  ```
  npm test                        : 1..52  / pass 52  / fail 0
  npm run test:ledger             : 1..576 / pass 576 / fail 0  (524 pre-existing + 52 new)
  npm run test:compiled           : 1..3   / pass 3   / fail 0
  npm run test:compiled-rejection : 1..11  / pass 11  / fail 0
  npm run test:finalized-state    : 1..27  / pass 27  / fail 0
  npm run test:preview            : 1..89  / pass 89  / fail 0
  npm run test:executor           : 1..52  / pass 52  / fail 0  (was 45 before this round)
  ```

  All baselines (52 / 3 / 11 / 27 / 89, and 524 as the pre-existing subset of the now-576
  `test:ledger` total) hold with zero regressions.
- `grep -niE 'AF_UNIX|MORIARTY_INVOCATION_SOCKET|MORIARTY_INVOCATION_NONCE|prover_lifetime\.c'`
  across the six executor-suite files, and a repo-wide `find` for `prover_lifetime*`: zero matches
  — confirms F4/F7 were not attempted.
- `grep -niE 'docker|wallet|budget|charge|debit|fetch|http|net\.|node:net|accounting|network'`
  across the six executor-suite files: exactly one hit, `prover-lifetime.mjs`'s own unchanged
  disclaimer comment — confirms no overreach.
- `ledger/prover-lifetime.mjs` and `ledger/prover-lifetime.test.mjs` are byte-for-byte unmodified
  from before this round.

## Open work (explicit)

1. **F4** — the concrete one-shot runner invocation handoff protocol (socket, nonce, env pinning,
   peer UID, PID ancestry, one-shot handshake, pre-handshake durable event, EOF semantics, seven
   denial cases). Requires a genuinely new module; excluded from this round's file scope and
   explicit instruction.
2. **F7** — the static PID-1 C wrapper, its build, and its offline process-level tests. A distinct
   engineering task in C, outside this round's language/directory boundaries.
3. Whether a future caller should synthesize `terminalEvidencePersisted`/`stopReceiptPersisted`/
   `timerCancelReceiptPersisted` from a real systemd collector integration (spec.md lines 74-79) is
   unresolved; this round deliberately leaves `PROCESS_SUCCESS` unreachable rather than guessing at
   that integration's shape.
