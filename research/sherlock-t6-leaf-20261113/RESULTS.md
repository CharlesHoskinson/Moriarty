# T6 results: the modular leaf, vertical slice (group V)

Row T6 of `research/sherlock-tested-design-20261113/LOOP.md`. Change `openspec/changes/sherlock-modular-leaf-20261111/` (`ML/` below), tasks V.1 to V.10. Trees, all on branch `t6-leaf`: Moriarty `~/Projects/Moriarty-wt/t6-leaf` (base `2de61edb8`), openvm `~/.cache/m7-wrap/src/openvm-t6` (base `b3fbaa5`). State, logs and outputs under `~/.cache/t6/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

**Scope change found by V.1.** No Sherlock program meets the V.1 condition "reads no ROM row beyond what INIT opens". Every run starts at `Eval { expr: e0 }`, and every rule at an Eval control reads the expression row `expressions[e0]` (`owner_site`, T5 section 3.3 atom `E_TAG`). INIT opens `prog_d` to `image_d` to `prog_root` and the entry index `entry_a`, not the row (`ML/specs/sherlock-init-admission` INIT-007, FP-4). So one ROM row read, with its ROM line under `prog_root` (FP-9.2, row I-6), enters the slice. The V.1 program needs no stack step and no flush, so STACKS and MBK-003 stay out. `ML/tasks.md` V.1 and `ML/design.md` section 8 are amended to say so.

**Readings and decisions to accept or overrule:**
1. **V.9 `slice_air_omitted` reading.** A slice AIR left out of the app proof is refused by the stock app verifier (`RequiredAirNoVData`). No leaf proof over such an app proof can be made, so no node or `L` ever sees one. The refusals "at N1, N3 and `L`" are the T3 recursion-AIR omissions rerun on the slice keys (section 4).
2. **`fp8_d3` two-column rule.** The stock `SoundnessCalculator` panics when a proof has one stacked column (the slice app proof). `fp8_d3` evaluates that case at 2 columns. This is conservative, and the app's `vk` and `bounds` rows pass without it (section 5, openvm `c541730`).
3. **`SHK_RULES` moved from 58 to 64.** 58 to 62 are the ENC-5 parts under the T9 lead ruling, and 63 is left for `SHK_SHAPE`. The tag is in CORE, so every receipt was rerun.
4. **`plan_d` diverges from T9 ENC-5.** The slice uses the tag-only `P2(DOM_PLAN)`, and T9 uses the 42-word ENC-5 form. Not fixed (section 6).
5. **What V.10 binds.** The chain binds the keys of the run, not the slice outcome. `L` carries the FP-3 interface statement, and the leaf statement does not travel up the tree yet (FP-2). N2 and N3 run over duplicated children (section 5).
6. **Voided first receipts.** The 18:34 reboot emptied the V.2 to V.8 gate scripts, so the first V.2 to V.8 receipts ran nothing. They are void, and their seven 3 ms lines stay in `qa/sherlock/runs.jsonl` (section 3). Every receipt that counts ran after the scripts were restored.
7. **Carbon Quint lanes not yet run.** Every circuit change is listed with file:line in section 7.

## 1. Status

| Task | Status | Receipt |
|---|---|---|
| V.1 | done | `qa/sherlock/traces/20261114T005955Z-V.1.log` PASS |
| V.2 | done | `qa/sherlock/traces/20261114T024644Z-V.2.log` PASS (copy `runs/receipt-V.2.log`) |
| V.3 | done | `qa/sherlock/traces/20261114T024718Z-V.3.log` PASS (copy `runs/receipt-V.3.log`) |
| V.4 | done | `qa/sherlock/traces/20261114T024747Z-V.4.log` PASS (copy `runs/receipt-V.4.log`) |
| V.5 | done | `qa/sherlock/traces/20261114T024810Z-V.5.log` PASS (copy `runs/receipt-V.5.log`) |
| V.6 | done | `qa/sherlock/traces/20261114T024852Z-V.6.log` PASS (copy `runs/receipt-V.6.log`) |
| V.7 | done | `qa/sherlock/traces/20261114T024938Z-V.7.log` PASS (copy `runs/receipt-V.7.log`) |
| V.8 | done | `qa/sherlock/traces/20261114T025005Z-V.8.log` PASS (copy `runs/receipt-V.8.log`) |
| V.9 | done | `qa/sherlock/traces/20261114T030955Z-V.9.log` PASS (copy `runs/receipt-V.9.log`) |
| V.10 | done: T6 done-when met | `qa/sherlock/traces/20261114T031526Z-V.10.log` PASS (copy `runs/receipt-V.10.log`) |

Every receipt V.2 to V.10 ran at openvm `3160f59` (V.2 to V.8 at Moriarty `04215fa55`; V.9 and V.10 at `9784e4940`, whose gate script also records the host load); each trace names the openvm commit and lists its cases. Run records in this folder, `runs/` (MEASURED, from those receipt runs): `v2.json` to `v8.json` (one per kit gate), `omit.json` and `sg6.json` (V.9), `chain.json` (V.10, every layer: keys, heights, stacked widths, times, statements), `d3.json` (D3 of all 7 keys), and the receipt traces `receipt-V.2.log` to `receipt-V.10.log`. Five runs of the V.10 chain (18:55, 19:20, 19:36, 19:56, 20:15) gave the same shapes, fixed heights and D3. The first two runs, before openvm `f143943`, had `SHK_RULES` = 58, so their app key differs. Every recursion key (leaf, N1, N2, N3, `L_iface`, `L_Fr`) is the same in all five runs, because the layers commit to the child key and their own keys do not depend on it.

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
| V.2 | done | `qa/sherlock/traces/20261114T024644Z-V.2.log` PASS (copy `runs/receipt-V.2.log`) |
| V.2 | done | `qa/sherlock/traces/20261114T024644Z-V.2.log` PASS (copy `runs/receipt-V.2.log`) |
| V.2 | done | `qa/sherlock/traces/20261114T024644Z-V.2.log` PASS (copy `runs/receipt-V.2.log`) |
| V.2 | done | `qa/sherlock/traces/20261114T024644Z-V.2.log` PASS (copy `runs/receipt-V.2.log`) |
| V.2 | done | `qa/sherlock/traces/20261114T024644Z-V.2.log` PASS (copy `runs/receipt-V.2.log`) |
| V.3 | done | `qa/sherlock/traces/20261114T024718Z-V.3.log` PASS (copy `runs/receipt-V.3.log`) |
| V.3 | done | `qa/sherlock/traces/20261114T024718Z-V.3.log` PASS (copy `runs/receipt-V.3.log`) |
| V.3 | done | `qa/sherlock/traces/20261114T024718Z-V.3.log` PASS (copy `runs/receipt-V.3.log`) |
| V.4 | done | `qa/sherlock/traces/20261114T024747Z-V.4.log` PASS (copy `runs/receipt-V.4.log`) |
| V.4 | done | `qa/sherlock/traces/20261114T024747Z-V.4.log` PASS (copy `runs/receipt-V.4.log`) |
| V.5 | done | `qa/sherlock/traces/20261114T024810Z-V.5.log` PASS (copy `runs/receipt-V.5.log`) |
| V.5 | done | `qa/sherlock/traces/20261114T024810Z-V.5.log` PASS (copy `runs/receipt-V.5.log`) |
| V.5 | done | `qa/sherlock/traces/20261114T024810Z-V.5.log` PASS (copy `runs/receipt-V.5.log`) |
| V.6 | done | `qa/sherlock/traces/20261114T024852Z-V.6.log` PASS (copy `runs/receipt-V.6.log`) |
| V.6 | done | `qa/sherlock/traces/20261114T024852Z-V.6.log` PASS (copy `runs/receipt-V.6.log`) |
| V.6 | done | `qa/sherlock/traces/20261114T024852Z-V.6.log` PASS (copy `runs/receipt-V.6.log`) |
| V.7 | done | `qa/sherlock/traces/20261114T024938Z-V.7.log` PASS (copy `runs/receipt-V.7.log`) |
| V.7 | done | `qa/sherlock/traces/20261114T024938Z-V.7.log` PASS (copy `runs/receipt-V.7.log`) |
| V.7 | done | `qa/sherlock/traces/20261114T024938Z-V.7.log` PASS (copy `runs/receipt-V.7.log`) |
| V.8 | done | `qa/sherlock/traces/20261114T025005Z-V.8.log` PASS (copy `runs/receipt-V.8.log`) |
| V.8 | done | `qa/sherlock/traces/20261114T025005Z-V.8.log` PASS (copy `runs/receipt-V.8.log`) |
| V.8 | done | `qa/sherlock/traces/20261114T025005Z-V.8.log` PASS (copy `runs/receipt-V.8.log`) |

V.5 "one positive per branch of each subset rule": each of the five subset rules has one branch in the V.1 run, and the two shard proofs cover all five.

Fix found by V.6 (MEASURED): MemAir sent the MEM bus message from the `addr` columns through the carry columns, so a window with appends did not balance (`crates/sherlock-leaf/src/mem.rs`, MEM bus send). The message is now `kind, addr, rec`. The slice run has no append, so V.2 to V.5, V.7 and V.8 did not see it.

Receipts: an incident voided the first V.2 to V.8 receipts. The 18:34 reboot left `check-slice.sh` and `check-v2.sh` to `check-v8.sh` at 0 bytes, and commit `afdc52209` holds them empty, so `doctorwatson check` ran an empty script (exit 0 in 3 ms, empty traces `20261114T015032Z` to `015036Z`). Those receipts do not count. Commit `6c9513f7d` restores the scripts, and the receipts below come from the rerun.

## 4. V.9 the slice leaf shard in place of W1

Example `crates/sherlock-wrap-export/examples/slice_chain.rs`, mode `omit`; gate `scripts/qa/check-v9.sh`. The app is the slice leaf shard, keyed with `sherlock_leaf::harness::key` (every AIR required, SG-1; inside the frozen app set, SG-4). The leaf layer is `VerifierSubCircuit<1>` with `KeyBind` (Leaf), and `KeyBind` receives the 54 slice public values (circuit change, section 7). N1 runs over the two leaf proofs, then N2, N3 and `L` as in T3.

Per-AIR shape (MEASURED, `runs/omit.json` `app_shape`):

| AIR | Columns | Constraints | Degree | Interactions | Height |
|---|---:|---:|---:|---:|---:|
| CORE | 676 | 1,476 | 3 | 145 | 8 |
| DEC | 9 | 18 | 2 | 1 | 8 |
| RANGE (stock `VariableRangeCheckerAir`) | 4 | 10 | 3 | 1 | 131,072 |
| SPONGE | 321 | 326 | 3 | 2 | 512 |
| HASH2 | 29 | 5 | 2 | 5 | 128 |
| ROM | 1,222 | 78 | 2 | 77 | 2 |
| MEM | 100 | 100 | 3 | 21 | 8 |

Keys of the layers (MEASURED, `runs/chain.json` `keys`): app 7 AIRs, 2,361 columns, vk 70,560 bytes; leaf layer 40 AIRs, 1,675 columns, 81,234 bytes; N1, N2, N3 40 AIRs, 1,637 columns, at most 84 interactions; N3 over N3 has the N3 key (fixed point); `L_iface` 44 AIRs, 1,893 columns; `L_Fr` 15 AIRs, 561 columns, vk 45,600 bytes. Every AIR is required in app, leaf, N1, N2, N3 and `L` (MEASURED, `omit.json` `all_required`).

`slice_air_omitted` and the T3 omit cases on the slice keys: 57 cases, all PASS (MEASURED, `runs/omit.json`):
- Each of the 7 slice AIRs left out of the app proof: the stock app verifier refuses the stripped real proof (`RequiredAirNoVData`). That is the one verifier refusal of a slice-AIR omission. Two further stops get no SG-3 credit under the harness rule, and the record lists them only as facts: the prover stops with a nonzero LogUp root, and the leaf layer cannot generate its trace over the stripped proof (index out of bounds in the recursion trace generator). So no leaf proof of an omitting app proof exists, and nothing above the leaf can carry the omission. The refusals at N1, N3 and `L` that V.9 names are the T3 recursion-AIR omissions (`KeyBindAir`, `ProofShapeAir`, `PublicValuesAir`) rerun on the slice keys. This reading of "`slice_air_omitted` refused at N1, N3 and `L`" is the lead's to accept.
- N1 and N3, each without `KeyBindAir`, `ProofShapeAir` or `PublicValuesAir`, are refused on the prover path and by the stock verifier, and the parent over the stripped child fails trace generation. `L` without each of the three is refused on the prover path and by the stock verifier, and the `L_Fr` tape generator refuses the stripped shape (`RequiredAbsent`) and the pinned honest shape (`Heights`).
- Honest controls accept: app, leaf, N2 over N1, N3 over N3, `L`, `L_Fr` tape.

SG-6 seconds per proved step (MEASURED, `--exclusive`, example `sg6` of `sherlock-leaf`, V.9 receipt run at openvm `3160f59`, `runs/sg6.json`, trace `runs/receipt-V.9.log`): GPU 0.139 s per proved step (`BabyBearPoseidon2GpuEngine` of `openvm-cuda-backend`, feature `cuda`; shard medians 0.356 s and 0.339 s over 3 timed runs after a warm-up; 0.087 s per slot). CPU: 1.86 s per proved step (shard medians 4.95 s and 4.33 s). The run proves the 5 steps of the V.1 run in 2 shards of 4 slots. Both engines key the same vk, and the stock CPU verifier accepts the first proof of each shard from each engine. The earlier exclusive runs gave GPU 0.140 s and 0.148 s. Linking the CUDA build needs `LIBRARY_PATH=/opt/cuda/lib64`, because the cuda-builder default is `/usr/local/cuda/lib64`. Host load (DG-006, in the receipt trace): `/proc/loadavg` was 21.51, 14.86, 14.15 before the run and 19.14, 15.17, 14.29 after it, and GPU utilization was 1 percent before it. The run held the heavy-slot exclusive lock, but other agents' jobs outside heavy-slot kept the CPU loaded. So the CPU figure is an upper bound under load, and the GPU figure carries less of that load.

## 5. V.10 one run through every layer

Run (MEASURED, `--exclusive`, V.10 receipt run at openvm `3160f59`, Moriarty `9784e4940`, `runs/chain.json`, trace `runs/receipt-V.10.log`): the V.1 program `entry x;` as 2 shards (4 and 1 proved steps), with `post_d` of shard 0 equal to `pre_d` of shard 1. Two app proofs, two leaf proofs, N1 over both, N2, N3, `L_iface` over N3, `L_Fr`: each layer is proved and accepted by its stock verifier, and every layer's heights pass G1 (`frozen::check_heights`). Wall time 395 s. Per layer: leaf 6.4 s for 2, N1 6.1 s, N2 4.2 s, N3 3.8 s, `L` 61.9 s, `L_Fr` keygen 59.8 s, `L_Fr` prove 216.4 s. The app proofs on the CPU take 8.8 s for the 2 shards. Host load (in the trace): `/proc/loadavg` 19.14 before the chain and 23.87 after it.

Fixed heights: both shards have log heights `[3, 3, 17, 9, 7, 1, 3]`, equal to `leaf::HEIGHTS` (MEASURED).

`key_root` = `key_root([key_triple(0, app.pre, leaf.pre, n1.pre)])` over this run's keys. It enters `prog_d` in the CORE public values (FP-8 form).

D3 (MEASURED, `fp8_d3` in the V.10 receipt run, `runs/d3.json`): every term is at least 100 bits and every grindable term at least 101 bits, at the measured shape, the vk bounds and the frozen-set bounds, for all 7 keys. Weakest term at the measured shape:

| Key | Max log height | Stacked columns | Weakest | mu | query | logup | fold |
|---|---:|---:|---|---:|---:|---:|---:|
| app | 17 | 1 (2 used) | shift 101.09 | 113.63 | 101.09 | 102.72 | 104.63 |
| leaf<1> | 17 | 10 | shift 101.01 | 110.46 | 101.01 | 101.72 | 105.63 |
| N1 | 19 | 98 | shift 101.16 | 104.03 | 101.16 | 102.72 | 102.63 |
| N2 | 17 | 72 | shift 101.16 | 104.48 | 101.16 | 102.72 | 102.63 |
| N3 | 17 | 48 | shift 101.16 | 105.07 | 101.16 | 102.72 | 102.63 |
| `L_iface` | 16 | 18 | shift 101.14 | 102.54 | 101.14 | 102.72 | 101.63 |
| `L_Fr` | 19 | 43 | query 101.14 | 226.46 | 101.14 | 215.95 | 232.86 |

The app proof is one stacked column, and the stock `SoundnessCalculator` asserts at least 2 for the mu term. `fp8_d3` uses 2 for a one-column proof (openvm `c541730`). That value is conservative: a single column has no mu batching, and the mu error grows with the column count. The `vk` and `bounds` rows of the app do not use the measured width and pass on their own. N2 now has a measured row (it had only vk and bounds in FP-8).

What V.10 does not show: `L_iface` proves over N3 with the FP-3 interface statement (`Dims::FULL`, all-entries case, seed 14), not with the slice statement. The leaf layer receives the slice public values but does not expose them, and N1 to N3 carry no slice statement (FP-2 node statement, group 5). One proof chain therefore binds the keys of the run, not the slice outcome, up to `L_Fr`. N2 and N3 run over two copies of their child because the run has a single N1 (the T3 bench topology).

## 6. Remaining T6 items (outside group V)

| Item | What it needs |
|---|---|
| X10 in the leaf (I-12, I-35) | The X10 check as leaf AIRs over the slice state (rows I-12 and I-35 of T5), their mutants as SG-3 real proofs, and a slice program that reaches X10. The `entry x;` run does not. |
| Multi-segment `TAG_NODE` merge in `L` | An `L`-side AIR that folds the per-segment statements (`pre_d`, `post_d`, `h_pre`, steps) into one `TAG_NODE` digest and checks the `post_d` to `pre_d` stitch across segments. It also needs the node statement below (FP-2), because `L` today sees the FP-3 interface words only. |
| FP-2 node statement | The leaf `KeyBind` receives the 54 slice public values but does not expose them. Carrying the leaf statement through N1 to N3 and into `L` is group 5 (NODE INTEGRATION). Until then, the chain binds the keys of the run, not its outcome. |
| N2 and N3 arity with one N1 | The run has one N1, so N2 and N3 run over two copies of their child (the T3 bench topology). A run-shaped tree needs a padding or odd-arity rule for nodes (group 5). |
| DEC beyond the subset | Six DEC rows (idle and classes 1, 10, 12, 67, 89) in a subset table. The full priority network waits on T5 row I-7. |
| MEM near the top address | The honest MEM window checks `addr + i` without a wrap at 2^64 (`counter_wrap_2_64`). A window that starts within 8 of 2^64 has no honest witness. Real runs never get there. |

Divergences from T9 (read-only sweep of `~/.cache/m7-wrap/src/openvm-t9/crates/sherlock-lfr/src/enc45.rs` and the T9 lead rulings; not fixed here):
- `plan_d`: T9 ENC-5 is `P2(DOM_PLAN, 1, itab_d, menu_d, policy_d, layout_d, place_d)` (42 words with the tag, part tags 58 to 62). The slice uses the tag-only `P2(DOM_PLAN)` for a program with no plan nodes (`enc.rs:324`). The slice `prog_d` therefore differs from the T9 reference for the same program until the slice computes `plan_d` by ENC-5.
- `SHK_RULES`: it was 58 and clashed with `DOM_PLAN_ITAB`; T6 moved it to 64 (decision above).
- Agree: `EMPTY_F = P2(37)` and `EMPTY_T = P2(38)` (T9 `empty_below`), `DOM_PLAN` 41, `DOM_ROMLEAF` 39, `DOM_ROMNODE` 40, `DOM_STATE_BODY` 30, `DOM_MEMREC` 33.
- ENC-4 task lane 3 `tag(2)` (lead ruling): the slice run pushes no task, so the slice has no such lane in a message yet.

Slice PICKs (each marked `PICK` in the source, open for the owner or the kit tasks): `SHK_RULES` code 64 (`enc.rs:34`; 58 to 62 are the ENC-5 parts by the T9 lead ruling and 63 is left for `SHK_SHAPE`, so T6 moved it from 58 to 64 in openvm `f143943` and reran every receipt), body schema id 1 and the 476-word body (`enc.rs:38`), empty frame and task heads `P2(37)`, `P2(38)` (ENC-4 open, `enc.rs:309`), genesis memory root and tail `D_EMPTY` (`enc.rs:317`), `plan_d = P2(DOM_PLAN)` for a program with no plan nodes (ENC-5 open, `enc.rs:324`), the ROM tree as a 69-level binary sparse tree over `(tbl, row)` with empty-subtree digests (`enc.rs:386`, `enc.rs:409`), `BUS_BASE` 64 and the sponge sequence bases (`bus.rs:11`, `bus.rs:61`), the DEC row form (O3-2 open, `dec.rs:5`), S = 4 slots (`core.rs:26`), the public values binding 0, `log_d = D_EMPTY` and `log_n = 0` (`core.rs:471`), the MEM append chain `tail' = P2(TAG_NODE, tail, rd)` (`mem.rs:9`), and the builtin `ty_d = P2(DOM_TY, row)` (`leaf.rs:95`). The admission uses versions 1 and a test-only seeded salt `[1..8]` (`leaf.rs:108`, test only, never a production salt).

## 7. Circuit changes for the Carbon Quint lanes

All in openvm `~/.cache/m7-wrap/src/openvm-t6`, branch `t6-leaf` (commits `810b370`, `c3bea58`, `6ec1364`, `c541730`, `f143943`, `3160f59`). Paths are relative to `crates/`.

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
