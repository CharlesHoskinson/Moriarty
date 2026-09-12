# Moriarty DeFi Kernel SDK Interface

**Status:** specified only. No adapter in this document has been tested against deployed network behaviour, and no tariff in it has been benchmarked. Every numeric parameter is a proposed launch value, not a measured one. Admission of the language changes named in section 3 is a precondition for the feature tiers that depend on them.

**Supersedes nothing.** This document extends `2026-09-11-defi-kernel-multichain-design.md` with a concrete interface, a revised cost model, an operator compensation scheme, and a staged admission ladder. Where the two disagree, this document governs, and section 12 records each disagreement.

---

## 1. Scope

The SDK is the orchestration layer around a bounded language. It arranges work that happens before any Midnight transaction exists, carries authorized payloads to foreign networks, and brings foreign results back as imported evidence. It does not extend the language, does not add a fifth effect, and does not make Midnight observe anything it cannot verify.

The namespace is `moriarty-kernel-sdk/1`. One Midnight coordinator instance anchors a session. Foreign legs never advance the coordinator through `Step`; they advance a bounded pending record through `ImportFrom`.

Three statements bound everything below:

- Cross-chain atomicity is not offered. What is offered is a bounded sequence with explicit custody, deadlines, imports and recovery.
- A signature is not a result. A `SignatureCertificate` is never accepted where an `ImportCertificate` is required.
- Submission is not success. An outer transaction that succeeds while the financial operation inside it fails is a failure.

---

## 2. The interface model

A session has one anchored instance on Midnight and zero or more foreign legs.

```text
IntentTerms
  -> ResolvedOrder / Plan
  -> Quote
  -> proof-independent ClaimSpec manifest
  -> Intent
  -> intent signatures
  -> instantiated transition claims
  -> transition proof
```

Quote signatures and proof bytes never enter their own preimages.

The lifecycle is:

1. **Resolve.** `resolve(program, terms)` publishes the bounded relation, amounts, approved primitives, evidence policies and recovery requirements. It publishes no wallet secret and no unrestricted signing authority.
2. **Quote.** A solver returns a `Plan` with typed legs and a dependency order, and a `Quote` binding `H(terms)` and `H(plan)`, itemizing every charge, reserving maximum metered work, and carrying a separate DUST estimate.
3. **Sign.** The user signs a selected bounded plan: the outcome, route, caps, evidence policy and materialization rules. The display reports every distinct asset, the maximum gross debit, net credit goals, all charges, deadlines, recovery exposure, and the words "threshold-attested foreign results" wherever that is the evidence class.
4. **Admit.** A `Step` consumes the intent nonce once, reserves consideration and authorized fees, creates the bounded `PendingIntent`, and fixes quote allocations. Nonce reservation, fund reservation and head update occur atomically. Only after finalized acceptance may an operator request a foreign signature.
5. **Dispatch.** Per leg: materialize the exact foreign payload, verify it, request a native signature bound to the decoded network, assemble, broadcast. A dropped acknowledgement causes reconciliation of the submission record, never a second transfer.
6. **Import.** Per leg: collect evidence under the pinned policy, verify it, and prepare `ImportFrom` against the **current** head. Importing one leg does not require another leg's result and does not declare the intent settled. Serial imports keep Midnight predecessor fan-in at 1.
7. **Settle.** When all leg predicates hold, a final `Step` checks actual — not projected — gross debits, net credits, fees and liabilities.

Continuations use the pending id and leg or import nullifiers. They do not re-consume the intent nonce and do not require the original head to still be current.

### Flow conservation

For every exact asset, across all participants including counterparties and fee collectors:

```text
sum(all account deltas for that AssetId) = 0
```

No equality is asserted between assets on different chains. Conservation per asset is a local accounting check; it is not evidence of cross-chain atomicity, and this document never presents it as such.

Foreign network fees paid by a solver appear as their own flows and charges. They are never subtracted from the user's promised credits.

---

## 3. Language changes this interface depends on

The interface is written against the eight known gaps. Each feature tier in section 11 names the ones it requires.

| Gap | Requirement |
|---|---|
| L1 | A `Pending` state with bounded contents. |
| L2 | `permittedCalls` non-empty, authorizing an off-ledger action. This authorizes; it does not make Midnight execute or observe anything. Every resulting fact still enters by import. |
| L3 | Chain-qualified asset identity: `AssetId { domain, issuer, assetReference }`. |
| L4 | Fan-in above 1. **Stays at 1 until a higher value is admitted**; the import sequence above is designed to work at fan-in 1. |
| L5 | A signed nominal liability cap. |
| L6 | Foreign signing authority bound to a decoded payload. |
| L7 | A solver interface. |
| L8 | Composition across instances. |

An SDK cannot simulate any of these with local bookkeeping. Where a tier's gap is unadmitted, the call surface returns `E_LIFECYCLE_UNSUPPORTED` or `E_PROFILE_NOT_ADMITTED` and names the missing change.

### State binding

An outcome relation specifies permitted results. A nonce specifies replay control. Neither removes the head from the signed digest on its own. The multichain mode therefore defines its state binding explicitly: the admitting `Step` binds the exact head, reserves funds against concurrent intents, and every continuation binds the pending id instead of the head. Leaving this implicit is an authorization gap, and `E_STATE_BINDING_UNDEFINED` rejects any profile that tries.

---

## 4. Type surface

Identity and reference: `ChainRef`, `AccountRef`, `InstanceRef`, `HeadRef`, `AssetId`, `Amount`.

Authority and policy: `BoundsProfile`, `SignerPolicy`, `CallPermission`, `ForeignAuthorization`, `EvidencePolicy`, `NetworkBinding`.

Economics: `FeeCapability`, `FeeBudget`, `MeterProfile`, `CapacityReservation`, `SponsorGrant`, `ChargeLine`, `MeterReceipt`, `JobTicket`, `PriorityBid`, `DUSTFundingOffer`.

Execution: `Leg`, `Plan`, `Quote`, `IntentTerms`, `PendingIntent`, `ClaimSpec`, `TransitionCore`, `ProofRequest`, `TransactionBundle`, `SettlementRecord`, `EffectRecord`.

Foreign: `AdapterManifest`, `ForeignPayload`, `SignatureRequest`, `SignatureCertificate`, `ForeignFact`, `ImportCertificate`, `ForeignFeeBudget`.

Typed foreign arguments, one record per registered primitive, never a generic blob: `NearFunctionCall`, `NearDelegate`, `HyperOrder`, `TronContractCall`, `TronResourceAction`.

The budget record is:

```text
FeeBudget {
  applicationCap,
  serviceCap,
  foreignCapsByLeg,
  priorityCapsByLeg,
  recoveryReserve,
  grossDebitCapsByAsset
}
```

Every total is a vector by domain-qualified asset. One scalar that adds DUST, TRX and a stablecoin is not a total.

---

## 5. Call surface

Grouped by stage; each group's calls return `Verified<T>` or a typed refusal from section 10.

- **Profile and adapter**: `loadProfile`, `listAdapters`, `adapterManifest`, `registerPrimitive`.
- **Authority**: `createCapability`, `revokeCapability`, `deriveForeignAccount`, `grantSponsorship`, `revokeSponsorship`.
- **Pricing**: `quoteWork`, `reserveQuote`, `buyCapacity`, `estimateDUST`.
- **Planning**: `resolve`, `requestQuotes`, `verifyPlan`, `prepareIntent`, `createSigningRequest`, `signIntent`, `verifyIntent`.
- **Admission**: `prepareTransition`, `buildProofRequest`, `bundle`, `verifyBundle`, `submit`, `awaitFinality`.
- **Foreign**: `materializeForeign`, `verifyForeignPayload`, `createForeignSigningRequest`, `signForeign`, `assembleForeign`, `broadcastForeign`, `reconcileSubmission`.
- **Import**: `collectImport`, `verifyImport`, `applyImport`.
- **Settlement and exit**: `settle`, `recover`, `claimFees`, `closeInstance`.

---

## 6. Foreign authorization

Authorization requires both Moriarty permission and native authority. The `CallPermission` answers *what may this intent cause*; the `ForeignAuthorization` answers *which native account may authorize that payload, under which current permission*. Both must hold. Holding a native key never enlarges the signed relation.

### Derivation

For chain-derived keys the SDK computes an injective path:

```text
"moriarty/1/"
  + hex(H(canonical(ChainRef)))
  + "/"
  + hex(principalId)
  + "/"
  + decimal(accountIndex)
  + "/"
  + decimal(keyVersion)
```

A developer selects `accountIndex` and nothing else. Arbitrary derivation paths are not accepted. The signing service verifies the same policy independently.

### Network binding

A free-standing chain label is not a network binding. `NetworkBinding` is decoded from the actual signature preimage and comes in two admitted forms:

- `SignedDomain` — binds codec and decoded field values. Required where the venue signs a typed domain with its own discriminators.
- `SignedRecentBlock` — binds a referenced block that the adapter authenticates to the selected network, with the limits of that binding documented.

A payload whose network identity cannot be decoded from what is signed returns `MissingNetworkBinding`.

### Native permission is venue-specific

Native permission systems are coarser than Moriarty's. A weighted threshold over a contract-type bitmap is not a callee, selector, argument or debit restriction. Where the native system cannot express the required restriction, the adapter supplies its own checks and, where necessary, a restricted custody contract. Delegated-call validation that checks action count, deposit, receiver and method does not decrement an allowance; sponsorship therefore imposes its own signed budget rather than relying on the native path.

---

## 7. Import acceptance

`ImportFrom` is admitted only when all of the following hold. The relation enforces them; the SDK does not.

1. The intent names this evidence class for this leg.
2. The policy hash is permitted by deployment and matches the intent exactly.
3. The deployed key set and threshold match; signer indices are distinct.
4. The threshold signature verifies over the canonical import statement.
5. Source chain, payload hash, pending id, intent id and leg id all match.
6. The extractor and source schema are the pinned versions.
7. Finality and freshness satisfy the named policy.
8. The extracted account, recipients, amounts, fees and liabilities satisfy the leg predicate.
9. The transaction or event identity has not already discharged another obligation.
10. Partial execution is recorded as partial execution, never as success.
11. The current pending state still admits this import, or admits its signed late-evidence recovery path.

Two nullifiers enforce uniqueness:

```text
source event:  H(sourceChain, transactionId, eventOrReceiptId, extractionPolicyHash)
leg progress:  H(instance, intentId, legId, attempt)
```

The first prevents reusing one foreign event as new value. The second prevents duplicate progress.

---

## 8. Cost model

**Five separately accounted planes.** The earlier four-plane model was incomplete: foreign execution consumes resources independently of both Midnight settlement and orchestration.

| Plane | Prices | Unit | Payer → recipient | Binding |
|---|---|---|---|---|
| Midnight settlement | Ledger acceptance | DUST | Transaction funder → Midnight's resource mechanism | Outside the contract-call statement. An SDK funding offer is not circuit evidence. |
| Foreign execution | Gas, bandwidth, account creation, venue and bridge charges | Native atoms or venue fee asset | Named leg payer → foreign mechanism or venue | Signed `ForeignFeeBudget` and payload limits; actual expenditure is imported evidence. |
| Orchestration services | Payload bytes, registered jobs, inventory commitment, observation risk | Settlement-asset atoms | User, sponsor or operator → named providers | `Quote`, `JobTicket`, `SponsorGrant`, escrow, accepted receipts. |
| Application | Distribution or financial product service | Settlement-asset atoms | Principal → approved application | Revocable capability, signed intent, ordinary `Fee` effect, gross-debit caps. |
| Priority | Optional ordering preference | Venue-native amount or explicit rate | Opted-in payer → venue recipient or burn | Separate `PriorityBid`, never inferred from an application fee. |

Priority stays separately accounted even where a venue collects it together with another charge.

### Meters

Two dimensions, both of registered work, neither a claim about elapsed CPU time:

- **PU** — one byte of canonical encoded, authorized payload delivered. Counted once per designated delivery across envelope, selected quote, foreign payload and certificate bytes. Unsolicited quotes, redundant gossip and operator retries are excluded.
- **CU** — one registered work credit. A profile assigns fixed credits to a bounded job with an identifiable output.

Proposed tariff, in one exact six-decimal settlement asset, as launch parameters pending benchmarks:

| Job | CU |
|---|---|
| validate | 100 |
| simulate | 400 |
| foreign signature | 1,000 |
| attestor observation | 400 |
| fused proof | 2,000 |
| observation | 10 |
| submission | 100 |

Fallback rates: 1 micro-unit per PU, 100 micro-units per CU.

Recovery windows use a 24-hour replenishment, priced in independent Moriarty units rather than any foreign network's resource accounting.

### Capacity is bought, not staked

Refundable stake cannot fund recurring operating costs without another revenue source, and proportional-share resource models do not yield deterministic capacity from a fixed stake. Recurring capacity is therefore purchased: `reserveQuote` holds a priced allocation for a quote lifetime, `buyCapacity` buys a fixed reservation. Security collateral is kept entirely separate from capacity purchase.

### Exhaustion stops work

**When capacity and the signed monetary budget are exhausted, execution stops before any further work.** A priced fallback may be used only where the payer explicitly authorized that fallback in the signed budget. A sponsor shortfall never becomes user debt, and an operator fault never becomes an unsigned user charge. This reverses the earlier "fallback charges, never hard-stops" rule, which contradicted bounded spending.

---

## 9. Application fee and operator compensation

### Application fee

A proportional fee with an absolute ceiling, carried by a revocable `FeeCapability` and realized as an ordinary `Fee` effect inside the signed gross-debit caps:

```text
feeAtoms = floor(basis × rateTenthBps / 100,000)
```

Rate caps: **1% spot, 0.1% derivatives**. Revocation and cap changes apply to future admissions only; they are never applied retroactively to an already dispatched obligation.

Protocol share is **5%** of collected service revenue (3% maintenance, 2% reserve). Primitive authors may opt into a **10% royalty on that primitive's base compute revenue**, disclosed in the profile. Congestion penalties and retries are excluded from royalty basis.

### Roles

| Role | Work and risk | Payment | Misbehaviour |
|---|---|---|---|
| Solver | Prices outcomes, reserves inventory, fills foreign legs | Winning quote's fixed fee or disclosed spread; losing bids earn nothing | No completion fee without fulfilment; performance collateral pays agreed replacement costs |
| Attestor | Extracts foreign facts and signs the pinned observation policy | Fixed observation fee plus exposure premium, independent of which outcome is reported | Invalid response earns nothing; equivocation is objectively penalized; other false claims go to adjudication |
| Sponsor | Reserves fee funds or capacity | Promotional sponsor earns zero; commercial sponsor charges a disclosed financing fee | Unfunded grant rejected; committed funds cannot be withdrawn; no authority to enlarge user liabilities |
| Relayer | Submits already authorized payloads | Fixed submission fee plus bounded actual-cost reimbursement | Unauthorized or duplicate submissions unreimbursed; provable breach consumes service collateral |
| Primitive author | Supplies an immutable bounded artifact | Optional 10% base-compute royalty | Defective version loses future admission and royalty eligibility; no automatic slash for authorship |
| Prover | Generates the fused proof | Fixed profile-priced fee for a valid proof | Invalid proof earns zero; missed committed delivery pays replacement costs |
| Watcher | Scheduled observations and alerts | Availability fee, not a per-allegation bounty | Unsupported alert earns no bounty; fabricated evidence rejected |
| Foreign signer | Signs only the authorized network-bound payload | Fixed signature-job fee plus quoted custody premium | Invalid output unpaid; unauthorized signature triggers the bonded-authority policy |
| Coordinator | Reserves budgets, dispatches jobs, carries payloads | Validation and payload tariffs | Duplicate receipts cannot be claimed twice; overbilling fails verification |

Authorship is bonded differently from authority. Supplying an immutable artifact carries reputational risk; operating a mutable oracle or holding a signing key carries custody risk, and only the latter posts an authority bond. An author who also operates an oracle assumes the operator obligations.

### Collateral and adjudication

Proposed pilot configuration:

- Observation quorum **5-of-7**, independent of the solver and the application benefiting from the observation.
- Aggregate unresolved exposure capped at **100,000 settlement units** per pool.
- Per-operator bond at least `max(10,000, 2 × aggregate exposure / 5)` — 40,000 at the pilot ceiling. Collateral may not simultaneously back another pool.
- Bonds exit no earlier than 14 days after notice and 7 days after the last covered exposure closes; unresolved disputes extend the lock.
- Penalties compensate verified losses and replacement costs first; only a residual is disposed of otherwise. Penalties are not burned.

This is collateral arithmetic, not a safety proof. A dishonest quorum can agree on one false claim without equivocating. Adjudication of that case uses a separately identified, deployment-bound quorum whose verdict is disclosed as **another trusted import**. Where that trust is unacceptable, the route does not launch before certificate verification exists.

---

## 10. Refusal contract

Every refusal is typed, names the missing artifact, and never degrades into a best-effort attempt.

`E_SCHEMA_NONCANONICAL`, `E_PROFILE_NOT_ADMITTED`, `E_EFFECT_NOT_REGISTERED`, `E_ASSET_UNSUPPORTED`, `E_LIFECYCLE_UNSUPPORTED`, `E_PROOF_ROUTE_UNSUPPORTED`, `E_STATE_BINDING_UNDEFINED`, `E_HEAD_STALE`, `E_NONCE_INVALID`, `E_REPLAY_STATE_UNSAFE`, `E_AUTH_INVALID`, `E_SIGNATURE_INVALID`, `E_SIGNING_DOMAIN_INVALID`, `E_PAYLOAD_MISMATCH`, `E_APPROVAL_UNBOUNDED`, `E_PAYER_NOT_AUTHORIZED`, `E_SPONSOR_GRANT_INVALID`, `E_SPONSOR_UNFUNDED`, `E_BUDGET_EXHAUSTED`, `E_CAPACITY_NOT_RESERVED`, `E_RESERVATION_REQUIRED`, `E_QUOTE_INVALID`, `E_METER_RECEIPT_INVALID`, `E_RECEIPT_REUSED`, `E_FEE_CAPABILITY_INVALID`, `E_FEE_CAP_EXCEEDED`, `E_FEE_ACCRUAL_UNBACKED`, `E_FOREIGN_COST_UNBOUNDED`, `E_GROSS_CAP_EXCEEDED`, `E_NET_GOAL_UNMET`, `E_LIABILITY_CAP_EXCEEDED`, `E_AMOUNT_INVALID`, `E_CONSERVATION_UNPROVED`, `E_PLAN_OUTSIDE_INTENT`, `E_IMPORT_REQUIRED`, `E_EVIDENCE_CLASS_MISMATCH`, `E_EVIDENCE_NOT_FINAL`, `E_EVIDENCE_CONFLICT`, `E_FACT_MISMATCH`, `E_OBSERVATION_POLICY_MISMATCH`, `E_QUORUM_INVALID`, `E_DEADLINE_INFEASIBLE`, `E_RECOVERY_UNSAFE`, `E_REPLACEMENT_UNSAFE`, `E_CANCEL_NOT_PROVEN`, `E_TERMINAL_STATE_INVALID`, `E_EXIT_HAS_LIABILITIES`.

### Lifecycle states

Product states are distinguished and never collapsed: `Submitted`, `UnknownExecution`, `Imported`, `Succeeded`, `Refunded`, `Compensated`, `ClosedWithClaims`.

`Succeeded` requires the specific financial result, not a successful outer transaction. A withdrawal path that can emit a failure event and return without reverting is the canonical case: the outer call succeeded and the withdrawal did not.

### Failure handling

When one leg succeeds and another cannot be proven: keep the imported fact, do not mark net goals satisfied, stop new dispatch at the signed cutoff, run the signed bounded recovery after `recoverAfter`, release unused local escrow, pay specified prefunded compensation, and preserve residual obligations. Return `Compensated` or `ClosedWithClaims`. Never `Succeeded`.

A deadline is not proof of non-execution. Releasing a reservation on a timeout can enable double spending; holding it can strand funds. Because no timeout resolves that, routes with irreversible user-funded foreign execution are admitted only where the selected recovery policy has **prefunded compensation that survives quorum and operator failure**, or a demonstrated bounded refund path. Otherwise quote verification rejects the route.

### Ranked failure modes

Critical: false import; timeout recovery releasing funds while a signed foreign transaction remains executable; double use of one authorization before an anchored reservation; payload, asset or authority confusion; obligations escaping signed liability bounds.

High: a foreign leg succeeding while settlement or import cannot proceed; sponsor capacity disappearing or metering racing past reservations; "success" meaning submission rather than the financial result; revocation applied retroactively to dispatched obligations.

Medium: quote spam, repeated proving, retry amplification and free cancellation exhausting capacity.

---

## 11. Admission ladder

A value-moving multichain SDK is **not admissible under the current profile**. Rather than block the whole interface on that, admission is staged, and each tier ships only when its gates close.

**Tier 0 — local orchestration.** Metered orchestration services, application fees, capacity reservation, sponsorship, quoting and settlement on Midnight alone. No foreign leg, no import, no foreign signing. Requires no unadmitted language change beyond the fee capability. Shippable against a verification-enabled Preview acceptance.

**Tier 1 — foreign observation.** Read-only imports: position, balance and status facts with no release of Midnight value conditioned on them. Requires L1, L3 and a pinned observation policy with a published threat model. Gates: adapter extraction evidence, freshness and finality policy, quorum independence assumptions.

**Tier 2 — value-moving legs.** Foreign signing and imports that release value. Requires L1–L3 and L5–L7, and every gate below.

### Tier 2 gates

- [ ] Required language changes admitted for the advertised feature set; fan-in stays 1 until a higher value is admitted.
- [ ] Complete lifecycle semantics for reservation, signature issuance, dispatch, unknown execution, import, compensation, refund and termination.
- [ ] Every crash boundary tested, especially immediately before and after signing, broadcast, import and payout.
- [ ] Nonce reservation, fund reservation and head update atomic; duplicate solvers and duplicate submissions cannot duplicate authorization.
- [ ] Foreign receipt consumption prevents reuse across intents, legs, instances and recovery branches.
- [ ] Gross debit, net credit and nominal liability caps cover the whole lifecycle including retries, fees and compensation.
- [ ] No normal or recovery action introduces a fifth effect or presents a foreign fact as anchored.
- [ ] Each adapter has retained deployed-behaviour evidence for encoding, network binding, permissions, fee limits, finality and financial success.
- [ ] Revocation, legacy-format rejection and nonce cleanup have adversarial replay tests, with no bypass for records that fail to decode as the versioned format.
- [ ] Quorum policy defines participants, threshold, independence assumptions, freshness, extraction, disagreement handling and loss limits.
- [ ] Threshold parameters have an explicit threat model naming what they do **not** protect against, including a common faulty data source.
- [ ] Meter profiles publish byte bounds, work limits, attempt limits, tariffs, rounding, quote lifetime and recovery budget. Missing benchmarked values block admission.
- [ ] Sponsor grants cannot be concurrently overspent; sponsor failure never silently bills the user.
- [ ] Compensation funded before irreversible foreign authority is issued, with explicit asset, amount, beneficiary and late-execution treatment.
- [ ] Launch exposure numerically capped per intent, instance, asset and quorum, with automatic admission stops at those limits.
- [ ] Verification-enabled Midnight Preview financial acceptance demonstrated on the exact candidate, with independent review.

### Adapter admission status

| Adapter | Status | Blocking condition |
|---|---|---|
| Midnight (anchored) | Tier 0 candidate | Preview acceptance on the exact candidate |
| Hyperliquid exchange action | Tier 2 candidate | `SignedDomain` binding is decodable; needs deployed conformance tests and bridge-outcome extraction |
| Tron contract call | Blocked | Raw transaction carries reference-block fields but no chain-id; cross-network replay analysis not established |
| NEAR delegate action | Blocked | Delegate record carries no network identity or recent-block hash; requires a domain-checking receiving contract whose exact invocation is signed |

Until a gate closes, the affected call returns `Unavailable` and names the exact missing adapter, policy or benchmark digest.

---

## 12. Decisions

| # | Decision | Alternatives rejected |
|---|---|---|
| D1 | Five cost planes; foreign execution is its own plane | Four planes, folding foreign cost into orchestration — it hides a cost the user pays in a different asset |
| D2 | Two meters, PU and CU, both of registered work | One compute meter; a meter claiming elapsed CPU time, which cannot be proven off-ledger |
| D3 | Capacity is purchased, not staked | Refundable stake for recurring capacity — it has no revenue source and no deterministic yield |
| D4 | Budget exhaustion hard-stops before further work | "Fallback charges, never hard-stops" — it contradicts bounded spending and converts sponsor shortfall into user debt |
| D5 | Foreign legs advance a pending record via `ImportFrom`, never via `Step` | Treating a foreign result as an anchored transition; fan-in above 1 to parallelize imports |
| D6 | Outcomes are `Succeeded`, `Compensated` or `ClosedWithClaims` | A binary success/failure that reports submission as success |
| D7 | Staged admission ladder | Shipping the full multichain surface now; or shelving the whole SDK on the strength of the Tier 2 objection |
| D8 | User signs a selected bounded plan | Signing exact foreign bytes, impossible before native nonces exist; or a broad outcome with unconstrained routing |
| D9 | Shared orchestration records plus typed, pinned venue primitives | One generic `{chain, to, data, value}` action; or unrelated per-venue SDKs |
| D10 | Independent disclosed adjudication for a capped pilot, certificates before wider exposure | The attesting quorum adjudicating itself; or claiming quorum signatures are objective truth |
| D11 | Penalties compensate victims first, residual disposed of afterwards | Burning slashed collateral, which pays no victim |
| D12 | Network binding decoded from the signature preimage | A chain label or `chainId` metadata field, which binds nothing |

---

## 13. Deliberately not offered

- No `signHash`, `sendRawTransaction` or unrestricted `call`. These bypass decoded permission and exposure checks.
- No arbitrary remote execution inside the language.
- No cross-chain atomicity promise.
- No same-token alias across chains. Conversion or bridging is an explicit primitive with an inventory source and evidence.
- No automatically reusable partial-fill authority. A second economic fill requires new authorization.
- No cancellation that erases dispatched work. Epoch invalidation prevents future admissions; it does not retroactively revoke an issued foreign signature.
- No fee amount reconstructed from an ambiguous aggregate. An adapter that cannot distinguish total from component charges produces unavailable evidence.
- No sponsor-to-user fallback by surprise.
- No hardcoded resource prices, finality delays or auction latencies. All are versioned quote and policy inputs.
- No claim that priority buys finality or success.
- No unbounded nonce table or watcher.
- No release-time certificate route yet. Threshold imports are the first supported evidence class.
- No kernel guarantee for arbitrary foreign staking exits. The SDK exposes position and withdrawal conditions, not an unconditional liquid balance.

---

## 14. Open parameters

Unresolved and explicitly not settled by this document: benchmarked CU weights and prices; a commercially adequate attestor exposure premium; enforceable loss adjudication; whether reference-block validation satisfies the network-binding predicate on Tron; and whether any candidate quorum's operators are genuinely independent.

The remaining uncertainty is empirical rather than a missing interface field. No inspected source proves that a proposed adapter matches deployed behaviour, that a chosen threshold represents independent operators, or that the pending and import relations fit admitted resource bounds. These are the release gates in section 11, not open questions about the shape of the interface.
