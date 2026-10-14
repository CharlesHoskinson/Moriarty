# FOREMAN_REPORT

- run_id: moriarty-ll-successor-20260913b
- role: implement
- slug: handoff-fable-repair
- branch: foreman/moriarty-ll-successor-20260913b/implement/handoff-fable-repair
- worktree: /home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair
- base_sha: 4c95364353e48540c686219fa5c318313994bda0
- status: completed (round 3, bounded correction under AFK authority; awaiting GPT-6 high check)
- implementer: Claude Fable 5.1 (claude-fable-5-1), medium effort, no delegation
- inputs this round: /tmp/moriarty-handoff-fable-r2-audit-gpt6-high.{md,json}, probe /tmp/moriarty-handoff-fable-r2-probes-gpt6-high.mjs and .jsonl (immutable, not edited)

## Summary

Both remaining round-2 findings are repaired: a late `buildResponse`
fulfilment can no longer write to a socket the protocol has already settled or
aborted, and protocol persistence still in flight at executor return (replay
denial records, denial writes, response writes) is now counted and reported as
outstanding in results and on rejections. The audit's looser-than-needed
timing allowance remark is also addressed. All earlier repairs (H1a-H1d, H3a,
H3b, L1) and the approved H2 bytes are preserved. The executor suite is 123
passing after the final source change. Production Python executor, accounting
and C wrapper integration remain the next capability and are not claimed here.

## Changed files (uncommitted; all under experiments/moriarty-midnight-financial/ledger/)

| File | This round | SHA-256 now |
| --- | --- | --- |
| fault-matrix.mjs | unchanged, approved H2 | 1d6b52248b8b1748e554395cfd22ee0075141de261e082edd5063a7fda4619dc |
| fault-matrix.test.mjs | unchanged, approved H2 | 3a2fc296104a0928095e5e413e6329353be0fb5a99d6eff00688fb278ef857b6 |
| executor-caller.mjs | persistence bookkeeping, outstanding-work reporting | 7452ab77b1c2d619858cdb5826495573877f2ffe793fdd0a6f4c53b53fc272f9 |
| executor-caller.test.mjs | 3 R2 tests; allowance 50 -> 25 ms | 0ce3b8bb5bae0fa92b68a6eb19fdf5407eda4f8a3dc14a433eda624e2dcf944a |
| invocation-handoff.mjs | post-await guard, persistence hooks, replay sockets on abort | 5b748c56a9bb53055077f2fe2b735d0cb6ca25b5c92124209c64618b4f50754f |
| invocation-handoff.test.mjs | 5 R2 tests | d781ed0e27bb2d5b74711b112633ca0017d0b8ef419ccbae94c49fe3ed08066f |

prover-lifetime.mjs (a046a77a…) and prover-lifetime.test.mjs (693f88c8…) are
unchanged. No Python, C, hooks, prover, financial, accounting, artifact or
network change. No commit, tag, stash or push.

## Repairs

### H1d-response-after-abort

`runHandoffServer` now rechecks `settled` immediately after `await
buildResponse()`, before validation, consumption and the response write. If a
timeout or abort settled during the build, the socket is destroyed if still
open and nothing is written; a builder rejection after settlement is ignored
rather than turned into a second denial. Abort also destroys replay sockets
whose denial is in flight, not only the active socket.

### H1d-pending-replay-not-reported

`runHandoffServer` accepts an optional `persistence: {begin, settle}`
bookkeeping pair (validated; the only interface addition). `begin` is called
before each denial record and before the response write; `settle` after the
denial write callback, after the response write callback, on recorder
rejection, or on the abort short-circuit. The executor supplies a counter per
run. `diagnostics.handoff` gains `pendingProtocolWork` and `outstandingWork`
is now `setup pending OR pendingProtocolWork > 0`. Rejections after runtime
allocation carry a non-enumerable `outstandingWork
{setupPending, setupPhase, pendingProtocolWork}`. The count is a report only:
nothing claims an outstanding record was cancelled or completed, and late
settlement decrements it after return.

### Timing allowance

`SCHEDULING_ALLOWANCE_MS` is 25 ms (was 50), below the 40 ms outer bound it
guards; the justification in the test file now cites the audit's 10 ms
replication.

## Evidence (this worktree, Node v24.18.1)

Round-2 probe on the final bytes, via an additive copy
`/tmp/moriarty-handoff-fable-r2-probes-worktree.mjs` that only rewrites the
frozen-snapshot import path to the worktree (the original probe and its
`/tmp/moriarty-handoff-fable-r2-freeze` snapshot are untouched):

| Probe | Audit observation (before) | Final bytes |
| --- | --- | --- |
| abort-during-build-response | writesAfterReturn 1, wroteOnDestroyedSocket true | writesAfterReturn 0, wroteOnDestroyedSocket false, listeners 0 |
| pending-replay-denial-hidden | outstandingWork false, no pending count | pendingProtocolWork 1, outstandingWork true, setup settled, cleanup removed |
| pending-replay-denial-late-settlement | pendingDenialWrites 0, secondWrites 0 | pendingDenialWrites 0, secondWrites 0 (no late write) |
| seven original probes | passing at audit | still passing (32 ms, reject at 19 ms, 39 ms, removed, removed, 0/0, PROVER_LIFETIME_LATE_ENTRY) |
| late-real-socket-allocation | passing | lateCleanup removed, nothing listening after |
| blocking-read-and-cleanup-shared-bound, 3 replicates | within 40 ms + 10 ms | 39.6, 40.2, 39.3 ms, all withinBound |

The adapted probe exits 1 at its line 115, the audit's own counterexample
assertion `writesAfterReturn === 1`; every success assertion before it passes.
That failure is the required outcome.

Tests, run once after the final source change and once after the allowance
change: `npm --prefix experiments/moriarty-midnight-financial run test:executor`
gives 123 tests, 123 pass, 0 fail, 0 cancelled, in 1.86 s. Before this round
the same command gave 115/115. The full language and financial gates are run
by the enclosing lane and were not repeated here.

New tests and what they detect:
- protocol: abort during an async response build (late response never
  written); setup timeout during an async response build (denied, no late
  response); bookkeeping through a real AF_UNIX success response and a replay
  denial record and write; abort with a replay record in flight (record stays
  outstanding until its own settlement, replay socket destroyed, no denial
  write); malformed bookkeeping rejected.
- executor: replay denial in flight at return after a successful handshake
  (`pendingProtocolWork 1`, `outstandingWork true`, no write after release);
  child error with a denial record in flight (`error.outstandingWork
  {setupPending true, setupPhase 'handshake', pendingProtocolWork 1}`, no write
  after release, no listener left); completed handshake with nothing in flight
  reports zero.

One new test failed on its first run: the protocol abort test found that
replay sockets were not destroyed on abort. That was a genuine gap in the
first edit of this round and was corrected in source before the final run.

Other checks: `git diff --check` clean; attribution grep over `git log --all`
matches nothing; stash list empty; fault-matrix hashes equal the approved
manifest.

## Original findings and their disposition

- First audit H2: fixed, approved, byte-identical.
- First audit H1/H3 and second audit H1a, H1b, H1c, H1d, H3a, H3b, L1:
  repaired in rounds 2 and 3; the seven original probes still pass.
- Round-2 audit H1d-response-after-abort and H1d-pending-replay-not-reported:
  repaired this round as above.
- C1: open. No Python loan executor or runner consumer of these helpers exists;
  not created here.
- C2: C wrapper source and its tests exist in the base and were not rerun this
  round. The author-observed history stands: 22 passed, 2 failed
  (`assert_pid_gone`, PID remaining under /proc) on this host; root is
  verifying the strong-namespace context separately. Production integration is
  not established.
- C3: open. The caller still hardcodes terminal evidence, stop and timer facts
  to false; `PROCESS_SUCCESS` cannot be produced from genuine evidence.
- Historical five-minute focused hang: not reproduced by the independent
  two-file check (82/82 in 1.91 s) or by any run this round (all under 2 s);
  no cause identified and no teardown fix is claimed. `afterEach` teardown
  existed in the base.

## Remaining production limits

- Outstanding I/O is reported, never cancelled. A synchronous blocking fsync
  inside a recorder, reader or remover is not preemptible; no supervised
  persistence worker exists.
- `pendingProtocolWork` is an in-process count; the diagnostics object is a
  frozen snapshot at return and later settlement is observable only to a
  caller that owns the recorder.
- Late allocation cleanup after a setup timeout is best effort through
  `lateAllocation.settled` and cannot settle if the allocation never fulfils.
- The schedule is per process (`performance.now()`) and not shared with the
  child or the payload's outer start.
- No K, Java, Docker, network, wallet or live financial action. The production
  Python executor and accounting remain the next capability.

## Open questions

None for this lane. Next: GPT-6 high check of these exact bytes.
