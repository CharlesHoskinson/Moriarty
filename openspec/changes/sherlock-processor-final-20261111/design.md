# Design: final design of the Sherlock co-processor

Status: S2, specified only. This file points at the design. It does not copy it. The source of truth is `research/sherlock-final-design-20261111/ARCHITECTURE.md` (below `FD/ARCHITECTURE.md`), with `FD/DECISIONS.md`, `FD/EXPERIMENTS.md`, `FD/TERMINOLOGY.md` and `FD/MODULARITY.md`. The merged proposal is `research/sherlock-processor-final-20261111/PROPOSAL.md` on main (`PR`). Where this change and the design differ, the design is correct and this change is corrected to it.

## 1. Goals and non-goals

Goal: one proof path from any Sherlock run to the unchanged ledger 9.1.0.1 verifier, zero knowledge end to end, under one universal key set, with each co-processor kit built and checked by one agent against its interface.

Non-goals: the machine-level leaf kits (`sherlock-modular-leaf-20261111`), Profile B, continuation mode 2, any change of the GP-001 to GP-004 text, the interface freeze (plan step 11) and Mainnet.

## 2. Where each spec reads the design

| Spec | Read |
|---|---|
| `sherlock-general-purpose` (GP-ZK, CPR) | `FD/ARCHITECTURE.md` header (hard rules), `FD/TERMINOLOGY.md` "Hard rules", `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` |
| `sherlock-shard-plan` | section 1.6 (layer stack, keys, binding the plan, the shard plan, what freezes), section 1.7 |
| `sherlock-field-log` | section 1.8 (log entries, capacity and encoding, reservation and capacity rule) |
| `sherlock-hash-domains` | section 1.8 (hash domains), section 1.9 (two OOD samples), `FD/EXPERIMENTS.md` section 2 (T-FS) |
| `sherlock-security-gate` | section 1.9 (target, parameter sets, margin rule, premises, gate G1 to G5), `FD/EXPERIMENTS.md` D3 and D10 |
| `sherlock-l-and-wrap` | kit L AND WRAP, section 1.1 |
| `sherlock-gpu-wrap` | kit L AND WRAP, row budget (kernels, estimates, measured byte identity, MSM skew, fallback triggers), `PR` section 8 |
| `sherlock-zero-knowledge` | kit ZERO KNOWLEDGE, `PR` section 5 |
| `sherlock-prover-daemon` | kit ZERO KNOWLEDGE (prover daemon, code skeleton), section 1.6 (path C) |
| `sherlock-compact-frontend` | kit COMPACT FRONTEND (tap, commands, lowering, tape, call vector, E-1 placement V and V+), section 1.7 |
| `sherlock-anchor-chunks` | kit COMPACT FRONTEND (chunked calls, anchor values, hybrid contracts), `FD/EXPERIMENTS.md` section 3 |
| `sherlock-midnight-side` | section 1.10, `PR` section 9 |
| `sherlock-derived-gates` | section 3.3 (gates, derived gates) |

## 3. Inputs, outputs and dependencies

- Inputs of a call: the program image and its plan (`prog_d`, `key_root`), the input behind the salted `input_d`, and the tape. Inputs of the design: owner rules, OD-1 to OD-18 (lead decisions, owner to confirm), AD-1 to AD-23 and the experiments D1 to D15.
- Outputs: one blinded wrap proof and the call vector of `2 + L_TX` words. Nothing else leaves the user's machine.
- Dependencies between the specs follow the kit dependencies of `FD/ARCHITECTURE.md` section 3.1: L AND WRAP needs NODE INTEGRATION, the stark-backend parameter code (build step 4) and the call vector of COMPACT FRONTEND. ZERO KNOWLEDGE needs L AND WRAP and HOST CHECKPOINT STORE. COMPACT FRONTEND needs CORE and DEC for `R_SL_*`.

## 4. Trust boundaries

- The user's machine (devices and GPUs the user owns, OD-9) holds the witness, the shard plan of a private program, every STARK proof and every checkpoint.
- The ledger sees the call vector and one blinded wrap proof. It trusts only `VerifierKey::verify`.
- Proof-system premises stay named hypotheses: A1 to A7 and `vkL_ok` of `bridge_sound` (`FD/ARCHITECTURE.md` section 1.9). No premise is an `axiom` in Lean.

## 5. Failure handling

- A constraint refusal is the only rejection credit. A panic, a synthesis error or a parser refusal earns none (DG-008).
- A failed derived gate leaves the GP target open and goes to the owner (DG-010). It never narrows a GP target.
- A fallback trigger of the wrap (LW-014, GPW-006) is recorded and goes to the owner. The fallback keeps k at or below 21.
- A failed gate check of G1 to G5 blocks the security claim and the freeze of the universal keys (SQG-011).

## 6. Semantic guarantees against feasibility

Semantic guarantees are the requirements of `specs/`. Feasibility is the derived gates (DG-001 to DG-007) and the measured cost in seconds per proved step. Every number in the specs is MEASURED with its record, DERIVED, ESTIMATE with its inputs, PENDING or LEAD, as in `FD/ARCHITECTURE.md`.

The `L` key bound is `w_stack` at most 48 with `mu` PoW 22 (LEAD, OD-3, owner to confirm). D3 section 8 (branch `d3-soundness`, commit `81abb8f7a`) measured that only the `mu` term moves with `w_stack`, and the keyed `L` structure points to about 36. D10 may tighten the bound.

## 7. Lean status

| Obligation | Spec | Status |
|---|---|---|
| BR-L1 `bridge_sound`, `wrap_view_only_claim` | LW-015 | built at `bdb92973a` (0 sorry, standard axioms only, 112 theorems) under premises A1 to A7 and `vkL_ok`, against the older `ProvenRegime` |
| BR-L2 restatement (per-fold PoW vector in `ProvenRegime`, two-OOD bound, `w_stack` bound, two-table `pv`, DAG key check and anchor checks in `wrap_refines`, two Fr sponge algorithms) | LW-015, SQG-012, FLG-009, HDM-010, SPL-010 | open |
| BR-L3 lemma 23 and the transcript-derived key checks | LW-015, FLG-009 | built at `bdb92973a` |
| BR-L4 `ReduceRel` in place of `ArithGadget` | LW-015 | open |
| DEC-L1 (O34) outcome-only statement | LW-007, LW-015 | open |
| ZK-L1 `wrap_view_only_claim` | GP-ZK, ZK-010 | built at `bdb92973a` under premise A7 |
| ZK-L2 padding uniqueness | GP-ZK, ZK-010, FLG-009 | open |
| CF-L1 effect equivalence | CF-012 | open |
| CF-L2 anchor atomicity | ANC-011 | open |

The GPU wrap prover, the prover daemon, the Midnight side and the derived gates have no Lean obligation in the design. Their specs say so.

## 8. Open items carried into the interface freeze

- Which layer opens the sealed SET head of a general run, and the effect-word form of the collection statement (LW-008, LW-009).
- The form of `prog_d` in the run statement of a general run. `FD/ARCHITECTURE.md` kit L AND WRAP puts `[prog_d, input_d, outcome_d, n_steps]` in the effect words, and kit ZERO KNOWLEDGE says that plain `prog_d` never reaches the ledger. The design does not reconcile the two. The freeze fixes it.
- The source of the claim address and `ep_hash` per chunk under placement V (ANC-010).
- The window position of E-1 under a variable-length SHA-256 (CF-010).
- The order of the machine-level kits against the build phases of `FD/ARCHITECTURE.md` section 3.0.
