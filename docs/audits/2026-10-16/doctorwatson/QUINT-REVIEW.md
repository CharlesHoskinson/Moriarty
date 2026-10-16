# DoctorWatson design review with quint-llm-kit

Scope: the entire documented **delivery workflow**, SF-01–15. Requirements intake,
state/operation separation, executable iterations, structural review, deterministic
scenarios, invariant sampling and reachability follow `quint-modeling` and
`quint-lang`. No Apalache/`quint verify` was run.

The authoritative implementation is the local shell/checker workflow and direct
agent/Git practice. Quint models their intended observable coordination. It does
not replace them with a scheduler or establish implementation refinement.

## Findings and corrections

- **Fixed:** candidate identity alone did not distinguish a late completion from
  an earlier attempt after retry. The failing scenario was captured before the
  fix. `JobToken` now binds candidate and attempt; SF-11 and the design explicitly
  require both. Historical attempts remain distinct.
- **Made explicit:** candidate/base and transitive consumed-input changes invalidate
  current checks and reviews, including profile, oracle, bounds, tool, projection
  and shared inputs. A changed integration base requires revalidation.
- **Retained failure:** the original full sampling run and first actual entrypoint
  timed out. A printed no-violation summary did not override a nonzero process
  exit. Lazy guards, stored job tokens and terminal dispatch reduced runtime;
  sample counts and resource limits were unchanged.
- **Coverage warning:** five rare predicates had zero hits in default random
  sampling. They have positive deterministic scenarios and a separate directed
  walk using the same guarded machine actions. This does not change the random
  counts into hits or constitute exhaustive verification.
- **Fixed after independent review:** upstream edits revived cancelled/blocked
  dependents and cleared their administration history. Separate RED scenarios
  reproduced both issues. The fix preserves terminal disposition and history
  while invalidating candidate evidence.
- **Fixed after independent review:** sampled witnesses alone did not gate
  directed completion. A new deterministic test requires all 47 steps to finish.
  An isolated disabled-step copy still returned 0 from `run`, but the new test
  failed as intended; the committed driver has no disabled step.

## Kit review checklist

| Check | Result and evidence |
|---|---|
| Requirements intake | SF-01–15 coverage maps abstract workflow behavior and concrete Python/shell obligations separately in the model README. |
| Shared-state model | One cohesive authoritative task/state record; no invented message transport. Three tasks/four actors are a finite example, not a factory slot limit. |
| Guards and pure updates | Operations separate eligibility from state update; typed phases/results/tiers replace phase strings. Actions assign state explicitly. |
| Initialization/parameters | Concrete instance supplies constants; deterministic assumption test and actual runs initialize every variable. |
| Step completeness | Main `step` includes all major operations; the directed driver assigns its cursor as well as machine state. No blanket stutter. |
| Useful invariants | Exact review/merge freshness and observed completion constrain current state. Concrete write sets are disjoint by construction; overlap refusal is a hand-mutated test. Local/non-escalation receipt fields are constructed, and oracle/reviewer applicability is trusted input. `noPrematureStop` adds little because cancellation is always available. |
| Failure/recovery | Checker fail-stop, timeout, stale attempts, cancellation, dependency invalidation and both stop rules have scenarios. |
| Reachability | Eighteen of 25 current default witnesses are positive. Seven zero-hit predicates have deterministic coverage; five of those plus directed completion are explicit witnesses in the separate 47-operation walk. |
| Terminal behavior | All 10,000 default traces reached an explicit terminal disposition, mainly cancellation; directed completion is also explicit. Cancellation is not successful delivery. |
| Evidence limits | No fairness, exhaustive search, real dependency discovery, producer authentication, circuit correctness or native/target result follows. |

## Actual local results

The current [entrypoint capture](quint/design-04.json) ran
`./scripts/doctorwatson.sh --design` from `/tmp`, with matching before/after hashes:

- Ordinary corpus/readback check and **33 Python tests** passed.
- Quint typechecking, **30 main scenarios** and **one directed-completion test** passed.
- Default TypeScript sampling: **10,000 samples**, at most **60 steps**, seed **1**;
  no invariant counterexample observed, **26.788 seconds** simulation time.
- Directed TypeScript walk: **one sample**, at most **50 steps**, seed **1**;
  **47 actual guarded operations** from one initialization, no invariant violation,
  all six requested witnesses positive, **64 ms** simulation time.
- Whole entrypoint exit **0**, **35.243 seconds** across five model commands, each
  retaining a 30-second wall cap and a 256 MiB Node old-space limit.

Seven current random witnesses had zero hits: `reviewed`, `rebased`, `integrated`,
`dependenciesChanged`, `layerRecorded`, `completed` and `sourceOnly`. Earlier
design-02/03 had five; those records remain scoped to their predecessor bytes.
The model's cancellation-dominated sampling is incomplete coverage. The directed
walk and scenario assertions establish reachable behavior separately, not random
hits or eventual success. The 30-second cap remains tight on slower hosts and
will fail closed; no limit was raised.

The installed tools were Node 24.21.0 and Quint 0.32.0. No install, Rust backend, Apalache, hosted
CI, SDK or private/native operation was selected. The simulator's printed replay
seed identifies its displayed trace; command seed 1 remains the run input.
See [raw final output](quint/design-04.stdout),
[directed result](quint/directed-result.json), and retained RED/timeout records.

The concrete checker/parser/first-difference fields are tested in Python. Actual
K, Quint interpreter projections, Lean/Clean, handwritten constraints, compiler
preservation, proof verification and Midnight stagenet remain separate mandatory
Sherlock obligations. Layer availability and abstract modeled receipts cannot
substitute for those observations. Every full Moriarty roadmap exit stays open.
