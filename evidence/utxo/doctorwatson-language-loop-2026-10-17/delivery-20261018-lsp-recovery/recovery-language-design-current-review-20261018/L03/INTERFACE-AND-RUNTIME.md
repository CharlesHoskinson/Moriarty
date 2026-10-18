# Proposed local interface, DTOs and runtime transition

This specifies the new ordinary capability completely enough to review its public shape. It is not implementation code or an adopted failure/security policy. Existing Core/2 types/functions remain unchanged.

## Opaque public library interface

Propose a new `mori_core::stage::recovery` module, initially explicitly experimental/source-gated rather than enabled by an unrelated target feature:

```text
load(base_source: &str, recovery_source: &str)
  -> Result<RecoveryProgram, RecoveryBuildError>

start(program: RecoveryProgram, action: &str, scenario: Scenario,
      committed_cap: Option<UInt>)
  -> Result<RecoveryProcess, RecoveryStartError>

RecoveryProgram::view() -> RecoveryProgramView
RecoveryProcess::resume(event: StageEvent, invocation_cap: Option<UInt>)
  -> Result<InvocationView, RecoveryInvocationError>
RecoveryProcess::recover(event: RecoveryEvent, invocation_cap: Option<UInt>)
  -> Result<InvocationView, RecoveryInvocationError>
RecoveryProcess::report() -> RecoveryProcessReport
```

Resume/recover methods take `&mut self`. The Process owns the loaded pair, selected action, actual current base StageOutcome/control and complete local invocation history. Its fields are private; it has no Clone/Deserialize/import-old-token constructor. This helps define one local value's control transition; it establishes no real-world uniqueness or global security. Starting another Process remains possible and external copies/authority/consumption stay untracked.

`start` calls the actual unchanged stage::start after the paired source checks. `resume` uses the actual embedded pending Continuation in the actual unchanged stage::resume. It never reconstructs a normal continuation from expected/report JSON. A successful normal Complete marks this Process forward-complete; a false or rejected resume preserves the actual prior pending Continuation according to current semantics. The wrapper tracks attempts/history separately.

`recover` resolves the source-bound handler for that pending action/site, binds its captured values and handler inputs, and runs a dedicated **internal recovery-body entry** on a working copy of its actual pending boundary. It reuses ordinary statement evaluation, not a call to unrelated public stage::start. New body evaluation must be explicit source/runtime work; a helper executing a refund action without changing this control does not implement it.

On successful recovery, mark the Process RecoveryComplete, retaining the closed post-boundary and all financial residuals. Do not construct a counterfeit base StageOutcome::Complete or rewrite its Continuation.committed to fit the normal plan. Subsequent resume/recover calls on this Process are defined not-pending invocations, with no new financial effects or evaluation charge. Global cancellation/replay/race meaning remains unassessed.

Initial scope is the public library consumer plus ordinary tests. Existing `mori simulate` file start/resume remains unchanged and must not advertise recovery. A CLI wrapper, serialized Process decoder, bare legacy-token import, resume from another host, asynchronous compensation or recovery of recovery is a later explicitly versioned consumer/persistence design. The minimum local feature cannot claim that journey or the full staged language release.

## Exact proposed DTO shapes

All members listed are present. `null` is allowed only where declared below; no alternate key-set is inferred. Existing nested DTOs retain their full existing shape. This is a new output schema requiring adoption and independent freeze; it makes no claim of observed public bytes.

```text
RecoveryEvent = {
  format:"moriarty-recovery-event/1", action:String,
  site:u16, handler:String, values:[NamedValue]
}
RecoveryProcessReport = {
  format:"moriarty-local-recovery/1",
  qualification:"local-reference-stage", authentication:"none",
  consumption:"untracked",
  base_semantics:"moriarty-core/2",
  recovery_semantics:"moriarty-core2-recovery/1",
  action:String,
  control: ControlView,
  accounting: AccountingView,
  residual:[Liability], history:[InvocationView]
}
ControlView =
  {kind:"pending", continuation:Continuation}
  | {kind:"forward-complete", boundary:Snapshot}
  | {kind:"recovery-complete", site:u16, await_label:String,
     handler:String, mode:"refund"|"compensate", boundary:Snapshot}
AccountingView = {
  profile:"moriarty-local-recovery-accounting/1",
  committed:UInt, attempted_total:UInt, committed_cap:UInt|null,
  ordered_effects:[Effect]
}
InvocationView =
  {kind:"start"|"resume", attempted:UInt,
   committed_before:UInt, committed_after:UInt,
   report:JsonObject}  // exact existing outcome_report value, including failures
  | {kind:"recover", site:u16, await_label:String, handler:String,
     mode:"refund"|"compensate", status:"not-taken"|"recovered"|"rejected",
     attempted:UInt, committed_before:UInt, committed_after:UInt,
     segment:RecoverySegment|null, error:Error|null}
RecoverySegment = {
  action:String, site:u16, await_label:String, handler:String,
  mode:"refund"|"compensate", frame_pre:PendingFrame,
  guard:{root:ExprId, value:Value, snapshot:Snapshot, effects:[]},
  execution:RecoveryExecution
}
RecoveryExecution = {
  semantics:"moriarty-core2-recovery/1", action:String, handler:String,
  pre:Snapshot, post:Snapshot, steps:[Step], effects:[Effect]
}
RecoveryErrorView = {
  format:"moriarty-local-recovery-error/1",
  qualification:"local-reference-stage", authentication:"none",
  consumption:"untracked",
  phase:"load"|"start"|"resume"|"recover"|"not-pending"|
        "observation-bound"|"accounting",
  error:Error, attempted:UInt,
  accounting_before:AccountingView|null,
  accounting_after:AccountingView|null,
  control:ControlView|null,
  base_report:JsonObject|null
}
```

The guard value is Bool true for a successful segment and its snapshot is the complete pre-boundary. Body Steps use existing ordinary Statement payloads but their ExprIds address the selected handler arena. Their numeric index is body source ordinal0..n−1; the guard is a separate source-clause observation, not a synthetic Step with a fake base ordinal. Complete Step.value/snapshot/delta rules apply. On false guard, segment=null/error=null/status=not-taken. On local body/guard failure, segment=null/error=full typed Error/status=rejected. On success, segment is complete/error=null/status=recovered.

`RecoveryBuildError`, `RecoveryStartError` and pre-body `RecoveryInvocationError` expose exactly RecoveryErrorView through a public report method. base_report is the full existing stage report when a real base invocation supplied one, otherwise null. Before a Process exists, absent cumulative accounting/control are **null/unknown**, never fabricated lifetime history. A failed actual fresh start can report its known attempted work and committed0 from its actual base rejection; it cannot invent prior invocations. For an invocation on an existing Process, error views include its actual unchanged accounting/control unless an evaluation charge was incurred, in which case attempted_total records it. Exact codes/messages/offsets, precedence, decoder rejection policy and admission/security meanings need owner review; no error oracle is invented here. Errors cannot masquerade as a successful InvocationView.

Method return convention is explicit: false guard returns Ok(not-taken); successful evaluation returns Ok of its corresponding successful InvocationView. An actual stage/handler rejection is appended as a rejected history record with its real attempts and then returned as Err(RecoveryInvocationError), whose view retains the same error/accounting. This retains failure visibility without reporting successful financial execution. A pre-evaluation wrapper error returns Err with attempted0 and unchanged state/accounting; it need not fabricate an evaluated history record. A failed start returns RecoveryStartError and no live Process.

No new input decoder exists for ProcessReport, ControlView, history or arbitrary snapshots. RecoveryEvent takes typed values in the library; a future textual event decoder must retain the existing bounded/strict JSON discipline and obtain its own versioned review. Existing `/2` 4MiB continuation/64KiB event limits and all current stage checks remain intact.

## State and work equations

Let C be cumulative successfully committed logical work, T all attempted invocation work, and E the ordered committed effects since this Process actually started. Initial history comes from the actual start outcome. An omitted old invocation cannot be guessed from a bare continuation: C alone does not reveal T or gross effects.

- Actual normal success: use the actual existing stage result's committed value for C, add its attempted work to T, append its segment's effects to E. A false normal resume retains its embedded old Continuation byte-for-byte while T increases in the separate Process.
- Actual normal rejection: retain prior pending state/C/E and append the full original rejection view; T increases by its attempted work. Do not add rejected effects.
- Recovery success with new successful work k and attempted t=k: C'=C+k, T'=T+t, E'=E++new_effects; control becomes recovery-complete. No normal committed_before/plan_total value is forged.
- Recovery false/failure: retain pending boundary/site/captures/C/E; T'=T+t, with complete false/failure view. Boundary/history before this invocation is not rolled back or rewritten.

Use checked UInt128 arithmetic. Financial totals derive per exact asset/account from E, not from net closing balances. Old fees remain; refunds never reduce earlier gross-debit counters. Liability evolution and complete current residuals stay separate. Caller/model accounts establish no authenticated funding/control.

The handler guard costs one dispatch plus distinct guard expression nodes. A successful body costs the sum of ordinary statement costs. No normal Await guard, entry-Await or hidden handler dispatch is added. Per-statement cache/work rules are unchanged for ordinary body statements. The separate guard observation is metadata for that source clause, not extra work. Recovery has no Await/calls/nesting, so its cost is finite.

RecoveryPlanView records normal plan_total and every `base committed_before(site)+guard_cost+body_cost`; its successful_path_ceiling is their maximum. Proposed paired start cap must cover that ceiling, and the same cap is passed to base stage::start. Normal calls keep existing cap behavior. Recovery cannot raise the stored ceiling; any invocation cap may only lower the effective allowed successful work. Cumulative attempts T may exceed a committed-path ceiling after false/failed invocations, so it must not be advertised as a lifetime attempt/fee budget. Exact provenance/admission/refusal rules and physical budget enforcement require owner review. No existing native counters or limits change.

The Process is bounded: propose at most256 actual invocations, matching the existing disjoint draft's finite observation ceiling, and preserve current source/trace/snapshot limits. Source pairing adds no broader Core/2 admission. The owner/resource review must assess actual history storage and command envelope before execution; a logical bound is not measured RAM or native cost. Reject the next invocation before evaluation when the local observation bound is reached; retain actual prior state/C/T/E. Never drop old history to make room.

Building the complete result/required local checks is part of the local evaluator's success boundary. No prior financial effects are undone on an output/formatting failure. If later serialization/export fails, retain the Process's actual state/accounting and report that output observation failed; do not rerun the financial invocation automatically or turn a missing report into nonexecution. Current bounded exporters and complete-report obligations remain; no CLI/public-byte result is claimed by this initial library scope.

## Financial distinction and failure boundary

Refund recovery executes explicit return transfers from modeled remaining custody. Compensation executes new forward transfers from the specified remedy source; it does not invent a merchant debit or erase old payment. Neither mode automatically discharges a liability. A handler may close normal control with positive residual duty. False outcome guards leave all such duties pending; deadline alone establishes no nonexecution.

A failed local handler publishes no new committed financial segment, retaining earlier state/effects/fees while charging attempted work. This is a proposed local atomic evaluator rule, not a statement about ledger phases. Real retained failure fees/effects, authentic outcome/funding/authority, duplicate recovery, races and copied-input consumption are owner-dependent. Qualification remains local-reference-stage/authentication:none/consumption:untracked in every report.
