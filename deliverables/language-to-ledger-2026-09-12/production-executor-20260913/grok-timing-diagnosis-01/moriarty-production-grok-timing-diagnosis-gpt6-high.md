# Independent executor timing diagnosis — GPT6 high

Confirmed phase-barrier race, absolute-cutoff timing sensitivity, and rejected-unknown diagnostic mechanism. Reported null-result failures cannot be assigned a unique root cause from retained logs. Preserve unresolved ownership and strict authentication; fix runtime deadline mechanism as production behavior, not by weakening test expectations or cutoffs.

Read-only independent diagnosis; no implementation, source approval or live admission. Required AGENTS/develop/status startup completed from release-integration-20260913. No source edits or real resource actions.

Captured files and exact SHA256/read timestamps: `/tmp/moriarty-production-grok-timing-snapshot-gpt6-high/manifest.json`. Source is changing, not a coherent final tree. `loan_executor.py` changed again during this review; this report applies to the captured bytes. Historical logs have no proven exact-source binding.

## Observed runs

- `focused-after-p11.txt`: 1 failed, 164 passed, 7 warnings, 126 subtests passed in 156.23s (0:02:36)
- `focused-r2-after-repair.txt`: 21 passed, 38 subtests passed in 28.97s
- `focused-r2-no-fifo-2.txt`: 19 passed, 1 deselected, 32 subtests passed in 26.22s
- `focused-r2-no-fifo.txt`: 1 failed, 18 passed, 1 deselected, 32 subtests passed in 21.19s
- `loan-executor-final.txt`: 2 failed, 113 passed, 7 warnings, 126 subtests passed in 127.35s (0:02:07)
- `loan-executor-suite.txt`: 1 failed, 114 passed, 7 warnings, 126 subtests passed in 166.02s (0:02:46)
- `p11-and-r2.txt`: 22 passed, 38 subtests passed in 37.15s

## TD-01: Unit-file existence is not completed startup

**confirmed fixture synchronization defect; explains possible early-loss containment False**

The fixture polls FakeServices.unit(), kills the parent as soon as the unit JSON exists, and expects PARENT_LOST with containmentComplete=true. Fake systemd-run writes that JSON before returning. The child then checks parent liveness before observing startup and before setting unit_started, unit_invocation_id or unit_receipt. A loss at that check leaves no authenticated startup identity. contain_owned correctly refuses to kill an active unit when expected_invocation is absent, with no-ownership-receipt and containment false. This is a valid execution of the captured code, not evidence that cleanup may guess ownership.

For the after-startup case, synchronize on completed validated startup and durable unit-ownership, preferably an explicit launched barrier emitted only after both writes and child state assignment. Mere file existence is not a durability acknowledgment: production exclusive writes create their path before fsync/close. Keep a separate deterministic loss-before-identity case that requires unresolved ownership, retained timer/parent ownership and no kill of an unowned unit. Keep the active reservation/runtime-held ownership after parent loss.

Expected isolated checks:

- Inject parent loss at unit launch return before identity: unknown, incomplete containment, no unauthenticated unit kill.
- Inject parent loss only after authenticated durable unit-ownership/launched acknowledgment: PARENT_LOST with owned cleanup independently observed and retained.
- Use bounded barrier waits and explicit failure diagnostics; do not increase arbitrary sleeps to establish phase ordering.

Captured source references: `tests/loan_executor/test_runner_handoff.py:268`, `tests/loan_executor/fake_services.py:225`, `scripts/moriarty_dev/loan_executor.py:914`, `scripts/moriarty_dev/loan_executor.py:941`, `scripts/moriarty_dev/loan_cleanup.py:309`.

## TD-02: Relative runtime duration has less than one second of variable activation slack

**confirmed production timing mechanism; historical failing-run attribution remains inferred**

RuntimeMaxSec=floor((bootstrapCut-now)/1s) is sampled before the subprocess command. The manager activation timestamp is later. The validation requires activated_us + RuntimeMaxSec*1e6 <= bootstrapCut_us. Thus permissible activation latency is only the fractional remainder, in [0,1s), and may be arbitrarily small. Fake executables use actual monotonic time and process startup; nominal valid tests can enter STARTUP_ACTIVATION_LATE based on timing. This is production behavior shared by the fake transport, not only a fixture defect.

Repair must enforce bootstrap termination by the fixed parent-anchor +1606 under bounded activation delay and parent loss, preserving +1620 independent containment, +1740 completion/ack and the outer margin from R2-08. Do not widen the absolute cutoff or classify a late effective runtime as normal success. A merely detected late activation after starting a unit is not proof that the unit cannot run past +1606, especially if the parent is lost or startup identity remains unavailable. Preserve the actual timer target/NextElapse checks and independent containment when foreground work cannot finish.

isolated-mechanism-probe.json: synthetic parent-relative now=100.950s, cut=1606s, RuntimeMaxSec=1505, activation=101.050s; only 50ms slack but 100ms activation delay yields end=1606.050s. This is formula reproduction, not historical captured numeric timing. Logs contain no actual activation timestamps.

The captured timer similarly computes a relative whole-second delay before activation, then checks actual next-elapse within 2s of +1620. That validation must not be removed or widened to make tests pass. No new timer probe or normative tolerance approval is made here.

Expected isolated checks:

- Control fractional remainder immediately below/above the activation delay; preserve absolute +1606 in both.
- Bound delayed activation and parent loss before startup identity, then independently establish that the scheduled process cannot overrun the absolute bootstrap cutoff.
- Retain negative wrong timer target/late next-elapse and exhausted cleanup/write deadline tests.

Captured source references: `scripts/moriarty_dev/loan_executor.py:898`, `scripts/moriarty_dev/loan_executor.py:909`, `scripts/moriarty_dev/loan_executor.py:930`, `scripts/moriarty_dev/loan_executor.py:1160`, `tests/loan_executor/fake_services.py:233`, `/tmp/moriarty-production-r2-audit-gpt6-high.json#R2-08`.

## TD-03: Legitimate startup failure diagnostics can be rejected as an unbound startup identity

**confirmed producer/consumer mismatch by isolated frozen-function probe; exact historical null-result cause uncertain**

On activation validation failure, the child persists unit-startup with observed nonzero InvocationID, but raises before setting result unit_invocation_id. Runner validates every unit-startup artifact against document.invocationId regardless of status; observed ID != null rejects the entire durable PROCESS_UNKNOWN result. Runner then exposes status/failureCode/resultSha256=null. The isolated AST-extracted actual validator reproduces this rejection. Matching IDs passes that one check only, not full result validation.

Establish consistent authenticated startup InvocationID semantics for legitimate unknown results, including timing-invalid startup. Retain exact current invocation SHA, charge, reservation, unit, valid startup identity and referenced evidence digest checks. Do not broadly exempt unknown results from identity checks, accept malformed startup identities, trust an unbound child report, or release unresolved ownership. Separate a valid observed unit identity from proof of compliant timing without inventing identity after the fact.

The logs record only compact CLI output, not the runner problem, result bytes, startup artifact or child stderr. The same null fields can result from missing/malformed evidence, mismatched identity or other result rejection. Therefore this mechanism is established but attribution of each P09/P11/live-profile failure is not proven.

Expected isolated checks:

- Produce authenticated STARTUP_INVALID timing evidence with a valid observed InvocationID: retain a validated unknown diagnostic, no success/release.
- Reject different/nonhex/all-zero/missing startup InvocationID, forged reservation/invocation SHA and stale evidence digests.
- Exercise full producer -> runner result validation -> CLI history consumer; checking only in-process child result misses this mismatch.

Captured source references: `scripts/moriarty_dev/loan_executor.py:927`, `scripts/moriarty_dev/loan_executor.py:937`, `scripts/moriarty_dev/loan_executor.py:945`, `scripts/moriarty_dev/runner.py:405`, `scripts/moriarty_dev/runner.py:561`, `scripts/moriarty_dev/runner.py:848`.

## TD-04: Progress channel drops fragmented JSON records

**confirmed independent runner framing defect; not sufficient to explain status null**

The runner splits each recv chunk on newline and parses immediately, discarding invalid fragments with no carry buffer. AF_UNIX SOCK_STREAM does not preserve sendall message boundaries. Fragmented contained progress can therefore lose containmentAcknowledged. That can leave disposition unresolved even when a valid result is loaded. It cannot by itself explain the reported child exit 3 plus status null: the loaded result is validated separately.

Use bounded line buffering and a closed progress-message check tied to the authenticated connection; retain incomplete chunks until a newline and fail closed on oversized/malformed protocol input. Do not infer acknowledgment from result stdout.

Expected isolated checks:

- Split a valid contained JSON line across multiple recv chunks, coalesce several lines, and split a multibyte payload; acknowledge exactly one complete validated record.
- Confirm missing/invalid contained acknowledgment prevents success while a valid loaded diagnostic remains visible.

Captured source references: `scripts/moriarty_dev/runner.py:804`, `scripts/moriarty_dev/runner.py:813`, `scripts/moriarty_dev/runner.py:570`.

## TD-05: Fork-in-thread warning must be separated from fresh CLI failures

**observed fork warnings; independent supervision hazards; no demonstrated causality for the reported failures**

The sibling-intruder test deliberately starts a thread while runner persistence uses os.fork. All seven warnings are attributed to that test. P09 and P11 invoke a new CLI interpreter as a subprocess; the intruder is joined after runner returns. Logs show no stack, timeout detail or surviving thread tying these warnings to P09/P11. run_bounded also waits for child exit before reading its pipe, while allowing a 65536-byte child write; values larger than pipe capacity can block the writer until timeout. No measured result size/pipe capacity or reproduced deadlock is present in these runs.

Keep the sibling-peer rejection test, but arrange independent intruder synchronization without creating an unrelated thread across persistence forks, or make the production supervisor safe for its supported caller model. Treat pipe drain/order as a distinct bounded-worker contract concern; preserve bound/kill semantics. Do not declare warnings the root cause of all flaky tests.

Expected isolated checks:

- A bounded worker with a payload beyond actual pipe capacity returns without wait-before-read deadlock or falsely timing out.
- Peer-ancestry rejection still works without unrelated inherited locks; a truly stalled worker is killed at its bound.

Captured source references: `tests/loan_executor/test_runner_handoff.py:159`, `tests/loan_executor/test_runner_handoff.py:182`, `scripts/moriarty_dev/loan_exit_retention/exit_retention.py:93`, `scripts/moriarty_dev/loan_exit_retention/exit_retention.py:109`, `scripts/moriarty_dev/loan_exit_retention/exit_retention.py:118`.

## TD-06: Compact CLI failure output removes the decisive rejection reason

**confirmed evidence/diagnostic gap**

CLI stores loan.problem in the isolated runner event but omits it from displayed JSON; failing assertions print only stdout/stderr. Fixture cleanup removes its temporary repository. Thus a test failure does not retain the result/runner event needed to distinguish startup rejection from result persistence or other identity problems.

In isolated failure diagnostics retain the current case source hashes, child exit/stdout/stderr, runner loanResult.problem/handshake/progress, raw result, referenced startup/ownership/containment evidence, synthetic timing and fixture-only event payload. Never inspect or dump canonical budgets/wallet or claim compact null fields prove no result file existed. Capture data before cleanup. A short focused valid/failure reproduction on a frozen complete tree is the next discriminating check after implementation stops.

Expected isolated checks:

- Failure artifact capture contains enough local evidence to identify the rejection without canonical DB access.
- A valid real CLI test and targeted parent-loss barriers pass from the same immutable source tree; record any source mutation instead of combining mixed-tree outcomes.

Captured source references: `scripts/moriarty_dev/cli.py:488`, `scripts/moriarty_dev/cli.py:500`, `tests/loan_executor/test_cli_consumer.py:60`.

Only new probe: pure arithmetic plus the AST-extracted captured validator, retained in `/tmp/moriarty-production-grok-timing-snapshot-gpt6-high/isolated-mechanism-probe.json`. No service transport, runner execution, DB read, wallet access or live mutation was performed. The synthetic timing example is not an actual failing-run timestamp.

## Changing-source supplement

Later captured _launch_unit removes require_parent(unit-launched) and require_parent(unit-startup-observed); retains runtime_max floor formula, absolute activation check and early-loss fixture unit-file-existence barrier. TD-01 immediate-loss control-flow example refers to initial captured bytes. Later deferral can acquire ownership before observing loss, but is not proof of deadline safety or a synchronized after-startup test. No workaround approval. Authentic identity, bounded post-loss work, fixed +1606 and unresolved ownership when identity unavailable remain required.

Later capture SHA256 `6cf377592dc61ff8c96233f36f250bd6228828a20b3168e5a0dfb48e899793f0`, read at `2026-09-13T22:02:21.846176+00:00`; source and full-gate log hashes are in `/tmp/moriarty-production-grok-timing-snapshot-gpt6-high/supplement-manifest.json`. Do not reuse the original executor hash for this later source.

`full-gate.txt`: 2 failed, 354 passed, 2 deselected, 235 subtests passed. Failures are normal complete handoff (child 3 versus expected 0) and parent-loss containment False. This adds evidence that the issue is not restricted to the CLI consumer; it still lacks the decisive retained per-case startup/result diagnostics.
