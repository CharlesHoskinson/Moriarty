# Five independent lenses

1. **Core semantics and typing:** exact pool state, `pre/post`, asset/share identity, transfer/effect/footprint derivation, linear resources, complete stage relation. Seek type errors and unsound rules.
2. **Arithmetic and pool economics:** constant-product exact-input and exact-output, fees, directed rounding, wide products, zero-liquidity bootstrap, LP mint/burn, virtual offsets and donation attack. Seek a minimal certifiable arithmetic basis.
3. **Intent completion and market structure:** route and venue holes, per-hop effects, RFQ authorization, competing solvers, partial fills, shared-write order-book/batch clearing, MEV and surplus allocation. Separate a U0/U3 trader envelope from later multi-signer admission.
4. **Compiler and native feasibility:** source/Core/K/ZKIR/ledger correspondence for one AMM stage, public inputs, authenticated reserve reads, signed budget binding, effects/readback, witness premises, bounded cost. Identify what U1/U2 can certify before Φ₁ and what must defer.
5. **Adversarial acceptance and release plan:** construct hostile counterexamples and positive controls for pool invariants, rounding, fee/gross/net accounting, route/refinement, aliasing/footprints, race/replay, LP claims. Rank design edits by safety and reviewability, with precise stop rules.
