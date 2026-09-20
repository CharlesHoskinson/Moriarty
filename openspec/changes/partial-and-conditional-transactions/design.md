# Context

**Consolidated schedule, 2026-09-19:** This file retains detailed requirements and historical planning. The [single U0–U7 roadmap](../../../ROADMAP.md) and [consolidated proposal](../consolidated-language-kernel/proposal.md) control current design and scheduling. Resolve former P/C/K owners through [traceability](../consolidated-language-kernel/traceability.md); do not dispatch this as a separate queue. Objective requirements and evidence remain in force.


The user defines contingent settlement as submitting a transaction to a destination whose delivery waits until a configured combination of additional signatures, documents, proofs, recipient signing or other supported conditions holds. **Conditional settlement with composable evidence requirements** is the proposed main term. Contingent settlement remains an alias; programmable escrow denotes the funded/locked variant.

# Goals / Non-Goals

Goals: first-class staged execution, partial fulfillment, persistent typed continuations, evidence-conditioned settlement, honest recovery and complete accounting. Programs remain permissionless and compile to a supported Midnight ZKIRv3 target.

This specification does not claim NEAR and Midnight share rollback, gas, callback or finality semantics. It does not claim documents or signatures prove external truth, that every pending transfer is refundable, or that a model vote proves implementation correctness.

# Decisions

A stage statement binds program/semantics, workflow/stage occurrence, predecessor, signed intent and condition policy, relevant state, authority, complete effects, residual duties, observations and cumulative work. The compiler must preserve the permitted trace under the actual Midnight phase layout.

Conditional submission, funding reservation, evidence collection, readiness and final delivery are separate observable facts. Supported policies may require conjunction, explicit alternatives or thresholds. A predicate must bind the exact request, asset, amount, destination, document/proof statement, issuers, freshness and applicable validity cut. A hash identifies document bytes; validating its meaning requires the specified computation, proof or attestation under explicit assumptions. Evidence accumulated earlier may require revalidation at release under the signed policy. Changes to conditions or destination require the defined amendment authorization.

Continuations bind unique occurrences rather than content hashes alone. Joins distinguish result availability from success and retain late-branch duties. An asynchronous callback uses its explicitly delegated stage authority; it need not manufacture a fresh originating signature. Each stage applies its own authorized consent, deadline, revocation, replay and fee policy.

Partial fills preserve per-fill and cumulative economic bounds, residual authority and remaining liabilities. Asset/custody/supply equations, authority quantities and liability evolution are separately typed. A timeout without reliable contrary evidence leaves an unresolved outcome. Refund uses still-controlled assets or enforceable claims; compensation is a forward action after a committed effect. Recovery may retain fees and unresolved duties. Race policies must prevent refund and late delivery from double-consuming value.

# Risks / Trade-offs

General history/correspondence proofs and the admissible observation relation are open. A target may not support a promised condition or rollback behavior. Such guarantees must remain explicit assumptions or fail validation; they must not silently weaken. Safety does not imply unconditional progress when a signer, document, solver, scheduler or funding source is absent. Private evidence needs both disclosure bounds and later witness availability.

# Migration Plan

Preserve current atomic profiles and their measured behavior as existing scoped evidence. Introduce a separately versioned staged profile only with its own semantics, proof relation and ZKIRv3 correspondence. Extend P0/P2/P4/P6/P7 through the task packages here. Keep all implementation tasks open until their exact evidence exists.

# Open Questions

The [MPLR theory log](../../../wiki/research/mplr/index.md) records behavioral needs separately from unsettled theory choices. Compare session/typestate, resource/authorization logics, transactional/compensation semantics and proof-relevant evidence types. Daml is the next study and may add MPLRs. No source language is adopted solely because it has a similarly named feature.
