Both `Write` and `ExitPlanMode` are disabled in this session — this matches `launch_external.py`'s design ("six independent **read-only** architect consultations", invoked with `claude -p --permission-mode plan`), where each slot's `.md` file is the captured response text. So the memo is the deliverable, returned inline.

---

# Operational protocol for a Moriarty DeFi Kernel: operation identity, idempotency, retry and reconciliation

**Architect slot:** `opus-ops` (model `claude-opus-5`, effort high). **Date:** 2026-09-29.
**Status:** independent advisory research under `deliverables/kernel-api-architects-2026-09-29/PROMPT-OPS.md`. This memo is **not** an adopted MIL/4 profile, not a Moriarty implementation, and not an implementation claim. No product code was edited. This is **one architect's view**: I did not read the other eight answers in this study, and **no consensus, agreement or majority is claimed or implied** by anything below. Where this memo agrees or disagrees with another architect, that fact is unknown to me.

---

## 0. Execution context

Per `AGENTS.md`, the host did not expose `moriarty-dev:develop`, so the checked-in skill at `plugins/moriarty-dev/skills/develop/SKILL.md` was read and applied. The guarded CLI was then run read-only from the worktree root:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

```json
{
  "capability": "SP01.6 loan-swap-subset",
  "lastResult": "Stages complete: atomic-prepare, atomic-accept, rp01-mc02; remaining stages: rp01-mc03, rp01-full, f0, f0a, f1-fixtures, f1, i2, f2, f3, mandatory, composition, finance, release, successor-frontend, successor-semantics, actus-semantics, defi-semantics, native-path-freeze",
  "blockedAction": "implementation/repair of loan-swap-subset",
  "reason": "operational history is unresolved or unverified; current evidence gaps: binding-input-stale:…; candidate-input-stale:…; current-accounting-missing:.moriarty-dev/runtime/current-accounting.json; resource-live-state-unavailable:sp01-loan-swap-grok-01",
  "nextAction": "sp01-loan-report",
  "missingEvidence": ["binding-input-stale:…", "candidate-input-stale:…", "current-accounting-missing:…", "resource-live-state-unavailable:sp01-loan-swap-grok-01", "operational-history"],
  "pendingTransactions": []
}
```

Two things follow. First, no Midnight Preview transaction notification is pending, so no transparency line is owed. Second — and this is a mild irony worth stating — the blocking `missingEvidence` entry is literally `operational-history`. The kernel protocol designed below is, at the product level, a proposal for how a *future* system would keep exactly the kind of operational history whose absence currently blocks dispatch. Read-only advisory research needs no campaign, runner or design vote (`AGENTS.md`, "Required startup", step 3).

---

## 1. The one question this memo answers

The supplied NEAR proposal already fixes the *architecture*: the kernel is an optional, capability-constrained execution and reconciliation service that "has no authority to change the signed financial relation, select a weaker evidence premise, erase committed effects, or declare a cross-chain workflow atomic" (`deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`, "Decision proposed"). The Hyperliquid comparison already fixes the *vocabulary*: venue financial kernel ≠ Moriarty DeFi Kernel ≠ intent language (`deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`, "Refined architecture").

What remains unfixed is the **operational protocol**: what a unit of work is called, when it may be repeated, and what evidence moves it forward. That reduces to a single question:

> **When the kernel does not know whether an effect happened, what is it allowed to do?**

Every hostile trace in §7 is an instance of answering that question wrongly. The NEAR proposal states the requirement — "The service must return a typed `unknown` when it cannot establish execution or nonexecution" — but a typed `unknown` in a receipt is not the same thing as an `unknown` that the *state machine* cannot be coaxed out of. The central claim of this memo is that the difference between a safe kernel and an unsafe one is a **shape of state**, not a discipline of operators, and that the shape must make "I refunded because I gave up waiting" **unrepresentable** rather than merely discouraged.

---

## 2. Primary sources

Five primary sources beyond the supplied NEAR and Hyperliquid pages. All were fetched and inspected on 2026-09-29; ICS-004, ERC-7683 and the CIDR paper were retrieved as raw text/PDF and quoted from the retrieved bytes.

| ID | Source | Exact URL | Kind |
|---|---|---|---|
| **S1** | IBC, ICS-004 *Channel & Packet Semantics* | `https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md` | Interchain standard |
| **S2** | Circle, *CCTP Technical Guide* | `https://developers.circle.com/cctp/references/technical-guide` | Official protocol/API spec |
| **S3** | Stripe, *Idempotent requests* | `https://docs.stripe.com/api/idempotent_requests` | Official API spec |
| **S4** | ERC-7683, *Cross Chain Intents* (Draft, Standards Track) | `https://eips.ethereum.org/EIPS/eip-7683` | Ethereum standard (draft) |
| **S5** | P. Helland, *Life beyond Distributed Transactions: An Apostate's Opinion*, CIDR 2007 | `https://www.cidrdb.org/cidr2007/papers/cidr07p15.pdf` | Systems paper |

### 2.1 Source facts (verbatim or close paraphrase, with attribution)

**S1 — IBC ICS-004.** *Facts:*
- Packet identity is `(sourcePort, sourceChannel, destPort, destChannel, sequence)`; the sending chain stores a hash commitment over the packet data together with `timeoutHeight` and `timeoutTimestamp` rather than the packet itself, under `packetCommitmentPath(portIdentifier, channelIdentifier, sequence)`.
- "All channels provide exactly-once packet delivery, meaning that a packet sent on one end of a channel is delivered no more and no less than once, eventually, to the other end."
- On successful receive over an unordered channel the destination writes a sentinel `SUCCESSFUL_RECEIPT` under a receipt path.
- Timeout requires a **positive proof about the destination's state**, not a clock reading alone. "In the case of an ordered channel, `timeoutPacket` checks the `recvSequence` of the receiving channel end and closes the channel if a packet has timed out. In the case of an unordered channel, `timeoutPacket` checks the absence of the receipt key (which will have been written if the packet was received)." The pseudocode comments read `// ordered channel: check that packet has not been received` and `// unordered channel: verify absence of receipt at packet index`.
- "Packets are delivered exactly once, assuming that the chains are live within the timeout window, and in case of timeout can be timed-out exactly once on the sending chain."
- For the `ORDERED_ALLOW_TIMEOUT` channel type: "the relayer must first attempt the receive on the destination chain before the timeout receipt can be written and subsequently proven on the sender chain in `timeoutPacket`."

**S2 — Circle CCTP.** *Facts:*
- Message passing is three steps: source component emits a message; "Circle's offchain attestation service signs the message"; the destination component receives it and forwards the body to the recipient. On EVM: `TokenMessengerV2#depositForBurn` → `MessageTransmitterV2#sendMessage`; "After sufficient block confirmations, Circle's offchain attestation service, Iris, signs the message. An API consumer must query this attestation and submits it onchain to the destination domain's `MessageTransmitterV2#receiveMessage` function."
- "A CCTP nonce is a unique identifier for a message that can only be used once on the destination domain. Circle assigns CCTP nonces offchain."
- The header carries `version, sourceDomain, destinationDomain, nonce (bytes32), sender, recipient, destinationCaller ("Address permitted to call MessageTransmitterV2 on destination domain, or bytes32(0) if message can be received by any address"), minFinalityThreshold, finalityThresholdExecuted, messageBody`.
- The burn body carries `maxFee` ("Maximum fee to pay on the destination domain") and `feeExecuted` ("Actual fee charged on the destination domain … (capped by `maxFee`)"), plus `expirationBlock` and `hookData`.
- Finality is an explicit, typed, *recipient-chosen* dial: "Implement `IMessageHandlerV2#handleReceiveFinalizedMessage` to receive messages with `finalityThresholdExecuted ≥ 2000`. Implement `IMessageHandlerV2#handleReceiveUnfinalizedMessage` to receive messages with `finalityThresholdExecuted < 2000`. This distinction allows the recipient to control the level of finality it requires before accepting a message."
- "An expired Fast Transfer burn is not permanently stuck. As long as the burn transaction still exists on the source blockchain, you can call `POST /v2/reattest/{nonce}` at any time to request a new attestation with a refreshed `expirationBlock`. There is no deadline after which re-attestation becomes unavailable."

**S3 — Stripe.** *Facts:*
- "Stripe's idempotency works by saving the resulting status code and body of the first request made for any given idempotency key, regardless of whether it succeeds or fails. Subsequent requests with the same key return the same result, including `500` errors."
- Keys are client-generated, up to 255 characters; V4 UUIDs suggested; "Avoid using sensitive data … as idempotency keys."
- "You can remove keys from the system automatically after they're at least 24 hours old. We generate a new request if a key is reused after the original is pruned."
- "The idempotency layer compares incoming parameters to those of the original request and errors if they're not the same to prevent accidental misuse."
- "We save results only after the execution of an endpoint begins. If incoming parameters fail validation, or the request conflicts with another request that's executing concurrently, we don't save the idempotent result because no API endpoint initiates the execution. You can retry these requests."

**S4 — ERC-7683 (Draft).** *Facts, including a significant negative fact:*
- The current draft "defines a solver-facing interface for intent protocols. A protocol exposes orders as opaque payloads and provides a *resolver* contract that translates those payloads into a common order representation." An order's requirements are "a list of **steps** and a list of **variables**", with acyclic hard dependencies: "a step MUST NOT execute before its hard dependencies have successfully executed."
- `RevertPolicy` has two values; for `ignore`, "The step MUST be considered executed. The solver MAY safely skip this action and proceed with fulfillment. An included reverted transaction MUST NOT be required."
- `TimingBounds` constrains `block.number` or `block.timestamp` inclusion windows.
- A resolver "MUST guarantee that an order may only abort as explicitly specified in revert policies. If no abort policy is triggered, a solver that begins to execute the steps of an order MUST be able to fulfill all requirements and receive all payments." Anything the resolver cannot check is surfaced as a named `Assumption` that "A solver MUST validate … before fulfilling the order."
- **Negative fact:** the current draft defines **no** stable order identifier, no open/fill deadline pair, no duplicate-fill rule, no partial-fill representation and no idempotency semantics. (My earlier recollection of a `ResolvedCrossChainOrder` / `orderId` / `fillDeadline` structure corresponds to an *older* revision; the live document at the URL above has been rewritten around resolvers, steps and assumptions. The fetched raw text is the authority here.)

**S5 — Helland, CIDR 2007.** *Facts:*
- "Since any message ever sent may be delivered multiple times, we need a discipline in the application to cope with repeated messages… In practice, the low-level management of this knowledge rarely occurs; messages may be delivered more than once."
- "The processing of a message is idempotent if a subsequent execution of the processing does not perform a substantive change to the entity. This is an amorphous definition which leaves open to the application the specification of what is and what is not substantive."
- Natural vs engineered idempotence: read-only messages "are naturally idempotent"; for the rest, "The application must include mechanisms to ensure that these, too, are idempotent. This means remembering in some fashion that the message has been processed so that subsequent attempts make no substantive change."
- On at-most-once acceptance: "there must be some unique characteristic of the message that is remembered to ensure it will not be processed more than once. **The entity must durably remember the transition from a message being OK to process into the state where the message will not have substantive impact.**"
- "In addition to remembering that a message has been processed, if a reply is required, the same reply must be returned. After all, we don't know if the original sender has received the reply or not."
- An *activity* is "the local information needed to manage a relationship with a partner entity"; "Transactions cannot be assumed across these entities."

### 2.2 Supplied-page facts I rely on but did not re-fetch

The NEAR and Hyperliquid facts below are taken **as characterized by the two supplied Moriarty reports** (both dated 2026-09-29, both recording Scrapling 0.4.15 captures with SHA-256 digests in their `SOURCES.md`). I did not re-retrieve these pages, so I treat them as second-hand within this memo and cite the report as well as the URL:

- HyperEVM→HyperCore: "CoreWriter emits a versioned action log; some order and vault actions are delayed before Core execution and appear first as enqueued, then executed" (COMPARISON.md row "App-to-core interface"; `https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore`, `…/interaction-timings`).
- API/agent wallets: "nonce state is per signer. The docs warn that pruning a deregistered agent's nonce state can make old actions replayable if its address is reused" (COMPARISON.md; `https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets`).
- Exchange API: distinct typed actions, and "order responses distinguish resting from filled" (COMPARISON.md; `https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint`).
- Builder codes: an approved, revocable per-builder maximum that later order-specific fees must respect (COMPARISON.md; `https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes`).
- Oracle: validators publish inputs used in funding, margin and liquidations (COMPARISON.md; `https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/oracle`).
- NEAR BTC refund: "requires a nonfinalized deposit, a timelock, a contract action, MPC signing and relayer broadcast… charges a storage deposit that is not returned" (PROPOSAL.md; `https://docs.near-intents.org/integration/bridging/btc-deposit-refund`).
- NEAR Chain Signatures: the caller requests a threshold signature over a supplied payload and derivation path, then constructs and broadcasts the chain-specific transaction itself (PROPOSAL.md; `https://docs.near.org/chain-abstraction/chain-signatures`).

---

## 3. Operation identity

### 3.1 Four levels, four different jobs

A single identifier cannot simultaneously be stable across retries, unique per network message, and meaningful to a foreign domain. The kernel needs four, and the discipline is in never substituting one for another.

| Level | Name | Derivation | Stability | Job |
|---|---|---|---|---|
| 1 | `intent_id` | `H(canonical_envelope_bytes)` over program+version+domain, objective, caps, grants, nonce, expiry | One per signed user authorization | Root of all authority accounting; the replay-consumption unit at the language level |
| 2 | `leg_id` | `H(intent_id ‖ plan_commitment ‖ leg_index ‖ leg_class ‖ payload_constraint_commitment)` | **Stable across every retry, every failover, every solver** | **The unit of at-most-once *substantive* effect.** The reconciliation key. Helland's *activity* key (S5) |
| 3 | `attempt_id` | `H(leg_id ‖ attempt_counter ‖ adapter_id@version ‖ exact_payload_hash ‖ signer_key_epoch ‖ deadline)` | **Never reused, for any reason** | The unit of network dispatch and of blame. Distinguishes "we sent it twice" from "it happened twice" |
| 4 | `external_ref` | Observed, not derived | Whatever the domain says | The domain's own name for the effect: tx hash, CoreWriter queue index, CCTP `nonce`, IBC `(port, channel, sequence)`, venue order id / `cloid`, oracle `(feed, round)` |

Two auxiliary identities are needed for bridge legs specifically: a `claim_id` (the entitlement the user holds against a route) and a destination `nullifier` (the thing whose consumption proves at-most-once delivery). The NEAR proposal already demands both — "claim nullifier, cumulative entitlement" — and the point of separating them from `leg_id` is that the entitlement can outlive many legs and attempts.

*Inference.* `plan_commitment` must be inside `leg_id`, not outside it. If `leg_id` were merely `H(intent_id ‖ leg_index)`, a re-planned workflow would reuse identity across two materially different payloads, and the kernel's own dedupe would then suppress a *legitimately different* second effect — or, worse, admit a substituted one under a trusted id. Binding the payload *constraint* commitment (not the exact payload) is the right granularity: it is stable across gas-price bumps and across two solvers competing on the same leg, but changes if the recipient, asset, cap or action class changes.

### 3.2 The rule that makes the scheme load-bearing

> **`attempt_id` is written durably, before the send, and never erased. `leg_id` is what the domain is asked about during reconciliation. `external_ref` is the only thing that may be counted as an effect.**

The write-ahead requirement is S5's central operational sentence applied literally: "The entity must durably remember the transition from a message being OK to process into the state where the message will not have substantive impact." A kernel that records the attempt *after* a successful send has a crash window in which a real external effect exists with no local handle at all — hostile trace H3.

### 3.3 Solver quotes are deliberately outside the identity scheme

A quote is not an operation. It has an identity (`quote_id`, `H(solver_pk ‖ leg_id ‖ terms ‖ validity_window)`) and a signature, but it creates no `attempt_id` and touches no authority axis. This follows the consolidated design directly: "An uncommitted candidate alone changes no ledger state or obligations."

Three operational consequences, all of them *inferences*:
1. **Quotes are naturally idempotent** in S5's sense — re-requesting a quote performs no substantive change — so quote retry needs no machinery at all. This is the only mechanism in this memo with that property, and it is worth saying explicitly so that the machinery is not reflexively applied where it costs latency for nothing.
2. **A quote is a refinement obligation, not a promise.** Its terms must *refine* the signed envelope (tighter caps, same recipient, same asset identity). The check belongs in `validatePlan`, and it must be re-run at reservation time because quote freshness is, per the NEAR proposal, an "operational promise" and nothing more.
3. **S4 is a cautionary example here.** ERC-7683's resolver "MUST guarantee that … a solver that begins to execute the steps of an order MUST be able to fulfill all requirements and receive all payments." That is a strong liveness guarantee issued by a contract the kernel does not control, resting on named `Assumption`s the solver "MUST validate … before fulfilling." A Moriarty kernel may consume such a guarantee as an *input to quote ranking*; it must never let it stand in for destination evidence. The guarantee is about the resolver's honesty, not about what the ledger did.

---

## 4. Idempotency classes: a required adapter declaration

Idempotency is a property of the **destination domain**, never of the kernel. The kernel's only genuine choice is whether to retry, and that choice is determined by what the domain natively enforces. I propose that every adapter be required to declare, per action class, exactly one of five classes — and that an adapter that declines to declare be inadmissible.

| Class | Meaning | Reference instances | Retry rule |
|---|---|---|---|
| **N** — native nullifier | Destination durably consumes a unique key; a second delivery is *rejected by the domain* | IBC receipt at `(port, channel, sequence)` (S1); CCTP `nonce` — "can only be used once on the destination domain" (S2) | **Unbounded automatic retry is safe.** Duplicate dispatch is absorbed by the domain |
| **D** — dedupe window | Server stores the first result under a client key for a bounded period and replays it | Stripe: first status+body saved "regardless of whether it succeeds or fails"; params compared and error on mismatch; pruned at ≥24h (S3) | **Automatic retry only inside the window, with byte-identical parameters.** After pruning, a retry is a *new effect* — S3 says so outright: "We generate a new request if a key is reused after the original is pruned" |
| **S** — scarce ordered nonce | A per-signer sequence number is the dedupe mechanism, and it is a consumable resource | EVM account nonce; Hyperliquid API-wallet per-signer nonce state (COMPARISON.md) | **Automatic retry only with the identical nonce and identical payload** (a true rebroadcast). A retry that takes a *fresh* nonce is a second effect and must be treated as one |
| **C** — client order id | Venue rejects a duplicate client-supplied id, with venue-defined retention | Venue `cloid`-style fields (COMPARISON.md, Exchange endpoint) | Retry safe while the venue retains the id; the retention period must be declared, not assumed |
| **O** — none | No duplicate suppression whatsoever | Naive "send and hope" adapters; any fire-and-forget notification | **Automatic retry forbidden.** See below |

**The retry rule, stated once:**

> A leg may be retried automatically **iff** its class is **N**, or **D** within the declared window with identical parameters, or **S** with an identical nonce and payload. In every other case, a retry requires a *completed reconciliation read* whose evidence class is at least the class required for that leg's terminal acceptance. If no such read is available, the leg's effect coordinate stays `indeterminate` and **only compensation or escalation may proceed** — never a re-dispatch.

*Inference, and the part I consider most likely to be argued with:* the reconciliation read must be **at least as strong as terminal acceptance**, not merely "best available". A weaker read is precisely the evidence downgrade the consolidated design forbids ("Signed fallback policies must not become an unsigned evidence downgrade"). If a leg's terminal acceptance demands a finalized destination observation, then an unfinalized RPC read may *delay* a retry but may never *authorize* one. Otherwise the kernel has invented a rule where uncertainty plus impatience equals permission — the same error, structurally, that the Aeon experiment falsified for refunds (§8).

**Two consequences worth naming.**

*S3's "not saved" case is a trap.* Stripe: "If incoming parameters fail validation, or the request conflicts with another request that's executing concurrently, we don't save the idempotent result." So a class-**D** adapter has a sub-state in which the key exists, the call failed, and *no* result was stored — meaning the next attempt is not a replay but a fresh execution. The adapter must distinguish "replayed stored result" from "executed fresh under a reused key", and only the former may be treated as evidence about the *first* attempt. A kernel that conflates them will read a fresh success as confirmation of an older one.

*S5's reply rule generalizes to receipts.* "if a reply is required, the same reply must be returned. After all, we don't know if the original sender has received the reply or not." The kernel is on both sides of this: it must replay its own `OperationReceipt` for a repeated caller request under the same `leg_id`, and it must expect adapters to replay theirs. A receipt that differs between two reads of the same `leg_id` is itself a reconciliation *fault*, not a data refresh.

---

## 5. Two state machine designs

### 5.1 Design A — the monolithic lifecycle enum

This is the design the NEAR proposal sketches: "Use distinct states for `planned`, `reserved`, `signed`, `submitted`, `included`, `finalized`, `delivered`, `refunding`, `refunded`, `failed` and `unknown`." One coordinate per leg, roughly twelve to fifteen values, transitions guarded by evidence predicates.

```
planned → reserved → authorized → signed → submitted → included
        → finalized → delivered → settled
            ↘ failed          ↘ refunding → refunded
            ↘ unknown ──────────────↗ (?)
```

**Real strengths.** It is small; it renders in one column of a UI; it is cheap to commit to a ledger as a single tagged value; an auditor can read it aloud. For a language whose acceptance predicate must be encoded into ZKIRv3 constraints and whose signing displays "must make the canonical intention inspectable" (consolidated design, "Vision"), these are not cosmetic advantages.

**The structural defect.** A single coordinate forces three logically independent facts to share one slot:
- how much **authority** has been consumed,
- how far the **dispatch** has got,
- what is **known about the effect**.

These have different algebras. Authority consumption is monotone and irreversible. Dispatch is a *set* (multiple attempts may be live at once). Effect knowledge is a partial order that can *increase* long after dispatch has stopped. Projecting all three onto one chain means each transition silently overwrites two facts it was not entitled to touch.

The defect becomes concrete at `failed` and `unknown`. In A, `failed` is a sibling of `submitted`, so entering it **destroys the dispatch record's status as live**, which is exactly the handle a late success needs. A therefore requires a resurrect-from-`failed` edge — and in my experience that edge, added late and under pressure, is where real systems break, because by the time it is added the code that fired on `failed` has already released reservations and notified the user. Likewise `unknown` in A is a single value with no cardinality, so it cannot express "one attempt unknown, one attempt confirmed-failed, cumulative delivery 0.4" — which is the actual state during hostile traces H2 and H3.

*Inference:* A can be made safe, but only by decorating it with so many side-tables (attempt list, cumulative delivery, evidence set, authority ledger) that the enum stops being the state and becomes a *label* on state held elsewhere. At that point the enum is a lie that fits in a column, and the invariants live in code nobody audits.

### 5.2 Design B — the orthogonal product machine (recommended)

Give each leg **three independent coordinates** plus an evidence qualifier on every observation.

**Axis 1 — Authority** (monotone, never reversed):

```
unauthorized → reserved → authorized → consumed → {released | expired | tombstoned}
```

`reserved` holds *spent + pending* exposure against the authenticated budget, per the consolidated design's "Concurrent solvers reserve spent plus pending exposure against the same authenticated budget." Crucially, **`consumed` is never undone by a refund**: "A refund cannot replenish authority to evade a gross cap unless that replenishment was explicitly authorized."

**Axis 2 — Dispatch** (a *set* of attempts, not a scalar):

```
undispatched → in-flight{attempt_id → sub-state} → quiesced
   sub-state ∈ { prepared, sent, source-accepted, source-rejected, no-answer }
```

`quiesced` means "the kernel has stopped sending", which is an operational fact about the kernel. It carries **no** implication about the world. This is the single most important representational decision in the memo.

**Axis 3 — Effect** (the only axis that may gate financial acceptance):

```
none → partial(q, ev) → complete(q*, ev) → excluded(non-occurrence certificate)
                    ↘ indeterminate ↗
```

`indeterminate` is a distinct inhabitant, **not** a flavour of `none`. `none` asserts that nothing happened, which is a claim requiring evidence. `indeterminate` asserts only that the kernel does not know. Collapsing these two is the root cause of every refund bug in §7.

**Evidence qualifier** on every effect observation — `(evidence_class, issuer, threshold, height_or_round, freshness, finality_level)` — with `evidence_class` drawn from an ordered lattice: `in-circuit proof > light-client proof > threshold attestation > signed operator report > unauthenticated RPC read`. S2 shows this is not theoretical: CCTP makes the finality level a *typed field in the message* (`minFinalityThreshold` vs `finalityThresholdExecuted`) and dispatches to a different recipient handler depending on it, so "the recipient [can] control the level of finality it requires before accepting a message." Moriarty should do the same, at the language level, for every observation class.

**Standing invariant**, checked at every transition on every axis:

```
delivered_cumulative + refunded + in_flight_entitlement  ≤  authorized_entitlement
fee_actual_cumulative                                    ≤  signed_fee_cap
```

The first conjunct is the Aeon-checked conservation relation (§8), generalized from a single leg to the cumulative case. The second is directly modelled on CCTP's `feeExecuted ≤ maxFee` (S2) and on builder-code approved maxima (COMPARISON.md) — in both, the *authorized* cap and the *actual* charge are separate fields, and only the actual one enters the outcome.

### 5.3 Why B, concretely

Four claims, each checkable against a trace in §7.

1. **Timeout cannot cause a refund, by construction.** Refund requires the effect axis to reach `excluded`, and `excluded` requires a non-occurrence certificate. A clock produces no such certificate. S1 makes this a protocol requirement rather than a preference: timing out demands `verify absence of receipt at packet index`, a *membership* proof about destination state. In A, refund-on-timeout is a one-line mistake; in B it does not typecheck. (H1.)
2. **Late success is an ordinary forward transition.** A quiesced dispatch with an `indeterminate` effect that later becomes `complete` has moved forward on one axis and not moved at all on another. No resurrection edge is needed. S2 establishes that this is a live production scenario and not a thought experiment: an expired burn can be re-attested "at any time", with "no deadline after which re-attestation becomes unavailable." (H1.)
3. **Duplicate dispatch is representable and therefore detectable.** Because the effect axis sums over **distinct `external_ref`s**, two attempts that the domain collapses into one nullifier consumption count once, and two attempts that produce two consumptions count twice — which immediately violates the standing invariant and raises a fault. In A, the second effect has nowhere to be recorded, so it is either dropped or silently netted. (H3.)
4. **Partial delivery has a home.** `partial(q, ev)` carries both the quantity and the evidence that established it, and is distinct from `complete(q*, ev)` where `q* = q`. The distinction is not pedantry: the Aeon experiment shows that `refund = sent − delivered` is arithmetically fine but semantically unsafe unless `delivered` is *final and authoritative*, and only the `complete` constructor asserts that. (H2.)

### 5.4 Honest costs of B, and Design C

**Costs.** Three coordinates plus an evidence set plus an attempt set is roughly 5–8× the state of A per leg. That hurts in three specific places: persistence volume; signing-display legibility (a user cannot be shown a product machine); and, most seriously, **Midnight acceptance encoding**, where every extra field is circuit cost. B is also harder to explain, and a design nobody can explain is a design that gets bypassed under incident pressure.

**The mitigation, which is part of the recommendation:** B is the *kernel's internal* model. What Midnight accepts is a **small typed projection** of B — plausibly `(authority_consumed, effect_class ∈ {none, partial, complete, excluded, indeterminate}, cumulative_quantity, evidence_class, fee_actual)` — chosen so that the projection is a *homomorphism*: every legal projected transition must be the image of a legal B transition, and no B state may project into a projected state that permits more than B does. That obligation is stated here and **not discharged**; see open decision (a).

**Design C — pure event log, state as a fold.** Persist only an append-only, content-addressed log of observations and dispatches; derive all three coordinates by folding. This is strictly more faithful than B (B *is* the fold's codomain) and is excellent for audit and for replaying an incident. Two reasons I do not recommend it as the normative model: the acceptance predicate then quantifies over a log of unbounded length, which fights Moriarty's "Every stage terminates within a checked bound" and its "Local finite arrays and append-only IDs require either explicit finite episodes or authenticated rollover"; and there is no fixed-size thing to show a user at signing time. C is the right *storage* layer under B, not a replacement for it.

---

## 6. Phase transitions and the evidence that permits each

Notation: `A:`/`D:`/`E:` = authority, dispatch, effect axis. Every row's third column is a **hard** negative: the listed item must not be accepted as sufficient.

### 6.1 Common preamble (all leg classes)

| # | Transition | Evidence that permits it | Explicitly insufficient |
|---|---|---|---|
| T1 | `quote_id` recorded | Solver signature over terms + validity window | A quote is not a reservation; an ERC-7683-style resolver liveness guarantee (S4) is not destination evidence |
| T2 | `A: unauthorized → reserved` | Exposure (spent + pending) fits the authenticated budget; `plan_commitment` fixed | Optimistic reservation against an unread budget |
| T3 | `A: reserved → authorized` | **`validatePlan` passes**: leg class, target/domain, recipient, asset identity, caps, adapter@version, evidence profile all refine the signed envelope | Operator approval; a previously valid plan whose adapter version has since changed |
| T4 | `D: undispatched → prepared` | `attempt_id` **durably written** with payload hash, adapter@version, key epoch, deadline (S5) | Nothing. This write is unconditional and precedes everything |
| T5 | signer call: payload signed | Signature over the pinned canonical digest, binding chain id, derivation path, recipient, value, nonce, key epoch, validity window | **A signature is not an execution.** NEAR Chain Signatures returns a signature; the caller still constructs and broadcasts (PROPOSAL.md) |
| T6 | `D: prepared → sent` | Adapter transmitted bytes | A send is not an acceptance |
| T7 | `D: sent → source-accepted` | Source-domain inclusion of the submitting transaction / venue ack with `external_ref` | Mempool acceptance; HTTP 200 without an `external_ref` |
| T8 | `D: * → no-answer` | Deadline elapsed with no `external_ref` | — (this is a kernel-local fact and permits nothing on the effect axis) |
| T9 | `D: in-flight → quiesced` | Retry budget exhausted, or class rule (§4) forbids further automatic retry | Does **not** set `E: none` |
| T10 | `E: * → indeterminate` | Any transition to `quiesced` with no terminal effect evidence | — |
| T11 | `A: authorized → consumed` | First `source-accepted` on any attempt for the leg | Consumption is **not** reversed by later failure or refund |

### 6.2 Native venue order (Hyperliquid-like `native-financial-call`)

| # | Transition | Evidence | Insufficient |
|---|---|---|---|
| V1 | `E: none → resting` | Venue order response reporting a resting order, with venue order id | Order responses "distinguish resting from filled" (COMPARISON.md): **a resting order is an open duty, not an effect** |
| V2 | `E: resting → partial(q)` | Venue fill record(s) with cumulative filled quantity and actual fee | Sum of fills the kernel inferred from its own dispatches |
| V3 | `E: partial(q) → complete(q*)` | Venue **authoritative order-state read** showing a terminal order state with final cumulative fill | The last fill message observed. Absence of further messages is not terminality |
| V4 | fee accounting | `fee_actual` from the venue record, checked against the signed cap | The quoted fee. (CCTP separates `maxFee` from `feeExecuted`; builder codes separate approved maximum from charged fee) |
| V5 | cancellation | See §6.6 | A cancel ack |

### 6.3 CoreWriter-like same-domain queued call

| # | Transition | Evidence | Insufficient |
|---|---|---|---|
| Q1 | `D: sent → source-accepted` | EVM-side transaction included and the versioned action-log event emitted | — |
| Q2 | `E: none → enqueued` | The queue event, with action version and queue position/index as `external_ref` | — |
| Q3 | `E: enqueued → executed` | The Core-side execution record for that queued action | **Enqueue never implies execution.** Per COMPARISON.md, "some order and vault actions are delayed before Core execution and appear first as enqueued, then executed", so "`submitted`/`enqueued` and `executed` must be separate receipts" |
| Q4 | ordering | Queue position is the *only* ordering fact; the kernel's send order is not | Two legs enqueued in one block may execute in either relative order unless the venue states otherwise |
| Q5 | version binding | Action version pinned at authorization (T3) and re-checked at Q1 | A version bump between authorization and dispatch invalidates the authorization |

*Inference:* the queued call is the cheapest available rehearsal for every asynchronous hazard in this memo — enqueue/execute split, reordering, late execution — and it happens **on one chain with no bridge and no foreign signer**. It is therefore the right first adapter for the U5 slice, ahead of anything cross-domain.

### 6.4 Bridge leg

| # | Transition | Evidence | Insufficient |
|---|---|---|---|
| B1 | `E: none → source-committed` | Source lock/burn event; commitment recorded; `claim_id` bound | A source transfer is not a delivery — and per PROPOSAL.md, a verifier-internal credit is a *third*, distinct fact |
| B2 | `→ attested` | Attestation over the message at the **required** finality level, from the named issuer | An attestation is an issuer's statement. S2: Iris signs "after sufficient block confirmations"; the signature attests confirmations, not correctness of downstream execution |
| B3 | finality gate | `finalityThresholdExecuted ≥ signed_minimum` (S2 pattern) | An attestation at a *lower* finality level than the signed policy demands. This is the evidence downgrade (H5) |
| B4 | `→ delivered_partial(q)` / `delivered_complete(q*)` | Destination receive succeeded **and the nullifier is consumed** (S1 receipt sentinel; S2 `usedNonces`) | A destination RPC balance read; a relayer's report |
| B5 | `→ excluded` | **Positive non-occurrence certificate**: S1-style absence-of-receipt proof at the packet index, or the ordered-channel `recvSequence` check, or an equivalently strong route-specific exclusivity certificate | **Timeout. Elapsed deadline. Operator report of non-arrival. An empty observation.** All four are rejected by the Aeon experiment and by S1's proof requirement |
| B6 | `excluded → refunding → refunded` | `refund ≤ authorized_entitlement − delivered_cumulative`, plus the route's own refund preconditions and **costs** | A refund is a *costed workflow*: NEAR's BTC refund needs a nonfinalized deposit, a timelock, a contract action, MPC signing and a relayer broadcast, and charges a non-returned storage deposit (PROPOSAL.md) |
| B7 | representation | Destination asset identity and representation ratio checked against the signed route | "Wrapped X" is not X; a generic success Boolean is inadequate (PROPOSAL.md) |

Note the asymmetry S1 forces and that I think Moriarty should adopt verbatim: **proving something happened and proving it did not happen are both positive proofs**, and the second is often the harder one to obtain. Any route that cannot produce B5 evidence is a route on which refund is *not available*, and the signed policy must say so up front rather than discovering it during an incident.

### 6.5 Oracle import

| # | Transition | Evidence | Insufficient |
|---|---|---|---|
| O1 | observation admitted | `(feed_id, round_id, issuer_set, threshold, timestamp)` matching the signed evidence profile, within the declared freshness window | A price fetched at convenience; an unnamed "latest" |
| O2 | dependent financial transition | The *selected round* was committed **before** the dependent decision | Choosing among several valid rounds after seeing the outcome — the prover must not pick a favourable premise (consolidated design: "a prover cannot choose a weaker relation") |
| O3 | — | — | **A threshold signature over a price is not the price.** COMPARISON.md: the kernel "cannot turn it into an objective fact by labeling it `verified`"; consolidated design: "threshold signatures cannot establish the truth of an oracle" |

### 6.6 Cancellation

Cancellation is **its own leg**, with its own `leg_id` and its own three axes. This is not bookkeeping fussiness — it is what makes the cancel/fill race representable.

| # | Transition | Evidence | Insufficient |
|---|---|---|---|
| C1 | cancel authorized | The envelope grants cancel authority for this leg class and target | An operator's judgement that cancelling is prudent |
| C2 | cancel `D: → source-accepted` | Venue accepted the cancel request | — |
| C3 | **target leg** `E: → complete(q*)` with `q* = q_filled_at_cancel` | Venue authoritative order-state read after the cancel settles | **A cancel acknowledgement is not a proof of non-fill.** The venue may have filled before processing the cancel; through a queued interface (§6.3) the cancel may even *execute after* the fill |
| C4 | residual duty released | The authoritative read shows a terminal state and a final cumulative fill | The cancel ack. Releasing a duty on the ack is H4 |

### 6.7 Compensation and recovery

| # | Rule | Basis |
|---|---|---|
| R1 | Compensation is a **new authorized action** with a fresh `leg_id`, never a rollback of an old one | Consolidated design: "Cross-domain workflow uses authenticated progress, conditional release and compensation; it must never claim global rollback from a local model" |
| R2 | Recovery authority is **separately scoped** and may outlive ordinary authority: "Ordinary authority can expire while a narrowly scoped recovery authority remains usable under its signed conditions" | Consolidated design |
| R3 | Revocation does not erase duties; an unavailable recovery path stays explicit rather than being silently substituted | Consolidated design |
| R4 | Exclusive terminal outcomes **consume/tombstone** the applicable authority, so a late success and a completed compensation cannot both be honoured | Consolidated design; S1's "a packet … can be timed-out exactly once" is the same exclusivity at protocol level |
| R5 | Compensation is costed and may fail; a compensation leg runs the *same* three-axis machine, including its own `indeterminate` | PROPOSAL.md refund-path costs; no special-casing |
| R6 | Manual escalation is a **typed terminal-for-now**, not a state erasure | Inference |

---

## 7. Traces

Format: `A/D/E` = the three coordinates. Each hostile trace names the rejecting judgment or the retained state, and contrasts Design A with Design B.

### 7.1 Positive traces

**P1 — Direct Midnight leg, no kernel at all.**

```
authorize(envelope) → A: authorized
compile → prove → submit                    D: prepared → sent → source-accepted
ledger inclusion, verifier accepts          E: none → complete(q*), ev = in-circuit proof
                                            A: consumed
```

Nothing in the workflow touched a solver, a signer service or a bridge. This trace is included first deliberately: the kernel "is not a prerequisite for a valid direct Midnight program" (PROPOSAL.md), and a protocol design that cannot express the no-kernel path has already over-reached. The evidence class here is the strongest one in the lattice, and it is the *only* trace in this memo that reaches it.

**P2 — Native venue order, partial fill, cancelled remainder.**

```
quote (naturally idempotent, no attempt)         quote_id recorded
validatePlan: limit, qty, recipient, fee cap ✓   A: reserved → authorized
dispatch order (class C, cloid = leg_id)         D: prepared → sent → source-accepted
venue: resting                                   E: resting            [V1: open duty, not an effect]
fills 40%                                        E: partial(0.40), ev = venue record
cancel leg (own leg_id) dispatched               C1–C2
authoritative order-state read: terminal, 40%    E: complete(q* = 0.40)  [V3, C3]
fee_actual ≤ signed cap ✓                        [V4]
residual 60% duty released                       [C4] A: consumed(0.40), released(0.60)
```

The user's authority was consumed for 40% and released for 60%; the net outcome check ran against `fee_actual`, not the quote.

**P3 — Bridge leg, clean.**

```
burn on source, claim_id bound                   E: source-committed          [B1]
attestation, finalityThresholdExecuted ≥ min     E: attested                  [B2, B3]
destination receive; nullifier consumed          E: delivered_complete(q*)    [B4]
feeExecuted ≤ maxFee ✓                           invariant holds
claim closed; entitlement exhausted              A: consumed, tombstoned      [R4]
```

Delivered = entitlement, refunded = 0, invariant satisfied with equality. Evidence class: threshold attestation plus destination nullifier consumption — strictly weaker than P1, and the signed policy must have said so in advance.

### 7.2 Hostile traces

**H1 — Late success after timeout.** *(mandated)*

```
t0    bridge leg dispatched                      D: sent → source-accepted, E: source-committed
t0+2h attestation service degraded; no evidence  D: no-answer → quiesced
      ── Design A ──
      leg marked `failed`; operator triggers refund of the full amount
      later: destination delivers in full
      RESULT: delivered = sent AND refunded = sent  →  delivered + refund = 2·sent
      This is exactly the Aeon counterexample: `sent=1, delivered=1`, full refund Rejected.
      ── Design B ──
      D: quiesced (kernel-local fact only);  E: indeterminate            [T9, T10]
      refund attempted → REJECTED: refund requires E = excluded,
      and excluded requires a non-occurrence certificate [B5]. A clock produces none.
t0+3h re-attestation obtained; destination receive; nullifier consumed
      E: indeterminate → delivered_complete(q*)   — an ordinary forward move
      invariant: delivered + 0 ≤ entitlement ✓
```

**Rejecting judgment:** B5 (refund requires a non-occurrence certificate). **Retained state:** `E: indeterminate`.
The scenario is not hypothetical: S2 states that an expired Fast Transfer burn "is not permanently stuck" and that re-attestation is available "at any time", with "no deadline after which re-attestation becomes unavailable." A protocol whose safety depends on late delivery being impossible is contradicted by the primary source.

**H2 — Partial destination delivery.** *(mandated)*

```
entitlement = 100. Destination delivers 60; the route stalls.
      ── naive ──
      "delivery failed" → refund 100.
      delivered + refund = 160 > 100.  Aeon: full refund after positive partial delivery → Rejected.
      ── slightly-less-naive ──
      refund 40 immediately on the strength of the observed 60.
      Still wrong: the observation is `partial(60)`, not `complete(60)`.
      If a further 30 lands, delivered + refund = 130 > 100.
      ── Design B ──
      E: partial(60, ev = destination receipt)
      refund attempted → REJECTED: refund requires excluded, or complete(q*) — a
      *final and authoritative* count. `partial` asserts no finality of the count.
      Path 1: obtain an exclusivity certificate for the residual 40 → E: complete(60)
              → refund ≤ 100 − 60 = 40. ✓
      Path 2: no such certificate exists on this route → E stays partial;
              refund unavailable; escalate under recovery authority [R6].
```

**Rejecting judgment:** the `complete` vs `partial` constructor distinction plus the standing invariant. This is precisely the Aeon experiment's own stated limit, promoted to a protocol rule: "The `refund_remaining` case also requires a final, complete delivery count; if additional delivery can arrive after refund authorization, this simple relation is insufficient."

**H3 — Duplicated dispatch.** *(mandated)*

```
Kernel writes attempt_1 durably [T4], sends, then crashes before recording any external_ref.
On restart: leg_id known, attempt_1 known as `sent`, external_ref unknown.

Class N (IBC / CCTP):  auto-retry permitted. attempt_2 either lands on an unconsumed
  nullifier (one effect) or is rejected by the domain. S1: "delivered no more and no
  less than once"; S2: the nonce "can only be used once on the destination domain".
  E: → complete(q*) once. Invariant holds. Safe.

Class D (Stripe-like): retry inside the 24h window with byte-identical parameters
  replays the stored status and body — "including 500 errors" (S3). Safe.
  BUT: if the key was pruned, "We generate a new request", i.e. a SECOND EFFECT. And
  if the original attempt failed validation or raced a concurrent request, no result
  was stored at all, so the retry executes fresh. Either way the class-D adapter must
  distinguish "replayed" from "freshly executed under a reused key" — treating the
  latter as confirmation of the former is a silent double-spend.

Class S (EVM / agent-wallet nonce): a true rebroadcast with the same nonce is safe. A
  retry that takes a FRESH nonce is a second effect. If both land:
      two distinct external_refs on one leg_id
      → E sums over distinct external_refs → cumulative 2q
      → delivered + refunded > entitlement  → INVARIANT VIOLATION, raised as a fault
      → freeze the leg; compensation only [R1]; no netting, no silent absorption.
  In Design A the second effect has no slot and is dropped or averaged away.

Class O: auto-retry FORBIDDEN (§4). E: indeterminate; compensation/escalation only.

Aggravating case (source: COMPARISON.md / Hyperliquid nonce docs): the external signer's
  per-signer nonce state is PRUNED after an agent is deregistered, and the address is
  later reused — old signed actions become replayable. Therefore Moriarty's consumed-
  authority record must be durable IN THE KERNEL and keyed by leg_id, never derived
  from the external signer's ephemeral nonce store. Key-epoch retirement must be
  explicit and must not garbage-collect consumption history.
```

**Rejecting judgment:** the idempotency-class retry rule (§4) plus the standing invariant. **Retained state:** `E: indeterminate` for class O; a raised fault for the class-S double landing.

**H4 — Cancel/fill race through a queued interface.**

```
Resting order, 40% filled. User cancels.
cancel leg dispatched via the queued interface     Q1: enqueued
same block: venue fills the remaining 60%
queue ordering: the cancel EXECUTES AFTER the fill  [Q3, Q4]
      ── naive ──
      cancel ack received → mark order cancelled at 40% → release the 60% residual duty
      → later reconciliation finds cumulative fill 100% against a released duty and a
        released reservation. The extra 60% is an effect with no authority behind it.
      ── Design B ──
      cancel ack moves ONLY the cancel leg's own dispatch axis [C2].
      The target leg's E may not move on a cancel ack [C3].
      Authoritative order-state read: terminal, cumulative fill 100%.
      E: partial(0.40) → complete(1.00).
      Check against the signed cap:
        within cap  → A: consumed(1.00); duty discharged; fee_actual re-checked.
        over cap    → unauthorized effect: record the liability, do NOT net it away,
                      escalate under recovery authority. A: consumed is irreversible,
                      so the excess is a *liability*, not a rolled-back transaction.
```

**Rejecting judgment:** C3 (a cancel ack is not a non-fill proof). This trace is why cancellation needs its own `leg_id`: in Design A the cancel and the order share one state slot, so "cancelled" necessarily overwrites "filling".

**H5 — Evidence downgrade during an outage, with a key-epoch rotation mid-`indeterminate`.**

```
Bridge leg: E: indeterminate after an attestation-service outage [T10].
Pressure to resolve. Three tempting moves, all rejected:

(a) Accept an operator's signed report of delivery in place of the required threshold
    attestation.
    → REJECTED at B3/§4: the reconciliation evidence class must be at least the class
      required for terminal acceptance. `signed operator report` sits strictly below
      `threshold attestation` in the lattice. Consolidated design: "Signed fallback
      policies must not become an unsigned evidence downgrade."

(b) Accept an attestation at finalityThresholdExecuted < signed minimum.
    → REJECTED at B3. S2 makes the threshold an explicit field precisely so the
      recipient can "control the level of finality it requires before accepting a
      message." Lowering it after the fact is choosing a weaker premise post hoc.

(c) Rotate the signer key epoch to a fresh quorum and re-sign the leg to "clear" it.
    → REJECTED at T5/§3: attempt_id binds signer_key_epoch, so a new epoch yields a
      NEW attempt_id, never a re-interpretation of the old one. Rotation cannot
      retroactively make an old attempt determinate, and a signature is not an
      execution in any epoch.

Meanwhile the envelope's ordinary authority EXPIRES while E is still indeterminate.
    → The leg does not become safe by expiring. Ordinary authority → expired;
      the separately scoped recovery authority remains usable under its own signed
      termination rule [R2]. Recovery may pursue B5 evidence or escalate; it may NOT
      dispatch a fresh ordinary effect.
    → If delivery later completes: E: → complete(q*), which is legal — the effect was
      authorized when dispatched; T11 consumed the authority at source-acceptance, and
      expiry of the right to *initiate* is not retroactive invalidation of an
      initiated leg.
    → If compensation had already completed under R4's tombstone, the tombstone makes
      the pair mutually exclusive and the late delivery is a recorded conflict requiring
      escalation — not a silently accepted second outcome.
```

**Rejecting judgments:** B3, §4's evidence-class floor, T5/§3's epoch binding, R2/R4. **Retained state:** `E: indeterminate` under recovery authority.

---

## 8. What the Aeon bridge experiment establishes, and what it does not

### 8.1 What it establishes

Facts, from `deliverables/aeon-kernel-experiments-2026-09-29/bridge/RESULT.md`:

- With AeonLang 4.9.0 at commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`, Z3 5.1.0.0 and cvc5 1.4.0, SMT synthesizer, synthesis budget 0, `no_main=True`, `strict_decidable=True`, no `native` escapes; probe script SHA-256 `9fcacbad85a2a40c487ed99aa81512dd94f07e58aa3d2dfbd143979385ba0feb`, raw judgments SHA-256 `df45213c73d6b8936c3b233d0fa53da2e02de142268e10aaff78e339be40bab4`.
- Under the conservation refinement `sent > 0`, `0 ≤ delivered ≤ sent`, `0 ≤ refund`, `delivered + refund ≤ sent`, four unsafe refund policies were **rejected with concrete integer counterexamples**: full refund on timeout alone (`sent=1, delivered=1`); on unknown status (`sent=1, delivered=1`); when observed delivery is zero (`sent=1, actual_delivery=1`); and after positive partial delivery (`sent=1, delivered=1`). Three were **accepted**: withhold refund while unknown; refund exactly `sent − delivered`; full refund under an input refinement asserting `delivered = 0`.

What that supports, as an inference I am willing to defend: **the rule "a timeout is not a non-receipt" is mechanically checkable at authoring time**, it is checkable *cheaply*, and the checker returns a counterexample a developer can read. That matters because this is the single rule that every hostile trace in §7.2 turns on, and because the consolidated design already commits to adopting Aeon's authoring ideas "in order: obligation/trust reports, exact advisory refinement checking with replayed counterexamples, then typed holes and bounded synthesis." The experiment is evidence that step two of that sequence pays for itself on the exact predicate the kernel most needs.

Secondarily, it establishes something about *specification*: `observed_delivery` and `timed_out` had to be modelled as variables with **no justified relationship** to actual delivery before the unsafe policies would fail. That modelling discipline — keeping the observation and the fact as separate sorts — is transferable to the kernel protocol independently of Aeon, and is the direct ancestor of Design B's `indeterminate ≠ none`.

### 8.2 What it does not establish

Stated in RESULT.md itself:

- Integer amounts are **mathematical integers**; "no token scale, rounding, fee, supply, representation ratio, or foreign chain semantics are included." Real bridges have all six, and several of the rejections could be re-derived — or lost — under directed rounding.
- Status code `1` is an **uninterpreted** operational value: "No status code proves delivery, nonreceipt, finality, or refund eligibility."
- The two interesting accepting judgments are **conditional on their premises**. `refund_remaining` "requires a final, complete delivery count; if additional delivery can arrive after refund authorization, this simple relation is insufficient." And the `delivered = 0` case passes only because "The checker trusts the input refinement; it does not verify a foreign chain." An adapter that can fabricate `delivered = 0` from a timeout defeats the whole exercise — which is why §6.4's B5 row is written as it is.
- The experiment is explicitly "a bounded authoring experiment" and "does not establish bridge operation, external finality, a Moriarty compiler judgment, or a ledger proof."

Limits I add, which RESULT.md does not enumerate:

- **Single leg, single claim, single attempt.** There is no attempt identity, no `external_ref` set, no cumulative-across-attempts accounting and no concurrency. The duplicated-dispatch hazard (H3) — arguably the most dangerous one, because it produces a *real* unauthorized effect rather than a refusal to act — is entirely outside the fragment.
- **Authoring time, not runtime.** It constrains what a program may *say*; it enforces nothing about what an adapter *does*. The gap between the two is the whole kernel.
- **No correspondence to the target.** Nothing links the Aeon fragment to Core, ZKIRv3, the Midnight verifier or PCD. Per the develop skill, "Tests and packet approvals cannot establish missing behavior."
- **It does not discriminate Design A from Design B.** Both can satisfy `delivered + refund ≤ sent`; A can be decorated until it does. The experiment shows the *predicate* is right, not that any particular state shape is. My preference for B in §5 rests on representability arguments and on the primary sources, and is an **inference** that the Aeon run neither supports nor undermines.
- **It establishes the necessity of an exclusivity certificate, not its feasibility.** The arithmetic tells us we need a final, authoritative `delivered`. Whether any given route can *produce* one — and at what cost, and under whose trust — is open decision (b), and it is the question on which the practical value of this entire protocol turns.

---

## 9. Recommendation

Adopt **Design B** as the normative leg model for the MIL/4 **kernel interaction profile** proposed in PROPOSAL.md §"Required changes", with these seven commitments:

1. **Four-level operation identity** (§3), with `leg_id` as the at-most-once substantive-effect key and `attempt_id` never reused. Write the attempt record durably **before** dispatch (S5).
2. **Adapter-declared idempotency class** (§4) as a mandatory, versioned field of every adapter action class, with the retry rule enforced by the kernel rather than by adapter convention. An adapter that will not declare a class is inadmissible.
3. **`indeterminate` as a first-class, non-terminal effect value**, distinct from `none`, which the kernel cannot exit without evidence. This is the single change that makes §7.2 safe.
4. **Refund and compensation gated** on either a positive non-occurrence certificate (S1's pattern) or a finality-qualified `complete` delivery count — never on a clock, a status code, an empty observation, or an operator's report below the signed evidence class.
5. **Evidence class as an ordered lattice**, carried on every observation, with a signed floor per leg and no downgrade path. Model the *mechanism* on S2, which puts the finality threshold in the message and dispatches to a different handler below and above it.
6. **Compensation is always a new authorized action** under separately scoped recovery authority (R1–R6); authority consumption is monotone; terminal outcomes tombstone.
7. **A small typed acceptance projection** for Midnight, distinct from the internal machine, with the homomorphism obligation of §5.4 discharged explicitly before any acceptance encoding is frozen.

**First slice.** Make this a U5 deliverable as PROPOSAL.md proposes, but order the adapters by hazard-per-unit-cost: start with a **same-domain queued call** (§6.3) against a simulated venue, because it exercises enqueue/execute separation, reordering, late execution and the cancel/fill race (H4) **with no bridge, no foreign signer and no attestation service**. Add one direct Midnight leg (P1) as the control. Only then add a bridge leg with a simulated foreign domain. The decisive negative cases are the nine in PROPOSAL.md §"First implementation slice" plus the five traces in §7.2 of this memo; each must name its rejecting judgment or its retained `indeterminate`.

**What I would refuse to ship.** A kernel in which `unknown` is a value that any operator action can clear; an adapter with an undeclared idempotency class; a refund path whose precondition is a deadline; and a receipt type with a Boolean `success` field.

---

## 10. Open decisions

| # | Decision | Why it cannot be settled here |
|---|---|---|
| (a) | **Which projection of Design B is verified in-circuit vs off-ledger**, and the proof that the projection is a homomorphism (no projected transition permits more than its preimage) | Depends on the pinned ZKIRv3/verifier/key tuple, which PROPOSAL.md and the consolidated design both leave open. Circuit cost per field is unknown to me |
| (b) | **Who issues the non-occurrence / exclusivity certificate per route**, and whether it is in-circuit verified, light-client verified, or a named quorum | This is the load-bearing trust choice. S1 gets it from a state-absence proof against a light client; S2 gets nothing equivalent and instead makes messages re-attestable forever. Moriarty must pick per route, and routes that can offer neither must be marked refund-unavailable *before* use |
| (c) | **Attempt-ID observability**: `attempt_id` is the reconciliation handle, but publishing it leaks retry behaviour, solver failover and timing | Directly trades against the consolidated design's "Explicit permitted disclosure". A commit-and-reveal scheme may work; I have not designed one |
| (d) | **Bounded attempt sets.** Design B's dispatch axis is a set, but Moriarty requires that "Every stage terminates within a checked bound" and that append-only structures use "explicit finite episodes or authenticated rollover preserving cumulative budgets, obligations, replay and terminal tombstones" | An unbounded retry history is inexpressible in a bounded stage. Needs either a per-leg attempt budget (and a defined behaviour at exhaustion — I suggest forced `quiesced` + `indeterminate`) or a rollover that preserves the invariant across episodes |
| (e) | **Cross-domain deadline and clock source.** S1 supports both `timeoutHeight` and `timeoutTimestamp`; S2 uses `expirationBlock`, and warns it is an L1 block number on ARB-stack chains | Deadlines are compared across domains with different clocks. A wrong comparison quiesces early, which is safe, or late, which is not |
| (f) | **Whether class-O adapters are admitted in v1 at all.** They can never be retried automatically and can only ever reach `indeterminate` | A defensible v1 answer is "no". That rules out some venues entirely, and the trade is the owner's |
| (g) | **Late fee reconciliation.** `feeExecuted` may arrive after the effect was accepted (S2) | If acceptance has already been taken against an estimated fee, a later actual fee that breaches the signed cap is an unauthorized effect. Needs either a two-phase fee acceptance or a mandatory conservative fee reservation |

Not in scope and deliberately deferred, per PROPOSAL.md: solver bonding, fee settlement, service eligibility. Those "can be designed after the safety contract is fixed."

---

## 11. Facts and inferences, separated

**Facts (primary sources, quoted or closely paraphrased above):** everything in §2.1 attributed to S1–S5; the `status --json` output in §0; the contents of the four supplied local files. Facts attributed to NEAR and Hyperliquid pages are **second-hand** — taken as characterized by the two supplied 2026-09-29 Moriarty reports and flagged as such in §2.2, not re-fetched by me.

**Inferences (mine, not established by any source):** the four-level identity scheme and the inclusion of `plan_commitment` in `leg_id` (§3); the five-class idempotency taxonomy and the retry rule, including the evidence-class floor for reconciliation reads (§4); Design B, the claim that it dominates Design A, and the acceptance-projection mitigation (§5); the assignment of evidence requirements in §6 beyond what each cited source states about its own protocol; the ordering of the U5 adapter sequence (§9); every limit added in §8.2 beyond those RESULT.md states. Where a source is silent, this memo says so rather than filling the gap with a plausible-sounding rule — S4 is the clearest case: the leading cross-chain intents standard defines no order identity and no idempotency semantics at all, which is a gap Moriarty must close itself and cannot inherit.

**Established by no one, including me:** that any of NEAR, Hyperliquid, IBC, CCTP, a threshold signer, a bridge or Midnight actually enforces a Moriarty predicate at runtime. Every claim in this memo about a foreign domain is a claim about that domain's *published description*, and, for §2.2, a claim about a Moriarty report's description of that description.

---

## 12. Provenance and limits

Retrieved 2026-09-29 by this session: S1 as raw markdown from `raw.githubusercontent.com/cosmos/ibc/main/…` (73,261 bytes) and quoted from the retrieved bytes; S4 as raw markdown from `raw.githubusercontent.com/ethereum/ERCs/master/ERCS/erc-7683.md` (26,400 bytes) — note that the parallel path under `ethereum/EIPS` returns 404, and the live text differs materially from older revisions of this ERC; S5 as PDF from `cidrdb.org` (880,896 bytes), text-extracted with `pdftotext`; S2 and S3 via page fetch, with S2's canonical URL confirmed by redirect (`/cctp/technical-guide` → `/cctp/references/technical-guide`; the path `/cctp/cctp-technical-guide` returns 404). Public research only; no private repository text was sent to any external service.

Local files read: `deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`, `deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`, `deliverables/aeon-kernel-experiments-2026-09-29/bridge/RESULT.md`, `docs/MORIARTY-CONSOLIDATED-DESIGN.md`, `AGENTS.md`, `plugins/moriarty-dev/skills/develop/SKILL.md`, and this study's `PROMPT-OPS.md`. **Not read:** `grok-ops.md`, `sol-ops.md`, and every `*-abi.md` / `*-trust.md` answer in this study.

Slot history: this is the `opus/ops` slot of `launch_external.py`, whose original run failed on a launcher invocation bug — `opus-ops.stderr.attempt1.log` contains `Error: Input must be provided either through stdin or as a prompt argument when using --print`, and `opus-ops.raw.attempt1.json` is 0 bytes. It was not a content refusal. This session re-ran the slot under `--permission-mode plan`; `Write` and file creation are disabled, so the memo is returned as response text for capture to `deliverables/kernel-api-architects-2026-09-29/opus-ops.md`.

Limits of this memo: no code was written, no adapter was built, no transaction was submitted, no Aeon run was repeated, and no claim is made that Moriarty, MIL/4 or the kernel implements any of the above. The design is advisory input to an open MIL/4 decision. **This is one architect's independent analysis. No consensus, agreement, majority or reviewer approval is claimed, and I have no knowledge of what the other eight architects concluded.**