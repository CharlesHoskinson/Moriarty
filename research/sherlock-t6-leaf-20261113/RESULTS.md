# T6 results: the modular leaf, vertical slice (group V)

Row T6 of `research/sherlock-tested-design-20261113/LOOP.md`. Change `openspec/changes/sherlock-modular-leaf-20261111/` (`ML/` below), tasks V.1 to V.10. Trees, all on branch `t6-leaf`: Moriarty `~/Projects/Moriarty-wt/t6-leaf` (base `2de61edb8`), openvm `~/.cache/m7-wrap/src/openvm-t6` (base `b3fbaa5`). State, logs and outputs under `~/.cache/t6/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead: scope change found by V.1

No Sherlock program meets the V.1 condition "reads no ROM row beyond what INIT opens". Every run starts at `Eval { expr: e0 }`, and every rule at an Eval control reads the expression row `expressions[e0]` (`owner_site`, T5 section 3.3 atom `E_TAG`). INIT opens `prog_d` to `image_d` to `prog_root` and the entry index `entry_a`, not the row (`ML/specs/sherlock-init-admission` INIT-007, FP-4). So one ROM row read, with its ROM line under `prog_root` (FP-9.2, row I-6), enters the slice. The V.1 program needs no stack step and no flush, so STACKS and MBK-003 stay out. `ML/tasks.md` V.1 and `ML/design.md` section 8 are amended to say so.

## 1. Status

| Task | Status | Receipt |
|---|---|---|
| V.1 | done | `qa/sherlock/traces/20261114T005955Z-V.1.log` PASS |
| V.2 to V.8 | gates pass on real proofs (section 3) | see section 3 |
| V.9, V.10 | running | |

## 2. V.1 the slice program

Probe: test `slice_v1_candidates` (`crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime/rule_maxima_tests/slice_v1.rs`), on the landed reference owner machine (`owner_step`, `HostFamilyPlans`, `OwnerGuards::new_owner`), with the `rule_maxima` harness bounds. With `SLICE_OUT=<dir>` it writes `slice_v1.json` (every step: rule, disposition, appends per kind, frame and task operations, heap reads, ROM rows from the X15 probe, and the image lanes before and after: root and frames, helper record, result record, tasks).

Candidates (MEASURED, `~/.cache/t6/out/slice_v1.json`):

| Program | Steps | Rules | Stack steps | Appends | ROM rows read | Outcome |
|---|---:|---|---|---:|---|---|
| `entry 7;` (and `entry true;`, `entry nil;`) | 6 | ValidateEntry, ValidateOccurrence, ValidateEmpty, ValidateExit, EvalLiteralValid, PublishValue | one task push, one task pop | 0 | `expressions[0]`, `constants[0]` | Halted |
| `type U { A(); } entry construct U::A();` | 2 | XAdtEvalConstructNullary, PublishValue | none | 0 | `expressions[0]`, `constants[0]`, `types[4]`, `constructors[4]` | Halted |
| `entry x;` | 5 | EnvGetEntry, EnvGetEmpty, EnvGetFailure, EvalVariableAbsent, PublishError | none | 0 | `expressions[0]` | Rejected code 3 |
| `entry 1 - 2;`, overflow | 20 | 15 distinct | frame push and pop, task push and pop | 0 | several | Rejected code 10 |

**The slice program is `entry x;`** (source `language "moriarty-sherlock-nominal/1"; program P { entry x; }`, fuel 4096, stack 64): no stack step, no record, one ROM row in one table, no value digest in the outcome (the Rejected payload is `[code, 0, ...]`, FP-4). The constructor program reads four ROM rows and needs `vd` of a constant for the Halted header payload, so it needs more of PAGE/ROM and VALUE DIGEST for fewer steps.

Rule list, with the DEC class (the priority ordinal of T5 section 3.4, FP-9.3) and the guard:

| Step | Rule | Class | Guard (T5 3.4) |
|---:|---|---:|---|
| 0 | `BHelperEnvGetEntry` | 89 | PRE AND helper = none AND result = none AND site = EvalVar |
| 1 | `BHelperEnvGetEmpty` | 12 | PRE AND helper = EnvGet AND status = Pending AND cursor absent |
| 2 | `BHelperEnvGetFailure` | 10 | PRE AND helper = EnvGet AND status = Negative |
| 3 | `KMachineEvalVariableAbsent` | 67 | PRE AND helper = none AND result present AND site = EvalVar AND result = EnvMissing |
| 4 | `KAdminPublishError` | 1 | outcome = Running AND helper = none AND result = none AND ctl = Error |

DEC rows of the subset: the idle row 0 and the classes 1, 10, 12, 67, 89 (6 rows). Writes per step (MEASURED lane diffs of the reference run): every step takes 1 from `remaining_work` and adds 1 to `consumed_work` (steps 0 to 3); the helper steps add 1 to `helper_work`; the machine step adds 1 to `reference_cursor`; step 0 sets helper kind 2 (EnvGet) and site 1; step 1 sets the helper status lane 11 to 2; step 2 clears the helper and sets result kind 3 (EnvMissing); step 3 clears the result and sets the control to `Error { code: 3 }` (control lanes 0, 1 = 9, 3); step 4 (publication, no fuel charge) sets the outcome to Rejected 3 (lanes 23, 24).

ROM tables of the program (MEASURED): `expressions` 1, `occurrences` 1, `types` 4, `constructors` 4, `ctor_fields` 5, `names` 1, every other table empty.

## 3. V.2 to V.8 the slice kits

Code: crate `crates/sherlock-leaf` in openvm `t6-leaf` (commits `810b370`, `c3bea58`), stark-backend `t6-leaf` commit `deeeb0a` (`SB_MUTANT_PROVER`, test only: the prover finishes a proof whose LogUp root sum is not zero, so the stock verifier, not the prover, refuses it). Patch series: `research/sherlock-stark-code-20261112/openvm-t6/` and `stark-backend-t6/`. Gate runner: `scripts/qa/check-slice.sh vN` (example `slice` of `sherlock-leaf`, one JSON record per mode under `~/.cache/t6/out/`).

Rule of every gate (SG-2, SG-3), from `crates/sherlock-leaf/src/harness.rs`: a positive is a real proof that the stock verifier accepts and the debug checker passes. A mutant passes only when the debug checker refuses it, the prover with `SB_MUTANT_PROVER` makes a proof, the stock verifier refuses that proof, and the same witness under the weakened control AIR set (one constraint or bus removed, named by a `weak` flag) proves and verifies. A prover error or panic earns no credit. Every AIR set is keyed with every AIR required (SG-1) and checked inside the frozen app set (SG-4).

Slice leaf shard shape (MEASURED, `v5.json` `shape`): 7 AIRs (CORE, DEC, RANGE, SPONGE, HASH2, ROM, MEM), 2,361 columns, at most 1,476 constraints (CORE), at most 145 interactions, degree 3, 54 public values; heights 8, 8, 131,072, 512, 128, 2, 8 (fixed for every shard, `leaf::HEIGHTS`). The run has two shards of S = 4 slots: shard 0 the steps 0 to 3, shard 1 step 4 and three disabled slots.

| Task | Case | Kind | Stock verifier on the mutant proof (MEASURED) | Weak control |
|---|---|---|---|---|
| V.2 | `bus_enc_vectors` | host | registry and lane vectors round trip | |
| V.2 | `bus_reserved_id_registry` | keygen | `check_registry` refuses id 7 | |
| V.2 | `bus_unassigned_chip` | mutant | GKR zero check fails | accepts |
| V.2 | `bus_limb_over_range` | mutant | GKR zero check fails | accepts |
| V.2 | `bus_port_received_twice` | mutant | GKR zero check fails | accepts |
| V.3 | `range_64_boundaries`, `range_128_boundaries` | positive | accepts | |
| V.3 | `range_limb_out_of_table` | mutant | GKR zero check fails | accepts |
| V.3 | `range_source_copy_removed` | mutant | inconsistent claims | accepts |
| V.4 | `digest_table_vectors` | host and AIR | FP-4 vectors equal the Rust reference | |
| V.4 | `hash_domain_swapped`, `hash_round_skipped` | mutant | sum claim mismatch | accepts |
| V.5 | `core_rules_shard0_entry_empty_failure_absent`, `core_rules_shard1_publish_error`, `core_height_edge` | positive | accepts | |
| V.5 | `core_rules_d_reference_vectors` | host and AIR | equal | |
| V.5 | `core_enable_nonprefix`, `core_rules_d_forged`, `dec_two_rules_selected` | mutant | sum claim mismatch | accepts |
| V.6 | `mem_window_three_appends`, `mem_window_no_append` | positive | accepts | |
| V.6 | `address_reuse_window_row`, `close_advances_len`, `counter_wrap_2_64` | mutant | sum claim mismatch | accepts |
| V.6 | `forged_tail_across_leaf_boundary` | mutant | GKR zero check fails | accepts |
| V.7 | `ok_boundary_poseidon_two_leaf_stitch` | positive | both shards accept, `post_d` of shard 0 = `pre_d` of shard 1 | |
| V.7 | `boundary_word_omitted` | mutant | GKR zero check fails | accepts |
| V.7 | `header_body_d_unlinked` | mutant | sum claim mismatch | accepts |
| V.8 | `init_expr_entry_no_input` | positive | accepts; the reference genesis state equals `init_state` of the admission | |
| V.8 | `init_non_genesis_leaf_any_state` | positive | accepts | |
| V.8 | `init_wrong_control`, `init_memory_not_genesis` | mutant | sum claim mismatch | accepts |

V.5 "one positive per branch of each subset rule": each of the five subset rules has one branch in the V.1 run, and the two shard proofs cover all five.

Fix found by V.6 (MEASURED): MemAir sent the MEM bus message from the `addr` columns through the carry columns, so a window with appends did not balance (`crates/sherlock-leaf/src/mem.rs`, MEM bus send). The message is now `kind, addr, rec`. The slice run has no append, so V.2 to V.5, V.7 and V.8 did not see it.
