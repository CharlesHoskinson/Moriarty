# Requirements: programmer-facing Moriarty language mockup

**Status:** requirements for the next session. This document requests a design mockup, not a grammar freeze, implementation claim, or ledger admission claim.

## Goal

Show what a programmer would write and read in a complete Moriarty financial agreement. Work from the author experience back toward the formal language and DeFi kernel. A reader should understand the intended language without first reading K, Quint, wire formats, or audit files.

The mockup must show the full proposed feature set, including features outside the current executable S0 slice. Every example must mark which constructs are **implemented locally**, **specified only**, or **open**. No example may imply that parsing, type checking, proof generation, signature validation, bridge finality, or ledger admission exists when it does not.

## Required artifact

Create one coherent, versioned language mockup with:

1. A short syntax tour for programmers: declarations, types, expressions, agreement structure, actions, stages, outcomes, and errors.
2. Complete, readable source examples for all eight DeFi families below, plus one composed example that crosses at least two families.
3. A compact reference table for every visible construct, its meaning, its status, and its intended Core/kernel boundary.
4. A side-by-side trace of one example through source intent, completed plan, typed Core request, kernel calls and evidence, prepared effects, and committed or rejected result.
5. A separate list of unresolved language choices raised by the examples, with a recommended choice and the formal obligation it creates. Keep unresolved choices visible in the source examples through explicit annotations; do not silently invent settled syntax.

Use the existing Source/6 and Core/5 S0 contract as the only locally executable baseline. Reuse Moriarty's terms when they have stable meanings: intent, completion, agreement, episode, stage, obligation, encumbrance, authority, observation, effect, duty, and recovery. New surface syntax may be proposed to improve readability, but it must include an explicit mapping to the existing terms and identify any changed semantics. The mockup is a successor candidate, not an automatic expansion of Source/6.

## Language surface to show

The syntax tour and examples must cover these programmer-visible needs:

| Area | Required visible behavior |
| --- | --- |
| Identity and assets | Nominal agreement, episode, stage, action, program, domain, asset, pool, share class, and instrument identities; asset representation and scale without treating symbols as identity. |
| Values and computation | Typed quantities, deltas, positions, prices, clocks, windows, checked arithmetic, rounding choices, conversion direction, and the boundary between the simple predicate profile and advanced certified arithmetic. |
| Agreements and intent | Parties, signed bounds and outcomes, fixed terms, solver-fillable holes, completion constraints, validity, nonce, policy commitment, and acceptance refinement of a signed template. |
| State and time | Explicit `pre` and `post` references, typed read and write cells, derived complete footprints, one-domain stage transitions, and ledger-linked multi-stage episodes. |
| Authority | Who may issue, fill, enforce, amend, recover, sign, or veto; scopes, expiry, revocation, work and allowance limits, and multi-signer requirements. |
| Evidence | Observation value, unit, source, observed time, freshness, finality and provenance; selected program and authenticated reads; rules for missing, stale, or conflicting evidence. |
| Outcomes | Ordered economic effects, conservation and liability changes, retained effects, pending duties, success, rejection, timeout, recovery, and terminal settlement. |
| Composition | Reusable libraries or profiles, named operations, cross-category composition, and explicit interfaces to external protocols. Show which operations can be atomic and which require an episode. |

The examples must make error behavior legible: a rejected stage has a named first failure and no published financial effects; a pending stage has explicit duties and a later continuation or recovery path. Do not use a generic fallback that evaluates only part of the signed relation.

## Eight required DeFi examples

Use the same family names as the [category map](../../concepts/intent-language/CATEGORY-MAP.md) and [MIL/2 coverage review](../../deliverables/mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md). Each example must show the desired user outcome, signed limits, state read and write set, kernel operations, evidence, rounding or loss beneficiary where relevant, failure path, and status of each required mechanism.

| Family | Minimum example and hard question to expose |
| --- | --- |
| AMMs and exchanges | Exact-input pool swap with fees, reserve invariant and slippage bound; show LP mint or redeem separately. Identify whether routed swaps and multi-party clearing need a new profile. |
| Lending and borrowing | Collateralized loan, funded repayment, and liquidation or default continuation. Show aggregate lock accounting and debt roll-forward. |
| Stablecoins and synthetic assets | Collateral-backed mint, burn or redemption, peg observation, and emergency settlement path. Show supply and backing obligations. |
| Derivatives | Fully collateralized cash-settled option with fixing, exercise and settlement. Identify the additional margin and funding rules needed for perpetuals. |
| Oracles and observations | Typed price observation with source, unit, time and status; show a stale or disputed read. Identify aggregation and verifier assumptions. |
| Governance | Policy change with authorization, timelock or veto, and protection of already signed duties. Keep governance process in a library if the language only expresses its effects. |
| Bridges and cross-domain settlement | Source escrow and destination claim as separate stages with paired identifiers, foreign proof and timeout recovery. Do not equate timeout with proof of foreign nonreceipt. |
| Staking, restaking and yield | Deposit-to-share, reward or slash, unbond request and withdrawal claim. Show share rounding, competing slash priority and pending withdrawal duty. |

## Intent language and DeFi kernel boundary

The programmer-facing source states the permitted outcome and constraints. The kernel owns concrete calls and evidence for balances, signatures, selected code and policy, observations, bridge proofs, chain state, execution, and atomic ledger updates. The mockup must show this boundary directly, with typed interfaces rather than a generic `call` escape hatch.

For each example, state which facts are authored, which values a solver may complete, which facts a provider must authenticate, which effects Core derives, and which side effects the kernel commits. A hash or caller-supplied effect list is never proof of the corresponding economic transition. Bridge calls, multichain signatures and asynchronous finality belong in the kernel interface and episode model; the language binds their permitted use and required evidence.

Use the [S0 contract](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md), [MIL/4 result](../../deliverables/mil4-k-quint-sprint1-2026-09-29/RESULT.md), and [kernel API recommendation](../../wiki-llm/kernel-api-recommendation-2026-09-29.md) as starting constraints. The current S0 implementation only prepares a local transfer or AccrualFirst repayment and returns `PreparedUnqualified`; it does not authenticate providers or commit to a ledger.

## Acceptance criteria for the next session

- A programmer can read the examples and identify the financial promise, authorized actor, amount limits, evidence source, state change, and recovery path without consulting the formal semantics.
- All eight examples use one consistent notation or explicitly mark profile-specific constructs; the composed example exposes any incompatibility.
- Every visible construct maps to a proposed type, Core operation or kernel interface. Every kernel call names its required authority and evidence.
- The mockup includes a coverage matrix for the surface features above and all eight families, with `local`, `specified`, or `open` status backed by repository links.
- At least one transfer and one repayment example are compared field by field with the existing Source/6 contract. Any changed syntax or meaning is called out as a successor proposal.
- The mockup identifies the smallest coherent programmer-visible profile that could be implemented next, while preserving the broader full-language vision.
- The document ends with explicit language decisions and formal obligations for K, Quint, elaboration, authentication, and atomic ledger consumption. It does not claim that a readable example is executable evidence.

## Session boundary

The next session should produce and review the mockup against these requirements. Implementation, grammar freeze, and new formal sprints follow only after the mockup exposes and resolves its language-level choices.
