---
title: "Lessons: running the factory and its agents"
project: lessons
date: 2026-10-24
tags: [lessons, agents, codex, agentbus, doctorwatson, sandbox]
---

# Running the factory and its agents


## Hold-and-wait between the heavy slot and the GPU lock stalls everyone (2026-10-24)
- **What happened:** jobs took DoctorWatson `heavy.lock` and then queued for `gpu.lock` behind 10-minute fuzz slices, or the reverse. A6, W9's GPU suffix and the lab probes all waited 15–30 minutes while the load was 2–4 on 20 cores.
- **Next time:** take both locks in one fixed order (heavy, then GPU), and only right before the proof starts, not during compile or setup. Batch several GPU tests in one reservation. Keep fuzz slices short.

## Agents that block on one sender miss their lead's tasks (2026-10-24)
- Two lab engineers ran `wait --from gpu-runner` and never saw TASKs from their lead. Tell agents to `wait` on any sender, or `recv` between runner waits.

## `pkill -f <script>` from a shell whose command line contains that name kills the shell (2026-10-24)
- Match on PIDs from `pgrep`, excluding your own process tree, rather than on a pattern in your own command.

## The Codex sandbox cannot reach the GPU or write `.git`, so give it a host runner (2026-10-24)
- **What happened:** `codex exec -s workspace-write` agents could not open the WSL GPU device. Adding `/dev` to the writable roots breaks bwrap.
  They also could not create `.git/worktrees/<wt>/index.lock`, even with the git common dir listed as a writable root. And
  downloads into `~/.cargo/registry/cache` failed intermittently with "Read-only file system". One agent dodged that by pinning older crate versions.
- **Fix:** a host-side AgentBus service, `scripts/qa/gpu-runner.py` (systemd user unit `dw-gpu-runner`). It accepts `RUN <tag>`
  (allowlisted `cargo` subcommands, including `fetch` and `fuzz` with a leading `+toolchain`) and `COMMIT <tag>` (git add + commit with
  hooks, refusing AI-attribution text) from sandboxed agents. Every argument is validated, it runs without a shell, one job at a time under a flock, and
  only inside allowed worktree roots.
- **Next time:** start the runner before launching sandboxed agents, and put the RUN/COMMIT/fetch protocol in the first brief.
  Restarting the runner can drop an in-flight job's reply: check `systemctl --user status` for child processes first.

## An OpenAI content filter can refuse a correctness brief (2026-10-24)
- **What happened:** the GPT-6.1 Sol fuzzing agent (W7) was stopped twice with "flagged for possible cybersecurity risk".
  The first brief said "soundness … false statement that verifies … CRITICAL"; the reworded "differential property testing" brief was refused too.
- **Fix:** W7 moved to a Claude Opus agent, which also runs the GPU directly (no sandbox) under the shared GPU flock.
- **Next time:** route fuzzing and negative-testing work to Claude from the start; keep Codex for implementation packages.

## AgentBus channel posts are only seen by subscribers (2026-10-24)
- **What happened:** coordinator notes sent to `#loki` did not reach agents that had not run `sub '#loki'`. Direct messages did.
- **Next time:** send operational notes as direct messages to every agent, and tell agents to `sub` at start-up.
  Delivery can also arrive late: bus messages from 23:4x showed up in a 19:12 local `recv`.

## The retry rule works: it stopped a blind re-run (2026-10-24)
- **What happened:** after A2 failed on a time budget during a cold compile, `doctorwatson check` refused the identical re-run until a `--hypothesis` was given.
- **Lesson:** budgets are measured on warm builds. Prebuild test binaries (`cargo test --no-run`) outside `heavy` in a fresh
  worktree, or the first run fails on its budget rather than on its code.

## Hook git variables leak into tests (2026-10-24)
- **What happened:** factory unit tests run from inside the pre-commit hook inherited `GIT_INDEX_FILE` and committed "one"/"two" into the real repository.
- **Fix:** `qa/tiers/lib.sh` `DW_SCRUB_ENV` unsets the repository-redirecting `GIT_*` variables for every step. There is a regression test in
  `qa/tiers/tests/test_tiers.sh`.

## Host resources are part of the design (2026-10-24)
- **Disk:** the root filesystem was 95% full. 1.13 TB was dead Midnight preprod chain data in rootful Docker (nodes crash-looping on a
  runtime upgrade since July). After deleting it, 1.2 TiB was free. Check `sudo docker system df -v` before blaming builds.
- **Root:** the owner granted passwordless sudo (`/etc/sudoers.d/90-charles-agent`) and does not want to run commands for the agent.
- **CPU:** 9 Codex agents compiling Rust pushed the load to ~31 on 20 cores. `CARGO_BUILD_JOBS=4` per agent plus one global `heavy`
  slot kept it workable. The single heavy slot made agents queue, so tell them up front that plain `cargo check/test` doesn't need it.
- **GPU:** one flock (`~/.cache/doctorwatson/gpu.lock`) is shared by the runner, the fuzzer and manual measurements. A long fuzz
  run holds it for 30 minutes, so measurements queue behind it.

## Concurrent agents need one owner per file and a coordinator who merges (2026-10-24)
- Six Loki branches each touched `Cargo.toml`, `prover.rs`, `domain.rs` and `msm.rs`. The merges needed hand resolution: the profiling
  hooks had to move inside the new `fft`/`ifft` wrappers, and `blst` was declared twice. Run the full gate after every merge.
- The five Lean agents worked because each owned one file and asked the others for changes over the bus. The auditor
  rejected drafts until they were faithful to the Rust code.
- A subagent that finishes while blocked on a dependency (lean-5 on Plonk) stops for good. Resume it with SendMessage once the dependency lands.
