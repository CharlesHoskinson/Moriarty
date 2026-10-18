# Recommended complete record and language semantics

## Decision and alternatives

Choose **A: explicit complete versioned record**. A public operation receives the actual retained source pair plus a complete record, performs the actual supported language transition, and returns a new complete record and full invocation report. Its persisted record is authoritative input to the next fresh process subject to designated-owner admission. Inspection/export/decoding do not themselves execute the language or establish provenance. No original field disappears behind a library object.

A directly exposes the checkpoint that CLI, test runner and independent comparator must carry. It makes pending versus closed control, C/T/history and residuals visible at each file boundary. It avoids serializing implementation caches, a VM stack or an in-memory Process layout. Existing Core2 wire remains unchanged, while the new companion profile has its own semantics/record version. Cost is explicit duplicated material (full histories and current state); bounds and admission must be settled by the owner before implementation, not by truncating history.

**B: owned process with checkpoint()/reopen().** B can make accidental in-memory misuse harder to express and encapsulate runtime bookkeeping. It still needs exactly the complete versioned wire below, source-bound reopening, public process exits and all A observations. Private clone checkpoints or a Process kept alive across test steps cannot implement B's durable reopen. B adds process reconstruction/caches and more lifecycle surface without reducing the wire/admission/IO obligations. It can be an internal later refactor, but is not the recommended public contract.

**Rejected substitution:** a bare old Continuation or complete waiting JSON viewer. Neither records cumulative attempted history or terminal recovery control; neither defines the missing recovery language transition. The wave report-viewer proposal stays historical and is not part of this first delivery. Full major3 inline-language design remains an alternative in the original immutable packet; choosing a compatible companion here avoids replacing all old Core2 IR, event and artifact profiles.

## Paired language and compatibility

Base source remains `language "moriarty/2"` with current Program/StagePlan/Continuation/StageEvent/StageOutcome wire and ordinary semantics intact. The explicitly selected companion is:

```text
recovery "moriarty-recovery/1";
for agreement AgreementName;
on action actionName at await awaitLabel {
  refund handlerName(eventName: Bool, amount: UInt) when guardExpression {
    ordinary finite statements
  }
  compensate otherHandler(otherEvidence: Bool) when otherGuard {
    ordinary finite statements
  }
}
```

The concrete case files show one handler each. A source may have a bounded finite list of action/site sections and handlers. A handler is a reusable language construct, not a special escrow/refund host helper. It reads the actual pending captured environment plus explicitly typed recovery-event binders. It has a separate fresh-occurrence expression arena: lexical handler order, children left-to-right before parent, no interning/folding; guard root recorded separately, followed by body expressions. It never renumbers base Program ExprIds or await sites. Base action ordinals stay zero-based; site stays one-based await occurrence. Captures preserve the selected site's exact existing Input/Let/Event layout. Proposed current examples use guard Name root0, body amount roots1/2.

Handler bodies use existing finite non-Await statements (Let/Requires/Ensures/Next/Emit/Transfer/Fee/Liability/Discharge) with their ordinary expression/value/effect behavior. No host/provider/native escape, nested await, unbounded loop, call framework or hidden external dispatch is introduced. A future suspended recovery needs a separately specified extension; existing broader partial/async obligations remain mandatory. Exact source/handler/trace/serialization numerical bounds are owner-dependent resource decisions; no doubled budget or new large allocation is selected here.

`refund` means an explicitly programmed recovery transition over still-controlled remaining custody; `compensate` means a new forward remedy after earlier effects committed. Both execute the handler's source statements and preserve earlier effects, fees, gross debits and duties. The source mode alone does not establish asset control, funding or authority; those are designated-owner prerequisites. Payment does not discharge a duty unless the handler contains the applicable explicit Discharge. Completing a recovery retires the selected pending forward path in the returned record; it does not execute the old suffix, globally revoke old files, erase economic history, or assert all residual obligations settled. No unrelated ordinary host action can retire that pending path.

The companion selection opts into new profiles: source `moriarty-recovery/1`, semantics `moriarty-core2-recovery/1`, inspection `moriarty-recovery-program/1`, plan `moriarty-recovery-plan/1`, checkpoint `moriarty-recovery-checkpoint/1`, event `moriarty-recovery-event/1`, local report `moriarty-local-recovery-report/1`, logical accounting `moriarty-local-recovery-accounting/1`, public test manifest `moriarty-tests/recovery/1`. These are proposed version names, not existing implementation. A command without a companion keeps the current profile/behavior. No implicit migration of a bare old token manufactures missing history/T. Owner must define exact migration/refusal diagnostics separately.

## Exact ordinary complete checkpoint

Every checkpoint has exactly these root members: `format`, `base_semantics`, `recovery_semantics`, `qualification`, `authentication`, `consumption`, `bindings`, `action`, `control`, `accounting`, `residual`, `history`.

- format is proposed `moriarty-recovery-checkpoint/1`; base_semantics is `moriarty-core/2`; recovery_semantics is the new companion semantics. Explicit labels are local-reference-stage / none / untracked.
- bindings contains base_program, base_plan, recovery_module, recovery_plan. Exact canonical identities/preimages are designated-owner work. Examples carry literal OWNER slots rather than fabricated hashes.
- action is the selected existing base action. Source/module/plan inspection exposes all four bindings, explicit local/unsigned/untracked labels and the entire pair, not only the selected handler or a selected-action projection.
- control is one of the three exact normal variants below.
- accounting is exactly profile, committed, attempted_total, committed_cap, ordered_effects. All UInt values use canonical decimal strings; cap is string or explicit null. ordered_effects contains the complete cumulative committed effects in invocation/source order, retaining duplicates and all fee/duty effects.
- residual is every positive current-boundary Liability in canonical boundary order. Full boundaries retain zero rows too. Residual presence does not keep the already retired forward path resumable.
- history is the entire ordered list of normal start/resume/recover observations. It retains all original baseline and recovery trace fields; no prefix/delta is silently dropped or re-evaluated on reopen.

Normal control variants:

1. `{kind:"pending", continuation:<full unchanged Core2 Continuation>}`. The embedded token carries actual action/site/captures/logical stored cap/committed/boundary. A false observation returns that entire embedded token unchanged; outer T/history can still advance.
2. `{kind:"forward-complete", boundary:<full Snapshot>}`. The actual normal action completed, including zero-await actions. There is no pending frame or continuation. History retains the actual complete baseline report and its idle post frame.
3. `{kind:"recovery-complete", site:<u16>, await_label:<String>, handler:<String>, mode:"refund"|"compensate", boundary:<full Snapshot>}`. The selected pending path has been retired by the actual successful named handler. Its full pre-frame/captures remain in the invocation trace/history. No implicit deletion of positive duties occurs.

These are complete ordinary successful/non-taken record variants. Owner-defined refused/partial/unknown failure results are **not coerced** into any of them. Exact additional control/result/error representation for each owner-permitted non-normal prefix is a required first-delivery owner output; this design does not invent it. Its absence blocks those implementations and prevents a complete first-delivery acceptance claim, without withholding the concrete ordinary recommendation.

## History, source traces and report shape

Each baseline entry has operation start|resume, attempted, committed_before, committed_after and baseline_report containing the entire existing qualified `stage::outcome_report` object. Thus Waiting advanced/segment/null/token/C/T/residual and Complete frame/steps/effects are preserved exactly. Start-before C0; subsequent before C equals prior committed. Baseline reports are historical observations, not an alternate authority source or an executable history to replay.

Each recovery entry has operation recover, site, await_label, handler, mode, status recovered|not-taken, attempted, committed_before, committed_after, guard, segment (complete or null), error:null on the ordinary paths. Every ordinary recovery entry retains a separate full guard record (root/typed Bool/snapshot/effects), true for recovered or false for not-taken. A successful body segment contains action/site/await_label/handler/mode/frame_pre and complete recovery Execution. Body Steps have their handler body ordinals, exact handler statements and arena roots, typed values, complete post snapshots and new effect deltas. Recovery Execution identifies its new semantics and handler; its effects concatenate body deltas in order. Guard source metadata is not injected as a fake base Await Step. A non-taken guard records its full false guard observation but no body segment/effects and retains the pending boundary/duties.

Local report has exactly format, qualification, authentication, consumption, operation, status, attempted, invocation, checkpoint. operation start|resume|recover|reopen; status waiting|forward-complete|recovery-complete reflects the returned normal control. attempted is this language invocation's logical work, and invocation is the actual complete last entry, except reopen has attempted0 and invocation:null. Reopen leaves the whole checkpoint C/T/history/effects/control unchanged. This proposed zero measures no language execution; decode/file/host costs remain separately bounded/unmeasured and must not be called free service or circuit cost. The owner supplies actual import/admission/resource policy.

## Exact ordinary work and economic prefix preservation

Normal base invocation work uses the current adopted Core2 statement/visited-expression schedule. Base start and normal resume still produce their actual complete baseline outcomes. The companion normal proposal charges recovery guard dispatch1 plus its visited nodes, then each body statement and distinct visited nodes, using fresh per-statement caches. There is no hidden handler-entry work unit. C grows only by successfully committed source guard/body or normal forward work; false observations add their actual work to T but not C. T is the sum of all actual recorded normal invocation attempts, never reset across publication/reopen. Error work and admitted failure prefixes remain designated-owner decisions; no rejection rule is inferred from these normal equations.

For a normal invocation i: T_after=T_before+attempted_i. For successful advancing i: C_after=C_before+committed_delta_i. For a non-taken guard: C_after=C_before and ordered_effects/boundary/residual stay unchanged. New committed effects append to the retained prefix exactly once in the returned record. Closing does not subtract historical gross debit, refund fees, reset cumulative work or erase unrelated duties.

Plan exposes complete base plan and per recovery arm guard_cost/body_cost/recovery_cost/committed_before/successful_path_total, plus each selected action's forward_total and successful_path_ceiling=max(normal forward total, each allowed successful recovery path total). This is required inspectable arithmetic metadata. **Owner must author combined registry/account/trace admission and paired ceiling preflight.** Compensation example normal plan_total11 cannot stand in for paired ceiling13. The ordinary metadata does not select refusal order, cap editing, provenance, recovery reserve or physical budget rules.

## API and actual implementation boundaries

Recommended public library contract (declarations in `CONTRACT.json`) is: load_pair→RecoveryProgram; inspect_pair→whole Program/Plan report; start→Transition; reopen actual bytes→ReopenedRecovery; resume/recover from that reopened record→Transition; checkpoint_bytes→complete wire; outcome_report→full local report. Transition carries actual full checkpoint and invocation. ReopenedRecovery exposes the complete checkpoint; it is constructed by the actual selected decoder/admission implementation, never a fixture or caller-injected successful validator. The owner supplies its exact admission/failure contracts. A Rust opaque type is not itself a proof or new public developer permission.

Public CLI readers use the retained source pair and actual on-disk bytes. The writer receives the actual resulting checkpoint from that transition; no expected JSON, host-computed financial replacement or bare token is substituted. Successful public state-changing operations require actual checkpoint publication and a complete report. The positive reader→transition→serialization→actual file→fresh reader connection is fixed; exact publication algorithm/order versus output/failures/crashes remains O-IO owner work. See `PUBLIC-LIFECYCLE.md`. No production writer/decoder is claimed complete by these signatures.

## Cap qualification for these normal examples

The literal13 is the proposed local **committed_cap**, not an adopted lifetime attempt/service budget. C13/T15 after a false observation preserves15 actual language attempts; it does not claim those15 fit a total-attempt cap13. Actual lifetime/physical/recovery-reserve limits and cap admission are O-CAP/O-TRACE owner outputs, and may require additional budget fields or revised fixtures before adoption. Examples are conditional on their complete normal histories being admitted under that eventual contract. No missing budget is silently treated as unlimited, free or reset; host/file/decode work and circuit/DUST/service costs remain distinct.
