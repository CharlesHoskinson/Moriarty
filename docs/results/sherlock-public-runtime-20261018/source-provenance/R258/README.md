# R258 public draft B execution source proposal

The capability is execution of actual dedicated draft source through the fixed B
owner, after the existing bounded parser/checker/copy/admission pipeline. The
public entry is
`mori_core::sherlock_compile_artifact::draft_execution::run_draft_host`.
`PUBLIC-API.json` supplies its complete signature and returned types.

The new leaf prepares exactly one fresh `AdmittedDraftSource`. It first projects
that pair into the owned, unstepped `DraftCompileArtifact`, then moves the same
admitted/image fields into `OwnerEngine::run`. No compile-only call, second
admission, A evaluator, program-name dispatch, evaluated result literal or
precomputed outcome is involved. The fixed owner constructor supplies the
existing engine; callers cannot inject a FamilyPlans or guard implementation.

`DraftHostRun` exposes the preparation snapshot and the existing actual
`CompletedRun`. Its `observe()` and `resource_report()` use the existing bound
owner observation. The preparation artifact keeps its truthful genesis-only
fields and all26 named-unproved construct correspondence obligations. Actual
post-run state and summary belong to `completed`, rather than overwriting the
unstepped snapshot. `Ok` includes actual Rejected/Exhausted completion; callers
must inspect the real outcome before claiming a successful application value.

Events are forwarded once, whole and in owner order, using the existing
security-owned `EventForwarding`. A real run HostViolation maps through the
existing `ConsumerError::Run` and `CompileFailure::consumer`, preserving the
sink's partial prefix count with evaluation_performed=true/completed=false.
Later observation refusal preserves the actual summary and count through the
existing `ConsumerError::Observation`. A post-run `failure()` helper preserves
completed-run metadata on a later publication error. Sink panic behavior is
the existing scalar consumer behavior; no catch/retry policy is added.

The source patch has three files: the new execution leaf, two-line module
registration against the R254 parent overlay, and a visibility-only re-export
of `EventForwarding` from `core_action_consumer`. The original private import
is removed to avoid a duplicate name; the security module, counter and all
guards/limits are byte unchanged. Root authorized this ordinary visibility
seam; it is not security approval. The changed public API belongs in the fresh
full-source audit before any execution.

No tests, runtime fixtures or expected outcomes were authored by this lane.
Only the frozen public types/signature were sent to the independent R259 oracle
author; their populated expectations remain unread here. No candidate/source,
compiler, model, native, provider or Git execution was performed. All source is
**UNRUN / REVIEW PENDING**. H1, owner/report allocations, source-AST cleanup and
caller-owned event retention remain explicit premises. No new guard, policy,
budget, default, native/CLI/export interface or compile-only behavior changes.

Root alone composes R254/R258, obtains the fresh designated review, selects any
ordinary operation and adopts any observed capability. Earlier compile-only
results do not establish this runtime path. `APPLY-RECIPE.json`, exact preimages
and afterimages, split integration patches and `STATIC-VALIDATION.json` support
that handoff without a runner or factory.
