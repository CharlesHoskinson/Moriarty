# Proposal: final design of the Sherlock co-processor (`sherlock-processor-final-20261111`)

Status: S2 (written design, specified only). Step 7 of `research/sherlock-processor-final-20261111/PLAN.md` (main working copy). Nothing in this change is built, keyed, proved or measured by this change. The design is `research/sherlock-final-design-20261111/ARCHITECTURE.md`, below `FD/ARCHITECTURE.md`. Its decisions are in `FD/DECISIONS.md`, its experiments in `FD/EXPERIMENTS.md` and its terms in `FD/TERMINOLOGY.md`. The merged proposal is `research/sherlock-processor-final-20261111/PROPOSAL.md` on main, below `PR`.

## Why

Sherlock is a universal, elastic proving co-processor for Midnight. The specs of `sherlock-vm-consensus-20261107` and `sherlock-modular-leaf-20261111` describe a halo2 leaf, a halo2 node and a halo2 run decider with row gates. The final design replaces that proof stack. Leaves and nodes are small-field STARK shards on the user's GPU. One bridge layer `L` compresses the tree. One blinded PLONK-KZG wrap verifies `L` and is the only proof that the unchanged ledger 9.1.0.1 verifier checks.

This change specifies the new and changed co-processor kits, makes zero knowledge end to end a requirement at the level of GP-001 to GP-004, and replaces the halo2 row gates by the derived gates of `FD/ARCHITECTURE.md` section 3.3. It can move the terminal evidence-gated decision of `sherlock-vm-consensus-20261107` because every GP acceptance line now has one proof path to the ledger, and every kit has its tests, its Lean obligations with their status and its measurements as gates.

## What Changes

- New requirement GP-ZK: zero knowledge end to end, at the level of GP-001 to GP-004. New requirements CPR-001 to CPR-005: the hard rules of the co-processor.
- One capability spec per new or changed co-processor kit or module (table below), each with SHALL requirements, scenarios and its Lean obligations.
- The derived gates replace the halo2 row gates: no BLS12-381 circuit above k = 21, the wrap at 85 percent of `2^k - 7` usable rows at the smallest k of 19, 20 and 21, fixed STARK shard shapes inside the shape bounds of their layer, and STARK cost in seconds per proved step.
- Kit L AND WRAP supersedes kit DECIDER of `sherlock-modular-leaf-20261111`. The test names `decider_seg_root_as_run`, `decider_set_head_forged`, `pos_decider_all_outcomes`, `pos_decider_collection_three_runs` and obligation DEC-L1 move to kit L AND WRAP with their meaning.
- `sherlock-modular-leaf-20261111` is corrected in place to the design: the machine-level decisions are named AD-1 to AD-23, the constraint tests of the leaf kits name the constraint checker, and NODE INTEGRATION verifies STARK children with no accumulator.

## Scope

| Spec folder | Kit or module | ID prefix | `FD/ARCHITECTURE.md` |
|---|---|---|---|
| `sherlock-general-purpose` | GP-level requirement GP-ZK and the hard rules | GP-ZK, CPR | header, 1.9 |
| `sherlock-l-and-wrap` | L AND WRAP | LW | kit L AND WRAP, 1.6 |
| `sherlock-zero-knowledge` | ZERO KNOWLEDGE | ZK | kit ZERO KNOWLEDGE |
| `sherlock-compact-frontend` | COMPACT FRONTEND with E-1 placement V and the bind gadget | CF | kit COMPACT FRONTEND, 1.7 |
| `sherlock-anchor-chunks` | anchor and atomic chunked calls | ANC | kit COMPACT FRONTEND, 1.10 |
| `sherlock-field-log` | deferred field log, capacity and `W_pv` | FLG | 1.8 |
| `sherlock-hash-domains` | hash domains and the transcript of `L` | HDM | 1.8, 1.9 |
| `sherlock-security-gate` | security statement and gate G1 to G5 | SQG | 1.9 |
| `sherlock-gpu-wrap` | GPU wrap prover | GPW | kit L AND WRAP row budget |
| `sherlock-prover-daemon` | resident Sherlock prover | PRD | kit ZERO KNOWLEDGE |
| `sherlock-midnight-side` | Midnight side | MID | 1.10 |
| `sherlock-shard-plan` | layer stack, shard plan and keys | SPL | 1.6, 1.7 |
| `sherlock-derived-gates` | derived gates and constraint tests | DG | 3.3 |
| `sherlock-freeze-prerequisites` | the BabyBear interfaces of freeze prerequisites FP-1 to FP-7 | FPR | 1.11 |

The ID prefixes do not collide with the prefixes of `sherlock-vm-consensus-20261107` and `sherlock-modular-leaf-20261111`. The Lean labels BR-L1 to BR-L4, ZK-L1, ZK-L2, CF-L1, CF-L2 and DEC-L1 are obligations, not requirement IDs.

Out of scope: the machine-level leaf kits (they stay in `sherlock-modular-leaf-20261111`), Profile B, continuation mode 2, any change of the GP-001 to GP-004 text, Mainnet operations, and the interface freeze itself (plan step 11). The interfaces that the freeze proposal freezes are in scope as FPR-001 to FPR-016; FP-3 stays open with three missing facts (FPR-009).

## Supersedes

Each item below keeps its history in git. The last revision with the old text is commit `0ab8cd79d`.

| Old text | Replaced by | How |
|---|---|---|
| `sherlock-vm-consensus-20261107/design.md` section 2.7.9: node at most 950,000 rows, decider at most 1,000,000 rows, `k_leaf <= 16` at S = 16 (line 107 at `0ab8cd79d`, cited as line 105 in the plan) | DG-001 to DG-007 | task G.1 edits the governing change. The edit waits for task G.1, because `openspec validate sherlock-vm-consensus-20261107 --strict` fails today on 26 warnings in council and owner text that this change does not rewrite |
| Kit DECIDER, spec `sherlock-decider`, DCD-001 to DCD-007 | LW-002 to LW-011, DG-001, DG-002 | corrected in place in `sherlock-modular-leaf-20261111` |
| NODE-004 accumulator folding and NODE-007 node row gate | the STARK recursion verifier of NODE-004 and the shape bounds of NODE-007 | corrected in place |
| MockProver as the constraint test of the leaf kits | DG-008 | corrected in place |
| Machine-level decision ids OD-1 to OD-23 of `sherlock-modular-leaf-20261111` | AD-1 to AD-23 (`FD/ARCHITECTURE.md` section 4.1) | renamed in place. The numbers are unchanged |

## Decisions that this change depends on

OD-4 to OD-18 are lead decisions with the recommended option, owner to confirm (`FD/DECISIONS.md`). OD-19 is an owner decision (2026-11-11) and closes OD-1 to OD-3, the internal part of OD-4 and OD-20. None of them binds before the owner confirms it. A requirement that depends on one names its id.

| Id | Decision | Requirements |
|---|---|---|
| OD-21 | ledger-readable wrap path, owner decision with no default (M7-0 BLOCKER) | LW-013, LW-014, LW-016, LW-017, MID-001, CPR-004 |
| OD-19 | unique decoding internal and `L` sets, one OOD sample | SQG-002, SQG-003, SQG-005, SQG-012, HDM-006, LW-002, DG-005 |
| OD-4 | proven rule on inner layers and the margin rule | SQG-002, SQG-004 |
| OD-5 | E-1 placement V with V+ | CF-008, CF-009, LW-005 |
| OD-6 | log capacity `V = P = 256`, `E = 0` | FLG-003, FLG-005 |
| OD-7 | `L_TX` = 1,024 | LW-003, CF-007 |
| OD-8 | hybrid native and Sherlock contracts | ANC-009, MID-005 |
| OD-9 | elastic trust domain: devices the user owns | ZK-001, PRD-003 |
| OD-10 | wait for the Fr Poseidon GPU kernel | GPW-006 |
| OD-11 | no second Fr proving engine | GPW-006, LW-014 |
| OD-12 | opt-in user batches | ZK-009 |
| OD-13 | resident prover daemon | PRD-001, PRD-002 |
| OD-14 | Compact `Field` stays emulated | CF-005 |
| OD-15 | one call-vector layout | LW-003, LW-007 |
| OD-16 | anchor lock and native prologue | ANC-002, ANC-003, ANC-009, LW-005 |
| OD-17 | `pv` encoding and entry capacity | FLG-003, FLG-004, FLG-006 |
| OD-18 | hash domains and transcript algorithm | HDM-001 to HDM-005 |

## Impact

- Code (proposed paths of the kits): `crates/mori-recursion/src/wrap/`, stark-backend parameter code, `crates/sherlock-prover/src/`, `tools/sherlock-compact/`, `crates/mori-core/src/sherlock/sl/`.
- Proofs: `clean/fork/Clean/Sherlock/` (the restatement BR-L2 on the round 2 files of branch `bridge-lean`, commit `bdb92973a`).
- Keys: the universal key set (internal-recursive, `L`, wrap) and the per-program keys under `key_root`. No universal key freezes before the security qualification gate passes.
- Specs: `sherlock-modular-leaf-20261111` corrected in place. `sherlock-vm-consensus-20261107` through task G.1.
