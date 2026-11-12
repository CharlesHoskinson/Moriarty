# Tasks

Status: S2, nothing ticked. The order is the build order of `PR` section 10 (`FD/ARCHITECTURE.md` section 3.0). Heavy jobs run one at a time through `scripts/doctorwatson heavy --id <id> -- <cmd>`.

Tick rule: tick a task only when a `scripts/doctorwatson check <ID>` predicate passes with a receipt. The receipt names the positive tests, each mutant with its weakened control, the measurements with their run records, and the Lean theorems with their axiom reports. A planned command is not evidence.

## G. Governance

- [ ] G.1 Replace the halo2 gates of `openspec/changes/sherlock-vm-consensus-20261107/design.md` section 2.7.9 by a dated pointer to DG-001 to DG-007. Keep sections 2.7.10 and 2.7.11 as dated history. Gate: `openspec validate sherlock-vm-consensus-20261107` passes, as for tasks F.4 and F.5 of `sherlock-modular-leaf-20261111`. Strict validation of that change is an owner item, because its council and GP requirement texts are longer than the strict limit and no word of them may change.
- [ ] G.2 Collect the owner answers to OD-4 to OD-18, F-1 to F-13 and FZ-1, FZ-2. Owner: the owner. Gate: each answer recorded in `research/sherlock-final-design-20261111/DECISIONS.md`, and every requirement that names a reversed decision updated.
- [ ] G.3 Fetch WHIR (ePrint 2024/1586) and BCGM26 and read both in full. Owner: the owner fetches. Gate: the papers on disk and a reading record. Feeds SQG-010.

## 1. Phase 1: first build items, no heavy job

- [ ] 1.1 Frontend M0: golden vectors for every example entry, with four mutants. Output: `tools/sherlock-compact/`. Gate: CF-003 scenario passes. Verify: the golden-vector test of the frontend.
- [ ] 1.2 Frontend M1: the reference machine over `R_SL_*` in `crates/mori-core/src/sherlock/sl/`, differential against native. Gate: CF-004 scenario passes. Verify: `cargo test -p mori-core --locked`.
- [ ] 1.3 E-1 placement V: `bind_label` on the `sha2` chip and the V+ `layout_d` word. First D4 and T0 on calls with a fallible transcript or non-empty effects, then T2 with every mutant and the `label_d` link, then T3. Gate: CF-008, CF-009 and CF-010 scenarios.
- [ ] 1.4 Atomic chunked calls: T1 at ledger level with anchor values in the `C_P` and `A_j` form, then the chunked mode (R-phase, R-cut, R-ctx, the anchor statement) and the native anchor prologue. Gate: ANC-001 to ANC-009 scenarios at ledger level.
- [ ] 1.5 stark-backend parameter code: explicit per-round query vectors with the keygen check, query PoW 21 for the app, leaf-aggregation and internal sets, the unique decoding internal and `L` sets of OD-19 (`WhirProximityStrategy::UniqueDecoding` in every round, one OOD sample, folding PoW 0, `mu` PoW 9 internal and 6 for `L`), the global interaction bound, the `L` Merkle hash and transcript. Gate: SQG-002, HDM-001 to HDM-006, and the T-FS vectors of HDM-009 on the CPU and GPU provers and the stark-backend verifier.
- [ ] 1.6 Rerun D3 on the frozen sets at their shape bounds, with `L` at the `w_stack` of D10 (at most 48, `mu` PoW 6). Gate: SQG-007.
- [ ] 1.7 Lean: the restatement BR-L2 on the round 2 files of branch `bridge-lean` (`bdb92973a`), with the unique decoding regime, the frozen query vectors and one OOD sample in `ProvenRegime`. Output: `clean/fork/Clean/Sherlock/`. Gate: SQG-009. Verify: `lake build SherlockSpec` and the axiom report, through `scripts/doctorwatson heavy`.

## 2. Phase 2: kernels and STARK path

- [ ] 2.1 The Fr Poseidon GPU kernel for `L`, starting with the microbench D7. Gate: D7 recorded against 5 to 15 M permutations per second. OD-10 applies if it slips.
- [ ] 2.2 CPU wrap items: affine SRS, cached twiddles and parallel bit reversal, dynamic chunks, limb sort, multiopen fix. Gate: GPW-003 byte identity after each item.
- [ ] 2.3 sppark one-batch patch and window from the call size, then D9. Gate: GPW-005 recorded.
- [ ] 2.4 Frontend M2a: base chip set leaf AIRs, the planner, `key_root` under `prog_d`, byte-equal internal-recursive keys across two chip sets. Gate: SPL-002 to SPL-005 and CPR-003 scenarios.
- [ ] 2.5 Atomic chunks T2 on the XC-9 devnet with T2-N and T2-N0. Gate: ANC-009 and ANC-010 scenarios.

## 3. Phase 3: the wrap

- [ ] 3.1 D10: `L` tracegen over one node proof for the trace heights, then `w_stack` and the high row case. A bound tighter than 48 reruns D3. Gate: SQG-005 and DG-005.
- [ ] 3.2 M7 (D15): the G4 wrap circuit with the wrap-rows bundle, V+, the anchor statement checks, the DAG key check and the direct writer, then the cost model on the real `L` shape. Output: `crates/mori-recursion/src/wrap/`. Gate: LW-002 to LW-006, LW-013, DG-002, DG-003. Freeze k, `L_TX`, the capacity with the `pv` encoding, the hash domains and the anchor template with the wrap key only after task 3.5.
- [ ] 3.3 Batched MSM on the MSM skew mask, then the NVRTC numerator (D14). Gate: GPW-004.
- [ ] 3.4 Frontend M2b: one wrap proof that the ledger 9 verifier accepts with `2 + L_TX` public inputs, the negative controls and EQ-ZK. Gate: LW-003, LW-012, ZK-003, MID-001.
- [ ] 3.5 Security qualification gate: G1 to G4 pass with G5 (discharged at the paper level). Gate: SQG-006 to SQG-010. Then freeze the universal keys (SQG-011, SPL-009).

## 4. Phase 4: on chain and coverage

- [ ] 4.1 Path C and the resident daemon with its slot pool. Output: `crates/sherlock-prover/src/daemon.rs`, `slots.rs`. Gate: PRD-001 to PRD-004 and D12.
- [ ] 4.2 Frontend M3 on the local devnet. Gate: CF-011 and MID-004 refusals.
- [ ] 4.3 Frontend M4: acceptance sets 1 and 2 end to end with the forced-overflow cases and `R` reserved. Gate: LW-004 and FLG-006 scenarios.
- [ ] 4.4 Frontend M5: Jubjub contracts, one GP-2 consumption and one GP-3 aggregation. Gate: CF-011.
- [ ] 4.5 E-1 T4 to T6 on the real wrap, and atomic chunks T3 with universal-key chunks. Gate: CF-010 and ANC-010.
- [ ] 4.6 Preview deploy and call of the wrap key. Gate: MID-004 and MID-006. Mainnet stays under SWF-009.

## 5. Zero knowledge and GP acceptance

- [ ] 5.1 The "what lives where" Quint property: no STARK proof, checkpoint or plan leaves the user's machine. Gate: ZK-001 scenario with a verdict file.
- [ ] 5.2 The seeded-proof guard and the reused-slot test. Gate: ZK-003 and PRD-002 scenarios.
- [ ] 5.3 Lean ZK-L2, CF-L1, CF-L2, BR-L4 and DEC-L1. Gate: proved theorems with axiom reports that hold only `propext`, `Quot.sound` and `Classical.choice`.
- [ ] 5.4 GP-ZK evidence on every GP acceptance line of `FD/ARCHITECTURE.md` section 3.3. Gate: GP-ZK scenarios. No GP target is ticked without it.
