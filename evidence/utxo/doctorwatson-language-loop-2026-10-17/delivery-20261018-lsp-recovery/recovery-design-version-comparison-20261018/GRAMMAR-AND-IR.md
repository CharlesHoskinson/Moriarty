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
