# Counterexamples for the Quint findings

Each finding of the four lanes has one row. The row gives the attempt made on the real code, its
input, the result, the constraint involved and a verdict:

- **CONFIRMED**: the code as built accepts the forgery, or refuses the honest witness.
- **NOT REPRODUCED**: the code as built no longer shows the defect (fixed since the lane read it).
- **GUARD HOLDS**: the attack is refused, and the refusing constraint is the one the finding names.
- **OPEN DESIGN**: the finding concerns a design or prose point. There is no code to attack.

Results are MEASURED on openvm `a549644` (branch `quint-reg` on T10mm `d26453c`) with the local
checker of the replay driver (README), unless the row says otherwise. Inputs are in `inputs/`, or the
trace itself is the input (merge-work-ladder rows: the handler reads the cell values from the
witness).

## Defects found

| class | ids |
|---|---|
| forgery accepted | MW-F01-WORK-A, MW-F01-WORK-B (and the B rerun trace); AFK-F7 |
| honest witness refused | AFK-F6, AFK-F9(b) |
| host/prose mismatch | GP2-F4.2 |
| conditional (standalone shard only) | GP2-C2.1 |

No attack that a lane called sound was accepted. Every guard row is refused by the constraint the
finding names.

## merge-work-ladder

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F-01 Work, test A (false carry) | MERGE-WORK at h_pre 4, step 1, `whi [2013200390, 1, 0, 0]`, `wcry [1, 0, 0, 0]`; then NW, NW2, N3 over it | `merge-work-ladder-F01-work-false-carry.itf.json` | accepted (Work and 3 rungs) | none: `whi` has no range lookup (stmt.rs:1050-1055); at `L`, `H_HI` passes as columns (code reading) | CONFIRMED |
| F-01 Work, test B (2^64 wrap bypass) | MERGE-WORK at h_pre 2^64 - 1, `whi [65536, 65535, 65535, 65535]`, `wcry 0` | `merge-work-ladder-F01-work-wrap-bypass.itf.json`, `...-wrap-bypass-run.itf.json` | accepted (Work and 3 rungs) | none (as above) | CONFIRMED |
| F-01 First | N1 over one leaf step at h_pre 65535: `hhi [65536, 0, 0, 0]`, `cry 0` | `inputs/MW-F01-FIRST.json` | refused | bus 963 (HHI_RANGE_BUS, T10) unbalanced on `[65536]` | NOT REPRODUCED (fixed in T10) |
| F-02 segment drop, swap, splice | the `w-left`, `no-pre-link`, `no-h-link`, `no-prog-link`, `h-wrap` and `merge-no-contig` attacks | the six traces | refused | #748 `[wk+16, child_words+17']` (W.pre_d = post_d(1)); #772 `[wk+40, child_words+29']`; #749 `[child_words+1, wk+8]`; #826 `[wcry+3]`; #734 `[child_words+17, gp.mode, valid', child_words+9']` | GUARD HOLDS |
| F-03 W's log omitted from `log_merge` | not attacked: CORE fixes `(LOG_D, LOG_N) = (D_EMPTY, 0)` (code reading) | none | none | CoreAir log columns | GUARD HOLDS (by code reading) |
| F-04 step words over STEP_PV_BUS | `stepv-once`, `stepv-wf-free`, `core-send-row0-only` | the three traces | refused | bus 197 (truncated CORE; a proof is refused first at the key's fixed CORE height of 8); #818 `[wf]`; bus 213 (STEP_PV_BUS) | GUARD HOLDS |
| F-05 key rows and half split | `row-unbound`, `half-split-off` | the two traces | refused at rung 0 | #61 `[valid, child_words+66, pv[66]]` (keys_d); #29 `[sel+7]` | GUARD HOLDS |
| F-06 parent key check, unique ladder | `key-check-none` (a fake SEG under a forged key) | the trace | refused at rung 0 | #6 `[valid, child_words+74, cc]` | GUARD HOLDS |
| F-07 `keys_d` universal | not attacked (consistency statement) | none | none | none | GUARD HOLDS (by code reading) |
| F-08 stale `UKeys` comment, F-09 Work in the second table half (prose) | not attacked | none | none | none | OPEN DESIGN (doc edits) |

## afk-picks

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F-1 `latest_root_only` breaks long runs | none: the ledger contract is prose | none | none | none | OPEN DESIGN |
| F-2 revocation of historical roots | none: the ledger contract is prose | none | none | none | OPEN DESIGN |
| F-3 mixed-root JOIN | Node JOIN of two SET children under `reg_root` R(10) and R(0) | `inputs/AFK-F3.json` | refused (the same JOIN with equal roots is accepted: AFK-HONEST) | #153 `[child_words+41, valid', child_words+41']` (gp.rs:313-315) | GUARD HOLDS |
| F-4 shape without intervals | the code as built binds `(h_lo, h_hi)` of a SEG output in `shape_d` (gp.rs:714-716), so the rejected option is not in the code; the residual collision is F-7 | none | none | none | GUARD HOLDS (by code reading; see F-7) |
| F-5 MERGE-WORK column bound | the widths of the Work statement and kernel AIRs, and of CORE in chip sets 0 and 2 | `logs/merge-work-columns.txt` | Work statement and inline kernel 3,735 columns (MEASURED, without the verifier sub-circuit); value-set CORE 738 vs 676 = +62 (MEASURED); MERGE-WORK key 5,284 (MEASURED in T8b); STACKS 71 words (ESTIMATE, ARCHITECTURE 1280-1282) | 5,284 + 62 + 71 = 5,417 > 5,400 (DERIVED). The value-set CORE has 1,298 constraints > 1,000 (MEASURED in T8b), so a second CORE split is needed too | OPEN DESIGN (bound does not fit STACKS + VALUE DIGEST; owner item T15) |
| F-6 N1 forces `reg_root` 0 (NT-2a) | N1 over leaf `[0, 5)` with public `reg_root` R(10) | `inputs/AFK-F6.json` | refused | #549 to #556 `[pv[41..49]]` (First: pv = e0 and e0 reg_root = 0) | CONFIRMED (honest witness refused) |
| F-7 N1 `shape_d` 0, N2 cut collision (NT-1) | N2 over N1[0,1)+N1[1,5) and N2 over N1[0,3)+N1[3,5) (model cuts 2 and 5 of 8); the second proof presented with the first one's `shape_d` | `inputs/AFK-F7.json` | accepted; both N1 `shape_d` are 0 and both N2 `shape_d` are equal | none: First copies `shape_d` 0 from the leaf envelope (stmt.rs:1063) | CONFIRMED (forgery accepted) |
| F-8 revocation after `acceptCollection` | none: ledger finality is a spec point | none | none | none | OPEN DESIGN (owner item T15) |
| F-9(a) `KSEL_NODE` fixed in the shape call | code reading: `gp::shape_w` writes `KSEL_NODE` for both slots (gp.rs:731-732); foreign slots do not exist yet | none | none | none | OPEN DESIGN (GP-3 foreign wiring not built) |
| F-9(a) `Kind::Fv` outputs `STMT_FV` | code reading: stmt.rs Fv branch sets `TAG = STMT_FV` (T7 stand-in) | none | none | none | OPEN DESIGN |
| F-9(b) MERGE-WORK needs two children | MERGE-WORK with one SEG child [0, 4) and the inline step at 4 (the honest merge-work witness with slot 1 off) | `inputs/AFK-F9.json` | refused | #747 `[valid']` (`fr.assert_one(en1)`), and the unconditional W link to slot 1 (#748 ff.) | CONFIRMED (honest witness refused) |

## gp2-value-set

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F1.1 to F1.6 dispatch and DEC padding | mutants `sel_no_bool` (sel 51, 77), `sel_sum_free`, `key_wrong_class`, `pad_on_bus` | `inputs/GP2-M-*.json` | refused | #24 `[sel+0]` (assert_bool); #32 `[en, sel+0, sel+1]` (ssum = en); #33 `[sel+0, sel+1, key]`; DecVAir #17 row 3 `[e+3..e+7, mult]` (pad * mult = 0) | GUARD HOLDS |
| F2.1 CNULL guards | mutants `cnull_no_running`, `no_hlp`, `no_res`, `no_ctl_eval` (continuation shard, forged pre state) and `no_expr_kind`, `no_expr_arity`, `no_has_const`, `no_ctor_type`, `no_ctor_arity`, `no_cst_kind`, `no_marker_free`, `no_cst_fields` (malformed ROM rows), `no_cid_bind`, `no_rom` | `inputs/GP2-M-cnull_*.json` | refused | #530 `[st.OT, sel+0]`, #531 `[st.hlp.0]`, #532 `[st.res.0]`, #533 `[st.ctl.0]`, #534 `[rom+0]`, #538 `[rom+13]`, #535 `[rom+17]`, #539 `[rom+1, r2+0]`, #540 `[r2+16]`, #536 `[r3+0]`, #537 `[r3+1]`, #544 `[r3+18]`, #545 `[rom+21, cidx+0]`; bus 68 (ROM) for `no_rom` | GUARD HOLDS |
| F2.2 PUBV guards | mutants `pubv_no_running`, `no_hlp`, `no_res`, `no_ctl_ret`, `no_val_const`, `no_cid_hi`, `no_frm_zero`, `no_cid_bind`, `no_rom` | `inputs/GP2-M-pubv_*.json` | refused | #642 `[st.OT, sel+1]` row 2, #643, #644, #645 `[st.ctl.0]`, #647 `[st.ctl.1]`, #646 `[st.ctl.2+4]`, #648 `[st.frm.count]`, #649 `[st.ctl.2, cidx]`; bus 68 for `no_rom` | GUARD HOLDS |
| F2.3 effects | mutants `pubv_payload_free`, `cnull_ctl_free` | `inputs/GP2-M-*.json` | refused | #660 `[sel+1, r3+22, st.OP']`; #578 `[sel+0, cidx+0, st.ctl.2']` | GUARD HOLDS |
| F3.1 frame rule | mutant `frame_skip_word` (`reg.19` changed across the PUBV step) | `inputs/GP2-M-frame_skip_word.json` | refused | #122 row 1 `[b, st.reg.19, st.reg.19']` | GUARD HOLDS |
| F4.1 INIT `val_d` only in chip set 2 | mutant `init_val_any_set`: not replayed. Chip set 0 CORE has no `C_VVAL` column, so there is no cell to forge (core.rs:661-665) | none | none | layout | GUARD HOLDS (structural, not replayed) |
| F4.2 ROM table typing | mutant `rom_no_tbl_bind` (PUBV reads the table-0 row as constants[0]) | `inputs/GP2-M-rom_no_tbl_bind.json` | refused | bus 68 (ROM) on `[19, ...]` | GUARD HOLDS |
| F4.2 `const_vd` ignores the nominal type | `leaf::halted_vd` of `ProgV::P` (type U) and `ProgV::C` (type V) | `inputs/GP2-F4.2.json` | `prog_d` differ, the two Halted digests are equal | leaf.rs:264-275 hashes `p2(45, [ty])` only | CONFIRMED (host/prose mismatch: nominal type collision) |
| C2.1 R_PUBV trusts the constant row | `unitctor` with `constants[0]` marker_free 0; the PUBV shard of the cut [1, 1] (h_pre 1) alone | `inputs/GP2-C2.1.json` | accepted; the genesis CNULL shard of the same program is refused (#537 `[r3+1]`, mutant row `cnull_no_marker_free`) | core.rs:431-444 has no `r3(0) = 6`, `r3(1) = 1`, `tsk.count = 0` | CONFIRMED (conditional: the pre state is unreachable from genesis under chip sets 0 to 2) |

## pathc-xcheck

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| 1 `L` alone cannot bind `X` | T7c cases `pathc_x_not_the_l_commitment_accepted_at_L` and `..._refused_at_L_Fr`, `..._refused_by_stock_L_Fr_verifier` (real prover, stock verifiers) | T7c run | accepted at `L`, refused at `L_Fr` | the `L_Fr` X check | GUARD HOLDS (MEASURED in T7c, cited) |
| 2 partial limb packing | code reading: the `L_Fr` X check sums all 9 limbs (t7_chain.rs X check) | none | none | none | GUARD HOLDS (by code reading) |
| 3 `L` key compiled into `L_Fr` | T7c: one `L` key over N3 and both path C leaf layers; omit set every AIR required | T7c run | as stated | `L_Fr` tape constants | GUARD HOLDS (MEASURED in T7c, cited) |
| 4 cross presentation | T7c case `pathc_x_of_the_full_tree_refused_at_L` | T7c run | refused | StmtAir (Root) DAG leaf to KeyPathAir to RootSealAir | GUARD HOLDS (MEASURED in T7c, cited) |
| 5 wrap pins `X == sh.x` | decode an `X` other than `keys.x` (limb 0 xor 1) | `inputs/PATHC-F5-wrap-x-unpinned.json` | accepted | iface.rs:749 and decode.rs:368 keep only the range and Canon of `X` | NOT REPRODUCED (fixed in T10) |
| 6(a) non-canonical X, carry family | `X` limbs `(l_k-1 + 2^30, l_k - 1)` | `inputs/PATHC-F6a-x-limb-carry.json` | refused | `dec`: "X: range or Canon" | GUARD HOLDS |
| 6(b) non-canonical X, `X + r` family | `X` limbs := `limbs30(X + r)` | `inputs/PATHC-F6b-x-plus-r.json` | refused | `dec`: "X: range or Canon" | GUARD HOLDS |
| 6 at `L` | T7c case `pathc_wrong_x_refused_at_L` | T7c run | refused | DAG key leaf leaves `key_root` | GUARD HOLDS (MEASURED in T7c, cited) |
| 7 slim Stmt and Call AIRs | code reading: public values are observed in `L`'s transcript; `decode.rs` reads every lane | none | none | none | GUARD HOLDS (by code reading) |
| 8 omitting `enc(X)`, `hp`, `H_PRE = 0`, KeyPathAir, RootSealAir | T7c cases `pathc_child_key_outside_key_root_refused_at_L`, `pathc_wrong_prog_d_opening_refused_at_L`; omit set (every AIR required) | T7c run | refused | KeyPathAir, RootSealAir, StmtAir (Root) | GUARD HOLDS (MEASURED in T7c, cited) |
