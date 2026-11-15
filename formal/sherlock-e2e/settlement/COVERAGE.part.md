| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| SLC-001 | C | settlement | MODELED | `Inv.deployShape`, `SLC_001_004_deploy_shape`, `Consistency.SLC_001_ok`, `SLC_001_matches_oracle`, pin `pin_deploy` | pending (Formal-Quint) | t20-batched form (LEAD PICK: the adopted LDG text as amended on t20-batched, see FAL-SETTLEMENT-1) |
| SLC-002 | C | settlement | MODELED | `SLC_002_anchor_slot`, `SLC_002_anchor_constructor_only` | pending (Formal-Quint) | the compiler refusal of an entry that writes slot 15 is slice A |
| SLC-003 | C | settlement | MODELED | `Inv.deployShape` (keyRun = keyColl = wrap key) | pending (Formal-Quint) | see ORACLE.md |
| SLC-004 | C | settlement | MODELED | `Inv.deployShape` (keyRoot = keyRunRead = native key) | pending (Formal-Quint) | see ORACLE.md |
| SLC-005 | C | settlement | MODELED | `SLC_005_other_program`, pin `pin_other_program` | pending (Formal-Quint) | node custom error 115 = refusal bit 4 |
| SLC-006 | C | settlement | SPEC-CHECKED | `Consistency.SLC_006_ok`; `buildArgs` op writes the file | n/a | a data form (4 words, acceptRun takes the first 3); no state beyond the argument file |
| SLC-007 | C | settlement | SPEC-CHECKED | `Consistency.SLC_007_ok`, `SLC_007_changed_effect` | n/a | a data form of the outcome |
| SLC-008 | C | settlement | SPEC-CHECKED | `Consistency.SLC_008_ok` | n/a | a data form of the input (fuel 0, stack 0, 8-word salt); consistent with ZK-006 |
| SLC-009 | C | settlement | MODELED | `Inv.argsUsed`, pin `pin_mismatch`, `pin_no_args` | pending (Formal-Quint) | see ORACLE.md |
| SLC-010 | C | settlement | MODELED | `SLC_010_native_refused`, pin `pin_native` | pending (Formal-Quint) | see ORACLE.md |
| SLC-011 | C | settlement | MODELED | `apply` replayStart branch, pin `pin_reuse` | pending (Formal-Quint) | see ORACLE.md |
| SLC-012 | C | settlement | MODELED | `SLC_012_unbound_refused`, pin `pin_unbound` | pending (Formal-Quint) | see ORACLE.md |
| BAT-001 | C | settlement | MODELED | `Inv.epochBounds`, `Inv.onePerEpoch`, `LDG_006_one_write_per_epoch`, pins `pin_epoch`, `pin_no_overrun`, `pin_no_early_close` | pending (Formal-Quint) | LEAD PICK: the clock (tick) cannot pass an open epoch's close time; close is enabled at the close time or at 1,048,576 claims |
| BAT-002 | C | settlement | MODELED | `BAT_002_operator_no_acceptRun`, pin `pin_operator` | pending (Formal-Quint) | the contract cannot tell an operator (spec status); the runner routes |
| BAT-003 | C | settlement | CONSTANT-CHECKED | `bat003_rates`, `Consistency.BAT_003_ok` | n/a | 492 calls/day, 24 tx/day, 163,128 B and 3,337,236 B, 291 jobs/s (t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:11,13,14,54,56); MEASURED 6,783 B, ESTIMATE 6,797 B |
| BAT-004 | C | settlement | CONSTANT-CHECKED | `bat004_counts` | n/a | 208 - 62 - 1 = 145 acceptRun, at most 106 deploys (/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md:45) |
| BAT-005 | C | settlement | MODELED | `planRefuses`, `BAT_005_cap`, `Inv.planCap`, `bat005_pins`, `kmax_*`, pins `pin_plan`, `pin_short_ttl` | pending (Formal-Quint) | LEAD PICK: K_MAX = min(ANC-012 formula, 2) |
| BAT-006 | C | settlement | MODELED | `BAT_006_default_ttl`, pin `pin_ttl` | pending (Formal-Quint) | see ORACLE.md |
| BAT-007 | C | settlement | MODELED | `Inv.membership` (pathsGiven = jobsClosed), pin `pin_epoch` | pending (Formal-Quint) | see ORACLE.md |
| BAT-008 | C | settlement | MODELED | `Inv.membership` (leafBytes = 32 x jobsClosed), pin `pin_epoch` | pending (Formal-Quint) | see ORACLE.md |
| BAT-009 | C | settlement | MODELED | `Inv.formFirst`, pin `pin_no_form` | pending (Formal-Quint) | see ORACLE.md |
| BAT-010 | C | settlement | FINDING | `deadline`, `BAT_010_close_before_era`, pin `pin_era_close`, `Consistency.finding_bat010_boundary` | n/a (finding) | FAL-SETTLEMENT-3: closing at end_eff - proof - inclusion lands at end_eff, where ERT-004 accepts nothing |
| BAT-011 | C | settlement | FINDING | `BAT_011_era_refusal`, pin `pin_era_run`, `Consistency.finding_bat011_boundary` | n/a (finding) | FAL-SETTLEMENT-3: a run ending exactly at end_eff is admitted (refused only after) but ERT-004 is not active at end_eff |
| BAT-012 | C | settlement | MODELED | `Inv.freezeOpen`, pin `pin_freeze_open` | pending (Formal-Quint) | see ORACLE.md |
| STL-001 | completion | settlement | SPEC-CHECKED | `Consistency.STL_001_ok` | n/a | a derivation of the 4 public words; same form as SLC-006 |
| STL-002 | completion | settlement | SPEC-CHECKED | `Consistency.STL_002_ok` | n/a | the prove chain app, leaf, N1, N2, N3, L, L_Fr, wrap at k = 21; proving is slice B |
| STL-003 | completion | settlement | MODELED | `Inv.deployShape`, `SLC_002_anchor_slot` | pending (Formal-Quint) | the trigger (rekey and T12 pass) is outside this context |
| STL-004 | completion | settlement | MODELED | `STL_004_runs_set`, pin `pin_settle` | pending (Formal-Quint) | see ORACLE.md |
| STL-005 | completion | settlement | MODELED | `Inv.mismatchSettled`, pin `pin_stl005` | pending (Formal-Quint) | see ORACLE.md |
| STL-006 | completion | settlement | SPEC-CHECKED | `Consistency.STL_006_ok` | n/a | 33 of 36 programs (LOOP.md:45); a run plan, no transition |
| ANC-012 | reused | settlement | FINDING | `chunks`, `kMax`, `planRefuses`, `kmax_default_ttl`, `measured_largest_calls`, `anc012_call_words`, `Consistency.finding_anc012_formula` | n/a (finding) | FAL-SETTLEMENT-2: the formula gives K_MAX 8 at 14,400 s, BAT-005 caps 2; constants 1,015 = 1,024 - 9, 474 and 699 <= 1,015 (t20-batched@5dc1be2ee:...RESULTS.md:116), floor(3,600/1,755) = 2 (RESULTS.md:122) |
| CLM-001 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_001_ok` | n/a | node circuit rule (slice B Aggregation owns the AIRs); kinds and domain words distinct |
| CLM-002 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_002_ok` | n/a | node circuit rule; no SEG or JOIN consumed |
| CLM-003 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_003_ok` | n/a | node circuit rule; openings of JOIN or DEP only (in circuit, LDG-004 t20) |
| CLM-004 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_004_ok` | n/a | deps chain keeps order and repetition |
| CLM-005 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_005_ok` | n/a | Rejected/Exhausted only as chain elements |
| CLM-006 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_006_ok` | n/a | build option AD-20 (a); a plan choice |
| CLM-007 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_007_ok` | n/a | DL fields and chain heads LEAF, DEP, FOREIGN |
| CLM-008 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_008_ok`, `SG_ok` | n/a | SG gate constants (2^19, 1,000, 200, 64, 3,000) at the bound; no measured value read here |
| CLM-009 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_009_ok` | n/a | a test obligation (refusal twins) |
| CRR-007 | reused | settlement | SPEC-CHECKED | `Consistency.CRR_007_ok` | n/a | consumer writes only its nullifier; consistent with LDG-004 |
| ELD-005 | reused | settlement | SPEC-CHECKED | `Consistency.ELD_005_ok`; native read keys in `Inv.deployShape` | n/a | read entries keep native keys and never write |
| ERT-004 | reused | settlement | SPEC-CHECKED | `Consistency.ERT_004_ok`, `ERT_004_consistent`, `active`, `endEff`, `finding_bat011_boundary`, `finding_bat010_boundary` | n/a | era activity is the governance context's model; here the half-open interval shows FAL-SETTLEMENT-3 against BAT-010/BAT-011 |
| ERT-009 | reused | settlement | SPEC-CHECKED | `Consistency.ERT_009_ok` | n/a | GRID rule of the governance era table; no transition in this context |
| GOV-006 | reused | settlement | SPEC-CHECKED | `Consistency.GOV_006_ok`, `GOV_006_consistent`; `eraEnd` cell (announceEra) feeds BAT-010/011 | n/a | the soundness_end verb is the governance context's; here only its end_eff is consumed |
| FREG-001 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_001_ok` | n/a | registry entry form (9 fields, pi_count < 2^32) |
| FREG-002 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_002_ok` | n/a | fclaim domain disjoint from SHK_RUN |
| FREG-003 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_003_ok` | n/a | parsing refuses trailing data |
| FREG-004 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_004_ok` | n/a | no accumulator |
| FREG-005 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_005_ok`, `SG_ok` | n/a | SG gate constants |
| LDG-001 | reused | settlement | FINDING | `Consistency.LDG_001_ok` (t20 text); `Inv.deployShape` | n/a (finding) | FAL-SETTLEMENT-1: this branch holds the pre-T20 text (foreign map, verified_roots) that contradicts SLC-001; the oracle models the t20-batched text (LEAD PICK) |
| LDG-002 | reused | settlement | MODELED | `LDG_002_at_most_once`, `BAT_002_operator_no_acceptRun`, `Consistency.LDG_002_args_ok`, pins `pin_replay`, `pin_both_node` | pending (Formal-Quint) | see ORACLE.md |
| LDG-003 | reused | settlement | FINDING | `closeStep`, `collBad`, `Inv.rootsDistinct`, pin `pin_dup_root` | n/a (finding) | FAL-SETTLEMENT-1: in-tree text takes (reg_root, set_root, shape_d); the t20 text (modeled) takes (reg_pub, claims_root) |
| LDG-004 | reused | settlement | FINDING | `Consistency.LDG_004_ok` | n/a (finding) | FAL-SETTLEMENT-1: in-tree text has a descend entry; SLC-001 and the t20 text have none |
| LDG-005 | reused | settlement | FINDING | `Consistency.LDG_005_ok`, `epoch_cap_depth20` | n/a (finding) | FAL-SETTLEMENT-1: in-tree text is an openClaim entry; SLC-001 and the t20 text (claims tree depth 20, modeled) have none |
| LDG-006 | reused | settlement | MODELED | `Inv.onePerEpoch`, `LDG_006_one_write_per_epoch`, pins `pin_epoch`, `pin_empty_epoch` | pending (Formal-Quint) | see ORACLE.md |
| LDG-007 | reused | settlement | SPEC-CHECKED | `Consistency.LDG_007_ok` | n/a | no foreign map; run and foreign leaf tags differ |
| LDG-008 | reused | settlement | MODELED | `collBad` (registry bit 64), pin `pin_registry`, `Consistency.LDG_008_once` | pending (Formal-Quint) | registry epochs are not a transition here: `regWrites` shows one write per changed epoch |
| LW-009 | reused | settlement | FINDING | `Consistency.LW_009_ok` | n/a (finding) | FAL-SETTLEMENT-1: in-tree LW-009 publishes [reg_root, set_root]; the t20 text (modeled) publishes [reg_pub, claims_root] |
| ZK-006 | reused | settlement | SPEC-CHECKED | `Consistency.ZK_006_ok` | n/a | salt form; the CSPRNG draw is slice B (ZKE) |
| ZK-011 | reused | settlement | SPEC-CHECKED | `Consistency.ZK_011_ok` | n/a | public digest forms, distinct tags |
