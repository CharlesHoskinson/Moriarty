---
title: "APSS permission: explanation"
diataxis: explanation
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# Permission in a permissionless financial language

This explanation asks how Moriarty can let every Midnight developer deploy programs while ensuring that each program uses only the authority its users actually grant. It is a research synthesis, not implemented Moriarty semantics. Source IDs resolve in [reference.md](reference.md) and `sources.json`; proposed requirements are separately identified there.

## Permissionlessness and authority answer different questions

Permissionless deployment answers who may introduce a program. Authorization answers which effects that program may produce on somebody else's assets, information or liabilities. The former must not require a Moriarty administrator, reviewer, campaign receipt, hosted API account or preferred solver. The latter requires precise machine-verifiable conditions. A developer can deploy a borrowing program without being authorized to create debt for Alice.

CAKE places key control and delegated signing in its Permission layer, between application intent and execution coordination. It describes alternatives, rather than establishing a single mandatory custody model. Its claims about ideal private execution and a cross-chain tradeoff should be read as architecture arguments, not universal cryptographic theorems. Moriarty should borrow the separation of responsibilities without importing wallet custody choices or a new cross-chain backend. [P01: CAKE, Permission Layer](https://frontier.tech/the-cake-framework).

A useful proposed judgment is `Authorized(authorityState, grant, candidateEffects, context)`. A signature authenticates a particular message under particular key rules. The judgment additionally checks what the message means, whether authority remains current, what has already been consumed, and whether the proposed effects satisfy the grant. An old proof that this judgment held for yesterday's state does not establish it for today's state. Contract-based signature validation can itself be time- or state-dependent. [P03: ERC-1271, Specification](https://eips.ethereum.org/EIPS/eip-1271).

## Permission should describe effects, not just entry points

A callable method is an interface boundary; an acceptable financial outcome is an economic boundary. A session key that may call `execute` can be harmless or catastrophic depending on how `execute` interprets arguments, dispatches to other contracts, creates debt and charges fees. Permission to move a token must not silently imply permission to borrow the same nominal amount, modify collateral rights, disclose confidential positions or install a new authorization policy.

Rich Authorization Requests provide a useful non-blockchain example: structured authorizations distinguish actions, resource locations and domain-specific amounts. They also expose a composition hazard: multiple values in common fields authorize their Cartesian product; multiple objects can express a narrower pairing. A Moriarty permission format must say whether collections combine by conjunction, union, product or exclusive choice. Otherwise a harmless-looking list can enlarge authority. OAuth's authorization-server architecture is not a proposed central authority for Moriarty. [P07: RFC9396 §§2–2.2](https://www.rfc-editor.org/rfc/rfc9396.html#section-2).

The proposed financial envelope therefore needs asset identity and denomination, fee-inclusive gross debit, net recipient outcome, liability creation/change, permitted state transition, confidentiality scope, temporal validity and cumulative use. User-approved route freedom can coexist with a fixed recipient and hard net minimum. Route choice need not require a fresh human click when existing authority covers it.

## Delegation must preserve a semantic subset

A delegated permission should allow no candidate rejected by its parent, under the same authenticated context and consumption state. This is a semantic implication, not a test that one numeric field is smaller. Narrowing a recipient list while introducing an unrestricted callback may increase practical authority. Sharing a global budget among children requires accounting for their joint consumption; individually bounded children do not automatically preserve the parent's aggregate limit.

Macaroons demonstrate delegation by adding contextual restrictions and discuss freshness/state-based revocation. Their HMAC construction is designed around a target service that knows a root secret; it is not directly a public ledger verification scheme. The transferable lesson is monotone restriction and explicit context, not replacing Moriarty's proof or signature mechanisms with bearer tokens. [P09: Macaroons, pp.2–3, 9–10](https://research.google.com/pubs/archive/41892.pdf).

The current ERC-7715 draft describes wallet grant negotiation, expiry rules, retrieval and revocation. It allows returned permissions to differ from those requested and uses “attenuate” to include increases. Moriarty should use separate vocabulary: **the owner may approve a different grant**, while **a delegate may narrow an existing grant**. Increasing authority requires a new root authorization, not relabeling a delegated increase as attenuation. A successful wallet revocation response is insufficient evidence of the effective revocation boundary on an independent chain. [P06: ERC-7715, Permission Types and request/revoke methods](https://eips.ethereum.org/EIPS/eip-7715).

## Signing the right bytes and preventing repeated effects are separate

EIP-712 supplies typed encoding and domain separation but explicitly leaves replay protection to the application. The intended action, its limits and its domain must be bound, and accepted execution must consume or advance the appropriate replay state. A nonce field without a corresponding uniqueness rule is only data. [P02: EIP-712, Abstract and Security Considerations](https://eips.ethereum.org/EIPS/eip-712).

DPoP offers a different warning. It binds possession of a key to a token and request method/URI, but does not cover the general request body or headers. A transport token that proves who sent an API request therefore cannot stand in for a signature on a complete financial intent. Its discussion of duplicate proof tracking also shows why replicas that do not share replay state have weaker guarantees. These HTTP mechanisms are comparison cases, not a proposal to route ledger authorization through OAuth. [P08: RFC9449 §§11.1, 11.7](https://www.rfc-editor.org/rfc/rfc9449.html#section-11).

Revocation is similarly temporal. A confirmed revocation may block later authorizations while earlier accepted obligations persist. Canceling future drawing authority must not erase an existing debt or reverse a finalized payment. Concurrent fill and revocation must have a defined ledger order; a wallet-local logout is not that order. State-dependent verification in ERC-1271 and delegation migration hazards in EIP-7702 reinforce the need to bind policy and state rather than caching a permanent “signature valid” bit. [P03](https://eips.ethereum.org/EIPS/eip-1271), [P05: EIP-7702, Security Considerations](https://eips.ethereum.org/EIPS/eip-7702#security-considerations).

## Programmable accounts do not eliminate enforcement or trust

ERC-4337 separates account validation, bundling, execution and optional fee sponsorship. Its mechanisms illustrate how owner policy can be programmable while a transaction submitter remains a replaceable service. Its validation, simulation and resource rules are specific to its execution environment. They do not prove that a Moriarty account adapter faithfully implements a user's financial intent. A paymaster may fund execution without acquiring permission to change the user's recipient or net outcome. [P04: ERC-4337, interfaces, paymasters and validation](https://eips.ethereum.org/EIPS/eip-4337).

EIP-7702 is especially important as a counterexample to harmless-looking delegation. Its account code delegation is persistent, and its security discussion identifies replay, initialization and storage-migration hazards. A session capability and an account-code delegation are not interchangeable scopes. A permission UI must disclose policy-changing authority separately from a one-action allowance. No EVM mechanism is being proposed as a Midnight backend. [P05](https://eips.ethereum.org/EIPS/eip-7702).

## Threshold signing protects keys under assumptions; it does not prove intent

NEAR Chain Signatures describes deriving external accounts from a NEAR account, path and MPC key, then requesting signatures from a distributed service. It distinguishes outbound signing from learning authenticated external-chain state, and warns about cross-chain signature reuse. Its current service-size and signature-domain statements are deployment-sensitive documentation, not a security theorem verified in this study. [P11: Chain Signatures, How It Works](https://docs.near.org/chain-abstraction/chain-signatures).

FROST formalizes a threshold signing protocol with explicit assumptions about generated shares, corrupted participants and communication. Its security section excludes robustness and metadata protection as intrinsic goals, and leaves message-policy validation to the application. A threshold of honest cryptographic operations can faithfully sign a maliciously chosen transaction unless participants or the requesting contract enforce the relevant intent. FROST is an independent comparison; this study does not claim NEAR implements this exact RFC. [P12: RFC9591 §7](https://www.rfc-editor.org/rfc/rfc9591.html#section-7).

For Moriarty, key custody, signature correctness, grant validity, transition proof and settlement each need their own evidence. MPC availability cannot be inferred from unforgeability; a valid external-chain signature cannot be inferred to have been submitted, finalized or economically completed.

## Privacy authority is not spending authority

The current Zcash specification distinguishes spending, proof-authorizing and viewing components. Its key-derivation diagrams and Orchard action structure separately expose authorization signatures, nullifiers, encrypted outputs and zero-knowledge proof fields. This is a concrete example of different cryptographic authorities within one transaction system. It does not establish an identical Midnight key model. [P10: Zcash specification, PDF pp.14–15, 45](https://zips.z.cash/protocol/protocol.pdf).

The Moriarty design inference is to give read/disclosure, spend, prove, recover and administer distinct rights. Allowing an auditor to inspect a position should not grant the ability to transfer it. Conversely, withholding a spending key need not keep balances private. Disclosure can be irreversible: revoking future access does not make a recipient forget plaintext already obtained. A privacy statement should name observers, information revealed, duration and re-disclosure assumptions rather than use “private” as a global boolean.

## The design boundary

The strongest counter-position is to start with a single owner signing an exact bounded action, and defer delegation, sessions and multi-party signing. That minimizes current proof obligations and may be the right initial profile. It does not justify baking permanent owner-only assumptions into the language or making reviewers the owners. Reserve semantic space for independently specified authority policies while implementing only what can be accurately enforced.

A first useful Permission design should freeze the grant/effect relation, exact signed representation, domain/replay state, aggregate consumption, revocation order and disclosure distinctions. Later adapters may support different wallet/key arrangements without replacing these invariants. Neither this research nor an adviser consensus proves them implemented. Compile-time reasoning establishes conditional predicates; runtime/ledger checks enforce currentness and unique consumption; finite tests challenge implementations; retained finalized effects establish particular executions.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
