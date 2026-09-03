---
id: moriarty.port.decision
type: decision
title: Marlowe-to-Moriarty port decision
status: active
updated_at: 2026-09-03T06:48:47Z
sources:
  - SRC-0003
  - SRC-0004
  - SRC-0005
  - SRC-0006
  - SRC-0007
  - SRC-0009
  - SRC-0015
  - SRC-0016
  - SRC-0017
  - SRC-0019
---

<!-- markdownlint-disable MD013 MD025 MD060 -->

# Marlowe-to-Moriarty port decision

**Decision:** proceed with a 90-day bounded feasibility phase for Moriarty as an
ahead-of-time compiled, verified financial Core targeting Compact and ZKIR 3.
Do not yet fund or describe it as a production DeFi kernel.

The port is technically credible because Marlowe's finite continuation model
and Compact's static types, non-recursive circuits, fixed iteration, explicit
private witnesses, public ledger state, and ZKIR generation align unusually
well. The first escrow lowering compiled and produced three valid ZKIR 3 circuit
models. The port is not mechanical: Cardano UTxOs, role tokens, payout scripts,
validity intervals, and a universal validator do not map directly to Midnight's
state, credentials, Zswap/unshielded effects, block-time predicates, and
per-entry-point circuits.

## Architecture scorecard

Scores are 0–5. Columns use the controlling assignment weights: assurance 20%,
ledger practicality 15%, useful expressiveness 12%, authoring 10%, migration
10%, runtime 10%, demand 8%, maintenance 8%, and interoperability 7%.

| Option | Assure | Ledger | Expr | Author | Migrate | Runtime | Demand | Maintain | Interop | Weighted |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Compact library only | 2.8 | 4.0 | 3.3 | 3.3 | 2.0 | 3.8 | 3.1 | 4.2 | 3.6 | 3.30 |
| Universal interpreter circuit | 4.6 | 2.0 | 4.1 | 3.8 | 4.0 | 3.1 | 3.0 | 2.0 | 3.1 | 3.42 |
| **AOT verified Core to Compact** | **4.5** | **4.1** | **4.2** | **4.4** | **3.6** | **3.8** | **3.7** | **3.0** | **3.8** | **4.00** |
| Direct Core to ZKIR | 4.0 | 4.4 | 4.0 | 3.8 | 3.2 | 2.9 | 3.2 | 1.8 | 2.6 | 3.51 |
| Portable multi-backend Core | 4.1 | 3.5 | 4.2 | 4.0 | 4.0 | 3.2 | 3.0 | 2.1 | 4.6 | 3.70 |

Uncertainty is approximately ±0.4 for ledger, runtime, demand, and maintenance
scores because full proof generation, deployment, fees, user research, and
production operator interviews have not been completed. A universal interpreter
becomes preferable if a bounded interpreter can stay within acceptable proving
latency/key size across the representative suite. Direct ZKIR becomes preferable
only if ZKIR gains a stable versioned interface and Moriarty can support the
additional compiler/proof burden. A Compact library becomes preferable if pilot
users value privacy composition but not a distinct analyzable language.

## Initial product boundary

The primary user is a Midnight application developer who needs an audited,
bounded financial state machine with private terms or eligibility evidence and
selective disclosure. The first pilots are private-term escrow, collateralized
loan servicing, milestone treasury/grant release, and atomic settlement. A
private auction is a useful stress case but should not be the first value-
bearing release.

The strongest case against the project is demand, not compilation. Compact can
already express these contracts directly. Moriarty is justified only if its
predeployment bounds, formal financial semantics, migration story, generated
explanations, and reusable audits reduce real integration and review costs for
at least two non-toy pilots.

The DeFiFormal corpus is now an explicit feasibility gate. Moriarty must cover
the 72-row roster through a smaller set of canonical behavior patterns and
parameterized product libraries, while preserving row-level manifests and
assumptions. It must not claim corpus coverage merely because all rows received
a family, facet, or historical M4+ label. The current 60-construction evidence contains zero complete
verdicts, so it is a requirements and residue source rather than proof that the
kernel already covers the market.

## First backend stop-test disposition

**CLM-0119.** Continue the bounded feasibility phase after E00. This is a
recommendation based on the reproduced S3 experiment in `SRC-0019`. Confidence
is medium because only one canonical application and mock compilation have
passed.

E00 produced a finite Core, generated Compact, a client-checkable manifest, and
a 1,000-trace certificate with zero divergence. Compact and the ZKIR mock
compiler accepted the corrected artifacts. No unbounded collection, dynamic
loop, recursion, or cross-contract call appears in generated Compact.

This result is for an abstract constructor template. It does not bind a real
Midnight address, authority digest, or token color. The 18 deadline-boundary
cells, terminal-call regressions, artifact hash chain, and disclosure negative
control strengthen the stop test without changing that limitation.

The E00 result does not select a production language. Keep the Compact-library
fallback active until at least the loan, option, mandate, and conditional-token
applications pass the same gate. Stop any privacy claim that depends on an
unlisted `disclose()` expression.
