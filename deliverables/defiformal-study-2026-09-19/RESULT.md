# DeFiFormal scope review

Source: CharlesHoskinson/defiformal, semantic-kernel-pivot, commit `33b9a9550ac1ed12c83c32d15277e530741787db`. Source inspection on 2026-09-19; no Lean build or deployed execution performed. A shallow sparse checkout was used; repository absence claims were checked against the tracked tree.

The current project is a formal semantic reference and verification library, not an implemented federated DeFi runtime. Its typed effects, explicit refusals, initialization/preservation obligations and distinct sequential, disjoint, interleaved and atomic operators inform Moriarty's native stage semantics. Their theorems do not automatically transfer to Moriarty or Midnight circuits. No Lean dependency is introduced.

[Composition study](composition-review.md) separates domain-admin capabilities from owner consent, trusted template registries from permissionless deployment, and configured net settlement from persistent financial duties. [Library study](libraries-review.md) identifies concrete arithmetic, narrow token0 liquidity and stable-time vault code; broader families and claims lifecycle remain planned. Normalize price orientation, numeric width, rounding and gross effects before transferring formulas.

[Current import graph](current-kernel-graph/GRAPH_REPORT.md) covers 193 selected Lean modules and 518 source imports; this is a textual import graph, not proof verification. [Interactive graph](current-kernel-graph/graph.html). Existing historical taxonomy graphs were not used as evidence of current implementation.

The consolidated Moriarty design must reuse these semantic contracts as reference obligations, then establish native implementation, adversarial-witness soundness and source-to-ledger correspondence separately. Do not impose a trusted project registry on developers, represent debt as token conservation, or equate model atomic publication with cross-chain rollback.
