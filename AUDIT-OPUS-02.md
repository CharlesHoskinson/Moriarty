# AUDIT-OPUS-02 — round-two independent audit of the correction to roadmap task 5.2

Auditor: Claude Opus 5 (1M context), read-only lane.
Date: 2026-09-12.
Worktree: `/home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-scope-reconciliation`.
Prior verdict: **BLOCKED** (`AUDIT-OPUS-01.md`, blocking findings F1 and F2).

**Verdict: APPROVED.**

Both blocking defects are genuinely fixed, not relabelled. The caller now carries real
kill provenance that I verified by mutation and by live probe; no invented scalar
survives in the matrix input; `PROCESS_SUCCESS` is structurally unreachable from this
caller and that is the spec-correct outcome for a caller that runs no collector, no
stop and no timer. All seven added tests are load-bearing — every one of them dies when
the fix it guards is reverted. No pre-existing test was weakened; the two changed
expectations were corrected toward the spec, and one of them (the F1 test) now asserts a
row the spec states verbatim. The two de-scoped requirements (F4, F7) are recorded
honestly and, this round, without the overstatement that made F4 a finding in the first
place. What remains is label coarseness inside an already-correct
`PROCESS_UNKNOWN`/3 outcome, plus one real but low-severity gap in what "abandonment"
means at the process level. None of it is an invented fact or a test defending broken
behaviour.

---

## 1. F1 — real provenance, or relabelling?

Real. The provenance is a distinct variable, set at exactly one place, and it gates
exactly one substitution:

- `executor-caller.mjs:105` — `try{deps.killGroup(child.pid,'SIGKILL');killRequested=true;}catch(error){killErrorClass=…}`.
  `killRequested` becomes true only if the caller's own kill call returned normally.
- `executor-caller.mjs:120` — `const matrixRawExit=killRequested?Object.freeze({kind:'unknown',code:null}):rawExit;`
- `:115` still computes and freezes the real `rawExit` first, and `:100` still returns
  it. The raw-exit retention placement my first audit praised is unchanged: the
  classifier gets a provenance-adjusted view, the *record* keeps the observation.

The mapping is the spec's own row, not an invention. `spec.md` line 66:
"collection cutoff with only running observations, **or outer kill without a retained
terminal observation** → unknown/null → PROCESS_UNKNOWN / MAIN_EXIT_UNAVAILABLE".
My round-one remedy suggested `DEADLINE_EXCEEDED`; `MAIN_EXIT_UNAVAILABLE` is the
tighter reading of the two, and the implementer cited the line.

Threading verified three ways:

- **Mutation.** Collapsing line 120 to `const matrixRawExit=rawExit;` fails exactly the
  retitled F1 test (`51 pass / 1 fail`). Hoisting `killRequested=true` above the `try`
  so a *denied* kill also launders fails exactly the new
  "a failed kill request does not hide a later retained natural main failure" test.
  The provenance is therefore load-bearing in both directions.
- **Live probe.** A kill that throws followed by a natural close still yields
  `PROCESS_FAILED`/1/`MAIN_EXIT_NONZERO`; a kill that succeeds followed by any close
  yields `PROCESS_UNKNOWN`/3/`MAIN_EXIT_UNAVAILABLE` with `result.rawExit` still holding
  the real observation.
- **Coincidence race.** Natural `exit 3` arriving after a successful kill →
  `MAIN_EXIT_UNAVAILABLE`, real `{kind:'exit',code:3}` retained in `result.rawExit`
  (probe, measured). So the downgrade is total for any post-kill close, not limited to
  signals.

Is information the spec requires retained now lost? No. The deadline fact survives in
`diagnostics.deadlineExceeded`, the kill error in `diagnostics.killErrorClass`, and the
true terminal observation in `result.rawExit`. The only loss is in the `failureCode`
label, and it errs toward "unresolved", which is the direction the spec demands
("Never convert unknown to a terminal failed defect"). The converse loss — a genuine
nonzero *exit* observation being absorbed into unknown — is broader than spec line 68
strictly requires (that line is about the signal row) but is the safe direction: it
retains ownership rather than deciding a financial failure. Recorded as N7, low.

**F1: fixed.**

## 2. F2 — invented facts, and the consequence

No invented scalar survives. `executor-caller.mjs:92-97` is the single matrix
construction site in the module (`classifyDisposition` is called once, `:98`):

```
refused:null, startupValid:true, startupWriteFailed:false, evidencePreexists:false,
invocationIdMismatch:false, rawExit:matrixRawExit,
terminalEvidencePersisted:false, stopReturnCode:null, stopErrorClass:null, stopReceiptPersisted:false,
containmentComplete:contained, timerCancelReturnCode:null, timerCancelReceiptPersisted:false,
deadlineExceeded, parentLost:false, resultWriteFailed:false
```

Both invented zeros are gone, `timerCancelReceiptPersisted:true` is gone, the two
derived persistence booleans are gone, and `stopErrorClass` no longer carries the
caller's own kill error (it is surfaced in `diagnostics.killErrorClass` instead).
`deadlineExceeded` and `containmentComplete` are the only observed inputs.

Is unreachable `PROCESS_SUCCESS` spec-correct, or an overshoot? Spec-correct. The
success row (line 74) requires "Startup, terminal, actual stop argv/rc, stop receipt, C,
cancel receipt", and line 79 states "PROCESS_SUCCESS requires confirmed cancellation as
well as C". This caller starts no transient unit, issues no collector stop, arms no
timer and writes no terminal evidence, so it cannot establish five of those six facts.
A caller that cannot observe success must not report it. The module that *can* report
success — `fault-matrix.mjs` — still does, proved by an untouched test
(`fault-matrix.test.mjs:15`). The unreachability is structural (a literal `false`), so
no future edit can silently re-open it without changing that line.

Two residues, both label-level, neither promoting anything:

- Because `terminalEvidencePersisted` is unconditionally `false`, **every** non-failure
  run from this caller now reports `EVIDENCE_WRITE_FAILED` — a write that was never
  attempted, let alone failed — and that branch precedes the deadline branch, so a
  deadline crossing whose kill was denied and whose child then exited zero is labelled
  `EVIDENCE_WRITE_FAILED` rather than `DEADLINE_EXCEEDED` (measured). The spec's closed
  vocabulary has no code for "terminal evidence never attempted", and its own table
  (line 85) does pair "unpersisted" evidence with this code, so this is vocabulary
  coarseness rather than a false claim of success. Status and exit code are the honest
  `PROCESS_UNKNOWN`/3 in every case. Recorded as N1.
- `startupValid:true` (and the `false` companions `startupWriteFailed`,
  `evidencePreexists`, `invocationIdMismatch`, `parentLost`, `resultWriteFailed`) are
  still asserted positions the caller never observed. Their entire influence is on which
  `PROCESS_UNKNOWN` code is reported — they cannot promote a result and cannot
  manufacture a terminal failure. Round one's report disclosed `startupValid`; the
  replacing report does not. Recorded as N4.

**F2: fixed.** The distinction I am drawing is deliberate: round one's invented facts
satisfied the final success gates and made `PROCESS_SUCCESS` reachable by construction;
what is left cannot change a status or an exit code.

## 3. Did the new matrix row change existing precedence?

The insertion point is as claimed: `fault-matrix.mjs:30`, after the known-main-failure
branch (`:28-29`) and before the terminal-evidence gate (`:31`). No existing branch
moved, and all 22 pre-existing matrix tests pass untouched.

But "no change to existing ordering" is not the same as "nothing previously classified
now differs", and the report's framing invites that stronger reading. I enumerated the
matrix exhaustively over 368,640 inputs (16 fields, 5 raw-exit shapes including `null`)
against a copy of the module with the new row removed:

| Change | Inputs | Assessment |
| --- | --- | --- |
| `PROCESS_SUCCESS`/0 → `PROCESS_UNKNOWN`/3/`MAIN_EXIT_UNAVAILABLE` | 2 | **A hole closed.** The old table could report success with no observed main exit. That was a latent defect in the module I praised in round one. |
| `EVIDENCE_WRITE_FAILED` → `MAIN_EXIT_UNAVAILABLE` | 2,304 | Relabel; the new code is the more specific fact. |
| `STOP_FAILED` → `MAIN_EXIT_UNAVAILABLE` | 1,920 | Relabel; absorbs a more specific cleanup failure. |
| `STOP_RECEIPT_FAILED`, `DEADLINE_EXCEEDED`, `PARENT_LOST`, `CONTAINMENT_UNRESOLVED`, `TIMER_CANCEL_FAILED`, `RESULT_WRITE_FAILED` → `MAIN_EXIT_UNAVAILABLE` | 382 | Same. |

Every difference is confined to `rawExit.kind==='unknown'`; no input with `exit`,
`signal` or `null` raw exit changed at all, so the claim that the insertion is
precedence-safe for the existing rows holds. Status and exit code are preserved in
4,606 of the 4,608 differences, and the two that change status change it in the
correct direction. No previously *correct* classification became incorrect. What the
high placement does cost is specificity: a genuine `STOP_FAILED` or `DEADLINE_EXCEEDED`
co-occurring with an unobserved main exit is now reported as `MAIN_EXIT_UNAVAILABLE`,
and no test pins the new row against those competing gates. Recorded as N2, low.

## 4. Are the seven added tests real?

52 tests, zero skipped, zero todo (`prover-lifetime` 7, `executor-caller` 22,
`fault-matrix` 23 — 45 + 7). I took a copy of `ledger/` into a scratch directory and
reverted each claimed fix in the copy. Every added test dies when its fix is undone:

| Mutation applied to the copy | Result |
| --- | --- |
| `matrixRawExit=rawExit` (F1 provenance removed) | fails #10 "outer timeout … classifies the caller kill as unknown" |
| `killRequested=true` hoisted above the `try` (a denied kill also launders) | fails #12 "a failed kill request does not hide a later retained natural main failure" |
| `terminalEvidencePersisted:true` restored (F2 reverted) | fails #2, #4, #5, #6, #9 |
| abandonment timer removed (F6 reverted) | fails #11 "settles at the abandonment bound" |
| `MAIN_EXIT_UNAVAILABLE` row deleted (F8 reverted) | fails #8, #10, #11, #30 |
| `env` key dropped from `spawn` (F10 reverted) | fails #2, #3 |
| `close` handler `try`/`catch` neutered (F11 reverted) | fails #14 "malformed close payload rejects" |
| `error` listener removed (F11 reverted) | fails #13 "child error rejects with the original spawn error" |
| durable-result schema check removed (F3 reverted) | fails #6 |
| durable-result status-agreement check removed (F3 reverted) | fails #6 |

None of the new tests is a "does not throw" test; each pins an exact
`{status,exitCode,failureCode}`, an exact error identity, an exact event log, or an
exact call count. The abandonment test carries its own 200 ms watchdog that rejects if
the executor fails to settle, which is the right shape for a liveness property. The
tautological `cleanupCalls` assertion is gone and the two real assertions in that test
survive (`fault-matrix.test.mjs:91-96`).

Spec-faithfulness of the two corrected expectations:

- The F1 test now asserts `PROCESS_UNKNOWN`/3/`MAIN_EXIT_UNAVAILABLE` — spec line 66's
  own row for "outer kill without a retained terminal observation" — and additionally
  holds `result.rawExit` at the real signal and `killGroup` at exactly one call, then
  re-checks the call count 20 ms later so a stray second kill would be caught. This
  asserts the spec, not merely what the code does.
- The two F5 tests now assert rejection **and an empty event log** (no `spawn`), which
  matches spec line 115's "no Docker start … and no proof child" and line 77's "no
  weaker fallback". They are stronger than what they replaced.

One pre-existing test lost its discriminating power as collateral of the F2 fix:
"durable completion without independently observed containment remains unknown"
(`:57-60`) now yields the same `EVIDENCE_WRITE_FAILED` whether containment is true or
false, because the terminal-evidence gate short-circuits first. It still pins an exact
code and still dies under the F2 mutation, so it is not vacuous, but it no longer tests
the property its title names. Recorded as N6.

## 5. Did F5's fix introduce a new failure mode?

No hang and no leak on a failed write. `runProverBoundedExecutor` now type-checks the
writer (`:143`) and awaits it unconditionally (`:147`) with no `try`/`catch`; a throw or
rejection propagates out of the function *before* `runPrepared` is reached, so no child
exists, no listener is attached, no timer is armed and there is nothing to clean up.
Both tests assert the event log stops at `['clock','write']`. The legacy
`requireControlRecord:false` path can no longer reach `spawn` by any route: with no
writer it refuses at `:143`, and with a failing writer it refuses at `:147`.

One hazard is worth naming, though it is pre-existing in kind: that `await` has **no
bound**. The durable *read* is wrapped in `beforeDeadline(…, +1000 ms)` (`:89`), but the
mandatory control-record write is not, so a hanging persistence worker now hangs the
executor before spawn on the only remaining path — the precise case spec line 47 calls
out ("a hanging write/fsync cannot be made safe by a clock check alone"). Recorded as
N5, low.

## 6. Is F6's abandonment bound sound?

The promise always settles. Verified by probe in all four shapes:

| Case | Outcome |
| --- | --- |
| kill succeeds, child never closes | settles at the bound, `MAIN_EXIT_UNAVAILABLE`, `killErrorClass:null` |
| kill throws, child never closes | settles at the bound, `MAIN_EXIT_UNAVAILABLE`, `killErrorClass:'EPERMish'` |
| child closes after the abandonment settle, then emits a second close and an `error` | no double settle, no throw, no unhandled rejection |
| child closes just before the deadline | zero kills — `clearTimers()` cancelled the deadline timer |

`clearTimers()` is called on every settlement path (`:108`, `:112`, `:116`, `:122`) and
clears both timers, so nothing is left armed. `settled` guards each entry point. The
`if(settled)return` inside the deadline callback sits *after* the kill, which is the
wrong order in principle — a queued deadline callback could signal a reaped pid group,
which is exactly what spec line 71's identity rule forbids — but `settled=true` and
`clearTimers()` are synchronous with the `close` event, so Node cancels the expired
timer before it runs; I could not reach it, and I record it as an ordering nit rather
than a finding.

What the bound does *not* do is abandon the child at the process level. Nothing
destroys or unrefs the child's `stdout`/`stderr`, nothing removes the listeners, no
second kill or escalation is attempted, and the result exposes no pid or process group.
Measured with a real spawn: the promise settled at 305 ms and the parent process stayed
alive until the abandoned child exited 4,028 ms later, because the child's pipes are
live handles. So a caller that "abandons" still cannot exit, still accumulates
diagnostics into a buffer no one will read (capped at 1 MiB), and cannot retain the
outstanding cleanup owner that the spec's deadline row requires ("cleanup owner" among
the retained facts). The finding as written — "the wait after the deadline kill is
unbounded" — is fixed; the residue is new and low. Recorded as N3.

## 7. Byte-identical claim for `prover-lifetime.mjs` and its test

Corroborated. Both files have `mtime == ctime == 17:39` (`prover-lifetime.mjs` 17:39:43,
`prover-lifetime.test.mjs` 17:39:30), which predates both `AUDIT-OPUS-01.md` (17:58) and
every file the correction round touched (`fault-matrix.test.mjs` 18:12,
`fault-matrix.mjs` 18:13, `executor-caller.test.mjs` 18:28, `executor-caller.mjs`
18:29). No write of any kind occurred after 17:39, so the bytes cannot have changed —
had they been rewritten with identical content, `mtime` would have moved. Content spot
checks agree with my round-one citations (the strict `entry < latestStart` rule, the
`min(entry+1500, control.killDeadline)` deadline, the six-line ASCII encoder, the
`120 s` boundary test at exactly test-file lines 20-26), and the suite still holds
exactly 7 tests. Caveat on method: these files are untracked, so no git baseline exists
to diff against; the claim rests on filesystem timestamps plus content inspection, not
on a stored digest. A digest recorded at round one would have made this direct.

## 8. Regression and overreach

- **Nothing financial, networked or retained is touched.** The overreach grep over all
  six files returns exactly two hits, both benign and both pre-existing:
  `prover-lifetime.mjs`'s own disclaimer comment, and `node:fs` inside the test that
  reads its own source for the purity check. No Docker, wallet, budget, charge, debit,
  accounting, `node:net`, HTTP or `process.env` read anywhere. No product module writes
  a file; the control-record write is still an injected callback.
- **`spawn` env is now closed, not empty-by-accident.** `env:options.env??{}` (`:76`)
  with a validated flat string map (`stringMap`, `:31-34`). This removes the silent full
  inheritance F10 named. It does not implement the two invocation variables — that is
  F4, correctly still open.
- **No production wiring.** Nothing outside the three modules and their tests imports
  `classifyDisposition`, `runExitRetainingExecutor` or `runProverBoundedExecutor`, so
  no existing caller's behaviour can have changed. `git status` shows one modified file
  (`package.json`) and six untracked new files; no pre-existing test file was touched.
- **`package.json`** gains only the three new files appended to `test:ledger` (the
  existing 25-file list is unchanged and in order) and retains `test:executor`. No
  dependency, no script weakened, no aggregate shortened. This is the F12 wiring fix.
- **No existing expectation was weakened.** Two changed expectations are corrections
  toward the spec (the F1 test to the spec's own `MAIN_EXIT_UNAVAILABLE` row; the two F5
  tests from "success without a control record" to "rejection with no spawn"). The
  further changes to already-passing executor expectations
  (`PROCESS_SUCCESS`→`EVIDENCE_WRITE_FAILED`, `CONTAINMENT_UNRESOLVED`→
  `EVIDENCE_WRITE_FAILED`) are consequences of the F2 correction, not accommodations of
  broken behaviour: the caller can no longer claim success or reach the containment gate
  because it observes neither.
- **F13 reconciled.** I ran `openspec validate --all --strict` myself: `12 passed,
  0 failed (12 items)`. The implementer's count was right and the brief's "13" was
  stale. Nothing in this diff touches `openspec/`.
- **Honesty of the de-scoping.** F4 and F7 are stated plainly as not attempted, and the
  report explicitly retracts round one's over-generous claim that the handoff
  requirement was implemented, naming the seven denial cases as unexercised. I confirmed
  the absence: no `AF_UNIX`, `MORIARTY_INVOCATION_SOCKET`, `MORIARTY_INVOCATION_NONCE`
  or `prover_lifetime.c` anywhere. That is the record the requirement deserves.
  The unused `decodeProverControlRecord` / `computeWrapperKillDeadline` /
  `validateClockIdentity` exports still have no production reader, which is the
  unavoidable tail of F7's de-scoping.

---

## Prior findings

| Id | Severity (round 1) | Disposition |
| --- | --- | --- |
| F1 | high, blocking | **fixed** — real provenance, mutation-proved, spec-cited mapping |
| F2 | high, blocking | **fixed** — no invented scalar; `PROCESS_SUCCESS` structurally unreachable and correctly so |
| F3 | medium | **fixed** — schema and exit/status agreement enforced; note N6 (the gate it feeds is now unreachable) |
| F4 | high | **descoped** — not attempted, honestly recorded, round one's overstatement retracted |
| F5 | medium | **fixed** — control record mandatory, unconditional, no swallow, no spawn on failure |
| F6 | medium | **fixed** — promise always settles; see N3 for the process-level residue |
| F7 | medium | **descoped** — not attempted, honestly recorded |
| F8 | low | **fixed** — `MAIN_EXIT_UNAVAILABLE` row added, `stopErrorClass` no longer carries the caller's own kill error; see N1, N2 |
| F9 | low | **fixed** — tautological assertion deleted, real assertions kept |
| F10 | low | **fixed** — explicit validated closed `env`; the two invocation variables remain F4 |
| F11 | low | **fixed** — `error` path tested, `close` body wrapped, malformed payload rejects |
| F12 | low | **partially fixed** — suite wired into `test:ledger`; the three consumerless prover exports remain, tied to descoped F7 |
| F13 | low | **fixed** — measured 12 passed / 0 failed; the stale number was in the brief, not the report |

## New findings

### N1 — low/medium — every clean run reports a write failure that never happened

With `terminalEvidencePersisted` unconditionally `false`, `fault-matrix.mjs:31` makes
`EVIDENCE_WRITE_FAILED` the outcome for every run from this caller that is not a known
main failure or an unknown exit — including a child that exited zero with a valid,
schema-checked durable result. No terminal write was attempted, so the retained code
names a false fact, and because `:31` precedes `:34` it also masks a real deadline
crossing (kill denied, child then exits zero → `EVIDENCE_WRITE_FAILED`, not
`DEADLINE_EXCEEDED`; measured). The closed vocabulary has no "terminal evidence not
attempted" code. Status and exit code are correct in every case, so this is a labelling
defect, not a manufactured outcome.

### N2 — low — the new row absorbs more specific codes for unknown exits

Exhaustive comparison (368,640 inputs) shows the new `MAIN_EXIT_UNAVAILABLE` row changes
4,608 classifications, all with `rawExit.kind==='unknown'`. Two close a real hole
(`PROCESS_SUCCESS` was previously reachable with no observed main exit); the other 4,606
keep `PROCESS_UNKNOWN`/3 but replace `STOP_FAILED`, `STOP_RECEIPT_FAILED`,
`DEADLINE_EXCEEDED`, `PARENT_LOST`, `CONTAINMENT_UNRESOLVED`, `TIMER_CANCEL_FAILED` and
`RESULT_WRITE_FAILED` with the less specific code. The report's "no change to existing
ordering" is literally true but reads as a stronger claim than the code supports. No
test pins the new row against those competing gates.

### N3 — low/medium — abandonment settles the promise but does not release the child

After the abandonment bound fires, nothing destroys or unrefs the child's `stdout` /
`stderr`, no listener is removed, no escalation or second kill is attempted, and the
result exposes no pid or process group. Measured with a real spawn: the promise settled
at 305 ms, the parent process stayed alive 4,028 ms until the abandoned child exited,
and diagnostics kept accumulating (capped at 1 MiB) with no consumer. So the bound is on
the promise, not on the caller's liveness, and the "cleanup owner" the spec's deadline
row requires retained cannot be read out of the result.

### N4 — low — asserted-but-unobserved boolean positions remain, and one lost its disclosure

`startupValid:true`, with `startupWriteFailed`, `evidencePreexists`,
`invocationIdMismatch`, `parentLost` and `resultWriteFailed` at `false`, are still
positions this caller never observed (it starts no transient unit and reads no startup
identity). They cannot promote a result or manufacture a terminal failure — their whole
influence is on which `PROCESS_UNKNOWN` code is reported — but round one's report
disclosed `startupValid` and the replacing report, which supersedes it in full, does
not. The disclosure should be carried forward.

### N5 — low — the mandatory control-record write is awaited without a bound

`executor-caller.mjs:147` awaits `writeControlRecord` with no deadline, in contrast with
the durable read at `:89`, which is wrapped in `beforeDeadline(+1000 ms)`. F5's fix puts
this unbounded await on the only path to spawn, so a hanging persistence worker hangs
the executor before any child exists — the hazard spec line 47 names explicitly. The
hang is pre-spawn and therefore harmless to owned resources, which is why this is low.

### N6 — low — the F3 validation no longer gates anything

`containment()` now validates schema and exit/status agreement, but its result reaches
`fault-matrix.mjs:35` only after `:31`, which the caller can never pass. So
`containmentComplete` has no effect on the disposition; the validation's only observable
consequence is `diagnostics.resultReadFailed`. The new F3 test asserts that flag and
therefore remains load-bearing, but the pre-existing test "durable completion without
independently observed containment remains unknown" now returns the same code for
containment true and false and no longer tests its own title.

### N7 — low — the provenance downgrade is broader than the spec requires

`killRequested` suppresses *every* post-kill observation, including a genuinely retained
`exit`/nonzero that merely coincided with the deadline (measured: `exit 3` after a
successful kill → `MAIN_EXIT_UNAVAILABLE`). Spec line 68 constrains the *signal* row;
absorbing a nonzero exit code as well is a conservative over-application. The direction
is safe (ownership retained rather than a financial failure decided) and the real
observation survives in `result.rawExit`, so this is an observation, not a defect to
repair before sign-off.

### N8 — informational — validated options are re-read at spawn time

`args` and `env` are read inside `runPrepared` (`:76`), after the `await
writeControlRecord` boundary that follows validation, so an injected writer could mutate
either between the check and the use. `args` is spread (a copy is made, but at spawn
time, from the possibly-mutated array) and `env` is passed by reference without copying.
This requires a hostile injected dependency, which is outside the module's trust model,
but freezing or copying the validated values at validation time would close it.

---

## Evidence

- `npm run test:executor`: `1..52 / pass 52 / fail 0 / skipped 0 / todo 0`.
  File counts: `executor-caller.test.mjs` 22 tests, `fault-matrix.test.mjs` 23,
  `prover-lifetime.test.mjs` 7 (45 + 7). No `skip`, `todo` or `only` markers.
- Mutation testing: `ledger/` copied to a scratch directory; ten separate reversions of
  the claimed fixes, each failing the expected added test and no others
  (table in section 4). The copy was restored and verified identical to the product file
  after the last mutation; no product file was written at any point.
- Live probes: abandonment with a successful kill, with a throwing kill, with a late
  close after settle, with a close just before the deadline, and a coincident natural
  exit after a successful kill; plus one real `spawn` measuring parent liveness after
  abandonment (305 ms settle, 4,028 ms process exit).
- Exhaustive matrix comparison: 368,640 inputs, current module versus a copy with the
  new row removed; 9 distinct difference classes, all under `rawExit.kind==='unknown'`.
- `openspec validate --all --strict`: `Totals: 12 passed, 0 failed (12 items)`.
- `git status --porcelain`: one modified file (`experiments/moriarty-midnight-financial/package.json`),
  six untracked new ledger files, plus the audit/report documents. `git log -1` =
  `056e6622`, unchanged; nothing committed.
- `stat` timestamps for the six files (section 7), establishing which files the
  correction round wrote and which it did not.

Files written by this audit: `AUDIT-OPUS-02.md`, `AUDIT-OPUS-02.json`. Nothing else.
