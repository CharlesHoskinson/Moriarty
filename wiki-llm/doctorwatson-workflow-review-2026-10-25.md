---
title: "DoctorWatson workflow review: 2026-10-25 retrospective"
project: doctorwatson
date: 2026-10-25
tags: [doctorwatson, workflow, retrospective, lessons, loki, sherlock, agentbus]
---

# DoctorWatson workflow review: 2026-10-25

This is the hub note for the second workflow retrospective. The earlier one is [[doctorwatson-workflow-review-2026-10-22]].
The owner asked to "recursively self improve your workflow, have 10 agents launched to review the last 7 hours of work and
see what stalled and what was ineffective and propose an update to our workflow tools". The owner then asked the reviewers
to "implement their findings upstream" and to update this wiki and its graph.

- **Window reviewed:** 2026-10-25, 10:00Z to 17:45Z.
- **Reviewers:** wf-1 to wf-10, one angle each (see the table below). They coordinate on AgentBus `#workflow`.
- **Implementation branches:**
  - `dw-retro` in `~/repos/Moriarty-wt-dw-retro` (DoctorWatson);
  - `agentbus-retro` (AgentBus source);
  - `loki/merge-train` (Loki merge tooling);
  - the wf-5 launcher and park tooling, kept in the coordinator scratchpad.
- **Improvement-log entries:** DW121 to DW130 in [[doctorwatson-improvement-log]].
- **Related lessons:** [[factory-and-agents]], [[prover-performance]], [[correctness]], [[sherlock]].
- **Related Loki notes:** [[LOKI-DESIGN]], [[SUITE]] (the benchmark closing condition) and the [[loki/README|Loki research index]].

## What happened

1. **The orphaned job.** About 08:51Z, loki-w12 submitted `w12_workspace_checks` to the host GPU runner as a `cargo test` job.
   That test is a launcher: its own source takes `heavy.lock` through `scripts/doctorwatson heavy` and then `flock`s `gpu.lock`
   (`crates/mori-recursion/tests/a3_runner/src/lib.rs:355-369,419,596-611` in `loki-wt/w12-a3-residual`). The command inside
   was a full `cargo test --workspace`, with no DoctorWatson budget.
2. **The runner timeout killed only cargo.** At 10:21Z the runner's 5400 s limit fired and the job returned exit 124. But
   `scripts/qa/gpu-runner.py:71-74` uses `subprocess.run(["cargo", ...], timeout=limit)`, with no new session and no group
   kill, so only `cargo` died. The test binary, the DoctorWatson wrapper and the workspace run survived, still holding
   both locks. The wrapper (`models/doctorwatson/factory/heavy.py`) had no cap, so it waited in `os.wait4` with no limit.
   Its own capped path, `run_command` (`heavy.py:74-76,118,127`), already uses `start_new_session` plus `killpg`.
3. **The kill was denied, and nobody was told.** At 09:59:58Z the operator permission system denied the coordinator's kill.
   That one command also bundled the coordinator's legitimate withdrawal of P7's queued smoke run. The denial text says to
   explain the problem to the user. Instead, the note "owner must reap" went into the coordinator's plan file, and no
   PushNotification was sent between 09:30Z and 17:31Z.
4. **Everything heavy waited.** Nine jobs waited for the locks: B4 stagenet, W12 GPU, the stagenet L2 GPU run, P5 slot (a),
   P4 native gates, P3 native reruns, the U023 test queue, the P7 smoke run and B5. The test finished FAILED after 29,296 s
   (log mtime 16:59:46Z). B4 then ran 17:10–17:32Z and passed: it had waited about 7 hours for a 22-minute job.
5. **The failures were not W12's.** W12 diagnosed the failures from source at 15:38Z: 12 pre-existing Moriarty C6 test bugs
   (no-op mutations, plus a mis-scoped Push guard). sherlock-u fixed them in `ad54d3aaa`. This was useful, but it came out
   of a run that had already expired.

## Findings beyond the orphan

| # | Finding | Evidence |
|---|---|---|
| F1 | **No escalation.** A denied action became a note in a file the owner doesn't read. | No PushNotification calls 09:30–17:31Z; the denial at 09:59:58Z; LOOP-PLAN "owner must reap" at 10:30Z. The owner returned at 17:31Z, after the orphan had exited. |
| F2 | **Idle agents.** Agents sat idle while source-only work was available. | Hand-backs: P5 10:30, P3 11:16, stagenet 11:22, P4 11:48, P7 12:34. Ticks at 12:59, 13:30 and 14:01 took no action, and tool calls fell from 40 to 2–3 per 30 minutes. The known P7 RNG-seeding fix was only assigned at 14:32. About 20 idle agent-hours. |
| F3 | **Health checks that could not fail.** | "[3 prior /loop wakeups found nothing actionable; loop is healthy]" was reported while 9 jobs were blocked. The plan's rule "append lessons each tick" was not followed: the last lessons commit was 2026-10-25 03:32Z, before the window. |
| F4 | **Work nobody asked for or used.** | astra-gpu opened S4 reservation-scope design work (rev1 to rev4, pulling in P7 and P4). The coordinator froze it, and S4 is not on the v1 finish line. Its four REVIEW.md commits (`cefd65d`, `d5889f2`, `95d2736`, `4a40fe9`) were not pulled anywhere: `loki-proofs` had 0 commits in the window. |
| F5 | **A recorded lesson recurred.** | astra-gpu missed `#loki` until 12:28Z because it wasn't subscribed, which is the 2026-10-24 lesson in [[factory-and-agents]]. Lessons are written down but not enforced by tooling. |
| F6 | **Duplicate posts.** | 8 same-second posts went to both the coordinator and a channel (sherlock-b, loki-w12, loki-stagenet). Status bodies are full of disclaimers like "No READY/DONE/RELEASE claimed" (9 in the window). |
| F7 | **The heavy lock is too coarse.** | `heavy.lock` is a single mutex. CPU-only gates (P5 slot (a), the 17 U023 fragment runs) queued behind one test on a 20-core host. |
| F8 | **A panel decision overruled in one sentence.** | The U014 panel split at 09:56Z, so U014, U015 and U017–U020 were held. The owner overturned the hold at 17:33Z ("Change whatever you like in the design of sherlock"). This is now standing authority (memory `sherlock-design-authority`). |
| F9 | **Zero merges.** | No Loki merge happened in the window: P3, P4, P5, P7, W10 and W12 all wait on native receipts that need the locks. CPU-mergeable work had no separate path. |

**Root cause.** Only `heavy.py` records a lock's holder, deadline and age, and the runner bypasses it. Nothing turns
"blocked", "idle" or "past deadline" into an action. Lessons go to prose, not to checks.

## Changes, by owner and improvement-log number

| DW | Owner | Change | Files |
|---|---|---|---|
| DW121 | wf-1 | Lease file per lock, a default wall cap, session kill on timeout or signal, and `PDEATHSIG`. The runner kills the whole job tree on timeout. | `models/doctorwatson/factory/heavy.py`, `scripts/qa/gpu-runner.py` |
| DW122 | wf-2 | A durable heavy queue with tickets, called through wf-1's hook (`queue.acquire_turn`). | `factory/queue.py`, `cli.py` (`doctorwatson q`) |
| DW123 | wf-3 | Cheaper workspace tests: dependency opt-level, nextest profiles and a `--tier` design. | `Cargo.toml` profile, `.config/nextest.toml` |
| DW124 | wf-4 | `doctorwatson reap`: dry-run by default, and it kills only an expired lease's own session. The kill path needs the owner's standing rule. | `factory/reap.py`, the standing-rule text |
| DW125 | wf-5 | `AGENT-STATE.json`, `launch.sh --resume`, the `park` protocol (exit 75) and a dry-run agent supervisor. | wf-5 impl dir, then the live `launch.sh` |
| DW126 | wf-6 | Typed bus messages with required fields, subject length checks, a lint for mashed words, auto-recv and replay flags. | `tools/agentbus` (agentbus-retro) |
| DW127 | wf-7 | `scripts/tick-check` (one command per tick) and Monitor triggers for a lock release or a BLOCKED/READY message. | `scripts/tick-check` and its trigger script |
| DW128 | wf-8 | A Loki merge train with a MERGE-CPU / RECEIPT-NATIVE split. | `scripts/train.sh`, `verify-main.sh --tier`, `deps.yaml`, `scripts/loki-dag` |
| DW129 | wf-9 | A clean-tree exemption for evaluator status files, `doctorwatson contract-check` and a holdout contract schema. | `qa/holdout/contracts/`, the check CLI |
| DW130 | wf-10 | The retrospective metrics loop (below) and this hub note. | this note, [[doctorwatson-improvement-log]] |

## How the workflow keeps improving itself (DW130)

1. **Data.** Once DW121 and DW127 land, every lock acquire, release and expiry is a lease record and a bus event. The bus
   database, every `qa/sherlock/runs.jsonl`, the leases and the agent-state files are then enough to compute the metrics.
   No transcript reading is needed.
2. **Metrics,** every 4 hours, written to `wiki-llm/lessons/retro-YYYYMMDD-HH.md`:
   - lock utilisation, and lock-hours held past the deadline;
   - blocked-job-hours;
   - QUESTION/BLOCKED-to-answer latency;
   - idle-agent-hours while the source-only backlog is non-empty;
   - duplicate-post rate;
   - unconsumed commits (review records not merged or pulled within 12 hours);
   - escalations sent per denied or blocked action.
3. **Rule.** The worst metric's remedy becomes the next coordinator task, with a RED test. The next retrospective must show
   that metric improved, or the remedy is reverted and the dissent recorded. A lesson that recurs (like F5) must become a
   check, not another paragraph.
4. **Escalation is a metric.** A lock more than 20 minutes past its deadline triggers one self-contained owner
   notification, then at most one per hour, for example "kill 4122246 (W12 tests, expired 10:21Z); blocks 9 jobs".

## Owner approvals needed

- **A standing rule for DW124:** `doctorwatson reap --execute` may kill a job's recorded session once its lease deadline
  has passed and the pid and start time match. The permission system stays the authority until the owner grants this.
- **Swapping in the new `launch.sh` and supervisor (DW125)** while agents are live.
- **Reinstalling AgentBus from `agentbus-retro` (DW126)** while agents are live.
- **Merging `dw-retro` into `factory` and `loki/merge-train` into Loki main.** There is no push without asking.

## Measured cost of the window

| Cost | Estimate |
|---|---|
| Lock-blocked jobs | About 9 jobs × 6.6 h (10:21Z to about 17:00Z) |
| Idle agents | About 20 agent-hours |
| Loki merges | 0 |
| Unconsumed review commits | 4 |
| Duplicate posts | 8 |
| Time until the owner was notified | About 7.5 h (never notified) |
