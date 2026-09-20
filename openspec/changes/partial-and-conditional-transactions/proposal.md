# Why

**Consolidated schedule, 2026-09-19:** This file retains detailed requirements and historical planning. The [single U0–U7 roadmap](../../../ROADMAP.md) and [consolidated proposal](../consolidated-language-kernel/proposal.md) control current design and scheduling. Resolve former P/C/K owners through [traceability](../consolidated-language-kernel/traceability.md); do not dispatch this as a separate queue. Objective requirements and evidence remain in force.


Moriarty must express transactions that progress through several accepted stages and transfers that wait for specified conditions. Local atomic checks or a successful verifier call do not establish downstream settlement. NEAR exposes concrete receipt, callback, partial-fill and escrow patterns; the user requires these capabilities in Moriarty on Midnight.

# What Changes

- Add explicit staged/partial transaction and conditional-settlement requirements.
- Bind combinations of signatures, documents, proofs and recipient actions to the submitted request and delivery conditions.
- Preserve per-stage authority, complete effects, fees, late results and residual duties through recovery.
- Establish MPLR theory-log IDs and trace them to EARS scenarios and a bounded internal Pel implementation plan.

# Capabilities

## New Capabilities
- `partial-conditional-transactions`: checked multi-stage programs and evidence-conditioned delivery.

## Modified Capabilities

Existing permissionless intention, financial accounting and target-correspondence obligations remain. Conditional settlement refines MOR-003/004/006/008/009/010/011/012; it does not waive them.

# Impact

The compilation target stays Midnight ZKIRv3. No NEAR runtime, admin policy or bridge trust model is imported. This documentation package specifies behavior; no new compiler, proof or deployed transaction behavior is claimed.
