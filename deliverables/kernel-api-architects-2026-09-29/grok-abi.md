MKAP-1 is the contract I recommend: a closed, positional, versioned binary ABI whose user signature covers the financial constraints, and whose kernel methods return typed phases and evidence. A solver, signer, or operator response is not settlement. This is one architect's proposal, not a study consensus and not an MIL decision.

This consultation is tool-free. The brief says the lead already ran guarded status, and the packet contains no status JSON, so this memo reports no gate result. No product code was changed. NEAR, Hyperliquid, and Aeon statements below are taken from the supplied packet. The five extra standards are published documents I know; I did not retrieve them for this memo. Packet facts, standard facts, and inferences are marked.

## Recommendation

Adopt **MKAP-1** (Moriarty Kernel API, version 1) as the optional Federated DeFi Kernel's wire contract.

The kernel parses a canonical authorization, checks a plan against that authorization, reserves occurrence ids, asks a pinned signer to sign only a matching payload, dispatches exact bytes, stores attempts, and imports evidence. It may keep operational state. It has no authority to change the signed relation, weaken the evidence class, erase a committed effect, or declare a cross-chain workflow atomic. Those limits are the packet's proposed kernel boundary, grounded in the NEAR verifier/bus/bridge/MPC split and the Hyperliquid venue-versus-coordinator split.

Direct Midnight execution uses the same user envelope and does not call the kernel. `U0`–`U3` language gates stay independent of this service. Exact-byte adapters and this ABI are a `U5` protocol proposal.

## Sources

**Packet, 2026-09-29, already researched.** NEAR: verifier holds credited balances and verifies signed intents inside one verifier transaction ([verifier](https://docs.near-intents.org/integration/verifier-contract/introduction), [intents repo](https://github.com/near/intents)); the message bus is optional ([market makers](https://docs.near-intents.org/integration/market-makers/introduction)); archived 1Click is a hosted flow with its own trust boundary ([1Click snapshot](https://github.com/defuse-protocol/gitbook-docs/blob/main/integration/distribution-channels/1click-api.md)); each bridge route has its own trust model ([bridges](https://docs.near-intents.org/integration/bridging/overview)); Chain Signatures are outbound and one-way ([chain signatures](https://docs.near.org/chain-abstraction/chain-signatures)), while foreign observation is a separate MPC capability ([mpc](https://github.com/near/mpc), [omni-transaction-rs](https://github.com/near/omni-transaction-rs)); a BTC refund is a costed multi-step workflow ([BTC refund](https://docs.near-intents.org/integration/bridging/btc-deposit-refund)). Hyperliquid: HyperCore holds matching and margin ([core](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview), [order book](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/order-book)); CoreWriter actions can be enqueued before execution ([CoreWriter](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore), [timings](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interaction-timings)); builder fees are a separate revocable cap ([builder codes](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes)); agent nonces are per signer and pruning can revive replay if an address is reused ([nonces](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets)); the exchange API's order statuses distinguish resting from filled ([exchange](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint)); a withdrawal request precedes validator signing and the external send ([trading](https://hyperliquid.gitbook.io/hyperliquid-docs/onboarding/how-to-start-trading)); an oracle label is not an objective fact ([oracle](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/oracle)). Aeon pin: [alcides/aeon](https://github.com/alcides/aeon) commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`, Aeon 4.9.0. The proposal says it refines `docs/MORIARTY-CONSOLIDATED-DESIGN.md`; this memo uses only what the packet says about that design.

**Five further primary sources, from prior knowledge of the published texts.**

| # | Source | Fact used here |
| --- | --- | --- |
| 1 | [EIP-712](https://eips.ethereum.org/EIPS/eip-712) | Typed-data signature preimage is `\x19\x01 ‖ domainSeparator ‖ hashStruct(message)`. The domain separator binds name, version, `chainId`, verifying contract, and optional salt. |
| 2 | [ERC-7683](https://eips.ethereum.org/EIPS/eip-7683) | A cross-chain order splits origin open from destination fill. A gasless order carries user, nonce, origin chain, open and fill deadlines, an `orderDataType` hash, and implementation-defined `orderData`. A resolver turns that into outputs and fill instructions. |
| 3 | [CAIP-19](https://github.com/ChainAgnostic/CAIPs/blob/main/CAIPs/caip-19.md) | An asset id is `{chain_id}/{asset_namespace}:{asset_reference}` with an optional token id. `chain_id` is CAIP-2 `namespace:reference`. Account ids in the same family are CAIP-10. |
| 4 | [IBC ICS-004](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md) | A packet commitment, a receive receipt, and an acknowledgement are different stored facts. `timeoutPacket` requires proof that the destination has no receipt, plus an elapsed timeout, plus a live source commitment. Elapsed time alone does not prove absence. |
| 5 | [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) | Deterministic CBOR uses definite lengths, preferred integer encoding, and sorted map keys. Map canonicalization is still an encoder agreement. |

**Inference from those standards, not a clause of any of them.** EIP-712 shows how to bind a signature to a typed domain, and it does not describe foreign payload codecs or evidence classes. ERC-7683 shows a useful open/fill split, and its opaque `orderData` is the wrong place to put Moriarty authority. ICS-004 is the receipt model to copy for "timeout is not non-execution." CAIP-19 names an asset on a chain and does not name a bridge representation or a ratio. RFC 8949's map rules are why MKAP-1 uses a positional layout instead of canonical JSON or CBOR maps. [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785) is a related JSON canonicalization; JSON numbers are a poor encoding for 256-bit amounts.

## Two designs

**Design A, recommended: MKAP-1 closed positional records.** The user signs one canonical envelope. Every authorizing object has a frozen field order, a schema id, and an exact body length. A call may contain venue bytes only after a typed prefix and only under a pinned adapter hash. Phase changes consume named evidence. Unknown schema bytes are rejected.

**Design B: ERC-7683-shaped header plus resolver-defined `orderData`.** The signed object is a short header (user, nonce, deadlines, type hash, opaque bytes). `resolve` on an origin settler produces the spend, the received amounts, and the fill instructions. Destination `fill` receives `originData` the filler does not re-derive from a user-checked codec hash.

| Question | Design A | Design B |
| --- | --- | --- |
| Where the money fields live | In the signed positional body | In resolver output from opaque bytes |
| Adding a venue | Publish a codec hash and a typed prefix | Deploy a resolver; the header stays stable |
| Adapter upgrade | New hash; old signatures do not cover it | Proxy or resolver upgrade can reinterpret old `orderData` |
| Queued versus executed | Different phases and evidence | One fill result unless the resolver adds its own status |
| Timeout | Stays `unknown` until proof of absence or a finalized remainder | Whatever the settler encodes; the standard has no absence proof |
| Direct Midnight | Same envelope, kernel unused | A hosted resolver becomes a practical dependency |
| Failure mode | Rejects a leg the schema does not name | Accepts a signature whose effect is decided at resolve time |

Design B integrates venues faster and matches a standard Ethereum wallets already approach. Its authority is the resolver. That contradicts the packet's rule that a relay cannot replace the accepted relation. Design A costs a frozen schema and a pinned codec per adapter. That cost is the mechanism that makes the hostile traces rejectable at the API boundary.

A third pattern, a hosted status API in the style of archived 1Click, is a product front end. It is not a candidate for this ABI. The packet already records that its statuses and custody assumptions are a different trust boundary.

## MKAP-1 wire rules

Integers are unsigned, fixed-width, and big-endian: `u8`, `u16`, `u32`, `u64`, `u128`, and `u256` (32 bytes). Amounts are `u256`. There is no decimal rescaling at decode time.

`bytes32` is 32 raw bytes. `utf8` is `u32 nbytes ‖ bytes`: valid UTF-8, no U+0000, no normalization, `nbytes ≤ 1024`. Comparison is raw equality. `blob` is `u32 nbytes ‖ bytes`. `list` is `u32 count ‖ elements`. `opt` is `u8` `0` (absent) or `1` (present plus value); any other tag rejects. `enum` is `u16`; an unassigned value rejects.

Maxima in v1: list count ≤ 256; call payload ≤ 262144 bytes; evidence body ≤ 1048576 bytes. A body that exceeds its maximum rejects.

Every object begins with a 12-byte header:

```text
magic         4     4D 4B 41 50        "MKAP"
api_version   u16   1
schema_id     u16
body_length   u32
body          body_length bytes
```

`body_length` equals the encoded size of that schema's field sequence. Trailing bytes reject. A short body rejects. v1 has no open extension area. A new field is a new `schema_id`. A v1 parser rejects any other `api_version` or `schema_id`.

**Inference, stated as a rule of this contract.** Protobuf-style "preserve and ignore unknown fields" is an appropriate transport rule for non-authorizing logs. It is unsound for signed financial records: a dropped field can be the recipient, the ratio, or a tighter cap. MKAP-1 therefore uses closed schemas for authority, plans, calls, receipts, and evidence.

### Hash and signature preimage

Hashes are SHA-256. The hash input is label, one `0x00` byte, then the specified body. Labels are ASCII:

```text
moriarty.intent.v1
moriarty.intent-id.v1
moriarty.quote.v1
moriarty.plan.v1
moriarty.leg-id.v1
moriarty.payload.v1
moriarty.attempt-id.v1
moriarty.sign.v1
moriarty.receipt.v1
moriarty.evidence.v1
```

The user signs the intent preimage, which is the `moriarty.intent.v1` label, `0x00`, and the exact envelope header and body. The signature bytes are outside the preimage.

```text
preimage = "moriarty.intent.v1" ‖ 0x00 ‖ header ‖ body
```

`sig_scheme` values:

| Id | Meaning |
| --- | --- |
| 1 | Ed25519 over the preimage. Public key 32 bytes, signature 64 bytes. |
| 2 | Reserved, rejected in v1. |
| 3 | EIP-712 binding for wallets that can sign only EIP-712. |

Scheme 3 signs an EIP-712 struct whose single message field is `preimageHash = SHA-256(preimage)`, plus `apiVersion` and `schemaId`. The EIP-712 domain name is `Moriarty`, version `1`, `chainId` is the wallet account's chain, and `verifyingContract` is the pinned EVM contract or the zero address when the account is not an EVM contract wallet. The verifier recomputes `preimageHash` from the MKAP bytes and checks the EIP-712 signature. The wallet `chainId` and the intent's settlement locus are both bound: the first by EIP-712, the second inside the preimage. Scheme `personal_sign` is not a v1 scheme. It does not bind `chainId` or a verifying contract (EIP-712 fact; the rejection of `personal_sign` is this contract's rule).

Collision resistance of SHA-256 is an assumption of scheme 3, not a result proved in this memo.

Ids are computed. A caller-supplied id is accepted only when it equals the computation.

```text
intent_id  = SHA-256("moriarty.intent-id.v1"  ‖ 0x00 ‖ envelope_header ‖ envelope_body)
payload_hash = SHA-256("moriarty.payload.v1" ‖ 0x00 ‖ adapter_code_hash ‖ codec_id ‖ exact_payload)
leg_id     = SHA-256("moriarty.leg-id.v1"     ‖ 0x00 ‖ intent_id ‖ leg_index ‖ action_class ‖ domain_utf8 ‖ occurrence_salt)
attempt_id = SHA-256("moriarty.attempt-id.v1" ‖ 0x00 ‖ leg_id ‖ ordinal ‖ payload_hash)
```

`occurrence_salt` comes from the user grant, so two legs of one class do not alias. `leg_index` is the index in the plan the user-side authorizer committed.

### Schema registry (v1)

| `schema_id` | Record | Signer |
| --- | --- | --- |
| 1 | `IntentEnvelope` | User |
| 2 | `Capability` | Adapter publisher, operational |
| 3 | `CapabilityGrant` | Inside the envelope, so the user |
| 4 | `Plan` | Kernel proposes; user envelope must already allow it. A plan is not a second user signature |
| 5 | `Leg` | Inside the plan |
| 6 | `Call` | Dispatcher, must match a leg commitment |
| 7 | `SignRequest` | Kernel asks; MPC or agent signs only the named payload |
| 8 | `OperationReceipt` | Operator key, operational authenticity only |
| 9 | `Evidence` | The issuer of `evidence_class` |
| 10 | `Continuation` | Operator key, operational |
| 11 | `Quote` | Solver |
| 12 | `AssetAmount` | Embedded |
| 13 | `DomainQualifiedAsset` | Embedded |
| 14 | `Reject` | Method response |

**`DomainQualifiedAsset` (13).** `caip19: utf8`, `representation: enum`, `route_id: bytes32`, `ratio_num: u128`, `ratio_den: u128`, `issuer_policy_hash: bytes32`.

`representation` is `1 native`, `2 bridged_voucher`, `3 pool_share`, `4 claim_iou`. `ratio_den` is nonzero. Native assets use ratio `1/1` and a zero `route_id`. Cap equality is equality of the whole record. A CAIP-19 string match with a different route or issuer does not net. **Inference from the packet:** NEAR documents distinct trust models per bridge route, and a Hyperliquid USDC bridge delivery is a different fact from a venue balance. CAIP-19 alone does not carry route, ratio, or issuer.

**`AssetAmount` (12).** One `DomainQualifiedAsset` and one `u256` amount.

**`IntentEnvelope` (1), field order.** `program_id bytes32`, `program_version u32`, `settlement_locus utf8` (CAIP-2 domain of the program that judges the relation), `execution_mode enum`, `signer_account utf8` (CAIP-10), `sig_scheme u16`, `nonce u128`, `not_before u64`, `expiry u64`, `objective` (a list of `AssetAmount` minima the user must end holding, plus a list of debit caps), `grants` (list of grant), `spend_caps`, `fee_caps`, `evidence_allow` (list of `u16` class ids, nonempty), `recovery_policy_id bytes32`, `key_epoch u64`, `occurrence_salt bytes32`.

`execution_mode`: `1` direct Midnight, `2` coordinated.

Direct Midnight (`1`) requires an empty grant list. The Midnight program parses this schema, checks the user signature, and consumes `nonce` in its own state. The kernel method set is not on the path. A kernel outage has no input into that judgment. **Packet fact:** a service refusal does not make a program invalid, and the kernel is not a prerequisite for a valid direct Midnight program.

Coordinated mode (`2`) may include a Midnight-domain leg plus external legs. The Midnight leg's settlement locus is Midnight. A kernel receipt in phase `submitted` does not settle it.

`fee_caps` and `spend_caps` are separate lists. **Packet fact:** Hyperliquid builder fees are an approved maximum, and later order fees have to fall inside it. A quoted fee is not a charged fee. Charged fees enter gross effects at reconciliation.

An empty `evidence_allow` rejects. There is no "any issuer" value.

**`Capability` (2).** `capability_id bytes32`, `adapter_code_hash bytes32`, `adapter_version u32`, `domain utf8`, `action_class u16`, `codec_id u16`, `trust_profile_id bytes32`, `evidence_profile_id bytes32`. This record advertises an adapter. It does not grant authority.

`action_class`: `1` native financial call, `2` same-domain queued call, `3` foreign-chain call, `4` signer call, `5` observation call. These are the five classes in the Hyperliquid comparison. The class is a wire discriminant, not a comment.

**`CapabilityGrant` (3).** `action_class u16`, `adapter_code_hash bytes32`, `codec_id u16`, `domain utf8`, `settlement_locus utf8`, `target_account utf8`, `parameter_commitment bytes32`, `spend_cap` list, `fee_cap` list, `recipient_account utf8`, `evidence_allow` list, `max_attempts u32`, `key_epoch u64`. For class `4`, the grant also contains `derivation_path utf8` and `canonical_payload_hash bytes32` when the payload is fixed, or a `value_max u256` when the user authorizes a bounded amount. For class `3`, the grant also contains `route_id bytes32`, source and destination `DomainQualifiedAsset`, and `amount_max u256`.

**`Plan` (4) and `Leg` (5).** A plan carries `envelope_intent_id`, `quote_id`, and a list of legs. A leg carries `leg_index u32`, `action_class`, `settlement_locus`, `predecessor_indexes` (list of `u32`), `payload_hash bytes32`, `adapter_code_hash`, `codec_id`, fee budget as `AssetAmount`s, and `expected_evidence_class u16`. Predecessors define the partial order. A cycle rejects.

**`Call` (6).** `leg_id`, `attempt_ordinal u32`, `adapter_code_hash`, `codec_id`, `action_class`, and `exact_payload blob`. The payload's first bytes are a typed prefix the kernel parses. The remainder is codec bytes. `payload_hash` covers both, as defined above.

Prefixes:

- Class `1` and `2`: `action_version u32`, `venue_action_id bytes32`, `limit_amount u256`, `quantity u256`, `recipient utf8`.
- Class `2` additionally: `queue_identity bytes32`. `execution_identity` is absent at dispatch. It appears only on a later receipt.
- Class `3`: `route_id bytes32`, `claim_id bytes32`, `nullifier bytes32`, source asset, destination asset, `amount_in u256`, `recipient utf8`, `fee AssetAmount`.
- Class `4`: no extra spend prefix. The signed object is `SignRequest`.
- Class `5`: `issuer_id bytes32`, `round_id u128`, `subject_hash bytes32`. An observation does not move funds.

A class-3 body decoded under a class-2 schema rejects. The kernel does not coerce.

**`SignRequest` (7).** `intent_id`, `leg_id`, `attempt_id`, `domain utf8`, `key_epoch u64`, `derivation_path utf8`, `sig_scheme u16`, `codec_id u16`, `adapter_code_hash bytes32`, `canonical_payload_hash bytes32`, `recipient utf8`, `value u256`, `nonce u128`, `not_before u64`, `expiry u64`.

The signer capability signs `SHA-256("moriarty.sign.v1" ‖ 0x00 ‖ sign_request_header ‖ sign_request_body)` only when every field equals the grant and the plan leg. There is no `sign(blob)` method. **Packet fact, NEAR chain signatures:** a threshold signature attests that a quorum signed supplied bytes. It does not attest broadcast, inclusion, or business validity. **Packet fact, Hyperliquid:** an agent signature is a separate authority from the master account, and nonce state can be pruned.

Once any signature bytes for an `attempt_id` are released, that attempt occupies the leg. A second sign request for the same `leg_id` rejects while the first attempt is `signed` or `unknown`. A replacement exists only when a new ordinal is authorized and the domain evidence shows the first signature cannot be included. Lack of that evidence leaves the execution question at `unknown`. The signature consumption itself remains.

**`OperationReceipt` (8).** `intent_id`, `leg_id`, `attempt_id`, `payload_hash`, `phase u16`, `effects` (list of `AssetAmount` with a direction `u16`: `1 debit`, `2 credit`, `3 fee`), `evidence_ids` (list of `bytes32`), `finality_height opt u128`, `observed_at u64`.

Phases:

| Id | Phase | What it asserts |
| --- | --- | --- |
| 1 | `planned` | A plan matched the envelope. No external bytes sent. |
| 2 | `reserved` | Occurrence and caps are held in kernel state. |
| 3 | `authorized` | `validatePlan` passed. Still no external effect. |
| 4 | `signed` | Signature bytes exist for this attempt. Execution is not known. |
| 5 | `submitted` | Bytes were handed to a network. Inclusion is not known. |
| 6 | `enqueued` | A same-domain queue accepted the action version. Execution is not known. |
| 7 | `included` | The domain included the payload. Finality is not known. |
| 8 | `source_finalized` | Source finality policy is met. Destination delivery is a separate fact. |
| 9 | `delivered` | The settlement locus reports an effect, possibly partial. |
| 10 | `finalized` | `evidence_allow` is satisfied for every required effect. |
| 11 | `open_duty` | A resting order or unfilled remainder remains. |
| 12 | `refunding` | A recovery edge is open. Refund is not complete. |
| 13 | `refunded` | Refund evidence plus conservation holds for the claim. |
| 14 | `failed` | The domain established non-execution, or the call was rejected before any possible effect. |
| 15 | `unknown` | The kernel cannot establish execution or non-execution. |

`unknown` is a result phase, not an error string. It does not clear `signed`, a payload hash, or a claim id already recorded.

Promotion, and nothing else, is legal:

| From | To | Required evidence |
| --- | --- | --- |
| planned | reserved | Envelope hash match and a refinement check |
| reserved | authorized | `validatePlan` acceptance |
| authorized | signed | Class `4` only, after a matching signature is stored |
| authorized or signed | submitted | `payload_hash` matches the leg; ordinal is free |
| submitted | enqueued | Class `2` queue log naming `action_version` and `queue_identity` |
| enqueued | open_duty or delivered | A later execution receipt. The queue log is not enough |
| submitted | included | Inclusion evidence for this payload hash |
| included | source_finalized | Finality depth named in the grant |
| source_finalized or included | delivered | Effect evidence of an allowed class at the leg's settlement locus |
| delivered | finalized | Every required effect has an allowed evidence id, caps hold, open duty is empty |
| delivered | open_duty | Partial fill or resting remainder, with venue evidence |
| any non-terminal | unknown | Timeout, silence, contradictory RPC answers, or a lost response |
| unknown | a later phase | New evidence of an allowed class. A clock value is not that evidence |
| open_duty | delivered | Fill evidence for the remainder or part of it |
| refunding | refunded | See conservation below |

`failed` is available only before a possible external effect, or when evidence of an allowed class establishes non-inclusion and no signature for that payload exists. A released signature blocks `failed`.

**Same-domain queued versus foreign.** Class `2` uses `enqueued` and then `open_duty` or `delivered` at the **same** settlement domain. Hyperliquid's CoreWriter delay is the packet's example: the queue event and the Core execution are different receipts, and a resting order is an open duty. Class `3` uses source acceptance, optional signer and relayer steps, `source_finalized`, and destination `delivered` under the route's evidence class. A class-2 leg has no `claim_id` and no refund against another chain. A class-3 leg does not treat a source transaction as destination delivery. **Packet fact:** a Hyperliquid withdrawal request, the validators' signatures, and the external bridge transaction are distinct, and a NEAR verifier credit is not a destination withdrawal.

**`Evidence` (9).** `evidence_class u16`, `issuer_id bytes32`, `verifier_premise_id bytes32`, `subject_hash bytes32`, `public_inputs_hash bytes32`, `not_before u64`, `not_after u64`, `uniqueness_key bytes32`, `body blob`.

Classes: `1` domain consensus or light-client proof, `2` threshold attestation, `3` named quorum signature, `4` trusted operator report, `5` proof of absence, `6` venue-authenticated receipt. The grant lists the classes that may support a transition. A class that is not listed does not substitute for one that is. A kernel operator signature on an `OperationReceipt` is class-less operational data. It is not class `1` and it is not class `6`.

Class `5` is the ICS-004-shaped fact: a named verifier attests absence of a claim, receipt, or inclusion under a stated height or round. A `timed_out` boolean is not a member of this enum.

**`Continuation` (10).** `intent_id`, committed `leg_id`s, phases of open attempts, consumed nonce, cumulative debits and fees, remaining caps, and open duties. This is the kernel's memory. It is not the settlement program's authority state. Midnight, a venue, or a bridge destination keeps the state that justifies an evidence body.

**`Quote` (11).** `solver_id`, `intent_id`, proposed legs (same leg schema), fees, `expiry`, solver signature over the quote preimage. `validatePlan` accepts a quote only as a refinement of the envelope: same recipient bytes, same domain bytes, same asset records, amounts at or below caps, fees at or below `fee_caps`, evidence class inside `evidence_allow`, deadlines inside `[not_before, expiry]`. A wider recipient, a higher fee, or a weaker evidence class rejects with `RefinementWidened`.

**`Reject` (14).** `reason u16` and `continuation opt`. Reasons used by the traces below: `1 SchemaUnknown`, `2 CapBreach`, `3 RecipientMismatch`, `4 DomainMismatch`, `5 PayloadHashMismatch`, `6 AdapterHashMismatch`, `7 KeyEpochMismatch`, `8 PhasePromotionForbidden`, `9 EvidenceClassInsufficient`, `10 TimeoutIsNotAbsence`, `11 AttemptWhileUnknown`, `12 DuplicateEffect`, `13 RepresentationMismatch`, `14 DirectModeRejectsKernel`, `15 ConservationBreach`, `16 RefinementWidened`, `17 SignBytesRejected`.

### Methods

Each method returns a typed record. None of them returns a bare success bit.

```text
validatePlan(envelope, plan) -> PlanAcceptance | Reject
reserve(plan)                 -> Continuation | Reject
authorizeSign(sign_request)   -> SignRequest | Reject
dispatch(call)                -> OperationReceipt | Reject
observe(leg_id)               -> OperationReceipt     # phase may be unknown
reconcile(intent_id)          -> Continuation
advanceOrRecover(intent_id, edge_id) -> Continuation | Reject
```

`dispatch` of an `attempt_id` already stored with the same `payload_hash` returns the stored receipt. `dispatch` of the same `leg_id` with a new ordinal, while any attempt is in `signed`, `submitted`, `enqueued`, `included`, `source_finalized`, `delivered`, `open_duty`, `refunding`, `refunded`, `finalized`, or `unknown`, returns `AttemptWhileUnknown` or `DuplicateEffect`. The kernel reconciles known attempts before it creates a new one.

`advanceOrRecover` on a class-3 claim checks one conservation relation before `refunded`:

```text
delivered_total + refund + fee_charged ≤ entitled_source
```

under the same `claim_id`, the same representation record, and evidence that `delivered_total` is complete or that the refund is only the proven-absent remainder. A partial destination delivery authorizes at most the residual. A second full refund of the original amount rejects with `ConservationBreach`.

`validatePlan`, `authorizeSign`, and `dispatch` on an envelope whose `execution_mode` is direct Midnight return `DirectModeRejectsKernel`. They do not mark the Midnight program failed.

### Schema evolution

v1 schemas above are frozen. A later market, a new evidence class, or a new signature scheme is `api_version = 2` or a new `schema_id`, implemented by a parser that still accepts v1 objects as v1. v1 signatures are not retargeted at v2 field layouts. In-flight attempts keep the `adapter_code_hash` they were authorized under. An upgraded codec is a new grant.

Hex case, address checksums, and decimal display are codec concerns. The kernel does not rewrite payload bytes after the hash is fixed.

## What the traces show

Design A means the MKAP-1 rules above. Design B means a signed ERC-7683-style header whose resolver interprets `orderData`. "Aeon" means the pinned 4.9.0 path in the packet, used as a gate: exit 0 would be treated as accept.

### Positive traces

**P1. Direct Midnight, kernel silent.** Envelope mode is `1`, grant list empty, recipient and caps inside schema 1, user signature valid, Midnight nonce free. Midnight's program accepts and settles. `dispatch` on that `intent_id` returns `DirectModeRejectsKernel` and does not write `failed`. Design A and a careful Design B both allow this if the Midnight program sees the signed fields. Design B fails the trace if the only parser that knows `orderData` is the hosted resolver: kernel absence then becomes execution absence. Aeon does not see signature bytes or Midnight, so an Aeon accept is irrelevant to P1.

**P2. Two solvers, one authorized plan, phases kept apart.** Envelope: recipient `R`, domain `D`, fee cap 5, gross cap 100, evidence class `6` for a native venue. Solver 1 quotes fee 4, same `R` and `D`. Solver 2 quotes fee 5 and a different recipient. `validatePlan` accepts solver 1 (`Refinement` holds) and rejects solver 2 with `RecipientMismatch`. `reserve` then `authorizeSign` release one signature. `dispatch` stores `submitted`. `observe` returns `included` without finality evidence. `reconcile` leaves the continuation short of `finalized`. Design A produces that sequence. Design B can represent it only if the resolver's private status happens to match these phases; the signed header does not require the split. Aeon can accept the integer skeleton of solver 1's quote. That accept is not the phase sequence, and the packet shows this pin also accepts `false`.

**P3. Partial delivery, residual refund.** Class `3`, `claim_id = C`, source entitlement 100, destination evidence class `1` says delivered 40, class `5` says the other 60 are absent at the agreed height, fee 0, same representation on both sides. `advanceOrRecover` moves to `refunded` with refund 60. `delivered + refund ≤ entitled` holds. Design A accepts. Design B accepts only if the resolver implements this residual itself; ERC-7683's fill struct does not contain a proof of absence or a ratio. The Aeon bridge probe accepted `refund = sent - delivered` when `delivered` was an input refinement. That is the arithmetic of P3 after the premise is granted. It is not the evidence.

### Hostile traces

**H1. Gross breach hidden in a combined predicate.** Principal 96, fee 5, stated gross 101, cap 100. Design A rejects at `validatePlan` with `CapBreach`, because gross, principal, fee, and both caps are separate fields and `96 + 5 = 101 > 100`. Design B can still show a valid user signature if the cap lives in resolver logic and `resolve` returns a spent amount the header did not bind. On the Aeon pin, the combined encoding `g = principal + fee && g <= maxGross` **accepted** gross 101, while a separate `grossWithinCap` declaration rejected it. `minimal_breach.ae` (`v = 101 && v <= 100`) and `minimal_false.ae` (`false`) exited 0. `minimal_cap.ae` (`v <= 100`) exited 13. An exit-0 gate would accept H1 under the combined encoding.

**H2. Signer changes chain, path, or payload.** Grant: domain `near:mainnet`, path `m/7`, adapter hash `A`, payload hash `H`, recipient `R`, epoch 5, nonce 9. The sign request flips the path, or one payload byte, or the domain. Design A returns `PayloadHashMismatch` or `DomainMismatch` and releases no signature, because there is no untyped sign method. Design B signs what the resolver emits from `orderData`; the user signature can stay valid while the MPC payload changes. The Aeon signing probe rejected integer substitutions for domain, path, digest tag, recipient, epoch, and nonce. It also accepted a `native` value declared as finalized. Integer-tag rejection does not check adapter bytes. **Packet conclusion, signing experiment:** the next real discriminator is an exact-byte fixture, which is what `payload_hash` is for.

**H3. Timeout or empty observation used as a full refund, including after a partial fill.** `sent = 100`, status unknown, `observed_delivery = 0`, or a pure timeout, and the operator requests refund 100. A variant has destination evidence of delivery 40 and still requests refund 100. Design A returns `TimeoutIsNotAbsence` or `ConservationBreach` and leaves the open attempt `unknown` or `delivered`. Design B's settler can implement "deadline passed, refund all" inside `orderData`. The Aeon bridge probe rejected timeout-as-full-refund with counterexample `sent = 1, delivered = 1`, and rejected an empty observation the same way. It accepted a full refund when the input refinement already said `delivered = 0`. So a rejection with a counterexample is informative, and an accept that trusts `delivered = 0` is not a bridge result. ICS-004's `timeoutPacket` rule is the same shape as Design A's class `5` requirement: absence is a proof object, not a clock.

**H4. Second submission while the first attempt is unknown.** Attempt ordinal 0 is `submitted` or `signed`, and the response is lost. Ordinal 1 reuses the `leg_id` with a fresh venue nonce. Design A returns `AttemptWhileUnknown` and keeps ordinal 0. Design B's `openFor` can assign a new nonce and fill twice if the opaque order does not bind a single occurrence. The Hyperliquid nonce fact is the related case: after an agent is deregistered and its nonce store is pruned, the venue may treat an old action as fresh. Design A still rejects, because consumed `nonce` and `leg_id` live in the continuation, not in the agent's nonce map. Design B follows the venue nonce unless the resolver stored its own occurrence. Aeon has no attempt store. A type check cannot see two submissions.

**H5. After authorization, the adapter or the story changes.** Three submissions, one trace family. (a) Adapter hash in the call is `A'` after the grant pinned `A`. Design A returns `AdapterHashMismatch`. Design B keeps the same settler address and a new resolver implementation, and the old signature still verifies. (b) The only evidence is a class-4 operator report, or a signature, and the caller asks for `finalized`. Design A returns `PhasePromotionForbidden` or `EvidenceClassInsufficient` and can store `signed`. (c) A class-2 queue log is presented as a fill. Design A stores `enqueued` and refuses `delivered`. Design B's single fill callback collapses (b) and (c) unless the resolver volunteers extra status. The Aeon signing probe rejected using a `signed` tag where a `finalized` tag was required, and accepted a literal finalized tag and a `native "1"` finalized tag. The tag split is an authoring check. The native accept is a forged finality at the type checker. **Packet fact:** a successful Aeon check cannot authorize a signature request, a bridge call, a refund, or settlement.

| Trace | Design A | Design B | Aeon exit 0 as a gate |
| --- | --- | --- | --- |
| P1 direct Midnight | Settles without the kernel | Settles only if opaque bytes are not resolver-owned | No signature or ledger judgment |
| P2 phased plan | Accepts the tight quote, keeps `included` short of `finalized` | Header does not require the phase split | Accept is non-evidence on this pin |
| P3 residual refund | Accepts 40 delivered + 60 absent | No standard absence or ratio field | Arithmetic only, premise trusted |
| H1 gross 101 | `CapBreach` | Resolver may still "resolve" | Combined encoding accepted |
| H2 payload swap | No signature released | User signature can survive the swap | Tags rejected; bytes unchecked; `native` finality accepted |
| H3 timeout refund | Stays `unknown` or rejects conservation | Settler-defined | Rejects some formulas; trusts `delivered = 0` |
| H4 retry while unknown | Second ordinal rejected | Second nonce can fill | No attempt state |
| H5 adapter or phase collapse | Hash and phase checks reject | Address-stable upgrade can pass | Literal and `native` finality accepted |

## What Aeon can check, and why it is not a gate

At authoring time, on this pin, Aeon is a one-sided probe of integer refinements that are already written in the term.

Observed rejections, with `LiquidTypeCheckingFailedRelation` or a concrete counterexample, show that **that file** failed the checker: wrong recipient, domain, fee, path, digest tag, epoch, or nonce; a split gross cap; `delivered + refund ≤ sent` violated by a full refund on timeout, on unknown, on an empty observation, or on a partial fill; a signed tag used as broadcast or finalized. Those results are useful when developing a plan encoding. They are properties of the files that were run.

Observed acceptances are not verifications. The same pin accepts `false`, accepts `v = 101 && v <= 100`, and accepts the combined gross constraint at 101. It accepts a `native` annotation as a recipient and as a finalized token, and the trust report names that value as native. Splitting one obligation into a separate declaration changed a wrong accept into a reject. The packet is explicit that the root cause was not identified. A checker that accepts an impossible refinement is unsound. A gate needs the opposite direction: accept only when the property holds. Exit 0 from this pin does not mean that.

Even a later sound Aeon would check only the mathematical skeleton it is given. It would not check canonical MKAP bytes, signature schemes, adapter hash correspondence, SHA-256 commitments, MPC release, venue inclusion, bridge representation, or proof of absence. The bridge probe's accept of `refund = sent - delivered` depends on `delivered` already being true in the refinement. The kernel's job is to justify `delivered` by evidence class and `claim_id`. Aeon cannot take that job.

Practical rule for this pin: a counterexample can retire a candidate formula; an acceptance authorizes nothing. `validatePlan` must not call this Aeon and must not take a Boolean "typechecked" input. The gross, recipient, domain, payload hash, phase, and conservation checks belong in the MKAP-1 parser and in the Midnight or venue verifier that sees the actual effects.

## Contract to adopt

1. One user signature scheme family, over the MKAP-1 intent preimage, with EIP-712 only as scheme 3 binding `SHA-256(preimage)`.
2. Frozen v1 schemas 1–14, positional, exact body length, new meaning only by new schema id or api version.
3. Domain-qualified assets are CAIP-19 plus representation, route, ratio, and issuer hash. Whole-record equality for caps.
4. Five action classes. Class `2` promotes `submitted → enqueued → open_duty|delivered`. Class `3` promotes through source finality and destination evidence. A signature, a queue log, or a source transaction does not promote either class to `finalized`.
5. Signer method is `authorizeSign` of schema 7 against the grant. Released signatures occupy the leg. `unknown` blocks a second ordinal.
6. Direct Midnight is `execution_mode = 1` and an empty grant list. Kernel methods reject that mode and do not fail the program.
7. Refunds require conservation under one `claim_id` and either a complete delivered total or a class-5 absence for the refunded part.
8. Evidence classes do not fall back. Operator receipts are not evidence.
9. Aeon 4.9.0 at `ef66bd95` is excluded from `validatePlan` and from settlement.

Custody ownership, whether a foreign fact is an in-circuit proof or a named quorum, and the first external adapter remain open. MKAP-1 carries those choices as `issuer_policy_hash`, `evidence_allow`, and `adapter_code_hash`. It does not infer them from NEAR or Hyperliquid.