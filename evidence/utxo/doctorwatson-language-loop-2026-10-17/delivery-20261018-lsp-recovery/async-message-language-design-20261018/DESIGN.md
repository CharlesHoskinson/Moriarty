# Asynchronous requests with explicit unresolved outcomes

Status: independent design proposal only. Base: `74a909d69639df8823a527a4d608a7516ccd69a2`. No source adoption, product execution, model run, compiler run, SDK call, proof or target observation is claimed.

## Falsifiable proposed capability

An external developer supplies a financial source file and an application-owned async request descriptor. The tool can inspect the request, project a pending/acknowledged/unknown/settled/recovered lifecycle and report every local effect, retained fee, reserve and residual debt. A deadline changes knowledge to unknown; it never supplies a recovery judgment. Twelve complete normal traces specify this behavior. Their settlement/recovery transitions are conditional on separately supplied owner-defined judgments, not claims that such judgments currently exist.

No maintainer registry, campaign, provider subscription or reviewer receipt is a public input. An application may choose its own supported observation policy and endpoint. Any eventual target implementation still needs the actual program, owner-authority, proof and ledger obligations.

## Existing source observations

The product contract requires explicit pending, partial, unknown, settled and recovered states, keeps earlier effects and fees, and says timeout is not evidence of nonexecution. The original partial-transaction EARS remain future requirements. MPLR-006 assigns origin, correlation, finality and consumption to the asynchronous evidence judgment. MPLR-010 distinguishes deadline observation from evidence establishing delivery, recovery or nonexecution. Its theory note specifically says known success must not be pushed back to unknown merely by expiry.

Current `crates/mori-core/src/stage/mod.rs` provides `moriarty/2`, `moriarty-core/2`, Continuation and StageEvent, but explicitly labels authentication `none` and consumption `untracked`. Its `Waiting` and `Complete` describe local segments. Its continuation stores one boundary and a logical count; its event has action/label/value binders. These facts do not establish an authenticated request lifecycle. Current `simulate` routes Core/2 to that local interface. This proposal preserves those profiles and outputs.

## Alternatives and recommendation

1. **Recommend an explicit request descriptor over ordinary finite financial actions.** The source remains a small readable financial program. A separately versioned async relation links each external lifecycle event to one source action, complete history and declared policy. The descriptor gives users an inspectable API without pretending current await binders are authenticated receipts. It requires a real compiler/consumer and owner-provided judgments before target adoption; a JSON sidecar alone supplies none.
2. Add request/outcome branching syntax directly to Core/2. This could eventually improve ergonomics, but mixes a new observation/lifecycle relation into a deliberately unauthenticated profile and requires larger parser/IR/proof changes. Defer syntax sugar until the semantic relation is reviewed.
3. Encode external outcomes as caller-supplied Bool arguments to existing await. That can illustrate a local assumption, but loses a first-class unknown state and cannot establish the required evidence semantics. Keep it within current local examples rather than use it as the async contract.

The first option is a bounded design slice, not a new orchestration service or another project acceptance framework. All descriptor names below are proposed, not adopted public profile names.

## Developer source and descriptor

`invoice.mori.proposed` contains ordinary Core/1 financial bodies with five actions: open, acknowledge, observe_deadline, settle and recover. The bodies use existing state/require/transfer/fee/liability/discharge syntax. This source has not been parsed or run here. Its Core check alone would check those bodies, not the async observations that select them.

`async-plan.proposed.json` binds a static endpoint name, exact asset/payload, occurrence, typed time domain, deadline rule, source actions and transition table. The endpoint name is resolved by the developer's own environment; it is not a maintainer catalog entry. RequestRecorded is a proposed lifecycle effect containing an outbox record. It records an intended request, not provider acceptance, external payment, delivery or finality. The normal example contains no external token transfer: all PAY movements are local source effects, so no external value movement is silently omitted from its accounting.

The descriptor must become part of an explicit async program/relation identity and the public compiler consumer. It cannot be a host-only hint checked after compiling unguarded actions. Program, descriptor, application policy, occurrence, relevant predecessor and complete frame are separate declared inputs to that future relation. Concrete binding encoding, admission and stage authority await their applicable owners; no hash or metadata flag establishes them here.

Potential future CLI spelling, for review rather than present execution:

```sh
"$RUST_MORI" check invoice.mori --async-plan async-plan.json --json
"$RUST_MORI" simulate invoice.mori --async-plan async-plan.json --action open --scenario scenario.json
"$RUST_MORI" simulate invoice.mori --async-plan async-plan.json --async-event assumed-outcome.json --resume request.json
```

`RUST_MORI` is the developer-selected absolute Rust executable, distinguished from the Node beta executable. These flags are not currently implemented. A new async descriptor does not silently change the grammar or claims of Core/1, Core/2 or Node Source/6. Ordinary check must report source/semantics and the separately admitted async profile; a current Core-only success cannot stand in for that admission.

## State and observation semantics

The local financial phase is idle0, active1, unknown3, settled4 or recovered5. A separate Bool records whether an acknowledgment was observed. Phase1 plus acknowledgment projects to acknowledged; phase1 without it projects to pending. Phase3 always projects to unknown, even if acknowledgment later arrives. The numeric gap reserves no semantic meaning.

| From | Event | Result | Financial effect |
|---|---|---|---|
| idle | create | pending | Reserve91, fee2, create invoice90 and service7 duties; record request |
| pending | owner-qualified acknowledgment | acknowledged | None; record knowledge only |
| pending/acknowledged | declared deadline observed | unknown | None; keep reserve91, fee2 and both duties |
| unknown | owner-qualified late acknowledgment | unknown with acknowledgment recorded | None; acknowledgment does not settle delivery |
| pending/acknowledged/unknown | owner-qualified settlement-ready judgment | settled for this request | Pay supplier90, fee1, discharge invoice90 |
| pending/acknowledged/unknown | owner-qualified recovery-ready judgment | recovered for this request | Return buyer90 from escrow, fee1; retain invoice90 and service7 |

The rows describe normal conditional transitions only. This lane defines neither a receipt protocol nor its authenticity, finality, consumption, cancellation, revocation or race judgments. The source's `at >= 100` comparison accepts a local fixture value; target time authority remains owner-dependent. No current wall clock is implicitly consulted. Deadline100 uses explicitly named `example-logical-time` ticks; it is not a Midnight slot/finality assertion.

Acknowledgment is knowledge that the declared acknowledgment occurred, not a promise of delivery or a financial completion flag. An owner-qualified settlement observation closes its corresponding request uncertainty. The proposed machine exposes deadline observation only while unresolved: it does not run an expiry handler after resolution. Normal traces leave the history of timeout and earlier acknowledgments visible after settlement. This does not define handling of competing or duplicate external observations; those judgments remain with the security owner.

Recovery is a forward use of the91 still-held PAY units. It never edits the submission fee, source history or invoice creation. It is not cancellation and does not claim every external request can be recovered. A target cannot use this row until the security owner defines a sufficient recovery-ready judgment for the application/endpoint and its relation to late effects. Without that definition, the example can project only pending/unknown and hypothetical conditional branches. Timeout alone leaves it unknown indefinitely.

## DTO boundary

A request descriptor carries source/profile identity, occurrence (workflow plus ordinal), endpoint, exact typed payload, time-domain/deadline policy, action map and owner-dependent policy references. The occurrence in the example is the public fixture identifier `invoice-37/1`; this lane does not define production occurrence uniqueness or consumption.

`event-dtos.proposed.json` gives the concrete event DTO shapes. An event DTO carries the request occurrence, event kind, typed values and an opaque reference to the externally supplied observation judgment. For local fixtures the judgment reference is marked assumed, never `authenticated: true`. Any actual validation/translation into a usable observation is owner-dependent and absent. A report DTO contains:

- Request identity and lifecycle status, acknowledgment history, and explicit external resolution duties.
- Complete Core pre/post snapshots and ordered source effects for this segment, plus cumulative source effects and separate ordered request events/history.
- Every balance, exact asset, present-zero liability and positive residual duty. Request resolution and whole-workflow completion are separate fields.
- Cumulative per-account gross debits, fee recipients/totals, reserve balance and net changes. Refunds leave gross debits and prior fees unchanged.
- Segment/cumulative Core logical work, separately unmeasured async-envelope and native costs.
- Qualification, authentication, consumption and external-execution coverage labels.

`normal-path-tests.proposed.json` gives these complete report DTOs for each normal segment. Its `design-only-assumed-observation`, `not-established` and `not-observed` labels deliberately qualify every branch. A UI must say simulated/assumed settlement when presenting that local fixture, even though its projected request_status is settled. Successful local arithmetic cannot establish actual external execution. A target-qualified future report would need a separately reviewed assurance format rather than changing these labels opportunistically.

All records and statements in this proposal are public examples. Private evidence/disclosure and later witness availability remain MPLR-013 obligations outside this slice.

## Worked financial example and independent arithmetic

A buyer starts with100 PAY. Escrow, inspector, relay and supplier each have an explicit zero balance. Source state starts phase0, acknowledgment false and a preserved marker31. Opening an invoice request incurs invoice debt90 to supplier and a separate service debt7 to inspector. Those are liabilities, not minted PAY. The buyer moves91 to escrow and pays relay2, leaving buyer7, escrow91, relay2. No debt is discharged merely by reserving funds.

The91 reserve has an explicit composition:90 principal plus1 terminal-stage fee. On the normal settlement branch, escrow sends90 to supplier and pays relay1. Buyer7 + supplier90 + relay3 equals the opening100; escrow and inspector remain0. Invoice debt becomes a present-zero tombstone; service debt7 survives. On recovery, escrow returns90 to buyer and pays relay1. Buyer97 + relay3 equals100. Invoice debt90 and service debt7 both survive, because recovery neither pays the supplier nor cancels either obligation.

Submission fees2 remain in every later snapshot. Total retained fees on either terminal branch are3. Buyer gross debit remains93 even after return90, giving buyer net debit3 on recovery. Escrow gross debit is91 on either terminal branch. Gross movements are counted per account and are not collapsed into a net balance change. There is no mint, burn, external token movement or implicit source of compensation in this example.

A resolved request is not the whole financial workflow. Every terminal fixture reports workflow_complete false because service7 remains, and recovery also leaves invoice90. Future payment of those debts is another explicit action with its own authority and accounting.

## Proposed complete normal tests

All twelve test traces are UNRUN and prospective. Each segment freezes full pre/post state, all five balances, all liabilities including tombstones, exact ordered Core effects (including State), cumulative effects, request events/history, request status, external and financial residual duties, fee/net/gross/reserve accounting and logical work. Each settlement/recovery trace states its owner-defined judgment premise rather than constructing a receipt or cancellation.

N01 stops pending; N02 records acknowledgment; N03 settles directly; N04 acknowledges then settles; N05 reaches deadline unknown; N06 acknowledges before deadline; N07 acknowledges late but remains unknown; N08 settles after deadline; N09 acknowledges late then settles; N10 recovers after deadline under a separately qualified judgment; N11 recovers without timeout under that judgment; N12 acknowledges then recovers. Endpoint and payload do not change across any trace. Original positive states and reports remain in cumulative history.

Independent Core work derivation uses the existing statement-attempt plus first uncached expression-visit rule, inspected in the Core language reference. A require over comparison costs4: one statement plus comparison, variable and literal. Each literal transfer/fee/liability/discharge/next costs2. Open costs14; acknowledge10; deadline observation14; settle16; recover14. Thus N04 ends at40 Core units, N09 at54 and N10 at42. The fixtures include all intermediate totals. These are prospective ordinary Core logical counts, not measurements. Envelope observation/serialization/dispatch work and native costs are unmeasured, not zero; no complete total fee, target resource or liveness claim follows. The async consumer must preserve cumulative work rather than restart its lifecycle total on resume.

Before any source adoption, the integrator should independently check these literal expectations, implement the reviewed public consumer, and run the selected ordinary normal-path tests with an explicit executable/profile. Native/target tests, authenticated receipt tests, hostile tests and cancellation/race tests belong to their separate owners and are not authored or judged here.

## Requirement coverage and open boundaries

| Requirement | Concrete design coverage | Remaining qualification |
|---|---|---|
| MPLR-001/002 | Explicit committed prefix, typed request record and full resumed boundary | Actual continuation relation, occurrence consumption and proofs |
| MPLR-003/018 | Inspectable endpoint/payload/deadline/recovery conditions | Authenticated condition policy and signing/display correspondence |
| MPLR-004/007 | Full residual debts and late acknowledgment/settlement histories | Partial fills, multi-request joins and late losing branches are not implemented by this single-request design |
| MPLR-005/010 | Forward refund of actual escrow reserve; deadline means unknown | Owner-defined recovery evidence, cancellation and late-effect relation |
| MPLR-006/008/011 | Explicit owner-judgment and stage-authority interface slots | Origin, correlation, finality, consumption, relevant-state and concurrency judgments |
| MPLR-009/017 | Exact source asset/fee/debt equations and Core logical totals | Envelope/native costs, charged-failure phases and target cumulative cap |
| MPLR-012 | Per-DTO assurance labels and separate request/workflow status | Real public consumer coverage and observed execution |
| MPLR-013/014 | No private or target claim is made | Privacy/witness availability and ZKIRv3/ledger-phase correspondence |
| MPLR-015/016 | Descriptor and source are developer-authored, with no project admission fields | End-to-end ordinary compile/prove/deploy demonstration remains open |

The original full partial/conditional requirements remain intact. This proposal does not close U0–U7, M/MC/SP/G exits, certify the current Core/2 interface, or make source/application policy files into authentic receipts. Root review and applicable independent owner reviews precede adoption. No shared files or Git state were changed.
