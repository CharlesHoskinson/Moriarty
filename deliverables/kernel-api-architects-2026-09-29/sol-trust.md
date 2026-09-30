# Trust and developer experience contract for an optional DeFi kernel

**Author:** independent GPT-6 Sol architect, trust and developer experience remit. The task routing named GPT-6 Sol; no machine-readable returned model identity was exposed to this agent. **Date:** 2026-09-29. **Status:** independent design recommendation, not an adopted MIL/4 rule or implemented capability. No other architects' outputs were read.

## Recommendation

Expose a **capability bundle** with four independently scoped authorities: user intent, delegated action, fee payment, and evidence appraisal. The optional kernel consumes the bundle to prepare and coordinate calls. A settlement verifier accepts effects only under the signed intent and the relevant verifier policy. No kernel method receives a general power to sign arbitrary bytes, spend unlimited fees, convert an observation into finality, or migrate a pending intent to new semantics.

This fits the existing [NEAR kernel proposal](../near-kernel-scope-2026-09-29/PROPOSAL.md) and [Hyperliquid comparison](../hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md). The former separates verifier, solver discovery, threshold signing, bridges, and recovery. The latter adds native venue calls, delayed same-domain execution, builder fee approval, and signer nonce retirement. These are repository observations. The proposed contract below is an inference from those comparisons and the primary sources cited here; it is not a claim that Moriarty currently enforces it.

### Why four authorities

| Authority | Principal and permitted decision | Required binding | Revocation and history |
| --- | --- | --- | --- |
| `IntentAuthority` | User chooses acceptable financial outcome and failure remedies | Program hash/version, verifying domain, selected profile, recipients, assets, gross debits, net receipts, cumulative duties, nonce and expiry | A new intent may supersede uncommitted work. Committed effects and accrued duties remain in history. |
| `ActionGrant` | User or governing account delegates a constrained call to an operator or signer | Grantee, action class, adapter code/hash, venue/domain, target, payload predicate, resource ceilings, count, valid interval and key epoch | Revoke future use at an authenticated policy head; never erase prior consumption. |
| `FeeGrant` | Payer authorizes a fee payer or sponsor to cover specified costs | Payer, beneficiary, fee asset, fee class, per-leg and lifetime caps, expiry and allowed action set | Separate allowance state and charged-fee ledger; a sponsor's refusal leaves the financial intent pending. |
| `EvidencePolicy` | User or governance selects which verifier output may justify a fact | Source domain, claim type, verifier identity/version, public inputs, finality/freshness rule and permitted downgrade set | Changes require a new signed policy or an explicitly authorized migration. Old receipts retain their original provenance. |

This separation is supported by the Cosmos SDK's distinct [authorization](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-030-authz-module.md) and [feegrant](https://github.com/cosmos/cosmos-sdk/blob/main/x/feegrant/README.md) mechanisms: the latter names granter, grantee, allowance and allowed message types. Hyperliquid requires the main wallet to approve a maximum [builder fee](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes) before orders can carry a fee; its [API wallets and nonces](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets) are separate signing machinery. These precedents motivate the separation; their exact limits are not Moriarty defaults.

## Trust contract

### Canonical authorization and replay

The wallet should display a structured summary of assets, recipients, maximum **gross** debits, minimum **net** receipts, fee payer and cap, allowed venues, expiry, recovery options and evidence trust class before it signs. The encoded envelope must commit to the complete structure, schema version, chain/domain, verifying contract or program identifier, policy head, nonce and operation ID. [EIP-712](https://eips.ethereum.org/EIPS/eip-712) shows why type and domain separation belong in the signed bytes, though Moriarty must select its own canonical encoding and verifier locus. A UI summary is only useful when it is generated from the exact encoded fields that the verifier checks.

Keep two replay records: the settlement domain's transaction nonce and Moriarty's durable `(intent ID, logical leg ID, effect kind)` consumption/nullifier. Hyperliquid documents that a deregistered API wallet's nonce state may be pruned, making reuse of its address hazardous. Its signer nonce therefore cannot serve as Moriarty's lifetime effect identity. A retry gets a new attempt ID and possibly a new chain nonce while preserving the logical leg ID. Reconcile known attempts before a new dispatch. Revocation blocks new signature issuance and use at a stated authenticated head; it does not assert that a previously broadcast foreign transaction disappeared.

### Exact signature authority

`requestSignature` accepts only a `PreparedCall` produced by a pinned adapter and a currently valid `ActionGrant`. The request commits to domain, chain ID, target, action class, adapter/version/hash, derivation path, key scheme and epoch, nonce, validity window, payload bytes digest, recipient, amount and fee bound. The signer checks the digest against the actual bytes it signs. A separate `SignatureReceipt` records signer/quorum and signed digest. It does **not** satisfy a condition requiring submission, inclusion, venue execution or delivery.

NEAR's [Chain Signatures documentation](https://docs.near.org/chain-abstraction/chain-signatures) treats threshold signing and foreign-chain transaction construction/broadcast as distinct steps. Hyperliquid's API wallet example illustrates that delegation to sign can have a different account identity and nonce state from the owner. If a foreign destination accepts only a threshold signature, compromise of that threshold can bypass Moriarty policy at that destination; an SDK must disclose this before the grant is signed. A destination contract that checks a Moriarty proof has a different trust premise. Do not label these profiles equivalently secure.

### Evidence provenance

Use an evidence graph whose nodes retain raw artifact digest, source domain, issuer/attester, verifier and version, verifier policy hash, public inputs, claim, observation time and domain height, finality rule, freshness window, and dependent receipt IDs. Distinguish `RawObservation`, `VerifiedClaim`, `AcceptedEffect` and `Unknown`. The kernel may store all four; only a selected verifier can create a `VerifiedClaim`, and only the settlement judgment can create an `AcceptedEffect`.

[RFC 9334](https://www.rfc-editor.org/rfc/rfc9334.html) distinguishes evidence produced by an attester, the verifier's attestation result, and the relying party's own appraisal policy. That vocabulary maps usefully to foreign-chain RPC data, a light-client or quorum verifier result, and Moriarty's acceptance judgment. This is a design analogy, not a claim that RFC 9334 specifies blockchain finality. A valid source-chain receipt proves neither bridge destination credit nor nonreceipt. A timeout is only elapsed time. The [Aeon signing experiment](../aeon-kernel-experiments-2026-09-29/signing/RESULT.md) showed that a `native` declaration of finality passed the authoring checker; the [bridge experiment](../aeon-kernel-experiments-2026-09-29/bridge/RESULT.md) rejected full refunds based only on timeout or an empty observation. These bounded observations argue for a separately authenticated nonreceipt/exclusivity premise.

### Fee semantics

Represent each cost as `(asset, payer, beneficiary, class, amount, logical leg, attempt, finality status)`. Classes should include venue execution, network gas, builder/solver, bridge, signing, storage, and recovery. A fee quote reserves capacity but is not a debit. Accepted effects charge actual fees against both the relevant `FeeGrant` and the intent's gross lifetime budget, unless the signed intent explicitly excludes a third-party paid amount from the user's debit while still reporting it. Net goals subtract all fees borne by the user. A refund does not reset previously incurred gross costs. The sponsor can pay gas without acquiring spend authority or a right to alter the solver's quote. [ERC-4337](https://eips.ethereum.org/EIPS/eip-4337) provides an instructive separate paymaster validation flow; it does not imply that paymaster sponsorship establishes business outcome.

## Developer-facing API sketch

The code is illustrative surface syntax. It is **specified-only** and assumes no current Moriarty compiler, SDK, adapter, or Preview support.

```ts
const intent = moriarty.intent("swap-net/1", {
  from: asset("midnight:USDC", 100_000n),
  receive: atLeast(asset("arbitrum:USDC", 98_000n)),
  recipient: account("arbitrum", "0xRecipient"),
  grossDebitMax: asset("midnight:USDC", 100_000n),
  feeMax: asset("midnight:USDC", 2_000n),
  expiresAt: domainTime("midnight", 4_200_000n),
  onUnknown: retainDuty("reconcile-before-retry"),
});

const action = grantAction({
  grantee: kernel("federation-A"),
  adapter: pinned("hypercore-order", "v1", "sha256:..."),
  action: "place-limit-order",
  domain: "hyperliquid:mainnet",
  target: "hypercore",
  quantityMax: 100_000n,
  recipient: "0xRecipient",
  key: { scheme: "secp256k1", path: "moriarty/7", epoch: 5 },
  usesMax: 1,
  expiresAt: 4_200_000n,
});

const fees = grantFees({
  payer: user, sponsor: kernel("federation-A"),
  allowed: ["network", "venue", "builder"],
  perLegMax: 1_000n, lifetimeMax: 2_000n,
  beneficiaryAllowlist: ["venue", "builder:0xB"],
});

const evidence = requireEvidence({
  claim: "destination-delivery",
  verifier: pinned("arbitrum-finality-verifier", "v1", "sha256:..."),
  policy: "finalized-height-and-unique-claim/1",
  noFallback: true,
});

const envelope = await wallet.reviewAndSign(
  canonicalize({ intent, action, fees, evidence, policyHead, nonce })
);
const plan = await kernel.quote(envelope);
const checked = await moriarty.validatePlan(envelope, plan);
const prepared = await adapter.prepare(checked.leg("order-1"));
const signed = await kernel.requestSignature(prepared); // signed, not executed
const submitted = await kernel.dispatch(signed);        // submitted, not filled
const result = await kernel.reconcile(submitted);        // effect | pending | unknown
```

SDK types should expose `PreparedCall`, `SignedCall`, `SubmittedCall`, `VerifiedClaim`, `AcceptedEffect` and `Unknown` as distinct states; a caller should need an explicit verifier/import operation to advance between them. A high-level helper can return a resumable `Continuation` with `status`, consumed budget, open orders, pending evidence and recovery actions. Error text should name the failed predicate and required evidence, for example `FEE_CAP: builder 12 bp exceeds signed 10 bp` or `FINALITY_UNKNOWN: destination claim C7 has no accepted nonreceipt proof`. A dry-run can explain a plan, but its result cannot be used as an acceptance certificate.

## Governance and upgrade boundary

Pin the authority-critical schema, verifier policy, adapter code hash, signing policy/key epoch and settlement verifier identity in each signed envelope. A governance action may register a new adapter or verifier for **future** intents. It may pause new dispatch, rotate a compromised key, or revoke a grant under the original signed revocation rule. It cannot widen a live intent's recipients, budgets, evidence class or recovery rights by changing a registry pointer. Migration of a pending continuation needs an explicit user authorization that references the old intent, committed prefix, unresolved attempts and new policy; otherwise the old continuation remains under its old policy or enters an explicit manual recovery state.

Use separate roles for proposing, approving and executing policy changes; publish action digest, old and new policy heads, delay, activation height, affected in-flight intents and emergency power. OpenZeppelin's [access-control guidance](https://docs.openzeppelin.com/contracts/5.x/access-control) describes timelock control for privileged operations. A timelock gives review time, not a proof of safe migration. For emergency pause, define exactly which operations stop and which reconciliation/refund observations remain possible. Preserve the direct valid Midnight path if the optional kernel is paused, where its actual verifier and contract allow that path.

Three governance alternatives deserve a decision: (A) immutable policy per intent with new-version opt-in (recommended for first slice); (B) signed policy family with bounded compatible upgrades, which reduces re-signing but makes compatibility proof a new obligation; (C) mutable registry head, which is easiest operationally but risks changing the meaning of a live signature. Alternative A minimizes upgrade authority and gives the SDK a comprehensible signature summary. It costs explicit migration for long-lived positions.

## Discriminating experiments and acceptance criteria

These are **specified-only** experiments. The existing [Aeon authority experiment](../aeon-kernel-experiments-2026-09-29/authority/REPORT.md) accepted an impossible combined refinement and even a literal false condition in its pinned path. Its selected rejection cases do not authorize using Aeon as a settlement checker.

1. **Exact bytes and review:** Generate two encoders from one schema; compare signed bytes and displayed fields. Mutate one recipient byte, chain ID, adapter hash, fee asset, nonce and recovery branch. The actual verifier must reject each mutation before signature issuance or effect acceptance.
2. **Grant competition:** Two solvers reserve against one gross cap, including a fee-paying sponsor. Admit at most the budgeted cumulative effects under concurrent attempts; preserve charged fees after a refund. Test revoke versus an already submitted foreign call as an `unknown` continuation.
3. **Signer lifecycle:** Rotate key epoch while an old payload is signed but not broadcast. Test path reuse, wrong payload digest and old external nonce reuse after signer deregistration. Require Moriarty logical-leg consumption independent of the venue's nonce store.
4. **Evidence substitution:** Feed an RPC success flag, a source lock receipt, a signed transaction, an enqueued CoreWriter event, and a valid destination verifier result into the same delivery condition. Only the policy-selected result with matching claim ID/public inputs may satisfy it. A resting or partly filled order must keep a residual duty.
5. **Refund race:** Deliver part of a bridge claim, delay observation, request refund, then deliver late. Without an authenticated destination terminal/nonreceipt certificate or mutual exclusion mechanism, remain `unknown`; never accept cumulative delivery plus refund above entitlement.
6. **Upgrade:** Change adapter/verifier registry and governance head after signing. Old intent remains bound to old hashes. Reissue only after a migration signature accounting for the committed prefix and outstanding attempts.
7. **SDK usability:** Give developers the same plan through low-level calls and the high-level helper. Compare exact envelope digests, error predicates, resumable state and wallet summary. This is an ergonomic experiment, not a security substitute.

For any claimed implementation result, record exact command, source commit, canonical bytes, verifier and contract IDs, observed ledger effects and adverse cases. The first live acceptance target should be a direct Midnight transfer with a fee grant and explicit effect receipt, followed by one external adapter with a named evidence verifier. A simulated foreign chain can validate protocol logic but cannot establish foreign finality or Preview financial settlement.

## Dissent and unresolved decisions

I dissent from making `reserve → authorize → dispatch` a universal mandatory kernel sequence. A direct settlement path can accept a fully specified signed program without a coordinator reservation; the reservation is an operational concurrency tool, and its guarantee depends on who enforces shared state. The kernel API should expose it for shared budgets while the settlement verifier independently checks cumulative consumption.

I also dissent from a generic `verified` status in the API. It hides *who* verified *which* claim under *which* policy head. Return a typed `VerifiedClaim<Claim, VerifierPolicy>` plus its provenance, then let the settlement judgment decide whether it is sufficient. This is consistent with RFC 9334's separation of verifier result and relying-party appraisal.

Open choices requiring design review are the first custody model, the final canonical byte format and verifier locus, the first foreign finality verifier, whether a sponsor can withdraw a fee grant while an attempt is pending, how old-key signed transactions are treated after rotation, and whether governance has an emergency power to stop already authorized foreign dispatch. Each choice changes a trust or liveness assumption. None can be inferred from NEAR, Hyperliquid, Cosmos, Ethereum, or the Aeon probes.

## Addendum: lightweight framework choices (2026-09-29)

**Recommendation:** Use the project's existing Node/TypeScript and Midnight integration as the host. Add small, domain-specific libraries at the edge; keep Moriarty's canonical bytes, grant accounting and evidence judgments in its own specified and verified contract. The repository's [language package](../../experiments/moriarty-language/package.json) already builds with `tsc`, and the [financial integration package](../../experiments/moriarty-midnight-financial/package.json) uses Node test and Midnight-specific modules. This is a repository observation, not a released SDK compatibility claim.

| Choice | Primary-source maintenance and license evidence | Fit and cost |
| --- | --- | --- |
| [Zod Mini](https://zod.dev/packages/mini) for SDK input and receipt-shape validation | The project documents a smaller tree-shakable API; the [package declares MIT](https://github.com/colinhacks/zod/blob/main/packages/zod/package.json) and its [release list](https://github.com/colinhacks/zod/releases) shows ongoing releases. | **Use at untrusted JSON boundaries only.** Gives readable errors and TypeScript inference with one small dependency. It cannot validate signatures, finality, cumulative budgets or exact canonical byte correspondence. Pin one version and fixture its parse/serialize boundary. |
| [viem](https://viem.sh/docs/actions/wallet/signTypedData) for an EVM adapter | Its official docs implement EIP-712 typed signing; the [repository reports MIT](https://github.com/wevm/viem) and has a [release history](https://github.com/wevm/viem/releases). | **Use only when an EVM adapter is selected.** It reduces custom EVM ABI/signature code. It adds an Ethereum-specific dependency and cannot define Moriarty's cross-domain envelope or prove foreign execution. Pin version and exact transaction bytes per adapter. |
| [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts/5.x/access-control) for a future EVM enforcement contract | The [license is MIT](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/LICENSE); official docs provide `AccessControl`, `AccessManager` and `TimelockController`. | **Conditional.** Use the smallest role/timelock primitive needed if Moriarty deploys an EVM contract. It does not run on Midnight, and `AccessManager` or `Governor` would add governance state and operations that the first kernel slice does not need. |
| [Cosmos SDK authz](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-030-authz-module.md) and [feegrant](https://github.com/cosmos/cosmos-sdk/blob/main/x/feegrant/README.md) | Official source specifies separate grant and fee allowance modules. | **Reference pattern, no dependency.** Importing a chain application framework into a Node/Midnight coordinator is disproportionate to four bounded record types. License and current release support were not checked for this candidate because adoption is not proposed. |

These are integration preferences, **not verified dependency selections**. I inspected public docs/repository metadata and the two local package manifests; I did not inspect transitive dependency trees, reproduce builds, audit current releases, verify license obligations of a composed distribution, or test compatibility with the current Midnight SDK. Those checks and pinned versions belong to an implementation decision. The first slice can omit all three optional libraries if the host's existing codecs and wallet interfaces already satisfy the exact-byte fixtures; adding a framework should remove concrete adapter code or developer errors, not introduce a second source of authorization truth.
