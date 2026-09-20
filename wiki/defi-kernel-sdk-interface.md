---
title: "Optional DeFi routing SDK — corrected scope"
diataxis: explanation
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# Optional DeFi routing SDK

The [Moriarty product contract](../docs/MORIARTY-PRODUCT-CONTRACT.md) defines the permissionless language, compiler/prover interfaces and mandatory ZKIRv3 execution on Midnight. Every DeFi developer can deploy supported programs without project approval. The [multichain interface design](../docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md) is optional managed-service research, not the universal language interface.

The old `194f633` commit and [pre-audit note](research/apss/sdk-interface-before-audit.md) remain historical evidence. The September 19 corrections supersede their claims that exact assets, chains or trust assumptions need never reach developers or signing users.

## Boundaries

A router can discover plans and an application can simplify presentation. Signed intent must nevertheless bind concrete domain-qualified assets or explicit substitution predicates, recipients, fees, liabilities, expiry and permitted failure outcomes. A symbolic asset name is insufficient. A service-local adapter catalog does not govern deployment of independent Mori programs.

Correctness proofs concern the bound program and effects. External observation provenance is separate: an attestation remains an assumption, not proof that the external fact occurred. Finality and completion are conditional on the selected mechanism. Neither a reserve nor an available recovery transition guarantees network inclusion or completion.

Revocation has local and domain-specific effects. The service can stop new local work immediately; external revocation needs its actual acknowledgment/finality and outstanding-authority accounting. The system must retain pending or unknown exposure and cannot claim instantaneous universal revocation.

Midnight's guaranteed/fallible phase distinction must appear in the compiler correspondence and signed failure policy. A fallible failure can preserve earlier effects and fees. Report complete effects and residual obligations rather than advertising global rollback.

## Research navigation

[APSS bank](research/apss/index.md) separates Applications, Permission, Solvers and Settlement using Diátaxis. [Settlement reference](research/apss/settlement/reference.md) records phase semantics. [Permission reference](research/apss/permission/reference.md) records authority and revocation boundaries. [Six-expert consensus](../deliverables/aeon-study-2026-09-19/review/FINAL-CONSENSUS.md) supplies the reviewed design direction; it is not proof or implementation acceptance.
