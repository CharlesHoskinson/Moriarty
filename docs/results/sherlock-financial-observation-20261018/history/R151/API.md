# Sherlock Core/2 financial observation API proposal

Source-only candidate, unregistered/uncompiled/unrun. Root owns the single module
registration proposal. No source or result acceptance, financial lowering, native,
ledger phase or authenticity claim follows from this module.

Module: `mori_core::sherlock_financial_projection`.

```rust
pub fn project_invocation<'a>(
    program: &'a stage::StagedProgram,
    input: InvocationInput<'a>,
    result: &'a Result<stage::StageOutcome,stage::StageRejection>,
) -> InvocationObservation<'a>;
pub fn account_effects(effects: &[Effect]) -> AccountingReport<'_>;
```

The caller captures the actual arguments and returned result of existing
stage::start/resume, then borrows them for project_invocation. This call executes
no evaluator. Its `.program` retains the actual complete StagedProgram, including
existing Program/StagePlan/hash getters. `.input` retains complete start scenario/
action/cap or resume continuation/event/CLI flags. `.result` borrows the complete
original Result, and `.outcome` projects every current result field. The API does
not validate that caller-supplied inputs produced that result or passed admission.
Existing stage::STAGE_LABELS remain the actual consumer's publication labels.

Complete Snapshot values preserve unrelated state/balances and all liability
rows, including present zero. Positive residual duties stay separate. Successful
segments preserve complete ordered Steps/statements/values/snapshots/effects and
pre/post pending frames. Waiting preserves its complete continuation and attempted
work; Complete preserves its segment, attempted and committed work. Rejected
preserves original Error and both counters, with no returned boundary invented.
Source logical work remains distinct from A/B fuel, accounting amounts and fees.

`.accounting` is PublishedSegment(report) only when a real successful segment is
published. NoPublishedSegment and RejectedNoPublishedDelta are unavailable reports,
not zero money or empty successful effects. A genuinely published empty segment
has a report with an empty original effect slice and empty account rows.

Rows group by exact existing AssetId and actor string, in first effect encounter
order, from then to. `gross_debit` includes transfer and fee amounts exactly once.
Transfer debit/receipts and fee debit/receipts remain independent subsets. Gross
receipts include both categories. NetFlow is the signed difference of those exact
gross receipts and debits, never a refund of prior gross debit. Original complete
Effect rows remain ordered. Zero rows and self-transfers are not erased. No supply
Effect variant exists on this Core carrier; no mint/burn or native fee is invented.

Each bucket reports Exact(UInt) or AboveU128 via checked addition. NetFlow is
Unavailable when either gross bucket is AboveU128. It does not clamp or assert a
net value after unknown larger-number cancellation. These are observation metadata,
not runtime errors, outcomes, cost planes, guards, admission or new caps. Aggregate
scope is one successful segment; later segment totals are not claimed cumulative.
Continuation alone contains no cumulative prior effect history.

`.tentative_publication` distinguishes the existing published successful segment,
no advanced segment, and UnavailableFromStageRejection. runtime::run_segment keeps
its tentative current/steps/effects local and returns only Error on failure;
StageRejection publishes only error/attempted/committed. Thus an exact failed
tentative snapshot/effect list is unavailable from this carrier. Captured input
continuation remains separately accessible and is not a newly returned rollback
continuation. Original UIREQ-019 remains partial until the actual required carrier
and existing designated-owner scenario evidence exist. No trigger was invented.

The old unregistered financial_projection source supplies the existing complete
borrowed projection functions unchanged. New project_invocation adds the previously
omitted actual StagedProgram/input/raw-result context. Legacy project_resume_result
still omits program context by definition; prefer project_invocation for complete
invocation capture. No second financial executor is introduced.

Independent input/declaration freezes01/02 precede accounting body construction.
R152 receives only those body-free declarations and exact positive source/scenario/
event inputs. No oracle goldens or actual subject outputs were read by this author.
The illustrative original100/40 specimen is distinct from F01/F02 source witnesses.
The exact new source fixes identities/unaffected data as input choices; it does not
claim those identities were mandated by the original prose or already executed.
Independent source-work/IR/hash/step expectations and actual results remain pending.
