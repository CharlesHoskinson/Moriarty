# Local factory EARS and QA

These developer workflow requirements implement UIREQ-021–026 and support
B-T01/B-T02/B-T11. SF identifiers identify this increment, not new product exits.
Their scope is the existing independent `sherlock-k-a/1` corpus and its literal
transcription. Current implementation coverage and observed checks are recorded
in the [local QA documentation](../../../docs/DOCTORWATSON-CI.md).

| ID | EARS requirement | Positive and refusal QA |
|---|---|---|
| SF-01 | WHEN local CI is invoked, it SHALL run the inspected ordinary corpus checker and regressions locally and propagate a failing exit. | Run the real command; compare its direct exit with a failing supplied observation. No network installs or hosted job. |
| SF-02 | WHEN the checker loads the independent corpus, it SHALL validate case identity, profile, source binding and required complete state/work/publication fields. | Existing frozen corpus passes its structural contract; missing identity or fields fail. |
| SF-03 | WHEN supplied observations differ from expectations, the checker SHALL return a nonzero exit and identify the first differing path with expected and actual values. | Complete matching documents agree; changed state, order, work and outcome differ. |
| SF-04 | WHEN an observation family is supplied incompletely, the checker SHALL reject the incomplete comparison rather than treating absence as agreement. | Supply both documents; supply only one and expect failure. |
| SF-05 | WHILE the ordinary checker runs, it SHALL compare existing data/source literals without invoking the evaluator, model engines, compiler, SDK, proof or target consumers. | Inspect its complete import/call closure; report source/static observations only. |
| SF-06 | WHEN a source literal readback is checked, the checker SHALL compare it against independently frozen expectations without replacing those expectations. | Existing transcription agrees; a changed literal fails. |
| SF-07 | WHEN a transition slice changes, its owner SHALL identify affected K, Quint, Lean/Clean and handwritten-circuit projections and record each unavailable or unrun layer as a gap. | Required correctness-loop matrix names all layers; ordinary CI cannot mark them passed. |
| SF-08 | WHEN parallel work starts, assignments SHALL name disjoint write sets, real consumers, independent expectation owners and dependencies; one integrator SHALL own shared interfaces. | Current checker and local CI lanes have separate paths; a shared-interface change goes to the integrator. |
| SF-09 | WHEN a PR requests integration, a fresh applicable non-author review SHALL assess the full candidate and actual scoped results; changed composition SHALL refresh affected checks/review. | Candidate/result audit names exact bytes and limitations; previous candidate approval is not transferred. |
| SF-10 | WHEN progress is reported, the report SHALL distinguish source implementation, observed QA, correspondence, native and target acceptance and retain all original roadmap exits. | A local static pass leaves model, circuit, native and stagenet rows open. |
| SF-11 | WHEN candidate source, a consumed dependency, profile, oracle, bounds, tool or projection changes, affected current results and reviews SHALL become stale transitively; their historical records SHALL remain. Each result SHALL bind both candidate and attempt identity. | Edit during a running check; rebase after another merge; mutate a consumed dependency; refuse stale evidence and late results from an earlier attempt on unchanged source. |
| SF-12 | WHEN a local check fails or times out, the workflow SHALL retain that outcome and stop dependent check stages until repaired and rerun on the current candidate. | A failed checker cannot start regressions; timeout is distinct from pass; an earlier failure survives a later repair. |
| SF-13 | WHEN work is cancelled or explicitly blocked, it SHALL release active write ownership and retain its failure/resource history. | Another eligible writer can proceed after release; dependent acceptance remains blocked. |
| SF-14 | WHEN two attempts repeat the same defect class, the owner SHALL reduce the reproducer and change approach before another broad attempt; WHEN two process-only cycles or thirty administration minutes accumulate, the owner SHALL perform eligible work or record the concrete blocker and owner. | Refuse a third blind retry or more administration; witness a reduced reproducer and changed approach, or an explicit blocker. |
| SF-15 | WHEN a tool or layer becomes available, availability SHALL NOT create a passing observation or complete the feature; source integration and full behavior/acceptance SHALL remain separate. | Make all layers available without running them and keep full completion false; record actual current results before claiming the corresponding tier. |

No requirement grants native authority, changes a resource/counter/C4 condition,
selects new semantics or authors substitute security tests. Security work stays
with its designated owner. Complete structured observation equality is not
semantic execution, proof verification or source authenticity.

The [DoctorWatson workflow model](../../../models/doctorwatson/README.md) checks
the abstract delivery transitions. The Python checker regressions separately
exercise concrete SF-01–06 behavior. Workflow simulation neither replays the
interpreter nor proves the shell, agents, Git or private tools implement the model.

The explicit `--host` consumer exercises SF-01/02/03/06/09/11/12/15 through an
actual Sherlock build and run. Its bounded-stepping feature supports the existing
Sherlock finite-step and observation requirements; the
[independent cycle contract](../../../docs/audits/2026-10-16/doctorwatson-delivery/acceptance/CONTRACT.md)
requires every admitted post-startup field and all 216 fuel prefixes. Actual
compiler/test/execution/comparison receipts and ordinary CLI/guard regressions
are concrete evidence; the workflow model abstracts these external command
results and does not prove wrapper refinement. Constructor startup, the other
semantic layers, native and target qualification remain explicit separate gaps.
