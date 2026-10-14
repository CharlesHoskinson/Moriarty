# AUDIT-OPUS-HANDOFF

Independent read-only audit of the one-shot invocation handoff, the last open
requirement of Moriarty roadmap task 5.2.

- Commit under audit: `953895a7` ("Add the one-shot invocation handoff with explicit replay denial"), shared base fix merged at `068d95e8`
- Specification: `openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md` lines 29-41 (requirement "Concrete one-shot runner invocation handoff", scenario "Actual runner-to-script protocol")
- Implementer: GPT-5.6 Sol via Codex. Auditor: Claude Opus 5, cross-vendor, cold from the code; `FOREMAN_REPORT.md` still holds template placeholders and was not used as evidence.
- Auditor wrote only this file and `AUDIT-OPUS-HANDOFF.json`. Every experiment ran on copies under the session scratchpad; no product file was created, edited or deleted.

## Verdict: WARNING

The protocol itself is good work. The socket, directory modes, nonce, constant-time
comparison, bounded `/proc` ancestry with retained start ticks, the closed child
environment, the one-shot rule and the EOF distinction are all real, all enforced
rather than asserted, and all load-bearing under mutation: every one of the 28
handoff tests and all 3 new executor-caller tests can be made to fail by a single
targeted edit. None of the four blocking conditions is triggered.

Two high-severity findings keep this short of approval. The first is a measured
unbounded hang in the new handoff wiring of `executor-caller.mjs`. The second is
that the commit's claim of byte-identical carry-over is false for exactly the file
that carries the first, which removed it from the audit's stated scope.

### Blocking conditions, each checked

| Condition | Result |
| --- | --- |
| A denial case claimed but not enforced | Not triggered. Everything the commit claims is enforced and mutation-detected. Boot mismatch has no denial path (F3), but the specification assigns that check to the child and no child exists in the repository; that is an incomplete scenario, not a false claim. |
| A durable record written after the answer rather than before | Not triggered. `executor-caller.mjs:104` awaits `appendInvocationEvent` before `runHandoffServer` is even constructed, so no answer path exists until the append resolves, and on append failure the connection handler is never attached (asserted at `executor-caller.test.mjs:209`, `connectionListeners === 0`). Denials await `recordDenialEvent` before any byte at `invocation-handoff.mjs:73`. |
| A test that cannot fail | Not triggered. 41 mutations applied, 31 detected; all 31 tests in scope fail under at least one. Two tests are duplicative or self-computing (F3, F6); neither is vacuous. |
| The environment still being inherited wholesale | Not triggered. Fixed at the spawn site. |

## 1. The replay fix: the orchestrator's judgement holds, with one correction

The denial is genuinely the one-shot rule, not an incidental path. `invocation-handoff.mjs:78`
(`if(consumed||active)`) is the only route into `refuseReplay` at `:76`, and
`HANDOFF_REPLAY_DENIED` has no other producer in the module. Disabling the guard
(mutation M1) and stripping the record-then-deny body (M18) each fail exactly the
replay test at `invocation-handoff.test.mjs:56` and nothing else.

The history is worth stating precisely, because it confirms the lane fixed the
implementation rather than the test. The round-one test (`ae9a9126`) already
demanded `{denied:'HANDOFF_REPLAY_DENIED'}` and already failed the peer close with
an explicit `assert.fail`. Round one's implementation did send a denial, but it
called `denySocket` synchronously on `connection`, before reading anything, so the
client's in-flight write hit EPIPE and the line never arrived. Round two makes
`refuseReplay` wait for the replay peer's first `data`, `end` or `close` (or the
setup timeout) and only then records the denial durably and answers. The
expectation was raised, not relaxed: the test additionally now asserts the durable
denial record and that confirm and response each ran exactly once.

The durable record genuinely precedes the answer **in the code**: `:73` awaits
`recordDenialEvent` and only then calls `denySocket`. The one correction to the
orchestrator's reading is that the *test* cannot see this. Reversing the
implementation so the bytes go first and the record is awaited afterwards (M20), or
making the record fire-and-forget (M2), leaves all 28 tests passing, because the
injected recorder's `setImmediate` always resolves before socket data reaches the
client. The ordering is real; the assertion that claims to pin it is not
load-bearing (F5).

## 2. The seven denial cases

| Case | Implemented | Genuinely tested | Where |
| --- | --- | --- | --- |
| Forged nonce | Yes | Yes | `invocation-handoff.mjs:84` with `:33`; tests `:49,:53`, `:33`, `:45`. M3, M8 caught. |
| Wrong authority / reservation | Authority yes; reservation delegated | Authority yes; reservation pass-through only | Authority `:84` (M4 caught). Reservation is the injected `confirmCurrentState` at `:85`; the test supplies the error whose message becomes the denial code, so M33 (stop propagating the message) is what fails tests 19-21. |
| Stale store event | Delegated | Pass-through only | Same gate, same shape. What is enforced is ordering: state confirmation strictly before any payload byte (M7 fails 5 tests). |
| Different launcher process | Yes | Yes | `verifyAncestry :40` over real `/proc` parsing `:39`; tests `:38`, `:54`. M6, M12, M13, M14, M15, M30 caught. Anchor weakness in F4. |
| Replay | Yes | Yes | `:78` → `:76` → `:73`; test `:56`. M1, M18 caught. |
| Boot mismatch | Comparator only | No | `verifyBootId :42`; the parent samples the real boot ID at `executor-caller.mjs:97` and binds it into the payload, but nothing converts a false comparison into a denial and no child implementation exists. Test 14 (`:43`) restates test 10. |
| Parent disappearance | Yes | Partly | Before setup: real and enforced (`:79`, `:43`; test `:57`, T4/M11 caught). After startup: `:44` wired at `executor-caller.mjs:104`, but test `:60` registers its own listener and calls the classifier itself, and the caller's `parentLost` cannot change any disposition (F6). |

The earlier grep's nine matches for denial wording are duplicates, not extra
coverage: four scenarios come from one parameterised loop at `:48-53`, and two
comparator tests (10 and 14) overlap.

## 3. Protocol details the specification fixes

| Requirement | Finding |
| --- | --- |
| Mode-0700 runtime directory | Real. `mkdir {mode:0o700}` plus an explicit `chmod` at `:35`, so a permissive umask cannot widen it; the test forces `umask(0)` and reads the mode back. M9 caught. |
| Mode-0600 socket | Real. `chmod` after listen at `:37`; the test stats the socket and exchanges real bytes. M10 caught. Brief bind-to-chmod window is contained by the 0700 parent directory (F13). |
| 256-bit nonce | Real `crypto.randomBytes(32)` with a length check on the injected generator; M8 (16 bytes) fails six tests. |
| Constant-time nonce equality | Real `timingSafeEqual` at `:33`, with a documented and safe early return on length (the hex shape check at `:83` fixes the length at 32 bytes before comparison). M3 caught. Authority equality is a plain comparison (F13). |
| Two environment values, explicitly set, nothing inherited | Real and closed. `buildInvocationEnv :34` returns a frozen two-key object; `executor-caller.mjs:100` spawns with `env:childEnv` and never spreads `process.env`; `:65` refuses `env` together with `handoff`. See section 4. |
| Peer UID | Real comparison over injected credentials, declared at the top of the module because Node exposes no `SO_PEERCRED`. Both hooks are mandatory options at every layer and `peerCredentialsUnavailable()` throws, so nothing passes silently (F12). Expected UID is the real `process.getuid()`. |
| Bounded ancestry with retained start ticks | Real. `/proc/<pid>/stat` parsed past the `comm` parenthesis with correct field offsets (ppid `tokens[1]`, start ticks `tokens[19]`), depth bounded at 32, PID reuse caught by comparing the anchor's start ticks, self-parent and vanished PID distinguished. Anchored on a caller assertion rather than the observed spawn (F4). |
| Durable invocation event before the answer | Real, by sequencing at `executor-caller.mjs:104`, with a test that a failed append attaches no connection handler at all. |
| Exactly one handshake | Enforced across connections; silently ignored rather than denied for a second line on the same connection (F8). |
| EOF before setup vs after startup | Real and distinct. Pre-line listeners are removed once a handshake line arrives (`:81`), so the two cases cannot be confused. |

Nothing is approximated or injected where the specification demands enforcement,
with the two honest exceptions the module itself declares (peer credentials) and
the one it does not (the reservation and store-event predicates live entirely in an
injected hook, F9).

## 4. Is the environment genuinely closed? Yes

The earlier audit's finding (`AUDIT-OPUS-01`: "no `env` key, so the detached child
inherits the entire parent environment") is fixed at the spawn site.
`executor-caller.mjs:100` passes `env:childEnv`, where `childEnv` is either the
frozen two-key `buildInvocationEnv` result or the caller's own closed map, never
`process.env`; the legacy path still defaults to `{}`. `:65` rejects a caller that
supplies both `env` and `handoff` before the monotonic clock is sampled or anything
is spawned. The test asserts `Reflect.ownKeys(actualOptions.env)` is exactly the
two variables. Leaking `process.env` at either site (M19, M22) is caught. And no
environment value alone grants authority: the same handshake also requires
authority equality, peer UID and ancestry before any payload byte.

## 5. Are the 28 tests load-bearing? Yes, 31 of 31 in scope

41 single-edit mutations, applied to scratchpad copies only. 31 detected, 10
survived. Every handoff test and all three new executor-caller tests fail under at
least one mutation, so no test in scope would pass against a stub, and none asserts
only that something does not throw.

The ten survivors mark the real coverage gaps:

- M2 / M20 — denial record fire-and-forget, or bytes before the record (F5)
- M17 — response payload digest no longer bound to the recorded invocation digest
- M23 — caller ignores `HANDSHAKE_DENIED`, so `STARTUP_INVALID` is never reached (F6)
- M24 — caller drops the parent-loss classification (F6)
- M26 — outer deadline never capped by the block deadline (F7)
- M28 — socketless denials (EOF before setup, setup timeout) not recorded
- M29 — parent `/proc` stat retry reduced to a single attempt
- M31 — runtime directory suffix not random
- M32 — a second handshake line on the same connection answered twice (F8)

Two tests are weak rather than absent. Test 14 (`:43`) only fails when
`verifyBootId` itself is broken, which test 10 already covers. Test 27 (`:60`)
computes the EOF classification in its own listener and compares it to a constant
already asserted in test 12; what it genuinely pins is that the real accepted
socket is returned and emits EOF.

## 6. Integration with the executor caller

The wiring is correct in order and content: runtime directory and socket, then
nonce and closed environment, then the boot ID sampled immediately before spawn,
then spawn, then the live `/proc` read of the spawned process, the payload built
from validated state, the durable append, and only then the handshake server, whose
response is the same frozen payload and digest that were recorded. The placeholder
closed-environment option is gone, replaced by a validated 21-field handoff
contract with typed digests, identifiers and bigint start ticks.

Byte-identity of the carried files, against the approved `cff5f1e3`:

| File | Approved | Audited | Identical |
| --- | --- | --- | --- |
| `fault-matrix.mjs` | `abc1efa041a9fb6c` | same | yes |
| `fault-matrix.test.mjs` | `19f0c29eb2ebd5d4` | same | yes |
| `prover-lifetime.mjs` | `a046a77a17cd38ab` | same | yes |
| `prover-lifetime.test.mjs` | `693f88c809668f0d` | same | yes |
| `executor-caller.mjs` | `2d61473270494654` | `ac87dccb3495154e` | **no** |
| `executor-caller.test.mjs` | `4d53e804ef477b3b` | `c85f0ff9009b578d` | **no** |

The executor-caller changes are additive and an improvement, and its test file adds
three tests without touching any existing assertion. But the commit says the file
is byte-identical, the brief repeated it, and the only high-severity defect below
lives in those new lines (F2).

### F1, in full, because it is the finding that matters

The socket is listening before spawn, which the protocol requires: the path goes
into the child's environment. The `connection` handler, however, is attached only
inside `runHandoffServer`, after `parentStat` and after the durable append have
resolved. Node drops a `connection` event that has no listener, so a child that
connects inside that window is lost: it is never read, never answered and never
closed.

Measured on the audited code, with the child connecting immediately and a 60 ms
durable append: `servedHandshake=false`, `clientGotBytes=false`, and
`runExitRetainingExecutor` had not returned after 8 seconds. With the connect
delayed 40 ms and the append instant: `HANDSHAKE_OK` and a normal return. A second
probe isolates the two mechanisms: a connection emitted before the listener is
attached is lost, and `net.Server.close()` never fires its callback while an
accepted-but-unlistened socket remains open — which is why `removeHandoffRuntime`
(`:73`, awaited at `:125`) never completes and the caller never returns.

The window is exactly the durable fsync, the slowest step in the sequence, and
connecting is the first thing the child does after reading its two variables. The
consequences are a spurious startup refusal for a correct child and, worse, an
unbounded await inside cleanup, against a specification that bounds cleanup,
durable result and acknowledgment by outer+1740 and releases the wallet lock only
after validated durable containment. The fix is small: buffer connections from the
moment of `listen` (attach a queueing handler at creation and hand the queue to
`runHandoffServer`), and bound the close in `removeHandoffRuntime`.

## 7. Overreach

None. The commit touches nine files: the four module/test pairs and two script
lines in `package.json`. No accounting, budget, wallet, Docker or network code; the
only socket is the local AF_UNIX channel. No build artefact is committed —
`.harness/`, `FOREMAN_REPORT.*` and `.verification-tmp.TLyf2E/` exist only as
untracked lane residue. No existing test was weakened anywhere; the one changed
expectation was raised, as section 1 sets out.

## What would move this to APPROVED

1. F1: buffer connections from `listen` and bound the close in cleanup, with a test that a child connecting during the append still completes.
2. F2: correct the provenance statement so the executor-caller change is reviewed as new code, which it is.
3. F3: either add the child side (boot ID, canonical digest against the store event, deadline, socket ownership) or state the deferral explicitly in the specification.
4. F4: anchor ancestry on the process this runner actually spawned, and assert the relationship in a test.
5. F5 and F6: make the denial-ordering assertion observable across an I/O turn, and assert `STARTUP_INVALID` at the caller.
