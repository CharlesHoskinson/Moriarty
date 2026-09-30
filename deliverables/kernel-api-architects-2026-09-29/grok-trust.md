# Authority, trust, and developer experience for the Moriarty DeFi Kernel

**Status:** advisory memo from one architect. **Date:** 2026-09-29. **Scope:** authority, trust, and the client developers actually call. This memo adopts no MIL/4 profile, claims no implementation, and records no agreement from any other architect.

This consultation uses the supplied local packet as its source of record. It does not re-run repository status, does not edit product code, and does not treat a new web retrieval as part of its evidence. NEAR, Hyperliquid, and Aeon claims below are facts from that packet unless marked as inference or recommendation. Five further standards are cited from prior knowledge of the published documents, at the level of detail those documents stably support. Macaroons are identified bibliographically and are not given a URL this consultation did not re-check.

---

## Recommendation

Make the optional Federated DeFi Kernel a **caveated-capability client** of a permissionless intent language. The user signature over canonical envelope bytes is the root grant. Every solver, sponsor, signer, and adapter receives a strictly narrower child grant. The kernel may refuse a grant exercise. Refusal leaves the program's validity unchanged, and the reference SDK keeps a direct settlement-submission client beside the kernel client.

The signer capability covers one payload: the bytes of a pinned adapter codec, after the grant's domain, derivation path, digest, recipient, value, fee interval, logical occurrence, and key epoch have been checked. A threshold signature is its own receipt. Submission, inclusion, finality, and financial acceptance are later receipts. `unknown` is a retained result with a reason and a permitted next action.

I recommend this over class-ambient agent keys. I recommend a signed fee interval, non-downgradable evidence, adapter pins of `(id, version, codec hash)`, and durable logical-occurrence consumption whose locus is written on the grant. Custody, key control, the foreign-fact verifier, and the first adapter stay owner decisions. The packet already lists those as open, and this memo does not close them.

---

## How to read claims

| Mark | Meaning in this memo |
| --- | --- |
| Source fact | Stated by a packet document, or a stable property of a named standard |
| Inference | Conclusion from those facts |
| Recommendation | This architect's proposed rule |
| Open | Owner decision, or a gap the packet leaves open |

---

## What the packet already fixes

Source fact. The NEAR comparison proposes an optional kernel that parses a canonical authorization, obtains quotes and evidence, coordinates chain-specific signing, submits bridge and venue calls, tracks effects, and advances or recovers a workflow. The same proposal states that the kernel has no authority to change the signed financial relation, select a weaker evidence premise, erase a committed effect, or declare a cross-chain workflow atomic. A service refusal does not make a program invalid. Direct Midnight execution remains a valid path when the kernel is down. Source: `deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`.

Source fact. That proposal treats the following as distinct: the user intent signature, an operator or custody signature, transaction broadcast, and foreign-effect evidence. Its suggested phases are `planned`, `reserved`, `signed`, `submitted`, `included`, `finalized`, `delivered`, `refunding`, `refunded`, `failed`, and `unknown`. Its suggested records are `IntentEnvelope`, `Capability`, `Plan`, `SignRequest`, `BridgeRequest`, `OperationReceipt`, and `Continuation`. The call sequence it proposes is `quote → validatePlan → reserve → authorize → dispatch → observe → reconcile → advanceOrRecover`. Exact bytes are open MIL/4 work.

Source fact. The Hyperliquid comparison separates a venue's native financial kernel from the Moriarty coordination kernel, and both from the intent language. It asks each leg to name a settlement locus. It treats builder-fee authority as a revocable user maximum, agent-wallet nonces as an insufficient consumption record, and an oracle publication as an issuer-and-round input rather than an objective fact. Source: `deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`.

Inference. Authority for this kernel is a set of separable grants over those records. Trust is the disclosed premise on each receipt. Developer experience is a client whose types follow those separations and whose convenience helpers preserve `unknown`.

The proposal describes itself as a refinement of `docs/MORIARTY-CONSOLIDATED-DESIGN.md` and states that the current design already places cross-chain routing, threshold custody, finality observation, and recovery in the optional federation. This consultation does not restate that document.

---

## What the Aeon signing experiment establishes

Source fact. The signing probe ran Aeon 4.9.0 from `alcides/aeon` commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`, with `strict_decidable=True`, synthesis budget 0, and no entry point. Identifiers and a digest were `Int` tags. `authorize` accepted only domain `1`, path `7`, digest `314159`, recipient `42`, value in `1..100`, epoch `5`, and nonce `9`, and it returned an encoded `signed` phase. Phase tags `1`, `2`, and `3` stood for signed, broadcast, and finalized. They were abstract tags. No signature, transaction, receipt, or proof was serialized. Source: `deliverables/aeon-kernel-experiments-2026-09-29/signing/RESULT.md`. Result file and probe: that directory's `results.json` and `probe.py`.

| Observation | Result on this path |
| --- | --- |
| Exact request, and a caller-supplied amount already bounded in range | Accepted |
| Wrong domain, path, digest, recipient, epoch, or nonce | Rejected, `LiquidTypeCheckingFailedRelation` |
| Amount `101`, amount `0`, unconstrained dynamic amount | Rejected, same error class |
| Signed value used as broadcast, signed used as finalized, broadcast used as finalized | Rejected, same error class |
| A literal token constructed with the finalized tag | Accepted |
| `native "1"` declared as a finalized token | Accepted, and the trust report names `observed` as `native` |

Source fact. The probe did not test canonical serialization, hash collision resistance, MPC threshold behavior, signer equivocation, cross-chain nonce semantics, replay protection, broadcast, foreign-chain finality, or the Moriarty compiler and ledger. No transaction was submitted. The report's own consequence statement is that a user-signed envelope and a pinned adapter codec remain the authority for each `SignRequest`, and that a signature must not drive a language transition that requires finality.

Inference, bounded. On this encoding, this pinned checker can reject the six field substitutions and the three phase confusions that were written down. A tag equality can be satisfied by writing the tag. A `native` premise satisfies the checker because the checker was told to trust it. Integer-tag acceptance does not show that adapter bytes match the tags.

### Trust limit from the same pinned compiler

Source fact. The sibling authority probe, same commit and driver settings, accepted `minimal_breach.ae` (`v = 101 && v <= 100`) and `minimal_false.ae` (`false`) with exit code 0. A split gross-cap obligation rejected a gross of 101. A combined conjunct `g = principal + fee && g <= maxGross` accepted gross 101 against cap 100. A `native` annotation was reported as a trust premise. Fresh-process CLI runs reproduced the impossible-refinement acceptances. The report says this path cannot be a sound admission or settlement checker, and that the internal compiler cause was not established. Source: `deliverables/aeon-kernel-experiments-2026-09-29/authority/REPORT.md`.

Inference. The signing probe's rejections are observations of one encoding on one path. They are not a soundness proof for Aeon 4.9.0 at this commit. A green Aeon result cannot authorize a signature request, a bridge call, a refund, or final settlement.

Source fact. The bridge probe on the same compiler rejected a full source refund justified only by timeout, by status `unknown`, by `observed_delivery = 0`, or by a positive partial delivery, with counterexamples in which `delivered = 1` while `sent = 1`. Refund of `sent - delivered` was accepted only under the premise that `delivered` is the complete authoritative count. Source: `deliverables/aeon-kernel-experiments-2026-09-29/bridge/RESULT.md`.

Inference. That arithmetic is useful once a verifier has supplied the premise. The probe does not supply the premise.

Recommendation. An authoring linter may run this Aeon pin only with the trust report visible and with no gate on sign, submit, or accept. The reference SDK has no status "Aeon: proven." Whether a later Aeon commit repairs the impossible-refinement acceptance is the open question already stated in the authority report.

---

## Two authority models

### Model I — Caveated capabilities

Recommendation, and the model the rest of this memo specifies.

The root is the user's signature over the canonical `IntentEnvelope` bytes, including caps, evidence profile, key policy, adapter pin, and recovery authority. A delegate produces a child grant by adding constraints. Verification recomputes the chain from the root signature and checks that each child is a subset of its parent. Exercise of a child consumes a logical occurrence. Revocation is a signed monotonic record. If the revocation record is missing or stale, a new signature is not produced.

This follows the attenuation rule published as Macaroons: a holder can add caveats and cannot remove them; verification uses the root and any third-party discharges (Birgisson, Politz, Erlingsson, Taly, Vrable, and Lentczner, "Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud," NDSS, 2014). The Moriarty instance is domain-specific: caveats are the financial fields in the kernel proposal, and the root is an intent signature rather than a bearer cookie.

Source fact, used as a limit on how far a signature grant reaches. EIP-2612 `permit` binds owner, spender, value, deadline, and a nonce under an EIP-712 domain. It authorizes an allowance. It does not bind a call payload, a bridge claim, or an evidence issuer. URL: [EIP-2612](https://eips.ethereum.org/EIPS/eip-2612). Inference. A permit-shaped cap is necessary for spend and fee limits and insufficient for `SignRequest` authority. Model I adds the payload digest, adapter codec hash, settlement locus, and evidence profile to the signed object.

Failure mode of Model I. A root grant written as "any adapter, any recipient, any evidence issuer, fee cap at the asset balance" restores ambient authority inside a capability costume. The SDK default templates are the practical control on that failure.

### Model II — Class-ambient agent keys

Source fact. Hyperliquid API wallets sign for a master account. Nonce state is per signer. The documentation warns that pruning a deregistered agent's nonce state can make old actions replayable if the address is reused. Source: [Nonces and API wallets](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets), as reported in the Hyperliquid comparison.

Source fact. The archived NEAR 1Click documentation describes a temporarily trusted swapping agent coordinating deposits, market makers, and delivery. The kernel proposal treats that hosted API as a different custody boundary, and it says the protocol can operate without the quote bus. Sources: [1Click snapshot](https://github.com/defuse-protocol/gitbook-docs/blob/main/integration/distribution-channels/1click-api.md), [market maker introduction](https://docs.near-intents.org/integration/market-makers/introduction).

Under Model II the master approves an agent key. Until deregistration, the agent signs the action class. Recipient, fee, and payload constraints live in operator convention or a second check, not in the key's authority.

Inference. Model II has the developer experience exchange integrators already know: one key, many orders, low latency. Its trust failure is the one Hyperliquid documents, plus payload substitution inside the class. A Moriarty kernel that held Model II keys would be an authority, not a coordinator. That conflicts with the proposal's rule that the kernel cannot change the signed relation.

Recommendation. Reject Model II as the default signer for this kernel. A short-lived session may still exist as a Model I grant whose caveats are already tight: one adapter pin, one domain, one recipient set, a fee interval, an epoch, and an expiry. The developer may call that object a session. The verifier sees caveats.

### Settlement re-check, required by both models

Source fact. The NEAR verifier holds credited balances, verifies signed intents, and executes matching internal balance changes atomically. Users can submit directly. Atomicity is the verifier transaction. Source: [verifier introduction](https://docs.near-intents.org/integration/verifier-contract/introduction), [near/intents](https://github.com/near/intents).

Source fact. EIP-4337 separates validation of a `UserOperation` by an EntryPoint, including signature and paymaster validation, from later execution. A paymaster sponsors gas. The atomic unit is one Ethereum transaction. URL: [EIP-4337](https://eips.ethereum.org/EIPS/eip-4337).

Inference. A kernel-only check is an operational filter. The settlement locus named on the leg re-checks whatever that locus can check. When the destination accepts a threshold signature and cannot evaluate Moriarty caveats, the grant states that quorum compromise can bypass the Moriarty check. The proposal already requires that disclosure for such destinations. Model I does not remove it.

### Rejected pattern — operator ACL

Recommendation. The protocol has no operator bit that widens a grant, replaces an evidence issuer, or marks a program invalid. An allow-list inside the kernel would make refusal and invalidity look alike to the developer, and it would make the direct path a second policy.

---

## Scoped grants

Recommendation. A grant is a typed record, versioned as a draft until MIL/4 fixes bytes.

```text
Grant
  grant_id, parent_grant_id          # root parent is null
  holder                             # user, solver, sponsor, signer quorum, kernel role
  action_class                       # quote, sign, venue_call, bridge, observe, sponsor_fee, recover
  domain                             # CAIP-2 chain id, see below
  settlement_locus                   # midnight_verifier | foreign_venue_ledger
                                     # | same_chain_system | bridge_destination
  adapter_id, adapter_version, codec_hash
  target, recipient, asset_id
  principal_cap, fee_min, fee_max, gross_cap
  payload_digest                     # absent only before a payload exists
  key_policy                         # scheme, epoch, path, quorum, bypass_disclosed
  evidence_profile                   # issuer set, verifier, finality, freshness
  logical_occurrence, not_before, expiry
  recovery                           # remedy class and the evidence it requires
  revocation_index_seen
```

Source fact. CAIP-2 identifies a chain as `namespace:reference`, for example `eip155:1`. URL: [CAIP-2](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-2.md). Recommendation. Domain-qualified asset and account identifiers use CAIP-2 for the chain and a profile-defined asset grammar. The packet requires domain-qualified asset IDs and does not name CAIP. Adopting the grammar is an encoding proposal for the open M4-C5 byte decision, not a claim that MIL/4 has adopted it.

Recommendation. The verifier evaluates three separate obligations on every value-moving grant and again on actual effects:

1. `fee_min ≤ fee ≤ fee_max`
2. `gross ≤ gross_cap`
3. `gross = principal + fee`, with the profile's stated rounding rule

The authority experiment is the reason they are separate obligations in every checker we run, including a future repaired refinement checker. One conjunct that embeds the sum and the cap is an unsafe encoding on the pinned Aeon path, even when the mathematics is the predicate we want.

Recommendation. `validatePlan` runs before any signature request or external submission, as the proposal already sequences it. A plan leg that widens recipient, domain, fee, adapter version, or evidence issuer relative to the grant is a refusal. The program remains valid. The solver may return a narrower plan.

Source fact. EIP-712 binds a signature to a domain separator that includes `chainId` and `verifyingContract`, so a change of domain changes the digest. URL: [EIP-712](https://eips.ethereum.org/EIPS/eip-712). Inference. EIP-712 is the right shape for EVM adapters and the wrong assumption for every chain. The Moriarty envelope has its own domain tag. Each adapter codec defines how that tag appears inside the foreign payload. Both layers are checked. A digest match at only one layer is an incomplete grant check.

---

## Multichain signature request authority

Source fact. NEAR Chain Signatures lets an account or contract request a threshold signature over a supplied payload and derivation path. The caller then builds and broadcasts the chain-specific transaction. The NEAR page describes outbound signing as one-way and points to bridges for external state. The MPC repository separately documents foreign-chain observation and threshold signatures over extracted results. Sources: [Chain Signatures](https://docs.near.org/chain-abstraction/chain-signatures), [near/mpc](https://github.com/near/mpc), [omni-transaction-rs](https://github.com/near/omni-transaction-rs).

Source fact. The kernel proposal says the signing service has no generic "sign arbitrary bytes" authority from an intent. A cryptographic signature proves key participation over specific bytes. It does not prove broadcast, inclusion, or the quoted outcome.

Recommendation. The signer service produces a signature only after all of the following hold:

1. The envelope signature verifies under the user key named in the grant.
2. The child grant is a subset of its parent, and the revocation index is fresh and does not revoke this grant, holder, adapter version, or epoch.
3. The registered adapter `(id, version, codec_hash)` matches the grant pin.
4. The pinned codec decodes the payload, and the recomputed digest equals `SignRequest.payload_digest`.
5. Decoded domain, derivation path, scheme, recipient, value, logical occurrence, and key epoch equal the grant. The chain identifier inside the payload equals the grant domain.
6. The three fee and gross obligations hold, and the sponsor attestation, if any, covers this same digest and this fee.
7. This logical occurrence is unconsumed in the durable consumption record.

A failure at steps 1–7 is `refused`. No signature is requested from the quorum. A failure of the quorum to answer after step 7 has passed is `unknown`: the quorum may have signed. The client reconciles that occurrence before it asks again.

`SignRequest` fields are the proposal's fields, made mandatory for this profile: `intent_id`, `leg_id`, `attempt_id`, `grant_id`, domain, key policy and epoch, derivation path, scheme, canonical payload digest, recipient, value, logical occurrence, and validity window.

Receipts that remain distinct, in this order: user envelope signature, quorum signature, submission, inclusion, finality evidence, financial acceptance. The Aeon probe's phase-tag rejections are an authoring illustration of this separation. The probe's acceptance of a hand-written finalized tag is the illustration of why a phase tag is not finality.

Source fact. The destination that accepts only the threshold signature can be moved by a quorum that skips the seven checks. The proposal requires the policy to state that. Recommendation. The grant field `quorum_bypass_disclosed` is true only when the user signature covered that sentence. The SDK renders the label `quorum_may_bypass_moriarty` on every subsequent receipt for that leg.

Replay locus is part of the same authority decision. Recommendation.

- `replay_locus_settlement`: the settlement verifier enforces the nullifier. The kernel's cache is a replica used to avoid a doomed signature request.
- `replay_locus_signer_log`: the destination cannot evaluate the Moriarty nullifier. Consumption is the signer set's durable log. The grant shows this label before authorization.

Source fact. Hyperliquid's per-signer nonce store is not a durable logical-request record, and pruning plus address reuse is a documented replay path. Recommendation. Retired signing keys are not reused for a new epoch. Consumption history for an occurrence outlives the key that signed it, for the life of the intent plus a retention window written in the trust policy. Pruning that history is a policy change, and it is itself a signed grant the user would have to accept. The kernel does not prune it as maintenance.

---

## Delegation and revocation

Recommendation. Delegation is attenuation.

| Parent field | Child may |
| --- | --- |
| Fee maximum, gross cap, principal cap | Lower or keep |
| Expiry | Shorten |
| Recipient, domain, adapter version, codec hash | Keep the same value |
| Evidence issuer set | Keep the same set or remove issuers |
| Action class | Drop classes |
| Key epoch and path | Keep, unless the parent already names a successor epoch |
| Recovery authority | Keep or drop remedies |

A child that adds an issuer, raises a cap, changes a recipient, or moves to "latest adapter" fails subset verification. The kernel refuses the child. The parent grant stays as it was.

Revocation is a signed record: `revocation_index`, `subject` (grant, holder, adapter version, or epoch), and the revoker's key. Indexes only increase. `authorize` and `sign` require a revocation proof no older than the freshness window in the trust policy. A failed fetch of that proof yields `unknown` with reason `revocation_status_stale`, or `refused` before any quorum call. It does not yield "authorized."

`previewQuote` may display a stale revocation label. Quote is not exercise. This split is the developer-facing form of the strict rule: exploration stays available, and signing stays blocked.

In-flight foreign signatures have a gap the SDK states in words: a signature produced while the grant was live can still be included by a chain that never sees the Moriarty revocation. That inclusion is a committed effect. Recovery is a new authorized action under the recovery grant. The NEAR verifier's atomicity does not extend across that chain, and EIP-4337's atomicity does not either. Both are one-ledger facts.

Source fact. Hyperliquid builder-code approval is revocable, and the fee is processed on-chain within the approved maximum. Source: [Builder codes](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes). Inference. Revocation of a fee approval stops later orders. It does not rewrite an order that already cleared. The same shape applies to Moriarty fee grants and to key epochs.

---

## Evidence issuer and verifier policy

Source fact. RFC 9334 (RATS) separates the Attester, who produces Evidence; the Verifier, who appraises Evidence under an Appraisal Policy using Endorsements and produces Attestation Results; and the Relying Party, who uses those results. URL: [RFC 9334](https://www.rfc-editor.org/rfc/rfc9334).

Recommendation. Map the roles as follows.

| RATS role | Moriarty holder |
| --- | --- |
| Attester | The named issuer: quorum, light-client prover, oracle publisher, or operator |
| Verifier | The named verifier in the evidence profile, running the signed appraisal policy |
| Relying Party | The intent-language acceptance judgment at the settlement locus |
| Collector | The kernel, which may fetch and attach evidence and may not appraise it into a stronger class |

The kernel is a collector. For a value-moving transition it is not the relying party, and it is not the sole attester.

Source fact. The proposal distinguishes threshold attestation, light-client proof, and trusted operator report, and it forbids silent fallback among them. A host field or an RPC response alone is not a proof. The Hyperliquid comparison states that validator oracle inputs used for funding, margin, and liquidation need an issuer and a selected round, and that a bond or a slash is not proof of a correct price. Sources: [Oracle](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/oracle), [HIP-3](https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-improvement-proposals-hips/hip-3-builder-deployed-perpetuals).

Recommendation. Each evidence import carries `evidence_type`, `issuer_id`, `issuer_epoch`, `verifier_id`, `policy_hash`, public inputs, subject (`leg_id`, claim id, transaction, oracle round), freshness, finality rule, and uniqueness. The appraisal policy is inside the user-signed envelope.

An empty issuer set means the leg has no evidence on which a dependent transition can fire. The kernel does not fill the empty set with its own operator key. Adding an issuer is a new user signature. Operator reports may be shown in the client under the label `evidence_operator_report`. They become an acceptance premise only when the user-signed profile names that operator as an issuer for that transition.

`unknown` stays available. Time, an empty observation, and status code `unknown` do not establish non-delivery. The bridge experiment's counterexamples are the arithmetic reason. The BTC refund guide is the operational reason: a refund path there requires a nonfinalized deposit, a timelock, a contract action, MPC signing, and relayer broadcast, and it charges a storage deposit that is not returned. A timeout is not that path. Source: [BTC deposit refund](https://docs.near-intents.org/integration/bridging/btc-deposit-refund).

Open, and left open. Whether a given foreign fact is an in-circuit proof or a named quorum is the owner's verifier-locus decision. The grant records which one the user signed. This memo requires the record and the non-downgrade rule under either choice.

---

## Fee sponsorship

Three different payments appear on a leg. Mixing them is an authority bug.

| Payment | Who authorizes it | What the payer may change |
| --- | --- | --- |
| Application or builder fee | User grant interval `[fee_min, fee_max]` | Nothing structural; the actual fee is an effect |
| Venue or protocol fee | Same interval, disclosed in the plan | Nothing structural |
| Gas or relayer sponsorship | A `sponsor_fee` child grant bound to the payload digest and a sponsorship maximum | Whether to pay, within that maximum |

Source fact. Hyperliquid's user approves a per-builder maximum; a later order may carry a builder fee within that cap; the approval is revocable; the on-chain fee enters the trade. Source fact. EIP-4337 paymasters sponsor gas after EntryPoint validation and do not, by that role alone, become the sender of the user's call. The paymaster remains a distinct principal with its own validation.

Recommendation. The sponsor signs a `SponsorshipAttestation` over `(payload_digest, fee, leg_id, grant_id)`. The sponsor does not co-sign a mutable intent body. A sponsor refusal before dispatch is `refused`, consumes nothing, and leaves the direct path usable, including a user-paid fallback if the grant named one. A sponsor disappearance after the domain has accepted a submission leaves the leg at `submitted` or `unknown`. It does not convert the leg to `failed`, and it does not authorize a refund by itself.

Recommendation. The user signs a closed interval. The quote's expected fee is displayed and stored on the plan. An actual fee inside the interval can be accepted after it appears in gross effects and the three obligations hold. An actual or proposed fee above `fee_max` is a new plan and needs a new authorization. This is the Hyperliquid maximum, plus an explicit lower bound so a "fee" cannot be redefined as a credit, plus digest binding so the sponsor cannot edit the call.

Alternative, recorded and not chosen as the default. Exact fee commitment at `authorize` time rejects any later difference, including a venue fee that moves from 3 to 4 inside a cap of 5. That rule is simpler to audit and will refuse more live venues. The owner can select it per adapter profile. The default I recommend is the interval.

Gross accounting uses the authority-experiment constants as a worked check: principal 94, fee 4, gross 98, gross cap 100, fee cap 5. Acceptance requires `98 = 94 + 4`, `4 ≤ 5`, and `98 ≤ 100` as three checks on the actual effect, not on the quote.

---

## Adapter registration and upgrade

Source fact. The proposal's negative cases include an adapter that changes after authorization, and a service outage that suppresses a direct valid path. NEAR's transaction builder is cited there as evidence that a payload is not generic bytes without a pinned codec.

Recommendation. Registration is a signed federation record: `adapter_id`, `version`, `codec_hash`, domain, action class, evidence profile, and the settlement locus the adapter can speak about. A new version is a new record. Existing grants name the triple `(id, version, codec_hash)`. They do not follow a floating "latest."

In-flight legs keep the codec that was authorized. An observed codec hash mismatch yields `refused` with `adapter_codec_mismatch` before dispatch, and `unknown` if the mismatch is discovered after a submission may have left the process. History is not rewritten to the new codec.

Emergency disable revokes the adapter version for new exercise. Legs already submitted stay observable under the old evidence profile. Disable is not a success, a failure, or a refund.

Developer experience. Generated clients are parameterized by the codec hash. A response whose hash differs from the client pin is a typed refusal. The client has no fallback to another version. An unpinned client is a development stub and cannot call `authorize`.

Open. The registration quorum and any timelock are federation governance parameters. They do not change the pin rule.

---

## An SDK that keeps `unknown` visible

Recommendation. Ship two clients.

- `DirectVerifierClient` submits a canonical envelope to the settlement locus the program names. It does not ask the kernel for permission.
- `KernelClient` exercises grants. Every method that has crossed the network returns a receipt. Transport failure after dispatch is a receipt with phase `unknown`.

Proposed result shape, draft API, not an implementation:

```text
Receipt
  phase: planned | reserved | signed | submitted | included
       | finalized | delivered | refunding | refunded | failed | unknown
  refusal: null | { code, program_invalid: false }
  unknown: null | { reason, last_phase, permitted_next }
  authority_consumed: { occurrence } | null
  premises: TrustLabel[]
  effects: Effect[]          # empty until observed
  evidence_ref: null | id
```

Normative client rules:

1. `unknown` is a variant the caller matches. It is not an exception, not `false`, and not a timeout mapped to `failed`.
2. The client never promotes `signed` to `finalized`. A helper named `assertFinal` is not in the reference API. `requirePhase(receipt, finalized)` returns the receipt unchanged when the phase differs, and the type narrows only in the finalized branch.
3. `waitUntilTerminal` returns `unknown` with reason `deadline_exceeded` when its deadline passes. The bridge probe is the reason this must not start a refund.
4. `dispatch(leg_id)` is idempotent. A repeat returns the stored continuation and reconciles known attempts before any new attempt id exists.
5. Grant construction exposes `attenuate`. The types offer no `widenFee`, `setRecipient`, or `useLatestAdapter`. Hand-built JSON that widens is still refused by the kernel. The type narrowing is a convenience. The signature and the subset check are the authority.
6. HTTP 200 means a receipt was parsed. Phase on that receipt may be `unknown`, `refused`, or `signed`. HTTP 503 before a dispatch is recorded as not dispatched. HTTP 503 after the kernel has acknowledged a dispatch is `unknown` for that occurrence.
7. Each refusal code tells the caller whether authority was consumed and that `program_invalid` is false.

| Refusal or stop | Authority consumed | What the caller can do |
| --- | --- | --- |
| `fee_exceeds_grant` | No | Submit another plan inside the interval, or use the direct client |
| `domain_mismatch` | No | Direct client remains usable for the original envelope |
| `epoch_retired` | No | A new grant naming the new epoch, if the user signs one |
| `evidence_unverifiable` | No | Dependent transition does not run; observe again under the same issuer set |
| `revocation_status_stale` | No | Retry the revocation fetch; do not sign |
| `service_unavailable_after_dispatch` | Unknown until reconciled | Reconcile this occurrence |

Trust labels the reference UI renders on every receipt that has reached `authorize`: `user_signed_envelope`, `adapter_pinned`, `fee_interval`, `replay_locus_settlement` or `replay_locus_signer_log`, `quorum_may_bypass_moriarty` when applicable, the evidence class actually imported, and `phase_unknown` when that is the phase. A receipt with no premise list is a client bug.

Quote responses carry `operational: true` and are not acceptance. The proposal's separation of solver promises from language semantics is the reason the type exists.

---

## Canonical examples

Draft records. They are not an accepted encoding. Amounts use the authority-experiment relation so the three obligations are visible. The digest is a stand-in, not the Aeon integer tag `314159`.

### Root grant and attenuated signer grant

```json
{
  "record": "moriarty.Grant/0-draft",
  "grant_id": "grant-root-7f3a",
  "parent_grant_id": null,
  "holder": "user-key-1",
  "action_class": ["venue_call", "sign", "sponsor_fee", "observe", "recover"],
  "domain": "eip155:42161",
  "settlement_locus": "foreign_venue_ledger",
  "adapter": {"id": "arb-call", "version": "1.2.0", "codec_hash": "sha256:codec-arb-1.2.0"},
  "recipient": "eip155:42161:0x0000000000000000000000000000000000000042",
  "asset_id": "eip155:42161:erc20:0xusdc",
  "principal_cap": "94",
  "fee_min": "0",
  "fee_max": "5",
  "gross_cap": "100",
  "key_policy": {
    "scheme": "secp256k1-threshold",
    "epoch": 5,
    "derivation_path": "m/44/60/0/0/7",
    "quorum": "2-of-3",
    "quorum_bypass_disclosed": true,
    "successor_epochs": []
  },
  "evidence_profile": {
    "issuers": ["quorum-finality-v1"],
    "verifier": "midnight-import-v1",
    "finality": "arbitrum-safe-head",
    "max_age_seconds": 120,
    "downgrade": "forbidden"
  },
  "logical_occurrence": "9",
  "replay_locus": "replay_locus_signer_log",
  "expiry": "2026-09-29T00:15:00Z"
}
```

```json
{
  "record": "moriarty.Grant/0-draft",
  "grant_id": "grant-sign-7f3a",
  "parent_grant_id": "grant-root-7f3a",
  "holder": "signer-quorum-epoch-5",
  "action_class": ["sign"],
  "payload_digest": "sha256:payload-leg-destination-1",
  "fee_max": "4",
  "key_policy": {"epoch": 5, "derivation_path": "m/44/60/0/0/7"}
}
```

The child keeps domain, recipient, adapter triple, and epoch, and it lowers the fee maximum. Subset verification allows it.

### Sign request and the receipt the developer must stop on

```json
{
  "record": "moriarty.SignRequest/0-draft",
  "intent_id": "intent-7f3a",
  "leg_id": "leg-destination",
  "attempt_id": "attempt-1",
  "grant_id": "grant-sign-7f3a",
  "domain": "eip155:42161",
  "adapter": {"id": "arb-call", "version": "1.2.0", "codec_hash": "sha256:codec-arb-1.2.0"},
  "key_policy": {"scheme": "secp256k1-threshold", "epoch": 5, "derivation_path": "m/44/60/0/0/7"},
  "payload_digest": "sha256:payload-leg-destination-1",
  "recipient": "eip155:42161:0x0000000000000000000000000000000000000042",
  "value": "94",
  "fee": "4",
  "logical_occurrence": "9"
}
```

```json
{
  "record": "moriarty.OperationReceipt/0-draft",
  "intent_id": "intent-7f3a",
  "leg_id": "leg-destination",
  "attempt_id": "attempt-1",
  "phase": "signed",
  "payload_digest": "sha256:payload-leg-destination-1",
  "signature_ref": "mpc-sig-88",
  "key_epoch": 5,
  "effects": [],
  "evidence_ref": null,
  "authority_consumed": {"logical_occurrence": "9", "scope": "sign_only"},
  "premises": [
    "user_signed_envelope",
    "adapter_pinned",
    "mpc_quorum",
    "quorum_may_bypass_moriarty",
    "replay_locus_signer_log"
  ],
  "refusal": null,
  "unknown": null
}
```

A developer who renders this receipt as a completed trade has ignored `phase` and an empty `effects` array. Acceptance is a later receipt whose premises include the named finality verifier and whose effects satisfy the three obligations.

### Refusal, and an unknown continuation

```json
{
  "record": "moriarty.OperationReceipt/0-draft",
  "intent_id": "intent-7f3a",
  "leg_id": "leg-destination",
  "phase": "refused",
  "refusal": {
    "code": "fee_exceeds_grant",
    "authorized_fee_max": "5",
    "proposed_fee": "6",
    "program_invalid": false
  },
  "authority_consumed": null,
  "effects": []
}
```

```json
{
  "record": "moriarty.Continuation/0-draft",
  "intent_id": "intent-7f3a",
  "committed_prefix": ["leg-source finalized"],
  "legs": [{
    "leg_id": "leg-destination",
    "attempt_id": "attempt-1",
    "phase": "unknown",
    "unknown": {
      "reason": "observe_deadline_exceeded",
      "last_phase": "submitted",
      "permitted_next": ["observe", "escalate"]
    }
  }],
  "consumed_authority": ["occurrence-9-sign"],
  "cumulative_gross": "98",
  "refund_authorized": false
}
```

`permitted_next` contains `observe` and `escalate`. It does not contain `refund` while delivery is unestablished.

---

## Three positive traces

### P1. Exact leg, phases kept apart

1. The user signs `grant-root-7f3a` with recipient 42, domain `eip155:42161`, fee maximum 5, gross cap 100, epoch 5, path `m/44/60/0/0/7`, occurrence 9, and issuer set `{quorum-finality-v1}`.
2. A solver plan proposes principal 94, fee 4, the same recipient and domain, and the pinned codec. `validatePlan` checks `4 ≤ 5`, `98 ≤ 100`, and `98 = 94 + 4` separately. The plan is reserved.
3. The signer grant matches the recomputed payload digest. The quorum signs. The receipt phase is `signed`. Effects are empty.
4. A later observe imports inclusion, then a finality attestation from `quorum-finality-v1` whose verifier is `midnight-import-v1`. Only that receipt has phase `finalized`.
5. Actual effects are principal 94 and fee 4. Acceptance runs the same three obligations on the effects.

The Aeon signing probe accepted the analogous exact integer request and rejected using the signed tag as a finalized tag. P1 is the operational form of that separation. The probe is not the evidence for step 4.

### P2. Sponsorship inside the interval

1. The plan's expected fee is 3. The user interval is `[0, 5]`. The sponsor's child grant is `sponsor_fee` only, maximum 5, bound to `sha256:payload-leg-destination-1`.
2. The sponsor attests fee 3 over that digest. The venue later reports fee 4. `4` is inside the interval. Gross becomes 98. The three obligations hold on the observed effect.
3. The sponsor's attestation did not change recipient, domain, or digest. The user envelope signature still matches the original bytes.
4. The direct client was never disabled. A sponsor refusal at step 2 would have been `refused` with `program_invalid: false`.

### P3. Key rotation with the successor already pinned, and no pending signature

1. Before any pending sign request exists, the user signs a successor grant: epoch 6, same quorum policy, same path scheme, new key material already registered, `successor_epochs` empty on the new grant.
2. Federation publishes epoch 6 and retires epoch 5 for new `SignRequest`s. Consumption history for epoch 5 occurrences stays.
3. A new leg uses occurrence 10 and epoch 6. Steps 1–7 of the signer check succeed against the successor grant. The old key is not asked to sign.
4. An attempt to send occurrence 9 again under epoch 6 is `refused` as a different grant exercise, and the epoch 5 log still shows occurrence 9 if it was used.

P3 is the clean rotation. H1 is the same event with an unsigned pending leg and no successor pin.

---

## Five hostile traces

### H1. Key rotation against a pending unsigned request

State. Occurrence 9 is authorized under epoch 5. The quorum has not returned a signature. Epoch 5 is then retired. The root grant's `successor_epochs` is empty.

Hostile requests. (a) Ask epoch 6 to sign the epoch 5 payload. (b) Ask the retired epoch 5 key to sign after retirement. (c) Mark the leg failed and refund because rotation made the old key unusable.

Required results. (a) `refused`, `epoch_mismatch`. Epoch 6 is a different key policy; signing with it is a new grant. (b) `refused`, `epoch_retired`. (c) Phase remains `unknown` or becomes `refused` with reason `epoch_retired_before_signature`. `refund_authorized` stays false until a recovery grant's evidence rule is met. If a signature for occurrence 9 is later found, the phase becomes `signed` under epoch 5 and observation continues. Automatic resigning is not observation.

This is the proposal's negative case "a key epoch changes with pending duties," specialized to the unsigned window.

### H2. Replay, including the Hyperliquid prune failure

State. Attempt 1 for occurrence 9 was submitted. The client timed out and retries. Separately, an operator prunes the retired agent nonce store and reuses the signing address, which is the failure Hyperliquid documents.

Required results.

- The retry calls `dispatch(leg-destination)` and receives the stored continuation for attempt 1. No attempt 2 is created until reconciliation shows the domain did not accept attempt 1. If acceptance cannot be shown, the phase is `unknown`, not a fresh submission.
- A second payload under occurrence 9 is `refused`, `occurrence_consumed`, when the log still holds the first digest.
- After a prune-and-reuse, a foreign chain that never saw the Moriarty nullifier may still include the old signature. That inclusion is an effect of occurrence 9, not a new intent. The grant that used `replay_locus_signer_log` already displayed that limit. The kernel does not adopt address reuse, and it does not prune the occurrence log as a substitute for revocation.

The Aeon signing probe rejected a wrong nonce tag. It did not test this replay. H2 is a kernel and settlement rule, and the probe supplies no evidence for it.

### H3. Wrong-domain signing

State. The grant domain is `eip155:42161`. The Aeon probe's analogous tag was domain `1`. A caller submits a `SignRequest` whose domain is `eip155:1`, or whose outer domain matches while the pinned codec decodes a payload chain id of `eip155:1`.

Required results. Step 5 refuses both, with `domain_mismatch` or `payload_domain_mismatch`. The quorum is not called. No signature receipt is stored. `program_invalid` is false: the original envelope is unchanged, and the direct client can still submit it where that envelope was valid.

A second hostile variant is a generic "sign these bytes" request that skips the codec. The signer capability does not cover that request. Refusal code: `unpinned_payload`.

The Aeon probe rejected domain `11` against an authorized domain `1` as a refinement failure, and it accepted a `native` value in the authority probe's trust report. H3's byte check is the check the integer tags do not perform. A native or host "domain ok" bit does not satisfy step 4 or step 5.

EIP-712's domain separator is the EVM instance of the same idea: the chain id is inside the signed digest. Adapters for other chains encode the domain in the pinned codec and still pass step 5.

### H4. Unverifiable oracle evidence

State. A liquidation leg's evidence profile names issuer `hl-validator-oracle`, a specific round, a maximum age, and verifier `quorum-oracle-v1`. The kernel collects one of: a price whose issuer signature fails; a price from an issuer outside the set; a host RPC field `mark_price` with no issuer; or a `native`-style boolean `verified: true` with no attestation result.

Required results. The import is stored as `evidence_unverifiable`. The liquidation transition does not run. The leg phase is `unknown` with reason `evidence_unverifiable` and `permitted_next: [observe, escalate]`. The kernel does not substitute a second oracle, an operator report, or a stale round. Elapsed time does not become a price and does not become proof that the round will never arrive.

If the user-signed profile had named a backup issuer, the receipt identifies which issuer was used. Silent substitution is a different event and is a refusal of the kernel's behavior, `evidence_downgrade`.

HIP-3's separation of a bond from a correct price applies here. A slashing story attached to the bad issuer is a governance remedy. It is not a price the liquidation can use.

The authority probe's acceptance of `native "1"` as a finalized token is the same class of mistake. The client that promotes that token to `finalized` has repeated the probe's trust-report case and ignored the report.

### H5. Fee increase

State. The signed interval is `[0, 5]`. The reserved plan fee is 3. Principal is 94. Gross cap is 100.

Hostile requests.

- The solver replaces the fee with 6 and asks for a signature.
- The sponsor attestation covers fee 6 and a new recipient.
- After a payload authorized at fee 4 is submitted, a receipt claims fee 6 and asks the kernel to accept.

Required results.

- Fee 6 is `refused`, `fee_exceeds_grant`, before any quorum call. Occurrence 9 is not consumed by the refusal. The authority probe rejected fee 6 against cap 5 when that obligation stood alone, and accepted a combined gross encoding of a different breach. The kernel check is the separate fee obligation, evaluated in the settlement path, not an Aeon conjunct.
- The sponsor's new recipient is `refused`, `recipient_changed`, as well as `fee_exceeds_grant`. The sponsor child grant's action class is `sponsor_fee` only.
- A post-submission claim of fee 6 does not become acceptance. If the domain effect is unconfirmed, the phase stays `submitted` or `unknown`. If the domain effect is confirmed at fee 6, the three obligations fail acceptance, the effect stays committed, and recovery follows the recovery grant. The kernel does not report `failed` in order to hide a committed debit, and it does not refund the full principal while destination delivery is only partly known. That last prohibition is the bridge probe's partial-delivery case, applied to a fee that changed the gross.

A fee that moves from 3 to 4 inside the interval is not this hostile trace. P2 accepts it after the actual effect is observed. The hostile event is the move above the signed maximum, or a fee change that edits the payload after authorization.

---

## What I am not deciding

These remain open. Each one changes trust, which is why they are listed here rather than filled in.

| Open decision | Why it stays open | What this memo still requires |
| --- | --- | --- |
| Initial custody model, and who controls bridge and signing keys | The NEAR proposal states that NEAR does not decide this for Moriarty | Whoever holds the keys, steps 1–7 run before a signature, and quorum bypass is disclosed |
| In-circuit proof versus a named quorum for a foreign fact | Verifier locus is an owner choice | The chosen premise is named on the grant; the kernel does not downgrade it |
| First external adapter | Implementation sequencing, and the proposal's U5 slice | The first adapter is pinned by version and codec hash, with one simulated foreign domain as that proposal describes |
| Exact-byte grant encoding, M4-C5 | The proposal leaves the bytes and the verifier locus to their own decision | The fields in this memo are the semantic content of that encoding |
| Registration quorum and timelock | Federation governance | Upgrades do not migrate in-flight codec pins |
| Exact fee commitment versus the interval default | Operational brittleness versus audit simplicity | Above-maximum fees re-authorize; actual fees enter gross effects |
| Aeon defect cause, and whether a later commit repairs it | The authority report leaves the cause unidentified | This pin is not an admission or settlement gate |

Solver bonding, fee settlement mechanics, and operational eligibility come after the safety contract, as the NEAR proposal already says. This memo does not design them.

---

## Dissent

I dissent from making the kernel mandatory for a program the settlement verifier can check directly. A mandatory coordinator would give developers one status enum, and it would also make kernel outage a liveness failure for an otherwise valid program. The proposal's outage discriminator is the right one: an outage must not suppress the direct path. The SDK rule that `program_invalid` stays false on kernel refusal is the same dissent in type form.

I dissent from shipping Model II agent keys as a temporary default "until caveats are ready." The Hyperliquid prune-and-reuse failure is a property of that temporary default, and consumption history is hardest to add after addresses have been reused. A tight session grant under Model I is the temporary form I would accept.

I dissent from exact fee commitment as the only legal profile. Venues change fees inside a user maximum, and Hyperliquid's builder codes are a worked example with on-chain enforcement and revocation. I recommend the signed interval as the default. Exact commitment can be an adapter-level profile the user opts into. I would oppose a third mode in which the operator raises the fee and the SDK shows the quote.

I dissent from displaying Aeon success as assurance, including after a hypothetical repair of the impossible-refinement bug. A repaired authoring checker would still not authenticate envelope bytes, adapter bytes, MPC behavior, or ledger effects. The signing report already says that. I would allow the probe to remain a labeled diagnostic that prints its trust report. I would not allow that diagnostic to sit on the sign path.

I record a standing objection to any later proposal that collapses `signed`, `submitted`, and `finalized` for developer convenience. The signing probe's most useful positive result is that those phase substitutions can be rejected when they are actually encoded. The client should not put them back together in a boolean.

---

## Sources

Packet documents this memo relies on:

- `deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`
- `deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`
- `deliverables/aeon-kernel-experiments-2026-09-29/signing/RESULT.md`
- `deliverables/aeon-kernel-experiments-2026-09-29/authority/REPORT.md`
- `deliverables/aeon-kernel-experiments-2026-09-29/bridge/RESULT.md`

Primary pages cited through that packet:

- NEAR Intents verifier: https://docs.near-intents.org/integration/verifier-contract/introduction
- NEAR Intents contract repository: https://github.com/near/intents
- NEAR Intents market makers: https://docs.near-intents.org/integration/market-makers/introduction
- Archived 1Click API snapshot: https://github.com/defuse-protocol/gitbook-docs/blob/main/integration/distribution-channels/1click-api.md
- NEAR Intents bridges: https://docs.near-intents.org/integration/bridging/overview
- NEAR Chain Signatures: https://docs.near.org/chain-abstraction/chain-signatures
- NEAR MPC repository: https://github.com/near/mpc
- NEAR transaction builder: https://github.com/near/omni-transaction-rs
- NEAR BTC deposit refund: https://docs.near-intents.org/integration/bridging/btc-deposit-refund
- Hyperliquid HyperCore overview: https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview
- Hyperliquid order book: https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/order-book
- HyperEVM interacting with HyperCore: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore
- HyperEVM interaction timings: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interaction-timings
- Hyperliquid builder codes: https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes
- Hyperliquid nonces and API wallets: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets
- Hyperliquid exchange endpoint: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint
- HIP-3: https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-improvement-proposals-hips/hip-3-builder-deployed-perpetuals
- Hyperliquid oracle: https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/oracle
- Aeon repository and pinned tree: https://github.com/alcides/aeon and https://github.com/alcides/aeon/tree/ef66bd95e6b7d2d5309453ee63640bc7fa1d988f

Five further standards, identified from prior knowledge and not re-fetched for this memo:

1. EIP-712, typed structured data and domain separation: https://eips.ethereum.org/EIPS/eip-712
2. EIP-4337, account abstraction, validation versus execution, paymaster sponsorship: https://eips.ethereum.org/EIPS/eip-4337
3. RFC 9334, Remote ATtestation procedureS, evidence roles and appraisal policy: https://www.rfc-editor.org/rfc/rfc9334
4. CAIP-2, chain identifier grammar: https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-2.md
5. EIP-2612, signed allowances with nonce and deadline: https://eips.ethereum.org/EIPS/eip-2612

Bibliographic identification, no URL attached here: Birgisson, Politz, Erlingsson, Taly, Vrable, and Lentczner, "Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud," NDSS 2014.