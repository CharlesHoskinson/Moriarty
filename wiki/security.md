---
id: moriarty.security.boundaries
type: security
title: Moriarty security and trust boundaries
status: active
updated_at: 2026-09-09T22:50:07Z
sources:
  - SRC-0110
  - SRC-0104
  - SRC-0100
  - SRC-0101
  - SRC-0102
  - SRC-0103
  - SRC-0005
  - SRC-0007
created: 2026-09-02
updated: 2026-09-09
tags:
  - moriarty
  - research
---

# Moriarty security and trust boundaries

| Threat | Asset and attack path | Required control | Residual risk |
|---|---|---|---|
| Malicious witness | Local TypeScript supplies fabricated private input | Circuit constrains commitments, signatures, units, ranges, freshness, and authorization | Compromised local secrets and selective withholding remain possible |
| Accidental disclosure | Private-derived data reaches public state/output/cross-call | Moriarty visibility types plus Compact information-flow check; generated `disclose()` reviewed in manifest | Traffic analysis and intentionally disclosed correlations remain |
| Compiler mistranslation | Core meaning changes in generated Compact or ZKIR | Independent Core interpreter, translation validation, differential traces, pinned reproducible toolchain | Compiler and validator bugs until proof/cross-check coverage is complete |
| Invalid initial state | Constructor violates positivity, bounds, or accounting | Total well-formedness checker and constructor assertions; reject deployment manifest | Backend mismatch could bypass incomplete checks |
| Token or unit confusion | Same integer used for different assets or time | Token-indexed amount and distinct time/duration types before lowering; runtime manifest checks | Compact/ZKIR erase some domain distinctions, so translation is trusted until validated |
| Authorization replay | Reuse of old proof, capability, or oracle statement | Contract/entry-point domain separation, sequence numbers, nonce/nullifier, deadline | Wallet/key compromise remains |
| Oracle manipulation | Stale, equivocated, replayed, or unit-confused statement | Signed typed observation: source, feed, unit, timestamp, freshness, sequence, bounds, fallback | Source can still lie; governance and dispute paths remain external |
| Continuation loss | Hashed future branch unavailable | Replicated content-addressed storage, preflight availability proof, participant export | Integrity does not create availability |
| Runtime substitution | Planner offers unintended transaction or artifact | Client recomputes Core hash, entry point, public effects, destinations, fees, versions, and artifact hashes before signing | Compromised wallet UI can still deceive the user |
| Backend version skew | Compact, runtime, ZKIR, ledger, or keys disagree | Lock every version and artifact hash; conformance matrix; no implicit “latest” | Emergency ledger upgrades can remove an execution path |
| Resource exhaustion | Bounded source expands into impractical circuit or ledger state | Predeployment limits on Core size, transition rows, proof memory/time, state cardinality, transaction size | Estimates need safety margin and live-network validation |
| External contract call | Unreviewed contract violates value or liveness assumptions | Exclude from V0; later capability manifest, allowlist, effect summary, separate audit evidence | External behavior cannot inherit Moriarty guarantees |
| Governance capture | Spec or registry changed without review | Multi-party ownership, public MIPs, reproducible releases, delayed activation, signed registries | Social-layer collusion cannot be eliminated technically |

Independent audits are split by the normative Core and proofs; parser/type
checker/elaborator; Compact backend and translation validator; generated
circuits and artifact registry; Runtime/client verifier; SDK/UI; and optional
oracle/composition protocols. A single “Moriarty audit” is not an adequate
claim for all of these boundaries.

## Taxonomy-driven adversarial requirements — 2026-09-08

**CLM-0928.** Zhou separates network, consensus, smart-contract, protocol-design and auxiliary dependencies from attacker knowledge and capabilities; its incidents are multi-label. Werner's technical/economic distinction concerns atomic exploitation versus non-atomic economic exposure, not simply code versus finance. Sources: SRC-0102 (IEEE SP 2023), PDF pp. 2–6 Fig. 2 and Tables I–III; SRC-0103 (arXiv v6, 2022-09-15), PDF pp. 6–11 §§5–6. Primary descriptive research; reviewed 2026-09-08; source facts, S2 research use; not empirically reproduced; confidence high for source definitions. [Detailed analyses and graph](../deliverables/defi-taxonomy-papers-2026-09-08/README.md).

**CLM-0929.** Derive adversarial fixtures with separate actor capability, vulnerable layer, precondition, ordered effects, violated predicate and loss outcome. ABI shape does not establish semantic compatibility; a valid oracle signature does not establish economic truth. Flash borrowing is a capability with legitimate uses. Sources: SRC-0102, PDF pp. 3–6 and 12 §VI; SRC-0103, PDF pp. 4–9 §§3.2,4–5 and p. 11 §6.4. Primary descriptive research; reviewed 2026-09-08; inference/recommendation, S2, not implemented; confidence medium. [Proposed tests](../deliverables/defi-taxonomy-papers-2026-09-08/DESIGN-IMPLICATIONS.md) separate compiler/Core checks, proof and ledger correspondence, and external assumptions. No survey result certifies Moriarty security.

## Vault valuation and claim boundaries — 2026-09-08

**CLM-0933.** The [vault report comparison](../deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md) proposes separate valuation roles, explicit conversion/preview/limit/execution semantics, complete direct-transfer and rounding accounting, persistent request duties and versioned dependency/authority rules. Six proposed tests distinguish local share accounting from downstream collateral and recovery assumptions. Source: SRC-0104, lines 81–100, 125–147, 246–325; report as-of 2026-09-08; secondary synthesis; reviewed 2026-09-08; S2 inference/recommendation, not implemented or reproduced; confidence medium. Existing local ERC-4626/7540 snapshots support a limited method/lifecycle comparison; other external incident, standard-status and theorem assertions remain report claims. The four mandatory proof obligations retain their explicit domains and assumptions.

## Security-token policy paths — 2026-09-09

**CLM-0945.** The report proposes policy-path completeness: ordinary transfers and exceptional issuance/burn, recovery, liquidation, wrapper and migration paths must each apply their defined policy and scoped authority. A successful deposit does not establish a safe withdrawal or liquidation path; custody and receipt transfers can change who holds the economic claim. Source: SRC-0110, lines 365–413 and 447–473; report date 2026-09-09; secondary descriptive synthesis; reviewed 2026-09-09; S2 source argument and test recommendation; not implemented or reproduced; confidence medium. A cryptographic credential proves its declared predicate under issuer/freshness assumptions, not the truth of external facts or legal compliance.

[Proposed AT01–AT08 cases](../deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md) cover wrapper restrictions, partial encumbrance/liquidation, pending redemption, scoped recovery, record dates, stale policy/migration, multi-asset batches and private eligibility. Bind asset domains and policy versions, preserve residual obligations, and distinguish a settlement hold from a freeze. Forced recovery is a new authorized transition, not deletion of history. All eight cases remain unexecuted proposals.

The report's CIP-0113 mixed-policy, unfracking and third-party-action concerns remain contested assurance questions. Its current status, audit, deployment and legal assertions have not been independently verified in this intake. Opaque citation markers and two missing sandbox catalogs prevent treating its claimed primary-source review as our own evidence. [Full source and graph](../deliverables/security-token-transformations-2026-09-09/README.md); [[wiki/moriarty-architecture#Assets, claims and transformations — 2026-09-09|language and agenda fit]].
