---
id: moriarty.research.journal
type: decision
title: Moriarty research journal
status: active
updated_at: 2026-09-03T07:23:21Z
sources:
  - SRC-0016
  - SRC-0017
  - SRC-0018
---

# Moriarty research journal

This journal records decision rationale, evidence changes, semantic scope, and
the next falsification test. It does not record private chain-of-thought. An
entry is complete only when its referenced evidence is preserved.

## Iteration S00.2: replace calendar progress with evidence gates

- Timestamp: 2026-09-03T07:23:21Z
- Repository base: `006c4d91ed09c0a89261861b6e7203b3efa3e2df`
- State: active

### Trigger

The user rejected calendar-based planning. The user required sprint-only
progress, an iteration-by-iteration semantic scope record, evidence that a
Compact DSL is possible, and a complete SDK specification.

### Decision rationale

Elapsed time is not evidence. Replace day windows with sprint gates. Retain the
same dependency order because proof, backend, and coverage packages still have
real input dependencies.

Treat E00 as a narrow feasibility witness. It proves that one finite
Marlowe-shaped Core slice can generate reviewable Compact and ZKIR artifacts.
It does not prove general compilation or production deployment.

Expand the SDK package from transaction verification to the complete authoring,
compilation, analysis, packaging, deployment, client-verification, and
operations toolchain.

### Active semantic scope

E00 contains `Close`, `Pay`, `If`, `When`, `Deposit`, bounded `Choice`, parties,
tokens, accounts, explicit timeout, payments, warnings, errors, and the atomic
swap specialization. All collections and contract paths are finite.

No later candidate construct is frozen. Attest, action sets, mandate,
conditional-token split or merge, external calls, minting, Merkleized
continuations, modules, and packages remain proposed or outside the E00 Core.

### Evidence state

- The positive Compact program compiled with the pinned toolchain.
- Four ZKIR 3.0 circuits passed the mock compiler.
- One thousand unique differential traces produced zero divergence.
- The negative disclosure fixture failed compilation as required.
- Real proving parameters, keys, proofs, ledger execution, and cost remain open.

### Next falsification test

Reproduce E00 from a fresh pinned environment. Then attempt a second canonical
application through the same generic surface-to-Core-to-Compact contracts. A
special-case generator that cannot support another shape is insufficient
evidence for a language.
