# Counterexamples for the Quint findings

Each finding of the four lanes has one row. The row gives the attempt made on the real code, its
input, the result, the constraint involved and a verdict:

- **CONFIRMED**: the code as built accepts the forgery, or refuses the honest witness.
- **NOT REPRODUCED**: the code as built no longer shows the defect (fixed since the lane read it).
- **GUARD HOLDS**: the attack is refused, and the refusing constraint is the one the finding names.
- **OPEN DESIGN**: the finding concerns a design or prose point. There is no code to attack.
- **NOT ATTEMPTED**: no counterexample was built in this run (the reason is in the row).
- **FIXED**: a CONFIRMED defect that T10q fixed. The row names the commit, the named test that failed
  before the fix (`logs/t10q-fails-before.txt`) and passes after it, and the refusing constraint.

Results are MEASURED on openvm `1733bb5` (branch `quint-reg` on T10mm `d26453c`) with the local
checker of the replay driver (README), unless the row says otherwise. Inputs are in `inputs/`, or the
trace itself is the input (merge-work-ladder rows: the handler reads the cell values from the
witness).

## Defects found

| class | ids | state |
|---|---|---|
| forgery accepted | MW-F01-WORK-A, MW-F01-WORK-B (and the B rerun trace); AFK-F7 | FIXED in T10q (openvm `beba730` (branch `t10q`)) |
| honest witness refused | AFK-F6, AFK-F9(b) | FIXED in T10q (openvm `beba730` (branch `t10q`)) |
| host/prose mismatch | GP2-F4.2 | FIXED in T10q (openvm `beba730` (branch `t10q`)) |
| conditional (standalone shard only) | GP2-C2.1 | open (outside T10q) |

No attack that a lane called sound was accepted among those attempted (see NOT ATTEMPTED rows). Every guard row is refused by the constraint the
finding names.

## merge-work-ladder

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F-01 Work, test A (false carry) | MERGE-WORK at h_pre 4, step 1, `whi [2013200390, 1, 0, 0]`, `wcry [1, 0, 0, 0]`; then NW, NW2, N3 over it | `merge-work-ladder-F01-work-false-carry.itf.json` | before T10q: accepted (Work and 3 rungs). After: refused | bus 197 (kernel RANGE, 16 bits) unbalanced on `[2013200390, 16]`: Work looks up its four `whi` limbs in the inline kernel's RANGE table | FIXED (openvm `beba730` (branch `t10q`); `mw_f01_work_whi_false_carry_refused`) |
| F-01 Work, test B (2^64 wrap bypass) | MERGE-WORK at h_pre 2^64 - 1, `whi [65536, 65535, 65535, 65535]`, `wcry 0` | `merge-work-ladder-F01-work-wrap-bypass.itf.json`, `...-wrap-bypass-run.itf.json` | before T10q: accepted (Work and 3 rungs). After: refused | bus 197 unbalanced on `[65536, 16]` (as above) | FIXED (openvm `beba730` (branch `t10q`); `mw_f01_work_whi_wrap_bypass_refused`, `..._run_refused`) |
| F-01 First | N1 over one leaf step at h_pre 65535: `hhi [65536, 0, 0, 0]`, `cry 0` | `inputs/MW-F01-FIRST.json` | refused | bus 963 (HHI_RANGE_BUS, T10) unbalanced on `[65536]` | NOT REPRODUCED (fixed in T10) |
| F-02 segment drop, swap, splice | the `w-left`, `no-pre-link`, `no-h-link`, `no-prog-link`, `h-wrap` and `merge-no-contig` attacks | the six traces | refused | on T10q: `[child_words+17, wk+16, valid+0', child_words+17']` (W.pre_d = post_d of the right-most enabled child); `[child_words+29, wk+40, valid+0', child_words+29']`; `[child_words+1, wk+8]`; `[wcry+3]`; `[child_words+17, valid+0', child_words+9']` (contiguity; Work has no GP block since T10q) | GUARD HOLDS |
| F-03 W's log omitted from `log_merge` | not attacked. The lane reads that CORE fixes `(LOG_D, LOG_N) = (D_EMPTY, 0)`; not checked here | none | none | CoreAir log columns | NOT ATTEMPTED (lane says sound) |
| F-04 step words over STEP_PV_BUS | `stepv-once`, `stepv-wf-free`, `core-send-row0-only` | the three traces | refused | bus 197 (truncated CORE; a proof is refused first at the key's fixed CORE height of 8); #818 `[wf]`; bus 213 (STEP_PV_BUS) | GUARD HOLDS |
| F-05 key rows and half split | `row-unbound`, `half-split-off` | the two traces | refused at rung 0 | #61 `[valid, child_words+66, pv[66]]` (keys_d); #29 `[sel+7]` | GUARD HOLDS |
| F-06 parent key check, unique ladder | `key-check-none` (a fake SEG under a forged key) | the trace | refused at rung 0 | #6 `[valid, child_words+74, cc]` | GUARD HOLDS |
| F-07 `keys_d` universal | not attacked (a consistency statement of the lane, not checked here) | none | none | none | NOT ATTEMPTED (lane says sound) |
| F-08 stale `UKeys` comment, F-09 Work in the second table half (prose) | not attacked | none | none | none | OPEN DESIGN (doc edits) |

## afk-picks

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F-1 `latest_root_only` breaks long runs | none: the ledger contract is prose | none | none | none | OPEN DESIGN |
| F-2 revocation of historical roots | none: the ledger contract is prose | none | none | none | OPEN DESIGN |
| F-3 mixed-root JOIN | Node JOIN of two SET children under `reg_root` R(10) and R(0) | `inputs/AFK-F3.json` | refused (the same JOIN with equal roots is accepted: AFK-HONEST) | #153 `[child_words+41, valid', child_words+41']` (gp.rs:313-315) | GUARD HOLDS |
| F-4 shape without intervals | the code as built binds `(h_lo, h_hi)` of a SEG output in `shape_d` (gp.rs:714-716), so the rejected option is not in the code; the residual collision is F-7 | none | none | none | GUARD HOLDS (by code reading; see F-7) |
| F-5 MERGE-WORK column bound | the widths of the Work statement and kernel AIRs, and of CORE in chip sets 0 and 2; T10q drops the GP block and `fv_state` from `Kind::Work` (MERGE-WORK has no other mode) | `logs/merge-work-columns.txt` | before T10q: Work StmtAir 760, Work statement and inline kernel 3,735 (MEASURED); MERGE-WORK key 5,284 (MEASURED in T8b); 5,284 + 62 (value-set CORE 738 vs 676, MEASURED) + 71 (STACKS words, ESTIMATE, ARCHITECTURE 1280-1282) = 5,417 > 5,400 (DERIVED). After: Work StmtAir 464, statement and kernel 3,439 (MEASURED, -296); MERGE-WORK key MEASURED in the T10q gate (`research/sherlock-t10q-20261114/RESULTS.md`) | the bound 5,400 stays. The value-set CORE has 1,298 constraints > 1,000 (MEASURED in T8b), so a second CORE split is still needed | FIXED (fit; openvm `beba730` (branch `t10q`)) |
| F-6 N1 forces `reg_root` 0 (NT-2a) | N1 over leaf `[0, 5)` with public `reg_root` R(10) | `inputs/AFK-F6.json` | before T10q: refused by #549 to #556 `[pv[41..49]]` (First: pv = e0 and e0 reg_root = 0). After: accepted | First takes `reg_root` as a free public value, as `keys_d`; JOIN still binds equal roots (AFK-F3) | FIXED (openvm `beba730` (branch `t10q`); `afk_f6_first_carries_nonzero_reg_root`) |
| F-7 N1 `shape_d` 0, N2 cut collision (NT-1) | N2 over N1[0,1)+N1[1,5) and N2 over N1[0,3)+N1[3,5) (model cuts 2 and 5 of 8); the second proof presented with the first one's `shape_d` | `inputs/AFK-F7.json` | before T10q: accepted; both N1 `shape_d` are 0 and both N2 `shape_d` are equal. After: the two cuts give different N1 and N2 `shape_d`, and the second proof with the first one's `shape_d` is refused | bus 961 (shape sponge) unbalanced: N1 computes `shape_d` from its cut (a leaf `SHK_SHAPE` per row, one MERGE node shape over them) | FIXED (openvm `beba730` (branch `t10q`); `afk_f7_n2_shape_binds_the_cut`) |
| F-8 revocation after `acceptCollection` | none: ledger finality is a spec point | none | none | none | OPEN DESIGN (owner item T15) |
| F-9(a) `KSEL_NODE` fixed in the shape call | code reading: `gp::shape_w` writes `KSEL_NODE` for both slots (gp.rs:731-732); foreign slots do not exist yet | none | none | none | OPEN DESIGN (GP-3 foreign wiring not built) |
| F-9(a) `Kind::Fv` outputs `STMT_FV` | code reading: stmt.rs:1143 and 1857 set `TAG = STMT_FV` (T7 stand-in) | none | none | none | OPEN DESIGN |
| F-9(b) MERGE-WORK needs two children | MERGE-WORK with one SEG child [0, 4) and the inline step at 4 (the honest merge-work witness with slot 1 off) | `inputs/AFK-F9.json` | before T10q: refused by #747 `[valid']` (`fr.assert_one(en1)`) and the W link to slot 1 (#748 ff.). After: accepted; the same witness with the step not linked to the child is refused by `[child_words+17, wk+16, valid+0', child_words+17']` (`afk_f9_work_one_child_unlinked_refused`) | W links to the right-most enabled child | FIXED (openvm `beba730` (branch `t10q`); `afk_f9_work_one_child`) |

## gp2-value-set

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F1.1 to F1.6 dispatch and DEC padding | mutants `sel_no_bool` (sel 51, 77), `sel_sum_free`, `key_wrong_class`, `pad_on_bus` | `inputs/GP2-M-*.json` | refused | #24 `[sel+0]` (assert_bool); #32 `[en, sel+0, sel+1]` (ssum = en); #33 `[sel+0, sel+1, key]`; DecVAir #17 row 3 `[e+3..e+7, mult]` (pad * mult = 0) | GUARD HOLDS |
| F2.1 CNULL guards | mutants `cnull_no_running`, `no_hlp`, `no_res`, `no_ctl_eval` (continuation shard, forged pre state) and `no_expr_kind`, `no_expr_arity`, `no_has_const`, `no_ctor_type`, `no_ctor_arity`, `no_cst_kind`, `no_marker_free`, `no_cst_fields` (malformed ROM rows), `no_cid_bind`, `no_rom` | `inputs/GP2-M-cnull_*.json` | refused | #530 `[st.OT, sel+0]`, #531 `[st.hlp.0]`, #532 `[st.res.0]`, #533 `[st.ctl.0]`, #534 `[rom+0]`, #538 `[rom+13]`, #535 `[rom+17]`, #539 `[rom+1, r2+0]`, #540 `[r2+16]`, #536 `[r3+0]`, #537 `[r3+1]`, #543 `[rom+5, r3+10]` (ctor lane), #545 `[rom+21, cidx+0]`; bus 68 (ROM) for `no_rom` | GUARD HOLDS |
| F2.2 PUBV guards | mutants `pubv_no_running`, `no_hlp`, `no_res`, `no_ctl_ret`, `no_val_const`, `no_cid_hi`, `no_frm_zero`, `no_cid_bind`, `no_rom` | `inputs/GP2-M-pubv_*.json` | refused | #642 `[st.OT, sel+1]` row 2, #643, #644, #645 `[st.ctl.0]`, #647 `[st.ctl.1]`, #646 `[st.ctl.2+4]`, #648 `[st.frm.count]`, #649 `[st.ctl.2, cidx]`; bus 68 for `no_rom` | GUARD HOLDS |
| F2.3 effects | mutants `pubv_payload_free`, `cnull_ctl_free` | `inputs/GP2-M-*.json` | refused | #660 `[sel+1, r3+22, st.OP']`; #578 `[sel+0, cidx+0, st.ctl.2']` | GUARD HOLDS |
| F3.1 frame rule | mutant `frame_skip_word` (`reg.19` changed across the PUBV step) | `inputs/GP2-M-frame_skip_word.json` | refused | #122 row 1 `[b, st.reg.19, st.reg.19']` | GUARD HOLDS |
| F4.1 INIT `val_d` only in chip set 2 | mutant `init_val_any_set`: not replayed. Chip set 0 CORE has no `C_VVAL` column, so there is no cell to forge (core.rs:661-665) | none | none | layout | GUARD HOLDS (structural, not replayed) |
| F4.2 ROM table typing | mutant `rom_no_tbl_bind` (PUBV reads the table-0 row as constants[0]) | `inputs/GP2-M-rom_no_tbl_bind.json` | refused | bus 68 (ROM) on `[19, ...]` | GUARD HOLDS |
| F4.2 `const_vd` ignores the nominal type | `leaf::halted_vd` of `ProgV::P` (type U) and `ProgV::C` (type V) | `inputs/GP2-F4.2.json` | before T10q: `prog_d` differ, the two Halted digests are equal. After: the digests differ | `const_vd` and the table 9 and 15 ROM rows use the T5 5.2 type normal form (`name_d`, `decl_d`, `ty_d_row`); host and ROM only, no circuit key changes for it | FIXED (openvm `beba730` (branch `t10q`); `gp2_f42_const_vd_binds_the_type_name`) |
| C2.1 R_PUBV trusts the constant row | `unitctor` with `constants[0]` marker_free 0; the PUBV shard of the cut [1, 1] (h_pre 1) alone | `inputs/GP2-C2.1.json` | accepted; the genesis CNULL shard of the same program is refused (#537 `[r3+1]`, mutant row `cnull_no_marker_free`) | core.rs:439-449 has no `r3(0) = 6`, `r3(1) = 1`, `tsk.count = 0` | CONFIRMED (conditional: the pre state is unreachable from genesis under chip sets 0 to 2) |

## pathc-xcheck

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| 1 `L` alone cannot bind `X` | T7c cases `pathc_x_not_the_l_commitment_accepted_at_L` and `..._refused_at_L_Fr`, `..._refused_by_stock_L_Fr_verifier` (real prover, stock verifiers) | T7c run | accepted at `L`, refused at `L_Fr` | the `L_Fr` X check | GUARD HOLDS (MEASURED in T7c, cited) |
| 2 partial limb packing | not attempted (the lane reads that the `L_Fr` X check sums all 9 limbs; not checked here) | none | none | none | NOT ATTEMPTED (lane says sound) |
| 3 `L` key compiled into `L_Fr` | not attempted here; T7c measured one `L` key over N3 and both path C leaf layers | none | none | `L_Fr` tape constants | NOT ATTEMPTED (lane says sound) |
| 4 cross presentation | T7c case `pathc_x_of_the_full_tree_refused_at_L` | T7c run | refused | StmtAir (Root) DAG leaf to KeyPathAir to RootSealAir | GUARD HOLDS (MEASURED in T7c, cited) |
| 5 wrap pins `X == sh.x` | decode an `X` other than `keys.x` (limb 0 xor 1) | `inputs/PATHC-F5-wrap-x-unpinned.json` | accepted | iface.rs:749 and decode.rs:368 keep only the range and Canon of `X` | NOT REPRODUCED (fixed in T10) |
| 6(a) non-canonical X, carry family | `X` limbs `(l_k-1 + 2^30, l_k - 1)` | `inputs/PATHC-F6a-x-limb-carry.json` | refused | `dec`: "X: range or Canon" | GUARD HOLDS |
| 6(b) non-canonical X, `X + r` family | `X` limbs := `limbs30(X + r)` | `inputs/PATHC-F6b-x-plus-r.json` | refused | `dec`: "X: range or Canon" | GUARD HOLDS |
| 6 at `L` | T7c case `pathc_wrong_x_refused_at_L` | T7c run | refused | DAG key leaf leaves `key_root` | GUARD HOLDS (MEASURED in T7c, cited) |
| 7 slim Stmt and Call AIRs | not attempted (lane reading: public values are observed in `L`'s transcript and `decode.rs` reads every lane) | none | none | none | NOT ATTEMPTED (lane says sound) |
| 8 omitting `enc(X)`, `hp`, `H_PRE = 0`, KeyPathAir, RootSealAir | T7c cases `pathc_child_key_outside_key_root_refused_at_L`, `pathc_wrong_prog_d_opening_refused_at_L`; omit set (every AIR required) | T7c run | refused | KeyPathAir, RootSealAir, StmtAir (Root) | GUARD HOLDS (MEASURED in T7c, cited) |

### pathc-xcheck mutants (lane section 3)

| mutant | attempt | verdict |
|---|---|---|
| `m_dag_leaf_no_x` | T7c `pathc_wrong_x_refused_at_L` (X limb 0 xor 1, stock `L` verifier) | GUARD HOLDS (MEASURED in T7c, cited) |
| `m_keypath_no_root` | T7c `pathc_child_key_outside_key_root_refused_at_L` | GUARD HOLDS (MEASURED in T7c, cited) |
| `m_lfr_no_x_check`, `m_wrap_drops_x_and_lfr_drops_x` | T7c `pathc_x_not_the_l_commitment_refused_at_L_Fr` and `..._refused_by_stock_L_Fr_verifier` | GUARD HOLDS (MEASURED in T7c, cited) |
| `m_dag_leaf_no_hpre`, `m_hpre_free`, `m_pc_free`, `m_seal_no_keyroot` | not attempted: needs an `L` statement build (`Kind::Root` with `PcWit`, KeyPathAir, RootSealAir) | NOT ATTEMPTED |
| `m_lfr_no_l_vk_pin`, `m_lfr_x_check_low_limbs_only` | not attempted: needs the `L_Fr` tape | NOT ATTEMPTED |
| `m_wrap_ignores_slim_call` | not attempted: needs the wrap circuit | NOT ATTEMPTED |

## Observation outside the lanes

A constants row whose `fields start` lane (lane 4) is 1 is accepted by a genesis CNULL step
(diagnostic `gp2_cst_fields_start`, MEASURED, local checker), and the Halted value is unchanged.
T10q verdict: GUARD HOLDS, reference-consistent, kept as the positive test
`gp2_cnull_fields_start_free_as_in_the_reference`. The compiler writes `start =
constant_fields.len()` for every constant, also for length 0 (`mori-core/src/sherlock/program_image.rs:305`).
The reference nullary rule reads `len` only (`owner_adt_runtime.rs:208-213`), and admission bounds
`start` by the table length only (`owner_ext_admit.rs:472-473`). A `start = 0` constraint would
refuse honest programs. The lane's `m_cnull_no_cst_fields` changes the constant's ctor lane, and
that mutant is refused (`[sel+0, rom+5, r3+10]`).

Since T10q, `program_v` refuses the ROM edits of `cnull_no_ctor_type`, `cnull_no_ctor_arity` and
`cnull_no_cst_fields` (the host builds the type normal form and `const_vd` from the edited rows and
asserts on them). The handler then attacks with the CORE cell edit, and the named constraint still refuses.
