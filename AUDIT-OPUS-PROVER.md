# Audit: static prover-lifetime PID 1 wrapper

Verdict: **APPROVED**

Artefact: `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c`,
`prover_lifetime.build.md`, `plugins/moriarty-dev/tests/test_prover_lifetime_wrapper.py`.
Requirement: "Prover lifetime established before daemon startup" in
`openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md`,
lines 103 and 123, including the scenario "Post-start receipt loss cannot unbound
the prover". Base `056e6622`. Audited bytes:

| File | sha256 |
| --- | --- |
| `prover_lifetime.c` | `d0b08d72d5fa8d1d85a4b1d6f6c43e26b07f4615f0ca0e76126407157cf11691` |
| `prover_lifetime.build.md` | `e873d917c4d5a2de4b257bfdf05ef08b54072828dac63347ee2a3fbf99bf1565` |
| `test_prover_lifetime_wrapper.py` | `0cf96fb4c8407d0a2a5ad8a197548e5ccace4e4a819de0c81fe73990e452f90b` |

This is good work. The wrapper is a correct, small, genuinely static PID 1
supervisor, and its tests bind real process behaviour rather than describing it.
No defect was found, no test was found that cannot fail, and no overreach was
found. Nine findings follow; the highest is a claim in the implementer's own
report that its evidence does not support. None of them blocks source approval.

## 1. The tests are process-driven

Twenty-four tests, twenty-four passing, zero skipped. Every child is a real
compiled C program built by the same module fixture that builds the wrapper; the
fixture compiles both with `-std=c11 -Wall -Wextra -Werror -O2 -static`, so a
warning would fail the whole file. There are no mocks, no fakes, and no
patching anywhere in the file.

I replaced the wrapper source with `int main(void){return 0;}` in a scratch copy
of the plugin and ran the suite against it: **23 failed, 1 passed**. The single
survivor is `test_build_is_static_and_import_free`, which asserts a build
property rather than a behaviour and so cannot distinguish a do-nothing binary
from a working one. It is still the only test covering the requirement's
import-free static clause and does real work on a real ELF file. Load-bearing
count: **23 of 24**.

I then injected twelve targeted mutants into the C source, each compiling
cleanly, and ran the tests they should break. Ten were killed:

| Mutant | Killed by |
| --- | --- |
| deadline never fires | deadline, dead-launcher |
| own deadline ignores the control deadline | deadline, dead-launcher |
| no SIGKILL escalation after the grace | ignoring-child escalation |
| forwarded signal reported as success | handler-exits-zero |
| encoder emits a trailing newline | byte agreement |
| clock identity check bypassed | boot-id and time-namespace mismatch |
| final reap does not kill the process group | forked grandchild |
| late entry accepted | late entry |
| no dedicated process group at all | five tests |

Two survived. The child-side `setpgid` can be removed alone because the parent
repeats it at `prover_lifetime.c:600`; removing both is caught by five tests, so
this is deliberate redundancy in the implementation, not a hole in the suite.
The other survivor is the subreaper flag, which is finding 1 below.

The namespace tests create a real namespace. The probe fixture runs
`unshare --user --map-root-user --pid --fork --mount-proc` and returns a reason
string on failure; it succeeded here, both namespace tests ran with full
assertions, and `-rs` reported no skips. I built the decisive positive control
myself: the same `detach-heartbeat` child, which forks a descendant that calls
`setpgid(0,0)` so no `kill(-pgid, ...)` can reach it, run **without** `unshare`
survives the wrapper's exit and keeps appending to its heartbeat file (8 bytes
to 12 bytes across a 250 ms window); run **with** `unshare` exactly as the test
does, the heartbeat freezes (8 bytes to 8 bytes). The test therefore measures
PID-namespace teardown and would fail without a real namespace, and it confirms
the requirement's own point that process-group signalling is not the containment
guarantee. The companion test SIGKILLs the wrapper from outside, through
`/proc/<unshare pid>/task/<pid>/children`, and proves the same teardown follows
abrupt PID 1 death rather than only a clean exit.

The scenario "Post-start receipt loss cannot unbound the prover" is covered by a
dedicated test that forks a launcher which starts the wrapper and immediately
exits, so the wrapper's bound fires with no live external supervisor. Every
refusal test asserts both the exit code and the absence of the target child's
marker file, which is what proves no fork or exec occurred rather than merely
that a code was returned.

## 2. Byte agreement with the JavaScript codec is real

The test drives real `node` (v24.18.1) on the embedded model to encode a fixed
record, drives the compiled C `--selftest-encode` hook on the identical field
values, and compares the two byte strings. It is a shared-input comparison, not
two isolated assertions, and not a comparison against a hardcoded copy of the C
output: the expected bytes are produced by node at test time. It then feeds the
node-produced bytes into `--selftest-decode` and checks all five parsed fields.

The embedded fixture is the real model, verbatim. I extracted the `JS_CODEC`
raw string and compared it with the sibling worktree file and with the committed
git blob:

```
a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb  sibling worktree prover-lifetime.mjs
a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb  extracted JS_CODEC fixture
a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb  git show cff5f1e3:experiments/.../prover-lifetime.mjs
```

Identical, 4949 bytes, no rewrite. Embedding it was sound. The alternative would
have been a test that reads across a sibling worktree path, which would fail on
any other machine and would silently pass or skip once that worktree is removed.
The digest above is the thing to re-check when the two branches merge; the
implementer flagged the branch divergence itself, which is the right call.

Two one-directional divergences are worth recording, both with C stricter than
JavaScript and therefore safe: the JS decoder accepts decimal values above
2^64-1 through BigInt while the C rejects overflow (tested at line 487), and the
JS decoder tolerates a NUL byte while the C rejects any 0x00. Canonical-decimal
handling agrees exactly, including accepting a bare `0` and rejecting `01`, and
the 512-byte ceiling agrees. The saturating add in the C reaches the same answer
as the unbounded BigInt arithmetic on the `min` comparison.

## 3. Behaviour as process one

Checked against the requirement's bounds and found correct.

Deadline arithmetic matches the model exactly: entry is captured with
`clock_gettime(CLOCK_MONOTONIC)`, entry at or after latest-start refuses, and
`own_deadline = min(entry + 1500s, control.killDeadline)` with ties going to the
control, which is `computeWrapperKillDeadline` written in C.

The timer is absolute, not accumulated. Each loop iteration re-reads the clock
and derives the `ppoll` interval from the fixed absolute deadline, so an
interrupted wait cannot extend the bound. The suite does not test this under
interruption, so I did: against a control deadline of entry+1.0s, with a child
forking a short-lived grandchild every 20 ms to generate a roughly 50 Hz SIGCHLD
storm, the wrapper terminated at 0.98s and exited 124. No drift.

Signal handling is a correct block-then-`ppoll`-with-unblocked-mask pattern, so
there is no lost-wakeup window between reading `caught_signal` and waiting.
Received terminating signals are forwarded unchanged to the child's process
group; SIGCHLD is tracked separately and never forwarded. The suite tests only
SIGTERM, so I verified the rest: SIGINT, SIGQUIT and SIGHUP forward unchanged
and produce 130, 131 and 129.

Reaping sweeps the whole descendant set, not just the tracked child:
`kill(-pgid, SIGKILL)` followed by a `waitpid(-1, WNOHANG)` loop that returns as
soon as ECHILD is seen, so the normal fast-exit path pays no penalty. `execv` is
called on the supplied path directly, with no shell and no `PATH` search, so
only the pinned executable and argv can run. The control file is opened
`O_NOFOLLOW | O_NONBLOCK`, must be a regular file, and is read to 513 bytes so
the 513th byte is what proves oversize; the parser enforces the closed six-line
format exactly, rejecting CR, NUL, any byte above 0x7f, non-canonical decimals,
uppercase hex, a missing trailing newline and any extra line.

Values the requirement does not fix, all disclosed in the build document: the
2-second escalation grace, the 500-millisecond reap sweep, the exit-code
assignments, the CLI shape, and the subreaper flag. One of them narrows the
requirement's literal wording and deserves the architect's explicit nod: the
requirement says to reap "until none remain or the fixed deadline requires PID 1
exit", and the implementation adds a third exit condition by capping the sweep
at 500 ms. This is safe, and arguably better than the literal reading, because
namespace teardown SIGKILLs everything still in the namespace, which is stronger
than reaping, and because a bounded PID 1 exit is the point of the whole
artefact.

## 4. The self-test hooks

They cannot start a child and cannot weaken the lifetime bound. Dispatch happens
on `argv[1]` before any other parsing, so a control path spelled exactly
`--selftest-encode` or `--selftest-decode` does enter the hook — the build
document's "never reachable" is slightly too strong. The consequence is nil,
which I confirmed rather than assumed. In every real-form shape the hook exits 2
with no child and no marker file; at `argc == 7`, where the encoder would
otherwise accept, it requires `argv[2]` to be a lowercase UUID, and `--` never
is. Under `strace` the encode hook issues no `clone`, `fork`, `execve`, `open`,
`stat` or `clock_gettime` beyond its own `execve`: it is pure codec plus
stdin/stdout. This is a wording fix, not an attack surface.

## 5. The build is static, import-free and deterministic here

`gcc -std=c11 -Wall -Wextra -Werror -O2 -static` at exit 0, `file` reports
statically linked, `ldd` reports not a dynamic executable, and `readelf -d`
reports no dynamic section. Two consecutive builds on this host produced
identical bytes, sha256 `5b7fd3d1a5e1985bd6432fe80ca86e35622a91a480d3f51a0f398d680a2b426d`,
BuildID `cc1a9c3ed0b677251673a49c3da0494694ce7508` — the same BuildID the
implementer reported, under gcc 15.2.0 and glibc 2.43.

I checked the call graph out of `main` for anything that would need a runtime
import: it reaches only libc syscall wrappers and stdio, with no `getaddrinfo`,
`getpwnam`, `dlopen` or `system` call. The binary does contain weak `dlopen` and
`popen` symbols, which is a normal static-glibc artefact and not reachable from
this program. The build document pins the flags exactly and states that no other
toolchain has been validated, but records no compiler version and no expected
digest; that is finding 4.

## 6. Nothing required is missing or silently skipped

Four `pytest.skip` calls exist, all capability guards with explicit reasons, and
none fired. `-rs` reported no skips. The requirement explicitly asks for an
explicit unavailable result when the host cannot supply a namespace, so the
guards are what it asks for, not evasion; the namespace guard even drives the
real process-group path before recording the narrower boundary. The only
`try`/`finally` in the file is process cleanup in the abrupt-death test and
swallows nothing. No conditional assertion, no bare `except`, no `xfail`.

Every clause of the test sentence at spec line 123 is present: the suite
compiles the actual wrapper, and drives children that exit, hang, fork, change
process group and ignore signals, across parser and clock boundaries, parent
death, deadline, reap and signal behaviour. No live Docker start appears
anywhere.

## 7. No overreach

No accounting, budget, wallet, Docker, network or store logic in any of the three
files. The only occurrence of those words is inside the embedded JavaScript
model's own docstring. The compiled binary's reachable call graph has no socket
or resolver call. No build artefact is retained: the tests build into
`tmp_path_factory`, and no binary or object file exists under `scripts/` or
`tests/`. Exactly three product files were added, all under
`plugins/moriarty-dev/`, and no `.py` file under `scripts/moriarty_dev` was
touched, so the hook digest pin is undisturbed. `.foreman-last.txt` at the
worktree root is adapter output; it is untracked and disclosed in the
implementer's report, and should not be committed.

This audit modified no product file; the three digests above are unchanged from
the values at the start of the audit.

## Findings

| # | Severity | Dimension | Location |
| --- | --- | --- | --- |
| 1 | MEDIUM | report-accuracy | `FOREMAN_REPORT.md:97`, `prover_lifetime.c:587` |
| 2 | LOW | spec-coverage | `prover_lifetime.c:25` |
| 3 | LOW | documentation | `prover_lifetime.build.md:18` |
| 4 | LOW | reproducibility | `prover_lifetime.build.md:6` |
| 5 | LOW | self-test-hooks | `prover_lifetime.build.md:22` |
| 6 | LOW | test-fixture-realism | `test_prover_lifetime_wrapper.py:229` |
| 7 | LOW | test-coverage | `test_prover_lifetime_wrapper.py:349` |
| 8 | LOW | path-resolution | `prover_lifetime.c:181` |
| 9 | INFO | disclosed-open-choices | `prover_lifetime.c:26`, `:27` |

**1. The subreaper claim is not what the test proves.** The report says a real
forked grandchild is "reparented via the subreaper flag, reaped by the final
sweep". Setting `PR_SET_CHILD_SUBREAPER` to 0 leaves the grandchild test and
both namespace tests passing, because the grandchild stays in the tracked child's
process group and is destroyed by `kill(-pgid, SIGKILL)`. No test binds the flag.
The flag is correct and worth keeping — as real PID 1 it is redundant, and
outside a namespace it is what keeps reparented grandchildren reachable for
`waitpid` — but the architect should not carry this test forward as evidence for
reparented-descendant reaping, and a later refactor could drop the flag with the
suite still green.

**2. The 1500-second cap is never the binding term.** Every fixture sets the
control kill deadline within five seconds of entry, so `min(entry + 1500s,
control.killDeadline)` is only exercised on its control branch. The constant and
its match to the model's `PROVER_MAX_RUNTIME_SECONDS` rest on source inspection.
A self-test hook for the deadline computation, in the style of the codec hooks,
would close this against the JavaScript on synthetic inputs without needing a
1500-second test.

**3. The exit-code table understates collisions.** It says a normal child exit
retains status "0-123 or 125-127" and that the wrapper "always exits 124" on
deadline. Verified otherwise: a child that exits 124, 64, 125 or 126 on its own
is propagated unchanged, colliding with the wrapper's deadline, refusal, unknown
and execv-failure codes. The collision is inherent to the requirement that a
normal early child exit retain its actual status, and is not load-bearing because
the durable result document is the authority rather than the exit code, but the
table should say so instead of implying 124 is reserved.

**4. Reproducibility pins flags but not the toolchain.** No compiler version and
no expected binary digest are recorded, although the build is byte-deterministic
on this host. The same requirement has item 3 inspect and retain the exact
wrapper bytes before authorizing the container, so naming the toolchain and the
expected digest is what makes that retention checkable by someone other than the
original builder.

**5. "Never reachable" should read "cannot succeed or start a child."** See
section 4; the hooks are dispatched on `argv[1]`, so they are reachable in
principle and inert in practice.

**6. Happy-path fixtures invert the spec's field ordering.** `encode_control`
defaults to latest-start at entry+5s and kill deadline at entry+2s, where the
requirement has latest-start at outer+120 and the kill deadline at outer+1620.
Harmless, because the two entry checks are independent, but the suite never
drives the happy path on a record with the production ordering.

**7. Two properties are correct but untested.** Signal forwarding is tested only
for SIGTERM, and the absolute-timer requirement is not tested under
interruption. Both hold; section 3 records the measurements. The suite would not
catch a regression in either, and the absolute-timer property is the one the
requirement names explicitly.

**8. `O_NOFOLLOW` guards only the last path component.** An intermediate symlink
in the control directory path is followed, and there is no `openat2` with
`RESOLVE_NO_SYMLINKS`. Mitigated by the same requirement's mandate that the
control directory be an allocation-owned read-only mount; it matters only if that
assumption is relaxed.

**9. Disclosed choices the requirement leaves open.** The 2-second escalation
grace, the 500-millisecond reap sweep, the exit-code assignments, the CLI shape,
the subreaper flag, and exiting PID 1 after the bounded sweep. All are disclosed
in the build document and all are reasonable. Recorded so the architect ratifies
them rather than inheriting them unexamined; the sweep cap is the only one that
narrows the literal wording, and section 3 explains why it is safe.

## Why this is approved rather than warned

There is no defect in the wrapper and no test that cannot fail. The strongest
finding is a mechanism misattributed in the implementer's own report, which
costs the architect one sentence of evidence and no behaviour. Everything else is
a documentation precision fix, a coverage gap whose property I verified by hand,
or a choice the requirement deliberately leaves to the implementer and which the
build document already discloses. The three items to close before item 3 pins the
wrapper bytes are findings 1, 3 and 4.
