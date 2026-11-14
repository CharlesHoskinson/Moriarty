# T6 results: the modular leaf, vertical slice (group V)

Row T6 of `research/sherlock-tested-design-20261113/LOOP.md`. Change `openspec/changes/sherlock-modular-leaf-20261111/` (`ML/` below), tasks V.1 to V.10. Trees, all on branch `t6-leaf`: Moriarty `~/Projects/Moriarty-wt/t6-leaf` (base `2de61edb8`), openvm `~/.cache/m7-wrap/src/openvm-t6` (base `b3fbaa5`). State, logs and outputs under `~/.cache/t6/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead: scope change found by V.1

No Sherlock program meets the V.1 condition "reads no ROM row beyond what INIT opens". Every run starts at `Eval { expr: e0 }`, and every rule at an Eval control reads the expression row `expressions[e0]` (`owner_site`, T5 section 3.3 atom `E_TAG`). INIT opens `prog_d` to `image_d` to `prog_root` and the entry index `entry_a`, not the row (`ML/specs/sherlock-init-admission` INIT-007, FP-4). So one ROM row read, with its ROM line under `prog_root` (FP-9.2, row I-6), enters the slice. The V.1 program needs no stack step and no flush, so STACKS and MBK-003 stay out. `ML/tasks.md` V.1 and `ML/design.md` section 8 are amended to say so.

## 1. Status

| Task | Status | Receipt |
|---|---|---|
| V.1 | done | `qa/sherlock/traces/20261114T005955Z-V.1.log` PASS |
| V.2 | done | `qa/sherlock/traces/20261114T020120Z-V.2.log` PASS |
| V.3 | done | `qa/sherlock/traces/20261114T020315Z-V.3.log` PASS |
| V.4 | done | `qa/sherlock/traces/20261114T020353Z-V.4.log` PASS |
| V.5 | done | `qa/sherlock/traces/20261114T020432Z-V.5.log` PASS |
| V.6 | done | `qa/sherlock/traces/20261114T020544Z-V.6.log` PASS |
| V.7 | done | `qa/sherlock/traces/20261114T020634Z-V.7.log` PASS |
| V.8 | done | `qa/sherlock/traces/20261114T020703Z-V.8.log` PASS |
| V.9 | gate passes (section 4) | pending |
| V.10 | gate passes (section 5) | pending |

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

Receipts: an incident voided the first V.2 to V.8 receipts. The 18:34 reboot left `check-slice.sh` and `check-v2.sh` to `check-v8.sh` at 0 bytes, and commit `afdc52209` holds them empty, so `doctorwatson check` ran an empty script (exit 0 in 3 ms, empty traces `20261114T015032Z` to `015036Z`). Those receipts do not count. Commit `6c9513f7d` restores the scripts, and the receipts below come from the rerun.

## 4. V.9 the slice leaf shard in place of W1

Example `crates/sherlock-wrap-export/examples/slice_chain.rs` (openvm `6ec1364`), mode `omit`; gate `scripts/qa/check-v9.sh`. The app is the slice leaf shard, keyed with `sherlock_leaf::harness::key` (every AIR required, SG-1; inside the frozen app set, SG-4). The leaf layer is `VerifierSubCircuit<1>` with `KeyBind` (Leaf), and `KeyBind` receives the 54 slice public values (circuit change, section 7). N1 runs over the two leaf proofs, then N2, N3 and `L` as in T3.

Per-AIR shape (MEASURED, `~/.cache/t6/v9/omit.json` `app_shape`):

| AIR | Columns | Constraints | Degree | Interactions | Height |
|---|---:|---:|---:|---:|---:|
| CORE | 676 | 1,476 | 3 | 145 | 8 |
| DEC | 9 | 18 | 2 | 1 | 8 |
| RANGE (stock `VariableRangeCheckerAir`) | 4 | 10 | 3 | 1 | 131,072 |
| SPONGE | 321 | 326 | 3 | 2 | 512 |
| HASH2 | 29 | 5 | 2 | 5 | 128 |
| ROM | 1,222 | 78 | 2 | 77 | 2 |
| MEM | 100 | 100 | 3 | 21 | 8 |

Keys of the layers (MEASURED, `~/.cache/t6/v10/chain.json` `keys`): app 7 AIRs, 2,361 columns, vk 70,560 bytes; leaf layer 40 AIRs, 1,675 columns, 81,234 bytes; N1, N2, N3 40 AIRs, 1,637 columns, at most 84 interactions; N3 over N3 has the N3 key (fixed point); `L_iface` 44 AIRs, 1,893 columns; `L_Fr` 15 AIRs, 561 columns, vk 45,600 bytes. Every AIR is required in app, leaf, N1, N2, N3 and `L` (MEASURED, `omit.json` `all_required`).

`slice_air_omitted` and the T3 omit cases on the slice keys: 57 cases, all PASS (MEASURED, `~/.cache/t6/v9/omit.json`):
- Each of the 7 slice AIRs left out of the app proof is refused three ways. The prover stops with a nonzero LogUp root, the stock app verifier refuses the stripped real proof (`RequiredAirNoVData`), and the leaf layer cannot generate its trace over the stripped proof (index out of bounds in the recursion trace generator), so no leaf proof and nothing above it exists. No N1, N3 or `L` proof can carry the omission. The circuit-level refusals at those layers are the T3 cases that follow.
- N1 and N3, each without `KeyBindAir`, `ProofShapeAir` or `PublicValuesAir`, are refused on the prover path and by the stock verifier, and the parent over the stripped child fails trace generation. `L` without each of the three is refused on the prover path and by the stock verifier, and the `L_Fr` tape generator refuses the stripped shape (`RequiredAbsent`) and the pinned honest shape (`Heights`).
- Honest controls accept: app, leaf, N2 over N1, N3 over N3, `L`, `L_Fr` tape.

SG-6 seconds per proved step: 1.79 s on CPU (MEASURED, `--exclusive`, V.10 run: app proofs 4.66 s and 4.32 s for the two shards, 5 proved steps over 8 slots, 1.12 s per slot). GPU gap: the E4 GPU path in this tree is the fractional-sumcheck hook of the Fr config, not a full BabyBear prover. No GPU number exists for the slice. A GPU figure needs a full GPU engine for the app set (the `cuda` backend of stark-backend), which is outside group V.

## 5. V.10 one run through every layer

Run (MEASURED, `--exclusive`, `~/.cache/t6/v10/chain.json`, log `~/.cache/t6/logs/v10-chain.log`, openvm `6ec1364`): the V.1 program `entry x;` as 2 shards (4 and 1 proved steps), with `post_d` of shard 0 equal to `pre_d` of shard 1. Two app proofs, two leaf proofs, N1 over both, N2, N3, `L_iface` over N3, `L_Fr`: each layer is proved and accepted by its stock verifier, and every layer's heights pass G1 (`frozen::check_heights`). Wall time 375 s. Per layer: leaf 5.7 s for 2, N1 6.7 s, N2 4.8 s, N3 3.6 s, `L` 65.2 s, `L_Fr` keygen 57.9 s, `L_Fr` prove 193.7 s.

Fixed heights: both shards have log heights `[3, 3, 17, 9, 7, 1, 3]`, equal to `leaf::HEIGHTS` (MEASURED).

`key_root` = `key_root([key_triple(0, app.pre, leaf.pre, n1.pre)])` over this run's keys. It enters `prog_d` in the CORE public values (FP-8 form).

D3 (MEASURED, `fp8_d3 ~/.cache/t6/v10`, `~/.cache/t6/v10/d3.json`): every term is at least 100 bits and every grindable term at least 101 bits, at the measured shape, the vk bounds and the frozen-set bounds, for all 7 keys. Weakest term at the measured shape:

| Key | Max log height | Stacked columns | Weakest | mu | query | logup | fold |
|---|---:|---:|---|---:|---:|---:|---:|
| app | 17 | 1 (2 used) | shift 101.09 | 113.63 | 101.09 | 102.72 | 104.63 |
| leaf<1> | 17 | 10 | shift 101.01 | 110.46 | 101.01 | 101.72 | 105.63 |
| N1 | 19 | 98 | shift 101.16 | 104.03 | 101.16 | 102.72 | 102.63 |
| N2 | 17 | 72 | shift 101.16 | 104.48 | 101.16 | 102.72 | 102.63 |
| N3 | 17 | 48 | shift 101.16 | 105.07 | 101.16 | 102.72 | 102.63 |
| `L_iface` | 16 | 18 | shift 101.14 | 102.54 | 101.14 | 102.72 | 101.63 |
| `L_Fr` | 19 | 43 | query 101.14 | 226.46 | 101.14 | 215.95 | 232.86 |

The app proof is one stacked column, and the stock `SoundnessCalculator` asserts at least 2 for the mu term. `fp8_d3` uses 2 for a one-column proof (openvm `c541730`). That value is conservative: a single column has no mu batching, and the mu error grows with the column count. N2 now has a measured row (it had only vk and bounds in FP-8).

What V.10 does not show: `L_iface` proves over N3 with the FP-3 interface statement (`Dims::FULL`, all-entries case, seed 14), not with the slice statement. The leaf layer receives the slice public values but does not expose them, and N1 to N3 carry no slice statement (FP-2 node statement, group 5). One proof chain therefore binds the keys of the run, not the slice outcome, up to `L_Fr`. N2 and N3 run over two copies of their child because the run has a single N1 (the T3 bench topology).

## 6. Remaining T6 items (outside group V)

| Item | What it needs |
|---|---|
| X10 in the leaf (I-12, I-35) | The X10 check as leaf AIRs over the slice state (rows I-12 and I-35 of T5), their mutants as SG-3 real proofs, and a slice program that reaches X10. The `entry x;` run does not. |
| Multi-segment `TAG_NODE` merge in `L` | An `L`-side AIR that folds the per-segment statements (`pre_d`, `post_d`, `h_pre`, steps) into one `TAG_NODE` digest and checks the `post_d` to `pre_d` stitch across segments. It also needs the node statement below (FP-2), because `L` today sees the FP-3 interface words only. |
| FP-2 node statement | The leaf `KeyBind` receives the 54 slice public values but does not expose them. Carrying the leaf statement through N1 to N3 and into `L` is group 5 (NODE INTEGRATION). Until then, the chain binds the keys of the run, not its outcome. |
| N2 and N3 arity with one N1 | The run has one N1, so N2 and N3 run over two copies of their child (the T3 bench topology). A run-shaped tree needs a padding or odd-arity rule for nodes (group 5). |
| SG-6 on GPU | A full GPU engine for the app set. Only the CPU figure exists (section 4). |
| DEC beyond the subset | Six DEC rows (idle and classes 1, 10, 12, 67, 89) in a subset table. The full priority network waits on T5 row I-7. |
| MEM near the top address | The honest MEM window checks `addr + i` without a wrap at 2^64 (`counter_wrap_2_64`). A window that starts within 8 of 2^64 has no honest witness. Real runs never get there. |

Slice PICKs (each marked `PICK` in the source, open for the owner or the kit tasks): `SHK_RULES` code 58 (`enc.rs:34`), body schema id 1 and the 476-word body (`enc.rs:37`), empty frame and task heads `P2(37)`, `P2(38)` (ENC-4 open, `enc.rs:308`), genesis memory root and tail `D_EMPTY` (`enc.rs:316`), `plan_d = P2(DOM_PLAN)` for a program with no plan nodes (ENC-5 open, `enc.rs:323`), the ROM tree as a 69-level binary sparse tree over `(tbl, row)` with empty-subtree digests (`enc.rs:385`, `enc.rs:408`), `BUS_BASE` 64 and the sponge sequence bases (`bus.rs:11`, `bus.rs:61`), the DEC row form (O3-2 open, `dec.rs:5`), S = 4 slots (`core.rs:26`), the public values binding 0, `log_d = D_EMPTY` and `log_n = 0` (`core.rs:471`), the MEM append chain `tail' = P2(TAG_NODE, tail, rd)` (`mem.rs:9`), and the builtin `ty_d = P2(DOM_TY, row)` (`leaf.rs:95`). The admission uses versions 1 and a test-only seeded salt `[1..8]` (`leaf.rs:108`, test only, never a production salt).

## 7. Circuit changes for the Carbon Quint lanes

All in openvm `~/.cache/m7-wrap/src/openvm-t6`, branch `t6-leaf` (commits `810b370`, `c3bea58`, `6ec1364`, `c541730`). Paths are relative to `crates/`.

| Change | File:line |
|---|---|
| New AIR CORE (slots, enable prefix, DEC dispatch, `rules_d`, five rule guards and effects, charges, BOUNDARY header and body, INIT, public values, ports) | `sherlock-leaf/src/core.rs:186` (struct), `:219-552` (eval) |
| New AIR DEC (subset table, provider) | `sherlock-leaf/src/dec.rs:60`, `:70-99` |
| New AIR SPONGE (Poseidon2 sub-AIR, sponge ports) | `sherlock-leaf/src/sponge.rs:57`, `:76-132` |
| New AIR HASH2 (HASH bus provider) | `sherlock-leaf/src/sponge.rs:216`, `:228-262` |
| New AIR ROM (ROM line, 69-level path, ROM bus provider) | `sherlock-leaf/src/rom.rs:43`, `:55-113` |
| New AIR MEM (dense append window, chain, token) | `sherlock-leaf/src/mem.rs:76-178`; MEM bus message fix at `:163-167` |
| Stock `VariableRangeCheckerAir` at 16 bits (RANGE) | `sherlock-leaf/src/leaf.rs` `airs` |
| Bus registry and ports (FP-1) | `sherlock-leaf/src/bus.rs` |
| Kit-test requester AIRs (RangeProbe, BusProbe, MemDriver; tests only) | `sherlock-leaf/src/kits.rs:47`, `:110`, `:159` |
| `KeyBind` (Leaf) receives the app public values on the PublicValuesBus | `sherlock-lfr/src/keybind.rs:136` (field), `:159` (`with_app_pvs`), `:243` (receive), `:495` (`trace_app`) |
| Leaf layer keyed with `with_app_pvs(54)` | `sherlock-wrap-export/examples/slice_chain.rs:168` |

stark-backend `~/.cache/m7-wrap/src/stark-backend-t6` commit `deeeb0a` (`SB_MUTANT_PROVER`, prover only, test only) changes no circuit or verifier.
