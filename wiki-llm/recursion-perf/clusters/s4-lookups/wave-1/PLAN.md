# s4-lookups wave 1 plan

SOURCE — Earlier study found at `../loki/nodes/loki-06-recursive-circuit-cost.md`, captured as `raw/s4-lookups/prior-loki06.md`. The supplied `10x/` filename does not exist here. This is research only; no codebase mutation, factory build, installation, commit, or deployment.

OPEN QUESTION — Pursue these nine leads, ranking by actual hot-path applicability and acceptance boundary:
1. Pinned Midnight 0.8.3 lookup implementation: histogram/table-index permutation instead of field sorting and randomized hashing.
2. Shared range-table compression and static indexing across the existing lookup lanes; distinguish challenge-dependent and reusable data.
3. Parallel CPU batch inversion and prefix-product scan as a 20-core control baseline.
4. sppark/CUDA GPU scan, batch inversion, and device-resident product construction (helper 2).
5. ICICLE Halo2 and cuSnark benchmark phase breakdowns: quantify the workload gap (helper 2).
6. Range-tag caps and decomposition planner: test the separate advice/table barriers and lane/degree costs.
7. LogUp logarithmic derivatives: characterize inverse/sum cost and verifier migration (helper 1).
8. cq cached quotients: preprocessing, query/table ratio, SRS powers and verifier obligations (helper 1).
9. Lasso/Jolt structured decomposition: distinguish virtual table savings from multilinear protocol replacement (helper 1).

OPEN QUESTION — Validate the table-index permutation algebra in a small exhaustive finite-field model, and calculate Amdahl/domain ceilings without representing them as proof-system benchmarks. Deliver FINDINGS.md and three concrete host experiment protocols. No measured prover timing is claimed.
