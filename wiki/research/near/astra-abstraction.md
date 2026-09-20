---
title: "NEAR astra-abstraction expert study"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, near-teardown, research]
---

# NEAR abstraction and PL interface: independent Astra review

Reviewed 2026-09-19 by the panel member configured as `gpt-6-astra`, requested effort `medium`. This is a bounded source study, not deployed-state attestation. The 39 claims and exact source pins are in [claims.json](../../../deliverables/near-teardown-2026-09-19/studies/astra-abstraction/claims.json) and [references.md](../../../deliverables/near-teardown-2026-09-19/studies/astra-abstraction/references.md). The controlling target is [Moriarty's product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md): a permissionless language for all Midnight DeFi developers, compiling to ZKIRv3 with proof-carrying signed formal intentions, including partial and contingent transactions.

The preparation draft is substantially correct. Its central separation survives independent inspection: account authority, wallet signatures, MPC signatures, supported Intents accounting, solver proposals and hosted 1Click orchestration establish different facts. The material additions are receipt-local atomicity, detached effects, branch-specific refunds, duplicate MPC requests, implicit account keys, protocol pause authority, and the distinction between a quote deadline and a signed intent deadline. No inspected NEAR interface establishes Moriarty's general program-correctness relation.

## Explanation: authority and semantic boundaries

| Interface | Authority or evidence | Boundary that must remain visible |
|---|---|---|
| NEAR function-call key | Receiver/method permissions and gas budget; no attached NEAR | Allowed methods can still change economically meaningful application state. Invocation restriction is not outcome refinement. |
| Meta transaction | Signed sender, receiver, action list, public key, nonce and maximum block height | A relayer sponsors or carries the specified actions; it does not receive unrestricted plan authority. |
| Chain Signatures | Curve/domain-specific signature over supplied bytes under predecessor/path-derived key | The request does not decode and prove a foreign transaction's economics, broadcast it, or establish finality. |
| Intents MultiPayload | Authenticated application payload, permitted account key, nonce/deadline and concrete intent execution | Concrete token accounting is not a proof of every developer program. Implicit-account key identity can satisfy membership without prior explicit registration. |
| 1Click quote signature | Service origin and integrity of quote fields | Neither owner authorization nor execution proof. |
| 1Click signedData | Owner signature in a supported native format | Submission acknowledgment and destination settlement remain different stages. |
| Partner credential | Access to a hosted service and its route/fee policy | It is not owner authority or a Moriarty deployment prerequisite. |

A-keys/A-meta/A-mpcsign/A-engine/A-quotesig/A-signed1click/AA-implicit support these distinctions. The Message Bus explicitly permits other quoting mechanisms and direct indexing; the hosted RPC requires authentication (A-bus/A-busrpc). The Intents contract separately has a pause gate on `execute_intents` (AA-entry). Therefore “optional hosted gate” must not be generalized into “no administrative protocol controls.” Conversely, a deployment's pause or contract roles do not justify maintainer approval of every Moriarty program. Objective semantic/version selection and application governance require explicit scopes. A signed application policy may restrict counterparties or authorized recipients; permissionless language use does not require ignoring those restrictions.

The SDK's `getQuote` is a useful concrete proposer-defense pattern: it rejects wrong assets and the wrong fixed amount before comparing candidates, and compares integer values with `BigInt` (AA-quotecheck). Proposal ranking follows admissibility. Moriarty should preserve that ordering while proving its own stronger acceptance relation.

## Reference: exact signed forms and stages

NEP-413 accepts `standard: "nep413"`, `payload: {message, nonce, recipient, callbackUrl?}`, `public_key`, and `signature`. The message string parses to `signer_id`, `deadline` and flattened intent content. `recipient` supplies the verifying contract; the 32-byte base64 nonce comes from the envelope. The signature covers SHA-256 of Borsh `(2147484061u32, payload)`, where payload fields are message string, nonce, recipient, optional callback URL in declaration order. Preserve the exact signed message bytes; a semantically equivalent JSON rewrite can invalidate the signature. A-nepadapter/A-nephash.

ERC-191 accepts `standard: "erc191"`, a `payload` JSON string and recoverable `signature`; the contract does not need a separate public-key field. The string contains the full Defuse payload: signer, verifying contract, deadline, base64 nonce and intents. Its digest is `Keccak256(0x19 || "Ethereum Signed Message:\n" || decimal UTF-8 byte length || exact message bytes)`. The current signing guide requires raw `r || s || v`, with recovery byte 0 or 1, encoded with the key-type/base58 convention. A-ercadapter/A-erchash/A-signingdocs. This is not EIP-712 typed-data signing. Five other standards are enumerated by MultiPayload but not individually cryptographically audited here.

Current Chain Signature request arguments are `{path, payload_v2, domain_id}`, nested as `request` when calling the named Rust argument through JSON. `payload_v2` is `{"Ecdsa":"<32-byte hex>"}` or `{"Eddsa":"<bounded hex message>"}`. Legacy `payload` and `key_version` aliases are accepted, but each legacy/new pair is mutually exclusive. The contract derives the tweak from the immediate predecessor and path; a wallet behind an intermediary contract is not automatically the derived-key principal. ECDSA scalar and protocol compatibility checks are distinct from semantic transaction validation. A-mpcwire/A-mpcpayload/A-mpcsign.

1Click signs the Base58 representation of SHA-256 over stable-key JSON formed by spreading `quoteRequest`, then `quoteResponse`, then `timestamp`. Reproducing this exact object construction matters where fields overlap. Quote-origin verification does not authorize the user debit. A-quotesig.

| Stage | Bound object / completion fact | Expiry and retry meaning |
|---|---|---|
| Quote discovery | Assets, fixed-side amount, candidate price, service signature | Solver offer `expiration_time` is an offer lifetime. A dry quote lacks execution deposit/deadline fields. |
| Origin-chain funding | User-signed chain transfer to deposit address, plus memo when required | Detailed quote API says `deadline` starts refund if incomplete; it must accommodate deposit mining time. It is not proof that a late transfer never executed. |
| Generate signed intent | Quote deposit address, signerId, standard; unsigned `intent` plus correlationId | API documents target-state/ownership checks but no stable duplicate-generation contract. Inspect actual returned deadline and nonce. |
| Submit signed intent | `{type:"swap_transfer", signedData:<MultiPayload>}` | API returns intentHash/correlationId for submission. No reviewed guarantee makes repeated HTTP calls idempotent or the returned hash evidence of destination delivery. |
| Verifier execution | Contract identity, current key authority, nonce, deadline, concrete effects | Engine rejects only when deadline is strictly less than current time. Versioned nonce salt and embedded deadline add checks. Replay rejection is not the same as idempotent success response. |
| Async resolution | Each withdrawal/callback result and actual refund effects | Earlier acceptance expiry cannot erase already-created duties. Recovery uses the signed stage policy and evidence, not a universal “expired means undo.” |

A-busrpc/AA-refunddeadline/AA-generate/AA-submit/A-engine/AA-ftresolve support the table. The quickstart calls deadline “quote expiration”; the detailed API and older Rust schema instead describe refund initiation. Retain this documentation discrepancy. No separate lifetime for `generate-intent`/`submit-intent` was established from the captured schemas. Do not invent one.

Current `INTENTS` and `CONFIDENTIAL_INTENTS` deposits use generate/sign/submit; confidential origin-to-origin routing using `confidentiality` with `ORIGIN_CHAIN` still follows ordinary chain funding. The old Rust SDK lists only `ORIGIN_CHAIN` and `INTENTS`; it is not authoritative for current confidential integration. Current quote types also include `FLEX_INPUT`, whose partial deposits must meet signed/service bounds, and `ANY_INPUT`, a partner-restricted service feature. The SDK monorepo supplies independent Intents builders and solver-relay utilities; it must not be conflated with the old generated 1Click Rust schema. A-signed1click/A-quoteold/AA-types/AA-builder.

## How-to: model partial execution without losing duties

Trace the concrete contract boundary first. `execute_intents` calls the engine and panics on errors; simulation uses cached state. The engine authenticates payloads, checks verifier/key/deadline/nonce, applies signed deltas and finalizes matching. Its contract State adapter schedules and detaches withdrawal, notification and auth-call promises. Consequently, synchronous acceptance and downstream settlement require distinct observations (AA-entry/AA-detached).

The NEP141 deposit path is especially instructive. With `refund_if_fails=true`, it calls `execute_intents` inline. With false, it schedules a detached self-call after crediting the deposit. The same application request can therefore use different receipt boundaries and recovery behavior. This is source evidence of an explicit composition policy, not evidence of a global rollback transaction (AA-deposit).

The FT withdrawal resolver goes further: failed plain `ft_transfer` is treated as zero used and refunded internally; failed `ft_transfer_call` is treated as fully used, because downstream transfer may have happened before resolution failed. A successful but malformed transfer-call result has a distinct zero-used branch. Copying either branch into a universal external-effect policy would be unsound. The important requirement is to encode each adapter's evidence interpretation and surviving duty explicitly (AA-ftresolve).

The SDK callback example joins four promises with `.and(...).then(...)`, passing ordered `Result<T,PromiseError>` values into a private callback. `.callback_unwrap` assumes success instead. This is a concrete model for typed branch results; it does not prove all branches succeeded or reverse completed siblings. A-callback.

MPC adds a less obvious continuation lesson: identical request keys may have several queued yields. The pending-request accessor deliberately returns only a representative for a presence test, and one timeout removes one slot while siblings remain. A content hash therefore cannot always identify one obligation or one continuation. Distinguish semantic request identity, invocation/attempt identity, and externally accepted effect identity (AA-fanout).

The user’s precise contingent-settlement case is submission to a destination followed by a wait for a combination of signatures, documents, proofs, recipient action and other conditions. Model this as conditional settlement with composable evidence requirements, and as programmable escrow when assets are funded or locked. Recorded funding or destination receipt must remain distinct from final delivery. Documents and signatures need explicit issuer/content/validity predicates; merely attaching a document is not proof of its truth.

For Moriarty, the proposed continuation carries program/semantics version, execution domain, stage and attempt identity, authenticated predecessor evidence, remaining affine authority, persistent liabilities/duties, resource bounds, observation requirements, allowed transitions and expiry/revocation policy. This is an interface requirement, not a settled choice of type system. A continuation can be consumed once while its liabilities persist through the successor state. A join authenticates the required branch set, accounts for each branch's effects, and preserves pending or failed-branch duties. An `unknown` result cannot be weakened into “no effect.”

The lowerer must bind the actual Midnight guaranteed/fallible phase layout and surviving charges. Source-local rejection, receipt-local rollback and external compensation are separate relations. A valid proof must bind the particular allowed failure transition, fees, consumed authority and residual history. This applies to any developer-authored supported program, without a hosted solver, program registry or reviewer admitting it.

## Tutorial: contingent exchange with a late callback

Consider an unsigned design exercise: reserve 100 units of A; release at most 100 A after evidence of at least 200 B delivered to the specified recipient; otherwise pursue the authorized recovery path, with cumulative fees capped at 2 A. Asset identities, observation finality assumptions and liability bounds are explicit inputs. These numbers are illustrative, not a live transaction.

1. The owner signs a versioned formal intention that permits reservation, conditional release, partial outcomes and bounded recovery. The display identifies these effects and the conditions; natural-language paraphrase does not replace them.
2. Any proposer supplies a candidate program execution and proofs binding the owner intention, program semantics, predecessor, observations, complete effects and phase layout. A signature alone cannot satisfy these proof obligations.
3. Stage one reserves A and records an outstanding delivery-or-recovery duty. An affine spending capability can remain unused; the duty cannot disappear merely because the stage returned successfully.
4. A delivery attempt times out. The continuation becomes unknown, preserving reservation and duty. It does not mint a fresh release capability or authorize an unconditional second payout.
5. A late authenticated callback reports B delivery. The acceptance relation checks invocation identity, predecessor linkage, replay state, observation assumptions and the signed late-result policy. Depending on that policy and current state, it releases once or follows an explicit recovery transition.
6. If parallel delivery/refund attempts exist, the join accounts for both. A refund acknowledgment cannot erase already-finalized delivery; compensation requires its own authorized effects and funding.

The existing NEAR formats do not express this entire proof statement. In particular, `token_diff: {A:"-100",B:"200"}` specifies exact deltas, not an arbitrary minimum-output predicate; Moriarty must define and prove its own refinement semantics. The example requires usable liquidity and evidence availability for progress, but its safety claim must remain valid when either fails.

## Proposed EARS requirements and MPLR candidates

The machine-readable candidate requirements and adversarial scenarios are in [REVIEW.json](../../../deliverables/near-teardown-2026-09-19/studies/astra-abstraction/REVIEW.json). Root should assign stable MPLR identifiers and file them; these are proposed requirements, not implementation claims or completed theory decisions.

The highest priority research questions are how to compose affine authority with persistent duties; how dependent/session/effect/refinement types can describe authenticated continuations and joins; how partial-commit phase semantics refine source semantics; and how retry equivalence separates semantic request, attempt and settled effect identities. No one candidate type discipline is selected by this study. Public compilation, proof generation and deployment must remain available without project-process credentials.

## Verification, changes and limits

All 25 draft source-file hashes matched. The key snippets were independently inspected and 14 source-backed claims added. The NEP-413 repository test vector was reconstructed independently using Python Borsh-layout bytes and `cryptography` Ed25519 verification; its SHA-256 digest was `94648d7168d4a58d1eecd11fc2ecad210ef7cb5efce7a22d15a19e1764031f59`. An altered-byte preimage was rejected. This checks one signing vector, not all adapters or ledger correspondence.

Existing AST graphs were traversed using exact file seeds and one-hop neighborhoods; source text supplies semantic interpretation. No full repository build, deployed-byte verification, foreign-chain execution or financial transaction was performed. Current 1Click idempotency, server implementation, confidential settlement trust assumptions and alternate-signature test vectors remain open. The broad “automatic retries” prose is additionally qualified by the Message Bus `requote=false` default in the same documentation (AA-requote). Runtime model/effort attestation is not exposed by this tool session; the configured/requested panel identity is recorded without fabricating an API receipt.
