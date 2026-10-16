# DoctorWatson security and quality audit: candidate tree `74bc0296…`

## Verdict: **APPROVE_WITH_SCOPE**

I found no blocking defect. I ran nothing: no commands, tools, writes or network. Every judgment below comes from reading the supplied bytes and captured results, and I could not inspect the gzip archives' contents. The harness identifies this model as `claude-opus-5-5`; I cannot observe my own effort setting.

## Corrections verified against the source

- **W2: terminal dispositions survive upstream edits (`doctorwatson.qnt`, `applyEdit`).**
  - An impacted dependent that is `Cancelled` or `Blocked` keeps its phase, `processCycles`, `adminMinutes`, `failures`, `reproduced` and `changedApproach`.
  - It still loses its current evidence through `reset` and the version bump, and its `frozen` flag is cleared.
  - No other path revives a disposed task. `cancel` and `nameBlocker` produce absorbing phases; `canEdit` admits only active or `Merged` tasks; `applySharedInput` skips disposed tasks.
  - The RED/GREEN chain holds:
    - `22-RED` failed on model `13fce4…`.
    - `25-RED` failed on the intermediate model `4b0931…` under the current test `7c1d1b…`.
    - `26-GREEN-1` passes 30 scenarios on the current model `29bf91…`.
- **W4: directed completion now gates the run (`doctorwatson_coverage_test.qnt`).**
  - `47.reps(driver::step).expect(directedFinished)` fails on any disabled cursor step. The mutant control `24-…-2` shows exit 1 with QNT513, while `run --witnesses` still exited 0 on the same mutant (`24-…-1`).
  - The committed driver has 47 cursor branches (0–46), each calling one guarded machine action. I traced the walk by hand: `merge(1)` at cursor 16 makes T2 stale, then `rebase(2)` at 17, observations at 36–39, `edit(1, Tool)` at 42, `changeDependencies(1, 2)` at 43, `cancel(1)` at 44, `nameBlocker(3)` at 45 and `finalize` at 46. Each guard is satisfied in sequence.
- **W6: the serializer is pinned before execution (`check.py`, `check_rust_transcription`).**
  - The helper's bytes are read once and their hash is required to equal `83acde…`, which matches the supplied `check-reference.py`.
  - Exactly those verified bytes are compiled with `dont_inherit=True` and executed in a module dict whose `__name__` is `sherlock_literal_serializer`. The helper's `main()` therefore never runs, `loader.exec_module` is not called and no bytecode cache is used.
  - Without `dont_inherit=True`, `compile` would inherit check.py's `annotations` future flag. The helper has no annotations, so the leak was harmless, but the fix is correct.
  - `hardening-RED` shows the unpinned helper ran (`RuntimeError: serializer executed`); `hardening-GREEN` passes 33 tests on `d0cab9…` and `04185967…`.
- **W7: invalid observed indices are rejected (`check.py`, `state_at`).**
  - Only an in-range, non-negative value of exact type `int` selects a state, so `True` and `-1` are both rejected. It is applied to both `publication.pre_state_index` and prefix column 1.
  - The other index sources come from JSON-pointer list positions inside the `first_difference` zip range, or are caught by the `try`.
- **W8 (`AGENTS.md`, `ROADMAP.md`):** both now say the October 16 override suspends Foreman, Pel and campaign dispatch for current work.
- **W10 (`DOCTORWATSON-CI.md`):** it states that relative overrides resolve against the checkout root, and its first-run wording now matches the preflight order.
- **Timeout exit code:** under GNU coreutils, a child killed with `--signal=KILL` makes `timeout` exit 137. `timeout-return-01.json` shows this host's uutils `timeout` returns 124. Either value is nonzero and stops `set -e`, and no exit was relabeled.
- **design-04 bindings:**
  - All seven before/after hashes match the manifest.
  - The serializer, the K source and the four oracle files are pinned inside `check.py`.
  - The two `.rs` hashes in stdout match the supplied file headers. Their structure matches `serialize_oracles` output field by field (`frames: &[…]`, `Some(n)`/`None`, the `EXTRA_` prefix and the trailing `];\n`).
  - Witness counts are 18 positive and 7 zero: reviewed, rebased, integrated, dependenciesChanged, layerRecorded, completed and sourceOnly. The directed run reports 6 of 6 in 64 ms, and the total is 35.243 s.

## Non-blocking warnings, each with a minimal fix

**N1. Other actors can clear a task's SF-14 stop counters.**
- **Where:** `doctorwatson.qnt`, in `applyEdit` (impacted tasks with `other != id` that are not disposed) and in `applySharedInput` (active and merged tasks).
- **Problem:** both set `processCycles` and `adminMinutes` to 0. An upstream owner's edit, or an integrator's shared-input change, therefore clears a stopped task's process history even though that task's owner did no eligible work.
- **Effect:** after re-freezing, the dependent can submit and start, because `version > 0` is already true. `applySharedInput` also leaves `failures` untouched, unlike `applyEdit`.
- **Fix:** reset the counters only when `other == id`, or document this as intentional, and add one regression.

**N2. Some evidence-index entries are dangling or stale.**
- `TERMINAL-AND-DIRECTED-CORRECTION.json` cites `23-terminal-and-coverage-GREEN-0…4`, and `TERMINAL-HISTORY-CORRECTION.json` cites `26-…-GREEN-3/4`. None of these are in the manifest.
- The models README's statements that the 23-* captures passed both regressions first, and that the 26 closure includes the coverage test and a one-sample main run, are unverifiable from the archive. design-04 independently covers the same hashes, which is why this is not blocking.
- The audit README says "isolated RED sources are retained as gzip files" in the paragraph about the 24-* negative control. The mutant coverage file `83ac17bb…` has no archive in the manifest.
- The current-results section of `QUINT-REVIEW.md` links `directed-result.json`, which binds the predecessor hashes `13fce4…` and `127ef16a…`.
- In the same file, the "Coverage warning: five rare predicates" bullet is not scoped to the predecessor run.
- `hardening-GREEN.json` binds the QA README at `5bd181…`, whereas the current README is `b6e36f…`. This affects documentation only.
- **Fix:** archive those receipts or mark them runtime-only, and repoint or scope the stale links and bullet.

**N3. The Python QA commands inherit the caller's environment.**
- **Where:** `scripts/doctorwatson.sh`.
- **Problem:** both Python commands run with the full caller environment and with the script directory on `sys.path`. The helper's imports that check.py has not already loaded (`shlex`, `subprocess`, `tempfile`) resolve through `models/sherlock/qa` first.
- **Scope:** this lies inside the disclosed trusted-checkout and unpinned-imports limit.
- **Fix (cheap hardening):** run both commands with `python3 -I`. `unittest discover -s` inserts its start directory into `sys.path` itself, so discovery should still work, but I have not verified that.

**N4. The pinned-bytes and `dont_inherit` behavior has no dedicated regression.**
- Only the hash refusal is tested; the compile-from-verified-bytes path is established by source inspection alone.
- **Fix (optional):** add a test that swaps the helper file after hashing, or that patches `loader.exec_module` and asserts it is never called.

**N5. `observation_state` accepts a malformed `post_overrides` value.**
- **Where:** `observation_state`, via `result.update(post_overrides)`.
- **Problem:** `dict.update` accepts a list of pairs. If an observed `post_overrides` is a list, it yields a strange reconstruction instead of `None`.
- **Effect:** diagnostics only, because the status is already a mismatch.
- **Fix:** require `type(...) is dict` first.

**N6. The manual site workflow is not supply-chain pinned (`.github/workflows/site.yml`).**
- Its actions are pinned by tag rather than commit SHA, and `pip install playwright` has no version pin. The workflow is manual-only.
- **Fix:** pin both when convenient.

**N7. The models README overstates the memory limit.**
- `models/doctorwatson/README.md` calls the limit a "256 MiB Node heap". It is an old-space limit only, as `DOCTORWATSON-CI.md` correctly says.

**N8. Previously disclosed limits remain accurate.**
- W1: sampled `ownership` cannot fail in this instance.
- W3: default sampling is dominated by cancellation.
- W5: `localOnly`, `noEscalation`, `noPrematureStop` and `independentOracle` are true by construction.
- The per-command timing headroom remains tight: the simulation alone took 26.8 s against a 30 s cap, and per-command elapsed time was not captured.

## Observed scope (supplied results only)

- **Script:**
  - Argument and preflight exits are 2 or 127 before any checks run.
  - Execution is fail-stop.
  - Model commands get a minimal environment with per-command wall and old-space caps.
  - There is no fallback and no install, verify, Rust-backend, hosted or native step.
- **Checker:**
  - Observed input is parsed strictly.
  - The structural diff compares exact types, checks identity first and refuses absent or partial observations.
  - Readback is byte-exact against the pinned serializer, and exit codes are 0, 1 or 2.
- **Model:**
  - JobToken binds both candidate and attempt.
  - Edit, rebase, merge and shared-input changes invalidate dependent evidence.
  - Dependency cycles are refused.
  - Terminal dispositions and their history survive upstream edits.
  - Layer availability does not count as an observation, and NativeLedger cannot be observed.
- **CI boundary:** the only hosted workflow is `workflow_dispatch`.

## Unverified or out of scope

- **Archived content I did not inspect:**
  - the gzip RED sources;
  - `performance-review.md`;
  - the runtime-only receipts (23-*, 26-*-3/4, the 24 mutant source, attempts 01–09);
  - the symlink target of `.npm-global/bin/quint`;
  - the `.rs` bytes beyond their structure and supplied hashes.
- **Implementation fidelity:** nothing here shows that real agents, Git, reviewers or the shell implement the model. Dependency-registry completeness and OS hermeticity are also unverified.
- **Sherlock correctness layers:** K execution, the Sherlock Quint projections, Lean/Clean, handwritten circuits, the Rust evaluator, and native, proof and stagenet behavior were not run.
- **Advisory material:** the raw planning reviews, research reports and V1 decomposition are not certified.
- **Unchanged constraints:** native authority 0, C4, the counters and the terminal Sherlock attempts are unchanged, and every U0–U7, M1–M17 and MC/SP/G exit remains open.
