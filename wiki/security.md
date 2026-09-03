---
id: moriarty.security.boundaries
type: security
title: Moriarty security and trust boundaries
status: active
updated_at: 2026-09-02T18:20:00Z
sources:
  - SRC-0005
  - SRC-0007
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
