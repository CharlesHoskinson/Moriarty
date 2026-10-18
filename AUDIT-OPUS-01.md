# AUDIT-OPUS-01 — independent read-only audit of roadmap task 5.2

Auditor: Claude Opus 5 (1M context), read-only lane.
Date: 2026-09-12.
Worktree: `/home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-scope-reconciliation`.
Reviewed bytes: the three new modules, the three new test suites, the `package.json`
diff, `openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md`,
`openspec/changes/language-to-ledger-lifecycle/design.md`, `ledger/receipt.mjs`
(`beforeDeadline`), `ledger/launch-local.mjs` (CLI argv/result shape) and the
implementer's `FOREMAN_REPORT.md`/`.json`.

**Verdict: BLOCKED.**

One real defect, cemented by a passing test, converts the caller's own outer-timeout
kill into a terminal financial failure (`PROCESS_FAILED` / `MAIN_SIGNAL`) that the spec
forbids three separate times. A second high finding is that the only gate standing
between child-exit-zero and `PROCESS_SUCCESS` is partly composed of facts the caller
invents rather than observes. Neither is a large repair, and the rest of the work is
good: the fault matrix and the prover-lifetime codec are genuinely strong, genuinely
pure, and genuinely tested.

---

## 1. Are the 45 assertions real?

45 test cases (prover-lifetime 7, executor-caller 16, fault-matrix 22). All 45 are
load-bearing: every one pins an exact value, an exact byte string, an exact error
code, or an exact call ordering. None is a "does not throw" test. None would pass
against a stub: the fault-matrix suite flips exactly one field per row and demands a
specific `{status,exitCode,failureCode}` triple, so a stub returning any constant
fails 21 of 22; the prover-lifetime suite demands literal bytes
(`'moriarty.prover-lifetime/1\n…'`) and exact rejection codes; the executor suite
counts `spawn`/clock invocations and asserts the precise event order
`['clock','write','clock','spawn']`, which a stub cannot satisfy.

Quality highlights that are unusual and worth naming:

- `executor-caller.test.mjs:85-99` proves refusal happens *before* any clock sample or
  spawn by counting both, and proves argv accessors and hidden array properties are
  rejected *without invoking the getter* (`reads===0`). That is a real property, not a
  restatement.
- `executor-caller.test.mjs:126-129` asserts the event log stops at `['clock','write']`
  when the required control write fails — the strongest test in the set, because it
  proves absence of a spawn rather than presence of a value.
- `fault-matrix.test.mjs:36-38` holds a known main failure at `PROCESS_FAILED` while
  simultaneously degrading eleven downstream cleanup facts. That is the spec's
  hardest ordering rule and it is tested head-on.
- `prover-lifetime.test.mjs:20-26` tests the inclusive/exclusive boundary at exactly
  `outer+120s` in all three directions (before, at, after).

Two caveats, neither making a test vacuous:

- `fault-matrix.test.mjs:91-93` declares `let cleanupCalls=0` and then asserts
  `cleanupCalls===0`. Nothing in the test can increment it; that single assertion is a
  tautology. The same test's other two assertions (the source-purity regex and the
  unresolved-evidence classification) are real, so the test survives, but the
  tautology should go.
- Three executor tests (`:77-83`, `:131-134`, `:136-139`) are load-bearing but lock in
  behaviour that diverges from the spec (findings F1 and F5). A test that pins the
  wrong expectation is worse than no test, because it defends the defect.

**Load-bearing count: 45 of 45 tests; 42 of 45 are load-bearing *and* spec-faithful;
1 individual assertion is dead.**

Mocking is proportionate, not total. The fakes are `spawn`, `monotonicNow`,
`killGroup`, `writeControlRecord` and `readDurableResult` — exactly the transport
boundary the spec requires to be injectable ("Tests SHALL use … explicit fake
transports"). The real code under test (field closure, argv validation, byte encoding,
diagnostic bounding, ordering, classification) is exercised, not stubbed.

---

## 2. Spec fidelity on the three scoped requirements

### 2a. "Prover lifetime established before daemon startup" — partially met, honestly disclosed

Met: bounds are derived from one authenticated outer-start sample
(`executor-caller.mjs:112-113`), the control bytes are encoded and written *before*
`spawn` (`:115-119`), the record is byte-exact to the spec's closed six-line ASCII
format including the `512`-byte bound and lowercase-hex/UUID constraints
(`prover-lifetime.mjs:48-58`), the latest-start rule is `entry < latestStart` so
"wrapper entry at or after outer+120" refuses (`:33-36`), and
`computeWrapperKillDeadline` is exactly `min(entry+1500, control.killDeadline)`
(`:38-43`) as spec line 109 requires.

Not met: spec line 103 requires "one small import-free static PID 1 executable from
`plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c`" and line 123 requires
offline tests that "compile the actual static wrapper and use controlled child
programs that exit, hang, fork, change process group and ignore signals". No `.c` file
exists anywhere in the tree; no compile step, no child-program tests, no
fork/process-group/reap/signal-forwarding behaviour. `prover-lifetime.mjs` is a pure
model of the wrapper's arithmetic and control format, not the wrapper. Consequently
the named scenario "Post-start receipt loss cannot unbound the prover" is *modelled*
(`executor-caller.test.mjs:119-124` proves the bounds are not recomputed or widened
after the post-start read fails) but not *established* — nothing in the changed set can
show that "the pre-established PID 1 control still ends all original prover
descendants by its fixed bound". The implementer states this plainly in
`FOREMAN_REPORT.md` ("The compiled static PID-1 C wrapper … is explicitly out of
scope"), so this is a scope shortfall, not a false claim. See F7.

Also note that three of the six exports — `decodeProverControlRecord`,
`computeWrapperKillDeadline`, `validateClockIdentity` — have no consumer anywhere in
the changed set; they exist for the wrapper that was not written. Their tests validate
a model with no production reader (F14).

### 2b. "Identity-safe external containment and deterministic fault disposition" — met

`fault-matrix.mjs` is the best part of this change. It implements the closed
disposition table with a total precedence order that respects both ordering rules the
spec states explicitly:

- spec line 54, "A malformed/mismatched identity takes precedence over apparent exit
  fields" → `fault-matrix.mjs:27` (identity) precedes `:28` (known main failure), and
  `fault-matrix.test.mjs:24-26` proves it against `rawExit {kind:'exit',code:9}`.
- spec line 79, "A known main failure stays PROCESS_FAILED when supplementary cleanup
  evidence is incomplete … otherwise incomplete evidence yields PROCESS_UNKNOWN" →
  `:29` precedes every cleanup gate, and `fault-matrix.test.mjs:36-38` proves it.

Every row of the table maps to a branch, `REFUSED`/2 carries the validator code
through unchanged, and each of the eleven `PROCESS_UNKNOWN`/3 failure codes is
reachable and tested. The scenario "Safety cleanup does not manufacture evidence" is
honoured at the module level: the classifier has no imports at all, performs no
cleanup, and issues no stop — it cannot manufacture anything. The residual complaint
is not with this module but with how `executor-caller.mjs` feeds it (F2).

One genuine gap in the table's vocabulary: the spec's classification table (line 66)
distinguishes `MAIN_EXIT_UNAVAILABLE` ("collection cutoff with only running
observations, or outer kill without a retained terminal observation") from
`EVIDENCE_WRITE_FAILED`. The matrix has no `MAIN_EXIT_UNAVAILABLE`, so an unobserved
exit is reported as a write failure (F8). That requirement is outside the lane's three,
so this is low, but it is a real mislabel.

### 2c. "Concrete one-shot runner invocation handoff" — not met as specified

Spec lines 29-40 fix a concrete protocol. The delivered caller implements none of it:

| Spec requirement (line 30-36) | Delivered |
| --- | --- |
| one private AF_UNIX stream socket in a mode-0700 runtime dir, socket mode 0600 | absent |
| cryptographically random 256-bit nonce | absent |
| "passes only `MORIARTY_INVOCATION_SOCKET` and `MORIARTY_INVOCATION_NONCE` as explicitly set child environment values, overwriting any inherited values" | no `env` key passed to `spawn` at all (`executor-caller.mjs:63`); the detached child inherits the entire parent environment |
| constant-time nonce equality, authority equality, socket peer UID, `/proc` PID ancestry with retained start ticks | absent |
| "Exactly one successful handshake is allowed" | absent |
| durable invocation event appended after spawn and before answering the handshake; "A failed append yields no handshake" | absent |
| "EOF before setup refuses startup. EOF after startup initiates owned safety cleanup and yields unknown" | absent |

What *is* delivered from this requirement: the fixed argv is passed through unmutated,
stdin is `'ignore'` (DEVNULL), the child is detached so the kill can target its group,
and the outer monotonic start is sampled immediately before spawn. That is roughly the
argv-and-clock corner of the requirement.

The named scenario "Actual runner-to-script protocol" enumerates seven denial cases —
forged nonce, wrong authority/reservation, stale store event, different launcher PID,
replay, boot mismatch, parent disappearance. None is exercised, and none can be
without the channel. `FOREMAN_REPORT.md` maps this scenario to the test "actual argv
protocol is handed to exactly one detached piped child without mutation"; that mapping
is too generous and is the one place the report's self-assessment overstates coverage
(F4). The report does, to its credit, disclose the absence of the socket in
"Deliberately left open" item 1.

---

## 3. The four hard interface rules

1. **Prover lifetime established before daemon startup — PASS.** Bounds computed at
   `executor-caller.mjs:113`, control bytes written at `:117`, spawn at `:119`→`:63`.
   The ordering is asserted as an exact event log
   (`executor-caller.test.mjs:114`: `['clock','write','clock','spawn']`) and the
   negative case is asserted too (`:128`: `['clock','write']` and no spawn when the
   required write throws). Caveat: the `requireControlRecord:false` opt-out defeats it
   by configuration (F5).
2. **Losing a receipt after start must not unbound the prover — PASS.**
   `proverLifetimeBounds` is computed once from the single outer-start sample, frozen
   (`:113`), and returned unchanged; `readDurableResult` throwing cannot reach it
   (`executor-caller.test.mjs:119-124` asserts equality with
   `computeProverLifetimeBounds(900n)` and exactly two clock samples). There is no code
   path that recomputes or extends a bound after spawn.
3. **A zero child exit must never by itself promote a result, and neither must stdout —
   PASS on the letter, WEAK in substance.** Exit zero alone does not promote: the
   durable read must succeed and report containment
   (`executor-caller.test.mjs:43-49` proves that child 0 plus
   `{"status":"PASS","containmentComplete":true}` on stdout stays
   `PROCESS_UNKNOWN` when the durable read throws; `:51-54` proves
   `containmentComplete:false` stays `CONTAINMENT_UNRESOLVED`). Stdout is never read
   for disposition — it only lands in bounded `diagnostics`. But the promotion bar is
   one unvalidated boolean on a duck-typed object plus four invented facts; see F2 and
   F3.
4. **Safety cleanup must never manufacture evidence, and raw exit is retained before
   any cleanup — raw-exit retention PASSES; non-manufacture FAILS.** Retention is
   correct and deliberate: `const rawExit=Object.freeze(classifyRawExit({status,signal}))`
   is the first statement of the `close` handler (`executor-caller.mjs:76`), before the
   `settled` check, before the durable read, before classification. But the caller then
   hands the matrix `stopReturnCode:0`, `stopReceiptPersisted:<derived>`,
   `timerCancelReturnCode:0` and `timerCancelReceiptPersisted:true` for operations it
   never performed (F2), and it treats the signal produced by *its own* SIGKILL as a
   retained main signal (F1). Both are manufacture.

---

## 4. Did it invent a launcher framework? — No

This is clean. There is no new launcher, no supervisor, no daemon, no registry, no
state machine around processes. The caller spawns the *existing* documented CLI
contract — `executor-caller.test.mjs:9` uses
`['ledger/launch-local.mjs','--run','--plan',…,'--sha256',…]`, which matches
`launch-local.mjs:254`'s own argv check (`a[0]==='--run' && a[1]==='--plan' &&
a[3]==='--sha256'`) exactly. The bounded wait reuses `beforeDeadline` from the existing
`receipt.mjs` rather than reimplementing a race-and-timer
(`executor-caller.mjs:7`, `:80`). `containment()` reads the shape the existing
`launchLocalFinancialCase` already returns (`{status, cleanup:{containmentComplete}}`),
so it is fitted to the existing result convention rather than replacing it. The result
reader itself is injected, so no parallel result-format authority is introduced. Total
added surface is 238 lines across three modules and a single `test:executor` script.
The design constraint is respected.

## 5. Determinism of `fault-matrix.mjs` — verified pure

Read line by line: zero `import` statements, no `Date`, no `Math.random`, no
`process.*`, no `node:fs`, no `node:child_process`, no timers, no mutable
module-level state (only two frozen-by-convention string/function constants). Output is
a function of the argument alone; the same input always yields the same
`{status,exitCode,failureCode}`. Field closure is enforced structurally
(`Reflect.ownKeys` length equality plus a sorted key-set comparison, `:8-11`), so getters,
symbol keys and non-enumerable properties are rejected rather than read — which also
means classification cannot execute caller code. `fault-matrix.test.mjs:88-90`
additionally greps the source for `node:fs|node:child_process|process.kill|spawn(`.
Determinism: confirmed.

## 6. Overreach — none found

`grep -niE "docker|wallet|budget|charge|debit|fetch|http|net\.|node:net|accounting|network"`
over all six new files returns exactly two hits: the word "network" inside
`prover-lifetime.mjs`'s own disclaimer comment, and `node:fs` in the *test* that reads
its own source for the purity grep. No accounting, budgets, charges, wallets, sockets,
network, Docker, systemd or retained build artifacts are touched. No file is written by
any product module — the control-record write is an injected callback, which is the
right boundary. `package.json` gains exactly one script and no dependency. Nothing is
written outside the package. No test spawns a real `launch-local.mjs`. No module claims
financial acceptance, settlement, or live containment support; the one containment
claim that *is* made is the `containmentComplete` boolean taken from the child's own
document, which is a fidelity issue (F3) rather than a live-acceptance claim.

`FOREMAN_REPORT.md` is unusually honest for an implementation record: it discloses the
missing socket, the missing C wrapper, the literal `startupValid`/`evidencePreexists`/
`invocationIdMismatch` defaults, the `terminalEvidencePersisted`/`stopReceiptPersisted`
conflation, the absent production wiring, its own chosen precedence order, and a
counter-to-expectation `openspec validate` total (12, where this audit's brief states
13). That disclosure is the reason most of the shortfalls below are medium rather than
critical. It does not disclose F1 or the invented zeros in F2.

---

## Findings

### F1 — high — outer-timeout kill is reported as a known main failure (BLOCKING)

`executor-caller.mjs:73` sends `SIGKILL` to the child's process group at
`outerTimeoutMs`. The child then closes with `signal='SIGKILL'`, `classifyRawExit`
returns `{kind:'signal',code:9}` (`:49`), and because
`fault-matrix.mjs:28-29` treats any signal as a known main failure *before* it reaches
the `deadlineExceeded` branch at `:33`, the disposition becomes
`{status:'PROCESS_FAILED',exitCode:1,failureCode:'MAIN_SIGNAL'}` — even though
`deadlineExceeded` is `true` in the same matrix. The caller has fabricated a terminal
financial failure out of its own kill request.

The spec forbids this three times:

- line 68: "A safety kill is a known signal only if an actual matching terminal
  observation establishes the signal row; **the kill request/outer exit alone does
  not**."
- line 72: "unloaded defaults, **outer signal** or an unobserved main exit **remain
  PROCESS_UNKNOWN** rather than successful or a fabricated known main failure."
- line 90, table row: "deadline crossing or parent loss after startup →
  `PROCESS_UNKNOWN` / 3 / `DEADLINE_EXCEEDED` or `PARENT_LOST` unless retained known
  main failure" — and there is no retained terminal observation here, only the caller's
  own kill.
- line 132 reinforces it for the consuming side: "parent timeout/signal … with outcome
  unknown", and "Never convert unknown to a terminal failed defect".

`executor-caller.test.mjs:77-83` asserts the wrong outcome as correct
(`assert.deepEqual(result.disposition,{status:'PROCESS_FAILED',exitCode:1,failureCode:'MAIN_SIGNAL'})`)
and its title frames it as a feature ("retains close signal"). The matrix is not at
fault — its precedence is spec-correct for a *retained* signal. The fault is that the
caller has no provenance tracking distinguishing "the unit's main process died of a
signal" from "I killed this child at my deadline", and passes the latter as the former.

Why it matters: this is the exact error class the spec was written to prevent. A
deadline crossing with unresolved containment becomes a clean, terminal
`PROCESS_FAILED`/exit 1, which downstream (`spec.md:132`) maps to "CLI 4 and failed
reservation" instead of `runner_unresolved` with ownership retained active. An unknown
with live resources outstanding would be recorded as a decided failure.

### F2 — high — the success gate is fed facts the caller never observed

`executor-caller.mjs:83-88` constructs the matrix input with hardcoded
`stopReturnCode:0`, `timerCancelReturnCode:0`, `timerCancelReceiptPersisted:true`, and
derives `terminalEvidencePersisted`/`stopReceiptPersisted` from
`terminalKnown && !resultReadFailed` — i.e. from "the exit was a known kind and I could
read a result". No stop was issued, no stop receipt exists, no timer was armed or
cancelled, and no terminal evidence write was observed. These are the final four gates
before `PROCESS_SUCCESS` (`fault-matrix.mjs:30-35`), so all four are satisfied by
construction.

Spec line 130: "Use null for unavailable scalar observations, **not invented zero**."
Spec line 79: "`C` means **independently observed**, durably retained containment";
"PROCESS_SUCCESS requires confirmed cancellation as well as C." Spec line 98: safety
cleanup "leaves unknown main outcome and unresolved resources **explicit**."

`FOREMAN_REPORT.md` item 4 discloses the `terminalEvidencePersisted`/
`stopReceiptPersisted` conflation but not the invented `timerCancel*` and
`stopReturnCode:0`. The honest shape here is `null` for unavailable scalars and
`false` for unestablished receipts, which would make `PROCESS_UNKNOWN` the correct and
only reachable non-failure outcome for this caller until the real observations are
wired — and would have surfaced the gap instead of papering it.

### F3 — medium — `PROCESS_SUCCESS` is promoted from an unvalidated, child-authored object

`containment()` (`executor-caller.mjs:54-59`) accepts any plain object and reads
`containmentComplete` from either the top level or a nested `cleanup`, with no schema,
digest, identity or status validation. The child's own `status` field is never read.
The test fixture (`executor-caller.test.mjs:10`) is
`{schema:'moriarty.local-financial-integration/1',status:'PASS',cleanup:{containmentComplete}}`
— not the `moriarty.loan-process-result/1` the spec defines — and the caller never
checks even that. Spec line 132 requires the caller to "validate schema/digests/current
invocation identities and compare the child's exit with the disposition", and "Child 0
requires valid `PROCESS_SUCCESS` and all success evidence". No exit/disposition
mismatch detection exists. Containment is asserted on the child's word rather than
independently observed.

Mitigation: the `moriarty.loan-process-result/1` requirement is outside the lane's
three, and `readDurableResult` is injected so a production caller could validate
upstream. But the module is what emits `PROCESS_SUCCESS`, so the validation gap lands
here.

### F4 — high — "Concrete one-shot runner invocation handoff" is reinterpreted, not implemented

See section 2c for the clause-by-clause table. None of socket, nonce, env pinning, peer
UID, PID ancestry, one-shot handshake, pre-handshake durable invocation event, or EOF
semantics is present, and none of the scenario's seven denial cases is exercised. The
spec is explicit that this cannot be assumed: "Source inspection and an actual isolated
launcher test SHALL establish that the pinned Foreman launcher inherits these
environment values through strong containment; if it does not, repair/pin the minimal
launcher adapter before source approval **rather than assume transport exists**"
(line 30). The implementer discloses the omission but `FOREMAN_REPORT.md` still lists
the requirement under "Spec requirements implemented" and maps the scenario to the argv
test. Either the lane's scope statement should be amended to the argv-only
reinterpretation, or the requirement stays open — it should not be recorded as met.

### F5 — medium — `requireControlRecord:false` is a sanctioned weaker fallback

`executor-caller.mjs:108` defaults the flag to `true` (good), but when set `false` the
function spawns with no control record at all, and `:117`
(`catch(error){if(requireControlRecord)throw error;}`) swallows a control-write failure
and proceeds to spawn. Two tests certify this as success
(`executor-caller.test.mjs:131-139`, both asserting `PROCESS_SUCCESS`). The spec's
startup-race table (line 115) says "Container/config/control validation or pre-start
file/directory fsync fails → **No Docker start**; existing container unchanged; retain
refusal/consumption and no proof child", and line 77 says unsupported containment
returns `CONTAINMENT_UNSUPPORTED` "with no resource start and **no weaker fallback**".
A function named `runProverBoundedExecutor` should not have a mode in which the prover
bound is never persisted and the run still reports success.

### F6 — medium — the wait after the deadline kill is unbounded

`executor-caller.mjs:73` fires exactly one `killGroup` and clears nothing else; the
returned promise settles only on `close` or `error`. If the group kill throws (recorded
into `killErrorClass` and then ignored) or the child is unreapable, nothing ever
resolves or rejects — there is no escalation, no second bound, and no abandonment path.
Spec line 47: "Bounded subprocess supervision SHALL cover service commands and
persistence workers; a hanging write/fsync cannot be made safe by a clock check alone."
No test covers a child that fails to close after the kill.

### F7 — medium — the specified static PID 1 wrapper does not exist, so its scenario is modelled, not established

Spec line 103 names `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c` with
"reproducible build instructions and offline tests"; line 123 requires those tests to
"compile the actual static wrapper" and drive children that "exit, hang, fork, change
process group and ignore signals", plus parent-death, reap and signal behaviour. None
exists (`find` for `prover_lifetime*` returns nothing). The delivered module is a pure
model: correct arithmetic and byte format, no process behaviour. The lane's named
scenario "Post-start receipt loss cannot unbound the prover" therefore holds only in
the weaker sense that the caller does not widen its own bounds. Disclosed in
`FOREMAN_REPORT.md`, so this is a scope finding rather than a false claim, but the
requirement cannot be recorded as satisfied on this evidence.

### F8 — low — an unobserved exit is mislabelled `EVIDENCE_WRITE_FAILED`; `stopErrorClass` is reused for the caller's own kill error

Because `terminalEvidencePersisted` is `terminalKnown && !resultReadFailed`
(`executor-caller.mjs:85`), an unknown close yields `EVIDENCE_WRITE_FAILED` although no
write failed; the spec's code for that situation is `MAIN_EXIT_UNAVAILABLE`
(`spec.md:66`), which the matrix does not define. Separately, `stopErrorClass` — the
spec's field for a failed collector `systemctl stop` — is loaded with the error class of
the caller's own `killGroup` (`:85`), so a deadline crossing whose kill errored is
reported as `STOP_FAILED` rather than `DEADLINE_EXCEEDED` (`fault-matrix.mjs:31`
precedes `:33`). Both outcomes are `PROCESS_UNKNOWN`/3, so the operational effect is
small, but the retained failure code names the wrong fact.

### F9 — low — tautological assertion inside the anti-manufacture test

`fault-matrix.test.mjs:91,93`: `let cleanupCalls=0` … `assert.equal(cleanupCalls,0)`.
Nothing can increment it. The other two assertions in that test are real; this one
should be deleted or replaced with an injected cleanup spy that the classifier could
in principle call.

### F10 — low — no child environment is set, so the detached child inherits everything

`executor-caller.mjs:63` passes `{cwd,detached:true,stdio:['ignore','pipe','pipe']}`
with no `env`. Spec line 30 requires the runner to pass only the two invocation
variables "as explicitly set child environment values, overwriting any inherited
values", and line 36 that "no environment value alone grants authority". Full
inheritance into a detached financial launcher is a containment gap; it may be
deliberate, since `launch-local.mjs` consumes `MORIARTY_*` environment inputs, but
nothing records that decision.

### F11 — low — untested failure paths: spawn `error`, and a malformed `close` payload

No test covers the `child.once('error')` path (`executor-caller.mjs:74`), which rejects
with the raw error and produces no disposition at all — arguably a `REFUSED`/no-start
case per `spec.md:94`. And `classifyRawExit` is called synchronously inside the `close`
listener (`:76`), outside any `try`, so a malformed `(status,signal)` pair from an
injected fake throws as an uncaught exception rather than rejecting the promise.

### F12 — low — the new suite is outside every aggregate script, and half the prover exports have no consumer

`package.json` adds `test:executor` but neither `test` (`tests/*.test.mjs`) nor
`test:ledger` includes the three new files, so the suite runs only when invoked by
name and can rot silently. Separately, `decodeProverControlRecord`,
`computeWrapperKillDeadline` and `validateClockIdentity` are exported and tested but
called by nothing in the changed set — they are the interface for the wrapper in F7.

### F13 — low — record discrepancy on `openspec validate`

This audit's brief states "openspec validate --all --strict: 13 passed";
`FOREMAN_REPORT.md` and `.json` report "Totals: 12 passed, 0 failed (12 items)" and
explicitly flag the difference as predating the diff. I did not re-run it, per
instruction. One of the two records is stale and should be reconciled before the lane
is signed off, since nothing in this diff touches `openspec/`.

---

## What is good, plainly

- `fault-matrix.mjs` is exactly what the requirement asked for: closed, total,
  precedence-ordered, import-free, deterministic, and tested row by row including the
  two orderings the spec states in prose. If F2's wiring were corrected, this module
  would need no change.
- `prover-lifetime.mjs`'s control-record codec is byte-exact, rejects CR, non-ASCII,
  oversize, wrong framing, non-canonical decimals and uppercase hex distinctly, and its
  field-closure guard defeats getter and non-enumerable-property poisoning — with tests
  that prove the getter never ran.
- BigInt nanoseconds throughout, so there is no float drift in deadline arithmetic, and
  `outerStartMonotonicNs` is sampled once and threaded rather than re-read.
- Raw exit retention is deliberate and correctly placed — the first statement of the
  close handler, before the `settled` guard and before any read.
- Diagnostics are bounded at exactly 1 MiB with a tested boundary, and are structurally
  incapable of reaching the disposition.
- No framework invention, no overreach, no live anything, one added script, zero
  dependencies.
- The implementation record discloses its own gaps, including an inconvenient
  verification count. That is worth as much as the code.

## What would unblock

1. Stop treating a caller-initiated kill as a retained main exit (F1). Retain the raw
   close for evidence, but pass `rawExit:{kind:'unknown',code:null}` — or a
   provenance-guarded equivalent — into the matrix when the caller itself requested the
   kill, so the deadline row is reached. Then change
   `executor-caller.test.mjs:81` to expect `PROCESS_UNKNOWN`/3/`DEADLINE_EXCEEDED`.
2. Stop inventing the stop and timer facts (F2). Use `null` scalars and `false`
   receipts for what was not observed, and accept that this caller's best honest
   outcome is `PROCESS_UNKNOWN` until the real observations exist.
3. Either amend the lane's scope to the argv-only reinterpretation of the handoff
   requirement, or leave that requirement open (F4). Do not record it as met with the
   argv test standing in for the seven denial cases.
