# FOREMAN_REPORT

- run_id: moriarty-ll-loop-20260912
- role: implement
- slug: prover-wrapper
- branch: foreman/moriarty-ll-loop-20260912/implement/prover-wrapper
- worktree: /home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-prover-wrapper
- base_sha: 056e662254c38936758af9c6f72c019705a1ff09
- status: complete

## Summary

Implemented the static PID 1 prover-lifetime wrapper required by
`openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md`
under "Prover lifetime established before daemon startup" (spec lines 103 and
123 pin the artefact path and its offline-test requirement). Delivered in
exactly three files, all under `plugins/moriarty-dev/`:

- `scripts/moriarty_dev/prover_lifetime.c` (604 lines) — the wrapper.
- `scripts/moriarty_dev/prover_lifetime.build.md` — reproducible build,
  verification commands, exit-code table, CLI contract.
- `tests/test_prover_lifetime_wrapper.py` (638 lines, 24 new tests) —
  process-driven offline tests.

Codex (gpt-5.6-sol via the codex adapter) wrote the implementation from a
five-part spec I authored covering the governing requirement text (verbatim),
the authoritative JS arithmetic/codec model (verbatim, embedded because it
does not exist in this worktree's branch history — see Findings), a fixed
CLI/exit-code contract the spec itself leaves open, and the required test
scenarios. I independently re-verified everything below rather than trusting
Codex's own report.

## What the wrapper implements

- CLI: `prover_lifetime <control-file-path> -- <proof-server-path> [args...]`.
  Wrong argc/missing `--` exits 2 before any file I/O.
- Control file: opened `O_NOFOLLOW`, must be a regular file, read bounded to
  512 bytes (513th available byte = oversize refusal). Parser enforces the
  exact six-line ASCII format from `prover-lifetime.mjs`: schema string,
  lowercase UUID, canonical-decimal time-namespace inode, canonical-decimal
  latest-start ns, canonical-decimal kill-deadline ns, lowercase 64-hex
  digest, single trailing newline, no CR, no byte > 0x7f, no extra lines.
  Decimal fields are parsed into `uint64_t` with explicit overflow rejection.
- Live clock identity: reads `/proc/sys/kernel/random/boot_id` and
  `stat("/proc/self/ns/time").st_ino`, refuses on mismatch against the
  control record (this is `validateClockIdentity` done with real `/proc`
  reads instead of passed-in values).
- Timing: `entry_ns` captured via `clock_gettime(CLOCK_MONOTONIC)` as early as
  practical; refuses if `entry_ns >= latestStartMonotonicNs` (matches
  `validateWrapperEntry`'s strict-less-than acceptance exactly) or if entry
  is already at/past the control's kill deadline. Own deadline =
  `min(entry+1500s, control.killDeadline)`, ties won by control — exactly
  `computeWrapperKillDeadline`.
- Fork/exec: child sets its own dedicated process group before `execv`
  (no shell, no `$PATH` search); parent double-confirms the group. Wrapper
  installs real signal handlers (not default/`SIG_IGN`) for
  SIGTERM/SIGINT/SIGQUIT/SIGHUP/SIGCHLD, sets itself as a child subreaper
  (`PR_SET_CHILD_SUBREAPER`) so reparented grandchildren are still reachable
  for reaping even outside a real PID namespace.
- Supervision loop: uses `ppoll` re-deriving the remaining interval from
  `CLOCK_MONOTONIC` against the fixed absolute deadline every iteration (no
  relative sleep computed once, no wall clock) — satisfies "monotonic
  absolute timer, not... accumulated sleeps." Forwards a received terminating
  signal unchanged to the child's process group, escalates to `SIGKILL` after
  a documented 2-second grace (or the remaining deadline if shorter); hitting
  the absolute deadline kills immediately.
- Final reap sweep bounded to 500ms (or less if the deadline is closer):
  `kill(-pgid, SIGKILL)` then `waitpid(-1, WNOHANG)` loop reaping every
  reapable descendant, not just the tracked child.
- Exit codes (fixed and documented in build.md, since the spec text pins the
  record format/timing but not exact wrapper exit integers): child's own
  `WEXITSTATUS` when it exits normally with no signal forwarded; `126` on
  `execv` failure; `128+N` when the tracked child dies from signal N *or*
  when the wrapper forwarded a termination signal (even if the child then
  exits 0 on its own — Codex's deliberate reading of the spec's "timeout/
  signal... is non-success," tested explicitly, see below); `124` whenever
  the wrapper's own absolute deadline is what triggered termination,
  unconditionally; `64` for any pre-fork refusal; `2` for CLI usage errors.
- Hidden `--selftest-encode`/`--selftest-decode` modes: pure codec-only, no
  clock/`/proc`/fork/exec, dispatched on a distinct `argv[1]` literal the
  normal parser never matches — used only by the byte-agreement test.

## Behaviours the tests actually drive vs. inject

All 24 new tests spawn and observe real OS processes; none stub or mock the
wrapper or re-check the JS model in isolation. Concretely, real subprocesses
cover:

- Normal exit (real child, real status propagation) and `execv` failure.
- Deadline expiry with a hung real child, asserted both for wrapper exit
  code and for the real child PID actually disappearing from `/proc`.
- An explicit "dead external launcher" scenario: a forked, immediately-
  exiting driver process starts the wrapper and disappears, proving the
  wrapper's own bound fires with no live external supervisor at all
  (directly tests the "Post-start receipt loss cannot unbound the prover"
  scenario).
- A real forked grandchild, reparented via the subreaper flag, reaped by the
  final sweep — proves "reap children until none remain."
- A real child that installs `SIG_IGN` for SIGTERM/SIGINT: proves the
  wrapper's grace-then-`SIGKILL` escalation is what actually terminates it
  (SIGKILL cannot be ignored), with a real elapsed-time bound.
- A real child that installs a genuine SIGTERM handler and exits 0 on its
  own: proves the forwarded-signal-is-non-success exit-code decision is
  real, not asserted only in prose.
- Two tests using a real unprivileged combined user+PID namespace
  (`unshare --user --map-root-user --pid --fork --mount-proc`, confirmed
  working unprivileged in this sandbox; plain `--pid` alone is EPERM here):
  one proves that a descendant which forks and **changes its own process
  group** (so a plain `kill(-pgid,...)` cannot reach it) is still destroyed
  by PID-namespace teardown when the wrapper (real PID 1) exits — verified
  by a live heartbeat file that stops advancing, not by mere disconnection;
  the other abruptly `SIGKILL`s the wrapper itself from outside and proves
  the same detached descendant still disappears, i.e. abrupt PID 1 death
  (not just clean wrapper-initiated exit) also tears down the namespace.
- Malformed/oversize/nonregular/symlink/FIFO control files, boot-ID and
  time-namespace mismatches, late-entry and expired-deadline refusals — each
  asserting both the exit code and that the target child program's marker
  file was never created, i.e. genuinely proving no fork/exec happened, not
  just checking the return code.
- CLI usage errors.

Nothing had to fall back to a narrower unprivileged boundary: the combined
user+PID-namespace probe succeeded in this sandbox, so both PID-namespace
tests ran with full assertions and zero skips. (The test file still contains
an explicit, non-silent `pytest.skip` fallback path for a host where that
probe fails, per the spec's "retain an explicit unavailable result if the
host cannot supply it" — it just wasn't exercised here because the capability
was actually available.)

## Byte-agreement result against the JS codec

`prover-lifetime.mjs` does not exist in this worktree's branch history (see
Findings). The test file embeds the exact verbatim source I read from the
sibling worktree/branch as a Python string constant, writes it to a temp
`.mjs` file at test time, and drives it through real `node` (v24.18.1,
confirmed present) to encode a fixed sample control record. It separately
drives the compiled C wrapper's `--selftest-encode` hook on the identical
field values, and asserts the two resulting byte sequences are equal
(`encoded_by_c == encoded_by_js`) — a genuine shared-input comparison, not
two isolated assertions. It also decodes the JS-encoded bytes through the
C `--selftest-decode` hook and confirms every field matches. Both assertions
pass:

```
test_c_codec_bytes_and_decode_agree_with_verbatim_js PASSED
```

## Build verification (I re-ran this myself, independent of Codex's own report)

Compiler invocation:

```
gcc -std=c11 -Wall -Wextra -Werror -O2 -static -o prover_lifetime prover_lifetime.c
```

Compiles with zero warnings under `-Wall -Wextra -Werror`. Static/import-free
evidence:

```
$ file prover_lifetime
prover_lifetime: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=cc1a9c3ed0b677251673a49c3da0494694ce7508, for GNU/Linux 3.2.0, not stripped

$ ldd prover_lifetime
	not a dynamic executable

$ readelf -d prover_lifetime
There is no dynamic section in this file.
```

## Verbatim pytest output (independently re-run from `plugins/moriarty-dev`)

```
............................................................ [ 26%]
................................................... [ 49%]
........................................................................ [ 82%]
........................................                                 [100%]
223 passed, 105 subtests passed in 18.25s
```

Baseline before this change (confirmed immediately before dispatching Codex):
`199 passed, 105 subtests passed`. 223 − 199 = 24 new tests, all passing,
zero failures, zero skips, zero regressions.

## Findings

- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.mjs` — the
  file the task description calls "authoritative" and asks me to read from
  this worktree — **is not present in this worktree's branch history**. It
  was added on the sibling, not-yet-merged branch
  `foreman/moriarty-ll-loop-20260912/implement/scope-reconciliation` (commit
  `cff5f1e3`), and physically exists only at
  `/home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-scope-reconciliation/experiments/moriarty-midnight-financial/ledger/prover-lifetime.mjs`.
  I read it there (read-only) and did not modify it or copy it into this
  worktree's tracked tree. Its exact verbatim source is embedded as a test
  fixture string inside `test_prover_lifetime_wrapper.py` so the byte-
  agreement test has no runtime dependency on the sibling worktree existing.
  **This is an architect-visible gap**: the prover-wrapper task (5.2) and the
  scope-reconciliation task that introduced the model it depends on are on
  diverging, unmerged branches. Reconciling/merging those two branches is
  outside this task's scope and outside what I'm permitted to do (no git
  writes), but the architect should be aware before treating this round as
  final — once the branches merge, it would be worth confirming the merged
  `.mjs` file's bytes are identical to what's embedded in the test (they are,
  as of `cff5f1e3`, verified above).
- No file outside the three named files was created or modified. `git log`
  HEAD is unchanged (`056e6622...` before and after); nothing was staged or
  committed. The only incidental untracked artifact is `.foreman-last.txt`,
  written by the Codex adapter's `--output-last-message` flag (not part of
  the deliverable).
- No `.py` file under `plugins/moriarty-dev/scripts/moriarty_dev/` was
  touched, so the hook-registration digest gate is unaffected.
- Nothing the specification requires was left undone for privilege reasons —
  the combined unprivileged user+PID-namespace technique
  (`unshare --user --map-root-user --pid --fork --mount-proc`) was available
  and used for the two namespace-dependent tests, so both ran with full
  assertions rather than a narrowed/skipped boundary.

## Evidence

- `git status --porcelain` before: `?? FOREMAN_REPORT.json` / `?? FOREMAN_REPORT.md` only (plus this report's own placeholders).
- `git status --porcelain` after: adds exactly `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c`, `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.build.md`, `plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py`, and the adapter's `.foreman-last.txt`.
- `head_before`: `056e662254c38936758af9c6f72c019705a1ff09`
- `head_after`: `056e662254c38936758af9c6f72c019705a1ff09` (unchanged)
- `status_digest_before`: `ca6f679ef50a955b50ee8ae851416319147513fbc34c9ac59b74bc77219513e5`
- `status_digest_after`: `9e81df604a7e3c6f6b54c515a69a21a4504a66c733f86d788e0787b43abe2dbe`
- `unauthorized_git_activity`: false

## Open questions

None blocking. The one open item — the two branches' divergence over
`prover-lifetime.mjs` — is recorded above for the architect to reconcile at
merge time; it did not block completing this task since the file's exact
bytes were available to read and embed.
