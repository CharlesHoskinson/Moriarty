# T8b results: closing the GP hooks that T8 left open

Row T8b of `research/sherlock-tested-design-20261113/LOOP.md` (the lead's brief of 2026-11-14). Trees, all on branch `t8b-gp`: Moriarty `~/Projects/Moriarty-wt/t8b-gp` (base `07af1c450`), openvm `~/.cache/m7-wrap/src/openvm-t8b` (base `a645de3`), stark-backend `~/.cache/m7-wrap/src/stark-backend-t8b` (base `deeeb0a`, no change). State, logs and outputs under `~/.cache/t8b/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

**Commits.** openvm `t8b-gp`: `fe2a341`, `5f15e07`, `4b2771b`, `2325f63`, `89bee35` (series `research/sherlock-stark-code-20261112/openvm-t8b/`). stark-backend: none. Moriarty `t8b-gp`: `da9cc4212`, `6e76d79d6`, `08df7b956`, `c13d880e3` and the final commit of this report.

**Done-when checks (MEASURED, real proofs, stock verifier):**

| Check | Result | Section |
|---|---|---|
| GP-2: a run consumes a prior run's Halted value; forged dependency refused | PASS: chip set 2 proves the Halted run and its consumer; SEAL-WITH-DEPS of the consumer over the input-link DL gives `DEP(claim_C, n, LEAF(P), claim_P)`; 9 twins refused; gp2 48 of 48 | 1, 2 |
| GP-3: a Compact proof of a named class verified in-circuit; forged proof refused | NOT DONE: no in-circuit Compact verifier exists and no Compact proof of the class is on the host; X14 units rerun and reproduce | 5 |
| GP-4: a parent verifies a MERGE-WORK child; forged continuations, points, final checks refused | PASS for the parent: NW, NW2, N3 over MERGE-WORK accepted, forged parent output, wrong key rows and a forged step-word column refused; x7 22 of 22 | 3 |
| GP-4: the inline step includes STACKS and VALUE DIGEST work | NOT DONE: no STACKS AIR exists; chip set 2 CORE is 1,298 constraints, over the kernel's 1,000 | 4 |
| V.9, V.10, D3 | PASS: cases 12, V.9 59 cases, V.10 A and B, D3 every term at least 100 and every grindable term at least 101 on every key including MERGE-WORK, NW and NW2 (weakest 101.0110, leaf shift) | 7 |
| I-13 and the four GP hook rows FROZEN | NOT FROZEN: all five stay FREEZE-PENDING with T8b evidence and the reason (ARCHITECTURE rows I-13, GP-2 hook, GP-3 hook, GP-4 hook, GP-3 general verifier) | 4, 5, 6 |

**Blocked or not done, with the reason.**
- `L` over a SET root (GP-2 to the ledger): not started; the brief says not to edit `L` before the lead reports that T10 has landed, and no such message came (section 6).
- GP-3 in-circuit verifier (X8): not built; it is a stream of FV work units (transcript, MSM, final check) that are not designed, and no Compact proof of the class is on the host (section 5).
- STACKS and VALUE DIGEST in the inline kernel: not built (section 4).

**AFK picks for owner confirmation at T15** (basis: the lead's brief of 2026-11-14 says the owner is AFK and to take the recommended option; LOOP.md in the main checkout has no T8b row):
1. Chip set 2 (the value set) is a per-program leaf key under `key_root` (set id 2), so N1 is one key over chip sets 0, 1 and 2 (CORE-015).
2. The `vd` of a constant comes from its constants ROM row, with `ty_d = P2(DOM_TY, row)` and the `decl_d` form of `leaf.rs` `const_vd`; the INIT input `val_d` is a witness for chip set 2 only.
3. `keys_d` has 7 rows: NW over MERGE-WORK, NW2 over NW and N3 over NW2 (SHP-009). This changes `keys_d`, so `L` and `L_Fr` rekey and T10's final keys go stale when T8b merges into `tested`.
4. The kernel CORE step words are main columns sent from every row, received 8 times by the Work statement (in place of 54 CORE public values that no parent can verify).

**Carbon Quint lanes:** seven circuit changes, Q1 to Q7 with file:line in section 8.

## 1. GP-2: the Halted leaf run (chip set 2, the value set)

**Done, MEASURED** (`out/value_set.json`, example `crates/sherlock-leaf/examples/value_set.rs`, openvm `5f15e07`).

- **The run.** The slice generator now exports the ROM rows of the Halted candidate `unitctor` (`type U { A(); } entry construct U::A();`): expression, constructor, constant and declaration tables (Moriarty `da9cc4212`, `out/slice/slice_v1.json`). The `unbound` export is byte-equal to T6's, so chip sets 0 and 1 see the same input.
- **The rules.** Chip set 2 has two rules: `XAdtEvalConstructNullary` (T5 3.4 row 119) and `KAdminPublishValue` (row 2). The first reads `expressions[e]`, `constructors[ctor]` and `constants[cid]` and returns `Const(cid)`; the second publishes the constant's `vd` as the Halted payload. The `vd` is the VALUE DIGEST constant formula over the constant's ROM row (`leaf.rs` `const_vd`), so the payload is bound to the program by the PROG premise.
- **The input.** The INIT opening of `input_d` takes a witnessed input value `val_d` for chip set 2 only (AD-17). Chip sets 0 and 1 keep the constant `VAL_NONE`.
- **The key.** 7 AIRs, 2,423 columns; CORE 738 columns, 1,298 constraints, 147 interactions, degree 3. G1 accepts it at the app set. It is a per-program leaf key under `key_root` (set id 2).
- **Proofs.** The producer (`U`) and the consumer (type `V`, input `val_d` = the producer's Halted `vd`) prove and verify with the stock verifier (8.46 s and 7.40 s on a shared slot, not timings).
- **Refusals.** Four one-cell forgeries are refused by the debug checker and by the stock verifier on a mutant-prover proof: the payload, the outcome tag (Halted relabelled Rejected), the constant row `vd`, and the INIT input value (LogUp mismatch on `input_d`).

## 2. GP-2: the honest SEAL-WITH-DEPS and its twins

**Done, MEASURED** (`out/gp2.json`, `t8_gp gp2`, openvm `2325f63`, the final key set of section 3; 48 of 48 cases pass, every proof checked by the stock verifier).

- **The honest chain.** The Halted producer P (chip set 2) is sealed (`pos_gp2_seal_halted_producer`, outcome tag 1, payload `vd`). A DL-STEP input-link element takes `in_link = claim P` under the cover `LEAF(P)` (`pos_gp2_dl_link_halted_producer`). The consumer C, whose input value is P's Halted value, is sealed with SEAL-WITH-DEPS over that DL (`pos_gp2_b_consumes_a_halted`); its output is `DEP(claim C, n, LEAF(P), claim P)` (asserted in the example). DESCEND from `DEP(C)` to its cover and a DL-STEP chain element that opens claim P under `DEP(C)` are accepted (`pos_gp2_descend_dep_cover`, `pos_gp2_claim_from_dep_cover`). This is the first SEAL-WITH-DEPS of a real consumer of a real Halted value.
- **Twins.** T8 refused SEAL-WITH-DEPS forgeries without an accepted honest case. Each refusal now has a twin that differs from the honest case in one input, and each twin is refused by the stock verifier: another `val_d` (`twin_claim_input_link_value_mismatch`), the value opening skipped, the producer relabelled Rejected, the Rejected producer A linked, a chain DL in place of the link DL (undischarged deps), a premise-only DL, another producer `prog_d`, a forged output root, and the producer as its own consumer.
- **What is still open on GP-2.** `L` does not yet accept a SET root (section 6), so a SET or DEP output cannot reach the ledger. X12 (the GP-2 discharge model in the circuit) and the Carbon lanes Q1 to Q7 are also still open. The hook row stays FREEZE-PENDING on these items.

## 3. GP-4: a parent over a MERGE-WORK child

**Done, MEASURED** (`out/x7.json`, `t8_gp x7`, openvm `89bee35`; 22 of 22 cases pass, every proof checked by the stock verifier).

- **The ladder.** A node key over the MERGE-WORK key is not N3: node keys built one over the other from MERGE-WORK differ from N3 at rungs 1 and 2 and equal N3 at rung 3 (`out/ladder.json`, `t8_gp ladder`). So the parent route is NW over MERGE-WORK, NW2 over NW, and N3 over NW2, and `keys_d` has three new node rows: `UROWS` 4 to 7 (`crates/sherlock-lfr/src/stmt.rs:89`). NW and NW2 are the same universal node circuit keyed over another child; no new circuit. AFK pick for owner confirmation at T15 (SHP-009).
- **The fix the parent needed.** The first NW proof failed (`LogupZerocheck(NonZeroRootSum)`, `logs/x7-r2.log`): the split kernel CORE of T8 had 54 public values (MERGE-WORK key public values 144), and a parent's verifier sends every child public value while the statement AIR receives only its own 90. The split CORE now holds the 54 step words as main columns on every row, sends them on `step_pv` from each row, and the Work statement receives them 8 times (the CORE height), so every row holds the received words (`core.rs` `C_PVK`, `leaf.rs` `kernel_core_split`, `stmt.rs` `STEP_PV_ROWS`). The constraints are unchanged; CORE stays at 999 constraints. A step word changed on CORE row 3 only is refused (`x7_step_column_forged_one_row`; a CORE constraint on that row fails too, so the refusal is not isolated to `step_pv`).
- **Keys** (MEASURED, G1 accepted): MERGE-WORK 49 AIRs, 5,284 of 5,400 columns, 999 constraints, 183 interactions, 90 public values; NW 2,285 columns, NW2 and N3 2,284 columns (41 AIRs, 818 constraints, 182 interactions), inside the internal set. App, leaf and N1 keys of chip sets 0 and 1 have the same pre-hashes as T8 (`x7r1`); N2, FV, N3 and MERGE-WORK change because `keys_d` grew.
- **Positive.** `pos_x7_parent_over_merge_work` (NW over the honest MERGE-WORK proof), `pos_x7_nw2_over_parent`, `pos_x7_n3_over_parent`: the MERGE-WORK SEG reaches N3, the input of `L`.
- **Refusals.** A forged NW output `post_d`; NW, NW2 and N3 each with a wrong own key row, refused by the layer above; the MERGE-WORK proof with the FV row as its key commitment, under NW (`x7_key_row_forged_refused_by_parent`). The T8 probe `probe_x7_key_row_checked_one_layer_up` (accepted at its own layer) is now refused one layer up. The 12 T8 MERGE-WORK cases still pass on the new key.
- **Merge note.** `keys_d` changes, so `L` and `L_Fr` rekey. T10's final keys go stale when T8b merges into `tested`.

## 4. GP-4: STACKS and VALUE DIGEST in the inline kernel

**Not done. The GP-4 hook row stays FREEZE-PENDING on this item.** What is missing, with sizes:

- **STACKS is not built anywhere.** No STACKS AIR exists in `crates/sherlock-leaf/src` (openvm `t8b-gp`). The state body carries the stack words (`frm.count`, `frm.below`, `tsk.count`, `tsk.below` and the cached tops, `enc.rs:187-193`), and INIT sets them to the empty constants (`core.rs:194-195`, `:699-700`, openvm `89bee35`). No slice rule pushes, pops or rewrites a stack: the five rules of chip set 0 and the two of chip set 2 write no stack word (frame rule, `core.rs` `frame_rule`). The kit needs the hash-chain AIR of ARCHITECTURE kit STACKS (one whole-sponge HASH slot per step per stack, frame record 52 words and task record 19 words at `ridx = u64`, T5-SCHEMAS section 3), its Quint model `hashStacks.qnt` (not written) and the stack rules of CORE.
- **No slice run reaches a hashing stack step within S = 4.** In the reference runs of `out/slice_v1.json` (MEASURED), `literal`, `boolconst` and `nil` push one task at count 0 and pop it at count 1 (`task_push_pop_rewrite` `[1,0,0]` then `[0,1,0]`): both are the no-hash paths of the kit. `underflow` and `overflow` push one frame and tasks, and the deepest stack of every candidate is 1 (frames) and 1 (tasks) (MEASURED, net push minus pop over `frame_push_pop_rewrite` and `task_push_pop_rewrite`). A spill or a restore needs a stack two deep, which none of the seven candidate programs reaches.
- **VALUE DIGEST in the kernel.** The value rules exist only in the leaf (chip set 2, section 1). Chip set 2's CORE has 1,298 constraints (MEASURED, `out/value_set.json`), over the 1,000-constraint per-AIR bound of the kernel set `merge_work`; the kernel would need a second split of CORE (as T8 split FRAME off). A MERGE-WORK node needs two enabled SEG children and at least one inline step, so 3 steps at least; the Halted value run `unitctor` has 2 steps (MEASURED), and the kernel's `work_kind` would need a second value (`WK_SLICE0` is chip set 0, `gp.rs`).
- **Kernel budget (MEASURED):** split CORE 999 of 1,000 constraints; MERGE-WORK 5,284 of 5,400 columns after T8b (5,128 in T8; T8b adds the key table rows of `UROWS` 7 and the 54 step columns, section 3), so 116 columns are left for a second CORE split and a value rule set.

## 5. GP-3: X14 units and the X8 verifier

**X14 units rerun on the T8b tree: done, MEASURED. X8 (a Compact proof verified in-circuit): not done. GP-3 hook and GP-3 general verifier stay FREEZE-PENDING.**

- **X14 rerun** (`crates/sherlock-wrap-export/examples/x14_units.rs`, a copy of `research/sherlock-x14-20261112/src/x_exp_node.rs`; openvm `2325f63`; `out/x14/`): `fr_mul` 194 columns, 66 constraints, 129 interactions; `fp_mul` 290 columns, 98 constraints, 193 interactions (the X14 values). The add unit node (with the harness sink) is 46 AIRs, 3,242 columns, 200 interactions at most; the double unit without `INV_y` 46 AIRs, 3,048 columns, 199 interactions; both prove and verify at 2^16 rows inside an arity-1 node. Negative cases: `add:same_pt` and `add:dup_id` refused, `dbl:y_zero` and `dbl:dup_id` refused with `INV_y`; without `INV_y` `dbl:y_zero` verifies, as X14 found (the point producer must check the curve). The units behave on the T8b tree as on the X14 tree.
- **What X8 needs and does not have** (READ, file:line in openvm `t8b-gp`):
  - No verifier of a Compact (midnight-proofs, KZG over BLS12-381) proof exists in any STARK crate: no Blake2b transcript AIR, no AIR that evaluates the Compact circuit's gate expressions at the challenge point, and no FV-FINAL pairing check. `sherlock-lfr/src/stmt.rs` `Kind::Fv` (the FV unit) carries only `fv_state` (`stmt.rs:1523-1536`); the FV unit in `t8_gp` and `t7_chain` takes a witness `fv_state` and verifies nothing foreign.
  - The X14 units are the multiplication and G1 chain kits only. The MSM of the verifier (about 23,000 G1 additions, ESTIMATE in X14) and the final check (about 20,000 `fp_mul` rows, ESTIMATE in X14) are not composed into a verifier.
  - No Compact proof of a named class is on the host. A verifier key of the class exists (`~/.cache/e6-int/build/keys/acceptRun.verifier`, 1,353 bytes, `midnight:verifier-key[v7]:` framing), but no proof file of it; making one needs a Compact proving run that this row does not schedule.
- **Size of the missing work (ESTIMATE, from X14):** the foreign `ivc_node` circuit is k 20, 652,433 rows, proof 6,320 bytes; a full in-circuit verifier is a stream of FV work units (MSM, final check, transcript) under the 3,300-column FV bound (owner ruling O2); none of these units is designed yet.
- **I-13** stays FREEZE-PENDING: the FV payload cannot freeze before a foreign proof is verified.

## 6. L over a SET root

**Not started, by the lead's order.** The brief says not to edit `L` until the lead says T10 has landed, and no such message came during this run. `L` is at 1,995 of 2,000 columns and StmtAir at 99 of 100 interactions on the T10 merge tree (MEASURED by T10, cited). The recommended route (a node mode below `L` that accepts the SET root, so `L` does not change) is not built. GP-2 hook row: FREEZE-PENDING on it.

## 7. Final run, V.9 and V.10

**PASS, MEASURED** (records in `out/gates/`; openvm tree clean at each run).

- **Cases** (`scripts/qa/check-chain.sh cases`, openvm `2325f63`): T7 cases PASS, 12 cases (`out/gates/cases.json`, `check-cases.log`).
- **V.9** (`omit`, openvm `2325f63`): PASS, 59 cases, every AIR required in app, leaf, N1, N2, N3, FV and `L`; app shape 7 AIRs, 2,361 columns, max interactions 145 (`out/gates/omit.json`).
- **V.10** (`chain`, openvm `89bee35`, the stock gate script at 64G): A and B each PASS: 2 shards, 5 steps, fixed heights, G1 accepted, D3 PASS on 8 keys, 7 cases, N3 fixed point, `L_Fr` 1,522 words; two programs under one `key_root` and one `keys_d` (`out/gates/chain-A.json`, `chain-B.json`, `d3-A.json`, `d3-B.json`). `L` 243,126-byte key, `L` public values 12,190.
- **D3** (stock `SoundnessCalculator`, `fp8_d3`): every term at least 100 and every grindable term at least 101 on every key. Weakest per key: app shift 101.0891, leaf shift 101.0110, N1, N2, N3 shift 101.1643, `L` mu 101.0730 (vk and bounds), `L_Fr` query 101.1435, FV shift 101.1643. The MERGE-WORK key under `merge_work` (5,284 columns) and NW and NW2 under the internal set: weakest shift 101.1643 at vk and bounds; for MERGE-WORK query 101.1643, mu 101.6304, LogUp 102.7205, fold 102.6276 (`out/gates/d3-work-nw.json`; no measured-shape row for these three, the stacked widths of their proofs are not recorded).
- **The gate runs.** `cases` and `omit` ran through a copy of `scripts/qa/check-chain.sh` that differs only in `--mem` (8G, `~/.cache/t8b/check-chain-mem.sh`); their code (`t7_chain`) is the same at `2325f63` and `89bee35` (the later commit changes `t8_gp` only). A first `chain` run at 34G was stopped after 98 minutes: `heavy.slice` was over its `MemoryHigh` of 64G (heavy-slot admits 90G) and both it and T10's `t10-pathc` sat in reclaim. The rerun used the stock script at 64G. The lead may want to align `heavy.slice` `MemoryHigh` with the heavy-slot budget.
- **Not a timing.** No `--exclusive` run; prove seconds in the records are on a shared host and are not labelled MEASURED timings.

## 8. Carbon Quint lanes (circuit changes, for the lead)

Each change below changes a constraint system and needs a Gemini Carbon Quint lane run by the lead. Paths are in openvm `t8b-gp` (`~/.cache/m7-wrap/src/openvm-t8b`, commit `89bee35`; the circuit code is the same as at `2325f63`).

| Lane | Change | File:line |
|---|---|---|
| Q1 | Value set rule table and DEC padding | `crates/sherlock-leaf/src/dec.rs:42-77`, `:122-145` |
| Q2 | Value set dispatch, guards and effects of the two rules | `crates/sherlock-leaf/src/core.rs:246`, `:325-327`, `:380-474` |
| Q3 | INIT witnessed input `val_d` (chip set 2 only) | `crates/sherlock-leaf/src/core.rs:661-662` |
| Q4 | Frame rule with the value set flag | `crates/sherlock-leaf/src/core.rs:735` |
| Q5 | Constant VALUE DIGEST and table typing, value set AIRs | `crates/sherlock-leaf/src/leaf.rs:263`, `:539`, `:575`, `:620-627`; `crates/sherlock-leaf/src/rom.rs:128` |
| Q6 | `keys_d` with 7 rows (NW, NW2, N3 over NW2) | `crates/sherlock-lfr/src/stmt.rs:89`, `:256-258`, `:948`; `crates/sherlock-wrap-export/examples/t8_gp.rs:331` |
| Q7 | Kernel CORE step words as columns, sent from every row; Work statement receives them 8 times | `crates/sherlock-leaf/src/core.rs:123-124`, `:249`, `:256-262`, `:277-281`, `:376`; `crates/sherlock-leaf/src/leaf.rs:163`; `crates/sherlock-lfr/src/stmt.rs:100`, `:1044-1048` |
