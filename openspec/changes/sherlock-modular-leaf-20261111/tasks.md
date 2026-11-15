# Tasks

Status: S2, nothing ticked. `O/` is `openspec/changes/sherlock-vm-consensus-20261107/`. Order: the prerequisite experiments, the baseline, the interface freeze and the governing-spec amendments, then the build-order groups of `design.md` section 8 (V, 0, R, 1 to 4, L, 5 to 8). `FD/` is `research/sherlock-final-design-20261111/`. `T5` is `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT, owner ruling O12); a task that waits on T5 waits until the named row of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN.

Refreshed 2026-11-13 to the STARK design (row T6a of `research/sherlock-tested-design-20261113/LOOP.md`). The kits are OpenVM AIRs. Group V, the vertical slice, is new and comes first: its last task V.10 is the T6 done-when and the first kit milestone. The task ids of the other groups are unchanged, because the specs and `sherlock-processor-final-20261111` cite them.

Tick rule for every task: tick a task only when a `scripts/doctorwatson check <ID>` predicate passes with a receipt. The receipt names the positive tests, the mutant count with each control, the Quint verdict file (invariants, bounds, seeds, mutant results), the Lean theorems that the kit requires with their axiom reports (no `sorryAx`), the Rust check results and T0. A kit task names its spec and its gate. In a kit gate, "its positives and mutants" are the SG-2 and SG-3 real proofs on the stock stark-backend verifier, and every kit gate also needs SG-1 (every AIR required), SG-4 (fixed shape inside the layer bounds), SG-5 (D3 at the measured shape) and SG-6 (seconds per proved step) of `design.md` section 9. MockProver is not a kit gate. A landed kit gets a conformance task, not a done mark. A prototype milestone has its own task, and its tick never ticks a GP target.

Experiments X1 and X2 are done on main (`FD/EXPERIMENTS-MAXIMA.md`) and have no task here. The dense-memory part of X3 is done (`qa/gemini-quint/c1-dense/verdict.md`).

## X. Prerequisite experiments still open

- [ ] X.3 Run the hash-stack part of X3: write `models/sherlock/quint/hashStacks.qnt` with the compound step, run it in a Gemini Quint lane, and re-run it as the lead. Gate: a verdict file with `cachedTopRefinesList`, `compoundStepIsOneEvent`, `belowEmptyAtCountLe1` and every seeded mutant. Unblocks STK-005.
- [ ] X.4 Prototype block lines and ROM lines behind `open_line` as AIRs. Measure the columns, interactions and height per line and a locality census of old reads and program rows on the corpus. Gate: the measured shapes with run ids. Decides L-2, the early-close rate and the ROM depth cost (AD-12, waits T5 row I-6).
- [ ] X.5 Wire the dense backend into the STARK leaf shard (after V.10) with INIT, the `rules_d` sponge, the header-and-body boundary, the read port and hash-chain stacks. Measure the leaf cost at S = 16 and S = 64 in seconds per proved step, then the node cost and the `L` shape over that leaf key. Gate: run ids. Decides the shape bounds of the leaf, the wrap row gate input and AD-6 (DG-004, DG-006).
- [ ] X.6 Do task 4.18 of `sherlock-vm-consensus-20261107`: build a constructor of more than 8 fields across a leaf boundary with linked slices and with contiguous slices. Gate: the measured shape and steps for each form. Decides AD-5 (OWNER for the D-1 status).
- [ ] X.7 Measure the binary node shapes: leaf key direct against a one-leaf lift (N-1), the MERGE-WORK node with two enabled prior proofs and one real owner step (N-6), and the opening of the sealed SET head in kit L AND WRAP (N-5). Gate: run ids, node cost in seconds per proof, and the node inside the shape bounds of the internal set (DG-004, DG-006).
- [ ] X.8 Probe one Compact adapter class and the foreign lift relation. Gate: a recorded probe result. Decides the native GP-3 scope (N-10).
- [ ] X.9 Prototype the HASH constant-start mode with a differential test against `sponge_native` and a key-shape check. Gate: the differential passes and the fixed-column count is recorded. Decides V-5 (HSH-004).
- [ ] X.10 Measure `open_state_header` with the 42-word header sponge (FPR-012) over the slice leaf of V.9, inside the NODE seal modes, and the SET head opening of `L`, against `boundary_header_leaf_vs_std` (rows I-12 and I-35 of `FD/ARCHITECTURE.md` section 1.12). Gate: the AIR columns, interactions and heights against the internal set, and seconds per proof, with run ids. Decides AD-19 and N-2.
- [ ] X.11 Measure the input admission forms on the reference machine and a leaf prototype: for by-digest admission, the `AuthIn` read cost per step, the VIEW-WORK cost per entered record and any reference charge for `Literal(x)` cells, owner-event count against `apply(prog, x)`, a wide constructor input, and the loading-prefix form. Gate: a recorded table. Input to AD-17 and AD-18 (OWNER).
- [ ] X.12 Write `models/sherlock/quint/claimConsumption.qnt` with the SET grammar, DESCEND, DL-STEP and both seals, run it in a Quint lane, and re-run it as the lead. Then measure the node shape of each mode against the internal set. Gate: a verdict file with the six invariants of CLM-001 to CLM-005. Decides N-3.
- [ ] X.13 Count every projection gap kind over the corpus, and the 18 rules that the X1 corpus did not fire. Gate: a census file. Sizes the REFERENCE BRIDGE work and is input to AD-21.
- [ ] X.14 Write a costed operation inventory for one PLONK or halo2 verifier, and find the largest unit that fits the node beside one child verification, at most 3,300 columns with the 17-bit checker (rulings O1 and O2). Gate: the inventory with costs. Decides the FV-SCHED unit budget and AD-22.
- [ ] X.15 Log every ROM row read per step, add the sites to the static bound, and assert dynamic <= static. Gate: the extended `FD/maxima/rule_maxima.md`. Decides `K_rom` and the ROM part of `EffectBounds`.

## P. Baseline and freeze prerequisites

- [ ] P.1 Clear the Rust baseline: make `cargo clippy --workspace --all-targets --locked -- -D warnings` pass under the pinned toolchain. Gate: a clean clippy run with its run id.
- [ ] P.2 Fix the record layouts per kind and the physical cells per record (AD-11). Owner: the lead drafts in T5, the owner signs (O12). Gate: row I-5 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN, with a schema review against `O/design.md:220-267`.
- [ ] P.3 Fix the ROM depth over physical rows and `K_rom` (AD-12, V-1, V-2). Owner: the lead drafts in T5, the owner signs (O12). Gate: X.15, row I-6 reads FROZEN, and a reading of the program table index types. A smaller depth or a per-program depth is not an acceptance path.
- [ ] P.4 Collect the OWNER answers for every item of the OWNER table in `proposal.md`. Owner: the owner. Gate: each answer recorded in `FD/DECISIONS.md`.

## F. Interface freeze and governing-spec amendments

- [ ] F.1 Freeze the interfaces: the bus schema and the ports (`FD/ARCHITECTURE.md` sections 1.2 and 1.3), the ports manifest with `AuthRec`, `AuthRom`, the append chain and the state header, the hash domain registry, the state schema with its header and body split, the record layouts, the rule inventory, the node statement types and the node relation table. Owner: the lead. Gate: P.2, P.3, X1, X2 and the B2 audit answered.
- [ ] F.2 Amend SRC-021 and the Init table of `sherlock-vm-consensus-20261107` to the one-arena field list of kit INIT. Gate: AD-18 answered by the owner, and `openspec validate sherlock-vm-consensus-20261107` passes.
- [ ] F.3 Close the input-admission open item of `O/design.md:194`. Gate: AD-17 answered by the owner after X.11.
- [ ] F.4 Edit `O/design.md:80` to "one schema, two lanes". Gate: `openspec validate sherlock-vm-consensus-20261107` passes.
- [ ] F.5 Replace D2, D3, D6 and D7 of `O/design.md:95-100` by the dense memory and hash-stack rules with the compound stack step. Gate: the same validation.
- [ ] F.6 Record that SRC-014 requires `S_MAX` in the relation bytes, with the code change in task 5.1. Gate: the same validation.
- [ ] F.7 Amend SRC-007 and the contract templates for universal program acceptance and collection opening. Gate: N-11 answered by the owner.
- [ ] F.8 Name the semantic reference for the full language in the governing change. Gate: AD-21 answered by the owner.
- [ ] F.9 Add a mid-run consume rule to the governing VM specification only if AD-20 selects it. Gate: AD-20 answered by the owner.
- [ ] F.10 Write the contract and template work: one contract (kit LEDGER CONTRACT) with two entry points over one `runs` map, `acceptRun` with `prog_d` as call data and `acceptCollection` and `openClaim` (no `descend`: the depth is in the opening proof, FP-3 fact 3), and the template recapture (SRC-020). Gate: F.7 and the SRC-020 `validate_call` check with the five mutations refused.

## V. Group V: vertical slice (first kit milestone)

The slice proves one Sherlock program through every STARK layer before any kit is built in full (`design.md` section 8). Each slice task builds its kit only as far as the V.1 program needs, as real AIRs, and its gate uses SG-2 and SG-3 real proofs. A slice task ticks no kit task of groups 0 to 4 and no GP target. Entry for the group: the F.1 parts that the slice uses, row T3 of `research/sherlock-tested-design-20261113/LOOP.md` (every AIR required) for V.9 and V.10, and T5 rows I-5, I-9 and I-15 for V.6 to V.10.

- [x] V.1 Choose the slice program and its rule subset: the smallest Sherlock program whose run needs no stack push or pop, allocates fewer than 8 records and reads no ROM row beyond what INIT opens. Record the rule list, the DEC rows of the subset and, if no such program exists, which STACKS step (STK-001 to STK-003), flush (MBK-003) or ROM line (`specs/sherlock-chip-page-cache` PAGE-004) enters the slice and why. Gate: the recorded program, its rule list and its run on the landed reference machine.
- [x] V.2 Slice BUS (`specs/sherlock-chip-bus`, BUS-001, BUS-005, BUS-007, BUS-008): the FP-1 bus registry for the buses the subset uses, the `KitAir` and `KitTrace` traits, and the ports manifest of the slice ports. Gate: `bus_enc_vectors`, the mutants `bus_unassigned_chip`, `bus_limb_over_range` and `bus_port_received_twice` as real proofs, T0. Entry: F.1 (bus and ports part).
- [x] V.3 Slice RANGE (`specs/sherlock-chip-range`, RNG-001, RNG-002): the stock `VariableRangeCheckerAir` at 16 bits with the `u64` and `u128` limb shapes. Gate: the positives `range_64_boundaries`, `range_128_boundaries` and the mutants `range_limb_out_of_table`, `range_source_copy_removed` as real proofs, T0. Entry: V.2.
- [x] V.4 Slice HASH (`specs/sherlock-chip-hash`, HSH-002, HSH-003, HSH-005): P2 over BabyBear with the FP-4 tag codes for the digests that the slice uses (`rules_d`, the boundary header and body, the `prog_d` and `input_d` openings). Gate: the FP-4 vectors `digest_table_vectors` (FPR-011) against the Rust reference, the mutants `hash_domain_swapped`, `hash_round_skipped` as real proofs, T0. Entry: V.2.
- [x] V.5 Slice CORE and DEC (`specs/sherlock-chip-core` CORE-001 to CORE-003, CORE-006, CORE-009; `specs/sherlock-chip-dec` DEC-001, DEC-004 over the subset rows): slots with the enable prefix and heights, `rules_d`, and the rules of V.1 only. Gate: `core_height_edge`, `core_rules_d_reference_vectors`, one positive per branch of each subset rule against the V.1 reference run, the mutants `core_enable_nonprefix`, `core_rules_d_forged`, `dec_two_rules_selected` as real proofs, T0. Entry: V.1 to V.4. DEC rows wait on T5 row I-7 for the full inventory only.
- [x] V.6 Slice MEM window (`specs/sherlock-chip-mem` MEM-001 to MEM-003, MEM-006; CORE-005; the tail opening of MBK-002): dense append, `len`, the append chain, and the memory token `[root, tail_d]` carried through with no flush. Gate: the mutants `address_reuse_window_row`, `close_advances_len`, `counter_wrap_2_64`, `forged_tail_across_leaf_boundary` as real proofs, T0. Entry: V.2 to V.5, T5 rows I-5 and I-9.
- [x] V.7 Slice BOUNDARY COMMITMENT (`specs/sherlock-chip-boundary`, BND-001, BND-002): the Poseidon header and body over the slice state, bound to the FP-1 leaf public values. Gate: `ok_boundary_poseidon_two_leaf_stitch`, the mutants `boundary_word_omitted`, `header_body_d_unlinked` as real proofs, T0. Entry: V.4, V.6, T5 row I-15.
- [x] V.8 Slice INIT (`specs/sherlock-init-admission`, INIT-001, INIT-002, INIT-004, INIT-007): the genesis selector, the Init field relation with the empty genesis memory, the FP-4 openings of `prog_d` and `input_d`, and the ROM line under `prog_root` for the rows that the V.1 program reads (PAGE-004 over those rows only; V.1 records that every program reads `expressions[e0]`). Gate: `init_expr_entry_no_input`, `init_non_genesis_leaf_any_state`, the mutants `init_wrong_control`, `init_memory_not_genesis` as real proofs, T0. Entry: V.7, T5 row I-15. The SRC-021 field list also waits on AD-18.
- [x] V.9 Assemble the slice leaf shard: V.2 to V.8 in one fixed-shape leaf shard in place of the bench W1 app, keyed with every AIR required (SG-1). Gate: the keyed shape inside the app set (SG-4), `slice_air_omitted` refused at N1, N3 and `L`, the per-AIR columns, interactions and heights, and seconds per proved step (SG-6), with run records. Entry: V.2 to V.8, T3.
- [x] V.10 T6 done-when, the first kit milestone: one run of the V.1 program proves leaf, N1, N2, N3, `L` and `L_Fr` on one key set, every leaf shard of the run has the fixed height of its shape, and D3 on the frozen set of every layer at the measured shapes prints every term at 100 bits or more and every grindable term at 101 or more (SG-5). Gate: the run record of every layer, the D3 log, and a `scripts/doctorwatson check V.10` receipt. Entry: V.9, T3, T5 rows I-5, I-9 and I-15. A failed D3 term goes to the owner (DG-010). This task ticks no kit task and no GP target.

## 0. Group 0

- [ ] 0.1 Implement kit BUS (`specs/sherlock-chip-bus`, BUS-001 to BUS-008): tagged table, ports manifest, row-role allocator. Gate: its positives and mutants, the `busContract.qnt` verdict, Lean O1 and O42 proved, T0. Entry: F.1, V.10.

## R. Group R (host, beside groups 0 to 4)

- [ ] R.1 Implement kit REFERENCE BRIDGE (`specs/sherlock-reference-bridge`, RFB-001 to RFB-004): deliverables R1 to R4. Gate: its host tests and seeded-divergence controls, coverage report with zero gaps, T0. Entry: F.1, AD-21, X.13.

## 1. Group 1

- [ ] 1.1 Implement kit RANGE (`specs/sherlock-chip-range`, RNG-001 to RNG-003): 64-bit shape and tagged query. Gate: its positives and mutants, Lean O3, T0. Entry: 0.1.
- [ ] 1.2 Implement kit HASH (`specs/sherlock-chip-hash`, HSH-001 to HSH-005): domain registry. Gate: its differential vectors and mutants, the `hashProtocol.qnt` verdict, Lean O7 stated, T0. Entry: 0.1.
- [ ] 1.3 Conform kit MEM window (`specs/sherlock-chip-mem`, MEM-001 to MEM-007) to the frozen ports and move the C1 model to `denseMemory.qnt`. Gate: the landed and added positives and mutants, the dense-model verdict re-run, Lean O12, O13, O43, O47 proved, T0. Entry: 0.1, AD-1 for TOP.

## 2. Group 2

- [ ] 2.1 Implement kit ALU (`specs/sherlock-chip-alu`, ALU-001 to ALU-004): in-chip operand ranges, SUB, MUL, EQ. Gate: its positives and mutants, the `aluArith.qnt` verdict, Lean O4, T0. Entry: group 1.
- [ ] 2.2 Implement kit DEC (`specs/sherlock-chip-dec`, DEC-001 to DEC-004) over the 120-rule inventory. Gate: its positives, mutants and configuration failures, the `decDispatch.qnt` verdict, Lean O5 and O6, T0. Entry: group 1.
- [ ] 2.3 Implement kit STACKS (`specs/sherlock-chip-stacks`, STK-001 to STK-005). Gate: its positives and mutants, the X.3 verdict, Lean O11 and O46, T0. Entry: group 1, X.3.
- [ ] 2.4 Conform kit MEMORY BACKEND dense (`specs/sherlock-chip-membackend`, MBK-001 to MBK-005) and add the seam test and the new mutants. Gate: its positives and mutants, the dense-model verdict, Lean O14, O15, O16, T0. Entry: group 1.
- [ ] 2.5 Implement kit VALUE DIGEST (`specs/sherlock-value-digest`, VDG-001 to VDG-007). Gate: `vd_differential_every_form`, its positives and mutants, the `valueDigest.qnt` verdict, Lean O23 to O27, T0. Entry: group 1, X.9 for the constant-start mode.

## 3. Group 3

- [ ] 3.1 Implement kit PAGE/ROM block cache (`specs/sherlock-chip-page-cache`, PAGE-001 to PAGE-006). Gate: its positives and mutants, the `denseLineCache.qnt` verdict, Lean O17 to O20, T0. Entry: group 2, X.4, X.15.
- [ ] 3.2 Implement the CORE framework (`specs/sherlock-chip-core`, CORE-001 to CORE-010, and CORE-013 if AD-17 selects form (b)): slots, ports, frame condition, append chain, read port, `rules_d`, definite assignment. Gate: its positives, mutants and controls, the framework part of the `processorCore.qnt` verdict, Lean O10, O44, O45, T0. Entry: group 2. O8 is over the full rule set and is a gate of L.1.

## 4. Group 4

Each family task implements CORE-011 and CORE-012 for the rules of its family, against its `RuleContract`s and the REFERENCE BRIDGE. Gate of each: one positive per branch, one mutant per written register class, per charge and per failure branch, the per-rule maximal-step positive, the family part of `processorCore.qnt`, T0. Entry: 3.2, the per-rule table (X1, and X.15 for ROM).

- [ ] 4.1 Implement CORE rule family: publications. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.2 Implement CORE rule family: helper group Scalar. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.3 Implement CORE rule family: helper group EnvGet. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.4 Implement CORE rule family: helper group EnvPut. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.5 Implement CORE rule family: helper group Find. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.6 Implement CORE rule family: helper group Bind. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.7 Implement CORE rule family: helper group Append. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.8 Implement CORE rule family: helper group Validate. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.9 Implement CORE rule family: helper group Equal. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.10 Implement CORE rule family: helper group Match. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.11 Implement CORE rule family: legacy sites. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.12 Implement CORE rule family: entry rules. Gate: the family gate above for every rule of the family. Entry: 3.2.
- [ ] 4.13 Implement CORE rule family: ADT Build group. Gate: the family gate above for every rule of the family. Entry: 3.2, R.1, X.6.
- [ ] 4.14 Implement CORE rule family: ADT Plan group. Gate: the family gate above for every rule of the family. Entry: 3.2, R.1.
- [ ] 4.15 Implement kit BOUNDARY COMMITMENT (`specs/sherlock-chip-boundary`, BND-001 to BND-004). Gate: its positives and mutants, `boundary_header_leaf_vs_std`, the `boundaryCommitment.qnt` verdict, Lean O28, O29, O48, T0. Entry: 3.2.
- [ ] 4.16 Implement kit INIT (`specs/sherlock-init-admission`, INIT-001 to INIT-005 and INIT-007, and INIT-006 if AD-17 selects form (b)). Gate: its positives, mutants and controls, the extended `init.qnt` verdict, Lean O49 and O50, T0. Entry: 4.15. The input part also waits for AD-17, AD-18 and X.11.

## L. Leaf integration

- [ ] L.1 Integrate all leaf kits in the STARK leaf shard, extending the slice AIRs of group V. Gate: the whole-leaf harness `crates/mori-chip/tests/diff_leaf_vs_sherlock_ka1.rs` green under three partitions on real proofs, the REFERENCE BRIDGE coverage report at zero gaps, the leaf cost in seconds per proved step and every menu shape inside the app set bounds measured (X.5), D3 at the measured shapes (SG-5), Lean O8 and O9 proved. Entry: group 4, R.1.
- [ ] L.2 Key the integrated leaf shapes with every AIR required (SG-1). Gate: the leaf keys under `key_root` and their manifest entries recorded (SRC-008). Entry: L.1.

## 5. Group 5

- [ ] 5.1 Implement kit NODE INTEGRATION (`specs/sherlock-node-integration`, NODE-001 to NODE-011, and NODE-012 and NODE-013 if AD-17 selects form (b)), with `open_state_header`, `S_MAX` in the relation, the `rules_d` lane rename, the `reg_root` equality across slots, the seal admission check and the ADMIT-WORK mode (and the VIEW-WORK mode under form (b)). Gate: its positives, mutants and host refusal, the `nodeIntegration.qnt` verdict, Lean O31 to O33 and O54 (O55 under form (b)), a fixed STARK node shape inside the internal set as raised on 2026-11-12 (64 AIRs, 3,000 columns, 200 interactions per AIR; DG-004 and DG-006 of `sherlock-processor-final-20261111`; the halo2 gate of 950,000 node rows is history, DG-007), T0. Entry: L.1, X.7, X.10.

## 6. Group 6

- [ ] 6.1 Superseded 2026-11-11: kit DECIDER is not built. Its requirements DCD-001 to DCD-007 are met by kit L AND WRAP of `sherlock-processor-final-20261111` (LW-002 to LW-011, DG-001 and DG-002, tasks 3.2 and 3.4 of that change). Gate: those tasks, with the `deciderOutcome.qnt` verdict and Lean O34. Entry: 5.1.
- [ ] 6.2 Implement kit HOST CHECKPOINT STORE (`specs/sherlock-checkpoint-store`, CKPT-001 to CKPT-003). Gate: its host tests and host refusals, T0. Entry: 5.1.
- [ ] 6.3 Implement kit CONTINUATION (`specs/sherlock-continuation`, CONT-001 to CONT-004). Gate: its positives, mutants and host refusals, the `continuation.qnt` verdict, Lean O36 and O37, T0. Entry: 6.2.
- [ ] 6.4 Implement the native path of kit GP-3 hook (`specs/sherlock-foreign-registry`, FREG-001 to FREG-004). Gate: its positives and mutants, the `foreignRegistry.qnt` verdict, Lean O39, T0. Entry: 5.1, X.8. This task is a milestone and does not tick GP-003.
- [ ] 6.5 Implement kit GP-4 hook (`specs/sherlock-proof-shape`, SHP-001 to SHP-006). Gate: its positives and mutants, the `pcdGraph.qnt` verdict, Lean O40 and O41, T0. Entry: 5.1, X.7. If X.7 does not fit, GP-004 stays open and the owner decides (N-6).
- [ ] 6.6 Run the key cascade: key the per-program leaf keys under `key_root` over the new leaf AIRs, and check that the universal internal-recursive, `L` and wrap keys stay byte-equal. Gate: the key manifest with every key recorded (SRC-008). Entry: L.2, 5.1, 6.1.

## 7. Group 7

- [ ] 7.1 Implement kit GP-2 hook (`specs/sherlock-claim-consumption`, CLM-001 to CLM-007). Gate: its positives and mutants, the X.12 verdict, Lean O38, O51 and O52, T0. Entry: group 6, X.12. Scope per AD-20.
- [ ] 7.2 Implement kit LEDGER CONTRACT (`specs/sherlock-ledger-contract`, LDG-001 to LDG-005): one Compact contract with one shared ledger state. Gate: its local contract tests and named refusals, the Quint verdict for `ClaimSettledOnceAcrossEntries`, `DiamondAncestorOpenedAtMostOnce` and `EveryJoinedClaimOpens`, Lean O35 (LDG-L1), T0. Entry: 6.1, 7.1 (SET grammar), F.10.

## 8. Group 8

- [ ] 8.1 Implement subkit FV-SCHED of the GP-3 general verifier (`specs/sherlock-foreign-verifier`, FVR-001, FVR-002, FVR-004, FVR-005: the `work_root` index schema and its initialization). Gate: its mutants, the extended `foreignRegistry.qnt` verdict, T0. Entry: 6.4, X.14.
- [ ] 8.2 Implement subkit FV-PARSE (FVR-001, FVR-005 writes). Gate: its mutants (`fv_skip_proof_element`, `fv_trailing_element_ignored`, `fv_point_not_on_curve`), T0. Entry: 8.1.
- [ ] 8.3 Implement subkit FV-TRANSCRIPT (FVR-002, FVR-005 writes). Gate: `fv_challenge_not_from_transcript`, `fv_challenge_index_swapped`, T0. Entry: 8.1.
- [ ] 8.4 Implement subkit FV-ARITH (FVR-001, FVR-005 reads). Gate: the differential against a host verifier, `fv_operand_substituted`, T0. Entry: 8.1.
- [ ] 8.5 Implement subkit FV-FINAL (FVR-003, FVR-005 reads). Gate: `pos_fv_plonk_proof_same_srs`, `pos_fv_plonk_proof_other_srs`, `fv_other_srs_folded_into_core_acc`, T0. Entry: 8.1.
- [ ] 8.6 Complete GP-3: the GP-3 hook and all five subkits pass together. Gate: `pos_gp3_sherlock_plus_compact`, `pos_fv_plonk_proof_same_srs` and `pos_fv_plonk_proof_other_srs` as real proofs, Lean O53 (GP3-L2, completion gate and stream integrity). Entry: 6.4, 8.1 to 8.5.

## A. GP acceptance matrix and qualification

Each task is one line of the GP acceptance matrix of `FD/ARCHITECTURE.md` section 3.3. Run heavy work only through `scripts/doctorwatson heavy`.

- [ ] A.1 GP-001: one key set and one verifier contract accept proofs of at least three programs that the key set was not built from. Gate: real-proof integration test with one contract address.
- [ ] A.2 GP-001: among them one program larger than one leaf's carriers, one run with input, and one run of each outcome kind. Gate: the same test.
- [ ] A.3 GP-001: one run of more than one leaf that allocates more memory than one leaf holds and builds a constructor of more than 8 fields across a leaf boundary. Gate: the same test, with R.1 complete.
- [ ] A.4 GP-001: the final proof has the same byte length as a one-leaf proof. Gate: byte lengths recorded.
- [ ] A.5 GP-001: the allocating run under three partitions and with one stop and resume gives the same run statement and private `n_steps`, and the second submission is refused by `claim_pub`. Gate: CONTINUATION and HOST CHECKPOINT STORE tests, SRC-023.
- [ ] A.6 GP-001: at the constraint boundary `2^64 - 1` is accepted and `2^64` refused. Gate: `pos_node_height_2p64_minus_1`, `node_height_wrap`.
- [ ] A.7 GP-002 (a): a run of B whose input is the verified result of a run of A, accepted once on chain. Gate: `pos_gp2_b_consumes_a_halted` as a real proof.
- [ ] A.8 GP-002 (b): one proof that aggregates at least three runs of different programs, accepted once on chain, with each claim opened. Gate: `pos_decider_collection_three_runs` as a real proof.
- [ ] A.9 GP-003: one proof that aggregates a Sherlock run and at least one Compact contract proof, checked against the registry, accepted on chain. Gate: task 8.6.
- [ ] A.10 GP-004: one key set proves a chain, a balanced tree and a DAG with one claim feeding two consumers, each at depth >= 8, with equal final proof size. Gate: real-proof acceptance test.
- [ ] A.11 GP-004: a node with two prior proofs plus new work. Gate: `pos_gp4_two_prior_plus_step` as a real proof of one MERGE-WORK node.
- [ ] A.12 Run the heavy end-to-end qualification: SRC-010 independent verifier and SRC-020 `validate_call` with the five mutations refused. Gate: receipts of both.

## B. Profile B (not part of the completion of this change)

- [ ] B.1 Benchmark Profile B (`specs/sherlock-chip-zmlogup`, ZML-001 to ZML-005, and BND-005) after M4, only if Profile A misses a throughput target (AD-7). Gate: its positives and mutants, the `writeOnceLogup.qnt` verdict, Lean O21, O22 and O30.
