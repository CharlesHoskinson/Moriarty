# Quint regression traces

Regression tests built from the outputs of four Quint QA lanes of 2026-11-14
(`~/.cache/gemini-lanes/qa-20261114-{merge-work-ladder,afk-picks,gp2-value-set,pathc-xcheck}/`).
Each `*.itf.json` file is a trace that Quint 0.32.0 wrote with `--out-itf`. The command for each
trace is in `models/traces.tsv`. The replay driver maps the witness of each trace onto the real
code and asserts the verdict that the code must give.

## Layout

| path | content |
|---|---|
| `*.itf.json` | the Quint traces (a violating `quint run`, or a named `quint test`) |
| `manifest.json` | one row per trace: id, handler, expected verdict, status, refusing constraint (`refused_by`), named test |
| `inputs/` | the counterexample inputs that the handlers built (forged cells, public values, edited ROM rows). The tests write them again with `QR_DUMP_INPUTS=1` |
| `models/` | the lane models (`*.qnt`), `traces.tsv` and `run.sh`. `run.sh` reruns every command: each `quint run` must exit 1, each `quint test` must pass, and the new ITF states must equal the committed trace |
| `logs/` | `replay_all.txt` (green run), `models-run.txt` (`models/run.sh`: 61 of 61 OK, MEASURED), `fails-before.txt` (the 8 open-defect tests on the code as built), `merge-work-columns.txt` (AFK F-5 input) |
| `COUNTEREXAMPLES.md` | one row per lane finding: the attempt, its input, the result, the constraint, the verdict |

## The driver

The driver is in the openvm fork, `~/.cache/m7-wrap/src/openvm-quint-reg`, branch `quint-reg`, in
`crates/sherlock-leaf/tests/quint_regressions.rs` and `tests/qr/`. It reads this folder from
`QUINT_REGRESSIONS_DIR` (default `~/Projects/Moriarty/tests/quint-regressions`).

- `ck.rs` is the local checker. It runs the stock `check_constraints` on every AIR, then the LogUp
  multiset check on every bus except the child-proof buses of the verifier sub-circuit (public
  values, cached commitment, pre-hash). The Poseidon2 permute bus is checked as an oracle. A
  refusal names every failing constraint with its key index, its first failing row and the columns
  that it reads (`wk+16`, `child_words+41'`, `st.OT`, `pv[41]`), or names the unbalanced bus.
- `mw.rs` maps a merge-work-ladder witness (`wit`) onto the MERGE-WORK statement, its inline
  kernel and the NW, NW2 and N3 node rungs.
- `afk.rs`: the afk model has no cell values (its digests are token lists). Each afk trace is the
  model witness of one finding. The handler builds the code-level counterexample that the report's
  negative test names (NT-1, NT-2, F-9(b)) and scales the model's cuts of `[0, 8)` to the 5-step
  slice program.
- `gp2.rs` replays the chip set 2 leaf (`airs_value`) over `unitctor`. When a builder accepts the
  forged input, the attack is rebuilt at the inputs of the witness builders: `program_v` ROM rows,
  `ShardIn` states and selectors, `core::Opts`. When the builder asserts on the forged input, the
  attack is a cell edit of the generated trace, and the row note says so. A guard row passes only
  when the constraint that the lane's mutant drops is among the failing ones.
- `pathc.rs` runs the reference decoder `iface::decode_with` (the native twin of
  `tools/sherlock-wrap/src/decode.rs`).

A row with `expect: refuse` must name `refused_by`. A harness error (the witness could not be
built) fails the row. It never counts as a refusal. A trace with no manifest row fails the run.

## Run

```
cd ~/.cache/m7-wrap/src/openvm-quint-reg
CARGO_TARGET_DIR=~/.cache/quint-reg/target T6_SLICE_JSON=~/.cache/quint-reg/slice_v1.json \
QUINT_REGRESSIONS_DIR=~/Projects/Moriarty/tests/quint-regressions \
  heavy-run --mem 8G -- cargo test --release --locked --offline -p sherlock-leaf --test quint_regressions
# open defects (each fails on the code as built):
  ... --test quint_regressions -- --ignored mw_f01 afk_ gp2_
# the Quint side:
tests/quint-regressions/models/run.sh
```

`replay_all` passes on openvm `a549644` (branch `quint-reg`, on T10mm `d26453c`): 52 rows pass,
8 open-defect rows and 1 not-replayed row are skipped (`logs/replay_all.txt`, MEASURED). Each of
the 8 named defect tests fails before any fix (`logs/fails-before.txt`, MEASURED).

## Rows

Kind: guard (the code must refuse), defect (open; the named `#[ignore]` test asserts the fixed
behaviour), positive (the code must accept). Status: pass, ignored-open-defect, not-replayed.

| id | lane | finding | trace | Rust test | kind | status | code location |
|---|---|---|---|---|---|---|---|
| MW-HONEST | merge-work-ladder | honest MERGE-WORK -> NW -> NW2 -> N3 ladder (soundHonestTest) | `merge-work-ladder-honest.itf.json` | `replay_all` | positive | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-W-LEFT | merge-work-ladder | F-02 | `merge-work-ladder-w-left.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-NO-PRE-LINK | merge-work-ladder | F-02 | `merge-work-ladder-no-pre-link.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-NO-H-LINK | merge-work-ladder | F-02 | `merge-work-ladder-no-h-link.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-NO-PROG-LINK | merge-work-ladder | F-02 | `merge-work-ladder-no-prog-link.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-H-WRAP | merge-work-ladder | F-02 | `merge-work-ladder-h-wrap.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-MERGE-NO-CONTIG | merge-work-ladder | F-02 | `merge-work-ladder-merge-no-contig.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-STEPV-ONCE | merge-work-ladder | F-04 | `merge-work-ladder-stepv-once.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-STEPV-WF-FREE | merge-work-ladder | F-04 | `merge-work-ladder-stepv-wf-free.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-CORE-SEND-ROW0-ONLY | merge-work-ladder | F-04 | `merge-work-ladder-core-send-row0-only.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-ROW-UNBOUND | merge-work-ladder | F-05 | `merge-work-ladder-row-unbound.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-HALF-SPLIT-OFF | merge-work-ladder | F-05 | `merge-work-ladder-half-split-off.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-KEY-CHECK-NONE | merge-work-ladder | F-06 | `merge-work-ladder-key-check-none.itf.json` | `replay_all` | guard | pass | stmt.rs (Kind::Work, StmtAir::eval merge and MERGE-WORK blocks) |
| MW-F01-WORK-A | merge-work-ladder | F-01 Test A | `merge-work-ladder-F01-work-false-carry.itf.json` | `mw_f01_work_whi_false_carry_refused` | defect | ignored-open-defect | stmt.rs:1050-1055 whi/wcry (no range lookup) |
| MW-F01-WORK-B | merge-work-ladder | F-01 Test B | `merge-work-ladder-F01-work-wrap-bypass.itf.json` | `mw_f01_work_whi_wrap_bypass_refused` | defect | ignored-open-defect | stmt.rs:1050-1055 whi/wcry (no range lookup) |
| MW-F01-WORK-B-RUN | merge-work-ladder | F-01 Test B (quint run m_whi_no_range invRefine) | `merge-work-ladder-F01-work-wrap-bypass-run.itf.json` | `mw_f01_work_whi_wrap_bypass_run_refused` | defect | ignored-open-defect | stmt.rs:1050-1055 whi/wcry (no range lookup) |
| MW-F01-FIRST | merge-work-ladder | F-01 (First half, fixed in T10 by HHI_RANGE_BUS) | `merge-work-ladder-F01-first-false-carry.itf.json` | `replay_all` | guard | pass | stmt.rs:968 HHI_RANGE_BUS, HhiRangeAir |
| AFK-HONEST | afk-picks | honest unary N1 lifts, binary N2, JOIN of equal reg_root | `afk-picks-honest.itf.json` | `replay_all` | positive | pass | stmt.rs Kind::First, Kind::Node, gp.rs JOIN |
| AFK-F3 | afk-picks | F-3 / C2.2 JOIN reg_root equality (gp.rs:313-315) | `afk-picks-F3-join-mixed-roots.itf.json` | `replay_all` | guard | pass | gp.rs:313-315 JOIN reg_root equality |
| AFK-F6 | afk-picks | F-6 N1 forces reg_root 0 (NT-2a) | `afk-picks-F6-n1-zero-reg-root.itf.json` | `afk_f6_first_carries_nonzero_reg_root` | defect | ignored-open-defect | stmt.rs:854-874, 1028-1075 (First env reg_root 0, pv == e0) |
| AFK-F7 | afk-picks | F-7 N1 shape_d 0, N2 cut collision (NT-1) | `afk-picks-F7-n1-zero-shape.itf.json` | `afk_f7_n2_shape_binds_the_cut` | defect | ignored-open-defect | stmt.rs:854-874, 1063 (First keeps shape_d 0); gp.rs:711-744 |
| AFK-F9 | afk-picks | F-9(b) MERGE-WORK needs two children | `afk-picks-F9-work-one-child.itf.json` | `afk_f9_work_one_child` | defect | ignored-open-defect | stmt.rs:1048 fr.assert_one(en1) |
| GP2-HONEST | gp2-value-set | honest unitctor shard (chip set 2) | `gp2-value-set-honest.itf.json` | `replay_all` | positive | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-F4.2 | gp2-value-set | F4.2 const_vd ignores the nominal type (U vs V) | `gp2-value-set-F4.2-const-vd-type-name.itf.json` | `gp2_f42_const_vd_binds_the_type_name` | defect | ignored-open-defect | leaf.rs:264-275 const_vd (p2(45, [ty])) |
| GP2-C2.1 | gp2-value-set | C2.1 R_PUBV lacks r3(0)==6, r3(1)==1, tsk.count==0 | `gp2-value-set-C2.1-pubv-marker-free.itf.json` | `gp2_c21_pubv_checks_the_constant_row` | defect | ignored-open-defect | core.rs:431-444 R_PUBV guards |
| GP2-M-CNULL-CTL-FREE | gp2-value-set | mutant gp2_value_set_m_cnull_ctl_free | `gp2-value-set-m-cnull-ctl-free.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-CID-BIND | gp2-value-set | mutant gp2_value_set_m_cnull_no_cid_bind | `gp2-value-set-m-cnull-no-cid-bind.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-CST-FIELDS | gp2-value-set | mutant gp2_value_set_m_cnull_no_cst_fields | `gp2-value-set-m-cnull-no-cst-fields.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-CST-KIND | gp2-value-set | mutant gp2_value_set_m_cnull_no_cst_kind | `gp2-value-set-m-cnull-no-cst-kind.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-CTL-EVAL | gp2-value-set | mutant gp2_value_set_m_cnull_no_ctl_eval | `gp2-value-set-m-cnull-no-ctl-eval.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-CTOR-ARITY | gp2-value-set | mutant gp2_value_set_m_cnull_no_ctor_arity | `gp2-value-set-m-cnull-no-ctor-arity.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-CTOR-TYPE | gp2-value-set | mutant gp2_value_set_m_cnull_no_ctor_type | `gp2-value-set-m-cnull-no-ctor-type.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-EXPR-ARITY | gp2-value-set | mutant gp2_value_set_m_cnull_no_expr_arity | `gp2-value-set-m-cnull-no-expr-arity.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-EXPR-KIND | gp2-value-set | mutant gp2_value_set_m_cnull_no_expr_kind | `gp2-value-set-m-cnull-no-expr-kind.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-HAS-CONST | gp2-value-set | mutant gp2_value_set_m_cnull_no_has_const | `gp2-value-set-m-cnull-no-has-const.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-HLP | gp2-value-set | mutant gp2_value_set_m_cnull_no_hlp | `gp2-value-set-m-cnull-no-hlp.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-MARKER-FREE | gp2-value-set | mutant gp2_value_set_m_cnull_no_marker_free | `gp2-value-set-m-cnull-no-marker-free.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-RES | gp2-value-set | mutant gp2_value_set_m_cnull_no_res | `gp2-value-set-m-cnull-no-res.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-ROM | gp2-value-set | mutant gp2_value_set_m_cnull_no_rom | `gp2-value-set-m-cnull-no-rom.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-CNULL-NO-RUNNING | gp2-value-set | mutant gp2_value_set_m_cnull_no_running | `gp2-value-set-m-cnull-no-running.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-FRAME-SKIP-WORD | gp2-value-set | mutant gp2_value_set_m_frame_skip_word | `gp2-value-set-m-frame-skip-word.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-INIT-VAL-ANY-SET | gp2-value-set | mutant gp2_value_set_m_init_val_any_set | `gp2-value-set-m-init-val-any-set.itf.json` | `replay_all` | guard | not-replayed | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-KEY-WRONG-CLASS | gp2-value-set | mutant gp2_value_set_m_key_wrong_class | `gp2-value-set-m-key-wrong-class.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PAD-ON-BUS | gp2-value-set | mutant gp2_value_set_m_pad_on_bus | `gp2-value-set-m-pad-on-bus.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-CID-BIND | gp2-value-set | mutant gp2_value_set_m_pubv_no_cid_bind | `gp2-value-set-m-pubv-no-cid-bind.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-CID-HI | gp2-value-set | mutant gp2_value_set_m_pubv_no_cid_hi | `gp2-value-set-m-pubv-no-cid-hi.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-CTL-RET | gp2-value-set | mutant gp2_value_set_m_pubv_no_ctl_ret | `gp2-value-set-m-pubv-no-ctl-ret.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-FRM-ZERO | gp2-value-set | mutant gp2_value_set_m_pubv_no_frm_zero | `gp2-value-set-m-pubv-no-frm-zero.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-HLP | gp2-value-set | mutant gp2_value_set_m_pubv_no_hlp | `gp2-value-set-m-pubv-no-hlp.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-RES | gp2-value-set | mutant gp2_value_set_m_pubv_no_res | `gp2-value-set-m-pubv-no-res.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-ROM | gp2-value-set | mutant gp2_value_set_m_pubv_no_rom | `gp2-value-set-m-pubv-no-rom.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-RUNNING | gp2-value-set | mutant gp2_value_set_m_pubv_no_running | `gp2-value-set-m-pubv-no-running.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-NO-VAL-CONST | gp2-value-set | mutant gp2_value_set_m_pubv_no_val_const | `gp2-value-set-m-pubv-no-val-const.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-PUBV-PAYLOAD-FREE | gp2-value-set | mutant gp2_value_set_m_pubv_payload_free | `gp2-value-set-m-pubv-payload-free.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-ROM-NO-TBL-BIND | gp2-value-set | mutant gp2_value_set_m_rom_no_tbl_bind | `gp2-value-set-m-rom-no-tbl-bind.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-SEL-NO-BOOL | gp2-value-set | mutant gp2_value_set_m_sel_no_bool | `gp2-value-set-m-sel-no-bool.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| GP2-M-SEL-SUM-FREE | gp2-value-set | mutant gp2_value_set_m_sel_sum_free | `gp2-value-set-m-sel-sum-free.itf.json` | `replay_all` | guard | pass | core.rs:390-460 (value-set rules), dec.rs:140, rom.rs |
| PATHC-HONEST | pathc-xcheck | honest L_Fr words decode | `pathc-xcheck-honest.itf.json` | `replay_all` | positive | pass | iface.rs:749 decode_with; tools/sherlock-wrap/src/decode.rs:368 |
| PATHC-F5 | pathc-xcheck | Finding 5: X other than keys.x decodes (fixed in T10, decode.rs:368 / iface.rs:749) | `pathc-xcheck-F5-wrap-x-unpinned.itf.json` | `replay_all` | positive | pass | iface.rs:749 decode_with; tools/sherlock-wrap/src/decode.rs:368 |
| PATHC-F6A | pathc-xcheck | Finding 6(a): non-canonical X limbs (carry family) | `pathc-xcheck-F6a-x-limb-carry.itf.json` | `replay_all` | guard | pass | iface.rs:749 decode_with; tools/sherlock-wrap/src/decode.rs:368 |
| PATHC-F6B | pathc-xcheck | Finding 6(b): non-canonical X limbs (X + r family) | `pathc-xcheck-F6b-x-plus-r.itf.json` | `replay_all` | guard | pass | iface.rs:749 decode_with; tools/sherlock-wrap/src/decode.rs:368 |

## Limits

- The checker checks each AIR set locally. It proves nothing. The child-proof buses are taken as
  sound, as the models take them. For MW-STEPV-ONCE, the local refusal is the kernel range bus
  (bus 197) of the truncated CORE trace. In a proof, the key's fixed CORE height (8) refuses the
  trace first.
- MW F-01 Work: the forged `h_hi` passes the Work statement and the NW, NW2 and N3 rungs (MEASURED
  here). At `L`, `root_map` takes the child `H_HI` as columns and has no range lookup, and
  `decode.rs` does not read `h_hi`. This was found by reading the code; `L` was not replayed.
- The `L` and `L_Fr` findings of pathc-xcheck are covered by the T7c real-prover cases (mutant
  prover, stock verifier, `research/sherlock-t7c-pathc-20261114/RESULTS.md`). They were not run
  again here, because T10m held the heavy slots.
