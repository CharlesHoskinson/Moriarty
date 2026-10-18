

===== PACKET LIMITS | REVIEW-SCOPE.md | review instructions =====
Two UNADOPTED complete ordinary alternatives follow. Evaluate required inspectable LANGUAGE recovery, eventual durable public continuations/CLI and retained full-language exits; do not substitute an easier opaque library-only wrapper as completion. Source-only design review, not security judgment/execution authority. Full physical originals are present. Every primary section identifies its exact representation. Mandatory supplementary full-file reads are listed in REQUIRED-READS.json. Missing/truncated/unread source means abstain or explicitly limited review, never pretend full coverage.


===== L03 COMPLETE RECOMMENDATION | L03/README.md | full verbatim file =====
# L03: smallest coherent recovery version/interface

Source-static ordinary design comparison against **exact merged main `74a909d69639df8823a527a4d608a7516ccd69a2`**, not the stale local main ref. No adoption, implementation, execution or security judgment is supplied.

Recommend a **separately versioned recovery language companion module and an opaque local RecoveryProcess**, bound to an unchanged Core/2 program. This is smaller than duplicating the complete language/normal-event/continuation/report family under my earlier `/3` proposal. It preserves every existing `/2` wire, forward ExprId and stage outcome while giving recovery its own source declaration, actual pending-path transition, trace, cumulative accounting and closed result. Existing independent design votes are absent; root must obtain the required two substantive non-author votes and designated-owner review before implementation.

Read:

- [Version comparison and baseline facts](VERSION-COMPARISON.md).
- [Complete ordinary grammar and IR proposal](GRAMMAR-AND-IR.md).
- [DTO, runtime and accounting proposal](INTERFACE-AND-RUNTIME.md).
- [Finite predicates, EARS mapping and input custody](PREDICATES-AND-CUSTODY.md).

The original unadopted `/3` proposal is preserved under `prior-proposal/`, including its fully worked financial examples. Current public source inputs are complete pinned copies under `source/`; [SOURCE-PINS.json](SOURCE-PINS.json) supplies Git blob/SHA-256 identities. [STATUS.json](STATUS.json) records the decision boundary.

The first scope is a public **in-memory library capability**, not a new serialized resume-token migration or CLI cancellation claim. It starts tracking at the actual Core/2 start result and owns all subsequent actual invocation observations. A bare legacy continuation lacks cumulative attempted history/effects; it cannot be silently imported and called complete accounting. Persistence/migration, authentic authority/outcome/funding, replay/races, cap provenance, target retained-failure effects and proof/ledger correspondence remain owner-dependent and open.

Only the new R packet was written. No production/shared worktree/Git mutation, build, evaluator/model/SDK/native/provider dispatch or subagent was performed. Source/metadata consistency does not close an original requirement.



===== L03 COMPLETE RECOMMENDATION | L03/VERSION-COMPARISON.md | full verbatim file =====
# Current baseline versus the recovery alternatives

## Current main observations

These are source facts from `74a909d69639df8823a527a4d608a7516ccd69a2`, not new runtime results:

| Actual current consumer | Constraint the design must respect |
| --- | --- |
| `stage/mod.rs` | `moriarty/2` / `moriarty-core/2`; Continuation format `moriarty-continuation/1`; StagePlan format `moriarty-stage-plan/1`; StageEvent format `moriarty-stage-event/1`. Continuation has format/program_hash/plan_hash/action/site/captures/logical/boundary. StageLogical has profile/cap/committed, with required cap key including null. |
| `stage/admit.rs::finish` | False resume returns the actual input Continuation unchanged; attempted work is per invocation. Normal completion requires committed==plan_total and an idle empty capture frame. A recovery branch cannot manufacture a normal Complete with a different work total. |
| `stage/mod.rs::StageRejection` | Error/attempted/committed only; it publishes no replacement continuation or segment/effects. Adding fields or a new variant silently would change existing public behavior. |
| `stage/report.rs::outcome_report` | Exact existing waiting/complete/rejected JSON projection plus local-reference-stage/authentication:none/consumption:untracked. Existing root key sets are adopted obligations, not free extension points. |
| `stage/cost.rs` | Existing normal segments and committed_before/plan_total are source-derived; stored cap is retained and cannot be raised by CLI override. Per-invocation attempts differ from successful committed work. No lifetime attempt counter exists in the token. |
| `parser.rs`, `ir.rs` | No recovery declaration, recovery association or cancellation Statement. Await and ordinary financial statements exist. Source expression IDs and complete Statement objects are observable; inserting inline handler expressions into existing `/2` allocation would change those objects/identities. |
| `stage/disjoint.rs` | New feature-gated local Pair owns actual member outputs, history and cumulative attempted work without a decoder; it does not mutate base StageOutcome/Continuation. This is an existing repository example of an opaque local wrapper, not proof that recovery exists. Pair remains a disjoint composition draft with its own scope. |
| `commands/simulate.rs` | Current public file workflow is start/resume. No pending recovery operation exists. Normal multi-condition and disjoint observations do not implement cancellation. |

The consolidated adopted clauses require exact Core/2 Step/index/ExprId/snapshot/delta, false-resume null segment, current DTO projection and separately frozen independent observations. Preserve those obligations for the old consumers. New recovery-specific semantics need their own adoption and observations; an API wrapper alone cannot satisfy complete financial or target requirements.

## Three real version choices

| Choice | Benefits | Costs/risks | Recommendation |
| --- | --- | --- | --- |
| Extend `/2` Program/Await/Continuation/StageOutcome in place | One apparent namespace | Breaks frozen Statement/ExprId and wire/key-set behavior; total-attempt updates contradict byte-identical false continuation; recovery totals conflict with normal plan_total. Calling this merely additive hides changed semantics. | Reject |
| Earlier full `moriarty/3` / `moriarty-core/3` family | Clean separate semantics for inline arms and a new persisted lifetime/accounting model; suitable if a major whole-language revision is desired | Duplicates Program/normal-event/trace/continuation/normal APIs and requires a broad new independent freeze before one recovery transition is useful. The prior version choice was reasoned but larger than the minimum local capability. | Coherent alternative; keep unadopted |
| **Separate recovery companion language/profile plus opaque Process** | Core/2 source/IR/wire and actual normal consumers remain unchanged; recovery has explicit source association and a separate schema; lifetime attempts/effects live in owned local history without inventing legacy token data | Two first-class modules must be paired explicitly. Initial scope has no persisted Process decoder/import or CLI recovery. Requires an ordinary module binding/trace/cost design and owner review of identities/authority. | **Smallest coherent first scope** |

The companion is a language module, not a script that runs an arbitrary refund action. It declares the pending Await association, handler scope, financial body and mode; the runtime has a dedicated recovery transition on its actual owned pending state. It cannot close a different Process, and the body is never dispatched through an unrelated public `stage::start` action.

## Version decision proposed for adoption

- Forward source and semantics remain exactly `moriarty/2` and `moriarty-core/2`.
- New recovery module header/profile is `moriarty-recovery/1`; its local transition semantics profile is `moriarty-core2-recovery/1` and report format `moriarty-local-recovery/1`.
- The pair is explicit: one unchanged Core/2 source plus one companion module. No preprocessing that strips recovery syntax and pretends the result was the original source. Parse each module independently; inspect both in the public paired Program DTO.
- Source-fixed action/Await-label association resolves to the existing one-based plan site and original source ordinal. Recovery uses its own expression arenas and route coordinates; normal forward IDs/ordinals are untouched.
- No changes to the old Continuation, StageEvent, StageLogical, StagePlan, StageSegment, StageOutcome, StageRejection, Error or `outcome_report` DTOs. Normal library/CLI paths retain their exact behavior and old fixture bytes.
- A new opaque Process exposes its own complete report. A nested baseline invocation report is the exact existing projection as a JSON value; wrapping it does not transfer its old standalone byte golden to the new outer report.
- Initial Process has no serialized input decoder, no import of bare old pending files and no cross-process migration. Its reports are output-only. A future persistence profile needs independent schema/identity/provenance/admission work; it cannot be quietly added under this decision.

These are unadopted choices. The earlier `/3` packet is not amended or labeled approved. Reviewers may prefer that alternative if persistence or inline recovery is required in the first release; then its larger schema/accounting/normal-output consequences must be accepted explicitly. The minimum choice does not delete downstream persistence/private/native obligations.

## Owner-dependent rules retained separately

No ordinary version choice establishes authentic program/pair identity, funded control of accounts, rights to choose recovery/discharge duties, reliable external finality or nonexecution, once-only consumption, late/duplicate-event races, signed cap provenance or native/ledger retained-failure behavior. Ordinary local body failure can discard unpublished working changes; it must not be translated into global rollback. Two agreeing non-author design votes and designated-owner review of the exact paired-language interface remain prerequisites to implementation.



===== L03 COMPLETE RECOMMENDATION | L03/GRAMMAR-AND-IR.md | full verbatim file =====
# Complete ordinary companion grammar and IR proposal

Everything below is proposed, not adopted. The module is first-class language input to a dedicated Core/2 recovery consumer. It is not a shell/host action, external solver command, mutable handler registry or rewriting preprocessor.

## Two explicit source inputs

The base file is an ordinary, unmodified `language "moriarty/2"; agreement ...` source, accepted through the actual `stage::load` path. The separate recovery file has this grammar:

```text
RecoveryModule := recovery String ';' for agreement Identifier ';' RecoverySite+
RecoverySite   := on action Identifier at await Identifier '{' Handler+ '}'
Handler        := Kind Identifier '(' BinderList? ')' when Expr '{' OrdinaryStmt* '}'
Kind           := refund | compensate
BinderList     := Identifier ':' Type (',' Identifier ':' Type)*
Type           := UInt | Bool
```

The header String must be `moriarty-recovery/1`. Expressions and OrdinaryStmt syntax are exactly the already supported Core/2 ordinary expressions and Let/Requires/Ensures/Next/Transfer/Fee/Liability/Discharge/Emit syntax. Await, handler nesting, recursion, action calls, dynamic dispatch, new state/asset declarations and external commands are outside the first recovery module. Every ordinary body statement keeps its semicolon. The recovery site/handler braces introduce no implicit financial statement.

Example companion to a base RefundExample with action deliver and Await release:

```mori
recovery "moriarty-recovery/1";
for agreement RefundExample;
on action deliver at await release {
  refund cancel(cancel_local: Bool) when cancel_local {
    transfer(PAY, "escrow", "alice", 60);
    discharge("return_due", 60);
    emit "refund_paid"(60);
  }
}
```

Example companion to CompensationExample/buy/acknowledgement:

```mori
recovery "moriarty-recovery/1";
for agreement CompensationExample;
on action buy at await acknowledgement {
  compensate part_payment(deadline_reached: Bool, outcome_known: Bool)
    when deadline_reached and outcome_known {
    transfer(PAY, "insurer", "alice", 20);
    discharge("remedy", 20);
    emit "compensation_paid"(20);
  }
}
```

The corresponding base source is precisely the earlier example with language changed to ordinary `/2` and the inline recovery clause removed **as a separately authored input**. This comparison supplies no preprocessing implementation and no assertion that existing parser results automatically equal a new projection. The fresh independent custodian must freeze the complete base sources/Program/Plan and companion IR separately.

## Static association and scope

- Agreement name must match the paired base program. Action and Await label resolve in the actual base program/plan; bind the resulting one-based site and zero-based Await ordinal. Do not select a host-provided action as the handler.
- Site blocks and handler names are unique for their action/site; order is lexical. Handler kind comes from source and cannot be overwritten by an event.
- Scope is the actual site's captured Input/Let/prior-Event slots, current complete boundary state/liabilities, and selected handler binders. Normal Await event binders that have not arrived and later forward locals are unavailable. Handler locals are lexical and do not mutate captured values or become state.
- Handler binders cannot shadow captures, state names or each other. Guards are Bool. Existing UInt128 checked operations and normal statement typing apply; no changed rounding, asset substitution or numeric convention is introduced.
- Bodies may reference declared base assets/state and source-fixed duty identities. A handler may create a new declared liability in its own body; the recovery module's registry records its exact name/asset/debtor/creditor for closing validation. It cannot silently reinterpret a base duty with a different identity. Determining authentic liability authority is owner-dependent.
- To preserve unmodified base admission, first-scope start scenarios must be admissible under the base source's current registry. Incoming liabilities known only to a recovery body are not newly admitted through old stage::start. New handler-created positive duties can appear in the recovery's closed residual boundary. Broader incoming-domain or resumable multi-stage recovery requires a separate extension; this finite restriction does not remove the full language obligation.
- Every account potentially used by a handler must already exist in the complete input boundary for its exact declared asset, including zero rows. Additional protected rows remain preserved. No source annotation creates a funded account or actual control.

`refund` identifies a source-declared return from the application's modeled remaining custody; `compensate` identifies a new forward remedy. Neither annotation authorizes money, supplies a balance, inverses history or discharges a duty. The actual statements determine local effects. A Boolean named outcome_known is still caller-supplied local data. Authentic classification/control/funding/authority remains owner-unassessed.

## Public paired Program/Plan DTOs

All objects below have exact listed members; reject unknown members in any future reviewed decoder. The initial proposal exposes output views only, not a new persisted program/process input decoder. Object member order has no semantic meaning; arrays have the specified order. UInt uses canonical decimal strings and indices use unsigned JSON numbers.

```text
RecoveryProgramView = {
  format: "moriarty-recovery-program/1",
  source_profile: "moriarty-recovery/1",
  semantics: "moriarty-core2-recovery/1",
  base: Program,                 // complete existing Core/2 Program DTO
  module: RecoveryModuleView,
  plan: RecoveryPlanView
}
RecoveryModuleView = { agreement: String, sites: [RecoverySiteView] }
RecoverySiteView = {
  action: String, await_label: String, site: u16, await_ordinal: u16,
  handlers: [HandlerView]
}
HandlerView = {
  name: String, kind: "refund"|"compensate", binders: [InputDecl],
  captures: [CaptureSlot], expressions: [Expr], guard: ExprId,
  statements: [Statement]        // ordinary variants only; no Await
}
RecoveryPlanView = {
  format: "moriarty-recovery-plan/1",
  base_plan: StagePlan,          // complete existing Core/2 StagePlan
  actions: [RecoveryActionPlan]
}
RecoveryActionPlan = {
  action: String, forward_total: UInt, successful_path_ceiling: UInt,
  arms: [{ site:u16, handler:String, guard_cost:UInt,
           body_cost:UInt, recovery_cost:UInt,
           committed_before:UInt, successful_path_total:UInt }]
}
```

Names/types such as Program, InputDecl, CaptureSlot, Expr and ordinary Statement denote current public DTO payloads, not modified base definitions. Pair identity derivation/binding requires owner review; this ordinary proposal supplies no guessed hashes. The public program view names both profiles and the complete paired inputs rather than treating a Core/2 hash alone as identity for the recovery module.

Each handler has a separate expression arena: allocate guard occurrences in ordinary left-to-right child-before-parent order, then body statements in order; no sharing/interning/folding and no cross-handler references. Handler ExprIds refer to its own `expressions`; forward ExprIds remain in the base Program's unchanged array. The source coordinate is `(action,site,handler,guard)` or `(action,site,handler,body_index)`. No synthetic Statement is inserted into the base action and no fake forward ordinal is assigned to the recovery guard.

For the refund companion, guard is local Name at0; the three body literal60 occurrences are distinct at1/2/3. For the compensation companion, guard nodes are Name(deadline)0, Name(outcome)1, And(0,1)2; body literal20 occurrences are3/4/5. These are design examples for a later independent complete freeze, not parser observations.

Bounds proposed for the paired consumer are no larger than current finite language ceilings: combined base+module expressions≤8,192; per action combined base statements plus handler guards/bodies≤1,024; combined declarations/handler declarations≤256; existing Await/capture/binder ceilings retained. These extra pairing checks do not change which standalone `/2` programs old stage::load accepts. Exact validation/cap/error ordering remains subject to owner review. No new physical command budget is granted.



===== L03 COMPLETE RECOMMENDATION | L03/INTERFACE-AND-RUNTIME.md | full verbatim file =====
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



===== L03 COMPLETE RECOMMENDATION | L03/PREDICATES-AND-CUSTODY.md | full verbatim file =====
# Finite feature predicates, requirement mapping and custody

## Source-author worked finite examples

The preserved [prior examples](prior-proposal/EXAMPLES.md) contain complete A–D account/liability/state/effect/work tables. This comparison retains their hypothetical financial policies and arithmetic; it changes the source pairing/interface and guard trace representation, not the amounts. These are authored design examples, not independent goldens or implementation observations.

The base A source is separately declared as ordinary Core/2:

```mori
language "moriarty/2";
agreement RefundExample {
  asset PAY = { network: "local", issuer: "recovery-design", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver() {
    fee(PAY, "alice", "treasury", 2);
    transfer(PAY, "alice", "escrow", 60);
    liability("return_due", PAY, "escrow", "alice", 60);
    await release(ready: Bool) when ready;
    transfer(PAY, "escrow", "merchant", 60);
    discharge("return_due", 60);
    emit "delivered"(60);
  }
  action record_warranty(amount: UInt) {
    liability("warranty", PAY, "operator", "alice", amount);
  }
}
```

Pair this with the refund companion in GRAMMAR-AND-IR.md. Opening account amounts are alice100/escrow0/merchant0/observer7/operator20/treasury0, state marker77 and incoming warranty9. An unused source action declares that exact warranty identity. A's prefix costs7 and leaves alice38/escrow60/merchant0/observer7/operator20/treasury2, present return_due60 and warranty9. Recovery costs8: guard2 plus three body statements at2 each. It appends return transfer60, discharge60 and Emit60; final alice98/escrow0, other accounts unchanged, return_due present-zero and warranty9 residual. Lifetime per-account gross debit remains alice62/escrow60, aggregate122; net alice−2/treasury+2; fees2. The actual base forward plan remains total15; recovery path total15. Direct Process C/T is15/15; false guard then true C/T is15/17. Embedded old Continuation remains unchanged during false while wrapper T increases.

The base B source is likewise ordinary Core/2:

```mori
language "moriarty/2";
agreement CompensationExample {
  asset PAY = { network: "local", issuer: "recovery-design", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action buy() {
    fee(PAY, "alice", "treasury", 2);
    transfer(PAY, "alice", "merchant", 50);
    liability("remedy", PAY, "insurer", "alice", 50);
    await acknowledgement(delivered: Bool) when delivered;
    discharge("remedy", 50);
    emit "accepted_delivery"(50);
  }
}
```

Pair it with the compensation companion in GRAMMAR-AND-IR.md. Opening alice100/insurer30/merchant0/observer7/treasury0, marker77, no liabilities. Prefix7 leaves alice48/insurer30/merchant50/observer7/treasury2 and remedy50. True/true recovery guard4 plus transfer/discharge/emit6 costs10; final alice68/insurer10/merchant50/observer7/treasury2, remedy30 residual and unchanged marker77. Original merchant50 and fee2 stay committed. Lifetime gross alice52/insurer20, aggregate72; nets alice−32/insurer−20/merchant+50/treasury+2/observer0. Duty equation0+50−20=30. Actual base normal plan_total is13; recovery path total17 and the paired proposed ceiling17. Never set a forged base committed value17 or label this base StageOutcome::Complete.

| B invocation path | Complete control/boundary effect | Committed C | Total attempts T |
| --- | --- | ---: | ---: |
| Prefix only | Actual pending acknowledgement; merchant50/fee2/remedy50 | 7 | 7 |
| Deadline true, outcome unknown/false | No guard segment/body effect; same complete boundary/control | 7 | 11 |
| Then true/true recovery | New insurer→alice20, remedy50→30, Emit20; RecoveryComplete | 17 | 21 |
| Direct true/true recovery | Same final state/effects/duty without the false observation | 17 | 17 |
| Separate normal delivered:true path | Existing normal forward completion; remedy explicitly discharged50; merchant50 and fee2 remain | 13 | 13 |

For a distinct local body-failure design case, B opening insurer10 makes the requested20 payment unavailable in the modeled snapshot. Under the proposed source evaluation order, guard costs4 and attempted transfer costs2, then body rejects: C7/T13, prior boundary and remedy50 retained, no new successful effect, fee2/merchant50 retained. Exact diagnostic/ordering/authority expectations remain owner-unassessed; this is no new security test or executed refusal claim.

Successful recovery body Steps now use indices0/1/2, with separate source-clause guard metadata; earlier `/3` gate-Step0/body1..3 representation is not carried over. Refund handler roots are1/2/3 after guard root0; compensation body roots3/4/5 follow guard root2. Complete per-step snapshots and financial deltas remain mandatory. A final account equality alone is insufficient.

## Single smallest coherent consumer predicate

After adoption and independent freeze, the public paired load/start/recover library consumer must take A's actual pending state and cancel_local:true, preserve the actual old forward receipt/Continuation shape, and return RecoveryComplete with exact complete new guard/body report, original ordered prefix plus refund effects, fee2, alice gross62, escrow gross60, present-zero return_due, warranty9 residual, C15/T15 and no pending normal successor. A later normal resume on that same Process is not a normal pending operation. This is value-level local closure, not external revocation/once-only evidence.

The fuller finite set adds false guard accounting, B's forward compensation/positive residual and deadline-only unresolved case, then ordinary body-failure coverage after diagnostic premises are adopted. These are new recovery predicates, not repetition of normal multi-condition passing prefixes. Startup/prefix observations are necessary actual input custody, not a new completion count.

## Proposed ordinary EARS crosswalk

The following are review clauses, not canonical edits or a replacement inventory:

| Proposal | Ordinary predicate | Retained requirement |
| --- | --- | --- |
| L03-01 | WHEN an old Core/2 source/API is used, its existing Program/Plan/Continuation/StageOutcome/report semantics and key sets SHALL remain unchanged; the recovery module SHALL use a separately named consumer/profile. | UNI-018/021; adopted finite Core/2/M01/normal report clauses |
| L03-02 | WHEN paired recovery is invoked on an actual pending Process, it SHALL select the source-bound site/handler, use actual boundary/captures and publish a distinct closed recovery result only after that body's normal completion. | MPLR-005; UNI-006/008; missing C0.1 recovery judgment |
| L03-03 | WHILE the guard is false, the pending boundary/control and committed effects/work SHALL remain; only separate cumulative attempted accounting increases. Deadline-only observation SHALL NOT create a local recovery-complete result. | MPLR-010/009; UNI-007/020 |
| L03-04 | WHEN refund or compensation changes finances, explicit source statements SHALL determine complete ordered asset effects, retained fees/gross debit and separately typed liability evolution; control closure SHALL NOT delete positive residual duties. | MPLR-005/017; UNI-005/020 |
| L03-05 | IF a local handler rejects, the result SHALL preserve prior committed state/effects/fees/duties and report actual attempts/full error, without a successful recovery segment. Actual target retained-phase effects SHALL remain separately qualified. | MPLR-009/017; UNI-006/021; product phase distinction |
| L03-06 | WHEN recovery is claimed on a target or as authentic cancellation, missing authority/outcome/control/consumption/history/proof/ledger evidence SHALL remain explicit; finite local wrapper observations SHALL NOT close those requirements. | MPLR-005/006/010/011/017; UNI-019/021/022; U3/U4 and full release exits |

No scope reduction is accepted by a language version. Original 119 UTXO/11 first-contract/18 PCD inventories, all U0–U7/M1–M17/MC/SP/G exits and full financial/composition obligations remain. Pair's local disjoint feature and normal multi-condition results retain their original scopes; they cannot vote or supply cancellation results.

## Independent input custody and decision gate

The current author has inspected the base implementation and the earlier design. New examples and DTOs are source-exposed author material. Root must obtain two substantive agreeing **non-author design votes** plus designated-owner review before implementation. None is present; no provider is dispatched here. Preserve dissent and refresh affected reviews if the exact packet changes.

A fresh non-author custodian, after design adoption, independently derives from adopted ordinary rules and complete source inputs: exact full base Program/Plan; separate companion handler arenas/layouts/costs; complete actual-input scenarios/events; complete new output DTO/key sets and trace/snapshot/delta/effect history; per-asset/account gross/net/fees/duty/work tables; and wrapper/old-base compatibility expectations. No candidate parser/evaluator/model output, shared wrapper or new actual result may generate expected data. Identity and failure goldens require owner-adopted premises before freezing; no guessed hashes/codes are supplied.

Existing M01/M03 and report adoption do not automatically adopt this companion grammar, handler-local arena, new accounting profile, Process control, guard metadata or failure schema. Those are explicit unadopted choices. Authority/actual funding/control, reliable external outcome evidence, cap/boundary provenance and failure ordering, copied-input replay/races, persistence/migration, identity binding and target phase/native/proof interfaces remain owner-dependent. The minimum public in-memory feature is not general authenticated cancellation or a deployed settlement result.

No production/shared source, W/I/J/K/G or Git state is changed. No build, runtime/evaluator/model/SDK/native/provider dispatch or subagent is used. Root's later implementation decision applies to the exact reviewed candidate, not this document's existence.



===== ORIGINAL FULL MAJOR3 DESIGN | major3/DESIGN.md | full verbatim file; supporting full files mandatory separately =====
# Pending recovery as an explicit language transition

Ordinary language design proposal against main `7396d7d316ab72957bca9709483f4638daa0d5cb`. **Not adopted, implemented, executed or security reviewed.** The requested model assignment is GPT-6.1 Sol high. Two substantive agreeing non-author consequential-design votes and designated-owner review must precede implementation. This packet dispatches no providers and casts no such votes.

## Problem and retained requirements

Current Rust Core/2 has source Await, `stage::load/start/resume`, Waiting/Complete results and a public simulate start/resume path. It has no recovery arm, pending-frame recovery entrypoint or recovery result. A separate ordinary refund action through start cannot close the prior pending computation. The retained [gap](CANCELLATION-TRANSITION-GAP.input.md) and complete pinned public [sources](SOURCE-PINS.json) establish that source scope; existing stub tests cover normal multi-condition execution only.

The [normative partial/conditional spec](source/openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md) requires:

- MPLR-005: distinguish refund of assets still controlled from compensation after committed effects, preserving actual history and unrecovered duties.
- MPLR-010: timeout does not establish nonexecution; unresolved external duties survive until reliable outcome evidence resolves them.
- MPLR-017: account separately for assets/custody, fees and typed liability evolution.

C0.1 recovery judgments and C3.2 cancellation/compensation remain open. The product contract distinguishes atomic local evaluator rejection from actual ledger phases that may retain effects/fees. This proposal defines a finite local reference transition and an explicit interface for future qualification; it does not satisfy authenticated/once-only/native/ledger clauses by itself.

## Alternatives and recommendation

| Alternative | Capability and cost | Disposition |
| --- | --- | --- |
| Source-bound recovery arm at an Await, with a typed pending-frame invocation | Reuses ordinary financial statements; statically identifies the affected pending control; bounded atomic handler; requires new plan/report/accounting versions | **Recommend** as the smallest coherent reusable language increment |
| General named alternative continuations, each able to Await, join and invoke other continuations | Expresses multi-stage sagas directly, but requires branch graphs, nested capture/lifetime rules, recovery-of-recovery, joins and a larger proof/cost model | Retain as a later generalization; too much language surface for the first missing transition |
| Separate ordinary refund/compensation action through existing start | Useful forward financial action with existing syntax, but cannot say which old pending control changed; a host deleting a continuation file supplies no language transition | Valid narrower example, insufficient for pending cancellation; do not label it implemented recovery |

The selected arm is reusable for any supported financial program. It is neither an application-specific escrow helper nor a framework. An arm is source code bound to exactly one Await site, not a runtime-selected arbitrary action. Initial handlers have no Await/calls/join: one finite body succeeds or rejects in a single local invocation. Multi-stage compensation remains an explicit missing extension.

## Source surface and scope

Use a **new proposed Rust source profile** `language "moriarty/3";` with semantics `moriarty-core/3`. These names require adoption; no consumer currently supports them. A major version is warranted because branch control, persisted work/effects and rejection outputs change. Do not retrofit new members into `/1` or `/2` DTOs or reuse their frozen bytes/identities. Legacy Source/3 research profiles with different header names are not this profile.

Proposed grammar, extending existing ordinary Core/2 expressions and financial statements:

```text
Await3       := await Name '(' Binders? ')' when Expr RecoveryArm* ';'
RecoveryArm  := recover RecoveryKind Name '(' Binders? ')' when Expr '{' OrdinaryStmt* '}'
RecoveryKind := refund | compensate
```

`OrdinaryStmt` is exactly existing Let, Requires, Ensures, Next, Transfer, Fee, Liability, Discharge or Emit; no nested Await/Recover, action calls or hidden host action. Normal statements after the Await remain the forward suffix. On successful recovery they are not executed. Zero recovery arms is allowed. Arm names are unique in the containing action; invocation identifies action/site/arm. Scope comprises the site's captured inputs/lets/earlier events, current boundary state/liabilities, and the selected arm's typed binders. The normal Await's unreceived event binders and later local variables are unavailable. Arm binders cannot shadow captures or each other. Ordinary static typing and financial statement rules continue to apply.

`refund` and `compensate` are distinct source/report classifications. They do **not** move assets, erase duties, invert a transfer or establish control by themselves. Every movement, fee and duty discharge must be an explicit statement. A refund is proposed for return from currently modeled controlled custody; compensation is a new forward action from the declared source of remedy. Authentic control, rights to discharge, funding and signed policy need the owner's future judgment. An annotation plus a caller-supplied balance or Boolean is not evidence of those facts.

No new deadline/nonexecution intrinsic is introduced. An arm may take local typed observations and guard on them. A deadline-only observation with unresolved outcome can leave its guard false and preserve pending duties. A true caller-supplied outcome flag enables only an explicitly unqualified local hypothetical execution. MPLR-010's reliable authenticated knowledge and late-delivery behavior remain unestablished until the designated owner supplies their binding contract.

## Transition semantics

Represent a local pending value as `P=(program,plan,action,site,captures,B,A)` where B is its complete boundary and A is cumulative local accounting. The initial recovery operation selects the arm from the **source declaration at that site**, reads captured values from P and evaluates on a working copy of B. It cannot accept an unrelated scenario that silently replaces B.

| Invocation outcome | Returned local control and state | Accounting and duties |
| --- | --- | --- |
| Guard false | Pending at the same action/site/captures/boundary; no public guard Step/segment or new effect | Add attempted guard work; retain committed work/effect prefix and complete positive residual duties |
| Guard true and body completes | Closed-by-recovery value for this workflow; no normal pending successor; return the complete recovery segment and post-boundary | Append only new committed effects; add successful handler work; derive residuals from complete post-boundary |
| Guard/body rejects | Retained pending value with original boundary/site/captures; no successful recovery segment/effects | Retain prior committed effects/fees/work; record attempted handler work and complete typed error; preserve prior residuals |
| Wrong profile/unsupported arm/input shape before evaluation | Defined local rejection; no recovery completion | No evaluation charge; original supplied local progress is retained; exact validation precedence needs owner review |

Success closes **the returned normal control path**, even if economic duties remain. A Closed value has no pending site and cannot be supplied to the typed resume/recover API. This is value-level lifecycle semantics, not revocation of all copies. As in current `/2`, a copied old input is untracked; this proposal supplies no global cancellation, lock, once-only consumption, proof, nonce, authority or race guarantee. An unrelated `start` action returning assets never creates Closed for the old P.

There is no global rollback. Earlier successful segments, transfers, charges and duties remain facts of the local modeled history. On a local failed handler only its unpublished working changes are discarded; earlier effects are not undone. Actual Midnight phase failures may retain new fees/effects, so this local atomic-failure model must not be lowered as a ledger rollback assumption. That mapping remains designated-owner unassessed.

Refund does not automatically discharge even a same-amount liability. Compensation need not eliminate the whole duty. Source explicitly describes what is discharged or newly owed; all untouched and positive residual liabilities remain after control closes. Recovery-complete means the selected finite handler ended, never that every economic obligation or external operation settled.

## Cumulative accounting and logical work

Propose `Accounting3={committed,attempted_total,effects}`. Both counters are canonical UInt128 strings, effects is the full ordered committed-effect prefix for this local workflow, and all arithmetic uses checked addition. It is local input data, not authenticated accounting evidence. Current `/2` persists only successful committed work, so the new accounting cannot be smuggled into a `/2` token.

For invocation i with attempted work t and successful handler work k:

```text
success: committed' = committed + k
         attempted_total' = attempted_total + t
         effects' = effects ++ new_segment.effects
false/rejected: committed' = committed
                attempted_total' = attempted_total + t
                effects' = effects
```

Successful invocation t=k. Normal `/3` start/resume uses the same cumulative equations. False observations and failed bodies therefore remain visible in total attempts. Their returned pending value has unchanged semantic boundary/captures but updated accounting; do not claim byte-identical continuation preservation as current `/2` does. Pre-evaluation rejection t=0. A malformed file that cannot yield a pending value cannot supply trustworthy prior counters; report unknown input accounting rather than invent zero cumulative history.

The effect prefix preserves financial history and duplicate ordered effects. Gross debits and fees derive per exact asset/account from those effects; refund credits never subtract earlier gross debit or fee charges. Liability creation/discharge and complete current residual state remain separate. No authority unit is added to money. Any caller-supplied prefix is explicitly unauthenticated until separately bound; this design claims no history soundness.

Reuse the ordinary per-statement schedule: one dispatch plus distinct reachable expression-node visits per statement, without short-circuit deletion or cross-statement cache discounts. The recovery clause's guard has one dispatch and its guard nodes, independently of the normal Await guard. On true, emit a declared recovery-gate Step with typed Bool true, then body Steps; on false emit no segment. No extra hidden handler dispatch, normal Await dispatch or exit-Await charge is added. No Await is permitted in the body. Thus `recovery_cost=1+nodes(arm.guard)+sum(body_statement_cost)`. Preserve costs already committed before the selected site; do not substitute a recovery cost for the prefix cost.

Plan3 includes every normal segment cost, recovery arm cost and each complete successful path total. Its declared committed-work ceiling is the maximum of the normal completion total and every prefix-plus-recovery total. Proposed logical cap admission must cover that maximum at start; the stored cap cannot be raised/reset by recovery. A lower per-invocation cap cannot raise it. Total attempted work is reported separately and can exceed a ceiling for successful committed work after repeated failed/false invocations; never label that ceiling a lifetime attempt/fee budget. Exact cap admission, provenance, refusal ordering and physical/resource enforcement need owner review before implementation; existing resource counters and native limits are untouched.

Retain current finite bounds as upper limits: total normal-plus-handler statements and gate nodes count within 1,024 statements per action; whole-source expressions within 8,192; declarations within256; Await sites16/action; each event binder list16 and capture layout256. Handler declarations count in declaration budgeting. Retain 65,536 trace-cell, 4MiB continuation and64KiB event ceilings including the cumulative effects. Reject an unrepresentable result rather than emit a truncated successful history or reset counters. Exact new decoder/admission integration and error ordering require owner review; these limits grant no command execution or larger resource budget.

## Exact representation and public API consequences

These are proposed interfaces, not code changes:

| Surface | Proposed change and compatibility consequence |
| --- | --- |
| Program | A separate `Program3/Action3/Statement3` family; `Await3` has ordered `RecoveryArm3 {kind,name,binders,guard,body}`. Ordinary statement payloads retain their declared meanings. No mutation of frozen `/1` decode wire or `/2` structures. |
| Expression arena | One global fresh-occurrence arena. Allocate normal Await guard, then each recovery guard/body in source order, then the later forward suffix. Recovery expressions are not deferred to invocation. Declaration metadata/binders allocate no nodes. New `/3` source gets its own IDs; do not transfer `/2` hashes/roots. |
| Trace | `Execution3` records `route:Forward` or `Recovery{site,arm,kind}` in addition to semantics/action/pre/post/steps/effects. Recovery gate has route-local index0 and raw `RecoveryGate3{arm,guard}`; body indices1..n retain exact raw Statements/ExprIds. Forward statement ordinals remain their lexical forward-list ordinals. Route+index identifies a Step; no fake forward source index. |
| Plan | New `StagePlan3` with explicit recovery capture layouts/costs and successful-path ceiling; no reuse of `/1` plan format bytes. Liability/account/asset discovery must traverse recovery bodies too, preserving complete untouched rows. |
| Pending | New `Pending3` contains program/plan identity fields, action/site/captures/boundary and Accounting3, stored cap, and an explicit pending state tag. Proposed wire format `moriarty-continuation/2`; `/2` decoders do not accept it. |
| Closed | `Closed3` contains closing boundary, Accounting3 and `ForwardComplete` or `RecoveryComplete{site,arm,kind}` disposition. It contains no resumable pending site; residual duties are preserved separately and in the boundary. It is a local output type, not a cancellation certificate. |
| Events | New normal `StageEvent3` format `moriarty-stage-event/2`; distinct `RecoveryEvent3 {format,action,site,arm,values}` format `moriarty-recovery-event/1`. No mode supplied by caller; kind comes from source. |
| Library | A separate `mori_core::recovery` public module exposing `load`, `start`, `resume`, `recover`, bounded decoders and `outcome_report`. `recover(&RecoveryProgram, Pending3, RecoveryEvent3, Option<UInt>) -> Result<RecoveryOutcome3,RecoveryRejection3>`. Only Pending3 is an input; no arbitrary action name/scenario replaces its control. |
| Result | `RecoveryOutcome3` is Waiting (including false), ForwardComplete, or RecoveryComplete. Success includes complete segment/control/accounting/residuals. `RecoveryRejection3` includes typed Error, invocation attempted work and retained Pending3 when input was available, with updated attempted total. Existing `/2` rejection has no replacement continuation; it remains unchanged. |
| Report | New `status` values waiting/complete/recovery-complete/rejected and explicit semantics `moriarty-core/3`. Keep qualification:`local-reference-stage`, authentication:`none`, consumption:`untracked`. Emit exact route, full segment, returned pending-or-closed control, accounting-before/after and complete residuals. New schema needs independent full DTO/key-set/byte freeze; no `/2` golden is inherited. |
| CLI | Existing `mori simulate` gains proposed mutually exclusive `--recover FILE --recovery-event FILE` only for the new profile. Conflict with resume, scenario/start action replacement, trace/target and signed/native options. Route reads the actual pending input; no separate host refund command. Core-only local path stays independent of SDK/provider/prover operations. |
| Targets | Source3 parsing/evaluation does not establish compiler support. Existing `/1`-/`2` lowerers, intent/native/target/replay consumers explicitly reject unsupported source3 rather than relabel it as `/2`. No private/recursive/ledger correspondence follows. |

Program/plan identity derivation and binding are owner-review prerequisites. No new hash/security recipe is supplied here. A source version name is a proposed namespace, not released support. A mechanical shared evaluator is implementation reuse, not an independent oracle.

## Evidence and decision boundary

The [worked examples](EXAMPLES.md) specify finite financial and work behavior; the [EARS proposal](PROPOSED-EARS.md) supplies candidate ordinary clauses and predicates. Neither file is an independent freeze. The [custody/review packet](CUSTODY-AND-REVIEW.md) identifies independent derivation and review gates. All examples are hypothetical local balances/observations; no public account is funded or controlled by this design.

Pending owner questions include authentic asset control and funding, intent/authority to select a handler or discharge a duty, reliable external classification/correlation/finality, ordinary and recovery races, replay/once-only behavior, history/completeness binding, cap/boundary provenance, all target phase outcomes and proof/native feasibility. None is answered by this ordinary author. Until owner review and two substantive non-author design votes agree on the exact candidate, implementation is blocked by this task's explicit design gate. Independent review/expectation preparation may proceed; no implementation stub is product acceptance.

All original MPLR, U0–U7, M1–M17, MC/SP/G, 119 UTXO, 11 first-contract, 18 PCD and full financial/composition obligations remain. In particular local closing cannot satisfy authenticated MPLR-005/010/017, native history/private composition or target settlement. Existing multi-condition tests stay labeled as such.



===== CURRENT COMPLETE RUST CODE | current/crates/mori-core/src/ir.rs | whole file code, only Rust comments removed and outside-literal whitespace collapsed; complete raw physical source supplied =====
use serde::{Deserialize, Serialize}; use std::collections::BTreeMap; pub const LANGUAGE: &str = "moriarty/1"; pub const SEMANTICS: &str = "moriarty-core/1"; pub const MAX_EXPRESSIONS: usize = 8192; pub const MAX_STATEMENTS: usize = 1024; pub const MAX_DECLARATIONS: usize = 256; pub const MAX_TRACE_CELLS: usize = 65_536;  #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)] pub struct UInt(pub u128); impl Serialize for UInt { fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_str(&self.0.to_string()) } } impl<'de> Deserialize<'de> for UInt { fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { let s = String::deserialize(deserializer)?; if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0')) { return Err(serde::de::Error::custom( "expected canonical unsigned decimal string", )); } s.parse().map(UInt).map_err(serde::de::Error::custom) } } #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)] pub enum Type { UInt, Bool, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(tag = "type", content = "value", deny_unknown_fields)] pub enum Value { UInt(UInt), Bool(bool), } impl Value { pub fn ty(&self) -> Type { match self { Self::UInt(_) => Type::UInt, Self::Bool(_) => Type::Bool, } } } pub type ExprId = u16; #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)] pub enum BinaryOp { Add, Sub, Mul, Eq, Ne, Lt, Le, Gt, Ge, And, Or, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(tag = "kind", deny_unknown_fields)] pub enum Expr { LiabilityPresent { name: String, }, LiabilityAmount { name: String, }, Literal { value: Value, }, Name { name: String, }, Binary { op: BinaryOp, lhs: ExprId, rhs: ExprId, }, Not { value: ExprId, }, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct StateDecl { pub name: String, pub ty: Type, pub initial: Value, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct InputDecl { pub name: String, pub ty: Type, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)] #[serde(deny_unknown_fields)] pub struct AssetId { pub network: String, pub issuer: String, pub reference: String, pub scale: u8, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct AssetDecl { pub name: String, pub identity: AssetId, }   #[derive(Debug, Clone, Serialize, PartialEq, Eq)] #[serde(tag = "kind", deny_unknown_fields)] pub enum Statement { Await { label: String, binders: Vec<InputDecl>, guard: ExprId, }, Let { name: String, value: ExprId, }, Requires { predicate: ExprId, }, Ensures { predicate: ExprId, }, Next { state: String, value: ExprId, }, Emit { name: String, value: ExprId, }, Transfer { asset: String, from: String, to: String, amount: ExprId, }, Fee { asset: String, from: String, to: String, amount: ExprId, }, Liability { name: String, asset: String, debtor: String, creditor: String, amount: ExprId, }, Discharge { name: String, amount: ExprId, }, } #[derive(Debug, Clone, Serialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Action { pub name: String, pub inputs: Vec<InputDecl>, pub statements: Vec<Statement>, } #[derive(Debug, Clone, Serialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Program { pub language: String, pub semantics: String, pub name: String, pub states: Vec<StateDecl>, pub assets: Vec<AssetDecl>, pub expressions: Vec<Expr>, pub actions: Vec<Action>, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Balance { pub account: String, pub asset: AssetId, pub amount: UInt, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Liability { pub name: String, pub asset: AssetId, pub debtor: String, pub creditor: String, pub amount: UInt, } #[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Scenario { pub state: BTreeMap<String, Value>, pub inputs: BTreeMap<String, Value>, pub balances: Vec<Balance>, pub liabilities: Vec<Liability>, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Snapshot { pub state: BTreeMap<String, Value>, pub balances: Vec<Balance>, pub liabilities: Vec<Liability>, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(tag = "kind", deny_unknown_fields)] pub enum Effect { State { name: String, before: Value, after: Value, }, Transfer { asset: AssetId, from: String, to: String, amount: UInt, }, Fee { asset: AssetId, from: String, to: String, amount: UInt, }, Liability { name: String, asset: AssetId, debtor: String, creditor: String, before: UInt, after: UInt, }, Emit { name: String, value: Value, }, } #[derive(Debug, Clone, Serialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Step { pub index: u16, pub statement: Statement, pub value: Option<Value>, pub snapshot: Snapshot, pub effects: Vec<Effect>, } #[derive(Debug, Clone, Serialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Execution { pub semantics: String, pub action: String, pub pre: Snapshot, pub post: Snapshot, pub steps: Vec<Step>, pub effects: Vec<Effect>, }    pub mod v1 { use super::{AssetDecl, Effect, Expr, ExprId, InputDecl, Snapshot, StateDecl, Value}; use serde::{Deserialize, Serialize}; #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(tag = "kind", deny_unknown_fields)] pub enum Statement { Let { name: String, value: ExprId, }, Requires { predicate: ExprId, }, Ensures { predicate: ExprId, }, Next { state: String, value: ExprId, }, Emit { name: String, value: ExprId, }, Transfer { asset: String, from: String, to: String, amount: ExprId, }, Fee { asset: String, from: String, to: String, amount: ExprId, }, Liability { name: String, asset: String, debtor: String, creditor: String, amount: ExprId, }, Discharge { name: String, amount: ExprId, }, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Action { pub name: String, pub inputs: Vec<InputDecl>, pub statements: Vec<Statement>, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Program { pub language: String, pub semantics: String, pub name: String, pub states: Vec<StateDecl>, pub assets: Vec<AssetDecl>, pub expressions: Vec<Expr>, pub actions: Vec<Action>, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Step { pub index: u16, pub statement: Statement, pub value: Option<Value>, pub snapshot: Snapshot, pub effects: Vec<Effect>, } #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] #[serde(deny_unknown_fields)] pub struct Execution { pub semantics: String, pub action: String, pub pre: Snapshot, pub post: Snapshot, pub steps: Vec<Step>, pub effects: Vec<Effect>, } } impl From<v1::Statement> for Statement { fn from(s: v1::Statement) -> Self { use v1::Statement as W; match s { W::Let { name, value } => Self::Let { name, value }, W::Requires { predicate } => Self::Requires { predicate }, W::Ensures { predicate } => Self::Ensures { predicate }, W::Next { state, value } => Self::Next { state, value }, W::Emit { name, value } => Self::Emit { name, value }, W::Transfer { asset, from, to, amount, } => Self::Transfer { asset, from, to, amount, }, W::Fee { asset, from, to, amount, } => Self::Fee { asset, from, to, amount, }, W::Liability { name, asset, debtor, creditor, amount, } => Self::Liability { name, asset, debtor, creditor, amount, }, W::Discharge { name, amount } => Self::Discharge { name, amount }, } } } impl From<v1::Action> for Action { fn from(a: v1::Action) -> Self { Self { name: a.name, inputs: a.inputs, statements: a.statements.into_iter().map(Into::into).collect(), } } } impl From<v1::Program> for Program { fn from(p: v1::Program) -> Self { Self { language: p.language, semantics: p.semantics, name: p.name, states: p.states, assets: p.assets, expressions: p.expressions, actions: p.actions.into_iter().map(Into::into).collect(), } } } impl From<v1::Step> for Step { fn from(s: v1::Step) -> Self { Self { index: s.index, statement: s.statement.into(), value: s.value, snapshot: s.snapshot, effects: s.effects, } } } impl From<v1::Execution> for Execution { fn from(e: v1::Execution) -> Self { Self { semantics: e.semantics, action: e.action, pre: e.pre, post: e.post, steps: e.steps.into_iter().map(Into::into).collect(), effects: e.effects, } } }   macro_rules! decode_via_v1 { ($($ty:ident),*) => {$( impl<'de> Deserialize<'de> for $ty { fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> { v1::$ty::deserialize(decoder).map(Self::from) } } )*}; } decode_via_v1!(Statement, Action, Program, Step, Execution); #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)] #[error("{code}: {message}")] pub struct Error { pub code: String, pub message: String, pub offset: Option<u32>, pub step: Option<u16>, } impl Error { pub(crate) fn new(code: &str, message: impl Into<String>) -> Self { Self { code: format!("mori::core::{code}"), message: message.into(), offset: None, step: None, } } }


===== CURRENT COMPLETE RUST CODE | current/crates/mori-core/src/stage/mod.rs | whole file code, only Rust comments removed and outside-literal whitespace collapsed; complete raw physical source supplied =====
mod admit; mod cost; pub mod decode; mod digest; #[cfg(feature = "core2-disjoint-draft")] pub mod disjoint; mod plan; mod profile; mod report; mod validate; pub use crate::LOGICAL_COST_PROFILE_2; pub use admit::{resume, start}; pub use plan::{ActionPlan, AwaitSite, CaptureSlot, Origin, StageBinder, StagePlan}; pub use profile::{require_core1, require_core2}; pub use report::outcome_report; use crate::{Error, Execution, Liability, Program, Snapshot, UInt, Value}; use serde::{Deserialize, Serialize}; pub const LANGUAGE_2: &str = "moriarty/2"; pub const SEMANTICS_2: &str = "moriarty-core/2"; pub const STAGE_PLAN_FORMAT: &str = "moriarty-stage-plan/1"; pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1"; pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1"; pub const MAX_AWAITS_PER_ACTION: usize = 16; pub const MAX_EVENT_BINDERS: usize = 16; pub const MAX_CAPTURE_SLOTS: usize = 256; pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576; pub const MAX_EVENT_BYTES: usize = 65_536;  pub const STAGE_LABELS: [(&str, &str); 3] = [ ("qualification", "local-reference-stage"), ("authentication", "none"), ("consumption", "untracked"), ];    #[derive(Debug, Clone)] pub struct StagedProgram { program: Program, plan: StagePlan, program_hash: [u8; 32], plan_hash: [u8; 32], } impl StagedProgram { pub fn program(&self) -> &Program { &self.program } pub fn plan(&self) -> &StagePlan { &self.plan } pub fn program_hash(&self) -> [u8; 32] { self.program_hash } pub fn plan_hash(&self) -> [u8; 32] { self.plan_hash }  pub(crate) fn action_index(&self, name: &str) -> Option<usize> { self.program.actions.iter().position(|a| a.name == name) } }   pub fn load(src: &str) -> Result<StagedProgram, Error> { let program = crate::parser::parse_v2(src)?; profile::require_core2(&program)?; validate::validate_v2(&program)?; let mut plan = plan::build(&program)?; validate::plan_invariants(&program, &plan)?; cost::fill(&program, &mut plan)?; validate::cost_invariants(&program, &plan)?; let program_hash = digest::program_hash(&program)?; plan.program_hash = digest::hex(&program_hash); let plan_hash = digest::plan_hash(&plan)?; Ok(StagedProgram { program, plan, program_hash, plan_hash, }) } #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct NamedValue { pub name: String, pub value: Value, } #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct StageLogical { pub profile: String,  #[serde(deserialize_with = "Option::deserialize")] pub cap: Option<UInt>, pub committed: UInt, }   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct Continuation { pub format: String, pub program_hash: String, pub plan_hash: String, pub action: String, pub site: u16, pub captures: Vec<NamedValue>, pub logical: StageLogical, pub boundary: Snapshot, } #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct StageEvent { pub format: String, pub action: String, pub label: String, pub values: Vec<NamedValue>, }   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct PendingFrame { pub site: u16, pub captures: Vec<NamedValue>, } impl PendingFrame { pub(crate) fn idle() -> Self { Self { site: 0, captures: Vec::new(), } } }   #[derive(Debug, Clone, PartialEq, Eq, Serialize)] pub struct StageSegment { pub action: String, pub frame_pre: PendingFrame, pub frame_post: PendingFrame, pub execution: Execution, }    #[allow(clippy::large_enum_variant)] #[derive(Debug, Clone, PartialEq, Eq, Serialize)] pub enum StageOutcome { Waiting { advanced: bool, segment: Option<StageSegment>, continuation: Continuation, attempted: UInt, residual: Vec<Liability>, }, Complete { segment: StageSegment, committed: UInt, attempted: UInt, residual: Vec<Liability>, }, }  #[derive(Debug, Clone, PartialEq, Eq, Serialize, thiserror::Error)] #[error("{error}")] pub struct StageRejection { pub error: Error, pub attempted: UInt, pub committed: UInt, }  pub(crate) fn residual(snapshot: &Snapshot) -> Vec<Liability> { snapshot .liabilities .iter() .filter(|l| l.amount.0 > 0) .cloned() .collect() }


===== CURRENT COMPLETE RUST CODE | current/crates/mori-core/src/stage/admit.rs | whole file code, only Rust comments removed and outside-literal whitespace collapsed; complete raw physical source supplied =====
use super::{ CONTINUATION_FORMAT, Continuation, NamedValue, PendingFrame, STAGE_EVENT_FORMAT, StageEvent, StageLogical, StageOutcome, StageRejection, StageSegment, StagedProgram, cost, decode, digest, profile, residual, }; use crate::runtime::{self, SegmentEntry, SegmentRun}; use crate::{Error, LOGICAL_COST_PROFILE_2, Scenario, UInt}; fn rejected(error: Error, attempted: UInt, committed: UInt) -> StageRejection { StageRejection { error, attempted, committed, } }  fn layout_matches<'a, T: 'a>( values: &[NamedValue], slots: impl ExactSizeIterator<Item = &'a T>, shape: impl Fn(&'a T) -> (&'a str, crate::Type), ) -> bool { values.len() == slots.len() && values.iter().zip(slots).all(|(value, slot)| { let (name, ty) = shape(slot); value.name == name && value.value.ty() == ty }) }   fn premise(error: Error) -> Error { if error.code == "mori::core::meter" { let mut violated = Error::new("stage_logical", "static cost premise violated"); violated.step = error.step; violated } else { error } }   fn emit_checked(sp: &StagedProgram, action: usize, c: &Continuation) -> Result<(), Error> { let unresumable = || { Error::new( "bound", "Continuation exceeds the resume decoder limits or cannot be resumed.", ) }; let text = serde_json::to_string(c).map_err(|_| unresumable())?; if text.len() > decode::MAX_CONTINUATION_BYTES { return Err(unresumable()); } let decoded = decode::continuation(&text).map_err(|_| unresumable())?; if decoded != *c { return Err(unresumable()); } decode::canonical_boundary(sp, action, &decoded.captures, &decoded.boundary) .map_err(|_| unresumable()) }   struct Admitted<'a> { sp: &'a StagedProgram, action: usize, entry: u16,  committed: UInt, stored_cap: Option<UInt>, frame_pre: PendingFrame, } impl Admitted<'_> { fn finish( self, run: Result<SegmentRun, Error>, meter: &crate::policy::Meter, unchanged: Option<Continuation>, ) -> Result<StageOutcome, StageRejection> { let Admitted { sp, action: index, entry, committed: kept, stored_cap, frame_pre, } = self; let attempted = meter.used().logical; let fail = |error: Error| rejected(error, attempted, kept); let plan = &sp.plan().actions[index]; let action = &sp.program().actions[index].name; let segment = usize::from(entry); match run.map_err(|error| fail(premise(error)))? { SegmentRun::NotAdvanced => {  let Some(c) = unchanged else { return Err(fail(Error::new( "stage_site", "A start segment has no guard to observe.", ))); }; let dispatch = cost::dispatch_cost(sp.program(), plan, entry).map_err(fail)?; cost::require_static(meter, dispatch).map_err(fail)?; emit_checked(sp, index, &c).map_err(fail)?; let owed = residual(&c.boundary); Ok(StageOutcome::Waiting { advanced: false, segment: None, continuation: c, attempted, residual: owed, }) } SegmentRun::Ran { execution, exit_site, captures_out, } => { let planned = *plan.segment_cost.get(segment).ok_or_else(|| { fail(Error::new( "stage_logical", "Segment is absent from the stage plan.", )) })?; cost::require_static(meter, planned).map_err(fail)?; let committed = cost::committed_after(plan, segment).map_err(fail)?; let expected_exit = if segment == plan.sites.len() { 0 } else { entry + 1 }; if exit_site != expected_exit { return Err(fail(Error::new( "stage_site", "Segment exit differs from the stage plan.", ))); } let frame_post = PendingFrame { site: exit_site, captures: captures_out, }; if exit_site == 0 { if !frame_post.captures.is_empty() || committed != plan.plan_total { return Err(fail(Error::new( "stage_logical", "Completed work differs from the action plan total.", ))); } let owed = residual(&execution.post); return Ok(StageOutcome::Complete { segment: StageSegment { action: action.clone(), frame_pre, frame_post, execution: *execution, }, committed, attempted, residual: owed, }); } let site = &plan.sites[usize::from(exit_site) - 1]; if !layout_matches(&frame_post.captures, site.captures.iter(), |s| { (s.name.as_str(), s.ty) }) { return Err(fail(Error::new( "stage_capture", "Segment captures differ from the next site layout.", ))); } if plan.committed_before.get(usize::from(exit_site)) != Some(&committed) { return Err(fail(Error::new( "stage_logical", "Committed work differs from the stage plan.", ))); } let continuation = Continuation { format: CONTINUATION_FORMAT.into(), program_hash: digest::hex(&sp.program_hash()), plan_hash: digest::hex(&sp.plan_hash()), action: action.clone(), site: exit_site, captures: frame_post.captures.clone(), logical: StageLogical { profile: LOGICAL_COST_PROFILE_2.into(), cap: stored_cap, committed, }, boundary: execution.post.clone(), }; emit_checked(sp, index, &continuation).map_err(fail)?; let owed = residual(&execution.post); Ok(StageOutcome::Waiting { advanced: true, segment: Some(StageSegment { action: action.clone(), frame_pre, frame_post, execution: *execution, }), continuation, attempted, residual: owed, }) } } } }    pub fn start( sp: &StagedProgram, action: &str, scenario: Scenario, cap: Option<UInt>, ) -> Result<StageOutcome, StageRejection> { let refuse = |error: Error| rejected(error, UInt(0), UInt(0)); let index = sp .action_index(action) .ok_or_else(|| refuse(Error::new("action", format!("Unknown action {action:?}."))))?; let selected = &sp.program().actions[index]; let plan = sp .plan() .actions .get(index) .filter(|plan| plan.action == action) .ok_or_else(|| { refuse(Error::new( "stage_site", "Action is absent from the stage plan.", )) })?; let pre = runtime::scenario_snapshot(sp.program(), selected, &scenario).map_err(refuse)?; let limit = cost::start_preflight(plan, cap).map_err(refuse)?; let mut inputs = Vec::with_capacity(selected.inputs.len()); for input in &selected.inputs { let value = scenario.inputs.get(&input.name).cloned().ok_or_else(|| { refuse(Error::new( "input", format!("Missing or wrong type for input {:?}.", input.name), )) })?; inputs.push(NamedValue { name: input.name.clone(), value, }); }   if !plan.sites.is_empty() { decode::canonical_boundary(sp, index, &inputs, &pre).map_err(refuse)?; } let mut meter = cost::segment_meter(limit); let run = runtime::run_segment( sp, index, SegmentEntry::Start { inputs: &inputs }, pre, &mut meter, ); Admitted { sp, action: index, entry: 0, committed: UInt(0), stored_cap: cap, frame_pre: PendingFrame::idle(), } .finish(run, &meter, None) }     pub fn resume( sp: &StagedProgram, c: Continuation, e: StageEvent, cli_action: Option<&str>, cli_cap: Option<UInt>, ) -> Result<StageOutcome, StageRejection> {  let kept = c.logical.committed; let refuse = |code: &str, message: &str| rejected(Error::new(code, message), UInt(0), kept); let refused = |error: Error| rejected(error, UInt(0), kept);  if cli_action.is_some_and(|selected| selected != c.action) { return Err(refuse( "stage_site", "Selected action differs from the continuation action.", )); }  if c.format != CONTINUATION_FORMAT || c.logical.profile != LOGICAL_COST_PROFILE_2 { return Err(refuse( "stage_format", "Unsupported continuation format or logical profile.", )); }  profile::require_core2(sp.program()).map_err(refused)?; if !digest::matches(&c.program_hash, &sp.program_hash()) || !digest::matches(&c.plan_hash, &sp.plan_hash()) { return Err(refuse( "stage_program", "Continuation does not belong to this program and stage plan.", )); }  let Some(index) = sp.action_index(&c.action) else { return Err(refuse("stage_site", "Continuation action does not exist.")); }; let Some(plan) = sp .plan() .actions .get(index) .filter(|plan| plan.action == c.action) else { return Err(refuse( "stage_site", "Action is absent from the stage plan.", )); }; if c.site == 0 || usize::from(c.site) > plan.sites.len() { return Err(refuse( "stage_site", "Continuation site is not a pending await of this action.", )); } let site = &plan.sites[usize::from(c.site) - 1];  if !layout_matches(&c.captures, site.captures.iter(), |s| { (s.name.as_str(), s.ty) }) { return Err(refuse( "stage_capture", "Captures differ from the pending site layout.", )); }  if e.format != STAGE_EVENT_FORMAT || e.action != c.action || e.label != site.label || !layout_matches(&e.values, site.binders.iter(), |b| (b.name.as_str(), b.ty)) { return Err(refuse( "stage_event", "Event does not match the pending await.", )); }  let limit = cost::resume_preflight(plan, c.site, &c.logical, cli_cap).map_err(refused)?;  decode::canonical_boundary(sp, index, &c.captures, &c.boundary).map_err(refused)?; let mut meter = cost::segment_meter(limit); let run = runtime::run_segment( sp, index, SegmentEntry::Resume { site: c.site, captures: &c.captures, event: &e.values, }, c.boundary.clone(), &mut meter, ); Admitted { sp, action: index, entry: c.site, committed: kept, stored_cap: c.logical.cap, frame_pre: PendingFrame { site: c.site, captures: c.captures.clone(), }, } .finish(run, &meter, Some(c)) }


===== CURRENT COMPLETE RUST CODE | current/crates/mori-core/src/stage/report.rs | whole file code, only Rust comments removed and outside-literal whitespace collapsed; complete raw physical source supplied =====
use super::{STAGE_LABELS, StageOutcome, StageRejection}; use serde_json::{Value, json};   pub fn outcome_report(result: Result<StageOutcome, StageRejection>) -> Value { let mut report = match result { Ok(StageOutcome::Waiting { advanced, segment, continuation, attempted, residual, }) => { let committed = continuation.logical.committed; json!({"status":"waiting","advanced":advanced,"segment":segment, "continuation":continuation,"attempted":attempted, "committed":committed,"residual":residual}) } Ok(StageOutcome::Complete { segment, committed, attempted, residual, }) => json!({"status":"complete","segment":segment,"attempted":attempted, "committed":committed,"residual":residual}), Err(rejected) => json!({"status":"rejected","error":rejected.error, "attempted":rejected.attempted,"committed":rejected.committed}), }; let object = report .as_object_mut() .expect("stage reports are JSON objects"); for (name, value) in STAGE_LABELS { object.insert(name.into(), Value::String(value.into())); } report }


===== CURRENT COMPLETE RUST CODE | current/crates/mori-core/src/stage/plan.rs | whole file code, only Rust comments removed and outside-literal whitespace collapsed; complete raw physical source supplied =====
use crate::{Error, InputDecl, Program, Statement, Type, UInt}; use serde::{Deserialize, Serialize}; use std::collections::BTreeMap; #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] pub enum Origin { Input, Let, Event, } #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct CaptureSlot { pub name: String, pub ty: Type, pub origin: Origin, pub ordinal: u16, } #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct StageBinder { pub name: String, pub ty: Type, } impl From<&InputDecl> for StageBinder { fn from(value: &InputDecl) -> Self { Self { name: value.name.clone(), ty: value.ty, } } } #[derive(Debug, Clone, PartialEq, Eq, Serialize)] pub struct AwaitSite { pub site: u16, pub ordinal: u16, pub label: String, pub binders: Vec<StageBinder>, pub captures: Vec<CaptureSlot>, } #[derive(Debug, Clone, PartialEq, Eq, Serialize)] pub struct ActionPlan { pub action: String, pub statement_count: u16, pub sites: Vec<AwaitSite>, pub segment_cost: Vec<UInt>, pub committed_before: Vec<UInt>, pub plan_total: UInt, } #[derive(Debug, Clone, PartialEq, Eq, Serialize)] pub struct StagePlan { pub format: String, pub program_hash: String, pub actions: Vec<ActionPlan>, }  pub(crate) fn build(program: &Program) -> Result<StagePlan, Error> { let mut actions = Vec::new(); for action in &program.actions { let mut types: BTreeMap<_, _> = program .states .iter() .map(|state| (state.name.clone(), state.ty)) .collect(); let mut captures = Vec::new(); for (ordinal, input) in action.inputs.iter().enumerate() { types.insert(input.name.clone(), input.ty); captures.push(CaptureSlot { name: input.name.clone(), ty: input.ty, origin: Origin::Input, ordinal: ordinal as u16, }); } let mut sites = Vec::new(); for (ordinal, statement) in action.statements.iter().enumerate() { match statement { Statement::Let { name, value } => { let ty = crate::runtime::expr_type(program, *value, &types, 0)?; types.insert(name.clone(), ty); captures.push(CaptureSlot { name: name.clone(), ty, origin: Origin::Let, ordinal: ordinal as u16, }); } Statement::Await { label, binders, .. } => { sites.push(AwaitSite { site: sites.len() as u16 + 1, ordinal: ordinal as u16, label: label.clone(), binders: binders.iter().map(StageBinder::from).collect(), captures: captures.clone(), }); for binder in binders { types.insert(binder.name.clone(), binder.ty); captures.push(CaptureSlot { name: binder.name.clone(), ty: binder.ty, origin: Origin::Event, ordinal: ordinal as u16, }); } } _ => {} } } actions.push(ActionPlan { action: action.name.clone(), statement_count: action.statements.len() as u16, sites, segment_cost: Vec::new(), committed_before: Vec::new(), plan_total: UInt(0), }); } Ok(StagePlan { format: "moriarty-stage-plan/1".into(), program_hash: String::new(), actions, }) }


===== L04 COMPLETE FINANCIAL OBJECTS | L04-LOSSLESS-TREES.json | lossless structural interning, equality roundtrip checked; whole raw financial objects supplied =====
{"encoding":"BOTTOM_UP_JSON_TREE_INTERNING/1","rule":"Every sole {\"$ref\":n} object expands recursively to pool[n]. Both complete original JSON objects are reconstructed exactly, including ALL success/failure/unknown variants; no field or array element omitted. Objects are an interning transport, not financial effect aggregation. Read full physical JSON if this representation is not fully inspected.","pool":[[],{"buyer_net_debit_after_refund":"4","controlled_principal_refunded":"60","fee_recipient_closing":"7","fee_recipient_opening":"3","gross_buyer_funding_and_fee":"64","merchant_net_receipt":"0","opening_BOND_total":"20","opening_PAY_total":"228","preserved_escrow_PAY":"5","refund_buyer_receipt":"60","refund_duty_closing":"present-zero","refund_duty_created":"60","refund_duty_discharged":"60","refund_gross_escrow_debit":"60","remedy_residual":{"$ref":0},"retained_fee":"4","unrelated_audit_BOND_duty":"4"},{"issuer":"independent-recovery-custody-refund","network":"local","reference":"BOND","scale":0},{"account":"auditor","amount":"0","asset":{"$ref":2}},{"account":"escrow","amount":"7","asset":{"$ref":2}},{"account":"observer","amount":"13","asset":{"$ref":2}},{"issuer":"independent-recovery-custody-refund","network":"local","reference":"PAY","scale":0},{"account":"buyer","amount":"200","asset":{"$ref":6}},{"account":"escrow","amount":"5","asset":{"$ref":6}},{"account":"merchant","amount":"11","asset":{"$ref":6}},{"account":"observer","amount":"9","asset":{"$ref":6}},{"account":"treasury","amount":"3","asset":{"$ref":6}},[{"$ref":3},{"$ref":4},{"$ref":5},{"$ref":7},{"$ref":8},{"$ref":9},{"$ref":10},{"$ref":11}],{"amount":"4","asset":{"$ref":2},"creditor":"auditor","debtor":"observer","name":"audit"},[{"$ref":13}],{"type":"UInt","value":"77"},{"marker":{"$ref":15}},{"balances":{"$ref":12},"liabilities":{"$ref":14},"state":{"$ref":16}},{"label":"opening","new_effects":{"$ref":0},"snapshot":{"$ref":17}},{"amount":"4","asset":{"$ref":6},"from":"buyer","kind":"Fee","to":"treasury"},[{"$ref":19}],{"account":"buyer","amount":"196","asset":{"$ref":6}},{"account":"treasury","amount":"7","asset":{"$ref":6}},[{"$ref":3},{"$ref":4},{"$ref":5},{"$ref":21},{"$ref":8},{"$ref":9},{"$ref":10},{"$ref":22}],{"balances":{"$ref":23},"liabilities":{"$ref":14},"state":{"$ref":16}},{"label":"fee retained","new_effects":{"$ref":20},"snapshot":{"$ref":24}},{"amount":"60","asset":{"$ref":6},"from":"buyer","kind":"Transfer","to":"escrow"},[{"$ref":26}],{"account":"buyer","amount":"136","asset":{"$ref":6}},{"account":"escrow","amount":"65","asset":{"$ref":6}},[{"$ref":3},{"$ref":4},{"$ref":5},{"$ref":28},{"$ref":29},{"$ref":9},{"$ref":10},{"$ref":22}],{"balances":{"$ref":30},"liabilities":{"$ref":14},"state":{"$ref":16}},{"label":"funding/payment committed","new_effects":{"$ref":27},"snapshot":{"$ref":31}},{"after":"60","asset":{"$ref":6},"before":"0","creditor":"buyer","debtor":"escrow","kind":"Liability","name":"custody-refund"},[{"$ref":33}],{"amount":"60","asset":{"$ref":6},"creditor":"buyer","debtor":"escrow","name":"custody-refund"},[{"$ref":13},{"$ref":35}],{"balances":{"$ref":30},"liabilities":{"$ref":36},"state":{"$ref":16}},{"label":"explicit remedy duty created; await entered","new_effects":{"$ref":34},"snapshot":{"$ref":37}},{"label":"condition not met; no financial transition","new_effects":{"$ref":0},"snapshot":{"$ref":37}},{"amount":"60","asset":{"$ref":6},"from":"escrow","kind":"Transfer","to":"buyer"},[{"$ref":40}],{"balances":{"$ref":23},"liabilities":{"$ref":36},"state":{"$ref":16}},{"label":"proposed new remedy transfer","new_effects":{"$ref":41},"snapshot":{"$ref":42}},{"after":"0","asset":{"$ref":6},"before":"60","creditor":"buyer","debtor":"escrow","kind":"Liability","name":"custody-refund"},[{"$ref":44}],{"amount":"0","asset":{"$ref":6},"creditor":"buyer","debtor":"escrow","name":"custody-refund"},[{"$ref":13},{"$ref":46}],{"balances":{"$ref":23},"liabilities":{"$ref":47},"state":{"$ref":16}},{"label":"proposed explicit duty discharge","new_effects":{"$ref":45},"snapshot":{"$ref":48}},[{"$ref":18},{"$ref":25},{"$ref":32},{"$ref":38},{"$ref":39},{"$ref":43},{"$ref":49}],{"BOND":{"$ref":2},"PAY":{"$ref":6}},{"principal":60,"retainedFee":4},{"refundAmount":60},{"effects":{"$ref":0},"financial_post":{"$ref":37},"financial_pre":{"$ref":37},"known_attempted":"2","known_committed":"7","liabilities_retained":{"$ref":36},"meaning":"Local false Bool only; not authenticated failure/nonexecution/cancellation evidence"},[{"$ref":35}],{"audit_BOND":"4 + creation0 + accrual0 - discharge0 =4","custody-refund":"absent + creation60 + accrual0 - discharge60 =0; present-zero retained when zero"},{"actual_reachability":"UNSPECIFIED","attempted":"UNSPECIFIED","committed":"UNSPECIFIED","conditional_financial_post":{"$ref":42},"conditional_new_effects":{"$ref":41},"duties_retained":{"$ref":36},"explicit_case_premise":"Only if the selected future recovery phase model permits this prefix to commit; current /2 recovery phase policy not selected","required_rule":"Do not infer discharge from payment or replay the transfer as if absent; outcome/next remedy policy remains unspecified","terminal_or_late_event_relation":"UNSPECIFIED"},{"admission_or_error_precedence":"UNSPECIFIED","attempted":"UNSPECIFIED","duties_retained":{"$ref":36},"explicit_case_premise":"No remedy transfer, fee, or discharge has occurred. This is a documentary boundary, not an assumed runtime atomicity/refusal rule.","financial_post":{"$ref":37},"financial_pre":{"$ref":37},"new_effects":{"$ref":0},"recovery_status_wire":"UNSPECIFIED"},["mint","burn","dedicated accrual","division/rounding/dust","implicit netting of duty into asset supply"],[{"$ref":19},{"$ref":26},{"$ref":33},{"$ref":40},{"$ref":44}],["Recovery transition binding/consumption of original pending frame and ordinary body","Terminal/cancelled/recovered versus still-pending result and treatment of later original accepted event","Recovery report/DTO/semantic-version identifiers, fields and byte rendering","Recovery dispatch/overhead/attempted and committed lifetime convention, cap/refusal policy","Per-phase atomicity, failure fees and partial-effect retention","Evidence/correlation/finality premises; no timeout-only inference","Authority/signatures/asset-control validation are unassessed, not established by balances"],["fee2","funding/payment transfer2","liability2","await entry1","false guard2","remedy transfer2","remedy discharge2"],{"arithmetic_sum_of_chosen_operation_work":"7+2+4=13; NOT a normative recovery lifetime counter","external_or_native_fees":"UNSPECIFIED","failure_attempted":"UNSPECIFIED","known_profile_for_existing_ordinary_primitives":"moriarty-reference-logical-cost/2","normal_false_observation":"dispatch1+guard Name1 =2; committed stays7","ordinary_remedy_body":"transfer dispatch1+Name1 + discharge dispatch1+Name1 =4, conditional on ordinary body acceptance","per_ordinary_statement_work":{"$ref":62},"recovery_cumulative_attempted":"UNSPECIFIED","recovery_cumulative_committed":"UNSPECIFIED","recovery_overhead":"UNSPECIFIED","separate_ordinary_action_committed":"4 for that standalone action only; not a recovery append policy","start":"fee dispatch1+Name1 + transfer dispatch1+Name1 + liability dispatch1+Name1 + await entry1 =7"},{"accounting":{"$ref":1},"action_source":"primitive-actions.proposed.mori","all_prefix_frames":{"$ref":50},"artifact_role":"Custodian-authored proposed normative economic example; this wrapper is documentary, not a Mori wire schema","assets":{"$ref":51},"chosen_inputs":{"$ref":52},"chosen_remedy_body_inputs":{"$ref":53},"closing_duties":{"$ref":47},"complete_initial":{"$ref":17},"complete_pending":{"$ref":37},"condition_not_met_outcome":{"$ref":54},"created_duties":{"$ref":55},"discharged_amount":"60","duty_evolution":{"$ref":56},"failure_after_remedy_transfer_before_discharge":{"$ref":57},"failure_before_any_new_financial_effect":{"$ref":58},"financial_policy":"Return only still-controlled custody principal60, retain already charged fee4 and unrelated duty","inapplicable":{"$ref":59},"no_product_execution":true,"opening_duties":{"$ref":14},"ordered_history":{"$ref":60},"ordinary_remedy_body_action":"refundControlled","ordinary_start_action":"fund","preservation":"All other exact-asset balance cells, marker, unrelated BOND audit duty4 and earlier committed effect history retained","proposed_successful_recovery_frame":{"$ref":48},"residual_positive_duties":{"$ref":14},"specific_pending_frame_gap":"Original pending custody action still captures principal60 and its source suffix pays merchant60; original compensation action still captures remedy20 while proposed residual is8. No existing ordinary remedy action alone is asserted to close/update either pending frame or alter that suffix. A recovery binding/late-event rule is required.","status":"PROPOSED financial policy; no actual recovery implementation/adoption/consumer observation claimed","unresolved":{"$ref":61},"work":{"$ref":63}},{"buyer_net_debit_after_compensation":"31","fee_recipient_closing":"5","fee_recipient_opening":"2","new_compensation_buyer_net":"12","new_compensation_reserve_gross":"12","not_refunded":"Original supplier payment40 remains a historical transfer; compensation12 is new reserve spending, not its inverse.","opening_BOND_total":"20","opening_PAY_total":"249","original_buyer_gross_payment_and_fee":"43","original_payment_gross":"40","original_payment_recipient_net":"40","remedy_closing":"8","remedy_created":"20","remedy_discharged":"12","remedy_opening":"absent","reserve_PAY_closing":"18","reserve_PAY_opening":"30","retained_fee":"3","supplier_closing_preserving_payment":"48","supplier_original_opening":"8","unrelated_audit_BOND_duty":"4"},{"issuer":"independent-recovery-forward-compensation","network":"local","reference":"BOND","scale":0},{"account":"auditor","amount":"0","asset":{"$ref":66}},{"account":"observer","amount":"13","asset":{"$ref":66}},{"account":"reserve","amount":"7","asset":{"$ref":66}},{"issuer":"independent-recovery-forward-compensation","network":"local","reference":"PAY","scale":0},{"account":"buyer","amount":"200","asset":{"$ref":70}},{"account":"observer","amount":"9","asset":{"$ref":70}},{"account":"reserve","amount":"30","asset":{"$ref":70}},{"account":"supplier","amount":"8","asset":{"$ref":70}},{"account":"treasury","amount":"2","asset":{"$ref":70}},[{"$ref":67},{"$ref":68},{"$ref":69},{"$ref":71},{"$ref":72},{"$ref":73},{"$ref":74},{"$ref":75}],{"amount":"4","asset":{"$ref":66},"creditor":"auditor","debtor":"observer","name":"audit"},[{"$ref":77}],{"type":"UInt","value":"91"},{"marker":{"$ref":79}},{"balances":{"$ref":76},"liabilities":{"$ref":78},"state":{"$ref":80}},{"label":"opening","new_effects":{"$ref":0},"snapshot":{"$ref":81}},{"amount":"3","asset":{"$ref":70},"from":"buyer","kind":"Fee","to":"treasury"},[{"$ref":83}],{"account":"buyer","amount":"197","asset":{"$ref":70}},{"account":"treasury","amount":"5","asset":{"$ref":70}},[{"$ref":67},{"$ref":68},{"$ref":69},{"$ref":85},{"$ref":72},{"$ref":73},{"$ref":74},{"$ref":86}],{"balances":{"$ref":87},"liabilities":{"$ref":78},"state":{"$ref":80}},{"label":"fee retained","new_effects":{"$ref":84},"snapshot":{"$ref":88}},{"amount":"40","asset":{"$ref":70},"from":"buyer","kind":"Transfer","to":"supplier"},[{"$ref":90}],{"account":"buyer","amount":"157","asset":{"$ref":70}},{"account":"supplier","amount":"48","asset":{"$ref":70}},[{"$ref":67},{"$ref":68},{"$ref":69},{"$ref":92},{"$ref":72},{"$ref":73},{"$ref":93},{"$ref":86}],{"balances":{"$ref":94},"liabilities":{"$ref":78},"state":{"$ref":80}},{"label":"funding/payment committed","new_effects":{"$ref":91},"snapshot":{"$ref":95}},{"after":"20","asset":{"$ref":70},"before":"0","creditor":"buyer","debtor":"reserve","kind":"Liability","name":"remedy"},[{"$ref":97}],{"amount":"20","asset":{"$ref":70},"creditor":"buyer","debtor":"reserve","name":"remedy"},[{"$ref":77},{"$ref":99}],{"balances":{"$ref":94},"liabilities":{"$ref":100},"state":{"$ref":80}},{"label":"explicit remedy duty created; await entered","new_effects":{"$ref":98},"snapshot":{"$ref":101}},{"label":"condition not met; no financial transition","new_effects":{"$ref":0},"snapshot":{"$ref":101}},{"amount":"12","asset":{"$ref":70},"from":"reserve","kind":"Transfer","to":"buyer"},[{"$ref":104}],{"account":"buyer","amount":"169","asset":{"$ref":70}},{"account":"reserve","amount":"18","asset":{"$ref":70}},[{"$ref":67},{"$ref":68},{"$ref":69},{"$ref":106},{"$ref":72},{"$ref":107},{"$ref":93},{"$ref":86}],{"balances":{"$ref":108},"liabilities":{"$ref":100},"state":{"$ref":80}},{"label":"proposed new remedy transfer","new_effects":{"$ref":105},"snapshot":{"$ref":109}},{"after":"8","asset":{"$ref":70},"before":"20","creditor":"buyer","debtor":"reserve","kind":"Liability","name":"remedy"},[{"$ref":111}],{"amount":"8","asset":{"$ref":70},"creditor":"buyer","debtor":"reserve","name":"remedy"},[{"$ref":77},{"$ref":113}],{"balances":{"$ref":108},"liabilities":{"$ref":114},"state":{"$ref":80}},{"label":"proposed explicit duty discharge","new_effects":{"$ref":112},"snapshot":{"$ref":115}},[{"$ref":82},{"$ref":89},{"$ref":96},{"$ref":102},{"$ref":103},{"$ref":110},{"$ref":116}],{"BOND":{"$ref":66},"PAY":{"$ref":70}},{"price":40,"remedy":20,"retainedFee":3},{"compensation":12},{"effects":{"$ref":0},"financial_post":{"$ref":101},"financial_pre":{"$ref":101},"known_attempted":"2","known_committed":"7","liabilities_retained":{"$ref":100},"meaning":"Local false Bool only; not authenticated failure/nonexecution/cancellation evidence"},[{"$ref":99}],{"audit_BOND":"4 + creation0 + accrual0 - discharge0 =4","remedy":"absent + creation20 + accrual0 - discharge12 =8; present-zero retained when zero"},{"actual_reachability":"UNSPECIFIED","attempted":"UNSPECIFIED","committed":"UNSPECIFIED","conditional_financial_post":{"$ref":109},"conditional_new_effects":{"$ref":105},"duties_retained":{"$ref":100},"explicit_case_premise":"Only if the selected future recovery phase model permits this prefix to commit; current /2 recovery phase policy not selected","required_rule":"Do not infer discharge from payment or replay the transfer as if absent; outcome/next remedy policy remains unspecified","terminal_or_late_event_relation":"UNSPECIFIED"},{"admission_or_error_precedence":"UNSPECIFIED","attempted":"UNSPECIFIED","duties_retained":{"$ref":100},"explicit_case_premise":"No remedy transfer, fee, or discharge has occurred. This is a documentary boundary, not an assumed runtime atomicity/refusal rule.","financial_post":{"$ref":101},"financial_pre":{"$ref":101},"new_effects":{"$ref":0},"recovery_status_wire":"UNSPECIFIED"},[{"$ref":83},{"$ref":90},{"$ref":97},{"$ref":104},{"$ref":111}],{"accounting":{"$ref":65},"action_source":"primitive-actions.proposed.mori","all_prefix_frames":{"$ref":117},"artifact_role":"Custodian-authored proposed normative economic example; this wrapper is documentary, not a Mori wire schema","assets":{"$ref":118},"chosen_inputs":{"$ref":119},"chosen_remedy_body_inputs":{"$ref":120},"closing_duties":{"$ref":114},"complete_initial":{"$ref":81},"complete_pending":{"$ref":101},"condition_not_met_outcome":{"$ref":121},"created_duties":{"$ref":122},"discharged_amount":"12","duty_evolution":{"$ref":123},"failure_after_remedy_transfer_before_discharge":{"$ref":124},"failure_before_any_new_financial_effect":{"$ref":125},"financial_policy":"Pay new compensation12 from separately controlled reserve against expressly declared remedy20; retain original completed payment40 and fee3; leave remedy8","inapplicable":{"$ref":59},"no_product_execution":true,"opening_duties":{"$ref":78},"ordered_history":{"$ref":126},"ordinary_remedy_body_action":"compensate","ordinary_start_action":"purchase","preservation":"All other exact-asset balance cells, marker, unrelated BOND audit duty4 and earlier committed effect history retained","proposed_successful_recovery_frame":{"$ref":115},"residual_positive_duties":{"$ref":114},"specific_pending_frame_gap":"Original pending custody action still captures principal60 and its source suffix pays merchant60; original compensation action still captures remedy20 while proposed residual is8. No existing ordinary remedy action alone is asserted to close/update either pending frame or alter that suffix. A recovery binding/late-event rule is required.","status":"PROPOSED financial policy; no actual recovery implementation/adoption/consumer observation claimed","unresolved":{"$ref":61},"work":{"$ref":63}},{"custody-refund":{"$ref":64},"forward-compensation":{"$ref":127}}],"root":{"$ref":128}}
