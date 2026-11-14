# T6 results: the modular leaf, vertical slice (group V)

Row T6 of `research/sherlock-tested-design-20261113/LOOP.md`. Change `openspec/changes/sherlock-modular-leaf-20261111/` (`ML/` below), tasks V.1 to V.10. Trees, all on branch `t6-leaf`: Moriarty `~/Projects/Moriarty-wt/t6-leaf` (base `2de61edb8`), openvm `~/.cache/m7-wrap/src/openvm-t6` (base `b3fbaa5`). State, logs and outputs under `~/.cache/t6/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead: scope change found by V.1

No Sherlock program meets the V.1 condition "reads no ROM row beyond what INIT opens". Every run starts at `Eval { expr: e0 }`, and every rule at an Eval control reads the expression row `expressions[e0]` (`owner_site`, T5 section 3.3 atom `E_TAG`). INIT opens `prog_d` to `image_d` to `prog_root` and the entry index `entry_a`, not the row (`ML/specs/sherlock-init-admission` INIT-007, FP-4). So one ROM row read, with its ROM line under `prog_root` (FP-9.2, row I-6), enters the slice. The V.1 program needs no stack step and no flush, so STACKS and MBK-003 stay out. `ML/tasks.md` V.1 and `ML/design.md` section 8 are amended to say so.

## 1. Status

| Task | Status | Receipt |
|---|---|---|
| V.1 | program chosen, reference run recorded | pending |
| V.2 to V.10 | not started | |

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
