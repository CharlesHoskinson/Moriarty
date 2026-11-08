# Synthetic Fenrir rollup data for Sherlock recursion

Status: DRAFT / PROPOSED, 2026-10-21. Specified only. Nothing in this package is implemented, executed or accepted. Design inputs are pinned in [design-cut.json](design-cut.json) (cut digest `345c4f139c724dbf261e7aed343f4537b8b6c636464e0aa1e4af7246bcb98e02`). The cut pins the final design: `ROLLUP-ARCHITECTURE.md` revision 20, the five module specs, 51 Quint files, the Lean sources and owner rulings Q1 to Q17. Amended 2026-11-08: the cut is history only; see the owner amendment below.

## Owner amendment 2026-11-08: Sherlock has no Fenrir dependency

The owner's words (2026-11-08, verbatim): "Sherlock should not depend at all on Fenrir, We have a synthetic rollup design, we use that". On the C4 question the owner decided (2026-11-08, verbatim): "keep them out of C4".

This amendment overrides any conflicting text in this package, including the original text below that is kept as history.

1. **Self-contained.** FSYN is Sherlock's own rollup workload and lives entirely in Moriarty. Generation, tests and checks read only tracked files of this repository. At build or test time they never read a Fenrir repository, `~/projectfenrir`, `~/research` or any other path outside the repository (SSRD-030).
2. **Moriarty-owned parameters.** The constants, field orders, domain tags, layouts, rules and bounds that the design cut pins are restated as Moriarty-owned synthetic parameters inside the repository (planned `models/sherlock/rollup-synth/params/`, schema `moriarty-fsyn-params/1`, with a manifest of path, bytes and SHA-256). Each entry is a value with a short description in this repository's own words. The parameter set is not a copy, excerpt or snapshot of a private Fenrir file, and no file of it carries a `> Source: file://.../projectfenrir/...` header (`.githooks/pre-commit` rule 0). Where FSYN needs an oracle model, Moriarty writes its own model in the repository from these parameters. [design-cut.json](design-cut.json) stays as history of where the values first came from. It is no longer a generation input or a gate (SSRD-001, SSRD-002).
3. **Sync is optional, offline and manual.** A maintainer may compare the parameters with a Fenrir design out of band and propose a parameter change as an ordinary commit. The comparison is never a gate, never runs in a predicate or in a T0-T4 tier, and a stale or changed Fenrir cut never stops generation or makes a fixture stale (SSRD-024).
4. **Profile B is restated as a synthetic profile.** It becomes "Profile B (synthetic production shape)": its leaf kinds, tree shape and arities are Moriarty-owned parameters. It no longer claims to be Fenrir's real format, and nothing is checked against a Fenrir format. Reason: Profile B is the only production-shape (30,000-transaction) workload in this package; dropping it would leave Sherlock with the 16-transaction strawman only (SSRD-011).
5. **S0 is a Moriarty golden.** The expected S0 envelope, view and `workloads.json` digests are frozen in the repository, and FSYN reproduces them byte for byte without `rtf generate` or an FRTF v3 tree. `rtf verify` and FRTF are no longer acceptance oracles (SSRD-005, SSRD-015).
6. **B rows.** The factory B rows in `qa/sherlock/predicates.yaml` are Sherlock's synthetic-rollup qualification on FSYN data: local first, then Preview, then Mainnet (SSRD-029; consensus change `sherlock-vm-consensus-20261107` SWF-013 and D14). Network runs happen only as that change's operations OP-PV-SYNTH-ROLLUP and OP-MN-SYNTH-ROLLUP, never from FSYN itself. The B rows stay out of the Sherlock C4 closure by owner decision ("keep them out of C4").
7. **No Fenrir handoff gate.** The handoff to the Fenrir format owner (task 2.11) and the `frtf-x/1` proposal as FRTF v4 (OQ-2) are dropped from FSYN's gates. `frtf-x/1` is Moriarty's own envelope.
8. **Current code.** Code that reads Fenrir paths today (the B1-B3 check scripts, `models/sherlock/rollup-synth/sources.py` and its callers, and `crates/mori-recursion/tests/sherlock_rollup*.rs`) is a recorded dependency, not accepted design. Consensus task 1.9 replaces those reads with FSYN-generated in-repository fixtures and adds the T0 guard F17 (SWF-014).

## Why

The [stagenet recursive-rollup experiment](../sherlock-stagenet-recursive-rollup/proposal.md) names a Fenrir-derived synthetic rollup as its main feasibility and performance workload (SRPOC-003, SRPOC-004). Two data sets exist today, and neither has the shape of the rollup that Fenrir will settle.

1. The [benchmark catalog /2](../../../models/sherlock/benchmarks/workloads/catalog.json) has a scalar account projection of 16/32/64/96 transactions. Its four tree roots are `unavailable`, and it is not Fenrir's note schema (repository observation, [methodology](../../../docs/sherlock/reference/benchmark-workload-methodology.md)).
2. Fenrir's rollup test format v3 (FRTF v3, `~/projectfenrir/recursion/rollup-test-format/v3/`, MANIFEST sha256 `15b8c81b…34970c`) gives both test profiles over the I1 strawman: 16 transactions and 50 headers a batch, a 12-field state and a 16-field statement (source fact, `HANDOFF-MORIARTY.md`).

The rollup that settles is larger and has more parts. The five module specs in `~/research/rollup-subarch-2026-10-21/modules/` define about 30,000 transactions a batch at 100 TPS (`H_B` = 50 headers of 6 s; RX-56), a 23-field `S5` state, a 17-field `stmt` with `ext_digest`, seven leaf step kinds, user-proof and TierC leaves, a Celestia inclusion tree per range (owner ruling Q8 = B), a light-client store with gaps up to `GAP_MAX` = 256, a DA miss path, and an exit subsystem with one exit key, one exit statement, a single execution-time hold check and shielded or unshielded payouts (source facts, VALIDITY, DA-PROOF, CELESTIA-AUTH, LIVENESS-EXIT, `EXIT-DESIGN-NOTE.md`). The contract is sealed for life (Q15), and exits disclose at the boundary only as Q16 and Q17 rule. The fleet that carries 100 TPS is sized from E-RB-2 and E-RB-8 (Q10 = A). Those experiments need inputs of the real shape, and they need the inputs before the recursion is ready, so that the first recursive proof meets a validated corpus on the first day.

This package changes the terminal decision of the stagenet experiment from "feasible on a 16-transaction strawman" to a measured answer on the rollup's real shapes, with every input validated against the executable Quint contracts before any proof runs.

## What Changes

Add a specification for FSYN, a deterministic, seeded generator of Fenrir rollup data in both test-format profiles, with its oracles, tests, fixtures and benchmark plan.

- **Generator.** Seeded workloads on a scale ladder from FRTF v3 (S0, reproduced byte for byte) to a production-shape 30,000-transaction batch (S3) and sustained multi-batch runs (S4). It covers the transaction mix (transfers, covenant calls, forced entries, deposits, DA misses, and every exit kind: L2 spend, forced transfer and withdrawal, escape, import, shielded and unshielded payout), FENC holds, header and blob sizes (planning mix of about 3 MB a header; cap `RECORDS_BYTES_MAX` 3.5 MiB), committee events (16 sealed seats, up to f = 4 Byzantine behaviours, key-preserving rotation only, migration at 3 permanent seat losses), Celestia light-client headers and pointer gaps up to `GAP_MAX`.
- **Profiles.** Profile A (`sherlock-stf/1`: one A0 step proof a transaction, arity-4 A1/A2 fold, re-keyed Ledger 9 decider) and Profile B (Fenrir's real format: state and user-proof leaves prove transfers; covenant `SherlockProof v1` folds through the ID-74 TierCLeaf) over one schema. Amended 2026-11-08: Profile B is restated as "Profile B (synthetic production shape)" with Moriarty-owned parameters; it no longer claims Fenrir's real format.
- **Disclosure.** Each exit fixture carries its public transcript, checked so that a shielded exit shows only asset and amount (Q17) and an unshielded exit shows its own destination and amount, unlinkable to L2 (Q16).
- **Fidelity.** A map from each artefact to its interface operation and statement (`stmt`, `stmt_da`, `stmt_lc`, `stmt_dm`). The Quint contracts in `~/research/rollup-subarch-2026-10-21/models/` are the oracles, with differential replay in both directions.
- **Tests.** Golden vectors, property tests from the module properties (P-VAL, P-DA, P-CEL, P-LIV, P-LIFE) with the Lean-proved bounds as assertions, mutation and adversarial suites, and replayable content-addressed fixtures.
- **Benchmarks.** Specified now and run when recursion is ready: prover throughput per leaf type, tree depth and latency, fleet sizing against E-RB-2 and E-RB-8, Ledger 9 decider cost, DA inclusion proof cost, the settled-TPS curve up to 100, and a reproducible report format on the existing benchmark schema /2.
- **Phasing.** Phases P0 to P2 run now, without recursion. Phases P3 to P5 switch on at named recursion and key milestones. Each phase has a gate.
- **Sync with Fenrir.** ~~One source of truth: constants, layouts and rules come from the pinned executable models and module specs. A stale cut stops generation.~~ Amended 2026-11-08: constants, layouts and rules come from the Moriarty-owned parameter set in the repository. Sync with Fenrir is optional, offline and manual; it is never a gate, and a stale Fenrir cut never stops generation.

Artefacts: [requirements](requirements.md) (SSRD-001 to SSRD-030; SSRD-029 and SSRD-030 added 2026-11-08), [spec delta](specs/synthetic-rollup-data/spec.md), [design](design.md), [tasks](tasks.md), [traceability](traceability.json), [design cut](design-cut.json).

## Impact

- New source paths (planned): `models/sherlock/rollup-synth/` (Python orchestration, oracle runners, catalog bridge) and `models/sherlock/rollup-synth/fsyn/` (a standalone Rust package outside the root workspace, pinned to the Fenrir Ledger-9 crate set for Poseidon and field encoding).
- Extended consumer: the benchmark catalog moves to `sherlock-benchmark-workloads/3` with new FSYN cases. Schema /2 cases stay unchanged.
- Fenrir side: no Fenrir file changes in this package. ~~FSYN proposes an FRTF production-shape extension to Fenrir's `sprint5-rollup-test-format` owner; Fenrir keeps ownership of the format (ID-08, ID-134).~~ Amended 2026-11-08: FSYN carries no Fenrir dependency and no Fenrir handoff; Fenrir's own testing on Preview and Mainnet belongs to the Fenrir project.

## Non-goals

- No proof, key, native build, wallet spend or stagenet call from FSYN itself (amended 2026-11-08: the B rows prove and submit FSYN data locally, on Preview and on Mainnet through the consensus change's operations, SSRD-029). Benchmark phases name their trigger; they grant no native authority and no resource class.
- No new security policy or judgment. Adversarial corpora derive from the existing security model and carry its expected outcomes.
- No replacement of SRPOC-001 to SRPOC-009, SBENCH-001 to SBENCH-010, UIREQ or SCON clauses. SSRD clauses are additive and do not enter the original completion denominators.
- No performance SLA. The 100 TPS figure is the owner's settled target (Q1); FSYN measures against it and invents no pass threshold.
- No hosted CI, no new orchestration framework, no human approval step.
