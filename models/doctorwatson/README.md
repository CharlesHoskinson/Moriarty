# DoctorWatson workflow model

This Quint model checks the abstract delivery workflow in
[DOCTORWATSON.md](../../docs/DOCTORWATSON.md), its
[local CI contract](../../docs/DOCTORWATSON-CI.md), and
[SF-01–15](../../openspec/changes/doctorwatson/requirements.md).
It models authoritative task/candidate records shared by workers and the
integrator. It is a design model, not an orchestrator or message protocol.

Run the inspected local entrypoint with existing Node and Quint installations:

```sh
./scripts/doctorwatson.sh --design
```

The default command without `--design` runs only Python ordinary QA. The optional
mode explicitly typechecks this model, runs deterministic scenarios with the
TypeScript backend, samples `doctorwatson_small` with seed 1, 10,000 samples
and 60 steps. A fourth command tests directed completion; a fifth runs the
directed coverage walk described below.
Each model command has a 30-second timeout and 256 MiB Node heap.
The entrypoint records a failing command as a failure; it has no smaller-run,
Rust backend, download, hosted CI, Apalache or native fallback. Consult the
entrypoint for the exact installed-tool discovery and complete witness arguments.

## State, operations and assumptions

`doctorwatson` is parameterized by task/actor sets, integrator and an externally
supplied applicable-review-owner predicate. `doctorwatson_small` supplies all
constants: three tasks, four opaque actors, two independent leaf write sets and
one dependent shared-interface write set owned by the integrator. These are
finite example participants, not a factory WIP or worker limit. Actors coordinate
through shared records; transport/provider behavior is outside the model.

Each task records phase, ownership, write set, dependencies, required layers,
independently frozen oracle, candidate version and integration base, local job
stage/result/token, current review and actual abstract layer observations. Shared
state retains historical result/review receipts, integration base, availability,
source merges and fully observed scoped slices. Every transition has a pure
guard and pure update; actions assign the one cohesive state record. No blanket
stutter is present. Finalization is finite and explicit, after all tasks have a
merged, cancelled or blocked disposition. It need not signify successful delivery.

The workflow includes claiming, independent oracle freeze, implementation edit,
submission, checker/regression stages, failure/timeout, review approval/refusal,
rebase, merge, dependency mutation, shared-input mutation, layer availability and
observations, scoped completion, repeat-failure diagnosis, process stops,
cancellation, blocker declaration and finalization. All operations occur in
`step`; deterministic tests do not supply otherwise missing operational behavior.
Enabled choices are selected inside branches. A randomly selected disabled owner
cannot truncate a trace while another task has an enabled operation.

A fingerprint binds task ID, candidate version, integration base and dependency
versions. A task version abstracts its exact consumed source/profile/oracle/
bounds/tool/projection/command/import membership, not a semantic hash computation.
`edit` invalidates that task and its transitive declared consumers. Actual graph
changes use `changeDependencies`, which refuses cycles. `sharedInput` represents
a changed profile/oracle/bounds/tool/projection consumed by **every** modeled task
and invalidates all consumers. Declaring a real dependency registry completely
is an external obligation; this small graph cannot discover undeclared consumers.

A merge atomically rechecks current local results, applicable independent review,
base and dependencies. It advances the integration base and clears other active
candidates' current observations/reviews. Rebase produces a new candidate that
needs fresh checks and review. Historical receipts retain their original task,
version, base, dependency versions and attempt ID. An explicit JobToken binds both
fingerprint and attempt, so a late result from a timed-out retry cannot complete
the new attempt even when candidate bytes are unchanged. A rejected review needs
a changed candidate before approval; it cannot be silently relabeled approved.

Oracle independence, actual checker classification, actual component observations,
review applicability and successful reproduction/changed approach are abstract
inputs with explicit guards. They are not certified by actor identity or a model
label. The owner predicate preserves the existing external security-owner policy;
this model neither defines that policy nor assesses security/circuit correctness.
Failures are observable terminal job results, not process crashes. Cancellation
and blocker declarations release active writes while retaining history. There is
no fairness or eventual-success claim: owners may cancel or keep changing inputs.

## Requirement coverage

| Requirement | Modeled predicate/operation and deterministic evidence | Concrete boundary |
| --- | --- | --- |
| SF-01 | `startLocal`, `checker`, `regressions`, `timeout`; checker rejection cannot enable regressions | Abstract command stages; real shell command/exit behavior has separate Python/shell checks |
| SF-02 | `Observation.IdentityMismatch`/`Incomplete`; oracle/source version binding | Full corpus fields and exact membership are validated by the existing Python consumer, not reimplemented in Quint |
| SF-03 | `StateMismatch` produces failed local result; `firstMismatchFailureTest` | First JSON path/raw-value reporting is concrete Python coverage, abstracted as a mismatch here |
| SF-04 | `incompleteFailureTest`; absent/incomplete observations cannot produce pass | Actual input parser and missing-file behavior remain Python checks |
| SF-05 | Ordinary receipts have `Ordinary` tier and local provenance; `noEscalation` | Static checker import/call closure is source inspection, not verified by workflow simulation |
| SF-06 | `freeze`, `independentOracle`, `oracleIndependenceTest`; semantic-input changes invalidate the freeze | No expected values are generated; actual independent oracle derivation stays external |
| SF-07 | Required/available/observed layer sets remain distinct; missing-required-layer scenario | Layer status is abstract; no K/Lean/Clean/circuit execution or clock correspondence is performed |
| SF-08 | `canClaim`, `disjoint`, dependency readiness, integrator guard; parallel and overlap/refusal scenarios | Write paths are opaque resource IDs; complete real path/dependency inventory stays external |
| SF-09 | `canReview`, `canMerge`, `fresh`, `exactReviews`, `exactMerges`; author refusal and merge/rebase race | No Git/GitHub or provider APIs are modeled |
| SF-10 | Separate source merges, scoped full observations and evidence tiers; `noEscalation`, `observedCompletion` | All product U/M/MC/SP/G exits remain outside and unclosed |
| SF-11 | `affected`, `applyEdit`, `sharedInput`, `changeDependencies`, rebase; transitive chain/shared-input/stale-running scenarios | Version equality abstracts exact consumed bytes; it does not authenticate them |
| SF-12 | Separate Failed/TimedOut, checker fail-stop, durable per-attempt receipts; late-attempt token refusal | No wall-clock scheduler/process-kill fidelity is claimed |
| SF-13 | `cancel`, `nameBlocker`, `active` ownership; cancellation/release and blocked-dependent scenarios | No physical worker/process cancellation is executed |
| SF-14 | Two repeated failures require reproduction+approach change; two process cycles/30 minutes stop administration | One abstract defect class per task; time is recorded administration minutes, not elapsed physical time |
| SF-15 | Availability does not add observed layers; `complete` requires current scoped observations after source merge | Successful layer receipts are assumptions about imported real evidence, not proofs derived from tool availability |

`workflowInvariant` combines ownership, independent oracle, independent exact
review, exact merge evidence, local provenance, non-escalation, observed scoped
completion and explicit ability to progress or finalize. A scoped completion
means all layers required by this example task have actual abstract observations.
The `Correspondence` tier name is an evidence category, not a refinement theorem.
Native/target tier values are represented but no transition emits those receipts;
NativeLedger availability cannot pass its excluded observation guard. Native
authority remains zero. No full V1, financial, compiler, proof or target acceptance
is established by this workflow model.

Several properties are restricted by construction in this concrete instance.
Its three write sets never overlap, so sampled `ownership` cannot fail; only the
hand-mutated overlap/refusal scenario exercises the conflict guard. `localOnly`
and `noEscalation` restrict constructed receipts, rather than testing real tool
provenance. Oracle and reviewer applicability are external trusted inputs.
`noPrematureStop` is weak because every nonfinal phase permits cancellation;
it establishes no useful eventual-success guarantee. Cancellation dominates the
default sampling, so merge/completion evidence comes from the deterministic
scenarios and directed walk, with the random zero counts retained below.

## Observations and retained failures

The current deterministic suite has 30 passing scenarios, including all three tasks
through source merge and explicit finalization, parallel work, failed checks,
timeouts, rejected/self reviews, stale source/base/dependency/shared-input
receipts, late prior-attempt completion, required-layer gaps, scoped layer
observations, cancellation and both stop rules. Mid-trace refusal checks inspect
the same pure action guards; an additional final disabled-action test exercises
Quint's `fail()` operator. No no-op transition was added to make those tests run.

A meaningful model defect was reproduced first: after timeout and restart on the
same candidate, the original fingerprint-only completion token accepted the old
attempt's result. `16-late-attempt-RED` retains the failing deterministic scenario;
JobToken attempt binding fixed it, and `19-terminal-guard-fix` records the 28-test
GREEN closure. Earlier syntax/import/effect/test-harness failures are also retained.

A later independently reviewed workflow defect was reproduced in
`22-terminal-dependent-RED`: editing a merged upstream task changed cancelled
and blocked dependents back to Queued. `applyEdit` now preserves those terminal
dispositions while invalidating their current results, reviews, frozen inputs
and version; historical receipts remain. Both new regression scenarios first passed in
`23-terminal-and-coverage-GREEN`, alongside all earlier scenarios (30 total).
A strengthened blocked-dependent regression then reproduced loss of its recorded
30 administration minutes/one process cycle (`25-terminal-history-RED`). The
terminal history counters and diagnosis flags now remain while current candidate
evidence is invalidated; active-task reset behavior is unchanged. The current
`26-terminal-history-GREEN` closure passes all 30 scenarios, the deterministic
coverage completion test, both typechecks and a fresh one-sample main run.
This enforces the existing cancellation/blocker disposition rather than creating
an automatic reactivation policy.

The new [deterministic directed test](doctorwatson_coverage_test.qnt) requires
exactly 47 driver steps followed by `directedFinished`; an early disabled step is
therefore a nonzero test failure. Its one scenario passes. An isolated runtime
copy with cursor 17's rebase guard disabled reproduced why this is necessary:
sampled `quint run --witnesses` still exited 0 with zero terminal/rebase hits,
whereas the deterministic test exited 1 (QNT513, repetition could not continue).
`24-disabled-driver-proof-*` retains this negative control and source snapshot;
no guard was disabled in the committed driver or main machine.

Historical local entrypoint capture `design-02` passed on unchanged model/test bytes:
31 Python tests, 28 deterministic Quint scenarios and 10,000 default traces at
60 steps, seed 1. No invariant counterexample was observed; the simulation took
25.538 seconds within its 30-second command limit. Every sampled trace reached
an explicit terminal disposition, primarily cancellation. **Five stochastic
witnesses had zero hits:** rebased, integrated, dependenciesChanged, completed and
sourceOnly. The other 20 witnesses were positive. These zero counts are sampling
coverage warnings, not evidence that those paths are unreachable; deterministic
scenarios positively reach them. This is not an all-action stochastic coverage
pass or exhaustive verification. The prior candidate's 10,000-trace capture had
seven zero witnesses and remains separately scoped to its own recorded bytes.

A separate [directed coverage walk](doctorwatson_coverage.qnt) now reaches all
five rare predicates, each in one trace, plus `directedFinished`. It imports the
same concrete machine under the `machine` namespace and adds only a cursor.
Initialization runs once. Each of its 47 steps calls exactly one existing guarded
machine action and advances the cursor once; it reaches the actual machine's
terminal disposition and then has no enabled driver step. It never resets the
machine mid-walk, overrides a guard, or changes the main sampler's transitions.

The walk prepares and reviews both leaves, merges T1, rebases/rechecks/reviews and
merges stale T2, then prepares T3, records all four required layer observations,
merges and completes T3. It finally reopens T1, adds merged T2 as its dependency,
cancels T1, records T3's dependent blocker and finalizes. Thus rebase, integration,
dependency change, scoped completion and source-only integration all occur as
actual atomic operations from the original initial state.

The updated `./scripts/doctorwatson.sh --design` includes a deterministic
coverage test before the directed run, as its fourth and fifth model commands. Its reachability evidence remains separate from the
default 10,000-trace result. With the existing Quint 0.32.0 CLI, its commands are:

```sh
quint typecheck models/doctorwatson/doctorwatson_coverage_test.qnt
quint test models/doctorwatson/doctorwatson_coverage_test.qnt \
  --main=doctorwatson_coverage_test --backend=typescript --max-samples=1 --seed=1
quint run models/doctorwatson/doctorwatson_coverage.qnt \
  --main=doctorwatson_coverage --backend=typescript \
  --max-samples=1 --max-steps=50 --seed=1 --verbosity=1 \
  --invariant=directedInvariant \
  --witnesses rebased integrated dependenciesChanged completed sourceOnly directedFinished
```

The captured execution used the same inspected absolute Node/Quint paths,
256 MiB heap, minimal environment and 30-second timeout as the other model
commands. Typecheck and the directed run both exited 0; simulation took 63 ms.
`21-directed-coverage-*` retains exact argv, source snapshots/hashes and outputs.
That directed capture used the main/test hashes of passing `design-02`; it did
not rerun or relabel default sampling. The terminal-disposition correction below
has its own changed source binding. The five zero counts remain warnings on the default random
sampling even though this directed reachable walk establishes non-vacuity.

The first full sampling attempt timed out at 30 seconds (`13-default-sampling-1`),
then cheaper guarded dispatch completed the next attempt in 27.840 simulation
seconds (`15-optimized-default-10000`). A separate entrypoint capture still timed
out and remains a timeout even though its stdout printed the sampled summary.
The JobToken correction also removes eager fingerprint construction for disabled
jobs and places the terminal guard before dispatch. The fresh `design-02` entrypoint result above binds that changed candidate;
prior observations do not transfer.

Runtime records live under
`/home/charl/Moriarty/.moriarty-dev/runtime/v1-factory-planning-20261016/`:
`factory-quint-model-build/` retains exact argv, outputs, statuses and source hashes;
`factory-local-design-entrypoint/` captures actual entrypoint runs. Full source
snapshots accompany attempts 10 onward. Attempts 01–09 retain hashes/output only,
so their original source bytes cannot be recovered from those receipts alone.
No attempt or terminal outcome was erased or relabeled.

Update this model and its independent scenarios before changing the modeled
workflow. Re-run the affected source closure after candidate/dependency/tool
changes. Passing finite scenarios and sampled invariants does not prove the
Python consumer, shell, agents, Git, external reviewers or private/native tools
implement this model. Serialization, authentication, semantic/circuit validity,
provider failures, resource budgets and product behavior remain separate checks.

Historical integrated `design-03` includes all four model commands, exits 0 and binds
its recorded three Quint source hashes before the terminal-disposition correction. It passes 31 Python tests and 28 Quint
scenarios; 10,000 default samples report no violation in 27.552 seconds, with the
same five zero-hit warnings; the directed sample reaches all six witnesses in
62 ms. Total entrypoint time is 34.185 seconds across individually bounded
commands. See the [public kit review](../../docs/audits/2026-10-16/doctorwatson/QUINT-REVIEW.md)
and [final capture](../../docs/audits/2026-10-16/doctorwatson/quint/design-03.json).

Current integrated `design-04` binds the corrected model, tests, script and Python
checker. It passed 33 Python tests, 30 main scenarios and the directed-completion
test, exiting 0 in 35.243 seconds across five model commands. Default sampling
observed no invariant violation in 26.788 seconds; **seven witnesses had zero
hits**: reviewed, rebased, integrated, dependenciesChanged, layerRecorded,
completed and sourceOnly. The other 18 were positive. The separate directed
walk reached its six requested witnesses in 64 ms. Deterministic scenarios
exercise review/layer recording as well; those are not random hits.
See the [current capture](../../docs/audits/2026-10-16/doctorwatson/quint/design-04.json).
