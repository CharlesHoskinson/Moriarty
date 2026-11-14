# T8 results: the GP hooks on the node layer

Row T8 of `research/sherlock-tested-design-20261113/LOOP.md`. Trees, all on branch `t8-gp`: Moriarty `~/Projects/Moriarty-wt/t8-gp` (base `fc830df0f`), openvm `~/.cache/m7-wrap/src/openvm-t8` (base `662235f`, the T7 tip), stark-backend `~/.cache/m7-wrap/src/stark-backend-t8` (no change). State, logs and outputs under `~/.cache/t8/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

Status: X12 node modes and N-4 done on real proofs and in Quint; X7, X8 and X14 measured or named; no GP hook row can read FROZEN yet (reasons per row below).

1. **N-4 reading: option (a). ACCEPTED by the lead (2026-11-13).** No circuit relation carries a checkpoint tag; the tag is on the persisted record digest only. Reason: `continuation.qnt` (section 1) shows the circuit side holds with an untagged record (`cont_recordTagRun --invariant circuitSide` `[ok]`), and all 7 sampled mutants are caught.
2. **`SHK_CKPT` code (LEAD pick).** Record tag, host digest, next free BabyBear code: 67, after `SHK_FV` 65 and `SHK_REG_ENTRY` 66.
3. **X7 and N-6: OWNER.** The MERGE-WORK node with the inline kernel is 48 AIRs, 4,246 columns, 1,476 constraints in `CoreAir` (MEASURED, section 2), over the 3,000-column and 1,000-constraint bounds. T8 does not trim the kernel or pick a layout (lead ruling). GP-4 inline merge work stays FREEZE-PENDING N-6.
4. **X12 readings (LEAD, built and MEASURED in section 3):** SEAL has no admission premise (slot 1 off; ADMIT-WORK is group 5); DL-STEP slot 0 is SET or SUB, slot 1 is DL or off; the link element joins its producer SET into the cover; the head opening is LEAF or DEP only (FOREIGN waits for GP-3); cover presence is tested by the inverse of the word sum (honest failure probability about 2^-31, DERIVED: one BabyBear element); an ABSENT cover is the zero digest; MERGE no longer requires equal `shape_d` (T7 `merge_check` and the merge rule in `stmt.rs` edited); the N1 output keeps `shape_d = 0` (the LEAF base case of `shape_d` with `rules_d`, AD-23 (a), is not built: `graph_leaf_shape_free` stays open); FV rows keep the T7 stand-in output with the GP mode bits forced to 0.
5. **GP-2 blocker: no Halted leaf run.** The T6 slice leaf only proves the unbound run (Rejected). The honest SEAL-WITH-DEPS of a run that consumes a prior run's value cannot be proved until the leaf kit has the literal and publish-value rules. All forged dependency, link and value cases are refused (MEASURED). GP-2 hook stays FREEZE-PENDING on that positive case.
6. **L needs a SET root (T7 or T10).** L takes a SEG root (1,986 of 2,000 columns, 93 of 100 interactions in T7). Accepting a SET or DEP root of the T8 block (FP-3, LW-008) is a change to L, not made here. No edit to `crates/recursion/src/proof_shape/` or the path C seams.
7. **O8 pick (LEAD, for owner confirmation): `compact-zkir3-kzg-bls12381-blake2b/1`** (section 4). GP-3 hook stays FREEZE-PENDING: no in-circuit verifier of the class, the transcript and expression-evaluation AIRs are not built (X14), and N-8.
8. **X14 under O1 and O2:** unit sizes MEASURED in X14 (section 5); GP-3 general verifier stays FREEZE-PENDING on the transcript, expression and FV-FINAL AIRs.
9. **ARCHITECTURE rows (proposed text, not edited by T8):** CONTINUATION and HOST CHECKPOINT STORE: N-4 closed by `continuation.qnt` with option (a) (lead rerun of the Quint set pending; the host refusals of HOST CHECKPOINT STORE outside the model stay open). GP-2 hook: "FREEZE-PENDING the Halted leaf run (honest SEAL-WITH-DEPS); X12 node modes built and MEASURED in T8 (34 cases)". GP-3 hook: "FREEZE-PENDING the X8 verifier of `compact-zkir3-kzg-bls12381-blake2b/1`, N-8". GP-4 hook: "FREEZE-PENDING N-6 (owner); per-mode `shape_d` MEASURED in T8". I-13: FREEZE-PENDING the same gates.
10. **Circuit changes for the Carbon Quint lanes (openvm `c26e247`):** `crates/sherlock-lfr/src/gp.rs:239` `eval` (mode bits, SEAL, SEAL-WITH-DEPS, DESCEND, DL-STEP, JOIN, `shape_d` constraints and sponge calls); `gp.rs:530` `fill` and `gp.rs:700` `shape` (trace); `crates/sherlock-lfr/src/stmt.rs:609` and `:683` (`gpc` columns after `fvs` for Node and Fv); `stmt.rs:957-991` (GP eval call, T7 merge rules gated by the MERGE bit, SHAPE_D output skipped for Node); `stmt.rs:356` (`merge_check` drops the equal-`shape_d` rule); `stmt.rs:1298` (`Wit::NodeGp`); `stmt.rs:1434-1465` (node trace with `gp::fill` and `gp::shape`, FV rows with bit 0); `stmt.rs:1517` (`sponge_height` for Node and Fv 32 to 64). Suggested lane: the X12 claim-consumption model `qa/gemini-quint/x12/claimConsumption.qnt` against `gp.rs` eval.

## 1. N-4: `continuation.qnt`

Model `models/sherlock/quint/continuation.qnt`, tests `models/sherlock/quint/continuation_test.qnt`, runner `research/sherlock-t8-gp-20261113/quint/rerun-continuation.sh`, logs `quint/runs/r2/` (quint 0.32.0, 5,000 samples, 40 steps, seed `0x20261113`).

What the model holds: one genuine run that the uninterrupted proof seals as Exhausted at `h = 6` (fuel 6, 7 steps to Halted, so a refreshed fuel would reach a different claim); leaves of one step proved by workers in any order; the tier 1 carry stack; MERGE (interval and stitch), SEAL (`h_lo = 0`, terminal) and the genesis gate of the leaf as the circuit relations; mode 1 records with the `CKPT` tag; an adversarial host that resumes from tampered records, proves leaves from any state it holds at any claimed height, merges and seals any proven SEG, and offers any digest it holds to a consumer that accepts only the head of a verified `LEAF` SET.

Invariants (the four of the kit plus three): `ResumeProducesIdenticalClaimD`, `OutOfOrderWorkerCompletionPreservesSegInterval`, `CheckpointRejectedBySealAndGP2`, `MaxStackDepthLeLogLPlus2`, `AdminStepsKeptApart` (pause and resume change no height, state or fuel), `RecordNeverParsesAsClaim` (N-4 (a)), `NoTerminalCheckpoint` (Exhausted is never a checkpoint trigger).

Lead-style runs (MEASURED, `quint/runs/r2/summary.txt`):

| Command | Result |
|---|---|
| typecheck model and tests | exit 0 |
| `continuation_test`: positives (out-of-order tier 1, resume at a power-of-two boundary, at an odd leaf and twice) and the 7 named mutants as attacks on the sound instance | 11 passing |
| one module per mutant (`noStitch` 2 cases, `noInterval`, `noGenesis`, `noTerminal`, `recordTagRun`, `consumeAnyRun`, `anyRunUntagged`, `pauseTerminal`) | every module passes: the attack completes and the named invariant breaks (or, for `consumeAnyRun`, the record tag still stops it) |
| `run` sound instance, `soundAll` and each invariant alone | `[ok]` |
| `run cont_recordTagRun --invariant circuitSide` | `[ok]`: the circuit side holds with an untagged record |
| reach witnesses `reachSealed`, `reachSealedAfterResume`, `reachConsumed` | `[violation]` as expected (not vacuous) |
| sampled mutants `noStitch`, `noInterval`, `noGenesis`, `noTerminal`, `recordTagRun`, `anyRunUntagged`, `pauseTerminal` | `[violation]` (caught) |
| sampled `consumeAnyRun` | `[ok]`: the record tag stops the tag-only consumer (expected) |

Named mutants of the kit and where each is refused: `cont_refresh_fuel` and `cont_change_input` by the MERGE stitch; `cont_skip_predecessor` by the MERGE interval; `cont_reset_height` by the genesis gate of the leaf; `cont_publish_as_run` by the SEAL terminal check; `cont_ckpt_as_gp2_claim` by the consumer's SET-membership check; Exhausted as a checkpoint by the scheduler rule. `cont_tampered_read_cell_unvalidated` and the host refusals of the HOST CHECKPOINT STORE kit are outside this model (no memory cells).

## 2. X7 and N-6: inline merge work (GP-4)

Probe: `t8_gp x7probe` (openvm `crates/sherlock-wrap-export/examples/t8_gp.rs`, mode `x7probe`), record `~/.cache/t8/out/x7/x7probe.json`, log `~/.cache/t8/logs/x7probe.log`. The probe builds the MERGE-WORK node layout of N-6 (the universal node AIR set with the 7 slice leaf AIRs inlined as the work kernel) and checks it against the internal set bounds. It does not prove.

| Layout | AIRs | Columns | Max constraints (AIR) | Max interactions | Bounds (64 / 3,000 / 1,000 / 200) | Label |
|---|---|---|---|---|---|---|
| universal node alone (N2 position) | 41 | 1,885 | 282 | 120 | inside | MEASURED |
| node + inline kernel, over N1 | 48 | 4,246 | 1,476 (`CoreAir`) | 145 | **columns and constraints outside** | MEASURED |
| node + inline kernel, over N2 | 48 | 4,246 | 1,476 (`CoreAir`) | 145 | **outside** | MEASURED |
| kernel alone (7 AIRs) | 7 | 2,361 | 1,476 | 145 | constraints outside | MEASURED |

Kernel per AIR (columns / constraints / interactions, MEASURED): `CoreAir` 676 / 1,476 / 145, `DecAir` 9 / 18 / 1, `VariableRangeCheckerAir` 4 / 10 / 1, `SpongeAir` 321 / 326 / 2, `Hash2Air` 29 / 5 / 5, `RomAir` 1,222 / 78 / 77, `MemAir` 100 / 100 / 21.

Reading: the inline kernel does not fit one node key under the bounds of the internal set. The overrun is 1,246 columns (DERIVED, 4,246 minus 3,000) and 476 constraints in `CoreAir` (DERIVED). The choice between trimming the kernel, splitting `CoreAir`, or a different N-6 layout is an owner decision (N-6 in section 0). T8 does not pick. GP-4 inline merge work stays FREEZE-PENDING on N-6.

## 3. X12: GP-2 node modes on real proofs (consuming a prior Sherlock run)

Code (openvm `t8-gp`, commit `c26e247`; the runs below used `1d39339`, same git tree `ad8341b`): `crates/sherlock-lfr/src/gp.rs` (new, 737 lines: the GP-2 and GP-4 mode block of the node), hooks in `crates/sherlock-lfr/src/stmt.rs`, run mode `gp2` of `crates/sherlock-wrap-export/examples/t8_gp.rs`. Record `~/.cache/t8/out/gp2/gp2.json`, log `~/.cache/t8/logs/gp2-1.log` (shared slot, 191 s wall, not a timing measurement; the log header says `dirty 2` because the run started before the code was committed, same content).

The run proves the slice leaf on programs A (`entry x;`) and B (`entry y;`), carries each through leaf, N1, N2 and N3 to a SEG root, then proves every GP-2 mode as an N3 node over N3 children on the one universal key set (N3 over N3 is the fixed point: `n3_over_n3_pre_equals_n3_pre=true`, MEASURED).

Node modes in the block (`gp.rs:1-35` documents the preimages): MERGE [0] (T7 rules, gated by the MERGE bit), SEAL [2] `SEG -> SET` (`h_lo = 0`, `post_d = P2(SHK_SD2, header)`, terminal outcome, `claim_d = P2(SHK_RUN, 1, prog_d, input_d, outcome_d)`, `set_root = P2(SHK_SET_LEAF, claim_d, h_hi)`), SEAL-WITH-DEPS [3], DESCEND [5] `SET -> SUB`, DL-STEP [4] (premise, input-link and chain elements), JOIN [6] `SET + SET -> SET`. Every output carries `shape_d = P2(SHK_SHAPE, T_NODE, mode, slot bits, key selectors, child shapes, ...)`.

Key shapes after the change (MEASURED, `keys` of `gp2.json`, all accepted by the G1 check): N2 and N3 41 AIRs, 2,182 columns, 809 constraints, 170 interactions, degree 4 (bounds 64 / 3,000 / 1,000 / 200); N1 41 / 2,435 / 620 / 144; FV unit (N2 shape) 41 / 2,182 / 756 / 170.

Cases (MEASURED, 34 of 34 pass, `T8GP2 all_pass=true cases=34`):

| Group | Positive (accepted) | Refused (forgery) |
|---|---|---|
| SEAL | `pos_gp2_seal_A`, `pos_gp2_seal_B` | `claim_consume_unsealed_segment` (Running header), `seal_header_not_post_d`, `claim_nsteps_forged_in_set_leaf`, `claim_swap_outcome`, `claim_seal_undischarged_deps`, `probe_seal_deps_premise_only` |
| JOIN and DESCEND | `pos_gp2_join_A_B`, `pos_gp2_descend_join_left`, `pos_gp2_descend_join_right` | `join_set_with_segment_refused`, `claim_descend_wrong_domain`, `claim_sub_root_unrelated`, `claim_head_of_join` |
| DL-STEP (dependencies) | `pos_gp2_claim_from_join_left`, `pos_gp2_chain_two_elements`, `pos_gp2_claim_from_own_seal`, `pos_gp2_repeated_consumption`, `pos_gp2_dl_link_element`, `pos_gp2_dl_premise_element` | `claim_consume_segment_as_set`, `claim_wrong_set_root`, `claim_chain_reordered`, `claim_drop_dependency`, `claim_link_after_chain_refused`, `claim_premise_after_element` |
| input link (value of a prior run) | none (see blocker) | `claim_input_link_rejected_producer`, `claim_rejected_as_value`, `claim_skip_value_opening`, `claim_input_link_value_mismatch` |
| GP-4 shape and key route | `pos_gp4_shape_bound_per_mode` (`shape_d` differs for SEG, SEAL, JOIN, SUB and DL outputs) | `graph_shape_word_forged`, `node_key_root_forged` |

Every refusal is a failed verification of the real node proof (constraint or LogUp sum mismatch), not a host check. Not every refusal isolates its named rule. The SEAL-WITH-DEPS cases (`claim_seal_undischarged_deps`, `probe_seal_deps_premise_only` and the four input-link cases) have no accepting twin: every real header carries `deps_d = EMPTY_DEPS`, so each of them may fail on the first SEAL-WITH-DEPS rule (`deps_d` of the header equals the DL chain, `gp.rs` SEAL block) and not on the rule it is named for. They are MEASURED as refused, rule not isolated. The other refusals each have an accepted twin that differs in one input (for example `claim_link_after_chain_refused` against `pos_gp2_dl_link_element`).

**Blocker (GP-2 not FROZEN).** The T6 slice leaf proves only the unbound run, which ends Rejected (outcome tag 2, code 3); the literal and publish-value rules are not in the leaf subset, so no Halted run exists on real proofs. The honest SEAL-WITH-DEPS case (B consumes the value of a Halted A through the input link) therefore cannot be proved, and under AD-20 (a) every real `deps_d` is `EMPTY_DEPS`. The refusals of the input link are MEASURED; the positive case needs a Halted leaf run (leaf-kit work, outside T8).

**Need outside T8 (no code here).** L (T7) takes a SEG root and seals it inside L; L is at 1,986 of 2,000 columns and 93 of 100 interactions (MEASURED in T7). L accepting a SET or DEP root of this block (FP-3, LW-008) is T7 or T10 work.

## 4. X8: the Compact adapter class (GP-3, O8)

O8 pick (LEAD, for owner confirmation): the class name is **`compact-zkir3-kzg-bls12381-blake2b/1`**. Members are Midnight Compact contract circuit proofs in ledger format, as `research/sherlock-gp-architecture-20261110/architecture.md` (section 2.3, "Compact contract proofs") reads them from the ledger source:

- proof tag `proof[v5]`, key tag `verifier-key[v7]`, key `MidnightVK` = (arch, k, public-input count, PLONK key); trailing key bytes refused;
- midnight-proofs PLONK with KZG over BLS12-381, the Midnight SRS, Blake2b transcript, one challenge phase, the stdlib configuration with two instance columns (committed column = identity commitment);
- one public-input vector: binding input, optional communications commitment, then the ledger operation transcript; guarded-off operations stay as zero fields;
- the class is keyed by (arch, k, public-input count): the count is exact per class, never padded to a cap.

The adapter commits the whole public-input vector into `out_d` and interprets nothing. It prepares the foreign proof and publishes the pairing accumulator; the final check is an FV-FINAL obligation. A registry entry `sherlock-reg-entry/1` (tag code `SHK_REG_ENTRY` 66, section 0 item 2) names the class, the (arch, k, count) triple and the adapter `vk_id`.

What exists on the T8 tree: no in-circuit verifier of this class. The node layer reserves the FOREIGN head opening (`T_FOREIGN` 3, tag `FOREIGN` 21 in `crates/sherlock-lfr/src/gp.rs`) and refuses it: the GP-2 block opens LEAF and DEP heads only, so no FOREIGN claim can be made without a verification. Missing pieces for a FROZEN GP-3 row: the Blake2b transcript AIR, the expression-evaluation AIR, and FV-FINAL (pairing) in work units (X14, section 5). GP-3 rows stay FREEZE-PENDING.

## 5. X14: FV work units (O1, O2)

The arithmetic units of X14 are MEASURED in `research/sherlock-x14-20261112/RESULTS.md` (tables at lines 235 and 278 to 295) under the owner rulings now in force: a 17-bit range checker of the unit's own (O1, FV work units only; the 16-bit RANGE table of LD-S10x-3 stays for everything else) and a 3,300-column bound for FV work units only (O2).

| Unit | AIRs | Columns | Interactions | Bound | Label |
|---|---|---|---|---|---|
| `fr_mul` | 41 | 1,735 | 129 | 3,000 | MEASURED (X14 line 46) |
| `fp_mul` | 41 | 1,831 | 193 | 3,000 | MEASURED (X14 line 47) |
| G1 add chain with condition rows, padding, identity flags | 45 | 3,143 | 200 | 3,300 (O2) | MEASURED (X14) |
| G1 double chain without `INV_y` | 45 | 2,949 | 199 | 3,000 | MEASURED (X14) |

D3 at 3,300 columns (internal set, 1,000 constraints, 64 AIRs, 200 interactions): stacked 110.9393, weakest 101.1643, so every term is at least 100 and every grindable term at least 101 (MEASURED by the calculator, `research/sherlock-x14-20261112/out/d3-bounds-cols.log`). T8 did not build or run these units on the openvm `t8-gp` tree: the numbers above are cited from X14, not remeasured. Forged points are refused on real X14 bench proofs (the `g1neg` cases, X14 `RESULTS.md` lines 197 to 206 and 264 to 268: forged lambda with P2 = P1 refused by `INV`, forged (0, 0) doubling by `INV_y`, padding holes and flags on real points by `PointsIn`), not on T8 proofs. The T8 node keeps the T7 FV stand-in output and forces the GP mode bits to 0 in FV rows (section 3). The transcript and expression-evaluation AIRs are not built, so forged transcripts and final checks cannot yet be refused on real proofs. The GP-3 general verifier row stays FREEZE-PENDING on those AIRs.
