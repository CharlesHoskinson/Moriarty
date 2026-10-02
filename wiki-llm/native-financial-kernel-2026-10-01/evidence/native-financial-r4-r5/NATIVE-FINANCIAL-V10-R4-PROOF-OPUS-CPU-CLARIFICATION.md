# NATIVE-FINANCIAL-V10-R4 proof: Opus clarification of C-1 (CPU admission)

- Requested auditor: `claude-opus-5-5`, effort `high`. The root records the returned runtime identity separately. This file's SHA is not a model identity.
- Subject: my own review `NATIVE-FINANCIAL-V10-R4-PROOF-OPUS-REVIEW.md`, SHA-256 595f9649e97ed911d634d438927ed4602beb3909ad2d581d1e5ff387db9eb4f0. I rehashed it at 2026-10-01T22:33Z and it matches.
- Frozen source: `NATIVE-FINANCIAL-V10-R4-PROOF-SOURCE-FREEZE.json` 312c0d1d…a13f. I did not reopen or alter it.
- Date: 2026-10-01. Evidence root B = `/home/charl/research/moriarty-signed-intent-2026-10-01`.

## Scope and method

This file interprets my existing full exact-source review. It contains no new native result and no new source audit. My A, B and C votes are unchanged.

I read only two things:

- My own report, in full.
- The checked-in develop skill header, as the startup instruction requires.

I did not read any counterpart R4 report. I did not invoke native, Cargo, systemd units, Git or caches, and I dispatched no subagents. I wrote nothing except this file.

I ran these read-only host queries: `/proc/loadavg`, `nproc`, one 5 s `/proc/stat` delta (computed in memory), and a `ps` snapshot. This session is fresh. I am interpreting the report's text and do not claim recall of the original session.

## The ambiguity

Two parts of the report point in different directions:

- **The C-1 body** says "I recommend that root's live admission confirm CPU headroom (load substantially below CPU count, no competing heavy jobs) rather than launching into the currently observed load."
- **The final line** says "C RESOURCES = APPROVE_SCOPED (conditions C-1 to C-5 for live admission)". The C heading also says "(the frozen envelope and launch form; not live admission)".

The word "recommend" is weaker than the report's own structure. I resolve the ambiguity toward the stricter reading because:

1. The C approval explicitly excludes live admission.
2. The summary line names C-1 as a condition for live admission.
3. The body expressly advises against "launching into the currently observed load".

This adds no strictness beyond the original text. It removes a word that could be read as optional.

## Answer

**C-1 is a mandatory admission prerequisite under my C vote. It is not a recommendation with accepted wall-stop risk.**

My C vote approves the frozen envelope and launch form. It does not approve a launch while the host is CPU-saturated. R4 has one irrevocable prove attempt. Admitting it into roughly 4× oversubscription makes the 600 s wall stop a foreseeable outcome rather than a residual risk. Approving that would waste the single attempt.

Once the condition below holds, the remaining wall, CPU and RSS stop risk is accepted. Any such stop would be a valid consumed resource stop. It would never be a financial result and would never be retried.

### Exact C-1 condition

Root evaluates this from the host within the 60 s before the `systemd-run` call and records the raw values with UTC timestamps. N is `nproc` at that moment; it is currently 6.

1. **Load average.** In `/proc/loadavg`, the 1-minute load is at most N − 2 (4.00 at N = 6) and the 5-minute load is at most N (6.00).
   - "Substantially below CPU count" means leaving at least two CPUs free, matching `RAYON_NUM_THREADS=2`.
   - The 5-minute bound excludes launching on a momentary dip.
   - Load merely below N, for example 5.9 of 6, does not meet my condition.
2. **Measured idle.** A `/proc/stat` aggregate delta over at least 5 s shows idle+iowait of at least 2.0 CPU-equivalents (fraction ≥ 2/N).
3. **No competing heavy jobs.** None of these processes is running: another native consumer, `cargo`, `rustc`, a prover, or a `lean`/`lake` build. This is in addition to the C-5 exclusivity checks.

If any part fails, there is no launch. Root may wait and re-sample any number of times. That is not a retry, because no reservation exists until launch.

Root may not:

- Lower these thresholds.
- Stop unrelated user jobs on its own authority.
- Raise any limit.
- Treat recovered free disk space as CPU headroom. Disk headroom and C-1 are independent conditions.

### Status of C-2 to C-5 (unchanged, stated precisely)

- **C-4 and C-5 are mandatory.** Root must record the unit, InvocationID, MainPID and authority SHA, and treat a `systemd-run` exit of 0 as launch only. Immediately before launch, root must re-check from the user manager:
  - No `PYTHON*` or `COVERAGE*` variables in the environment.
  - The namespace smoke passes.
  - Free space is at least the 10 GiB floor, and target/cache sizes are within limits.
  - No competing native process is running.
  - Unit-name and path exclusivity hold.
- **C-2 and C-3 are disclosed, accepted risks.** C-2 is no linger (session loss). C-3 is suspend or pause outside monotonic time. Neither is an admission prerequisite, because removing either would need a host mutation outside this scope. Root records the linger state, and a stop from either cause is a consumed stop.

## Current observation and decision

All values below are read-only, taken 2026-10-01 22:33:05Z to 22:33:10Z:

- `nproc` = 6.
- `/proc/loadavg` = 19.96 / 23.71 / 24.28, then 20.04 / 23.67 / 24.26.
- The 5 s `/proc/stat` idle fraction was 0.003, so about 5.98 of 6 CPUs were busy.
- Run states: 10 R and no D. The load is genuine runnable contention, not I/O wait inflation.
- The top consumers are unrelated user `chrome`/`MainThread` processes, several at 35–68 % CPU each.

Free disk space may well have recovered, but CPU headroom has not. The 1-minute load is about 5× the threshold, and measured idle is about 0.02 CPU-equivalents against the required 2.0.

**Decision: REFUSE_CURRENT_LIVE_ADMISSION**, until conditions 1–3 of C-1 and the C-5 checks hold at a fresh pre-launch measurement.

When those conditions hold, my C vote **admits one finite R4 prove execution** under these requirements:

- The limits are unchanged: 600 s wall, 1200 s group CPU, 4 GiB RSS/AS, 10 GiB free floor, and the other frozen disk and output bounds. The outer `RuntimeMaxSec` is 900 s.
- Root uses the frozen launch form.
- Root completes fresh checks of identity (source 312c0d1d, wrapper 839f6fa0, helper 4dc92714, input 5e8e11b4, ELF ae7e8dab), memory, disk, namespace, environment and exclusive paths.
- The vote entries are the fresh R4 reports (A-1).

That admission does not promise or predict that proving completes. Success, a native refusal, or a wall, CPU or RSS stop are all possible terminal outcomes. Any stop is consumed forever, with no automatic retry. Verify stays gated on actual R4 Success plus the root whole-output result freeze.

**Votes unchanged: A SOURCE = APPROVE_SCOPED; B ACTUAL RESULT = APPROVE_SCOPED; C RESOURCES = APPROVE_SCOPED, with C-1, C-4 and C-5 mandatory for live admission and C-2 and C-3 as accepted disclosed risks. Current live admission: REFUSE_CURRENT_LIVE_ADMISSION.**
