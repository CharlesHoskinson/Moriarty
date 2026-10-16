# DoctorWatson performance repair, October 16

Scope: only `models/doctorwatson/doctorwatson.qnt` in checkout `/home/charl/Moriarty/.worktrees/sherlock-runtime-observations-20261016` was edited. No scripts, docs, tests, bounds, witness expressions, invariant expressions, resources, backends, or production implementation were changed. The existing N1 preservation of administration counters and disposed task history remains intact.

Baseline SHA256: `5f69fcb361159822de48a154f831b546e37612fef66c8387409f177e801b6531`.
Candidate SHA256: `788eed642b74654b489a6081b48470a8e660dab7f6d81d51c26ce0082ceacf4d`.

## Diagnosis

The earlier design-01 failure remains preserved. It reached the unchanged 10,000-sample TypeScript workflow run and timed out; no unchanged full retry was performed for this repair.

The installed TypeScript evaluator evaluates every user-operator argument before its guard (`runtime/impl/builder.js`, normal eager application), and runtime value caching applies to `val`/`pure val`, rather than ordinary `def`/`pure def` (`buildDefWithMemo`). The three finish-related sampler branches therefore constructed a JobToken for every task before checking Candidate/Running, and repeated that scan independently. Immutable initial state and empty evidence records were also rebuilt on every use because they were zero-argument pure defs.

A 1,000-sample diagnostic used exactly the installed Node/TypeScript backend, existing 60 steps, seed 1, workflowInvariant, all 25 witnesses, minimal environment, 30-second timeout and 256-MiB old-space setting. Node CPU profiling was enabled identically for baseline and candidate; profiles are saved here. The baseline profile prominently attributes samples to garbage collection and generic eager operator argument evaluation. This is sufficient to identify allocation work but does not assign a measured percentage to an individual Quint expression.

## Repair and transition preservation

- Converted the immutable zero-argument definitions admittedLayers, emptyFingerprint, emptyReview and initialState to pure val. Their expressions are unchanged; the evaluator can reuse these immutable values per module instance.
- Added canFinishCurrent(s,id), exactly equivalent to canFinish(s,id,jobToken(s,id)): token equality checks in the latter are reflexive for that token, leaving phase/test/base/current-fingerprint checks. Existing public canFinish and completion actions retain their full supplied-token checks.
- Added a scoped lazy finishable set, shared by Checker, Regressions and Timeout branches. Each branch retains exactly the same eligible IDs, stage filters, nondeterministic choices and action invocation. No sampler branch or allowed transition was removed. No random draw was added or removed.

The 1,000-sample diagnostic produced identical witness counts and identical final RNG state before/after, providing bounded execution evidence alongside the source equivalence argument.

## Actual checks

Every check below completed with exit 0 on the candidate hash above, using the installed Node, TypeScript where applicable, minimal environment, unchanged 30-second timeout and 256-MiB old-space setting.

| Check | Result | Process wall time |
| --- | --- | --- |
| Model typecheck | Passed | 2.067 s |
| Existing deterministic suite | 34 passing | 2.792 s |
| Directed 47-step completion assertion | 1 passing | 2.281 s |
| Directed invariant/witness run | No violation; all six witnesses 1/1, including directedFinished | 2.337 s |
| Separate 1,000-sample diagnostic | No violation; unchanged witness counts and final RNG | 5.228 s |

The profiled diagnostic sampling portion improved from 3,772 ms to 3,128 ms (17.1%); total process time improved from 5.991 s to 5.228 s. This is a diagnostic comparison, not the requested 10,000-sample acceptance check and not a guarantee about its wall time. No full 10,000 run was performed, pending Root's signal.

Receipts 01 through 06 contain exact commands, source hashes, process exits, elapsed times, stdout and stderr. baseline.qnt and candidate.qnt retain exact source snapshots. Existing design-01 and stop-counter failure histories were not modified.

Next check: Root may run the unchanged design command against this exact candidate and retain the actual result. The finite workflow model supplies no Sherlock interpreter, SDK, native, target or whole-product acceptance result.
