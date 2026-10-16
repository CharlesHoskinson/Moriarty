# DoctorWatson TypeScript sampling performance diagnosis

Scope: source-only, nonsecurity runtime-cost review. No model, test, build, native, or network commands were run by this reviewer. No production or shared model files were edited. The permitted read-only development status command was inspected; historical execution blockers are unrelated to this source diagnosis.

Inspected checkout: `/home/charl/Moriarty/.worktrees/v1-factory-plan-20261016`.
Installed runtime: `/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/`.
Skills applied: `moriarty-dev:develop`, `quint-llm-kit:quint-modeling` review guidance, and `quint-llm-kit:quint-lang`. The report cannot supply fresh runtime pass claims because execution was excluded from this assignment.

## Existing observations

- Receipt `factory-quint-model-build/13-default-sampling-1.json` records timeout at 30.0446 seconds for TypeScript, 10,000 samples, 60 steps, seed 1, workflowInvariant and all 25 witnesses.
- A later receipt appeared during this diagnosis: `15-optimized-default-10000.json` records exit 0, completed, 29.1851 seconds under the same requested settings. Source digest: `a2528112bb8f4524f5b57bd596474dffb67036f0c6c81a661de32a66ad57433c`. Its stdout reports no violation in 27,840ms, 359 traces/second. This leaves little process-time margin and does not quantify any individual cost driver.
- All 10,000 traces reached finalized. Seven witnesses had zero counts: reviewed, rebased, integrated, dependenciesChanged, completed, approachChanged, sourceOnly. These are coverage gaps in that random run, despite the directed scenarios in the test module.

## Runtime source facts and concrete cost mechanisms

1. `runtime/impl/builder.js:416-435` evaluates every argument of a user-defined operator before its body. Thus `canFinish(state, id, fingerprint(state,id))` constructs a fingerprint even when the task is not Candidate or Running. In the three finish-related step branches, a scanned task can incur this allocation three times before any useful action is possible. Fingerprint includes `dependencies.mapBy`, which allocates a map (`builtins.js:603-617`).
2. `builder.js:323-345` caches runtime results only for top-level `val` and `pure val`. Ordinary `pure def` bodies are evaluated on each call. `canFinish` repeats fingerprint computation through its explicit comparison and its call to `fresh`.
3. Logical `and`, `or`, and `implies` already short-circuit (`builtins.js:103-133`); changing all logical blocks into if expressions is not generally an optimization. Cheap guards must precede expensive calls and expensive expressions must not be eager call arguments.
4. `actionAny` shuffles all branch indices then tries branches until one succeeds (`builtins.js:135-175`). When terminal is true every action is disabled, so the last attempted step visits all 22 branches and their filters. With finalized in every captured trace, this unnecessary terminal dispatch occurs in each trace.
5. `exists` and `forall` evaluate their predicate over the whole set before calling JavaScript some/every (`builtins.js:578-583`). A nested `TASKS.exists(dep => canChangeDependencies(...))` therefore evaluates all three dependency candidates, even after one succeeds. Existing editable filtering reduces this cost. The three-task transitive closure is small, but repeated calls still allocate sets.
6. Every witness is evaluated at every successful step (`evaluator.js:206-211`), even after already witnessed in that trace. Most model witnesses are cheap event checks. Receipt histories are normalized when inserted into sets, but they grow only on successful results/layer observations, which the captured random run reaches rarely. This source profile does not support deleting receipt history or claiming it is the dominant driver.

These are source-grounded mechanisms, not CPU-profile measurements. Exact contribution requires an authorized runtime measurement on the final candidate.

## Smallest semantics-preserving changes

Wrap the existing step without changing its branches:

```quint
action step = if (state.terminal) false else {
  // Existing step body, unchanged.
}
```

Every constituent action already requires not terminal, including finalize. Therefore this changes no enabled transition or allowed interleaving; it only rejects an already disabled step earlier.

For the pre-job-token model, change the three finish-related filters and the chosen action argument to the already stored fingerprint:

```quint
TASKS.filter(id => canFinish(state, id, state.tasks.get(id).tested))
// Preserve the Checker/Regressions stage conjunct where present.
checker(id, state.tasks.get(id).tested, observation)
regressions(id, state.tasks.get(id).tested, passed)
timeout(id, state.tasks.get(id).tested)
```

`canFinish(s,id,fingerprint(s,id))` and `canFinish(s,id,s.tasks.get(id).tested)` have the same truth value: both require tested equal the current fingerprint and matching integration base. The second expression allows cheap Candidate/Running guards to prevent fingerprint construction. Consolidate `fresh` within canFinish:

```quint
pure def canFinish(s: State, id: int, stamp: Fingerprint): bool = and {
  not(s.terminal),
  s.tasks.get(id).phase == Candidate,
  s.tasks.get(id).test == Running,
  s.tasks.get(id).base == s.integrationBase,
  s.tasks.get(id).tested == stamp,
  stamp == fingerprint(s, id),
}
```

The author independently reports an attempt-identity defect and plans a JobToken containing stored tested fingerprint and attempt. That correctness change should supersede the fingerprint-only snippets above; apply the same lazy-guard pattern and avoid reconstructing the current fingerprint to create the call argument. This review does not make a security judgment about that defect.

Do not precompute all fingerprints eagerly merely to share them: this can move unnecessary work into every step. A shared top-level state `val` is cached per state but its mapBy still computes every task. Prefer removing eager arguments first.

## Bounded validation plan

The final author candidate needs typecheck, all directed scenarios, and the existing 10,000 x 60 TypeScript invariant/witness check under the unchanged 30-second and memory limits. No backend switch, raised cap, disabled action, or weakened invariant is proposed.

If the full run still times out, preserve its receipt as incomplete. A separately labeled diagnostic run with fewer samples can estimate throughput under the same backend/steps/invariants; it cannot replace the requested full check. Avoid repeated timed-out full runs without a changed hypothesis.

Random terminal success and invariant nonviolation do not establish witness coverage. Keep the main step intact and use test-module scenarios that include the actual production actions to reach the seven missing witnesses. Existing named scenarios cover most of these cases: mergeRebaseRaceTest, transitiveDependencyInvalidationTest, actualLayerObservationsPermitScopedCompletionTest, repeatFailuresRequireReproducerAndApproachTest, and missingRequiredLayersAllowOnlySourceMergeTest. Record each witness explicitly at the relevant intermediate state and check workflowInvariant after each transition where practicable; a final-only invariant assertion can miss a transient violation. A directed-prefix run with the unchanged general step afterward can supply sampled continuations from hard-to-reach legitimate states, but must be labeled conditional exploration rather than sampling from ordinary init.

Acceptance remains limited to the finite DoctorWatson workflow model and the executed traces. This diagnosis supplies no Sherlock interpreter, cross-layer, native, target, or whole-product result.
