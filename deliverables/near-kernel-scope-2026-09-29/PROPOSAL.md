# Scope the Moriarty DeFi Kernel as a cross-domain execution service

**Research date:** 2026-09-29. **Status:** design proposal, not an adopted MIL/4 profile or implementation result. The question is which work a DeFi kernel must perform beyond finding solvers, and which constraints must remain in the intent language and settlement verifier. Public NEAR documentation and source repositories were the comparison set. No live transaction or contract build was performed.

## Decision proposed

Define the **optional Federated DeFi Kernel** as a capability-constrained execution and reconciliation service. It accepts a canonical Moriarty authorization, obtains quotes and evidence, coordinates chain-specific signing, sends bridge and venue calls, tracks their actual effects, and advances or recovers the workflow. It may maintain operational state and optional threshold-controlled assets. It has no authority to change the signed financial relation, select a weaker evidence premise, erase committed effects, or declare a cross-chain workflow atomic.

Moriarty remains the language of valid transitions and signed authority. An application supplies financial programs for AMMs, loans, vaults, derivatives and the other reviewed categories. A domain adapter supplies exact payload construction, transport and observation for one venue or chain. Midnight and other domains settle only what their actual verifier and ledger enforce. The kernel is useful but is not a prerequisite for a valid direct Midnight program. This is a refinement of the current [consolidated design](../../docs/MORIARTY-CONSOLIDATED-DESIGN.md) and [roadmap](../../ROADMAP.md), which already place cross-chain routing, threshold custody, finality observation and recovery in the optional federation.

## What NEAR actually separates

| NEAR component | Source-grounded role | Moriarty lesson |
| --- | --- | --- |
| Verifier contract | Holds credited balances, verifies signed intents, executes matching internal balance changes atomically; users can submit directly or through a third party. [Verifier docs](https://docs.near-intents.org/integration/verifier-contract/introduction), [contract repository](https://github.com/near/intents) | Keep the accepted relation at settlement. A relay cannot replace it. Atomicity applies to the verifier transaction, not to deposit and external withdrawal. |
| Message Bus and market makers | WebSocket quote broadcast, signed competing quotes, selected quote submission; documentation says the protocol can operate without the bus. [Market maker docs](https://docs.near-intents.org/integration/market-makers/introduction) | Solver discovery and selection are optional services. They are not language semantics. |
| Hosted 1Click service | Coordinates deposits, market makers and delivery, with observable statuses including processing, incomplete deposit, refund and failure. Its archived documentation describes a temporarily trusted swapping agent. [1Click documentation snapshot](https://github.com/defuse-protocol/gitbook-docs/blob/main/integration/distribution-channels/1click-api.md) | A convenient API can introduce a different custody or operator trust boundary. Model it as an adapter/service profile with explicit status and fees. Do not inherit its assurances into the core language. |
| Token bridges | Current NEAR Intents documentation lists Omni, PoA and HOT bridge routes, and states that each has a distinct trust model. [Current bridge docs](https://docs.near-intents.org/integration/bridging/overview) | A `bridge` call must name its route, asset representation, finality rule, fee, claim identity and verification premise. A generic success Boolean is inadequate. |
| Chain Signatures and MPC | A NEAR account or contract requests a threshold signature over a supplied payload and derivation path; the caller then constructs and broadcasts a chain-specific transaction. The MPC repository also describes independent foreign-chain observation and threshold signatures over extracted results. [Chain Signatures docs](https://docs.near.org/chain-abstraction/chain-signatures), [MPC repository](https://github.com/near/mpc), [transaction builder](https://github.com/near/omni-transaction-rs) | Signing is neither authorization by itself nor proof of execution. Distinguish user intent signature, operator/custody signature, transaction broadcast and foreign-effect evidence. |
| Bridge refund path | The BTC refund guide requires a nonfinalized deposit, a timelock, a contract action, MPC signing and relayer broadcast. It charges a storage deposit that is not returned. [BTC refund guide](https://docs.near-intents.org/integration/bridging/btc-deposit-refund) | Recovery is an explicit, costed workflow. A timeout or signature request cannot establish completed refund. |

The separation above is a structural comparison. It does not imply that NEAR's Verifier, bridges and MPC are one operator or share one security guarantee. NEAR's Chain Signatures page describes outbound signing as one-way and directs readers to bridge mechanisms for external state; its MPC repository separately documents foreign-chain transaction verification. Those are distinct capabilities and must be specified separately in Moriarty.

## Kernel service boundary

| Capability | Kernel operation | Language/acceptance requirement | External trust or failure boundary |
| --- | --- | --- | --- |
| Intake and permission | Parse canonical envelope, check eligibility, distribute intent | Bound program/version/domain, exact signed bytes, grant scope, nonce, expiry and disclosure | Service refusal does not make a program invalid. |
| Solver market | Discover, quote, simulate, rank, reserve and schedule candidates | Candidate refines signed caps, net goals, recipients, fees and failure policy | Quote freshness, solver performance and service availability are operational promises. |
| Chain execution | Build, simulate, submit and retry exact adapter payloads | Signed `permittedCall` class, target/domain, parameter commitment, spend and fee caps, unique occurrence | Chain-specific nonce, gas, address and mempool rules; submission is not finality. |
| Multichain signing | Derive account, build payload, request threshold signature, assemble and broadcast transaction | User authorization and signing policy bind chain, derivation path, algorithm, payload digest, recipient, value, nonce and epoch | MPC quorum controls a key; a valid signature does not prove business validity or inclusion. |
| Bridge routing | Select registered bridge, lock/burn/mint/release, track claim and message, request withdrawal/refund | Domain-qualified assets, representation ratio, claim nullifier, cumulative entitlement, fee and authorized recovery | Bridge-specific validators, contracts, relayers, reorg/finality and liquidity; partial delivery remains possible. |
| Evidence | Observe receipts, finality and external predicates; obtain attestation/proof; correlate events | Evidence type, issuer/verifier, public inputs, freshness, finality, uniqueness and downgrade policy | Attestation says what its signer checked; a host field or RPC response alone is not a proof. |
| Continuation | Persist pending legs, reservations, retries, handoff and reconciliation | Committed prefix, residual duties, gross lifetime budgets, idempotency and exclusive terminal outcomes | Retry can duplicate effects if the domain lacks an idempotency key; unknown is a real state. |
| Recovery | Invoke authorized cancel, refund, compensation or manual escalation | Remedy preconditions, retained liabilities, late-result race, cost and terminal evidence | Compensation is a new action; no global rollback across chains. |
| Service governance | Maintain federation membership, key epochs, adapters, availability and audit log | Signed trust policy and stable program acceptance independent of membership | Threshold compromise, equivocation, upgrade and operator outage must be disclosed. |

The eight DeFi categories remain **application profiles**, not eight kernel subsystems. Their financial predicates differ, but all can call the same bounded execution services. Bridge settlement is both a category with financial invariants and a kernel capability that can serve other categories. An AMM swap may require a bridge leg; a loan liquidation may require a foreign signature and venue call; a vault withdrawal may require asynchronous evidence. The language must preserve each application's semantics while the kernel handles the common mechanics.

## A minimal service contract

The kernel should expose versioned records, not an untyped `call()` with a success flag:

```text
IntentEnvelope  = canonical signed program, policy, objective, caps, grants, nonce
Capability      = adapter ID/version, domain, action class, trust/evidence profile
Plan            = ordered or partially ordered legs, exact payload commitments,
                  reservations, deadlines, fees, expected evidence, recovery edges
SignRequest     = intent/leg IDs, domain, key policy and epoch, derivation path,
                  scheme, canonical payload digest, validity window
BridgeRequest   = route/claim/message IDs, source and destination assets,
                  amount/ratio, recipient, fee, finality and refund policy
OperationReceipt= intent/leg/attempt IDs, payload hash, submitted tx hash,
                  phase, observed effects, evidence reference, fee, time/finality
Continuation    = committed prefix, pending/unknown legs, consumed authority,
                  cumulative exposure, remaining entitlement and duties
```

Proposed API phases: `quote → validatePlan → reserve → authorize → dispatch → observe → reconcile → advanceOrRecover`. The exact call schema and accepted-state encoding are **open MIL/4 design work**, not currently implemented. `validatePlan` must run before a signature request or external submission. Every dispatch gets a stable logical leg ID and distinct attempt IDs. A retry reuses the logical effect identity and first reconciles all known attempts. The service must return a typed `unknown` when it cannot establish execution or nonexecution.

The signing service must have no generic “sign arbitrary bytes” authority from an intent. A user signs the Moriarty envelope. A separately scoped signer capability may sign only a payload produced by a pinned adapter, checked against the authorized leg, chain, recipient, value, nonce, derivation path and key epoch. For a destination chain whose contract accepts only a threshold signature, the policy must explicitly state that quorum compromise can bypass the Moriarty check. A cryptographic signature proves key participation in signing specific bytes; it does not prove the external transaction executed or the quoted outcome was delivered.

## State and evidence discipline

Use distinct states for `planned`, `reserved`, `signed`, `submitted`, `included`, `finalized`, `delivered`, `refunding`, `refunded`, `failed` and `unknown`. These are suggested kernel phases; acceptance must use a smaller typed subset only after its exact meaning is specified. For each leg, retain the submitted bytes or commitment, domain transaction identity, observed complete effects, bridge message/claim/nullifier, fee charged, evidence provenance and finality policy. A bridge deposit credited inside a verifier is a different fact from a source-chain transfer; a verifier balance change is a different fact from destination withdrawal.

For each accepted MIL stage, the stage/intent/effect/authority/history/failure judgments proposed in [MIL/4](../mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md) must bind the **actual** effect and evidence set. An external observation enters as an import with an explicit verifier premise. A threshold attestation, light-client proof and trusted operator report are different evidence classes. Do not silently fall back among them. The profile must state whether the destination is proof-enforcing or merely signature-enforcing. Where nonreceipt cannot be authenticated, time alone cannot authorize a refund that conflicts with a late delivery.

## Required changes to the MIL/4 proposal

1. Add a **kernel interaction profile** outside the eight financial profiles. It defines typed external calls, asynchronous legs, stable logical IDs, attempt IDs, evidence imports and retained continuation. It does not make a kernel mandatory for direct Midnight execution.
2. Extend the signed authority record to include domain-qualified asset IDs, permitted action classes, adapter/version and target, recipient and payload constraints, gross fee/spend caps, signature/key policy, evidence/finality profile, and recovery authority. The final byte encoding and verifier locus need their own M4-C5 decision.
3. Add a **bridge-specific profile** over the common interaction contract: claim/message/nullifier, representation ratio, cumulative destination delivery, remaining entitlement, source refund exclusion, and separate bridge trust policy. The existing `bridge-pair/1` row remains specified-only until a qualified foreign verifier exists.
4. Specify a **signing profile** independent of bridging. It must test cross-domain replay, path reuse, payload substitution, wrong chain ID, key epoch change, valid signature without broadcast, and broadcast without finality. NEAR's chain-specific transaction builder illustrates why the payload cannot be treated as generic bytes without a pinned codec.
5. Make the **kernel protocol a U5 deliverable** with exact-byte adapters, concurrency reservations, retries/failover, signer and bridge service interfaces, receipts, and two independent solvers. U0–U3 language/native gates stay independent. Audit both the semantic candidate and the service protocol, since a passing language proof cannot establish adapter correctness or federation availability.

## First implementation slice and discriminators

The first service slice should use a direct Midnight leg and one exact-byte external adapter with a simulated foreign domain. Demonstrate two independent solver candidates under one signed outcome envelope, one authorized signature request, one bridge-like pending message, an actual imported result and a retained recovery path. This is an **acceptance target for U5**, not a claim that the current S0 or MIL/4 implementation supports it.

The decisive negative cases are: a solver widens the recipient or fee; a signer changes chain/path/payload; a bridge uses the wrong representation or claim ID; the same logical leg is submitted twice; a timeout is treated as nonreceipt; a partial destination delivery is refunded in full; an operator claims `SUCCESS` from a signature or source transaction alone; an adapter changes after authorization; a key epoch changes with pending duties; a service outage suppresses a direct Midnight valid path. Each case should identify the rejecting judgment or the retained `unknown`/recovery state.

## Open decisions and limits

The owner still must select the initial custody model, who controls bridge and signing keys, whether a foreign fact is verified by an in-circuit proof or a named quorum, and the first external adapter. Those choices change trust assumptions and cannot be inferred from NEAR. Solver bonding, fee settlement and operational service eligibility can be designed after the safety contract is fixed. NEAR documentation establishes useful separations and examples; it does not establish Moriarty's native proof, external verifier or production safety.

## Research provenance

Scrapling 0.4.15 retrieved public pages on 2026-09-29. The [source ledger](SOURCES.md) records URLs, captures, hashes and limits. Two stale documentation URLs returned 404 and empty captures; those were discarded. The older 1Click GitHub documentation is an **archived** source and used only to describe that hosted interface's published flow. Previous Moriarty [NEAR teardown](../near-teardown-2026-09-19/RESULT.md) and [multichain kernel study](../defi-kernel-multichain-2026-09-11/REPORT.md) informed the questions, but current claims above use the linked primary sources. Public research only; no private repository text was sent to external services.
