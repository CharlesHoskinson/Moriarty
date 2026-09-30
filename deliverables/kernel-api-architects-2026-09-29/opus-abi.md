Both `Write` and `ExitPlanMode` are disabled in this session, so I can't create `opus-abi.md` or exit plan mode. Delivering the memo inline — it is complete and self-contained, ready to be saved to `deliverables/kernel-api-architects-2026-09-29/opus-abi.md`.

---

# Architect memo — API/ABI and interface semantics for the Moriarty DeFi Kernel

**Seat:** ABI / interface semantics (one of nine independent architects).
**Author model:** `claude-opus-5`. **Date:** 2026-09-29.
**Status:** advisory research. This is **not** an adopted MIL/4 profile, a design vote, an implementation, a proof result, or a Midnight ledger result. **No consensus is claimed**; this memo records one architect's independent proposal and should be read against the other seats' answers, none of which were inspected.

## 0. Startup and guarded status

Per `AGENTS.md` and `CLAUDE.md`, the checked-in develop skill (`plugins/moriarty-dev/skills/develop/SKILL.md`) was applied — the host did not expose `moriarty-dev:develop` — and the guarded CLI was run read-only from the worktree root:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

```json
{
  "capability": "SP01.6 loan-swap-subset",
  "blockedAction": "implementation/repair of loan-swap-subset",
  "reason": "operational history is unresolved or unverified; ...",
  "nextAction": "sp01-loan-report",
  "missingEvidence": ["binding-input-stale:...", "candidate-input-stale:...",
                      "current-accounting-missing:.moriarty-dev/runtime/current-accounting.json",
                      "resource-live-state-unavailable:sp01-loan-swap-grok-01",
                      "operational-history"],
  "pendingTransactions": []
}
```

No pending Midnight transactions, so no public notification line is due. This memo is read-only research; it does not occupy the blocked delivery slot, request a campaign dispatch, or alter the SP01.6 stop.

**Not done:** no compile, no proof, no test run, no Preview transaction, no Aeon re-run, no product-code edit, no inspection of any other architect's answer (`sol-*.md`, `grok-*.md`, `*.raw.json` in the same directory were left unopened).

## 1. Sources

### 1.1 Assigned local files (read in full)

- `deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md`
- `deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md`
- `deliverables/aeon-kernel-experiments-2026-09-29/authority/REPORT.md` (plus its `CLI-RESULTS.md`, cited for exact exit codes)
- `docs/MORIARTY-CONSOLIDATED-DESIGN.md`
- Consulted for interface obligations already on the register: `deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md` (M4-C1 … M4-C5).

### 1.2 External primary sources (five, retrieved 2026-09-29)

| # | Source | Exact URL | Status as published |
|---|---|---|---|
| S1 | ERC-7683, *Cross Chain Intents* | https://eips.ethereum.org/EIPS/eip-7683 (raw: `https://raw.githubusercontent.com/ethereum/ERCs/master/ERCS/erc-7683.md`) | **Draft**, Standards Track: ERC, `requires: 7930` |
| S2 | EIP-712, *Typed structured data hashing and signing* | https://eips.ethereum.org/EIPS/eip-712 | **Final**, Standards Track: Interface |
| S3 | EIP-5792, *Wallet Call API* | https://eips.ethereum.org/EIPS/eip-5792 | **Final**, Standards Track: Interface |
| S4 | CAIP-19, *Asset Type and Asset ID Specification* | https://chainagnostic.org/CAIPs/caip-19 (canonical: https://standards.chainagnostic.org/CAIPs/caip-19) | **Review** |
| S5 | RFC 8949, *Concise Binary Object Representation (CBOR)*, §3.4, §4.2 | https://www.rfc-editor.org/rfc/rfc8949.html | **Internet Standard** (STD 94) |

Throughout, **[F]** marks a direct fact taken from a named source and **[I]** marks my inference or design proposal. A design proposal is never evidence.

### 1.3 The direct facts I rely on

**S1 — ERC-7683 (Draft).** [F] The current text is resolver-centric, not struct-centric: "A protocol exposes orders as opaque payloads and provides a *resolver* contract that translates those payloads into a common order representation." [F] The resolved form is

```solidity
struct ResolvedOrder {
    bytes[] steps;        // IStep ABI calldata
    bytes[] variables;    // IVariableRole ABI calldata
    bytes[] payments;     // IPayment ABI calldata
    Assumption[] assumptions;
}
function resolve(bytes calldata payload) external view returns (ResolvedOrder memory);
```

[F] Steps are `Call(target, selector, arguments, attributes)` with attributes `SpendsERC20`, `SpendsGas`, `TimingBounds`, `RevertPolicy`, `NeedsStep`, `NeedsVariable`. [F] "Hard dependencies MUST be acyclic. Fulfillment MUST proceed in hard-dependency order." [F] "Values assigned to variables are untyped and represented only by their ABI encoding and whether they are statically or dynamically sized." [F] Framed ABI encoding is `abi.encode("", value)`. [F] Conditions the resolver cannot check are surfaced as `Assumption { string name; bytes data; }` and "A solver MUST validate the assumption (e.g., checking against a whitelist) before fulfilling the order." [F] "Publishing the resolver onchain is useful because the resolver is the point of trust." [F] Resolution happens off-chain via `eth_call`, deliberately outside gas constraints. [F] `SpendsERC20`'s `amountFormula` is an upper bound, and the spec notes the call "does not have a hard dependency on the variables in `amountFormula`" and that time-dependent formulas SHOULD decrease with time so a solver can bound them.

**S2 — EIP-712 (Final).** [F] `EIP712Domain` fields are `string name`, `string version`, `uint256 chainId`, `address verifyingContract`, `bytes32 salt`. [F] `typeHash = keccak256(encodeType(typeOf(s)))`; `hashStruct(s) = keccak256(typeHash ‖ encodeData(s))`; the signed preimage is `"\x19\x01" ‖ domainSeparator ‖ hashStruct(message)`. [F] "The domain separator prevents collision of otherwise identical structures… By introducing a domain separator the DApp developers are guaranteed that there can be no signature collision." [F] And, verbatim: "It does not include replay protection."

**S3 — EIP-5792 (Final).** [F] Capabilities are shaped `Record<chainId, Record<capabilityName, capabilityObject>>`, and the spec says "Capabilities are nested in per-chain objects because wallets may support different capabilities across multiple chains." [F] "MUST reject the request if it contains a `capability` … that is not supported by the wallet and the `capability` is not explicitly marked as optional." [F] `atomicRequired: boolean` in the request; the `atomic` capability has one property `status: 'supported' | 'ready' | 'unsupported'`, and "This capability is expressed separately on each chain and should be interpreted as a guarantee only for batches of transactions on that chain." [F] Batch identifiers: "Identifiers, whether provided by the app or generated by the wallet, MUST be a unique string up to 4096 bytes." [F] Status codes: 100 pending, 200 confirmed, 400 offchain failure with no retry, 500 "reverted **completely** and only changes related to gas charge may have been included onchain", 600 "reverted **partially** and some changes related to batch calls may have been included onchain".

**S4 — CAIP-19 (Review).** [F] Grammar:

```
asset_type:        chain_id + "/" + asset_namespace + ":" + asset_reference
chain_id:          Namespace+Blockchain ID as per CAIP-2
asset_namespace:   [-a-z0-9]{3,8}
asset_reference:   [-.%a-zA-Z0-9]{1,128}
asset_id:          asset_type + "/" + token_id
token_id:          [-.%a-zA-Z0-9]{1,78}
```

[F] Examples: `eip155:1/slip44:60`, `eip155:1/erc20:0x6b175474e89094c44da98b954eedeac495271d0f`, `eip155:1/erc721:0x06012c8cf97BEaD5deAe237070F9587f8E7A266d/771769`.

**S5 — RFC 8949 §4.2 (STD 94).** [F] §4.2.1: "Preferred serialization MUST be used"; "Indefinite-length items MUST NOT appear"; "The keys in every map MUST be sorted in the bytewise lexicographic order of their deterministic encodings." [F] §4.2.2: for tags, "This protocol's deterministic encoding needs either to require that the tag is present or to require that it is absent, not allow either one." [F] §4.2.3 offers length-first map key ordering as an RFC 7049-compatible alternative. [F] §3.4/§5.3.2: a decoder "does not need to understand" every tag number and may hand unknown tags to the application — i.e. tolerance is the *default* posture of a general CBOR decoder, not a property a security protocol gets for free.

## 2. The problem this memo addresses

The NEAR-derived proposal states the service contract as a prose record list and then says plainly that "the exact call schema and accepted-state encoding are **open MIL/4 design work**, not currently implemented" (`PROPOSAL.md` §"A minimal service contract"). The Hyperliquid comparison adds two call classes the NEAR framing under-weights — `native-financial-call` and `same-domain-queued-call` — and requires "an explicit **settlement locus** on each leg". `MORIARTY-CONSOLIDATED-DESIGN.md` fixes the non-negotiables: four separate judgments (contract properties, intent refinement, transition validity, compliant history), complete effects rather than a prover-chosen projection, and "A host JSON field or host-computed verification Boolean is insufficient."

[I] The recurring interface failure across all three is a single one: **one wire format is being asked to serve three audiences with incompatible evolution rules.** A signed authority record must be closed-world and byte-exact forever; a service RPC must evolve weekly; an adapter payload must match a foreign chain's encoding exactly and must never be migrated at all. Any ABI that merges these gives one audience's flexibility to another audience's security predicate. The rest of this memo is built around separating them.

## 3. Recommended contract: `MK/1`

### 3.1 Three planes, three evolution rules

Every message carries a version triple `{ authority, kernel, adapter }`.

| Plane | Version tag | Who verifies it | Encoding | Evolution rule |
|---|---|---|---|---|
| **Authority** | `authority/1` | Moriarty / Midnight verifier (the `authority_p` and `intent_p` judgments) | Deterministic CBOR, RFC 8949 §4.2.1 **[F: S5]** | Closed world. **Unknown tag or unknown map key ⇒ reject.** `/3 → /4` only through an explicit migration function producing one canonical digest or a deterministic reject. |
| **Kernel** | `kernel/1` | Nobody, normatively | JSON or CBOR, unconstrained | Additive; unknown fields tolerated. **No kernel-plane field may appear in any settlement predicate.** |
| **Adapter** | `adapter/<id>/<ver>` | The foreign/native domain, by its own rules | Whatever that domain requires, byte-exact | Pin, never migrate. New bytes ⇒ new adapter version ⇒ **re-authorization**. |

[I] The reject-on-unknown rule is the load-bearing decision, and RFC 8949 is the reason it must be stated rather than assumed: a conforming CBOR decoder is explicitly permitted to pass unknown tags through **[F: S5 §3.4]**, and §4.2.2 requires a deterministic profile to declare each tag required-or-prohibited **[F: S5]**. Silence here yields H4 below.

### 3.2 Exact payload bytes and the canonical digest

[I] Authority-plane canonicalization, stated so it can be implemented from this memo:

- Deterministic CBOR per §4.2.1: preferred serialization, no indefinite-length items, bytewise-sorted map keys **[F: S5]**. Length-first ordering (§4.2.3) is **rejected** — one ordering only, no profile switch.
- Integer map keys from a frozen registry, not text keys. Text keys re-introduce a rename surface that M4-C1 must otherwise adjudicate.
- One tag number per authority-plane constructor; every tag is *required* (never "optional tag"), satisfying §4.2.2 **[F: S5]**.
- Domain separation generalized from EIP-712 **[F: S2]**, with Moriarty's own prefix so no Moriarty preimage is ever a valid Ethereum `\x19\x01` preimage:

```
MoriartyDomain = {
  1: name,               // "moriarty"
  2: langVersion,        // e.g. "MIL/4"
  3: authorityVersion,   // e.g. 1
  4: settlementDomain,   // DomainId (§3.4)
  5: verifyingLocus,     // circuit | ledger-primitive | named-external-verifier
  6: salt                // 32 bytes
}
domSep  = H(det_cbor(MoriartyDomain))
preimg  = 0x4D ‖ 0x01 ‖ domSep ‖ H(det_cbor(Envelope))
digest  = H(preimg)
```

- `verifyingLocus` is in the separator because `MORIARTY-CONSOLIDATED-DESIGN.md` requires the enforcement map to "state whether signed-intent authentication occurs in the circuit, a bound ledger primitive or another explicitly justified native boundary." Moving that check without re-signing must invalidate the signature, not silently succeed.
- **Consumption is a language judgment, not a signer's bookkeeping.** EIP-712 states "It does not include replay protection" **[F: S2]**, and Hyperliquid's docs warn that pruning a deregistered agent's nonce state can make old actions replayable if the address is reused (`COMPARISON.md`). [I] Therefore the nullifier / logical-request ID lives in `authority_p`, and an external signer's nonce store is never consulted for it.

### 3.3 Record set

```
Capability = {
  capId, adapterId, adapterVersion, class, domain,
  actionClasses[], assetScope[], evidenceProfile, trustProfile,
  settlementLocus, quotedFeeSchedule, optional: bool
}

Plan = { planId, envelopeDigest, legs[Leg], edges[LegEdge], deadline, quoteRef }
Leg  = { legId, capId, adapterVersion, callShape, expectedEffects[], expectedEvidence,
         settlementLocus, recoveryEdge? }
LegEdge = { from, to }                      // acyclic hard dependency

CallShape = {                                // the authorized shape, not the bytes
  domain, target, actionClass, asset, recipient,
  valueMax, feeMax, nonceBinding, derivationPath?, keyEpoch?, timingBounds?
}

SignRequest = { legId, attemptId, domain, scheme, keyPolicyId, keyEpoch,
                derivationPath, payloadDigest, callShapeWitness, validityWindow }

Dispatch = { legId, attemptId, adapterVersion, payloadBytes, payloadDigest,
             callShapeWitness, submittedAt }

Receipt = { legId, attemptId, phase, settlementLocus,
            domainTxId?, observedEffects[], actualFee, evidenceRef?,
            finality, observedAt }

Evidence = { evidenceId, class, issuer, verifierPremise, publicInputs,
             freshness, finality, uniqueness, downgradePolicy: "prohibited" }

Continuation = { committedPrefix[], pendingLegs[], unknownLegs[],
                 consumedAuthority, cumulativeExposure,
                 remainingEntitlement, residualDuties[] }
```

[I] Two rules give this set its teeth:

1. **`CallShape` must be re-derivable from `payloadBytes` by the pinned adapter codec.** `callShapeWitness` is the codec's decode proof-obligation, not a restatement. A commitment to `payloadDigest` alone is explicitly insufficient — see H2.
2. **`legId` and `attemptId` are separate and both mandatory.** EIP-5792 gets one identifier per batch, "MUST be a unique string up to 4096 bytes" **[F: S3]**; that is a submission identity. Moriarty additionally needs a *logical effect* identity that survives retry, which the NEAR proposal already demands ("A retry reuses the logical effect identity and first reconciles all known attempts").

### 3.4 Domain-qualified assets

[I] Identity is structural, display is textual:

```
DomainId        = { 1: namespace, 2: reference }          // CAIP-2 shape
AssetId         = { 1: DomainId, 2: nsCode:uint, 3: reference:bstr, 4: tokenId:bstr? }
```

`nsCode` indexes a frozen integer registry (`slip44`, `erc20`, `erc721`, `midnight-native`, …) rather than CAIP-19's `[-a-z0-9]{3,8}` text namespace **[F: S4]**. The CAIP-19 string (`eip155:1/erc20:0x6b17…`) is retained as the **display and interop form only** and is never the hashed identity. [I] Reason: CAIP-19's `asset_reference` is a case-sensitive character class, so `0x6B17…` and `0x6b17…` are distinct strings for the same ERC-20; hashing the display string would make checksum casing a consensus-relevant fact. This binds directly to M4-C1's requirement to "name both nominal assets and domains, and reject implicit reciprocals": `Price<Base, Quote, Scale>` takes two `AssetId`s, and `Base.domain ≠ Quote.domain` is a legal, explicitly-encoded cross-domain price rather than an accident.

### 3.5 Capability classes and the null-kernel invariant

Six classes, taken from the Hyperliquid comparison's adapter table and the NEAR capability matrix, typed here:

| Class | Settlement locus | Terminal evidence required before a dependent transition |
|---|---|---|
| `direct-midnight` | Midnight ledger | Midnight acceptance only. **Requires no `Capability` object at all.** |
| `native-financial-call` | Foreign venue ledger | Authenticated venue effect; a resting order is an open duty, not a completed trade |
| `same-domain-queued-call` | Same-chain system contract | Queue event **and** later execution result; enqueue alone completes nothing |
| `foreign-chain-call` | Bridge destination | Source acceptance + signer/relayer steps + destination evidence under the signed trust policy |
| `signer-call` | none (no settlement) | Signature over exact authorized bytes; **signing is not execution** |
| `observation-call` | none (no settlement) | Selected source, round, finality, verifier premise |

**Null-kernel invariant [I]:** the `authority/1` envelope for a `direct-midnight` program must be verifiable with **zero** kernel-plane objects present. `capId`, `planId`, `quoteRef` are absent — not null, absent — from the settlement digest. A `kernelBinding` field may optionally be signed when the user wants to bind a specific coordinator, and when present it may only *narrow*, never widen, the authorized set. This is the ABI-level expression of the consolidated design's "Moriarty can also run on Midnight without this federation" and the product contract's permissionlessness clause. P1 below is its test.

Capability negotiation borrows EIP-5792's shape — `Record<domain, Record<capabilityName, capabilityObject>>` and per-chain scoping **[F: S3]** — and its reject-unless-`optional` rule **[F: S3]**, with the default inverted: in `MK/1` a capability is required unless explicitly marked `optional: true`, and an unrecognized capability name is always a rejection.

### 3.6 Same-domain queued vs foreign-chain calls

[I] These differ in **who can witness phase two**, and that is the distinction the ABI must encode, not "local vs remote":

- **`same-domain-queued-call`:** the queue event and the execution result are both in one domain's authenticated state. Hyperliquid's CoreWriter emits a versioned action log with actions "appearing first as enqueued, then executed" (`COMPARISON.md`, citing the HyperEVM interaction-timings page). The receipt automaton is `planned → dispatched → enqueued → executed | rejected`, and both phases are in principle reachable by that domain's own verifier. [I] This is the class that ledger induction can eventually cover.
- **`foreign-chain-call`:** phase two is in a different authentication domain. The automaton is `planned → signed → submitted → included → finalized → delivered | refunding | refunded | failed | unknown`, and the step into `delivered` requires an `Evidence` import with a named `verifierPremise`. `MK/1` therefore carries `remainingEntitlement` in `Continuation` and refuses to collapse partial delivery into either "no receipt" or "completed" — exactly the gap `DESIGN-MIL4-WORKING.md` names for `bridge-pair/1` ("A partial 99-unit receipt needs a state distinct from both no receipt and completed delivery"). Absent a qualified foreign verifier this class stays **specified-only**.

EIP-5792's code 600 — partial revert, "some changes related to batch calls may have been included onchain" **[F: S3]** — is the concrete precedent that a production wallet API found a boolean insufficient. [I] `MK/1` goes further: `unknown` is a first-class returnable phase, because 400/500/600 all assert knowledge the kernel may not have.

### 3.7 Phase API

```
capabilities → quote → validatePlan → reserve → authorize
             → dispatch → observe → reconcile → advanceOrRecover
```

Normative rules [I]: `validatePlan` MUST precede any `SignRequest` or external submission; **only `authorize` consumes signed authority**; `dispatch` never widens `CallShape`; `reconcile` MUST run over all known `attemptId`s before a retry; `advanceOrRecover` may return `unknown`; and a kernel refusal at any phase is a service outcome, never program invalidity.

## 4. Alternatives compared

### Design A — resolver / opaque payload (ERC-7683-shaped)

The protocol ships an opaque `payload`; a published resolver contract translates it into a common solver-facing representation off-chain via `eth_call` **[F: S1]**.

*For:* maximal protocol flexibility; shared solver liquidity across protocols; rich semantics unconstrained by gas **[F: S1]**; a genuinely elegant answer to fragmentation. Resolvers are also not inherently EVM-specific — the ERC says so explicitly **[F: S1]**.

*Against, for Moriarty specifically [I]:* the resolved representation is deliberately untyped — "Values assigned to variables are untyped and represented only by their ABI encoding" **[F: S1]** — so Moriarty's `effect_p` (complete canonical lines, conservation, post-state) has nothing to bind at settlement. And the trust locus is wrong for a permissionless language: "the resolver is the point of trust" **[F: S1]**, vetted by "security audits, bounties, and lindiness" **[F: S1]**. That is a whitelist. Making a whitelisted resolver a precondition for authoring or settling a Moriarty program would violate the product contract's clause that no "approved solver or central program registry is required to author, compile, prove or deploy a supported program." Finally, `Assumption { name, data }` is an honest escape hatch **[F: S1]**, but it delegates validation to the solver — the party with the profit motive.

### Design B — closed typed capability registry

Every leg type enumerated in a registered, version-pinned ABI; unknown tags reject; no resolver, no untyped values.

*For [I]:* exact bytes and a decidable canonical digest; every field a settlement predicate reads is typed; direct fit with the six MIL judgments; nothing to whitelist at authoring time.

*Against [I]:* each new venue costs a registered capability version, and if registration is gated, the registry becomes exactly the central gate the product contract forbids; adoption is slow; and pinning per-adapter bytes means the registry grows monotonically and must never garbage-collect a version with live duties.

### Design C — recommended: typed authority envelope, sealed adapter region

Design B governs everything a settlement predicate reads; Design A's flexibility is confined to the adapter payload region, behind a `CallShape` the pinned codec must re-derive from those exact bytes.

*For [I]:* the settlement predicate never reads an untyped value; new venues need a new adapter version, not a language-version bump; no resolver whitelist is a precondition for a `direct-midnight` program.

*Against, stated honestly [I]:* it requires a working, audited codec per adapter, and the entire burden shifts onto adapter qualification — a real cost the memo does not minimize. A codec bug is an authority bug. Design C is only better than B if adapter qualification is genuinely cheaper than registry governance, which is an empirical claim this memo does not establish.

## 5. Discriminating traces

Each trace names the rejecting judgment (`stage_p`/`intent_p`/`effect_p`/`authority_p`/`history_p`/`failure_p` per `DESIGN-MIL4-WORKING.md` §2) and what it separates.

### Positive

**P1 — direct Midnight, zero kernel objects.** The S0 slice: a literal-fee transfer authorized, proved and settled on Midnight with no `Capability`, `Plan`, `Quote` or `Receipt` in existence. Accepts under `stage_p`/`intent_p`/`effect_p`.
*Discriminates:* A requires a deployed, vetted resolver for the order to be consumable at all **[F: S1]**; B and C permit the null-kernel path. Any design that cannot run P1 fails the product contract before any DeFi question arises.

**P2 — same-domain queued call, two receipts, one leg.** A CoreWriter-shaped enqueue followed by execution: `Receipt{legId=L, attemptId=A, phase=enqueued}` then `Receipt{legId=L, attemptId=A, phase=executed}` with distinct observed effects. Any dependent transition attempted between them rejects under `stage_p` for missing the executed-phase effect.
*Discriminates:* a boolean-success receipt cannot express this at all; EIP-5792's 100-then-200 progression **[F: S3]** shows the two-phase shape is already load-bearing in a Final standard. A design with one terminal receipt per leg fails P2.

**P3 — foreign bridge leg with partial delivery.** Source lock 100, verified destination delivery 99, signed fee bound 1. Accepts with `Continuation{ remainingEntitlement: 1, residualDuties: [...] }` and **no** terminal `delivered`. The unexplained unit must be typed as either a charged fee (with a named beneficiary) or undelivered entitlement — `DESIGN-MIL4-WORKING.md` requires exactly this for `bridge-pair/1`.
*Discriminates:* designs whose receipt is `{success: bool, amount}` must round 99 to either success or failure; both are wrong.

### Hostile

**H1 — fee/gross-cap escalation through a solver-decided amount.** The signed envelope caps gross debit at 100 and fee at 5. The plan carries an amount formula whose resolved value is 101.
*Rejecting judgment:* `authority_p` (gross cap), at settlement, against **actual** effects.
*Discriminates:* under A, `amountFormula` is an untyped upper bound evaluated off-chain by the solver, and the ERC explicitly notes the call "does not have a hard dependency on the variables in `amountFormula`" **[F: S1]** — nothing at settlement re-checks it against a user cap. Under B/C the cap is a settlement predicate. Hyperliquid's builder-code model — a user-approved per-builder maximum with the actual fee processed on-chain (`COMPARISON.md`) — is the precedent for separating *approved maximum* from *charged fee*. This is also, precisely, the arithmetic Aeon accepted (§6).

**H2 — adapter payload substitution with a self-consistent digest.** The authorized leg pays recipient R on domain D. The kernel dispatches bytes paying R′, and computes `payloadDigest` over the substituted bytes. Digest and payload agree perfectly.
*Rejecting judgment:* `authority_p`, via `callShapeWitness` — the codec re-derives `recipient = R′ ≠ R`.
*Discriminates:* **this is the single test that separates "commit to the payload digest" from "commit to the authorized shape."** Any design whose signed record contains only a hash of the adapter payload passes a forged dispatch, because the forger controls both sides of the hash. The NEAR proposal already states the requirement in prose — a signer capability "may sign only a payload produced by a pinned adapter, checked against the authorized leg, chain, recipient, value, nonce, derivation path and key epoch" — and NEAR's own chain-specific transaction builder is cited there as the reason a payload cannot be generic bytes. Variants that must also reject: wrong chain ID, reused derivation path, rotated key epoch with pending duties.

**H3 — cross-locus replay.** A valid envelope, correctly signed, is replayed against a different `verifyingLocus` (circuit vs bound ledger primitive) or a second settlement domain.
*Rejecting judgment:* `authority_p` (nullifier already consumed) and, for the locus case, signature invalidity because `verifyingLocus` is inside `MoriartyDomain`.
*Discriminates:* EIP-712's domain separator covers chain and verifying contract **[F: S2]**, but the spec says outright "It does not include replay protection" **[F: S2]**. A design that adopts EIP-712-style separation and *stops there* passes the second submission. A design that also prunes consumption state when an external signer is deregistered reproduces the Hyperliquid agent-nonce hazard verbatim (`COMPARISON.md`).

**H4 — version-downgrade through tolerant decoding.** An `authority/2` envelope adds a `maxSlippage` cap field. It is presented to an `authority/1` decoder that, following ordinary CBOR practice, hands unknown items to the application and proceeds **[F: S5 §3.4]**. The envelope verifies; the cap is silently absent from the checked relation.
*Rejecting judgment:* `intent_p` — the decoder MUST reject the unknown key rather than verify a weaker relation.
*Discriminates:* **the sharpest of the eight.** It separates designs by their *default*, not their intent, and it is invisible in every positive trace. Protobuf-style unknown-field preservation and default CBOR tag tolerance both fail it. RFC 8949 §4.2.2's requirement to declare each tag required-or-prohibited **[F: S5]** is the standard's own acknowledgement that this must be stated explicitly. This trace is why `MK/1` splits the planes: the kernel plane *should* be tolerant, and merging the two planes forces one wrong answer.

**H5 — timeout-driven refund racing a late partial delivery, with a duplicate attempt.** A foreign leg times out. The kernel treats timeout as nonreceipt, issues a full source refund, and retries — while the destination delivers 99 of 100 from the original attempt.
*Rejecting judgment:* `failure_p` — timeout must transition to `unknown`, not to authenticated nonreceipt; the full refund is rejected because `remainingEntitlement` is 1, not 100; the retry is rejected by `reconcile` over all `attemptId`s under one `legId`.
*Discriminates:* designs lacking a typed `unknown`, or lacking `legId`/`attemptId` separation, double-spend here. `MORIARTY-CONSOLIDATED-DESIGN.md` states the rule already — "Timeout alone proves neither nonexecution nor entitlement to refund" — and NEAR's own BTC refund guide (cited in `PROPOSAL.md`) shows a real refund path requiring a nonfinalized deposit, a timelock, a contract action, MPC signing and relayer broadcast, with a non-returned storage deposit. Refund is a costed workflow, not a timer expiry.

**Coverage note [I]:** P1 discriminates A from B/C. P2, P3, H5 discriminate receipt models regardless of plane split. H1 and H2 discriminate typed-shape commitment from digest-only commitment. H4 discriminates decoder default. A design can pass all three positives and H1–H3 while failing H4, which is why H4 belongs in any acceptance suite.

## 6. Aeon: authoring-time value, and why it cannot be the acceptance gate

### What Aeon can check at authoring time

The consolidated design already sets the adoption order — "obligation/trust reports, exact advisory refinement checking with replayed counterexamples, then typed holes and bounded synthesis" — and the authority experiment supports it. At the pinned commit, Aeon **did** reject recipient 43 against the authorized 42, domain 11 against 10, fee 6 against a cap of 5, and gross 97 where principal 94 plus fee 4 requires 98 — each with `LiquidTypeCheckingFailedRelation` (`authority/REPORT.md`). [I] Mapped onto `MK/1`, that is real value in exactly one place: **authoring-time `CallShape` and plan-refinement review**, before anything is signed. An author can encode the envelope constraints as refinements, get a counterexample when a candidate `Plan` widens a recipient or domain, and iterate. It is a good linter for the H1/H2 *shapes*, and its trust reports make premises visible.

### Why the contradictory-refinement anomaly rules it out as a gate

At pinned commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`, `aeon 4.9.0`, Python 3.13.15, fresh CLI processes returned (`authority/CLI-RESULTS.md`):

| Input | Refinement | Exit code | Result |
|---|---|---:|---|
| `minimal_false.ae` | `false` | 0 | **accepted** |
| `minimal_breach.ae` | `v = 101 && v <= 100` | 0 | **accepted** |
| `minimal_cap.ae` | `v <= 100` | 13 | rejected |
| `gross_cap_violation.ae` | `g = principal + fee && g <= maxGross`, gross 101 | 0 | **accepted** |
| `gross_cap_violation_split.ae` | same, split obligations | 13 | rejected |

[I] Three reasons this is disqualifying for an acceptance gate, not merely a bug to work around:

1. **Acceptance carries no information.** A checker that accepts `false` has, on the affected path, lost the property that makes a verdict meaningful. From a contradiction everything follows; a "pass" from such a path is not evidence of anything. An acceptance gate whose positive verdict is uninformative is not a gate.
2. **The specific failure is the specific attack.** `gross_cap_violation` accepting 101 against a cap of 100 is H1, in the concrete. The one check the kernel most needs — gross debit within signed cap — is the one this path waves through. That the *split* encoding rejected it is not a rescue: it shows the outcome depends on how an author happens to phrase the obligation, and the report is right to say that one rejection "is no proof for all candidates or compiler paths."
3. **Even a repaired Aeon would not be a gate.** The `planRecipient` case makes this independently: native value `43` was accepted against recipient 42's refinement, with the trust report naming `planRecipient` as `native` — an annotation is a *premise*, not a verified implementation. And nothing in a type check establishes the things `MK/1` actually depends on: exact-byte authentication of the signed envelope, the codec's re-derivation of `CallShape` from dispatched bytes, or Midnight's verifier accepting the complete effects. A passing Aeon check cannot authorize a `SignRequest`, a dispatch, a refund or a settlement. This is the report's own conclusion and I concur with it.

[I] Disposition for `MK/1`: Aeon is an **advisory authoring aid and counterexample generator** at `validatePlan`-time only, with its verdicts recorded as `Evidence{class: "advisory-static-check"}` that no transition may depend on. The open upstream question — which defect at this pin accepts impossible refinements, and whether a later commit changes it — remains open and is deliberately kept separate from this design conclusion; resolving it would not change the disposition.

## 7. Open decisions I do not dispose

These belong to the recorded two-reviewer design vote (`AGENTS.md`), not to this memo: the verifying locus for signed-intent authentication (M4-C5); whether a foreign fact is established by in-circuit proof or a named quorum, and whether any foreign verifier currently qualifies; the initial custody model and who controls bridge and signing keys; the first external adapter; the `/3 → /4` encoding and migration decision (M4-C1); and the per-primitive rounding and reserve-beneficiary questions (M4-C2) that `CallShape`'s `feeMax` interacts with but does not settle.

## 8. Limits

Everything above is an interface proposal on paper. No Moriarty source, Core, K, native proof or Preview ledger result was produced or consulted as evidence for any claim here. The five external sources establish what those standards say, not that any Moriarty rule works; ERC-7683 in particular is **Draft** and its structure has already changed substantially from the order-struct form that earlier Moriarty research may reference. The Aeon results are quoted from the existing experiment record at its stated pin and were not re-run. Two designs were compared plus the recommendation; the other seats' answers were not read, and nothing here should be presented as agreement among architects.

---

**Two notes on delivery:** the `Write` and `ExitPlanMode` tools are disabled for this session, so I could neither create the plan file nor save the memo to `deliverables/kernel-api-architects-2026-09-29/opus-abi.md`. If you re-enable file writes (or drop plan mode so I can use a Bash heredoc), I'll save it there unchanged — that path matches the seat's naming convention and fills the gap left by the failed `opus-abi` launcher run.