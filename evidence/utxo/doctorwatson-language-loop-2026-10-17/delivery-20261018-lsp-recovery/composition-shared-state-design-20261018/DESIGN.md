# Shared-state interleaving: one local head, two pending actions

Status: proposed ordinary source semantics, not adopted, implemented, executed, proved or accepted on a ledger. Subject: main `74a909d69639df8823a527a4d608a7516ccd69a2`, tree `f18c2346ff435f9ac95a7f6f56457c2edde3f71c`. The root obtains substantive design votes before implementation. This proposal supplies no authority, hostile-input, native or resource-adequacy judgment.

The falsifiable predicate is: a real public consumer can start two actions on one complete financial head, preserve both pending frames, and resume either action against the current shared head while keeping its earlier captures. Both explicit resume orders must match complete independently frozen observations. Sequential action replay must remain distinguishable from these traces.

## Current source fact

`stage::disjoint::Pair` is a local product of two independent staged programs. It refuses overlapping whole-program state names, exact asset identities and source liability names. It also refuses overlapping snapshot rows. It starts left then right, resumes each member from that member's own continuation, and merges both boundaries. The public `mori test` disjoint consumer calls this actual API and compares each complete report before lossless prefix referencing. These are useful existing behaviors. None supplies a shared-head operator.

`stage::resume` instead runs one pending segment against the boundary carried by its input continuation. It checks R0–R7 and preserves the supplied captures. `runtime::run_segment` seeds state names from that boundary and then inserts captures and event values. Stage plans capture action inputs, prior `let` values and prior event binders; they do not automatically capture state declarations. Therefore a saved `let quoted = tier` can remain 1 while a later un-captured read of `tier` sees 2. Today no composition consumer owns the rule for choosing a changed shared boundary. Manually editing a continuation in a test is not that consumer.

## Alternatives considered

1. **Separate shared Session, recommended.** One admitted program, one full head and two distinct selected actions. Each invocation publishes at most one source segment. Pending action frames persist while another action changes the head. This adds a small explicit local operator without changing disjoint contracts.
2. **Weaken Pair collision checks.** Refused as the proposed path: Pair's independent snapshots cannot decide which shared update to retain, and simply choosing a side discards state or duties. This would replace the adopted disjoint contract.
3. **Run complete actions sequentially.** Useful baseline, not shared interleaving. The second action would capture the new fee tier after the first completes. It cannot produce the two simultaneously pending frames in the selected example.

## SI1: membership and lifecycle

Propose a separate non-default `core2-shared-interleaving-draft` feature and `stage::shared::Session`. The names are proposed, not existing APIs. The first slice accepts exactly one `StagedProgram` and two slots selecting distinct actions from it. The selected slots and their immutable input vectors are fixed when the Session opens. Arbitrary cross-program aliases, dynamic slot creation, reentrant instances of the same action and external shared stores are outside this slice.

Each slot is `not-started`, `waiting` or `complete`. Its frame is a `PendingFrame`; idle and complete both have site 0/empty captures, so status must remain explicit. `start(slot)` is explicit and may occur before or after the other slot completes. `resume(slot,event)` selects exactly one waiting slot. There is no implicit start order, fairness, scheduler, simultaneous execution or whole-session rollback.

The Session owns opening and current complete Snapshots. The head contains every declared state, every explicit balance row and every source-registered liability row, including untouched positive and zero rows. It never merges independent copies or projects away another slot's duties. One successful advancing segment replaces the complete head and increments the head revision by one. Every other slot retains its exact status, site, captures, cap and committed work.

## SI2: the new boundary-selection rule

An invocation executes one entire existing source segment, from entry to the next await or completion. No other member runs between that segment's statements. This is local segment indivisibility, not ledger atomic synchronization, a same-contract Join, private handoff or concurrent processor execution.

Start uses the current head plus only the selected slot's immutable inputs. Resume uses the selected slot's program/plan/action/site/captures/stored logical fields and the **current Session head**, even if another slot has changed it since this frame was created. Captures are not recomputed. State reads not explicitly stored in captures observe this current head. The Session records the frame's origin revision and the invocation's actual entry revision separately.

This is a new composition rule. Ordinary `stage::start`, `stage::resume`, their wire formats, and `Pair` remain unchanged. Proposed implementation may construct an internal invocation continuation from the owned frame plus current head and call the existing admission path, or factor the same admission into a crate-private helper. It must retain R0–R7, H25 emitted-boundary resumability, H26 exact static cost, the same scalar evaluator, full StageOutcome/StageRejection and complete original traces. It must not expose a public “replace boundary” method or call a separate success evaluator. No change to capture layout, statement costing or scalar financial operations is proposed.

The current head must satisfy the existing canonical-boundary rules for every selected action before it is admitted/published as the shared head, not merely the action that just ran. This includes exact state types, required explicit balance accounts, whole-program liability identities, canonical ordering, existing count/trace bounds and emitted continuation limits. This stronger shared-head compatibility check is proposed; exact refusal codes/messages need adoption before implementation. A local adapter returning `Ok` without owning and reporting these rules is not implementation of SI2.

## SI3: complete publication and observation

Every history row records slot, frame-origin revision, entry revision, exit revision, and the complete existing `stage::outcome_report` envelope. Earlier rows are immutable. Their boundaries are historical, not current global projections. The current head is reported separately. The most recent old outcome for a waiting or completed slot must never overwrite the current head after another slot runs.

The report includes the full opening/current Snapshots, revision, all slot statuses/frames/caps/committed counters, complete history, complete invocation-then-source-ordered effects, aggregate work and every positive current residual liability. It preserves complete Execution.pre/post, every Step/statement/value/snapshot/effect, frame_pre/post, captures, existing program/plan hashes and all outcome labels. Latest history references point to original rows without relabeling their snapshots as current. No pair hash or authenticated identity is invented.

False guards preserve the head/revision and the selected frame/captures/committed work; the invocation observes the current head, returns the actual Waiting/advanced=false/segment=null envelope, and contributes its actual attempted dispatch work. An error preserves the financial head and frames and retains the original complete StageRejection or a distinct composition diagnostic. It does not fabricate a successful segment or erase prior duties/effects. Exact local accounting/publication on such an error is an unadopted SI4 decision; no false/error fixture is authored in this ordinary positive slice.

Report labels proposed for this separate format are qualification=`local-reference-stage`, authentication=`none`, consumption=`untracked`, composition_qualification=`unadopted-local-shared-interleaving`, and format=`moriarty-local-shared-interleaving-proposal/1`. The adopted disjoint labels and profiles remain byte-unchanged. A successful local report does not imply authentication, unique consumption, settled duties or a proof.

## SI4: finite work proposal, with owner-dependent consequences

Keep the existing cost profile: statement dispatch plus visited expression nodes; entering an await costs 1 in its start segment, and its guard belongs to resume. Each selected action in the example has segment costs [5,18], committed prefixes [0,5], and total 23. The unused declaration action costs 2 but is not a selected slot and never executes. The example's two per-slot stored caps are 23, descriptive sum 46. Propose that configured invocation_limit is within 1..=256, with no claim that this ceiling establishes resource adequacy. Proposed positive fixture bounds are attempted work 46 and four source invocations. This arithmetic is not a machine-resource, circuit-row, fee, authority or production allocation claim.

Propose that the Session carries a cumulative attempted-work counter separate from the sum of per-slot committed prefixes. Before an invocation it checks the remaining static segment ceiling against its remaining attempted budget; it never borrows or resets another slot's cap. After actual admission/evaluation it accounts actual attempted work, including a false guard or StageRejection. A pre-evaluation refusal accounts zero evaluator work. Invocation count and observed error diagnostics must also survive a failed call, while financial head and frames stay unchanged. Public errors must distinguish an unchanged head from these accounting observations. There is no native charged-failure or fee policy here.

This conservative preflight means a false guard may consume work that prevents the planned positive completion under the tight 46 fixture budget. The caller cannot silently reset it. Admission of this error-accounting policy, the relationship of attempted budget to an eventual authenticated head/lifetime budget, and all actual resource adequacy/allocations remain designated-owner decisions before implementation. Existing disjoint caps, its 256-invocation ceiling and all historical limits remain unchanged. No new campaign or execution authority follows from these proposed numbers.

## Financial example and why ordering matters

Two delivery requests share the same reserve, exact CASH identity and `tier`. Starting each request records its quote and creates its delivery duty. Completing a partial delivery pays the fee tier current at that segment, increments the tier, transfers a partial amount, and explicitly discharges exactly that amount of the duty. The source deliberately distinguishes indicative quote from charged tier; authorization of such a policy is not established here.

Opening tier=1, marker=77, reserve=100, observer=11, all recipients/fee accounts=0. There is an untouched outside duty 5. A creates duty 10 and delivers 7; B creates duty 20 and delivers 12. In both interleavings A and B start while tier is still 1, so both saved quotes are 1. If A resumes first, fees-a=1 and fees-b=2. If B resumes first, fees-b=1 and fees-a=2. Both finish at tier=3, reserve=78, alice=7, bob=12, observer=11, duties A=3/B=8/outside=5, cumulative work 46. The complete ordered effects and charged emits differ.

For sequential A then B, B starts after tier becomes 2 and saves quote=2. Sequential B then A likewise gives A quote=2. Thus equal final balances under one ordering cannot substitute for full frame/history comparison. The proposal includes all four explicit schedules and complete per-boundary financial/frame/work tables. The authored worked examples are proposed arithmetic, not independent oracles or observed evaluator results.

## Actual consumer adoption path

Add a separate profile branch in existing `crates/mori/src/commands/test.rs`, beside its current stage and disjoint branches, routed to a thin proposed `test/shared_test.rs`. It loads one actual source through `stage::load`, one complete opening Snapshot, fixed slots/bounds, explicit start/resume operations and all complete expected reports **before** invocation. Each operation calls the new public Session API and compares the whole report without adding a reference codec in this slice. The source-level and binary tests must consume that real path. Existing disjoint manifest parsing, dispatch, Pair, prefix table behavior and all goldens remain untouched.

The proposed library test in this packet is a complete positive test source against this proposed API. It cannot run today: the API is absent and independent full expected files have not been frozen. A proposed public manifest uses the same four scripts and binds five observations each (opening plus four calls). Its full-array aggregate requires an adopted wire contract and separate complete independent expectations before execution. Do not count these design files as passing tests.

## Decisions root must obtain before implementation

SI1: adopt/revise the single-program/two-distinct-actions/local-only Session and explicit lifecycle.

SI2: adopt/revise current-head resume with immutable saved captures, segment granularity, all-selected-action canonical checks, no ordinary-stage/Pair changes.

SI3: adopt/revise the complete report/provenance contract and separate public consumer/profile/feature. Field layout is proposed wire, not an independently established codec.

SI4: adopt/revise cumulative attempted accounting, conservative preflight and durable accounting diagnostics on failed calls. Design votes do not decide authority, native failure charging, resource adequacy or new execution allocation; designated owner dispositions are still required.

Independent finance/work custodian: derive full Program/Plan/canonical-preimage expectations and all 20 complete Session/StageOutcome/Execution/Step observations from the original source, opening, scripts and adopted clauses, without producer outputs. Author arithmetic is supplied only as a disclosed non-independent example. Freeze complete public aggregate expectations before the first actual consumer run. Preserve any disagreement rather than regenerating expectations from production.

Head/authority owner: specify how a shared head, its pending frames, revision, lineage, authority, residual duties and global lifecycle budget become authenticated and uniquely accepted. This packet supplies no security recommendation or hostile tests. Full shared-state acceptance stays open without that rule, actual proof/correspondence and ledger evidence. Root must not advertise SP10 shared-state support based on the local Session alone.

## Separate operator dispositions

Sequential: baseline schedules only; native Step/history acceptance remains an independent requirement.

Disjoint parallel: existing merged Pair remains its qualified local positive subset; no overlap check or original requirement changes.

Shared-state interleaving: this unadopted local design is the next finite source slice; all full acceptance obligations remain open.

Atomic synchronization: no all-or-nothing multi-action Join or cross-contract synchronization is proposed or observed.

Asynchronous messaging: no message transport, delivery, settlement, unknown/recovery or liveness semantics are proposed or observed.

All U0–U7, M1–M17, MC/SP/G, R7, native/proof/ledger/target, authority, confidentiality, hostile-input and resource exits retain their original ownership and status. No required operator is removed or treated as equivalent to two-stage sequencing.
