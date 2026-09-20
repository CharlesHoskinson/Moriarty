---
title: "MPLR-035: Constraint-preserving solver completion"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, anoma, pl-theory]
---

# MPLR-035: Constraint-preserving solver completion

## Required behavior

When a solver completes, combines or routes a partially specified authenticated intention, the language and acceptance relation shall establish that the completion refines every applicable signed constraint and permitted choice, binds all introduced effects and disclosures, and preserves existing commitments and residual duties. An out-of-scope change requires a separately authorized amendment. No developer or solver membership list may replace proof checking.

Precommit candidates remain distinct from accepted partial stages. Failure to complete a candidate cannot roll back already committed external effects. A solver may choose any solution permitted by the signed policy; optimality and eventual completion require separate claims.

## Positive and hostile witnesses

Accept an unregistered solver's compatible route exchanging 10 A for at least 20 B at the signed recipient, with a fee no greater than 1 A within the aggregate 11-A debit cap. Accept an alternative route allowed by the same constraints. Preserve explicit obligations when settlement spans stages.

Reject independently mutated extra fee legs, changed recipients or fee denominations, conflicting settlement domains, substituted policy circuits, prohibited disclosure, and silently dropped residual duties, even when transaction balance and cryptographic proofs otherwise pass. Distinguish an incomplete candidate from a committed prefix. Prevent incompatible settle/refund branches consuming the same entitlement.

## Theory research

Investigate refinement/effect systems for solver-filled holes, authenticated constraint intersection, linear resource/obligation semantics, concurrent quantitative budgets and relational privacy. Define sound completion relative to authenticated policy, evidence and state; prove arbitrary accepted witnesses preserve constraints. Establish non-vacuity with useful allowed completions and bind the relation to actual Midnight ZKIRv3 effects.

## Relationship and evidence

This relation complements [MPLR-019](MPLR-019.md) consent, [MPLR-023](MPLR-023.md) mandatory-condition enforcement and [MPLR-031](MPLR-031.md) delegated authority. Those rules alone do not define how solver-filled holes or combined signed solution sets refine an intention. See [Anoma synthesis](../anoma/explanation.md), [source evidence](../anoma/reference.md) and [research plan](../../../deliverables/anoma-study-2026-09-19/RESEARCH-PLAN.md).

## Status and roadmap

Research-draft, allocated 2026-09-19 after Anoma source study. Not an implemented feature or proved theorem. Feed the reference relation and positive/hostile witnesses into the existing kernel-security K0–K2 specification/model/target correspondence work. Product acceptance still requires concrete OpenSpec/EARS traceability and a bounded Pel execution package before implementation.


## Native proof-stack boundary

User clarification, 2026-09-19: implement solver-completion constraints through Midnight native proving and verification on pinned ZKIRv3. Lean is not a dependency. Preserve the semantic and compiler-correspondence obligations without prescribing a separate prover backend.
