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

## Slices A and C (Formal-Lean-AC findings, 2026-11-15)

The full text of each finding is in `research/sherlock-e2e-spec-20261115/bus/formal-findings.md` (`FAL-*`). The Lean counterexamples are op lists for each context's oracle, pinned as named theorems in `<context>/lean/*/Tests.lean`. Where no code exists to build the counterexample on (the runner or tool is TO BUILD in the spec status), the verdict is OPEN DESIGN or CONFIRMED on spec text, and the table says so.

| Finding | IDs | Counterexample | Basis | Verdict |
|---|---|---|---|---|
| FAL-WRAP-1 | WRP-012, PVW-013 | four WRP-012 bits then deploy, refused (`Wrap.pin_deploy_four_bits`); a runner that checks only PVW-013 accepts the same deploy | spec text; no deploy runner exists | OPEN DESIGN (LEAD PICK: both lists must pass) |
| FAL-LEDGER-VERIFICATION-1 | MID-002, SLC-003, SLC-004, ELD-005 | the T17 runner asserts the wrap key on every entry | code `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:644-650` | CONFIRMED (MID-002 text predates SLC-004; narrow it to write entries) |
| FAL-SETTLEMENT-1 | LDG-001..005, LW-009, DCD-004, SLC-001 | in-tree LDG text has `descend` and `openClaim`, SLC-001 has four entries | spec text; t20-batched 5dc1be2ee holds the fixed text (LPC-010 merge) | OPEN DESIGN (oracle follows t20-batched, LEAD PICK) |
| FAL-SETTLEMENT-2 | ANC-012, BAT-005 | a 2,971-word plan needs 4 chunks at TTL 14,400 s, and the settlement oracle refuses it | spec text; the chunk planner is not built (`sherlock_plan.rs` has no cap) | CONFIRMED on spec text; OPEN DESIGN on code |
| FAL-SETTLEMENT-3 | BAT-010, BAT-011, ERT-004 | a run that lands at time 1,000 when the era ends at 1,000 | spec text; the settle runner is TO BUILD | OPEN DESIGN |
| FAL-NETOPS-1 | PVW-006, FRC-006 | controls are enabled on inclusion, before the readback | code `t17.sh:102` asserts the readback first | GUARD HOLDS |
| FAL-NETOPS-2 | FRC-003, FRC-005, FRC-013 | a call that timed out and then resolved as not included | code `t10a-ledger-run.mjs:686-689` | OPEN DESIGN (LEAD PICK: reuse) |
| FAL-NETOPS-3 | FRC-004, OBS-008 | a non-transport balancing error | code `t10a-ledger-run.mjs:764-775` stops on it | OPEN DESIGN (oracle follows the code) |
| FAL-NETOPS-4 | FRC-013, FRC-005 | an included, unindexed call is marked refused (`NetworkOps.pin_resolve`) | code `t10a-ledger-run.mjs:686-689` | CONFIRMED (code gap, FRC-013 TO BUILD) |
| FAL-COMPACT-COMPILATION-1 | CF-001, ECC-001 | `cf001_ecc001_conflict` | spec text | OPEN DESIGN |
| FAL-COMPACT-COMPILATION-2 | ECC-010, ECS-006, ECS-011, ECS-013 | REPORT.md:9 says 139/6; SUITE-COVERAGE.md:5 says 142/3 | files of the mycroft-factory worktree | CONFIRMED (regenerate REPORT.md) |
| FAL-COMPACT-COMPILATION-3 | ECC-014, ECC-007 | Jubjub point argument and keccak calls are BLOCKED | `compiler/out/suite-coverage.json` | CONFIRMED (code gap) |
| FAL-COMPACT-COMPILATION-4 | ECC-010 | ops `[1000010000, 2000000000]` with deploy=1 give refusal 1, and a seal-first order gives 5 | spec text | OPEN DESIGN (LEAD PICK: pipeline order) |
| FAL-CONFORMANCE-1 | ECS-004, ECS-008, ECS-010, ECS-014 | config=1, ops `[1000000001, 2000000001, 3000000001, 4000000000, 6000000000, 7000000003, 8000000000]` give cls 1 with 0 Preview tx | oracle; `ECS_014_complete_needs_evidence` blocks the report | OPEN DESIGN (class); GUARD HOLDS (completion) |
| FAL-EXEC-1 | MYC-KEY-003, MYC-KEY-008 | `MYC_KEY_003_prog_d_ignores_ukey` | spec text | OPEN DESIGN (owner question for T15) |
| FAL-EXEC-2 | MYC-REF-052, ERS-015, CF-005 | MYC-REF-052 refuses the per-program key that the code builds | code `rekey-chips` dcf1b4c `rsl/mod.rs:12-13` (odin tree `formal-cx-rekey-chips-dcf1b4c`) | CONFIRMED (MYC text stale) |
| FAL-EXEC-3 | MYC-CHIP-001, EKT-002 | the chip set has two keccak AIRs, against "at most 9 in all" | code `rsl/trace.rs:133`, `:432-433` | CONFIRMED (wording; the per-program bucket holds) |
| FAL-EXEC-4 | ERS-009, EUI-004, GP-001, CPR-005 | an R_SL Rejected outcome has no proof route | spec text | OPEN DESIGN (LEAD PICK: option (a), ERS-009 scoped to the settlement route) |
| FAL-EXEC-5 | EUI-010, RFB-003 | `EUI_010_rule_gap`: 7 < 120 | code `dec.rs:23-29`, `:40-41` | CONFIRMED (code gap, TO BUILD) |
| FAL-GOVERNANCE-1 | EGV-002, ERT-002, ELD-002 | op `[9000010200]` | spec text; no deploy tool | CONFIRMED on spec text (low) |
| FAL-GOVERNANCE-2 | ERT-004, ERT-005 | `end_eff` with P = e (`pin_handover`, `pin_before_handover`) | spec text | CONFIRMED on spec text (low; LEAD PICK LP-1) |
| FAL-GOVERNANCE-3 | ELD-006 | 1,024 against "below 1,026" | spec text; `measured_ELD_006` | CONFIRMED on spec text (low) |

The table above records the verdicts on the day of the finding. The resolution below supersedes the OPEN DESIGN and CONFIRMED verdicts.

### Resolution (A/C resolver, 2026-11-15 12:55)

All 22 FAL-* findings have a verdict. Spec text is fixed in this branch unless a commit is named. Code gaps have fix requests in `~/.cache/lead/fal-fix-<ID>.md`. LEAD PICKs are marked for T15.
- Settlement. FAL-SETTLEMENT-1: FIXED. The t20-batched text of LDG-001..005, DCD-004 and LW-009 is now in this branch (no `descend`, no `openClaim`, four entries as SLC-001). FAL-SETTLEMENT-2: FIXED. ANC-012 is in processor-final with `K_MAX = min(floor(TTL / (10 x 175.5 s)), 2)` (BAT-005); the planner cap is a fix request. FAL-SETTLEMENT-3: FIXED (LEAD PICK). BAT-010 closes at `end_eff` minus proof time, inclusion time and one block interval; BAT-011 refuses a run that ends at or after `end_eff`.
- Ledger and wrap. FAL-LEDGER-VERIFICATION-1: FIXED. MID-002 puts the universal wrap key on the write entries only; the read entries keep native keys (SLC-004). The runner key check is a fix request. FAL-WRAP-1: FIXED (LEAD PICK). PVW-013 also requires the 5 WRP-012 checks, in one gate record.
- Network operations. FAL-NETOPS-1: GUARD HOLDS (`t17.sh:102`); PVW-006 now triggers on the readback. FAL-NETOPS-2: FIXED (LEAD PICK reuse, FRC-003). FAL-NETOPS-3: FIXED, new FRC-016 (other balancing errors stop the call). FAL-NETOPS-4: CONFIRMED code gap, fix request (`t10a-ledger-run.mjs:722`).
- Compact compilation and conformance. FAL-COMPACT-COMPILATION-1: FIXED, CF-001 applies where the build is not the e2e prototype (D-016). -2: FIXED in the text, ECC-010 cites one source (mycroft-factory 5b2f6047d `SUITE-COVERAGE.md:5`: 145 equal, 62 refused, 1 pure, 0 blocked); stale `factory/REPORT.md:9` is a fix request. -3: ECC-014 MET in code by Mycroft T24 5b2f6047d; ECC-007 PARTIAL (keccak equal to native through the tier 3 kit, `gir.rs:89`), fix request. -4: FIXED (LEAD PICK), refusal order read, IR version, input types, lowering, seal. FAL-CONFORMANCE-1: FIXED, new ECS-015 (a dry settle is blocked, reason `preview off`).
- Execution. FAL-EXEC-1: FIXED (LEAD PICK), MYC-KEY-008 keeps `prog_d` as MYC-KEY-003. FAL-EXEC-2 and -3: FIXED, MYC-REF-052 and MYC-CHIP-001 match rekey-int 95cf796. These three are in mycroft-factory cb65c605d. FAL-EXEC-4: FIXED (LEAD PICK (a)), ERS-009 applies on the T23 settlement route. FAL-EXEC-5: CONFIRMED code gap (7 rules, `dec.rs:23-29`), fix request.
- Governance. FAL-GOVERNANCE-1 to -3: FIXED in t22-governance cf794dc97: ERT-002 `threshold == 1` (LP-2), ERT-004 `end_eff` for `P != e` and an unset end unbounded (LP-1), ELD-006 scenario bound `L_TX` = 1,024. EGV-002 status updated.
- Regression: the Lean pins are the named replay tests (`Settlement.replay_bat010_boundary`, `replay_bat011_boundary`, `anc012_cap2`, `MID_002_read_entries_native`, `pin_era_close`; `Conformance.pin_preview_off`; `NetworkOps.pin_other_error`). They fail on the old reading and pass on the new one; odin `lake build --wfail` rc 0. COVERAGE-FORMAL-AC.md has 0 FINDING rows. COVERAGE-FORMAL.md merges B and AC, one row per ID (745 rows, 1 FINDING: LFP-003, text fix open with F-leaf-proving-Q1); the B row CPR-005 is SPEC-CHECKED by D-024.

