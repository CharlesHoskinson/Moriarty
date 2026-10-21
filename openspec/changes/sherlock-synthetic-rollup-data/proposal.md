# Synthetic Fenrir rollup data for Sherlock recursion

Status: DRAFT / PROPOSED, 2026-10-21. Specified only. Nothing in this package is implemented, executed or accepted. Design inputs are pinned in [design-cut.json](design-cut.json) (cut digest `ae5e4b4d259ff8367e9470b76c16c6c93fde5acb207ef9401987d85645916f5f`).

## Why

The [stagenet recursive-rollup experiment](../sherlock-stagenet-recursive-rollup/proposal.md) names a Fenrir-derived synthetic rollup as its main feasibility and performance workload (SRPOC-003, SRPOC-004). Two data sets exist today, and neither has the shape of the rollup that Fenrir will settle.

1. The [benchmark catalog /2](../../../models/sherlock/benchmarks/workloads/catalog.json) has a scalar account projection of 16/32/64/96 transactions. Its four tree roots are `unavailable`, and it is not Fenrir's note schema (repository observation, [methodology](../../../docs/sherlock/reference/benchmark-workload-methodology.md)).
2. Fenrir's rollup test format v3 (FRTF v3, `~/projectfenrir/recursion/rollup-test-format/v3/`, MANIFEST sha256 `15b8c81b…34970c`) gives both test profiles over the I1 strawman: 16 transactions and 50 headers a batch, a 12-field state and a 16-field statement (source fact, `HANDOFF-MORIARTY.md`).

The rollup that settles is larger and has more parts. The five module specs in `~/research/rollup-subarch-2026-10-21/modules/` define about 30,000 transactions a batch at 100 TPS (`H_B` = 50 headers of 6 s; RX-56), a 23-field `S5` state, a 17-field `stmt` with `ext_digest`, seven leaf step kinds, user-proof and TierC leaves, a Celestia inclusion tree per range (owner ruling Q8 = B), a light-client store with gaps up to `GAP_MAX` = 256 and a DA miss path (source facts, VALIDITY, DA-PROOF, CELESTIA-AUTH, LIVENESS-EXIT). The fleet that carries 100 TPS is sized from E-RB-2 and E-RB-8 (Q10 = A). Those experiments need inputs of the real shape, and they need the inputs before the recursion is ready, so that the first recursive proof meets a validated corpus on the first day.

This package changes the terminal decision of the stagenet experiment from "feasible on a 16-transaction strawman" to a measured answer on the rollup's real shapes, with every input validated against the executable Quint contracts before any proof runs.

## What Changes

Add a specification for FSYN, a deterministic, seeded generator of Fenrir rollup data in both test-format profiles, with its oracles, tests, fixtures and benchmark plan.

- **Generator.** Seeded workloads on a scale ladder from FRTF v3 (S0, reproduced byte for byte) to a production-shape 30,000-transaction batch (S3) and sustained multi-batch runs (S4). It covers the transaction mix (transfers, covenant calls, forced entries, deposits, escapes, exits and claims, DA misses), header and blob sizes (planning mix of about 3 MB a header; cap `RECORDS_BYTES_MAX` 3.5 MiB), committee events (16 sealed seats, up to f = 4 Byzantine behaviours, permanent seat loss up to the migration trigger of 3), Celestia light-client headers and pointer gaps up to `GAP_MAX`.
- **Profiles.** Profile A (`sherlock-stf/1`: one A0 step proof a transaction, arity-4 A1/A2 fold, re-keyed Ledger 9 decider) and Profile B (Fenrir's real format: state and user-proof leaves prove transfers; covenant `SherlockProof v1` folds through the ID-74 TierCLeaf) over one schema.
- **Fidelity.** A map from each artefact to its interface operation and statement (`stmt`, `stmt_da`, `stmt_lc`, `stmt_dm`). The Quint contracts in `~/research/rollup-subarch-2026-10-21/models/` are the oracles, with differential replay in both directions.
- **Tests.** Golden vectors, property tests from the module properties (P-VAL, P-DA, P-CEL, P-LIV, P-LIFE) with the Lean-proved bounds as assertions, mutation and adversarial suites, and replayable content-addressed fixtures.
- **Benchmarks.** Specified now and run when recursion is ready: prover throughput per leaf type, tree depth and latency, fleet sizing against E-RB-2 and E-RB-8, Ledger 9 decider cost, DA inclusion proof cost, the settled-TPS curve up to 100, and a reproducible report format on the existing benchmark schema /2.
- **Phasing.** Phases P0 to P2 run now, without recursion. Phases P3 to P5 switch on at named recursion and key milestones. Each phase has a gate.
- **Sync with Fenrir.** One source of truth: constants, layouts and rules come from the pinned executable models and module specs. A stale cut stops generation.

Artefacts: [requirements](requirements.md) (SSRD-001 to SSRD-024), [spec delta](specs/synthetic-rollup-data/spec.md), [design](design.md), [tasks](tasks.md), [traceability](traceability.json), [design cut](design-cut.json).

## Impact

- New source paths (planned): `models/sherlock/rollup-synth/` (Python orchestration, oracle runners, catalog bridge) and `models/sherlock/rollup-synth/fsyn/` (a standalone Rust package outside the root workspace, pinned to the Fenrir Ledger-9 crate set for Poseidon and field encoding).
- Extended consumer: the benchmark catalog moves to `sherlock-benchmark-workloads/3` with new FSYN cases. Schema /2 cases stay unchanged.
- Fenrir side: no Fenrir file changes in this package. FSYN proposes an FRTF production-shape extension to Fenrir's `sprint5-rollup-test-format` owner; Fenrir keeps ownership of the format (ID-08, ID-134).

## Non-goals

- No proof, key, native build, wallet spend or stagenet call. Benchmark phases name their trigger; they grant no native authority and no resource class.
- No new security policy or judgment. Adversarial corpora derive from the existing security model and carry its expected outcomes.
- No replacement of SRPOC-001 to SRPOC-009, SBENCH-001 to SBENCH-010, UIREQ or SCON clauses. SSRD clauses are additive and do not enter the original completion denominators.
- No performance SLA. The 100 TPS figure is the owner's settled target (Q1); FSYN measures against it and invents no pass threshold.
- No hosted CI, no new orchestration framework, no human approval step.
