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
