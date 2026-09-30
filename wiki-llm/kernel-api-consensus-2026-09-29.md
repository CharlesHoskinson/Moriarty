# Kernel API study: consensus candidate

**Status:** audited research candidate, not an adopted MIL/4 profile. **Date:** 2026-09-29. Nine substantive architect memos are in [the study directory](../deliverables/kernel-api-architects-2026-09-29/), with [six external terminal receipts and three host-assigned Sol seats](../deliverables/kernel-api-architects-2026-09-29/study-receipts.json). The independent audit first requested changes and then approved the revised candidate within this research scope. This note records the proposed contract, its source basis, disagreements, and a bounded interface probe. It makes no proof, compiler, adapter, or live-chain claim.

## Decision boundary

There are three distinct components:

1. **Moriarty intent language:** states the signed authority, allowed plan relation, financial bounds, evidence policy, accepted effects, and duties that survive a partial or unknown outcome. Its validity cannot require a hosted kernel.
2. **Optional DeFi kernel:** builds and pins native calls, coordinates signatures, bridge and venue requests, journals attempts, imports observations, and proposes a language transition. Its own API response cannot make an external effect true.
3. **Financial venue or settlement locus:** a Midnight verifier, foreign chain, bridge endpoint, order book, or same-domain system that actually accepts or rejects an effect. Its trust and finality policy are named per leg. HyperCore is an example of a financial venue kernel; it is not the Moriarty coordination kernel.

This split is grounded in the [Moriarty product contract](../docs/MORIARTY-PRODUCT-CONTRACT.md), the [NEAR scope proposal](../deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md), and [Hyperliquid comparison](../deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md). The NEAR verifier can settle credited balances under its own transaction boundary, while deposits, external withdrawals and bridge routes carry separate assumptions ([verifier](https://docs.near-intents.org/integration/verifier-contract/introduction), [bridges](https://docs.near-intents.org/integration/bridging/overview)). A HyperEVM CoreWriter transaction can enqueue a HyperCore action whose execution follows later ([CoreWriter](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore), [interaction timing](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interaction-timings)). These are source facts; the three-component Moriarty contract is our inference.

## Candidate contract

The language-facing ABI is a small set of typed records, independent of HTTP, a queue framework, or any one chain SDK. Every record has a schema version and a stable logical identity. Authority-critical fields must be in a closed, signed byte region; ignorable display metadata belongs in a separate region and cannot change meaning. Financial amounts are exact integers with an asset scale and a domain-qualified asset identity. All authority-relevant bytes, hash framing and verifier loci remain open until MIL/4 M4-C1 and M4-C5 publish a profile and independent vectors.

| Record or call | Contract |
| --- | --- |
| `IntentEnvelope` | User-signed relation: program and Core digests, principal, action class, domain, asset, native target and parameter predicate, exact adapter manifest digest, recipient, gross and fee caps, net floor, expiry, nonce, permitted evidence/finality, settlement locus and recovery bounds. |
| `CapabilityManifest` | Immutable adapter pin: code/codec digest, supported action class, target, native signing domain, extraction rule, native success discriminator, verifier locus and upgrade version. |
| `PreparedCall` | Native unsigned bytes and decoded field map with a digest. `prepare` is advisory until authority is checked against the exact bytes. |
| `SignRequest` | Exact preimage digest, scheme, domain, chain, derivation path, key epoch, recipient, value, fee and nonce. A signature receipt proves only that those bytes were signed under its named trust premise. |
| `OperationKey` and `AttemptId` | A stable logical economic effect ID and a distinct ID for each actual send. A repeated API key returns recorded journal state; it does not trigger another network send. A repeated key with changed payload is a conflict. A lost acknowledgement remains unresolved until evidence shows whether a send took effect. |
| `Observation` and `AcceptedEffect` | Source bytes, issuer, native identity, phase, finality and verifier premise; then distinct evidence-verification and language-acceptance judgments, with separate source-event and leg-progress consumption IDs. Signature, submission, inclusion, queueing, execution, delivery and refund are distinct facts. |
| `Continuation` | Authenticated predecessor and consumption identity; cumulative delivery, refunds, fees, reservations, open orders/claims, live attempts and residual duties. `unknown` remains expressible and blocks unsafe retry or refund. |

The kernel API may expose `prepare`, `reserve`, `sign`, `dispatch`, `observe`, `reconcile`, `cancel` and `recover`, each with a versioned request and a typed result. The accepting language judgment rechecks the complete effects. A direct Midnight caller can use the same authority and acceptance rules without this optional service. [EIP-712](https://eips.ethereum.org/EIPS/eip-712) illustrates domain-separated structured signing, but its text says replay protection is separate; Moriarty's exact bytes and nullifier must be defined in its own profile. [CAIP-2](https://standards.chainagnostic.org/CAIPs/caip-2) and [CAIP-19](https://standards.chainagnostic.org/CAIPs/caip-19) are candidate identifier grammars, not adopted proof of native enforcement.

For a hosted implementation, persist the exact payload commitment, authority reservation and nonce allocation before an external send, and serialize use of shared authority. A possible send followed by transport failure keeps its attempt live. Repeating an API request may retrieve that state; rebroadcasting signed bytes requires adapter-specific proof that native replay is harmless. No generic HTTP idempotency key can establish one financial effect across chains.

Before freezing the ABI, each action class must state its enforcement locus and any `unenforcedFields`, including when a threshold signer or destination contract cannot check Moriarty caveats. The contract must define durable logical-request consumption, grant attenuation, revocation freshness, signer key epochs, and the exact rule for a rejected request versus an ambiguous post-send outcome. A stale revocation view or missing response after a possible send cannot silently become authority to sign again. These obligations are specified here, not implemented.

### Outcome model

Store an append-only set of causal facts with separate source, effect, recovery and control views. Derive a display status from those facts. Never overwrite a partial fill with a timeout or turn a queue log into execution. `unknown` is an outcome with live duties, not permission to start a fresh attempt. The source of a finality or nonreceipt fact and its verifier must be named. IBC ICS-004 gives a concrete primary example: a packet timeout is a proof-bearing protocol path, not merely an elapsed local timer ([IBC packet semantics](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md)).

The [Aeon probes](aeon-kernel-experiment-evidence-2026-09-29.md) support this separation only as authoring experiments. They caught selected phase substitutions and unsafe refunds. The authority probe also accepted an impossible refinement and a bare `false` term; a native declaration of finality was accepted. Aeon therefore cannot be the acceptance gate. No external finality, threshold signing, exact native bytes or Midnight correspondence was proved.

## Lightweight open-source bias

Prefer a small implementation with few operational dependencies. If the first runtime is TypeScript and a persistent service is required, use a [supported PostgreSQL release](https://www.postgresql.org/support/versioning/) for the append-only journal, unique operation/attempt keys, reservations and outbox; start with a small worker. [Graphile Worker](https://worker.graphile.org/docs/requirements) is a candidate only if its Node and PostgreSQL requirements fit the deployed runtime. The current Graphile page says PostgreSQL 12+ and Node 22.18+; PostgreSQL's support page lists versions 14–18 as supported on this date, with 14 nearing end of support, so prefer a supported version with useful remaining life. This is an implementation choice, not the semantic contract.

If deterministic CBOR is chosen for **new** Moriarty authority records, [RFC 8949 §4.2](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2) supplies a standard profile and [`cborg`](https://github.com/rvagg/cborg) is a small maintained TypeScript candidate. Specify one restricted canonical profile, reject unknown critical fields, and require independent byte vectors before signatures. Do not change existing ZKIR or Midnight encodings by implication. [`@noble/hashes`](https://github.com/paulmillr/noble-hashes) is a candidate for a pinned TypeScript digest implementation only after the algorithm and domain framing are frozen. [`viem`](https://github.com/wevm/viem) belongs in an EVM adapter if one is chosen, not the common ABI. A schema parser such as [Zod Mini](https://zod.dev/packages/mini) may validate SDK input; it cannot prove native payload meaning. [OpenZeppelin Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts) is relevant only if an EVM enforcement contract is built. No package is selected or installed by this study.

## Disagreements retained

The nine seats converge on the language/kernel/venue split, exact action authority, distinct evidence phases, durable operation identity, and an explicit unresolved state. The proposals differ on wire bytes and on how much adapter flexibility to expose:

| Seat | Main proposal | Contribution to this candidate |
| --- | --- | --- |
| [Sol ABI](../deliverables/kernel-api-architects-2026-09-29/sol-abi.md) | Closed semantic ABI; restricted deterministic CBOR candidate | Typed authority, pinned manifests, receipt algebra; byte choice remains open. |
| [Grok ABI](../deliverables/kernel-api-architects-2026-09-29/grok-abi.md) | Closed positional binary `MKAP-1` | Strong alternative to CBOR; exact widths and signature scheme are not adopted. |
| [Opus ABI](../deliverables/kernel-api-architects-2026-09-29/opus-abi.md) | Three planes, with closed authority and pinned native payloads | Separates authority evolution from service RPC and adapter evolution. |
| [Sol operations](../deliverables/kernel-api-architects-2026-09-29/sol-ops.md) | Journal, logical effect and attempt IDs, reconciliation | Pre-send durable reservations and distinct unknown outcomes. |
| [Grok operations](../deliverables/kernel-api-architects-2026-09-29/grok-ops.md) | Four operational rails and derived outcome | Causal facts for queued, partial and recovery paths. |
| [Opus operations](../deliverables/kernel-api-architects-2026-09-29/opus-ops.md) | Orthogonal product machine; native idempotency classes | Retry depends on the destination's declared, evidenced dedupe behavior. |
| [Sol trust](../deliverables/kernel-api-architects-2026-09-29/sol-trust.md) | Separate intent, action, fee and evidence authorities | Exact sign request, revocation and verifier-locus disclosure. |
| [Grok trust](../deliverables/kernel-api-architects-2026-09-29/grok-trust.md) | Caveated capabilities and visible unknown outcomes | Narrow delegated grants and a direct verifier client. |
| [Opus trust](../deliverables/kernel-api-architects-2026-09-29/opus-trust.md) | Envelope-rooted narrowing grants | Separate evidence verification and language acceptance verdicts. |

The [current ERC-7683 draft](https://eips.ethereum.org/EIPS/eip-7683) is resolver-centric. The Grok ABI memo describes the earlier `open`/`fill` draft and should not be used as a statement of the current ERC. This version mismatch is a reason to pin primary-source versions before adopting external schema terms.

| Question | Candidate choice | Dissent or unresolved work |
| --- | --- | --- |
| Lifecycle | Causal facts and derived statuses | A compact single status enum is easier for clients, but cannot store source-final/destination-unknown, queued actions, partial fills and multiple attempts at once. It can remain a projection. |
| Adapter API | Closed authority fields and typed capability classes; sealed native payload behind a pinned codec | A generic resolver/opaque payload can support new venues faster. It pushes a large trust obligation onto the resolver and its assumptions. [ERC-7683](https://eips.ethereum.org/EIPS/eip-7683) is useful as an evolving comparison, not an adopted Moriarty wire format. |
| Encoding | Publish one exact-byte profile after independent vectors | Deterministic CBOR is attractive, but current Moriarty and native chain bytes may be different. A premature universal codec would create migration risk. |
| Authority | Narrow, attenuable grants and exact sign requests | A broad agent key can reduce latency but lets the operator choose details outside the signed intent. Hyperliquid's [nonce guidance](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets) also shows replay hazards around agent-key lifecycle. |
| Operational framework | PostgreSQL journal plus a small worker if a hosted service is needed | Temporal and NATS may help at later scale but increase the initial control plane and do not prove external effects. The first runtime and deployment target are not yet fixed. |

## Bounded consensus experiment

[`consensus_probe.py`](../deliverables/kernel-api-architects-2026-09-29/consensus_probe.py) is a standard-library state probe of the proposed logical effect ID, attempt ID, queue, partial delivery, refund and outcome rules. Its [results](../deliverables/kernel-api-architects-2026-09-29/consensus_probe.results.json) record expected and observed decisions. The phase setter accepts fixture claims without checking provenance or valid phase transitions; `prove_exclusive` is a bare fixture flag; and duplicate dispatch returns without inspecting an external send. A late delivery after a refund is retained and labelled `breach`, because discarding that evidence would hide an accounting failure. These results test a few internal branch decisions only. They are **not evidence of recovery safety**, nonreceipt, finality, idempotent network behavior, or chain implementation. Hostile cases for native replay, false exclusivity evidence, reorgs and concurrent workers remain specified only.

## Open gates

The next review must select one native adapter, publish exact signed bytes and independent decode vectors, name each settlement verifier and evidence extractor, test recovery against real route semantics, and check a direct Midnight path. The blocked SP01.6 guarded campaign remains separate. This note does not authorize dispatch, refund, live signing, adoption of a new MIL/4 rule, or a product release.
