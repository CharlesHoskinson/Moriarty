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
| forgery accepted (`L_Fr` tape, chunk mode) | T10B-F03 | FIXED in T10q-cx (openvm `bcf2cfd` (branch `t10q-cx`)) |

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

## t10-bound (T10q-cx)

Lane `qa-20261114-t10-bound` finding F-03 (verdict D-2). Handler `lfr` (`tests/qr/lfr.rs`): the
real `L_Fr` tape (`gen_r_check`, then `gen_ops_check`) on chunk-mode `L` words built by
`iface::case_run_bound` with the run operands of program B (`r_P`, `prog_d` B) and the ledger
anchor operand `anchor_d2(r_P, prog_d A)` on every `SHK_PROGC` anchor entry, checked by
`Tape::execute_checked`; then the wrap's decoder twin `iface::decode_with`. Before: openvm `dd3593e`
(`logs/t10q-cx-fails-before.txt`); after: `bcf2cfd` (`logs/t10q-cx-after.txt`). MEASURED.

| finding | attempt | input | result | constraint | verdict |
|---|---|---|---|---|---|
| F-03 chunk 1 anchor (`A_0 = C_P`) | chunk 1 of 1 and 1 of 2 of program B, `a_in` operand = ledger anchor of A | `t10-bound-F03-chunk-j1-anchor.itf.json` (lane witness `modeBit 1`, `chunkJ 1`, `chunkK 1`); `inputs/T10B-F03-chunk-j1-k*-forged.json` | before: tape accepted (0 failed assertions) and the decoder accepts with the `a_in` lane = `C_L` of ledger A and the `TPL_CHUNK` lanes. After: refused | `gen_ops_check` assert #3: the `a_in` operand equals the `SHK_PROGC` operand when `tag_in` is `SHK_PROGC` (ORD or chunk 1) | FIXED (openvm `bcf2cfd`; `t10b_f03_chunk_anchor_is_c_p`) |
| F-03 chunk `k` anchor (`A_k = C_P`) | chunk 2 of 2, `a_out` operand = ledger anchor of A | `inputs/T10B-F03-chunk-j2-k2-forged.json` | before: no check on `a_out` in the code. After: refused | `gen_ops_check` assert #4: a chunk's `a_out` operand equals the `SHK_PROGC` operand when `tag_out` is `SHK_PROGC` | FIXED (`t10b_f03_chunk_anchor_ends`) |
| F-03 controls | honest chunks 1/2, 2/2, 1/1, 2/3 | `inputs/T10B-F03-chunk-*-honest.json` | accepted before and after | none | positive |

Reach (DERIVED from the source): the head words (`mode`, `j`, `k`, `e_in`, `e_out`) are
prover-chosen public values of the `Stmt` interface AIR, and the anchor entries are bound only by
the onion digest, so the `L` prover can produce chunk-mode words. Only `t7_chain::lfr_gen_any`
(the `L_Fr` tape of `t7_chain t10`, `cases`, `pathc` and the T16 runner) calls `gen_ops_check`;
the tapes of `j5_gate`, `lfr_e3`, `slice_chain` and `fp8_keyset` run `gen_r_check` alone and never
had the anchor tie. On `SherlockLedger` the forgery is not reachable: a mode-1 call vector must
carry the `TPL_CHUNK` anchor-write lanes 11 to 20 and `LANE_A_OUT = a_out` (`decode.rs:598-605`),
and no entry of `tools/sherlock-sdk/sherlock-ledger.compact` writes the anchor cell (the header:
"the ledger has no chunked entry"). It is live for any contract with a chunked entry (the
ARCHITECTURE chunk path, FINAL-DESIGN anchor lock `A_0 = A_k = C_P`).

End to end (`t7_chain t10 chunkcx`, unit `t10q-cx-e2e`, `logs/t10q-cx-chunkcx.{txt,json}`, MEASURED):
the real `L` prover over the N3 proof of program A, with chunk-mode words (mode 1, chunk 1 of 1)
whose `SHK_PROGC` anchor entries carry the ledger anchor operand of program B. The stock `L`
verifier accepts this `L` proof. The `L_Fr` tape with the anchor rule as built has 0 failed
assertions, and the decoder twin accepts with the `a_in` lane equal to `C_L` of ledger B: a proof
of A presents itself to a B ledger. With the fix the tape has 2 failed assertions (`A_0`, `A_k`). Scope: the `L_Fr` tape runs natively
(`execute_checked`); no `L_Fr` STARK proof, wrap proof or ledger call was made. The "as built" rule in
this run is a replica of the pre-fix `gen_ops_check` in `t7_chain.rs` (`gen_ops_check_as_built`);
the pre-fix function itself was measured at word level (`logs/t10q-cx-fails-before.txt`).
The honest ORD run passes both tapes. Run on openvm `475b80d` (branch `t10q-cx-e2e` = `beba730`
plus the fix), because the `L` key of the t10q tip `8bff541` does not pass the FP8 G1 bounds at
this time (`Interactions { set: "L", air: 0, got: 107, bound: 100 }`; `dd3593e`: 2,003 columns
against 2,000); the `L_Fr` check does not depend on that change.

Keys: the `L_Fr` tape gains the `A_k` check on every call, so the `L_Fr` key and the wrap key
that pins it change (DERIVED). `L`, N1 to N3 and the leaf keys do not change. MEASURED on the T10 ORD tape: 499,360 ops
as built, 499,623 fixed (+263), and the 14 `L_Fr` machine AIR heights are equal (largest 2^19,
bound 2^20), so the `L_Fr` key shape does not change (MEASURED); the `L_Fr` program table and its
commitment change, so the `L_Fr` key changes and the wrap key that pins it changes with the same
circuit size (DERIVED).

Opening-proof form of a chunk (T10q-cx follow-up, `iface::Case::OpenChunk`, openvm `4d8177b`):
`DIGEST(SHK_SET)` at entry 4, `a_in` at entry 5, `a_out` at entry 6, `e_in = 5`, `e_out = 6`.
Test `t10b_f03_open_chunk_anchor_ends` (`logs/t10q-cx-lock-after.txt`, MEASURED): honest open
chunks 1/2, 2/2, 1/1 and 2/3 pass the tape and the decoder twin; a forged `a_in` of chunk 1 is
refused by assert #3 and a forged `a_out` of chunk `k` by assert #4. This covers the tape rule at
entries 5 and 6. The lane layout of an open chunk is ESTIMATE: the `openClaim` lanes moved by the
11 lanes of the anchor write (as `LANE_CLAIM` for a chunk) and `a_out` at `LANE_A_OUT` in the call
vector, not lane-mapped (the five pairs are used by `openClaim`); no compiled ledger entry has this
form.

### T10B-LOCK: the lock anchors of a chunked run (T10q-cx follow-up)

Source: the F-03 work above (code review of `gen_ops_check`, not a Quint lane finding; the
t10-bound model has no lock operand, so there is no ITF trace). Handler `lfr::lock_call`: chunk
`j` of `k` of program B with the honest lock operands of its own run, or, on one position, the
lock operand of a run of program A (another `C_P` operand and `run_tag`). Before: openvm `4d8177b`
(`logs/t10q-cx-lock-fails-before.txt`); after: `0a4a2c8` (`logs/t10q-cx-lock-after.txt`). MEASURED.

| attempt | input | result | constraint | verdict |
|---|---|---|---|---|
| chunk 1 of 3 of B writes `a_out` = `A_1` of A's run | `inputs/T10B-LOCK-chunk-j1-k3-a_out-of-A.json` | before: tape accepted (0 failed assertions), decoder accepts. After: refused | `gen_ops_check` assert #6, lock rule `A_j` (`a_out`) | FIXED (`t10b_chunk_lock_is_bound`) |
| chunk 2 of 3 of B reads `a_in` = `A_1` of A's run | `inputs/T10B-LOCK-chunk-j2-k3-a_in-of-A.json` | after: refused | assert #5, lock rule `A_(j-1)` (`a_in`) | FIXED |
| chunk 2 of 3 of B writes `a_out` = `A_2` of A's run | `inputs/T10B-LOCK-chunk-j2-k3-a_out-of-A.json` | after: refused | assert #6 | FIXED |
| chunk 3 of 3 of B reads `a_in` = `A_2` of A's run | `inputs/T10B-LOCK-chunk-j3-k3-a_in-of-A.json` | after: refused | assert #5 | FIXED |
| chunk 65,536 of 65,537 reads a foreign `A_65535` (borrow of `j - 1` across the 16-bit limb) | `inputs/T10B-LOCK-chunk-j65536-k65537-a_in-of-A.json` | after: refused | assert #5 | FIXED |
| controls: honest chunks 1/3, 2/3, 3/3, 1/2, 2/2, 65,536/65,537, 65,537/65,537 | `inputs/T10B-LOCK-chunk-*-honest.json` | accepted before and after | none | positive |

Verdict CONFIRMED. As built, every `SHK_LOCK` anchor operand was free in the `L_Fr` tape (no code
outside the ENC-1 test vector computes `DOM_LOCK`). No other check ties a middle chunk to `prog_d`
or to its run: the decoder twin checks only the `TPL_CHUNK` template lanes and `a_in`, `a_out`
lanes against the logged anchors; the claim chain `(addr_c, ep_hash, comm_(j+1))` of FINAL-DESIGN
and ANC-010 has no code yet; `pre_d`, `post_d` of a chunk are `L` words that no later chunk reads.
So a middle chunk of program B could read lock `A_1` of contract A's run and write `A_2`: B's code
applied to A's state between A's chunk 1 and chunk `k`. Scope as F-03: native tape plus decoder
twin; on `SherlockLedger` not reachable (no chunked entry, DERIVED).

Fix (openvm `0a4a2c8`): `gen_ops_check` ties `a_in` under `SHK_LOCK` to `x_(j-1)` and `a_out` under
`SHK_LOCK` to `x_j`, with `x_j = sponge(U_LOCK = 8; c, pack(run_tag), j)` over Fr
(`iface::lock_x`): `c` is the operand of the `DIGEST(SHK_PROGC)` entry, already tied to the `L`
root statement (`P2(DOM_PROGC, r_P, prog_d)`), `run_tag` the 8 head words 81 to 88, `j = j_lo +
2^16 j_hi` (each limb range-checked to 16 bits in `gen_r_check`). 3 `Perm` operations, no BabyBear
`P2` in the tape, no change to `L` or `OPS_WORDS`. Lock values stay `A_j = DIGEST(SHK_LOCK, x_j)`.
The use code 8 is outside the OD-18 codes 0 to 7, so no other sponge shares the capacity.

PICK for T15 confirmation (design text): ANC-002
(`openspec/changes/sherlock-processor-final-20261111/specs/sherlock-anchor-chunks/spec.md`),
FINAL-DESIGN 1.6 line 172 and the ENC-1 test vector (`iface.rs`, `d2_lock`) write the lock
operand as `pack(P2(DOM_LOCK, r_P, prog_d, run_tag, j))`. The `L_Fr` tape has no BabyBear `P2`,
so that form needs two more `P2` calls in the `L` root statement (an `L` key change, while the
`L` key of the t10q tip is at the FP8 G1 bounds). The Fr form binds the same values (`r_P` and
`prog_d` through `c`). These texts are not changed here. Zero knowledge: `A_j` is public on the
ledger as before; it hides `prog_d` and the run as long as `run_tag` stays a value of `L` only
(never a wrap public input), unchanged from FINAL-DESIGN.

Keys: the `L_Fr` key and the wrap key change (DERIVED); `L`, node and leaf keys do not.
MEASURED on the T10 ORD tape (`t7_chain t10 chunkcx` on openvm `a74fbc9` = `475b80d` plus
`4d8177b` and `0a4a2c8`, `logs/t10q-cx-lock-chunkcx.txt`): 499,657 ops with the lock rule (+34
over 499,623), and the 14 `L_Fr` machine AIR heights are unchanged (largest 2^19, bound 2^20).
The honest ORD run passes; the chunk 1 of 1 forgery still fails 2 assertions.

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
