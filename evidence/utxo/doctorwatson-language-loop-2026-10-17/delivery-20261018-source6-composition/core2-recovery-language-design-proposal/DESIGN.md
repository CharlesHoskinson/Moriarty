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
