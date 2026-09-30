---
id: moriarty.beta.20260930.workflows
title: Stages, completion and asynchronous recovery
type: comparison
status: active
created: 2026-09-30
updated: 2026-09-30
tags: [moriarty, beta, stages, recovery, specified-only]
sources: [WF-SWIFT, WF-KOTLIN, WF-ELM, WF-DAML, WF-S0, WF-MIL3, WF-MIL4]
---

# Stages, completion and asynchronous recovery

**Recommendation; specified only:** expose named, one-domain stages inside a persistent episode. Give each stage explicit authenticated inputs, authority, conditions, derived effects, outcomes and continuing duties. Completion fills bounded holes in the signed template; reconciliation learns what happened; recovery applies a separately authorized remedy. The source must show these distinctions. Ordinary `await`, exception retry and a global rollback block do not express them safely.

This is domain research for the requested five-reviewer beta decision, not an adopted grammar. Baseline: `870f998b36ecda04622fa4274132e74902942d0b`. No production code, runtime experiments, proofs, submissions, commits or canonical wiki transactions occurred here. URLs, inspected sections, retrieval dates and hashes are in [workflows-sources.json](../workflows-sources.json).

## Sources and what their mechanisms establish

**Source facts WF-F01–F04.** Swift's official [Concurrency language guide source](https://raw.githubusercontent.com/swiftlang/swift-book/main/TSPL.docc/LanguageGuide/Concurrency.md), sections *Defining and Calling Asynchronous Functions*, *Calling Asynchronous Functions in Parallel*, *Tasks and Task Groups* and *Task Cancellation*, marks possible suspension points with `await`, introduces child tasks with `async let` and task groups, and describes cooperative cancellation. Parent cancellation propagates to child tasks; tasks still respond through cancellation checks. These mechanisms describe task scheduling and lifetime. They establish no blockchain inclusion, finality, authenticated observations, durable restart or reversal of earlier financial effects. The captured source is a hash-pinned `main` snapshot, without a resolved Git commit; it is not a release pin.

**Source facts WF-F05–F08.** Kotlin's official [Cancellation and timeouts](https://kotlinlang.org/docs/coroutines-cancellation.html), sections *Cancel coroutines*, *Cancellation propagation*, *Make coroutines react to cancellation*, *Run non-cancelable blocks* and *Timeout*, describes cancellation through `Job`, propagation to children and cooperative checks. Catching a cancellation exception can break propagation unless it is rethrown. Cleanup can use `finally` and `withContext(NonCancellable)`; the guide warns that using `NonCancellable` with other builders breaks the parent-child relationship. `withTimeoutOrNull` cancels a timed operation and returns `null` on timeout. None of these local coroutine facts establishes remote nonexecution or permits an economic refund. The article displays 27 July 2026; the immutable acquisition receipt leaves publication date unestablished, and the manifest records the displayed date separately.

**Source fact WF-F09, reused September 9 capture.** Elm's [Commands and Subscriptions](https://guide.elm-lang.org/effects/), *element*, has programs return `Cmd` and `Sub` values to a runtime that performs external work and returns messages. This offers a useful readable separation between preparing a request and receiving a result. A command value is not delivery evidence; the guide supplies no transactional financial semantics.

**Source facts WF-F10–F13, reused September 19 study and inspected pinned code.** [Daml settlement](../../../deliverables/daml-study-2026-09-19/studies/settlement/REPORT.md), DS03/DS05/DS06/DS10/DS11/DS13/DS17, distinguishes pending allocations, final settlement, authorization and released funding. Splice acceptance may signal allocations *will* be created; it does not certify delivery. The pinned withdrawal helper denies authorizer withdrawal of committed allocations without a deadline and requires ledger time strictly beyond a present deadline. The OTC example explicitly warns that consuming cancellation cannot handle allocations arriving afterward, and suggests retaining a marker. Daml Finance's atomic final transaction does not erase earlier allocation transactions. Authorized off-ledger acknowledgement is also distinct from an actual holding transfer. These are comparison mechanisms with stated runtime assumptions, not Midnight guarantees.

**Acquisition observations:** two document targets, three Fetcher attempts, one initial redirect. Swift and the canonical Kotlin article returned HTTP 200 with substantive text. The legacy Kotlin URL returned only JS/meta navigation to the canonical URL; that receipt remains intact and supports no coroutine claim. The lead explicitly authorized one bounded redirect resolution. No JavaScript execution, authentication, bypass or private egress occurred. Raw responses and separate extracts are retained under `/home/charl/research/moriarty-beta-2026-09-30/`; duplicate Kotlin text from overlapping containers is an extraction limitation. No cookies were saved or new general site pattern required.

## Baseline and the proposed extension

**Repository observations WF-R01–R03.** The [Source/6 S0 contract](../../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md), *Formation*, *Admission* and *Boundary*, admits one closed transfer or funded AccrualFirst repayment proposal. Nonempty observation, retained-effect, duty, failure and recovery forms reject at source formation. Local success is `PreparedUnqualified`; signature, snapshot, successor and ledger qualification remain missing premises. Ordered judgments are Stage, Intent, Effect, Authority, History and Failure. Rejection publishes neither post-state nor financial effects, and reports the first failing judgment/code.

The [MIL/3 design](../../../deliverables/mil3-design-draft-2026-09-29/DESIGN-MIL3-DRAFT.md), §§3–7, and [MIL/4 semantic contract](../../../experiments/moriarty-language/formal/mil4/semantics-contract.md) specify completion refinement, pre-only guards, post-state outcome checks, persistent duties, current-head consumption and paired bridge claims. The [current finite result](../../../deliverables/mil4-k-quint-sprint1-2026-09-29/RESULT.md) describes bounded S0 K/Quint/Core observations; it does not demonstrate executable asynchronous episodes. The [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md), *What provable intention means*, requires explicit ledger phases and accepted failure outcomes retaining authorized effects, fees and remedies.

**Recommendation WF-D01:** use a separate beta profile. Rich workflow syntax can be parsed, inspected and checked as specified-only authoring material. Unsupported local execution must return a named diagnostic before any preparer or dispatch; an empty simulator path must not report successful settlement. Preserve Source/6 formation and its closed grammar. First beta execution can lower only supported transfer/repayment forms, with field correspondence and no weakening of S0 checks.

## Three readable syntax approaches

All following fragments are **specified-only alternatives**, not executable Moriarty.

| Approach | Fragment | Benefit | Risk and disposition |
| --- | --- | --- | --- |
| Named declarative stages | `stage Claim on Destination { requires receipt; perform bridge.receive(...); ensures post(claim).received; }` | Makes domain, guards and commit boundary visible; state machine can be rendered directly | More declarations; recommend this as the authoritative form |
| Sequential durable workflow | `episode { commit Escrow; wait Receipt until cutoff; commit Swap; recover Refund; }` | Familiar sequence and visible waits | Compiler must synthesize durable state, evidence policy, continuation, races and duties; defer sugar until expansion is inspectable and proven |
| Event and transition table | `transition Pending -> Settled on Receipt { ... }` | Exhaustive outcomes and incoming events are easy to audit | An event label can conceal authentication and effects; support an IDE view or library notation after the same typed lowering exists |

**Inference WF-I01:** braces and named stages fit the proposed beta surface and make financial boundaries easier to inspect. No captured source measures their usability or AI accuracy. A human authoring task comparison should test whether readers identify fees, pending duties and refund conditions; taste does not establish superiority.

**Recommendation WF-D02:** avoid an authoritative `try { await bridge(...) } catch { refund(); }`. Suspension identifies a control-flow point; it does not name a verifier, a durable claim, a terminal receipt or consumption authority. A late foreign receipt can follow a local timeout. Restarting such code can also repeat a dispatch. Named stages require those facts explicitly.

## Episode, status, evidence and authority contract

**Recommendation WF-D03; specified-only semantics.** Persist episode identity, signed intent digest, selected program/policy, domain-qualified heads, stage and logical effect IDs, per-attempt IDs, cumulative gross/fees, allowance and nonce consumption, reservations, residual duties, continuation commitment and terminal tombstones. Logical delivery, signing, sending and settlement are separate facts. A lost response leaves the economic effect unresolved even when the sending task ended. Reconciliation must be idempotent under duplicate and reordered evidence.

| Visible status | Required meaning | What authoring tools show |
| --- | --- | --- |
| `Pending` | Valid committed progress with a named outstanding duty | Remaining amount, beneficiary, reservation, admissible continuations and recovery authority |
| `Unknown` | Available evidence cannot establish whether an external effect occurred | Last known send/effect facts; held reservation; reconciliation duty; uncertainty reason |
| `Settled` | The selected scope's exact success relation and evidence hold | Complete effects and discharged duties; stage settlement is distinguished from episode settlement |
| `Rejected` | A proposal failed formation/admission/evaluation | First code, source location, unchanged financial state; the existing episode keeps its prior status |
| `Recovered` | A signed remedy committed and its own required evidence holds | Retained fees/losses, refunded amounts and explicit residual duties; terminal only when no remaining duty can settle |

The five labels are a user display vocabulary, not one mutable lifecycle cell. A rejected attempt can occur while its episode remains Pending. Unknown concerns knowledge of an effect, not permission to forget a liability. Partial delivery is represented by delivered and remaining quantities plus duty; it does not need an ambiguous global `Success` flag.

**Recommendation WF-D04:** every evidence input binds nominal subject, value/unit, source/domain, observed time, authenticated stage clock, freshness, round selection, provenance, finality policy and verifier/program version. Reject missing, stale, disputed or conflicting required evidence conservatively. Missing remote evidence retains Unknown; a malformed supplied proof rejects that proposal. A skipped Boolean branch cannot launder an inadmissible named observation. Confirmation count is not a portable finality rule; pin each domain's evidence contract and retain reorganization handling.

**Recommendation WF-D05:** completion may fill a typed route/amount hole only within signed finite bounds and selected program constraints. Fixed recipients, domains, assets/representations, fees, finality, recovery beneficiary, policy and failure scope cannot become solver conveniences. Conditions use `pre` and admitted evidence; `ensures` checks derived `post` and effects. Filling a hole with an entire control-flow fragment is unsupported until its type describes footprint, authority, duties, outcomes and finite work.

Recovery uses a distinct grant with scope, epoch, expiry, allowance and work reservation. Canceling a worker grants no recovery right. Local deadlines use `Instant<SourceClock>` or `Instant<DestinationClock>`; crossing clocks needs an explicit authenticated relation. Budgeting reserves work/state for reconciliation and remedy without promising eventual inclusion or liquidity. Governance amendments preserve pinned existing duties unless the required preservation relation or affected-party authorization exists.

## Cross-family and cross-domain example

**Specified-only design sketch; all workflow, bridge, AMM, receipt and recovery constructs below are proposed.** Identity literals represent declared nominal IDs, not authenticated facts. This example combines bridge settlement and an AMM. It exposes success, foreign nonreceipt recovery and recovery after received assets cannot meet the swap floor. All stages commit on one domain. Programs and verifiers are pinned externally; cryptographic and ledger correspondence remain open.

```mori
profile "moriarty-beta-workflow/0";
agreement BridgeThenSwap {
  intent Trade {
    owner: Alice, recipient: Bob,
    source: Source, destination: Destination,
    input: A, representation: WA, output: B,
    gross_cap: atoms(asset: A, value: 11),
    source_fee_cap: atoms(asset: A, value: 1),
    destination_fee_cap: atoms(asset: WA, value: 0),
    net_floor: atoms(asset: B, value: 20),
    recover_to: Alice, nonce: "trade-1",
    partial: HoldDuty, foreign_policy: ReceiptPolicy,
  }
  episode Delivery governed_by Trade {
    stage Escrow on Source {
      requires pre(claim).unused;
      perform bridge.escrow(claim: X, amount: 10 A, owner: Alice);
      ensures post(claim).pending;
      outcome Pending duty DeliverOrRecover(X);
    }
    stage Receive on Destination {
      evidence origin: FinalSourceEscrow<X> via SourcePolicy;
      requires pre(receipt).unconsumed;
      perform bridge.receive(claim: X, recipient: TradeCustody);
      ensures post(receipt).received && post(custody).balance == 10 WA;
      outcome Pending duty SwapOrReturn(X);
    }
    stage Swap on Destination {
      requires pre(receipt).received;
      perform amm.exactInput(pool: P, input: 10 WA, recipient: Bob);
      ensures credited(Bob, B) >= 20 B;
      outcome Settled;
    }
    recovery Refund on Source authorized_by RefundGrant {
      evidence stopped: FinalDestinationNonreceipt<X> via ReceiptPolicy;
      requires pre(claim).pending;
      perform bridge.refund(claim: X, recipient: Alice);
      outcome Recovered;
    }
    recovery Return on Destination authorized_by ReturnGrant {
      requires pre(receipt).received && pre(swap).unconsumed;
      perform bridge.returnClaimed(claim: X, recipient: Alice);
      outcome Recovered;
    }
  }
}
```

This is a **workflow fragment**, not a complete signing or provider bundle. The surrounding declaration must fix X's source/destination, assets, 1:1 representation ratio, escrow custody, exact fee beneficiary, time windows, selected programs, authorities, nonce scopes and recovery policy. `Return` delivers WA on Destination under the signed recovery outcome; it does not promise A on Source. The sender must expressly sign that alternative and any remaining backing liability. The fragment abbreviates these terms only for comparing workflow syntax.

**Recommendation WF-D06:** initial policy permits at most one source fee of 1 A and zero destination fees, including retained AMM fees. If the venue charges any fee, this candidate rejects. A realistic fee-bearing successor needs explicit per-asset fee accounting and any certified conversion; silently valuing WA fees as A is invalid. Refund never resets gross/fees. Actual gross outflows, custody reservations and remaining entitlements remain distinct.

A destination receipt decides received versus terminal nonreceipt once. Source-clock expiry initiates reconciliation; it cannot authorize Refund. Missing or conflicting destination evidence retains Unknown and locked source custody. A finalized receive without a feasible swap leaves SwapOrReturn duty. Swap and Return serialize on current custody, receipt, claim and episode heads; an atomic receive-plus-swap optimization would require one selected destination composite program with the complete footprint and effects. Cancellation leaves a marker capable of rejecting or resolving late results. Settled/Recovered episode labels require closure evidence for all paired claims and late effects, beyond a local stage result.

## Grammar, tools, formal obligations and implementation order

**Recommendation WF-D07:** use explicit episode/stage/recovery braces, semicolon-terminated statements, closed outcomes and named arguments. Separate stage guards, evidence declarations, economic operations, postconditions and outcomes into typed AST nodes. Reject duplicate stage IDs, unknown transitions, cross-domain writes, post-state guard reads, unfixed economically relevant holes and unsupported operations. Bound stages, edges, holes and term work; no general runtime loops or recursion are implied. Editor recovery can synchronize at semicolons and closing braces, but execution always needs a complete valid AST. Offer a state-machine view, inferred footprints, pending-duty hover, signed-versus-filled display, and goto-definition for policy/verifier/authority. A single analysis service should power CLI and editor diagnostics.

**Formal obligations WF-O01–O06; open.** K must define total typed stage preparation, complete ordered effects, stable first rejection, phase-specific accepted failure and persistent duty/head/nonce transitions. Quint must model restart, duplicate attempts, lost acknowledgements, concurrent reservation, late receive/refund and policy changes. Invariants include conservation per domain, aggregate locks, no duplicated entitlement, monotone cumulative gross/fees, unique consumption and no duty erasure. A bounded trace is not a universal theorem; an environmental finality Boolean is an assumption, not a verifier. The source/Core/K/Quint projection must preserve signed scope, exact statuses, effects and rejection. Compiler/native/ledger correspondence remains separate. Any liveness claim names inclusion, evidence availability, work/state capacity, authority and liquidity assumptions.

Local rejected preparation remains atomic. A real Midnight failed fallible phase may retain guaranteed-phase effects and fees. The lowerer must implement a signed explicit phase layout and failure relation; it cannot reuse local no-effect rejection or manufacture a charged remedy from an exception. Native retained effects, fee/nonce authority and residual duty require exact readback evidence.

**Hostile controls WF-H01–H12; specified tests:** reject refund from timeout alone; retain Unknown after lost send acknowledgement; forbid replay with a new attempt ID; reject wrong foreign domain/asset/recipient; reject stale or conflicting receipts; reject downgraded finality filled by a solver; serialize late Receive versus terminal nonreceipt; serialize Swap versus Return; retain source fees after refund; reject omitted pending duty/footprint; preserve remedy after revocation or governance pause under its pinned contract; reject self-fulfilling post-state guards. Include feasible well-formed positives for each check, combined-fault first-code controls and restart between reservation, send and observation.

**Phased plan WF-P01–P04; recommendation.** First implement beta authoring AST, diagnostics, examples and S0-only execution lowering, explicitly rejecting every richer execution request. Next add a one-domain Pending/continue/recover reference profile with exact duty, time and reservation semantics and K/Quint hostile expectations. Then establish native Midnight phase and current-head consumption evidence for that profile. Finally add one named foreign verifier and paired bridge machines, qualify its evidence/recovery limits, and compose the destination AMM through a checked typed interface. No generic scheduler or managed service becomes a developer prerequisite. Broad syntax can be specified early; each executable claim closes only with its own contract and retained evidence.
