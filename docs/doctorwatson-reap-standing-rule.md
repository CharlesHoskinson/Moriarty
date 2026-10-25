# Expired-job reaping: standing rule for the owner, and escalation design

## Why
On 2026-10-25 a `cargo test --workspace` descendant outlived its 5400 s runner cap and held
heavy.lock and gpu.lock from 10:21Z to about 16:37Z. The coordinator's stop was denied by the
permission system, correctly, because nothing proved the process was the leased job. Nine
heavy jobs waited. The fix has two halves: jobs terminate themselves (heavy.py lease, session
kill, default wall cap) and, when that fails, a narrow reap command whose target is provable.

## Proposed standing rule (owner decision; nothing is enabled until you create the file)
> A process may be signalled by `scripts/doctorwatson reap --execute` if and only if all of:
> 1. it is in the session recorded in a DoctorWatson lease file (`heavy.lock.lease` or
>    `gpu.lock.lease`) whose declared deadline has passed;
> 2. the lease's recorded pid and `/proc/PID/stat` starttime still match (no pid reuse), and
>    the session id matches;
> 3. the session is not the caller's own;
> 4. it is TERM first, KILL only after the grace period, and only processes verified at signal
>    time to be in that session.
> No other process is covered. Agents may not use `kill` directly. Enabling is the existence of
> `~/.config/doctorwatson/reap-enabled`; deleting it revokes the rule.

Until the file exists `reap --execute` exits 3 and signals nothing. Default `reap` is a dry run
(`--json` for machines). Orphaned descendants of a dead leader are covered because a session id
cannot be recycled while any member lives.

## Read-only .git: `scripts/hostcommit WORKTREE "message" PATH...`
Run on the host (coordinator or runner) for an agent whose sandbox cannot write .git. Commits
exactly the listed paths, never pushes, refuses messages with AI attribution. Longer term, put the
worktree gitdir in the launcher's writable roots (wf-5's launch.sh).

## Owner-blocked escalation: `scripts/blocked-notify`
`add KEY "text"` when a decision only the owner can make blocks the factory, `clear KEY` when
resolved, `due` each coordinator tick. `due` prints one line per item blocked at least 20 minutes
(repeating at most hourly) and also adds any expired lease from `reap --json`. A script cannot push,
so the coordinator passes each printed line to PushNotification. The line is self-contained, for
example `[lease:heavy:W12] blocked 25 min: expired heavy lease W12 sid 4122246 overdue 1500s; reap
needs owner rule`.

## Bus identity claim prefixes (exit 4)
`agentbus claim slot:...` fails for launched identities because they have no claim prefixes. That is
AgentBus identity configuration (wf-6 / launcher), not DoctorWatson code: register `slot:`, `lock:`,
`wallet:` prefixes at identity creation in launch.sh, then verify one claimant succeeds and a second
is refused.
