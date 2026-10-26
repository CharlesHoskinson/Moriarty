# s7-recursion wave 1 research plan

OPEN QUESTION — Nine concrete leads, tested against the earlier Loki-05, Loki-07 and Astra-2 studies (2026-10-24); no performance measurements authorized or claimed here.

1. Direct witness composition: derive the fixed-pair and mono16 domain break-even including state binding, table extents, and unaccelerated overhead; earlier Astra-2 is the control.
2. Integer tree/partition search: 16 transactions versus 16 proof leaves, power-of-two domains, leaf-only versus genuinely recursive nodes, terminal placement. Run a transparent arithmetic model, not a prover benchmark.
3. Fused verifier terminal: establish when adding the existing pairing gadget crosses k20→k21, and when removing an entire parent proof actually wins.
4. BCLMS KZG accumulation and Axiom snark-verifier: challenge binding and the remaining decider obligation; helper 1 checks whether anything goes beyond existing deferral.
5. Upstream Midnight fflonk: inspect exact gadget/PCS sources and PR #575, including cost/degree changes and the boundary to ledger-9 stock architecture.
6. Nova curve/compression support and Sonobe implementation maturity: helper 2 checks actual supported cycle, decider and missing wrapper; prioritize a rejection test over a speculative port.
7. Newer folding/accumulation alternatives (MicroNova/NeutronNova or single-curve IVC): find a primary implementation or paper and charge the complete Midnight bridge.
8. Hinted scalar multiplication and cofactor-torsion attacks: helper 1 checks new 2026 evidence, subgroup obligations and whether proposed MSM savings survive.
9. Foreign-field/table and terminal-pairing geometry: published Midnight emulation and residue work; isolate a table-driven k floor from arithmetic-row savings and set explicit falsification thresholds.

SOURCE — Requested earlier Loki notes were found at ../../wiki-llm/loki/nodes/loki-05-recursion-accumulation.md and ../../wiki-llm/loki/nodes/loki-07-folding-alternatives.md; requested 10x locations are absent. Astra-2 exists at ../../wiki-llm/loki/10x/astra-2/PROPOSAL.md.

OPEN QUESTION — Graphify has no existing graph in this research folder. Its graph-build writes fall outside the requested output trees, so use direct source inspection within the user's research scope; no installation or graph rebuild.
