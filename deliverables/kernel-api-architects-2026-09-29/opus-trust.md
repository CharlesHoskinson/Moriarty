Both `Write` and `ExitPlanMode` are disabled in this session, and plan mode forbids me from writing files by other means. So I'm delivering the memo inline as my output rather than to `deliverables/kernel-api-architects-2026-09-29/opus-trust.md`. Say the word (or re-run with edits enabled) and I'll write the identical text to that path.

---

# Authority, Trust and Developer Experience for an Optional Moriarty DeFi Kernel

**Architect:** independent `trust`-theme architect (`claude-opus-5`), nine-architect study.
**Date:** 2026-09-29.
**Status:** advisory design research. Not an implementation, not product acceptance, not a Moriarty adopted profile, and **not a consensus position** — the other eight architects' answers were deliberately not inspected, and nothing here should be read as agreement with them.

## 0. Method and provenance

Startup per `AGENTS.md`: the host did not expose `moriarty-dev:develop`, so the checked-in develop skill (`plugins/moriarty-dev/skills/develop/SKILL.md`) was read and applied. The guarded CLI was run read-only from the worktree root:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
→ capability: "SP01.6 loan-swap-subset"
  blockedAction: "implementation/repair of loan-swap-subset"
  missingEvidence: [binding-input-stale:…sp01-financial-contract-and-execution-admission.md,
                    candidate-input-stale:…(same), current-accounting-missing:.moriarty-dev/runtime/current-accounting.json,
                    resource-live-state-unavailable:sp01-loan-swap-grok-01, operational-history]
  pendingTransactions: []
```

No pending transaction notification is owed. The implementation slot is itself blocked on unresolved operational history, which is an additional reason this memo confines itself to design.

Local inputs read in full: `deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`; `deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`; `deliverables/aeon-kernel-experiments-2026-09-29/signing/RESULT.md`; `docs/MORIARTY-CONSOLIDATED-DESIGN.md`.

Throughout, **Fact:** marks a claim traceable to a cited source's own text, and **Inference:** marks my design reasoning. NEAR and Hyperliquid facts are attributed to the two supplied deliverables, which carry their own capture ledgers; I did not re-retrieve those pages.

## 1. Primary sources beyond the supplied NEAR/Hyperliquid pages

Five sources, retrieved 2026-09-29. Status and date matter as much as content: a Draft ERC is a *shape to borrow*, never a dependency to rely on.

| Tag | Source | Exact URL | Status / date |
|---|---|---|---|
| **S1** | ERC-7715, *Request Permissions from Wallets* | `https://eips.ethereum.org/EIPS/eip-7715` (text also read at `https://raw.githubusercontent.com/ethereum/ERCs/master/ERCS/erc-7715.md`) | **Draft**, created 2024-05-24, `requires: 4337, 7710` |
| **S2** | EIP-712, *Typed structured data hashing and signing* | `https://eips.ethereum.org/EIPS/eip-712` | Final |
| **S3** | ERC-4337, *Account Abstraction Using Alt Mempool* | `https://eips.ethereum.org/EIPS/eip-4337` | Final |
| **S4** | RFC 9591, *The Flexible Round-Optimized Schnorr Threshold (FROST) Protocol* | `https://www.rfc-editor.org/rfc/rfc9591.html` | IETF RFC (Informational), 2024 |
| **S5** | *Verifiable Credentials Data Model v2.0* | `https://www.w3.org/TR/vc-data-model-2.0/` | **W3C Recommendation, 15 May 2025** |

Source facts I rely on:

- **S1 Fact:** the methods are `wallet_requestExecutionPermissions`, `wallet_revokeExecutionPermission`, `wallet_getSupportedExecutionPermissions`, `wallet_getGrantedExecutionPermissions`. The request carries `chainId`, optional `from`, `to`, a `permission` object with `type`, `isAdjustmentAllowed` and `data`, and an optional `rules` array (the documented example rule is `type: "expiry"` with `data.timestamp`). The response returns the request fields plus an opaque `context` used for revocation and redemption, a `delegationManager` address, and a `dependencies` array for undeployed accounts. Security considerations state that wallets MUST correctly enforce permissions, that dApps should request minimal permissions with reasonable expirations, and that malicious dApps may pose as legitimate ones to obtain broad grants.
  **Note on drift:** older secondary write-ups call the first method `wallet_grantPermissions`. The current text uses `wallet_requestExecutionPermissions`. That rename inside a Draft is precisely why S1 is cited as a shape, not an interface contract.
- **S2 Fact:** `EIP712Domain` carries `name`, `version`, `chainId`, `verifyingContract` and `salt`; signing is `sign(keccak256("\x19\x01" ‖ domainSeparator ‖ hashStruct(message)))`; the domain separator exists so that "two DApps come up with an identical structure … that should not be compatible" cannot collide. **Critically, S2 places replay detection outside its own scope** — recognising the same signed message twice is left application-specific.
- **S3 Fact:** `validatePaymasterUserOp` receives the user operation, its hash and a `maxCost`, and returns a `context` byte array plus `validationData`; `postOp` runs when `context` is non-empty and is called with mode `opSucceeded` **or `opReverted`, in which case the paymaster still pays gas**; `paymasterAndData` encodes the paymaster address, `paymasterVerificationGasLimit`, `paymasterPostOpGasLimit` and `paymasterData`; the operation carries `maxFeePerGas` and `maxPriorityFeePerGas`, which a bundler must check against a configurable minimum; a 10% penalty applies to unused `callGasLimit` and `paymasterPostOpGasLimit` when unused gas exceeds 40000; maliciously crafted paymasters are called out as a DoS risk mitigated by reputation or stake.
- **S4 Fact:** FROST is a two-round threshold Schnorr protocol with a Coordinator and signer participants holding Shamir shares. The Coordinator "is not trusted with any private information" and signing traffic may cross a public but authenticated and reliable channel; the Coordinator *is* trusted not to mount DoS and to identify misbehaving participants. Round one publishes hiding and binding nonce commitments; binding factors are computed over the group public key, the message hash and the encoded commitment list. The protocol is **not robust** — one malformed or absent share denies service. Deterministic nonce derivation "allows for a complete key-recovery attack." Participant identifiers are `NonZeroScalar` values that MUST be distinct. **Key generation is explicitly out of scope**, and key refresh/rotation is not addressed.
- **S5 Fact:** roles are issuer, holder, subject, verifier, plus a verifiable data registry. *Verification* is "the evaluation of whether a verifiable credential … is an authentic and current statement of the issuer or presenter"; *validation* is "the assurance that a claim from a specific issuer satisfies the business requirements of a verifier for a particular use." The specification states plainly: **"Verifiability of a credential does not imply the truth of claims encoded therein."** Validity is bounded by `validFrom` and `validUntil`; `credentialStatus` conveys revocation or suspension and must be checked when present. Verifiers "trust certain issuers for certain claims and apply their own rules."

Supplied-deliverable facts I build on, restated so this memo is self-contained:

- The NEAR proposal establishes that a verifier contract holds the accepted relation and that a relay cannot replace it; that solver discovery is optional and not language semantics; that each bridge route has a distinct trust model; that a threshold signature over a payload is neither authorization nor proof of execution; and that recovery is an explicit, costed workflow. It also sets the rule I treat as load-bearing: the signing service must have **no generic "sign arbitrary bytes" authority** from an intent.
- The Hyperliquid comparison establishes that a same-chain system call can still be asynchronous (`enqueued` then `executed` are separate facts); that a user-approved *per-builder maximum* fee is a distinct authority from an operator's quoted fee and is revocable; that an external signer's ephemeral nonce store is unsafe as the system of record, because the docs warn that pruning a deregistered agent's nonce state can make old actions replayable if the address is reused; that a universal `success` receipt destroys meaningful state (resting vs filled); and that validator-published oracle inputs remain an issuer assumption.
- The consolidated design supplies the constraints I must not violate: signed intention and program acceptance are independent of kernel service eligibility; a host JSON field or host-computed Boolean is insufficient evidence; revocation cannot erase outstanding duties; scoped recovery authority may outlive ordinary authority under its own signed termination rule; a refund must not replenish gross authority unless explicitly authorized; and a foreign destination accepting only a threshold signature can be bypassed if that threshold is compromised even when honest signers check proofs.

## 2. Three authority models compared

The question is *what the settlement verifier actually accepts*, not what the kernel believes.

### Model A — standing session grant

The user signs a grant naming permitted call classes, caps and an expiry; the kernel (or a dApp session account) holds a key and acts within the grant. This is the S1 shape, and the `context`/`delegationManager`/revocation triad makes it operationally clean.

- **Strength:** excellent DX. Automation, subscriptions, multi-leg continuation and recovery all work with the user offline. Standing authority is enumerable (`wallet_getGrantedExecutionPermissions`) and revocable by opaque handle.
- **Failure mode (Inference):** the enforceable relation becomes the *grant*, not the outcome. Within its caps, the holder may originate financial relations the user never priced — a swap at a legal-but-terrible rate, a liquidation-adjacent action, a fee at the ceiling rather than the quote. Compromise of one session key is bounded only by caps and time, and caps are a blunt instrument: "spend ≤ 10 000 USDC by Friday" is not "accept ≥ 9 950 USDC net out of 10 000 USDC in."

### Model B — per-outcome envelope only

The user signs the exact outcome envelope; the kernel holds no standing authority whatsoever. This is the NEAR Verifier shape as described in the supplied proposal.

- **Strength:** the signed intention *is* the accepted relation. Kernel compromise cannot change the financial result; the worst case is denial of service, and a direct Midnight program remains valid regardless.
- **Failure mode (Inference):** asynchronous reality does not fit. A bridge leg pending for an hour, a resting order, a late oracle round, or a refund whose precondition matures after expiry all require *someone* to act after the user's signing session ended. Model B either forces the user online at unpredictable moments or silently pushes authority into an operator's discretion — which is Model A without the honesty.

### Model C — envelope-rooted narrowing grants (recommended)

The user-signed **outcome envelope is the sole root** the settlement verifier accepts. Grants exist, but only as *strictly narrowing derived authorities* over an envelope that already exists.

Two rules carry the model:

1. **Monotone narrowing.** A child grant may never widen recipient, domain, asset, gross spend, fee ceiling, permitted call class, adapter version, expiry, or required evidence premise relative to its parent. `narrowsFrom` makes the chain explicit and checkable without trusting the issuer's intent.
2. **Two-key rule.** No kernel-held or delegated key may *originate* a financial relation. It may only *advance* a relation already rooted in a user envelope. Origination requires the root signature; advancement requires a derived grant plus a payload commitment that the envelope already fixes.

Scoped **recovery authority** is the single sanctioned exception to co-termination: it may outlive ordinary authority under its own signed termination rule, per the consolidated design. It is still narrowing — a recovery grant can cancel, refund or compensate, and cannot open a new position.

| Property | A: session grant | B: envelope only | C: envelope-rooted narrowing |
|---|---|---|---|
| What the verifier enforces | grant caps | exact outcome | exact outcome, with grants bounding *dispatch* only |
| Blast radius of kernel key compromise | any relation inside caps | denial of service only | denial of service, plus mis-sequencing inside a relation the user already priced |
| Offline async continuation | yes | no | yes |
| Revocation latency | registry/wallet-dependent | n/a (nothing standing) | registry-dependent, but bounded by the envelope regardless |
| Can service refusal invalidate a direct Midnight program? | must not, and does not | must not, and does not | must not, and does not |
| Hostile-trace coverage (§12) | fails H4, H5 without extra machinery | fails nothing but cannot reach P3 | passes all five with the mechanisms in §3 |

**Inference:** C is A's ergonomics under B's acceptance semantics. It costs one extra concept (the root/derived distinction) and one extra check (narrowing) and buys the property that no operator can price a relation the user did not.

## 3. Versioned records, with canonical request/response examples

Everything is a versioned typed record. There is no `call()` returning a Boolean anywhere in this design.

### 3.1 `Grant/1` — scoped grant

The domain separator is S2's lesson transposed out of EVM terms. A Moriarty grant must be uninterpretable outside its exact program, numeric/semantic profile and ledger, because the same struct under a different rounding convention or price orientation is a different financial instruction.

```json
{
  "record": "Grant/1",
  "grantId": "grant:01JB8Q…",
  "moriartyDomain": {
    "name": "moriarty-defi-kernel",
    "version": "1",
    "programId": "prog:sha256:9f2c…",
    "semanticProfile": "mil/2+numeric/u0",
    "ledger": "midnight:preview",
    "verifyingLocus": "ledger-primitive:intent-auth/1",
    "salt": "0x7d1a…"
  },
  "root": { "kind": "envelope", "envelopeId": "env:sha256:41ab…" },
  "narrowsFrom": null,
  "grantee": { "keyId": "kernel-signer:eu-1", "keyEpoch": 7 },
  "domains": ["midnight:preview", "eip155:42161", "hypercore:mainnet"],
  "permittedCallClasses": [
    { "adapterId": "arb-usdc-bridge", "adapterVersion": "3.2.0",
      "codecId": "codec:evm-tx/eip1559@1", "action": "withdraw",
      "settlementLocus": "bridge-destination" },
    { "adapterId": "hypercore-order", "adapterVersion": "1.4.1",
      "codecId": "codec:hypercore-action@4", "action": "limit-order",
      "settlementLocus": "foreign-venue-ledger" }
  ],
  "caps": {
    "grossSpend": [ { "asset": "eip155:42161/erc20:0xaf88…", "max": "10000.000000" } ],
    "feeCeiling": { "user": "4.500000", "service": "1.000000", "venue": "2.000000",
                    "asset": "eip155:42161/erc20:0xaf88…" },
    "occurrences": 2
  },
  "rules": [
    { "type": "notBefore", "data": { "timestamp": 1790000000 } },
    { "type": "expiry", "data": { "timestamp": 1790086400 } },
    { "type": "recipientAllowList", "data": { "recipients": ["eip155:42161:0xA11ce…"] } },
    { "type": "payloadCommitmentRequired", "data": { "mode": "exact-digest" } },
    { "type": "oneShot", "data": { "logicalRequestScope": "per-leg" } }
  ],
  "isAdjustmentAllowed": false,
  "evidencePolicyRef": "evpol:bridge-strict/2",
  "revocation": { "registry": "midnight:preview/rev-registry@1", "statusId": "st:0x51…" },
  "signature": { "scheme": "midnight-intent-sig/1", "value": "0x…" }
}
```

Response:

```json
{
  "record": "GrantAck/1",
  "grantId": "grant:01JB8Q…",
  "accepted": true,
  "context": "0x9c41…",
  "enforcementLocus": "ledger-primitive:intent-auth/1",
  "unenforcedFields": ["caps.feeCeiling.service"],
  "narrowingProofChain": ["env:sha256:41ab…", "grant:01JB8Q…"],
  "expiresAt": 1790086400
}
```

**Inference, and the single most important field in this memo:** `unenforcedFields`. A grant record can express constraints that no verifier actually checks — a kernel-side service fee ceiling, for instance, may be an operator promise rather than a ledger predicate. The API must *name* the gap on every acknowledgement rather than let a well-typed record imply enforcement. The consolidated design's rule that a host-computed Boolean is insufficient becomes an API obligation here, not a footnote.

`isAdjustmentAllowed: false` is borrowed from S1 and defaulted to false; S1 leaves it per-permission.

### 3.2 `SignRequest/1` and `SignAuthorization/1`

```json
{
  "record": "SignRequest/1",
  "bindsEnvelope": "env:sha256:41ab…",
  "underGrant": "grant:01JB8Q…",
  "legId": "leg:withdraw-usdc",
  "attemptId": "att:3",
  "logicalRequestId": "lrq:sha256:c0de…",
  "targetDomain": "eip155:42161",
  "adapterId": "arb-usdc-bridge", "adapterVersion": "3.2.0",
  "codecId": "codec:evm-tx/eip1559@1",
  "payloadDigest": "sha256:8a7f…",
  "payloadPreimageRef": "blob:local/att3.bin",
  "derivationPath": "m/44'/60'/0'/0/11",
  "scheme": "frost-secp256k1-sha256/1",
  "keyEpoch": 7,
  "recipient": "eip155:42161:0xA11ce…",
  "valueBound": { "asset": "eip155:42161/erc20:0xaf88…", "max": "10000.000000" },
  "validFrom": 1790001000, "validUntil": 1790001600
}
```

```json
{
  "record": "SignAuthorization/1",
  "requestDigest": "sha256:1f4b…",
  "decision": "authorized",
  "consumed": { "logicalRequestId": "lrq:sha256:c0de…", "consumptionRecord": "cons:0x77…" },
  "phase": "signed",
  "signature": { "scheme": "frost-secp256k1-sha256/1", "keyEpoch": 7, "value": "0x…" },
  "notEstablished": ["broadcast", "inclusion", "finality", "delivery"]
}
```

**Inference:** `notEstablished` is deliberately part of the success response. The Aeon result and the NEAR proposal agree that a signature is not execution; the SDK type system should not be the only place that says so, because integrators read JSON.

`logicalRequestId` consumption is recorded **durably in Moriarty's own state**, not in the signer's nonce store. This directly answers the Hyperliquid warning restated in §1: an external signer's nonce pruning plus address reuse can resurrect old actions, so consumed authority must not be derived from the signer's ephemeral bookkeeping. S2's explicit exclusion of replay from its own scope points the same way: the application owns replay, always.

There is no method that signs caller-supplied bytes. `payloadDigest` is only acceptable when the kernel can recompute it from `payloadPreimageRef` through the pinned `codecId` and match every field the envelope constrains.

### 3.3 `EvidenceClaim/1`

The `verifierPremise` ladder is ordered, and downgrade is a rejected transition rather than a fallback:

| Premise | What it establishes | May bear acceptance? |
|---|---|---|
| `in-circuit-proof` | the statement under the pinned verifier | yes |
| `light-client` | inclusion/finality under the client's own assumptions | per policy |
| `threshold-attestation` | a quorum signed this statement (S4: participation, nothing more) | per policy, with bypass disclosure |
| `operator-report` | one operator says so | no |
| `native-declaration` | a host or type-level assertion | **never** |

```json
{
  "record": "EvidenceClaim/1",
  "statement": { "kind": "bridge-delivery",
                 "legId": "leg:withdraw-usdc",
                 "asset": "eip155:42161/erc20:0xaf88…",
                 "amount": "9987.250000",
                 "recipient": "eip155:42161:0xA11ce…" },
  "issuer": "issuer:omni-relayer-quorum@2",
  "verifierPremise": "threshold-attestation",
  "quorum": { "threshold": 5, "total": 8, "keyEpoch": 4,
              "bypassDisclosure": "compromise of 5 shares can assert this statement without Moriarty checking it" },
  "round": null, "height": 284113907,
  "finalityRule": "arbitrum-l1-confirmed",
  "validFrom": 1790002000, "validUntil": 1790005600,
  "status": { "registry": "issuer-status@1", "state": "active" },
  "verificationVerdict": "authentic-and-current",
  "validationVerdict": "satisfies:evpol:bridge-strict/2"
}
```

**Inference, straight from S5:** `verificationVerdict` and `validationVerdict` are two fields because they are two questions, and collapsing them is the standard error. A perfectly authentic attestation from an issuer this policy does not trust for this statement must be representable as `verification: authentic-and-current, validation: refused`. S5's sentence — verifiability does not imply truth — is the whole reason the kernel cannot manufacture a fact by labelling it `verified`.

### 3.4 `FeeAuthorization/1` and `Sponsorship/1`

Three fee populations are separated, following the Hyperliquid builder-code lesson: the user's signed gross ceiling, the kernel's service fee, and the venue/chain fee.

```json
{ "record": "FeeAuthorization/1",
  "bindsEnvelope": "env:sha256:41ab…",
  "ceilings": { "user": "4.500000", "service": "1.000000", "venue": "2.000000",
                "asset": "eip155:42161/erc20:0xaf88…" },
  "quotedAtAuthorization": { "user": "3.100000", "service": "0.750000", "venue": "1.400000" },
  "repriceRule": "reject-above-ceiling",
  "refundReplenishesGrossCap": false }
```

```json
{ "record": "Sponsorship/1",
  "sponsor": "sponsor:moriarty-fed-eu@1",
  "sponsorContext": "0x2b8e…",
  "maxCost": "2.000000",
  "postOpLiability": { "onSuccess": "sponsor", "onRevert": "sponsor", "onUnknown": "sponsor-until-reconciled" },
  "reimbursement": { "from": "user", "cap": "2.000000", "condition": "delivered-and-reconciled" },
  "sponsorWithdrawalNotice": 300 }
```

**S3 Fact → Inference:** because `postOp` runs with mode `opReverted` and the paymaster still pays, sponsorship liability on failure must be written down *before* dispatch, and `maxCost` must be a real bound rather than an estimate. The 10% penalty on over-reserved gas limits is a reminder that over-quoting is not free, so the SDK should surface reservation efficiency rather than encourage inflated ceilings.

### 3.5 `AdapterRegistration/1`

```json
{ "record": "AdapterRegistration/1",
  "adapterId": "arb-usdc-bridge", "adapterVersion": "3.2.0",
  "codecId": "codec:evm-tx/eip1559@1",
  "domain": "eip155:42161",
  "actionClasses": ["withdraw", "claim", "refund"],
  "byteFixtureDigest": "sha256:dd41…",
  "settlementLocus": "bridge-destination",
  "trustProfile": { "verifierPremise": "threshold-attestation", "bypassDisclosure": "…" },
  "reviewRecord": "review:2026-09-20/two-independent",
  "activationEpoch": 41,
  "supersedes": "arb-usdc-bridge@3.1.4",
  "deprecation": { "state": "active", "pendingDutyDrain": null } }
```

Rules (Inference): registration is append-only and versions are immutable; an upgrade is a **new version**, never an in-place edit, because grants pin `adapterId@adapterVersion` and the supplied NEAR proposal lists "an adapter changes after authorization" as a decisive negative case. Deprecating a version moves it to `draining`: no new authorizations, existing pending duties may still be advanced, and the version cannot be deleted while `pendingDutyDrain` is non-empty. A codec change is a version change even when the wire bytes look compatible, because `byteFixtureDigest` is what a grant is really pinning.

### 3.6 `Outcome/1`

```json
{ "record": "Outcome/1",
  "variant": "Unknown",
  "envelopeId": "env:sha256:41ab…",
  "committedPrefix": ["leg:approve", "leg:swap"],
  "unresolvedLegs": [ { "legId": "leg:withdraw-usdc", "lastPhase": "submitted",
                        "attempts": ["att:1","att:2","att:3"],
                        "whyUnknown": "no authenticated destination observation within finality window" } ],
  "consumedAuthority": { "grossSpent": "10000.000000", "feesCharged": "3.900000",
                         "occurrencesUsed": 1, "logicalRequestsConsumed": ["lrq:sha256:c0de…"] },
  "residualDuties": [ { "duty": "reconcile-or-recover", "before": 1790092800 } ],
  "recoveryHandle": "rec:0x88fa…",
  "lateResultRace": "a destination delivery after refund initiation remains possible" }
```

The six variants are `Settled`, `CommittedPrefix`, `Pending`, `Unknown`, `Refused` (service declined; program validity untouched), `Rejected` (a judgment rejected it). **Inference:** `Refused` and `Rejected` are separate because the consolidated design requires that optional-kernel service eligibility never be confused with program validity.

## 4. Delegation and revocation

- **Narrowing check is structural**, evaluated on the chain `[envelope, …grants]` without trusting any issuer's stated intent; depth is bounded (I suggest 3) so that the chain is auditable in a signing display.
- **Revocation is prospective.** Per the consolidated design, revocation cannot erase outstanding duties. A revoked grant stops authorizing new dispatch; it does not cancel a submitted leg, and it does not discharge a liability. Revoking while `Unknown` legs exist must return the `recoveryHandle` rather than silently orphaning them.
- **Enumeration is a user right.** S1's `wallet_getGrantedExecutionPermissions` is the right idea and I adopt it: any holder of standing authority over a user must be listable by that user, with each entry showing caps, expiry, consumed amounts and `unenforcedFields`.
- **Key-epoch retirement ≠ revocation.** Retiring `keyEpoch 7` invalidates future signatures under it while leaving grants, consumption records and duties intact. Conversely, revoking a grant does not retire a key that other grants still use. Conflating these is the root of hostile trace H1.
- **Recovery grants declare their own termination rule** — an expiry or an owner-chosen indefinite duration with scoped revocation — and cannot be implicitly co-terminated with ordinary authority.

## 5. Multichain signature request authority: what FROST does and does not buy

**S4 Facts → Inferences:**

1. The Coordinator is untrusted for secrets but trusted against DoS, and FROST is **not robust**. So kernel availability is a liveness assumption that must be stated in the trust profile, and "the quorum did not sign" can never be read as "the transaction did not execute."
2. Binding factors bind the group key, the message hash and the commitment list — so a share cannot be transplanted onto a different message *within FROST*. That is message binding at the signature layer; it says nothing about whether those bytes were the *authorized* bytes. That check is entirely Moriarty's, via `payloadDigest` recomputed through the pinned codec. This is exactly the gap the Aeon experiment probes at the type level and cannot close at the byte level (§13).
3. Deterministic nonces enable complete key recovery. So signer implementations are in scope for review, and a "deterministic for reproducibility" request from an integrator must be refused.
4. **Key generation is out of scope of RFC 9591, and rotation/refresh is unaddressed.** Therefore key rotation policy is Moriarty's own design obligation and cannot be imported. Concretely: an epoch register with `activeFrom`/`retiredAt`, refusal to sign under a retired epoch, a requirement that pending duties be re-authorized rather than inherited across epochs, and identifier non-reuse (S4 already requires distinct participant identifiers; I extend that to never reusing a retired `keyId`).
5. A threshold signature is participation of a quorum over bytes. Per the consolidated design, a destination that accepts only a threshold signature is bypassable if the threshold is compromised **even when honest signers check proofs** — so `bypassDisclosure` is a mandatory, non-empty string on every `threshold-attestation` profile.

## 6. Evidence issuer and verifier policy

Policy is per-statement, not per-issuer (S5: verifiers trust certain issuers for certain claims). `evpol:bridge-strict/2` names, for each statement kind: the acceptable premise set, the acceptable issuers, freshness (`validUntil`), required finality rule, whether a status check is mandatory, and the conjunction/threshold rule when multiple claims are combined. Two rules are absolute:

- **No silent downgrade.** If policy requires `light-client` and only `operator-report` is available, the transition does not occur and the outcome stays `Unknown`. It does not become `Settled` with a weaker note attached.
- **Time is not evidence.** Per the consolidated design and the supplied NEAR refund workflow, a timeout changes which authorized transitions may be attempted; it does not prove non-execution, and where non-receipt cannot be authenticated, time alone cannot authorize a refund that conflicts with a late delivery. `lateResultRace` on `Outcome/1` exists so that this is visible in the API rather than only in prose.

## 7. Fee sponsorship

| Situation | Who bears cost | Authority consequence |
|---|---|---|
| Success within quote | sponsor pays venue; user reimburses ≤ `maxCost` | fees enter gross effects and the net-outcome check |
| Execution reverts | sponsor still pays (S3 `opReverted`) | reimbursement condition `delivered-and-reconciled` is unmet, so no user charge; fee is a sponsor loss |
| Outcome `Unknown` | sponsor bears until reconciled | user's gross cap is charged only against established effects |
| Venue/chain reprices above ceiling | nobody; the leg is not dispatched | `Rejected: RepriceRequired` — see H5 |
| Sponsor withdraws mid-flight | notice window; pending legs drain | pending duties do not evaporate with the sponsorship |
| Refund received | — | `refundReplenishesGrossCap: false` — a refund must not top up authority to evade the gross cap |

**Inference:** sponsorship is an operator trust boundary with an economic flavour, never an authority. A sponsor may decline, but a sponsor may never widen a recipient, raise a ceiling or accept a weaker evidence premise.

## 8. An SDK that does not hide unknown outcomes

Design rules, in priority order:

1. **No boolean success exists in the surface.** Every terminal call returns `Outcome/1`.
2. **`Unknown` is not unwrappable.** There is no `unwrap()`, `isOk()`, `orElse(default)`, or truthiness coercion that turns `Unknown` into a value. The only ways to consume it are `reconcile(handle)`, `recover(handle, remedy)`, or `park(handle, reason)` — and `park` records the reason durably. In Rust this is a `#[must_use]` non-`Result` enum with no `From` into `Result`; in TypeScript it is a branded type whose only exported consumers are those three functions, plus a lint rule forbidding `as any` on it.
3. **`validatePlan()` before any signature request or external submission**, adopting the supplied proposal's ordering. The SDK refuses to issue a `SignRequest/1` for a plan that has not been validated against the current envelope, grant chain, adapter version and evidence policy.
4. **`explain(outcome)` names the rejecting judgment** — which record, which field, which policy, which of the four judgments (contract property / intent refinement / state-effect transition / compliant history). "Rejected" without an attribution is a bug.
5. **`whatCouldStillHappen(outcome)`** enumerates open races: a late destination delivery, a resting order that may fill, an enqueued action not yet executed, a refund that may collide with delivery. Integrators build UIs from this, which is how "a timeout is not non-execution" survives contact with product design.
6. **Four distinct authoring verdicts preserved**: `unknown`, `unsupported`, `timeout`, `inconsistent`, per the consolidated design's Aeon adoption note. Collapsing them into "failed" is forbidden.
7. **`unenforcedFields` is surfaced in the signing display**, not just the JSON. The user should be able to see which of their constraints a verifier actually enforces.
8. **Conformance suite = the hostile traces below**, shipped as fixtures so an integrator's CI fails when their handling of `Unknown` regresses.

Sketch:

```ts
const plan = await kernel.quote(envelope);           // Plan, no authority consumed
const check = await kernel.validatePlan(plan);       // Rejected | Validated  (required)
const run   = await kernel.dispatch(check.validated); // Outcome, never a boolean

switch (run.variant) {
  case "Settled":         return run.netEffects;
  case "CommittedPrefix": return ui.showPartial(run.committedPrefix, run.residualDuties);
  case "Pending":         return ui.watch(run.recoveryHandle, kernel.whatCouldStillHappen(run));
  case "Unknown":         return ui.mustResolve(run.recoveryHandle, kernel.explain(run),
                                                kernel.whatCouldStillHappen(run));
  case "Refused":         return ui.serviceDeclined(run, { programStillValid: true });
  case "Rejected":        return ui.rejected(kernel.explain(run));
}
```

There is no default branch, and `Unknown` has no path that yields a value.

## 9. Three positive traces

### P1 — direct Midnight program, kernel absent

The developer compiles and submits a bounded program with a signed envelope, no kernel involved. The kernel is polled and returns `{"record":"Outcome/1","variant":"Refused","reason":"service-unavailable","programStillValid":true}`. The program settles on Midnight normally.

**Establishes:** the kernel is optional and service refusal is not program invalidity — the property the consolidated design insists on and the one an ergonomic SDK is most likely to erode.

### P2 — sponsored two-leg swap with a native venue call

`quote` → `validatePlan` → `Grant/1` (as in §3.1, `occurrences: 2`) → leg A is a `hypercore-order@1.4.1` limit order, leg B is the bridge withdrawal.

Leg A produces two receipts, not one:

```json
{ "record":"OperationReceipt/1","legId":"leg:order","attemptId":"att:1","phase":"enqueued",
  "observedEffects":[], "evidence":"EvidenceClaim/1:queue-event", "notEstablished":["executed","filled"] }
{ "record":"OperationReceipt/1","legId":"leg:order","attemptId":"att:1","phase":"executed",
  "observedEffects":[{"kind":"partial-fill","filled":"60","resting":"40"}],
  "residualDuties":[{"duty":"open-order","quantity":"40"}] }
```

The outcome is `CommittedPrefix`, not `Settled`, because 40 units rest as an open duty. Fees actually charged (`3.900000` against a `4.500000` user ceiling) enter the net-outcome check; the sponsor's `2.000000` venue cost is reimbursed only on `delivered-and-reconciled`.

**Establishes:** same-domain calls have asynchronous phases; a resting order is a duty, not a completed trade; and the actual fee — not the quote — is what the net check uses.

### P3 — scoped recovery after ordinary authority expiry

Ordinary authority expired at `1790086400` with `leg:withdraw-usdc` unresolved. The recovery grant (`narrowsFrom` the same envelope, `permittedCallClasses: [arb-usdc-bridge@3.2.0/refund]`, own termination rule) authorizes a refund. The refund requires a `light-client` non-delivery premise; a bare timeout is refused. On refund:

```json
{ "record":"Outcome/1","variant":"Settled","viaRecovery":"rec:0x88fa…",
  "consumedAuthority":{"grossSpent":"0.000000","feesCharged":"1.250000"},
  "retainedLiabilities":[{"party":"user","amount":"1.250000","reason":"recovery-cost"}],
  "refundReplenishedGrossCap":false }
```

**Establishes:** recovery authority can outlive ordinary authority; recovery is costed; a refund does not replenish gross authority; and revocation/expiry did not erase the duty.

## 10. Five hostile traces

Each names the rejecting judgment and the retained state. None ends in an untyped failure.

### H1 — key rotation with pending duties, plus nonce-store pruning

The federation rotates `keyEpoch 7 → 8` while `leg:withdraw-usdc` is `submitted` under epoch 7. Two attacks follow. First, a retry is requested under epoch 8 against the epoch-7 grant. Second — the Hyperliquid pattern — the retired signer's nonce state is pruned and its address reused, and an old `att:2` payload is re-presented.

```json
{ "record":"Outcome/1","variant":"Rejected",
  "judgment":"authority/key-epoch-mismatch",
  "detail":"Grant grant:01JB8Q… binds grantee.keyEpoch=7; SignRequest presents keyEpoch=8. Epoch change does not inherit pending duties.",
  "required":"re-authorization under a narrowing grant naming keyEpoch=8, scoped to leg:withdraw-usdc",
  "retained":{"legId":"leg:withdraw-usdc","phase":"submitted","variant":"Unknown","recoveryHandle":"rec:0x88fa…"} }
```

```json
{ "record":"Outcome/1","variant":"Rejected",
  "judgment":"history/logical-request-already-consumed",
  "detail":"lrq:sha256:c0de… has consumptionRecord cons:0x77…. Signer-side nonce state is not the system of record; pruning it does not release Moriarty authority.",
  "retained":{"consumedAuthority":"unchanged","duties":"unchanged"} }
```

**Why it holds:** consumption is durable in Moriarty state and epochs do not inherit duties. **S4 Fact:** RFC 9591 puts key generation out of scope and does not address rotation — so this rejection comes from Moriarty's policy, not from the signing protocol, which is precisely why it must be written down.

### H2 — replay across a reused grantee address

A previously valid, fully-formed `SignRequest/1` is replayed verbatim after the original leg settled, from a grantee address that has been re-registered.

```json
{ "record":"Outcome/1","variant":"Rejected",
  "judgment":"history/logical-request-already-consumed",
  "detail":"oneShot rule with logicalRequestScope=per-leg; leg:withdraw-usdc terminal at Settled; occurrencesUsed=1 of 1 remaining after leg:order.",
  "alsoWouldHaveFailed":["authority/grant-expired"],
  "retained":{"variant":"Settled","noNewEffects":true} }
```

**S2 Fact → Inference:** EIP-712 explicitly leaves replay detection out of scope. Any design that assumes a domain separator prevents replay has misread the standard; the separator prevents *cross-domain* confusion (H3), not *repetition*. Replay is defeated only by durable consumption of a logical request identity.

### H3 — wrong-domain signing

An identical payload struct is presented for a different chain, a different verifying locus, and — the subtler one — the same chain under a different numeric profile.

```json
{ "record":"Outcome/1","variant":"Rejected",
  "judgment":"intent/domain-separator-mismatch",
  "detail":"Grant moriartyDomain binds {programId prog:sha256:9f2c…, semanticProfile mil/2+numeric/u0, ledger midnight:preview, verifyingLocus ledger-primitive:intent-auth/1}. Presented request resolves to eip155:1 / numeric/u0-draft. Three of four domain fields differ.",
  "subCase":"same struct, different semanticProfile → different financial instruction (price orientation and rounding direction differ)",
  "retained":{"noSignature":true,"noAuthorityConsumed":true} }
```

**Why the profile is in the separator (Inference):** the consolidated design records that DeFiFormal prices are quote-per-base while Moriarty's convention is base-per-quote, and that adaptation requires a dimensioned conversion rather than a rename. A struct that is byte-identical under two numeric profiles is two different instructions, so the profile belongs in the domain separator alongside S2's `chainId` and `verifyingContract` analogues.

### H4 — unverifiable oracle evidence

A settlement transition requires a price observation. Three presentations are attempted: an operator report labelled `verified`; a `threshold-attestation` from an issuer not trusted for this statement; and a `native-declaration` — the Aeon pattern.

```json
{ "record":"Outcome/1","variant":"Rejected",
  "judgment":"evidence/premise-below-policy",
  "presented":[
    {"issuer":"issuer:kernel-ops","verifierPremise":"operator-report","claimedLabel":"verified",
     "verificationVerdict":"authentic-and-current","validationVerdict":"refused:premise-not-in-evpol:bridge-strict/2"},
    {"issuer":"issuer:unlisted-quorum@1","verifierPremise":"threshold-attestation",
     "verificationVerdict":"authentic-and-current","validationVerdict":"refused:issuer-not-trusted-for-statement-kind:price"},
    {"issuer":null,"verifierPremise":"native-declaration",
     "verificationVerdict":"not-applicable","validationVerdict":"refused:never-acceptance-bearing"} ],
  "detail":"No downgrade path exists from the policy premise to any presented premise.",
  "retained":{"variant":"Unknown","unresolvedLegs":["leg:settle"],"lateResultRace":"a valid later round may still arrive"} }
```

**S5 Fact → Inference:** the first case is the standard error and the one the spec names directly — the attestation is authentic and current, and that establishes nothing about the truth of the price. Splitting `verificationVerdict` from `validationVerdict` is what makes "authentic but not usable" a first-class, loggable state instead of a judgement call in an operator's code. The third case is the Aeon counterexample promoted to a runtime rule: a type-level or host-level declaration is never acceptance-bearing.

### H5 — fee increase

Two variants. (a) Between quote and dispatch, the venue fee rises from `1.400000` to `2.600000`, taking the total past the `2.000000` venue ceiling. (b) The operation reverts after the sponsor has paid.

```json
{ "record":"Outcome/1","variant":"Rejected",
  "judgment":"intent/fee-ceiling-exceeded",
  "detail":"venue fee 2.600000 > ceilings.venue 2.000000; repriceRule=reject-above-ceiling; isAdjustmentAllowed=false.",
  "typedSignal":"RepriceRequired",
  "required":"a new narrowing grant or root re-authorization naming the higher ceiling; the kernel MUST NOT re-sign at the higher fee",
  "retained":{"noDispatch":true,"noAuthorityConsumed":true,"quoteExpired":true} }
```

```json
{ "record":"Outcome/1","variant":"CommittedPrefix",
  "legs":[{"legId":"leg:withdraw-usdc","phase":"executed","result":"reverted"}],
  "feeAccounting":{"venuePaidBySponsor":"1.400000","userCharged":"0.000000",
                   "reason":"postOp mode opReverted: sponsor pays; reimbursement condition delivered-and-reconciled unmet"},
  "residualDuties":[{"duty":"reconcile","before":1790092800}] }
```

**S3 Fact → Inference:** `validatePaymasterUserOp` receives a `maxCost` and `postOp` can run as `opReverted` with the paymaster still paying. So (i) a sponsorship bound must be a hard bound checked before dispatch, and (ii) failure is a *funded* event whose cost lands somewhere by design rather than by accident. The Hyperliquid builder-code separation is the other half: a user-approved maximum is the ceiling, and the later per-order fee must respect it — an operator's quote is never authority to exceed it.

## 11. What the Aeon signing experiment establishes, and its trust limits

**Establishes (Fact, from the supplied `signing/RESULT.md`):** with `alcides/aeon` at pinned commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`, `strict_decidable=True`, zero synthesis budget, exit code `0` — an `authorize` function whose refinement fixes domain `1`, path `7`, digest `314159`, recipient `42`, value in `1..100`, epoch `5` and nonce `9` rejected all six single-field substitutions (domain, path, digest, recipient, epoch, nonce) with `LiquidTypeCheckingFailedRelation`; rejected amount `101`, amount `0`, and an *unconstrained dynamic* amount; and rejected all three phase substitutions (signed-as-broadcast, signed-as-finalized, broadcast-as-finalized) via distinct refined phase tags.

**What that supports (Inference):** an authoring-time refinement checker can diagnose mismatched signing parameters and lifecycle-phase confusion *in the checked source*, before any key is touched. That is real value for the SDK: the §3.2 `SignRequest/1` field set is exactly what such a checker can hold, and the unconstrained-dynamic-amount rejection is the useful one — it catches the integrator who forwards a user-supplied amount without a bound, which is the most common real-world version of this bug.

**Trust limits (Fact, from the same file):** the experiment's own counterexample is decisive — a `native "1"` declared as a finalized token was **accepted**, with `compute_trust_report` identifying `observed` as `native`. It did not test canonical serialization, hash collision resistance, MPC threshold behaviour, signer equivocation, cross-chain nonce semantics, replay protection, actual broadcast, foreign-chain finality, or the Moriarty compiler/ledger path. No transaction was submitted. The identifiers and digest are small `Int` tags, so nothing establishes that an adapter's bytes correspond to those tags.

**Consequences for this design (Inference):**

- The experiment validates the *shape* of the signing authority check and validates nothing about its *enforcement*. It is therefore evidence for the SDK and authoring layer, and no evidence at all for acceptance.
- The `native` acceptance is why `native-declaration` sits at the bottom of the §3.3 premise ladder as never acceptance-bearing, and why H4's third case is a rejection rather than a warning.
- The phase-tag results are why `SignAuthorization/1` carries `notEstablished: ["broadcast","inclusion","finality","delivery"]` in a *successful* response: type-level phase separation demonstrably works, so the wire format should not undo it.
- **The gap to close next** is exactly the one the supplied result names: an exact-byte adapter fixture checked against a signed policy, then independently observed chain effects, then a negative wrong-byte request. Until that exists, `payloadDigest` matching is a design commitment, not a demonstrated property. `byteFixtureDigest` on `AdapterRegistration/1` is where that future evidence attaches.

## 12. Recommendation

Adopt **Model C — envelope-rooted narrowing grants** with the two-key rule, and treat these four as non-negotiable:

1. `unenforcedFields` on every authority acknowledgement — the API must name what no verifier checks.
2. Separate `verificationVerdict` and `validationVerdict` on all evidence, with no downgrade path.
3. Durable `logicalRequestId` consumption owned by Moriarty, never inherited from a signer's nonce store.
4. An `Unknown` outcome variant that no SDK affordance can collapse into a value.

**Smallest first slice, as an acceptance target and not a claim:** one direct Midnight leg plus one exact-byte external adapter against a simulated foreign domain; one `Grant/1` with a narrowing child; one `SignRequest/1` whose `payloadDigest` is recomputed through the pinned codec and compared byte-for-byte against the fixture; the six hostile presentations from §10 as negative fixtures; and one deliberately unresolved leg carried to a reconciled `Unknown`. Per the supplied proposal this belongs in U5 and must be audited separately from the semantic candidate, because a passing language proof cannot establish adapter correctness.

This is one architect's recommendation. I have not read the other eight answers and assert no agreement with them.

## 13. Dissent and open decisions

**My dissent from the supplied material:**

- The NEAR-based proposal's eleven-state phase list (`planned … unknown`) is useful operationally but should not reach acceptance unreduced. I would freeze a five-state acceptance subset and treat the rest as kernel telemetry. The proposal already hedges this ("acceptance must use a smaller typed subset only after its exact meaning is specified"); I am asking for the reduction to be a gate, not a note.
- I would go further than the supplied texts on one point: a kernel-side constraint that no verifier enforces should be **structurally distinguishable** in the record, not merely documented. Hence `unenforcedFields`. A well-typed record is the most effective way to imply enforcement that does not exist.

**Open decisions I cannot settle and will not paper over:**

1. **Where a grant is actually enforced** — in-circuit, in a bound ledger primitive, or kernel-side only. Kernel-only enforcement is not acceptance, and until this is decided every `Grant/1` field is potentially an `unenforcedFields` entry. This is the single decision that determines whether Model C is real or decorative.
2. **Custody and quorum ownership** — who holds signing shares, at what threshold, and who publishes `bypassDisclosure`. Cannot be inferred from NEAR, Hyperliquid or RFC 9591.
3. **Revocation substrate** — on-ledger registry (latency, cost, privacy leakage) versus issuer attestation (freshness, availability). S5 gives `credentialStatus` as a hook but not an answer.
4. **Rotation policy** is genuinely absent from RFC 9591. My epoch design in §5 is a proposal with no standards backing, and it is the part of this memo I would most want a cryptographer to attack.
5. **ERC-7715 is a Draft whose method names have already changed.** I borrow its shape (`context`, revocation by handle, grant enumeration, `isAdjustmentAllowed`) and recommend against any wire-level dependency.
6. **Whether `Unknown` may ever be collapsed.** I say never. The counter-argument deserves recording: integrators will route around a type they cannot unwrap, either by wrapping the SDK or by treating `park()` as a default, and a widely-bypassed safety type is worse than an honest timeout with a loud warning. I still hold the stricter line, because the supplied NEAR and Hyperliquid analyses both identify the universal-`success` receipt as the specific failure this prevents — but the ergonomics risk is real and should be usability-tested rather than assumed away.
7. **Whether fee sponsorship belongs in the first slice at all.** It adds a third fee population and an operator liability surface before the byte-level adapter evidence exists. I included it because it was in scope; I would sequence it after the §12 slice.

## 14. Citations

**Primary sources (retrieved 2026-09-29):**

1. ERC-7715, *Request Permissions from Wallets* — Draft, created 2024-05-24 — https://eips.ethereum.org/EIPS/eip-7715 (text also at https://raw.githubusercontent.com/ethereum/ERCs/master/ERCS/erc-7715.md)
2. EIP-712, *Typed structured data hashing and signing* — https://eips.ethereum.org/EIPS/eip-712
3. ERC-4337, *Account Abstraction Using Alt Mempool* — https://eips.ethereum.org/EIPS/eip-4337
4. RFC 9591, *The Flexible Round-Optimized Schnorr Threshold (FROST) Protocol* — https://www.rfc-editor.org/rfc/rfc9591.html
5. *Verifiable Credentials Data Model v2.0* — W3C Recommendation, 15 May 2025 — https://www.w3.org/TR/vc-data-model-2.0/

**Supplied local inputs:** `deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`; `deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`; `deliverables/aeon-kernel-experiments-2026-09-29/signing/RESULT.md`; `docs/MORIARTY-CONSOLIDATED-DESIGN.md`. NEAR and Hyperliquid documentation URLs are cited as they appear in those two deliverables, whose own source ledgers record captures and digests; I did not re-retrieve them. Aeon facts are attributed to the supplied result at pinned commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`.

**Limits of this memo:** design research only. No product code was inspected for modification, no build or transaction was performed, no implementation or acceptance is claimed, and no consensus with the other architects is asserted or implied.

---

Two notes on execution, outside the memo:

- **Not written to disk.** `Write` and `ExitPlanMode` are both disabled here and plan mode bars other write paths, so nothing was created in `deliverables/kernel-api-architects-2026-09-29/`. The `opus-trust` slot remains empty on disk.
- **Startup was completed as specified**, and the `trust`/`opus` slot's prior failure is recorded in-repo: `opus-trust.receipt.attempt1.json` has `exit_code: 1` with stderr `Error: Input must be provided either through stdin or as a prompt argument when using --print`, and `opus-trust.raw.attempt1.json` is 0 bytes. I read only `PROMPT-TRUST.md` from that directory; `grok-*.md` and `sol-*.md` were not opened.