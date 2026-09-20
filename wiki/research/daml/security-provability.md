---
title: "Security model and PCD requirements"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, daml, research]
---

# Security model and PCD requirements for Moriarty

Research design, 2026-09-19. This document states proposed obligations, not discharged theorems. It supplements the existing product contract and will be reconciled with the three Daml studies and Simplicity evidence.

## What the language is intended to guarantee

For a declared supported profile, an accepted Moriarty transition must realize the signed formal intention under explicitly recorded assumptions. Its proof must bind the actual executed Midnight ZKIRv3 artifact and the actual ledger effects. Safety properties exclude defined bad outcomes; confidentiality requires a separate observation model; progress requires additional availability, inclusion, liquidity and finality assumptions.

Turing incompleteness is a design constraint that can support bounded execution and tractable analysis. It is not a proof of business correctness, oracle truth, absence of leaked metadata, or eventual settlement. Bounded execution does not make arbitrary property inference cheap, and termination of each transaction does not terminate a business workflow spanning many transactions. PCD must establish an inductive statement about the accepted history, not merely a chain of signatures or hashes.

## Adversary and trust boundaries

Assume attackers can construct programs within the public language, propose transactions, choose all witness data, run modified provers, compose valid-looking predecessor proofs, reorder/duplicate/delay messages, exploit concurrent spends, select versions where permitted, and submit misleading external attestations. Assume neither the compiler nor an honest witness generator is the sole source of candidate artifacts. Any trusted component must be named and pinned. Cryptographic assumptions, concrete Midnight verifier/ledger behavior, and external chain/attestor assumptions remain explicit.

Application-selected controllers and counterparties are permitted. There is no Moriarty project approval or registry membership condition on deployment or program validity.

## Proposed PCD statement and local relation

A versioned public statement commits, as required by the privacy policy, to the program and property, source semantics, target artifact and verification relation, signed intent and amendment policy, execution domain, predecessor state/proof references, consumed resource identities, observed external evidence, complete resulting effects, residual obligations, nonce/replay domain, and cumulative resource bounds. Some components may remain hidden behind binding commitments, with equality/consistency proved in the circuit. Public visibility of every component is not required.

A local compliance relation must establish:

1. Base case: initial state is legitimate and satisfies the invariant; a prover cannot choose an arbitrary already-approved/funded state.
2. Recursive case: predecessor proofs verify under the intended relation, not arbitrary prover-selected verification keys. Parent statements match the consumed state, contract, intent, version and domain.
3. Authorization: each action has sufficient scoped consent, and no nested helper gains unrelated authority or creates an unconsented obligation.
4. Transition semantics: the certified source transition and actual target execution agree on success/failure, all observable effects, and state changes.
5. Financial invariants: asset quantities, ownership, liabilities, fees, reservations and residual duties each obey their own accounting equations. A refunded amount cannot erase a completed remote delivery or discharge a different liability.
6. Composition: branches and joins account for every consumed resource and all duties. Reusing a proof does not authorize reusing a spendable resource. Ledger-enforced uniqueness and finality are distinct premises that PCD must reference, not assume it creates.
7. Evidence: signatures, documents and external proofs are bound to the exact predicate, subject, domain and freshness policy. Cryptographic authenticity is not real-world truth.
8. Bounds: every accepted stage has a checked bound under a pinned cost model, including late-bound components and cumulative obligations where claimed.
9. Privacy: the agreed disclosure policy covers commitments, public inputs, branch structure and allowed metadata. Hidden data does not become unconstrained data.

The intended induction is: valid base evidence plus sound local compliance and sound recursive proof verification imply the specified invariant for every accepted composed transition. The theorem remains conditional on the external premises and the actual ledger enforcing its part of the relation. A proof of a weaker relation cannot establish a stronger intention.

## Attack-to-obligation matrix

| Attack | Required exclusion property | Language/proof consequence |
|---|---|---|
| Direct creation of an already-approved state | Valid state provenance, not just valid current fields | Phase/state introduction rules; base-case invariant and lineage proof |
| Authority laundering through a nested call | Scoped, non-ambient consent | Effect/authorization judgments and explicit delegation policy |
| Recipient obligation injection | Consent to each material obligation | MPLR-019, distinction between positive receipt and duty |
| Double spend or replay across branches/chains | Single-use resources and domain separation | Ledger uniqueness plus proof-bound resource IDs/nonces; composition rule |
| Partial-fill overpayment or refund after delivery | Complete prefix/residual accounting | Resource/obligation types and inductive conservation |
| Cancel/settle race or timeout treated as failure | Outcome precision under concurrency | Explicit pending/known-success/known-failure states and authenticated finality |
| Upgrade or late-bound code changes signed semantics | Intent-preserving evolution or explicit reauthorization | Version-bound code identity, semantic compatibility evidence and cost checks |
| Wrong circuit, wrong verifier or unconstrained witness | Adversarial accepted-witness soundness | Artifact-bound relation and compiler/constraint correspondence; WShape obligations |
| Boolean check computed but ignored | Required condition dominates acceptance | Explicit assertion/refinement effects and negative acceptance witnesses |
| Overflow, rounding, scale or fee ambiguity | Exact arithmetic/financial semantics | Bounded numeric types, units and checked side conditions |
| Oracle equivocation or stale documentary evidence | Correct policy evaluation under declared trust | Evidence types, freshness/revocation, explicit external assumptions |
| Data/metadata leakage through composition | Stated observation policy | Information-flow/projection analysis plus target-specific ZK proof |
| Exhaustion or unavailable continuation | Claimed resource safety; conditional progress | Stage bounds, reserve accounting, recovery policies; separate liveness theorem |

## How this sculpts the language

Start with a small total, typed semantic core and versioned certified primitives. Permit richer capabilities only when their semantics, resource behavior and composition obligations are represented. General recursion, hidden I/O, ambient authority, implicit overflow and unbounded dynamic code are poor defaults for the certified profile. Bounded iteration, explicit staged continuations, typed evidence, certified extensions and off-chain search can provide expressiveness without putting arbitrary execution into the acceptance relation.

This is a research recommendation, not a claim that every listed restriction is already necessary or sufficient. Compare alternatives using the same hostile traces. Define what the language rejects statically, what a proof discharges, what the runtime checks, and what remains an assumption. A failure to establish the claim must not be silently downgraded to an advisory result while being presented as proven.

## Evidence plan and success tests

Require positive witnesses proving the profile is usable, negative/adversarial witnesses detecting the excluded attacks, and mutation tests for statement binding and constraint completeness. Model the workflow before implementation, then prove base case and preservation and establish the compiler/target/ledger bridge. Existing ZKIR PR17 conditional results do not remove its adversarial WShape and concrete backend obligations. Native tests, model checks, formal proofs and live-ledger observations must retain different assurance labels.

Daml supplies counterexamples about authority, state introduction, disclosure and upgrades. Simplicity supplies a bounded reference core, semantics-oriented jets and commitment/resource distinctions. NEAR supplies asynchronous failure and partial-effect cases. None by itself provides Moriarty's end-to-end guarantee.

## Refinements from independent source review

Disclosure-policy checking in one transition is only a local condition. A confidentiality claim also needs a relational observation-security theorem (and the appropriate cryptographic zero-knowledge argument), with errors, resource usage, network and hosting observations either covered or explicitly declared leakage.

Where absence, uniqueness, lack of revocation, or completeness of liabilities matters, bind an authenticated state domain and prove its frame/completeness or nonmembership property. A prover-selected subset or private lookup returning None is insufficient. State which authority or ledger establishes that domain's completeness.

Choose a sequence or DAG history semantics and enforce well-founded composition, legitimate base provenance, parent compatibility and correct joins. A cyclic proof reference or arbitrary already-funded base state cannot bootstrap validity. This is additional to actual ledger prevention of conflicting resource consumption.

Version migration of persistent continuations must preserve residual duties, recovery rights, disclosure constraints and resource semantics, or obtain the explicitly required amendment authorization. Schema/interface compatibility alone is insufficient.


[Study index](index.md) · [Kernel boundaries](kernel-boundaries.md)
