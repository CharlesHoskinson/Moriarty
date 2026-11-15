# Counterexamples of the formal findings (sherlock-e2e)

Every finding of the Quint lanes, the replay and the Lean lanes gets a counterexample built on the real code (the openvm trees under `~/.cache/m7-wrap/src/`, the Moriarty worktrees, Mycroft). Heavy builds run on odin. The verdict is one of:

- **CONFIRMED**: the counterexample runs on the code and shows the defect.
- **NOT REPRODUCED**: the code does not show the defect (the finding is a spec text defect only, or a model defect).
- **GUARD HOLDS**: the code has a guard that stops the counterexample; the test that shows it is named.
- **OPEN DESIGN**: the spec leaves the point open (a GAP or an owner question); the code state is recorded.

Odin build tree for the openvm counterexamples: `~/sherlock/src/formal-cx-rekey-chips-dcf1b4c` (git archive of openvm `rekey-chips` dcf1b4c, the commit DDD.md names for Leaf Proving), target `~/sherlock/target/formal-cx`. Logs: odin `~/sherlock/formal-e2e/cx-*.log`.

## Leaf Proving

| Finding | IDs | Counterexample on the code | Result | Verdict |
|---|---|---|---|---|
| F-leaf-proving-Q1 | LFP-003, RSL-002, LFP-002 | Read `rsl/trace.rs:48-49` and `names()` (`:121-138`): `MAX_APP_AIRS = 8` counts the app AIRs without FP-1; a SHA-256 or Keccak-256 app has 8, so 9 with the FP-1 probe AIR. The literal reading of LFP-003 (FP-1 counted) is not what the code does. | the 9-AIR d04 and s02 shapes pass the `<= 8` assert | NOT REPRODUCED (spec text: LFP-003 must say "the FP-1 AIR not counted") |
| F-leaf-proving-Q2 | LFP-003, RSL-004, RSL-007, GP-001, CPR-005 | odin: `cargo test --release -p sherlock-leaf --test rsl_keccak rsl_sha_and_keccak_in_one_program_refused` (inserts a `PersistentHash` before the `Keccak256` of suite call s02) | `RSLKECCAK both: persistentHash and keccak256 in one program`, 1 passed (log `cx-leaf-q2.log`): `compile.rs:848` refuses with `Refuse::Unsupported` | OPEN DESIGN (the R_SL route has no proof for a mixed-hash program; GP-001 then depends on the interpreted family, EUI-010; D-024 owner question for T15) |
| F-leaf-proving-Q3 | RSL-001, SPL-004, LFP-002 | Terminology: per-program leaf-layer keys are committed under `key_root` (`t7_chain.rs` `keys()`); "universal leaf key" is defined in DDD.md section 7 as the N1 pre-hash property | no behaviour differs | NOT REPRODUCED (wording only) |
| F-leaf-proving-Q4 | LFP-005 | LFP-005 status scenario states the GAP: chip set 3 has no named shape vector | - | OPEN DESIGN |
| F-leaf-proving-Q5 | LFP-012, LFP-008, INIT-002, INIT-007 | The R_SL chip set proves one call as one app shard (`rsl/trace.rs` `ChipSet::new` over the whole op list); no R_SL continuation shard exists in the code, so no shard can consume a tape slice | the multi-shard R_SL case cannot be built | OPEN DESIGN (the text must fix the run-level tape digest before R_SL gets continuation shards) |
| F-leaf-proving-Q6 | LPC-018, LFP-011, LFP-006 | LFP-006 is the AIR constraint and LFP-011 the prover fill; the R_SL `PROG_D` binding is TO BUILD in both status scenarios (`rsl/air.rs:199-203`, probe carries zeros) | the pair of requirements binds `PROG_D`; the code gap is the known TO BUILD | NOT REPRODUCED (no spec defect; LPC-018 is SPEC-CHECKED) |
| R-leaf-1 | LFP-009, LFP-007 | Model defect (N1 merged before `LeafProofsReady`); the code driver `t7_chain` merges after the leaf records exist | regression `lp_R_leaf_1_N1WaitsForLeafProofsReadyTest`, trace `leaf-proving/traces/regressions/R-leaf-1.itf.json` | NOT REPRODUCED (Quint model fixed) |

## Slice B (Formal-Lean-B findings, resolved 2026-11-15 by the slice B resolver)

Odin trees: `~/sherlock/src/openvm-rekey-int-3826e38-fbbr2` (copy of openvm `rekey-int` 3826e38 plus the test `bridge/cx/fbbr2_test.rs`, not committed to openvm) and `~/sherlock/src/openvm-rekey-int-3826e38-fbbr2mut` (the same plus the mutant `bridge/cx/fbbr2_link_constant_mutant.py`), each with a fresh empty target. Logs: odin `~/sherlock/formal-e2e/cx-fbbr2.log`, `cx-fbbr2-link-constant.log`, `cx-fbagg1.log`; extracts in `bridge/cx/` and `aggregation/cx/`.

| Finding | IDs | Counterexample on the code | Result | Verdict |
|---|---|---|---|---|
| F-B-BR-1 | LTR-003, CLB-002, ANC-018 | Code read: openvm `rekey-int` 3826e38 `frozen.rs:202` `max_columns: 2200`; FP-3 `L` 2,071 DERIVED (`LIST.md:44`, `:47`) | the old text bound 2,000 refuses the FP-3 `L` key that the code accepts | CONFIRMED; FIXED in the text (2,200, LEAD PICK FP3-LCOLS for T15). Replay: Lean `Bridge.ltr003_fp3_accepted`, `ltr003_fp3_fits`, `pin_ltr003` (2,201 refused, 2,200 accepted) |
| F-B-BR-2 | CLB-005, ANC-002, ANC-017, HDM-001 | odin: `cargo test --release -p sherlock-leaf --test t13_t10q_chunk fbbr2` on rekey-int 3826e38: chunk 2 of 3 with `a_in`, `a_out` from the 6-operand text form | honest 7-operand chunk accepted; 6-operand chunk refused by the `L_Fr` tape (`gen_ops_check` asserts #5 and #6, lock rule `A_(j-1)` and `A_j`) | CONFIRMED (text against code); FIXED in the text to 7 operands (D-038 OWNER; LEAD PICK for T15) |
| F-B-BR-2 (link needed) | ANC-019, CLB-003 | odin mutant: `lock_x` and the `L_Fr` tape keep `LOCK_LEN` 7 but fix the link to the constant `pack(vd(Nil))`; full `t13_t10q_chunk` run | 12 pass, `fp3_k_is_the_true_slice_count` FAILS: records 1 and 2 of a 3-slice run close the anchor as a 2-chunk chain (anchor end `Ok`) | the link is a soundness binding, not an encoding detail; the code keeps it, no code fix request |
| F-B-AGG-1 | SHP-003, NAG-011, GPH-005 | odin: `cargo test --release -p sherlock-leaf --test quint_regressions afk_f9` on rekey-int 3826e38 (traces of Moriarty `t10q` `tests/quint-regressions`) | `afk_f9_work_one_child` accepted (one SEG child, step at 4); `afk_f9_work_one_child_unlinked_refused` refused (StmtAir `wk+16` link) | CONFIRMED (the code accepts one linked child); FIXED in the text: SHP-003 "one or two" (LEAD PICK for T15) |
| F-B-LFP-1 | LFP-003, RSL-002 | Code read: `rsl/trace.rs:137` asserts `<= 8`; `names()` builds only 5 or 8 app AIRs | no 6- or 7-AIR program can be built | GUARD HOLDS (text fix open with F-leaf-proving-Q1) |
| F-B-LFP-3 | LFP-003, GP-001, CPR-005 | typed refusal `Refuse::Unsupported` (`rsl/compile.rs:884`, test `rsl_sha_and_keccak_in_one_program_refused`, F-leaf-proving-Q2) | mixed-hash program refused by name | OPEN DESIGN closed by D-024 (OWNER 2026-11-15: temporary named refusal; a route stays open) |
| F-B-KM-2 | RKY-002, UKS-003, UKS-009, UKS-010 | Lean `KeyManagement.t25_refused_rekey_int` on rekey-int ca65f27 and LIST.md 11:22 | keygen refused only by `row-uncommitted` | GUARD HOLDS |
| F-B-BF-2 | PDM-004 | `~/.cache/box-bus/all.log:42` reads "STARK lfr_call half not run" | the cited record is the wrap baseline | CONFIRMED (text); FIXED: evidence OPEN, verification TO BUILD |
| Minor | UKS-010, LFP-006, PDM-001, NAG-005, SHP-008, SQG-013, D-012 | Reads of LIST.md 11:38 (rows at `:37`, `:44` to `:46`, `:51`, `:57`, `:63`, `:64`), dcf1b4c `core.rs:46`, wiki `log.md:3691`, rekey-int 3826e38 `frozen.rs:165` to `:169`, `:429` | stale count and line numbers | CONFIRMED (text); FIXED. MERGE-WORK: 4,988 MEASURED now, 5,121 DERIVED, bound 5,400 at `:169`; 5,128 and 5,284 are earlier layouts |
