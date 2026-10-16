# DoctorWatson security and quality audit: exact candidate `7c81ebc7…`

## Verdict: **APPROVE_WITH_SCOPE**

I found no blocking defect. The supplied results support each claim made in the final docs, and none of the supplied sources contradicts those results:

- `design-03` exited 0 with matching before/after hashes.
- 31 Python tests and 28 Quint scenarios passed.
- 10,000 default samples reported no violation, with 20 of 25 witnesses positive and 5 zero-hit warnings.
- The directed walk reached all 6 of its witnesses.
- The total run took 34.185 s, with each model command capped individually.

The security boundary holds as written:

- **Local only:** the checker is stdlib Python and observed input is parsed strictly. Frozen inputs are hash-pinned.
- **Bounded tools:** model commands run under a minimal environment with a wall cap and a heap cap. No install, network, native or target step runs.
- **No hosted CI:** the only hosted workflow is `workflow_dispatch`.
- **No native receipts:** the model emits no Native or Target receipt, and NativeLedger availability cannot produce an observation.
- **Owner predicate unchanged:** `ELIGIBLE_REVIEWERS` remains an external, declared assumption.

## Findings (non-blocking; fix at the owner's discretion)

**W1. The `ownership` invariant cannot fail in the concrete instance.**
- **Where:** `models/doctorwatson/doctorwatson.qnt`, `initialTask` (`writes: Set(id)`), `disjoint`, `ownership`.
- **Problem:** each task's write set is `{id}` and never changes. So `disjoint`, the `canEdit` overlap clause and `ownership` are always true in sampling.
- **Effect:** SF-08 conflict refusal is exercised only by `overlapRefusalTest`, which checks the guard against a hand-mutated state.
- **Minimal fix:** give two tasks an overlapping resource, such as T3 `writes: Set(1, 3)`. Alternatively, state in the README that sampled `ownership` is vacuous for this instance.

**W2. `applyEdit` resurrects Cancelled and Blocked dependents.**
- **Where:** `models/doctorwatson/doctorwatson.qnt`, `applyEdit`.
- **Problem:** editing a Merged task resets every transitive dependent to `Queued`, including Cancelled and Blocked ones. `applySharedInput` explicitly preserves those phases, so the two updates are inconsistent.
- **Effect:** a Cancelled disposition is not stable under upstream edits. No invariant breaks, because finalization still requires a fresh disposition.
- **Minimal fix:** add `if (t.phase == Cancelled or t.phase == Blocked) t` inside `applyEdit`, as `applySharedInput` already does. Alternatively, document that resurrection is intended.

**W3. Default random sampling is dominated by cancellation.**
- **Evidence (from `design-03.stdout`):**
  - `cancelled` 99.77%
  - `finalized` 100%
  - `testPassed` 2 traces
  - `reviewed` 1 trace
  - `integrated` 0 traces
- **Effect:** `exactMerges`, `observedCompletion` and post-merge `exactReviews` were checked only by the 28 deterministic scenarios and the single directed trace. They were not checked in the 10,000 random traces.
- **Docs:** they already report the zero counts honestly.
- **Minimal fix (optional):** add a separate sampler profile without `cancel`, or a fairness-biased step, as a further `run`. Leave the existing counts unchanged.

**W4. The directed command does not fail when reachability is lost.**
- **Where:** `scripts/doctorwatson.sh`, lines ~58–64.
- **Problem:** `quint run --witnesses` exits 0 when a witness has zero hits. If a guard regression disabled a cursor step, the walk would stop early and the entrypoint would still exit 0.
- **Current state:**
  - Reachability is fail-stop today only through the deterministic `.then` chains in `quint test`, which fail when an action is disabled. These are `mergeRebaseRaceTest`, `transitiveDependencyInvalidationTest`, `actualLayerObservationsPermitScopedCompletionTest` and `missingRequiredLayersAllowOnlySourceMergeTest`.
  - `directedFinished` is established only by the captured stdout of `design-03`.
- **Minimal fix:** add a deterministic `run` test that chains the same 47 actions and ends with `.expect(state.terminal)`. Alternatively, document that the fourth command reports reachability but does not gate on it.

**W5. Several invariant conjuncts are true by construction.**
- **Where:** `localOnly`, `noEscalation`, `noPrematureStop`, and `independentOracle` under the given constants.
- **Why:**
  - Every receipt literal is `local: true` with tier Ordinary or Correspondence.
  - Any non-final phase is cancelable.
  - The oracle owner (3) never owns a task.
- **Effect:** the README is accurate ("no transition emits those receipts"). These conjuncts should not be cited as observed evidence for SF-05 or SF-10.

**W6. The serializer is executed but not pinned.**
- **Where:** `models/sherlock/qa/check.py`, `check_rust_transcription`.
- **Problem:** `check-reference.py` is executed and its hash reported (`83acde…`, which matches the supplied context), but a changed serializer does not fail the check. A coordinated change to the serializer and the `.rs` files would still pass. This is within the documented trusted-checkout boundary and is caught only by SF-11 review.
- **Minimal fix:** add the serializer's SHA-256 to the pinned set and `require` it before `exec_module`.

**W7. A diagnostic overlay can select the wrong state.**
- **Where:** `models/sherlock/qa/check.py`, `observation_state`.
- **Problem:** `trace['states'][row[1]]` and `publication.pre_state_index` accept negative or boolean indices from the observed document. Python then silently picks the wrong state for `observed_state`.
- **Effect:** diagnostics only; the status is already a mismatch.
- **Minimal fix:** require a nonnegative `type(...) is int` index before indexing, otherwise return `None`.

**W8. Agent instructions conflict.**
- **Where:**
  - `AGENTS.md`, "Required startup" step 3, prescribes `run --action <ACTION_ID>` for campaign dispatch. The top current section prohibits "campaign dispatch".
  - `ROADMAP.md`, "Evidence and internal workflow", still says "Foreman/Pel governs project implementation only."
- **Minimal fix:** add one line to each noting that the October 16 override suspends Foreman and campaign dispatch for current DoctorWatson work.

**W9. Two records are stale.**
- `models/doctorwatson/README.md` still calls `design-02` the "Final local entrypoint capture", while a later paragraph names `design-03` as final.
- `docs/audits/2026-10-16/doctorwatson/README.md`, "Workflow design check", indexes only `design-02` (three commands). It does not link `design-03` or `directed-result.json`.
- **Minimal fix:** relabel `design-02` as the predecessor and add the `design-03` link.

**W10. Minor robustness notes.**
- **Relative overrides:** `doctorwatson.sh` runs `cd` (line 15) before resolving tools (lines 19–30). Relative `DOCTORWATSON_NODE` and `DOCTORWATSON_QUINT` values therefore resolve against the checkout root, not the caller's directory. This should be documented.
- **Timeout headroom:** default sampling took 27.55 s against a 30 s cap. This fails closed, but it will be flaky on slower hosts. Record this; do not raise the limit.
- **Timeout exit code:** with `--signal=KILL`, GNU `timeout` exits 137, not 124. Confirm whether the `design-01` exit 124 came from an outer wrapper or an earlier script variant. Future timeouts of this script will report 137.

## Tested scope (supplied sources and results)

- **Script control flow:**
  - argument handling (exit 2 on unknown arguments);
  - tool preflight (exit 127 before any check);
  - fail-stop through `set -e`;
  - minimal child environment;
  - per-command wall and heap caps;
  - no fallback.
- **Checker:**
  - strict observed parsing: duplicate members, nonfinite numbers, deep nesting;
  - exact-type structural diff;
  - identity checked before state;
  - absent or partial observations refused;
  - byte-exact readback;
  - exit codes 0/1/2.
- **Model and coverage:**
  - JobToken binding of candidate plus attempt;
  - edit, rebase and merge invalidation;
  - transitive dependents and cycle refusal;
  - review freshness and author refusal;
  - availability kept distinct from observation;
  - NativeLedger exclusion;
  - finite finalization.
  - The witness, sample, step and timing figures above match the captured output.

## Unverified or out of scope

- **Captured artifacts:** the `design-01`/`design-02` contents, the gzipped RED sources, `performance-review.md`, the `.rs` byte contents (only their reported hashes), and the Node version (absent from `design-03.json`).
- **Implementation fidelity:** the model does not show that real agents, Git, the shell or reviewers implement it. Layer observations are abstract trusted receipts.
- **Gaps that stay open:** dependency-registry completeness, the OS sandbox and hermeticity, and the 01–09 snapshot gap (disclosed).
- **Sherlock correctness layers:** none were executed. This covers K, the Sherlock Quint projections, Lean/Clean, the circuits, native work and stagenet.
- **Exits:** no U/M/MC/SP/G exit closes.
