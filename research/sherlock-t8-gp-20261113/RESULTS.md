# T8 results: the GP hooks on the node layer

Row T8 of `research/sherlock-tested-design-20261113/LOOP.md`. Trees, all on branch `t8-gp`: Moriarty `~/Projects/Moriarty-wt/t8-gp` (base `fc830df0f`), openvm `~/.cache/m7-wrap/src/openvm-t8` (base `662235f`, the T7 tip), stark-backend `~/.cache/m7-wrap/src/stark-backend-t8` (no change). State, logs and outputs under `~/.cache/t8/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

Status: N-4, X12 and X7 are built and run on real proofs or in Quint. X8 has a named class. X14 is cited from its own run. No GP hook row reads FROZEN; each row names its open gates (item 11). The Moriarty branch is rebased onto `t7-bind` `316b17682`. openvm `t8-gp` ends at `a645de3` on `662235f`.

**First for the owner: the MERGE-WORK column bound is 5,400, not the ruling's "about 4,400" (AFK pick, for confirmation at T15).** 4,400 could not be met once the ruling also required the CORE split. DERIVED: 4,246 (probe of the T7 layout) + 522 (FRAME) = 4,768 is already over 4,400 before the GP block. The key as built is 5,128 columns (MEASURED). D3 at 5,400 has weakest term 101.1643 (MEASURED). CORE sits at 999 of the 1,000-constraint bound.

1. **N-4 reading: option (a). ACCEPTED by the lead (2026-11-13).** No circuit relation carries a checkpoint tag. The tag is on the persisted record digest only. Reason: in `continuation.qnt` (section 1) the circuit side holds with an untagged record (`cont_recordTagRun --invariant circuitSide` `[ok]`), and the sampler catches all 7 mutants.
2. **`SHK_CKPT` code (LEAD pick).** This is the record tag, a host digest. It takes the next free BabyBear code: 67, after `SHK_FV` 65 and `SHK_REG_ENTRY` 66.
3. **N-6: owner ruling of 2026-11-14 ("own bounds").** MERGE-WORK nodes get their own universal parameter set `frozen::MERGE_WORK`. It has the internal parameters, its own column bound, and the per-AIR bounds of the internal set (1,000 constraints, 200 interactions, 64 AIRs, log height 19). The shared internal set keeps 3,000 columns. Built and run in T8 (section 2). The ruling is recorded in ARCHITECTURE N-6 and the GP-4 hook row.
   - **AFK pick, for owner confirmation at T15: the column bound is 5,400, not "about 4,400" (see the note above).** The ruling sized 4,400 from the probe of the T7 node layout (4,246). The key as built is 5,128 columns (MEASURED). The extra columns come from the GP block in the node statement, the MERGE-WORK step columns and FRAME (522 columns). 5,400 leaves about 5 percent margin. D3 at the 5,400 bound: weakest term 101.1643, so every term is at least 100 and every grindable term at least 101 (MEASURED). No hard rule moves.
   - **CORE split (LEAD build).** CORE keeps 999 constraints, and FRAME holds the frame rule (494 constraints, MEASURED). Both use one function, `frame_rule`. The leaf's whole CORE calls the same function, so one encoding of the machine relation stays, with no separate emitter. CORE sends each row to FRAME over the new port `core_frame` (5 chunks of at most 121 words, under the message bound of 128). The one-hot row index travels in the message, so row `j` pairs only with row `j`. The app and leaf keys do not change: their pre-hashes at the T8 tip equal the T7 values (MEASURED, `T7KEY` lines of `out/final/final.log` against `out/chain-recheck-c26e247.log`).
   - **Kernel buses (LEAD).** The node's recursion buses use indices up to 79 and collide with the kit buses at 64 and above (MEASURED by the bus probe). In a MERGE-WORK node, every kit AIR is wrapped in `Rebus`, which moves its buses by 128 while it is built and evaluated. The kit AIR and its constraints stay the same. The kernel buses are then 194 to 198 and 208 to 213. Only `step_pv` (213) joins the node and the kernel (asserted in `work_layer`).
   - **Key row (LEAD).** The MERGE-WORK key takes a row in the second half of the `keys_d` table, beside the FV row. All 4 node rows are in use (`UROWS` = 4). `keys()` of `t7_chain` and `t8_gp` builds the key (shared file `examples/common/work.rs`), so every run, V.9 and V.10 included, keys the same set. The `keys_d` of `x7.json` equals the `keys_d` of chains A and B and of `gp2.json`: `0028aa65...2691` (MEASURED). L and `L_Fr` are rekeyed over this `keys_d`, and `L_Fr` still has 1,522 public words.
   - **Inline segment binding (as specified, SHP-003 and the MERGE-WORK row of NODE INTEGRATION).** The two children merge by the MERGE rules. The inline segment W is linked at the right end: `W.pre_d = post_d(1)`, `W.h_pre = h_hi(1)` and `W.prog_d = prog_d`. The output takes `post_d = W.post_d` and `h_hi = W.h_pre + steps` (16-bit limbs with carries, no wrap). W's CORE public values reach the node statement over `step_pv`. The kernel key is fixed by the MERGE-WORK key, so no plan or menu opening of a chip set is made for the step. The step's log is empty (CORE PICK of the slice). `shape_d` carries `work_kind = 1` (`WK_SLICE0`) and `work_steps`.
   - **Open (next work, not T8):** a parent key that verifies a MERGE-WORK child. The verifier sub-circuit is keyed per child AIR shape, and no node row is free. The STACKS and VALUE DIGEST kits are not in the slice leaf, so the inline step has MEM work but no stack or value-digest work.
4. **X12 readings (LEAD, built and MEASURED in section 3):**
   - SEAL has no admission premise (slot 1 off; ADMIT-WORK is group 5).
   - DL-STEP slot 0 is SET or SUB; slot 1 is DL or off.
   - The link element joins its producer SET into the cover.
   - The head opening is LEAF or DEP only. FOREIGN waits for GP-3.
   - Cover presence is tested by the inverse of the word sum. The honest failure probability is about 2^-31 (DERIVED: one BabyBear element). An ABSENT cover is the zero digest.
   - MERGE no longer requires equal `shape_d` (T7 `merge_check` and the merge rule edited).
   - The N1 output keeps `shape_d = 0`. The LEAF base case with `rules_d` (AD-23 (a)) is not built, so `graph_leaf_shape_free` stays open.
   - FV rows keep the T7 stand-in output with every GP mode bit 0.
5. **GP-2 blocker: no Halted leaf run.** The T6 slice leaf proves only the unbound run, which ends Rejected. An honest SEAL-WITH-DEPS (a run that consumes a prior run's value) cannot be proved until the leaf kit has the literal and publish-value rules. Every forged dependency, link and value case is refused (MEASURED). The SEAL-WITH-DEPS refusals have no accepting twin, so they do not isolate their named rule (section 3).
6. **L needs a SET root (T7 or T10).** L takes a SEG root (1,986 of 2,000 columns and 93 of 100 interactions in T7). To accept a SET or DEP root of the T8 block (FP-3, LW-008), L must change. T8 makes no such change and no edit to `crates/recursion/src/proof_shape/` or the path C seams.
7. **O8 pick (LEAD, for owner confirmation): `compact-zkir3-kzg-bls12381-blake2b/1`** (section 4). The GP-3 hook stays FREEZE-PENDING. No in-circuit verifier of the class exists, the transcript and expression-evaluation AIRs are not built (X14), and N-8 is open.
8. **X14 under O1 and O2:** the unit sizes are MEASURED in the X14 run and cited here, not rerun on the T8 tree (section 5). The GP-3 general verifier stays FREEZE-PENDING on the transcript, expression and FV-FINAL AIRs.
9. **AFK picks on OWNER items (lead order of 2026-11-14), for owner confirmation at T15:**
   - **N-7, recommended option: bind intervals.** The T8 `shape_d` binds `(h_lo, h_hi)` of a SEG output as `out_a`, `out_b` (`gp.rs` `shape_w`). Disclosure stays the open dissent item D-6.
   - **N-8, recommended option: root history with revocation.** T8 builds no part of the `reg_root` lifecycle. Every node mode keeps `reg_root` equal across the enabled slots. FOREIGN is not opened, so the `FOREIGN(reg_root, fclaim_d)` form is not needed yet.
10. **Regressions found and fixed (no gate failed twice).** (a) The first T7 cases rerun on the T8 layout failed `fv_unit_proves` and `fv_unit_under_N3_accepted` (MEASURED, `out/chain-recheck-c26e247.log`). The non-MERGE output rules were gated by one minus the MERGE bit, so they also held in an FV unit. Fix `569b23c` gates them by the mode bit sum. All 12 cases pass at the fix and at the tip (section 6). (b) A first X7 build placed W between the two children. That contradicts SHP-003, and the code now follows the spec (`a645de3`).
11. **OpenSpec (strict-validated).** `sherlock-modular-leaf-20261111`: SHP-007 points the MERGE-WORK key to SHP-008, SHP-008 is new (its own set, the CORE split), and NODE-015 is new (the shared internal set does not change). `sherlock-processor-final-20261111`: SQG-013 is new (the `merge_work` parameter set). Both changes are valid under `openspec validate --strict`.
12. **ARCHITECTURE rows.** T8 edited the N-6 entry and the GP-4 hook row. Proposed text for the others, not edited by T8:
    - CONTINUATION and HOST CHECKPOINT STORE: N-4 is closed by `continuation.qnt` with option (a). The lead's rerun of the Quint set is pending, and the host refusals of HOST CHECKPOINT STORE (outside the model) stay open.
    - GP-2 hook: "FREEZE-PENDING the Halted leaf run (honest SEAL-WITH-DEPS) and L over a SET root; the X12 node modes are built and MEASURED in T8 (34 cases)".
    - GP-3 hook: "FREEZE-PENDING the X8 verifier of `compact-zkir3-kzg-bls12381-blake2b/1`, N-8 (AFK pick: history with revocation)".
    - I-13: FREEZE-PENDING the same gates.
13. **Circuit changes for the Carbon Quint lanes (openvm `a645de3`):**
    - `crates/sherlock-lfr/src/gp.rs`: `:241` `eval` (mode bits; SEAL, SEAL-WITH-DEPS, DESCEND, DL-STEP and JOIN rules; `shape_d` with `work_kind`, `work_steps`); `:427` (`nm` gated by the mode bit sum); `:536` `fill`; `:706`, `:711` `shape`, `shape_w`.
    - `crates/sherlock-lfr/src/stmt.rs`: `:97` `STEP_PV_BUS`; `:350` `merge_check` (no equal-`shape_d` rule); `:616`, `:695` to `:700` (`gpc` and the Work columns); `:947` (allowed key rows); `:977` to `:1034` (GP eval call, the T7 merge rules gated by MERGE, the MERGE-WORK link at `:1001`); `:1036` (the `step_pv` receive and `W.h_pre + steps`); `:1360` `Wit::Work`; `:1486`, `:1532` (Node and Work traces); `:1608` (`sponge_height`).
    - `crates/sherlock-lfr/src/frozen.rs:157` `MERGE_WORK`.
    - `crates/sherlock-wrap-export/examples/common/work.rs` `work_key` (the MERGE-WORK AIR list).
    - `crates/sherlock-leaf/src/core.rs`: `:203` `CoreAir::split`; `:320` (frame send and `step_pv` send); `:597` `frame_rule`; `:650` `FrameAir`; `:698` `frame_trace`.
    - `crates/sherlock-leaf/src/leaf.rs`: `:111` `airs_kernel`; `:129` `Rebus`; `:151` `traces_kernel`; `:369` `shards_cut`.
    - `crates/sherlock-leaf/src/bus.rs`: `:36` (ports `core_frame`, `step_pv`); `:57` to `:70` (bus offset).
    - Suggested lanes: the X12 claim-consumption model `qa/gemini-quint/x12/claimConsumption.qnt` against `gp.rs` eval; a MERGE-WORK stitch model (two SEGs and one step, with interval and stitch mutants) against `stmt.rs:1001`; and the frame split (FRAME and CORE as one relation).

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

## 2. X7 and N-6: the MERGE-WORK node (GP-4)

The owner's ruling of 2026-11-14 is in section 0, item 3. Code: openvm `a645de3`. Run: mode `x7` of `crates/sherlock-wrap-export/examples/t8_gp.rs`. Record: `out/final/x7.json`. Log: `out/final/x7.log`. Both come from the final run at the tip (section 6), on a shared slot, so the wall times are not timings.

**What the node does.** One MERGE-WORK proof covers program A's run (5 steps), cut into three parts:
- slot 0: an N3 SEG proof of steps [0, 2) (leaf, N1, N2, N3);
- slot 1: an N3 SEG proof of steps [2, 4);
- inline segment W: the owner step [4, 5), proved inside the node by the slice kernel (CORE, FRAME, DEC, RANGE, SPONGE, HASH2, ROM, MEM, built by the leaf kits' `airs` and `traces` functions; `sl::airs_kernel`, `sl::traces_kernel`).

The node's statement AIR (`Kind::Work`) merges the two children by the MERGE rules and links W at their right end (SHP-003). Its output is the SEG of the whole run: `h_lo` 0, `h_hi` 5, `pre_d` of slot 0 and `post_d` of W (MEASURED, `output` of `x7.json`). The own key commitment in the output equals the MERGE-WORK row of `keys_d` (MEASURED: `vkc_out_equals_work_row`). That row is in the `keys_d` that V.9 and V.10 ran on (section 0, item 3).

**The MERGE-WORK key** (`frozen::MERGE_WORK`, MEASURED by keygen with the G1 check):

| | Value | Bound | Label |
|---|---|---|---|
| AIRs | 49 (41 node, 8 kernel) | 64 | MEASURED |
| columns | 5,128 | 5,400 (AFK pick; ruling "about 4,400") | MEASURED |
| max constraints per AIR | 999 (CORE) | 1,000 | MEASURED |
| max interactions per AIR | 171 (statement AIR) | 200 | MEASURED |
| max log height of the proof | 17 | 19 (k at most 21) | MEASURED |
| verifying key, bitcode | 183,452 bytes | none (not a wrap key) | MEASURED |

Per AIR (columns / constraints / interactions, MEASURED):
- statement AIR (Work): 658 / 843 / 171
- CORE (split): 676 / 999 / 151
- FRAME: 522 / 494 / 5
- DEC: 9 / 18 / 1
- RANGE: 4 / 10 / 1
- SPONGE: 321 / 326 / 2
- HASH2: 29 / 5 / 5
- ROM: 1,222 / 78 / 77
- MEM: 100 / 100 / 21

**D3** (`fp8_d3` under `merge_work`; `out/final/d3work/d3.json`, MEASURED):
- At the key (`vk` row): weakest `shift` 101.1643, stacked 110.3034. There is no `measured` row (proof heights and stacked widths) for this key; `fp8_d3` computes it only for the chain keys.
- At the bounds (5,400 columns, 64 AIRs, 1,000 constraints, 200 interactions): weakest `shift` 101.1643, stacked 110.2288, `mu` 101.6304, `logup` 102.7205.

Every term is at least 100 and every grindable term at least 101.

**Cases** (13 of 13 pass, MEASURED). Every refusal is a failed verification of the real MERGE-WORK proof. The failing AIR comes from a `T7_DEBUG` run of the same cases (`out/x7-debug-a645de3.log`).

| Case | Expect | What changes |
|---|---|---|
| `pos_x7_merge_work` | accept | the honest node |
| `x7_step_wrong_position` | refuse | inline segment [0, 2) in place of [4, 5): `W.pre_d` is not `post_d(1)` (statement AIR) |
| `x7_step_other_program` | refuse | the inline step is program B's |
| `x7_step_pv_forged_in_statement` | refuse | the statement's copy of the step's `post_d` (`step_pv` bus) |
| `x7_step_post_forged_in_kernel` | refuse | CORE's public `post_d` and the statement's copy. Rule not isolated: CORE BOUNDARY fails (CORE, row 4), and so does the output `post_d` |
| `x7_step_height_forged` | refuse | W's `h_pre` in CORE and the statement. Rule not isolated: the link `W.h_pre = h_hi(1)` fails (statement AIR), and so do the inputs of the CORE INIT gate |
| `x7_frame_row_forged` | refuse | one state word of FRAME row 2 only (FRAME's frame rule and the `core_frame` bus) |
| `x7_frame_rule_in_frame` | refuse | state word 1 on rows 2 and 3, changed in CORE and FRAME alike (bus balanced). Slots 1 to 3 are disabled and rows 2 and 3 are not hashed, so only `frame_rule` in FRAME fails (MEASURED: FRAME, row 1). The split keeps the frame rule |
| `x7_one_prior_proof` | refuse | slot 1 off |
| `x7_children_swapped` | refuse | suffix in slot 0, prefix in slot 1 |
| `x7_output_post_forged` | refuse | output `post_d` |
| `x7_output_shape_forged` | refuse | output `shape_d` (`work_kind`, `work_steps`) |
| `probe_x7_key_row_checked_one_layer_up` | accept | the FV row as the output key commitment. Accepted at this layer by design: the T7 key route checks a child's key commitment one layer up, and no parent over a MERGE-WORK child exists yet (section 0, item 3) |

**What stays open.** No parent verifies a MERGE-WORK proof yet, so a forged own-key row is not refused on a real proof. The inline step has MEM work. It has no stack or value-digest work, because those kits are not in the slice leaf (ARCHITECTURE N-6 names them). The Gemini Carbon lane is the lead's. GP-4 hook: FREEZE-PENDING on these.

A first build linked W between the two children, against SHP-003. It was replaced (section 0, item 10). The earlier probe of the T7 node layout plus the unsplit kernel (48 AIRs, 4,246 columns, `CoreAir` 1,476 constraints; `out/x7probe-t7layout.json`) is superseded. It is the number the owner's ruling was sized from.

## 3. X12: GP-2 node modes on real proofs (consuming a prior Sherlock run)

Code: openvm `t8-gp`. The node block is `crates/sherlock-lfr/src/gp.rs` (the GP-2 and GP-4 modes), with hooks in `crates/sherlock-lfr/src/stmt.rs`. The run is mode `gp2` of `crates/sherlock-wrap-export/examples/t8_gp.rs`. Record: `out/final/gp2.json`. Log: `out/final/gp2.log`. Both are from the final run at the tip `a645de3` (section 6), on a shared slot, so the 202 s wall time is not a timing. The first run, at `c26e247`, also passed 34 of 34. It predates the FV fix of section 0, item 10.

The run proves the slice leaf on programs A (`entry x;`) and B (`entry y;`), carries each through leaf, N1, N2 and N3 to a SEG root, then proves every GP-2 mode as an N3 node over N3 children on the one universal key set (N3 over N3 is the fixed point: `n3_over_n3_pre_equals_n3_pre=true`, MEASURED).

Node modes in the block (`gp.rs:1-35` documents the preimages): MERGE [0] (T7 rules, gated by the MERGE bit), SEAL [2] `SEG -> SET` (`h_lo = 0`, `post_d = P2(SHK_SD2, header)`, terminal outcome, `claim_d = P2(SHK_RUN, 1, prog_d, input_d, outcome_d)`, `set_root = P2(SHK_SET_LEAF, claim_d, h_hi)`), SEAL-WITH-DEPS [3], DESCEND [5] `SET -> SUB`, DL-STEP [4] (premise, input-link and chain elements), JOIN [6] `SET + SET -> SET`. Every output carries `shape_d = P2(SHK_SHAPE, T_NODE, mode, slot bits, key selectors, child shapes, ...)`.

Key shapes at the tip (MEASURED, `keys` of `gp2.json`, all accepted by the G1 check): N2 and N3 41 AIRs, 2,182 columns, 809 constraints, 170 interactions, degree 4 (bounds 64 / 3,000 / 1,000 / 200); N1 41 / 2,435 / 620 / 144; FV unit (N2 shape) 41 / 2,182 / 756 / 170.

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

## 6. Final run at the tip and the T7 gates

The unit `t8-final2` runs `~/.cache/t8/final.sh` at openvm `a645de3` (clean tree), on shared slots. Log: `out/final/final.log`. Records: `out/final/`. Every key below is built in one `keys()` with the MERGE-WORK row in `keys_d` (`0028aa65...2691` in every record, MEASURED).

| Check | Result | Label |
|---|---|---|
| `sherlock-lfr` lib tests, `t9_enc45`, `sherlock-leaf` tests | 17, 3 and 3 passed | MEASURED |
| `t8_gp x7` (section 2) | 13 of 13 cases pass | MEASURED |
| `t8_gp gp2` (section 3) | 34 of 34 cases pass | MEASURED |
| V.10 `check-chain.sh chain` (chains A and B, `t7_chain`, `fp8_d3`) | PASS for A and B: 2 shards, 5 steps, fixed heights, G1 accepted, D3 PASS on 8 keys each, N3 fixed point, 7 cases each. L and `L_Fr` are rekeyed over the new `keys_d`. `L_Fr` public words are still 1,522, so the T10a wrap fit holds (DERIVED). The wrap and its key are not rerun | MEASURED |
| `check-chain.sh cases` (T7 binding cases) | 12 of 12 pass, FV unit cases included | MEASURED |
| V.9 `check-chain.sh omit` | PASS, 59 cases, every AIR required in app, leaf, N1, N2, N3, FV, L. The panics in the log are the expected omit refusals | MEASURED |
| D3 on every key (`d3.json` of chains A and B; `d3work/d3.json` with the MERGE-WORK key) | every gate PASS. Weakest terms: app 101.0891, leaf 101.0110, N1 to N3 and FV 101.1643, `L` 101.0730 (`mu`, bounds), `L_Fr` 101.1435, MERGE-WORK 101.1643 (vk and bounds). Every term is at least 100 and every grindable term at least 101 | MEASURED |

These are `check-chain.sh` runs, not doctorwatson receipts. The T8 tree has no `qa/sherlock` receipt for V.9 or V.10. The lead runs `doctorwatson check --closure V.10` on `t8-gp` if a receipt is needed.

Commits:
- openvm `t8-gp`: `c26e247` (GP block), `569b23c` (FV fix), `7298247` (X7 MERGE-WORK), `a645de3` (W at the right end, MERGE-WORK key in `keys()`, frame-rule case).
- Patch series: `research/sherlock-stark-code-20261112/openvm-t8/` (`git format-patch --binary 662235f..t8-gp`).
- stark-backend: no change.

Reproduce: `~/.cache/t8/final.sh`.
