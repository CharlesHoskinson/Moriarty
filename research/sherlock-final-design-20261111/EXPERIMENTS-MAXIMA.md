# Per-rule memory maxima and the one-arena non-interleaving check (2026-11-11)

Asked for by `DECISIONS.md` R-6, L-2, L-9 and E-3, `research/sherlock-runlength-consensus-20261111/AUDIT.md` finding 6 and `research/sherlock-lookup-cache-20261111/AUDIT.md` blocker 1 and finding 8: before leaf sizes (`K_page`, `K_rom`, `W_flush`, arena count) freeze, (1) a per-rule table of the maximum appends, reads, segments, stack operations and hashes of one step, and (2) a mechanical check of the one-arena non-interleaving claim.

Labels used below:

- **STATIC**: derived mechanically by a rule-sliced scan of the Core source text (an upper bound, see "Method").
- **OBSERVED**: measured mechanically on real owner steps over a corpus (a lower bound of the true maximum; a rule that did not fire has no observed value).
- **READ**: code reading by the author of this report, with the code named.
- **DERIVED**: a formula over STATIC or OBSERVED counts.

## Result in short

- Rules: 120 enabled step rules, of which 108 are the `supported_rules()` set of `c6_dispatch_step.rs` and 12 are the `XAdtHelperBuild*` and `XAdtHelperPlan*` rules (entry plus group) that the 108 leave out. The Build and Plan rules are real Core steps on every ADT program, so they are in the table with `in_108 = 0`. Every other `RuleIdC` variant is one of the 11 rules `is_disabled` or `is_extension` refuses, or one of the two `KAdminStartup*` rules (no step). STATIC, asserted.
- Maximum over all 120 rules, one step:

| Quantity | Max | Rule(s) | Label |
|---|---|---|---|
| appends, all kinds | 3 cells | `BHelperScalarExit` (Cons or Branch node: 2 FIELDS + 1 DATA) | STATIC = OBSERVED |
| appends DATA / FIELDS / ENV / SLOTS | 1 / 2 / 1 / 1 | DATA: node or constructor rules; FIELDS 2: `BHelperScalarExit`; ENV: `BHelperEnvPutInsert`, `BHelperBindInsert`, `BHelperMatchBinding`, `XAdtHelperPlanBind`; SLOTS: `BHelperAppendCopy`, `BHelperAppendTail` | STATIC = OBSERVED |
| appends of two kinds in one step | 2 rules | `BHelperScalarExit` (FIELDS then DATA), `XAdtHelperBuildField` (FIELDS then DATA) | STATIC |
| one-arena segments appended into | 2 | `BHelperScalarExit` (3 cells across a boundary) | OBSERVED; 2 is also the bound for 3 consecutive cells (DERIVED) |
| frame operations | 1 (push, pop or rewrite-top) | push: `KMachinePushAccept`; pop: return rules; rewrite: `KMachineReturnLet`, `KMachineReturnLeft`, `KMachineReturnArgumentsNext`, `XAdtReturnCtorFieldsTyped`, `XAdtReturnCtorFieldsNext` | STATIC = OBSERVED |
| task operations | 1 push or pop, plus 1 rewrite-top in the same step | push + rewrite: `BHelperValidateField`, `BHelperEqualField` | STATIC (OBSERVED for `BHelperEqualField`) |
| frame and task operation in one step | none | | STATIC |
| distinct heap records read (DATA, FIELDS, ENV, SLOTS) | 4 | `BHelperEqualField` (left row, left field cell, right row, right field cell) | STATIC for 116 rules (call-site bound at most 4); READ for the 4 rules whose call-site bound is above 4 (`BHelperScalarTag` 7, `BHelperValidateField` 5, `BHelperEqualPair` 5, `BHelperEqualField` 9); OBSERVED 4 |
| distinct heap records read during selection only | 2 | `BHelperEqualPair` (Equal `shallow_differ` reads of the group) | OBSERVED |
| distinct old (flushed) one-arena segments read | 2 observed; at most 4 | `BHelperEqualPair`, `BHelperEqualField`, `BHelperMatchField`, `XAdtHelperPlanWalk` | OBSERVED 2; READ bound 4 (one per record) |
| distinct one-arena segments touched (reads and appends) | 3 observed; at most 4 + 2 | `BHelperEqualField` | OBSERVED 3; DERIVED bound 6 |
| distinct four-arena segments touched | 2 observed | several | OBSERVED |
| ROM reads logged (family code) | 3 entries in 3 tables, 3 blocks of 8 | `BHelperMatchArm`, `XAdtEvalConstructFields`, `XAdtHelperPlanSwitch` | OBSERVED, a lower bound: the log counts one `program_view()` call as one entry, so the ADT rows below are not in it; selection adds at most 2 PROG expression rows (READ, see below) |
| ADT ROM row reads through `program_view()` | 6 call sites in 4 tables | `XAdtHelperPlanSwitch` (`constructors`, `plan_cases`, `plan_nodes`, `types`); 4 sites in `XAdtEvalConstructFields` and `XAdtReturnCtorFieldsNext` (`constructors`, `ctor_fields`, `types`) | STATIC (call sites `view.<table>[..]` and `view.<table>.get(..)` in the kept text, no loop); 11 ADT rules read ROM this way |
| read age | 507 steps | `BHelperMatchField` | OBSERVED: reads reach cells written hundreds of steps earlier |
| hash calls without cold reads | 3 | `BHelperScalarExit` (3 record hashes) | DERIVED |
| hash calls per cold read | 1 record hash + `TOP + 3` node hashes | every rule with a heap read | DERIVED |

- Non-interleaving: **holds** for all 120 rules. No counterexample. The premises S1 to S4 below are checked mechanically (static over all rules, and dynamically on every corpus step); the induction that joins them is a code reading. 9,031 slices of two or more elements were checked on 6,668 applied steps with no violation.
- Coverage: 90 of the 108 rules and 102 of the 120 fired on the corpus. 16 of the 18 rules that did not fire are failure or refusal paths that the corpus programs do not reach; the other 2 (`BHelperValidateField`, `BHelperBindPropagate`) are not failure paths and the corpus did not reach them (list in "Open"). The 18 have STATIC values only, and all of them have zero appends (STATIC).

## Method

Code (test-only, `#[cfg(test)]`, no owner, family or circuit change):

- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime/rule_maxima_tests.rs`, declared at the end of `owner_adt_runtime.rs`. It is a child of `owner_adt_runtime` so that it can use the private `impl_adt_runtime_reads!` macro, `family_of_rule`, `helper_group`, `legacy_group`, `owner_select_when`, `OwnerGuards::new_owner` and the private `MachineImage` fields.
- `.../rule_maxima_tests/scan.rs`: the text scanner.
- Test: `rule_maxima_table_and_one_arena_noninterleaving`. With `MAXIMA_OUT=<dir>` it writes `rule_maxima.csv`, `rule_maxima.md` and `noninterleave.json`. Copies are in `research/sherlock-final-design-20261111/maxima/`.

### Rule universe (STATIC)

The 108 are built as `supported_rules()` builds them: the three publications, `helper_group` of the nine kinds Scalar to Match, `legacy_group` of the 27 sites, and the nine entry rules; the test asserts 108. The 12 Build and Plan rules are added. The variant names of `RuleIdC` are parsed from `shared.rs`; the test asserts that the names outside the 120 are exactly the 7 `is_disabled` rules, the 4 `is_extension` rules and `KAdminStartupValid`/`KAdminStartupInvalid`, and that none of the 120 is disabled or an extension rule. The backend list is a separate copy of the same group tables in `c6_dispatch_step.rs`; that it equals the Core groups is READ, not checked by this test.

### Static scan (STATIC)

For each rule, the scan starts at the family entry that `family_of_rule` names (for example `environment::plan_append`, `legacy::dispatch`). In every `match` it removes the arms whose pattern names only other rules, and a `_` arm when a pure arm names this rule. It follows free calls inside the scanned files (families, both fragments, `owner_adt_runtime.rs`, `adt_runtime.rs`, `owner_runtime.rs`), `reads.<method>` calls into the ADT runtime, and `GuardId::<name>` uses into the arms of `OwnerGuards::eval_guard`. A call resolves to the named module, then to the calling module, and otherwise to every definition of that name in the scanned files (an over-approximation when the name is ambiguous). The test asserts that every rule's family entry is found. Free-call names with no definition in the scanned files are listed per rule in `noninterleave.json` (`unresolved_calls`); over all rules they are only constructors, enum variants and standard methods (`Const`, `CtorIdC`, `Data`, `Err`, `Ok`, `Push`, `RewriteTop`, `Set`, `Some`, `checked_add`, `checked_mul`, `checked_sub`, `default`, `empty`, `from`, `normalized`). Every branch not excluded by a rule pattern stays in, so the kept text over-approximates the code the rule can run.

From the kept text:

- Writes: whether `alloc_data` (and which `AllocDataWrite` variants), `alloc_data_field`, `alloc_constructor`, `alloc_env`, `alloc_value_slot`, `task_rewrite_top` are set to something other than `None`, and whether `FrameWrite::{Push, Pop, RewriteTop}` and `TaskOp::{Push, Pop}` occur. Each `alloc_*` field is one `Option`, and `alloc_data` carries at most 2 fields, so this bounds the appends: DATA + FIELDS is `1 + field_len` for `alloc_data` or `1 + 1` for a Build field with its constructor (`owner_validate_plan` refuses the two together), plus 1 ENV and 1 SLOTS.
- Reads: call sites of the `PreImageView` read methods, summed over all kept branches, in the family text and separately in the `select_when` arms of the rule and of every rule before it in its group (the order `owner_select` evaluates them). A value read (`deref_borrowed`, `kind_of`, `u128_of`, `bool_of`, `scalar_test_type_ok`) counts 1 record, `field_at` counts 2 (row and field cell). The sum is an upper bound when the kept text has no loop; the scan found no `for`, `while`, `loop` or iterator in any rule's kept text.
- ADT ROM rows: the ADT runtime reads ROM rows directly from the `ProgramView` that `program_view()` returns. The scan counts the call sites `view.<table>[..]` and `view.<table>.get(..)` (any spacing; `.len()` is table metadata and is not counted) and records the tables (CSV columns `st_view_rom_row_sites`, `st_view_rom_tables`).
- Publications (`KAdminPublish*`) do not reach a family; `owner_step` P1/P2 writes only the outcome (READ, `owner_runtime.rs` `owner_step`).

### Dynamic trace (OBSERVED)

Each corpus program is parsed with `parse_nominal_source`, admitted with `admit_ext_program` and run with `host_owner_engine().admit_run`. Before each `owner_step`, the test runs the same `owner_select`, the same `select_when` sequence and the same family plan over a logging `PreImageView` (`LogView`, with `AdtPreImageAccess` and the real ADT-runtime implementation, so ADT reads are logged too). Every read is logged as a record (`DATA[i]`, `FIELDS[j]`, `ENV[k]`, `SLOTS[p]`, or a ROM row); `field_at` logs the row and the field cell. The appends follow the plan's writes in the order of `owner_apply_validated`. The test asserts that the logged selection equals the fired rule, and that the post-image table lengths, frame count and task count equal the plan.

Address maps: one arena is a single append counter over DATA, FIELDS, ENV and SLOTS in apply order (frames and tasks excluded, as hash-chain stacks); four arenas use the per-kind index. A segment is 8 cells. A read is old (flushed at S = 1) when its segment is below `floor(len_pre / 8)`. The read age is the step distance from the cell's append.

The test asserts, for every fired rule, that the observed appends and stack operations are within the STATIC set, and that the observed distinct heap records are within the STATIC call-site bound. It passed.

### Corpus

28 programs in `moriarty-sherlock-nominal/1` (in `corpus()` of the test): factorial, tree fold, list sum, list and tree building (6 and 4 deep), three-argument calls, let/if/not/lt, literal and Bool, Tree and List constants, list and tree equality, subtraction underflow and addition overflow (code 10), stack overflow at stack limit 8 (code 4), fuel exhaustion, data-node cap exhaustion, a 10-field constructor built and matched (crosses an 8-cell segment), a 10-field constant, a two-constructor sum type, a non-exhaustive match (code 9), nested constructor patterns, a wildcard arm, nullary constructors, a zero-argument call and a Box of a list sum. All 28 ran to an outcome; 6,696 steps, 6,668 applied.

## Commands and runs

Every build and test ran through `scripts/doctorwatson heavy` with `CARGO_TARGET_DIR=~/.cache/final-design/maxima/target`, one at a time. `heavy.slice` `memory.current` was read by hand before 8 of the 14 runs (readings 29.5 GB to 33.5 GB, all under the 40 GiB limit); the `doctorwatson` preflight passed on every run. Run ids are in `qa/sherlock/runs.jsonl`, traces in `qa/sherlock/traces/`, logs in `~/.cache/final-design/maxima/logs/`.

| Run id | Command | Result |
|---|---|---|
| `20261111T101112Z-maxima-build` | `cargo test -p mori-core --lib --locked --no-run` (as `systemd-run --user` unit) | exit 101: wrong relative `include_str!` paths |
| `20261111T101141Z-maxima-build` | same | exit 0 |
| `20261111T101203Z-maxima-test` | `MAXIMA_OUT=... cargo test -p mori-core --lib --locked rule_maxima -- --nocapture` | pass; two programs refused by `admit_run` (fuel above `value_slots`) |
| `20261111T101235Z-maxima-test` | same, corpus fixed and extended | pass |
| `20261111T101322Z-maxima-clippy` | `cargo clippy -p mori-core --all-targets --locked -- -D warnings` | exit 101 in `mori-lexer` (`collapsible_if`, a file this change did not touch; also noted in `EXPERIMENTS.md` C1) |
| `20261111T101421Z-maxima-clippy` | `cargo clippy -p mori-core --no-deps --all-targets --locked -- -D warnings` | 3 findings in the new module, the rest in untouched files |
| `20261111T101511Z-maxima-clippy` | same, after the fix | 0 findings in the new module; cargo ends with "could not compile `mori-core` (lib) due to 271 previous errors" and "could not compile `mori-core` (lib test) due to 320 previous errors", all in files this change did not touch, under the pinned toolchain |
| `20261111T101710Z-maxima-test` | as `101235Z` | pass; first tables |
| `20261111T101724Z-maxima-t0` | `bash qa/tiers/t0.sh` | fail in the factory test `test_aggregate_kill_cgroup_slice` (cgroup timing, nothing staged yet) |
| `20261111T101920Z-maxima-t0` | same, files staged | pass (8 passed, 1 skipped) |
| `20261111T103010Z-maxima-test` | as `101235Z`, scan follows ambiguous calls and `adt_runtime.rs`, counts ADT view rows | pass; all earlier columns unchanged |
| `20261111T103123Z-maxima-test` | same, view-row matcher accepts line breaks between tokens | pass; the tables in `maxima/` come from this run |
| `20261111T103325Z-maxima-t0` | `bash qa/tiers/t0.sh` | fail: `cargo fmt --check` on the new module |
| `20261111T103539Z-maxima-t0` | same, after `rustfmt` (layout only) | pass (8 passed, 1 skipped) |

## Per-rule table

The full table is `maxima/rule_maxima.csv` (one row per rule, 59 columns: `st_*` STATIC, `dy_*` OBSERVED, `hash_*` DERIVED) and `maxima/rule_maxima.md`. Rows with any append, any stack operation or any heap read:

| Rule | 108 | Fired | Appends D/F/E/S (STATIC) | Appends total STATIC / OBSERVED | Frame | Task | Heap read bound STATIC | Heap records OBSERVED | Old segs 1-arena OBSERVED | Hash calls DERIVED |
|---|---|---|---|---|---|---|---|---|---|---|
| BHelperScalarExit | y | 211 | 1/2/0/0 | 3 / 3 | | | 0 | 0 | 0 | 3 |
| BHelperScalarConstruct | y | 37 | 0/0/0/0 | 0 / 0 | | | 3 | 1 | 1 | 0 |
| BHelperScalarArithmetic | y | 68 | 0/0/0/0 | 0 / 0 | | | 4 | 1 | 1 | 0 |
| BHelperScalarTag | y | 200 | 0/0/0/0 | 0 / 0 | | | 7 | 1 | 1 | 0 |
| BHelperEnvGetCell | y | 223 | 0/0/0/0 | 0 / 0 | | | 1 | 1 | 1 | 0 |
| BHelperEnvPutInsert | y | 2 | 0/0/1/0 | 1 / 1 | | | 0 | 0 | 0 | 1 |
| BHelperBindPair | y | 63 | 0/0/0/0 | 0 / 0 | | push | 1 | 1 | 1 | 1 |
| BHelperBindType | y | 63 | 0/0/0/0 | 0 / 0 | | pop | 1 | 1 | 1 | 1 |
| BHelperBindDuplicate | y | 6 | 0/0/0/0 | 0 / 0 | | pop | 2 | 1 | 1 | 1 |
| BHelperBindInsert | y | 63 | 0/0/1/0 | 1 / 1 | | pop | 0 | 0 | 0 | 2 |
| BHelperBindPropagate | y | 0 | 0/0/0/0 | 0 / - | | pop | 0 | - | - | 1 |
| BHelperAppendCopy | y | 100 | 0/0/0/1 | 1 / 1 | | | 0 | 0 | 0 | 1 |
| BHelperAppendCell | y | 100 | 0/0/0/0 | 0 / 0 | | | 1 | 1 | 1 | 0 |
| BHelperAppendTail | y | 94 | 0/0/0/1 | 1 / 1 | | | 0 | 0 | 0 | 1 |
| BBookkeepingTaskRelease | y | 1 | 0/0/0/0 | 0 / 0 | | pop | 0 | 0 | 0 | 1 |
| BHelperValidateInvalid | y | 0 | 0/0/0/0 | 0 / - | | pop | 1 | - | - | 1 |
| BHelperValidateOccurrence | y | 116 | 0/0/0/0 | 0 / 0 | | pop, rewrite | 3 | 0 | 0 | 1 |
| BHelperValidateField | y | 0 | 0/0/0/0 | 0 / - | | push, rewrite | 5 | - | - | 1 |
| BHelperValidateEntry | y | 116 | 0/0/0/0 | 0 / 0 | | push | 0 | 0 | 0 | 1 |
| BHelperEqualEntry | y | 20 | 0/0/0/0 | 0 / 0 | | push | 0 | 0 | 0 | 1 |
| BHelperEqualExit | y | 20 | 1/0/0/0 | 1 / 1 | | | 0 | 0 | 0 | 1 |
| BHelperEqualDifferent | y | 16 | 0/0/0/0 | 0 / 0 | | pop | 2 | 1 | 1 | 1 |
| BHelperEqualPair | y | 11 | 0/0/0/0 | 0 / 0 | | pop, rewrite | 5 | 2 | 2 | 1 |
| BHelperEqualField | y | 7 | 0/0/0/0 | 0 / 0 | | push, rewrite | 9 | 4 | 2 | 1 |
| BHelperMatchOccurrence | y | 45 | 0/0/0/0 | 0 / 0 | | | 1 | 1 | 1 | 0 |
| BHelperMatchField | y | 42 | 0/0/0/0 | 0 / 0 | | | 2 | 2 | 2 | 0 |
| BHelperMatchBinding | y | 42 | 0/0/1/0 | 1 / 1 | | | 0 | 0 | 0 | 1 |
| KMachinePushAccept | y | 345 | 0/0/0/0 | 0 / 0 | push | | 0 | 0 | 0 | 1 |
| KMachineReturnLet, ReturnLeft, ReturnArgumentsNext | y | | 0/0/0/0 | 0 / 0 | rewrite | | 0 | 0 | 0 | 0 |
| KMachineReturnRight, Unary, IfTrue, IfFalse, Match, ArgumentsFinal, CallValid, Scope | y | | 0/0/0/0 | 0 / 0 | pop | | 0 | 0 | 0 | 1 |
| XAdtReturnCtorFieldsTyped, XAdtReturnCtorFieldsNext | y | | 0/0/0/0 | 0 / 0 | rewrite | | 0 | 0 | 0 | 0 |
| XAdtReturnCtorFieldsFinal | y | 9 | 0/0/0/0 | 0 / 0 | pop | | 0 | 0 | 0 | 1 |
| XAdtHelperPlanWalk | n | 18 | 0/0/0/0 | 0 / 0 | | | 2 | 2 | 2 | 0 |
| XAdtHelperPlanSwitch | n | 8 | 0/0/0/0 | 0 / 0 | | | 1 | 1 | 1 | 0 |
| XAdtHelperPlanBind | n | 16 | 0/0/1/0 | 1 / 1 | | | 0 | 0 | 0 | 1 |
| XAdtHelperBuildField | n | 31 | 1/1/0/0 | 2 / 2 | | | 0 | 0 | 0 | 2 |
| XAdtHelperBuildCell | n | 31 | 0/0/0/0 | 0 / 0 | | | 1 | 1 | 1 | 0 |

All other rules have zero appends, no stack operation and no heap read (STATIC); their reads are registers and ROM rows only.

Notes on the read columns:

- "Can the address be older than the leaf window": **yes for every rule with a heap read** (READ). Every heap read address comes from a register value (control `Return` value, helper fields, the top task, the top frame or an environment root), and those values can name any cell written earlier. OBSERVED: all 16 rules with a heap read on the corpus read an old segment at S = 1 at least once, and the largest read age was 507 steps. Appends never read.
- The STATIC heap bound sums call sites over branches. The distinct-record maximum by code reading is 4 (`BHelperEqualField`: two `field_at`, rows re-read by the guard `GEqualFieldNontail` and by the `shallow_differ` select reads of the earlier group rules), 2 for `BHelperEqualPair`, `BHelperValidateField`, `BHelperMatchField` and `XAdtHelperPlanWalk`, and 1 for the scalar rules (one operand per step) (READ, `families/structural.rs` lines 153 to 195, `fragments/scalar.proposed.rs` lines 93 to 210).
- `BHelperValidateField` reads a constant in the corpus (literal validation reads ROM), but its reference is not restricted to constants by the code, so it is counted as a heap read.
- ROM: the selection reads at most the Eval expression row and the control-origin expression row (`owner_site`, `owner_helper_plan`; READ, `owner_runtime.rs` lines 1207 to 1311; these use `PreImage` directly and are not logged). This matches the two PROG rows of the dispatch facts in `c6_dispatch_partition.rs`. Family ROM reads OBSERVED: at most 3 log entries in 3 tables per step, where one `program_view()` call is one entry. The ADT rules also read rows directly from that view: STATIC at most 6 call sites in 4 tables in one step (`XAdtHelperPlanSwitch`), with no loop in the kept text. A STATIC bound on all ROM rows of a step is the ROM method call sites plus the view row sites: at most 9 for `XAdtHelperPlanSwitch` (3 + 6, where the 3 include the `program_view()` calls), 7 for `XAdtEvalConstructFields`.

Hash calls (DERIVED, design of `research/sherlock-runlength-consensus-20261111/CONSENSUS.md` 1.2.3 and the C1 build in `EXPERIMENTS.md`): one record hash per appended cell, one chain hash per frame or task push or pop (the top stays in registers, so a rewrite-top costs none), and per cold read one record hash plus `TOP + 3` node hashes (64 at `TOP = 61`). Leaf-level work (tail open and close, flush walk) is per leaf, not per step, and is not in the column. Maximum per step without cold reads: 3. Hash invocations of the present C6 carrier circuits were not counted.

## Non-interleaving check

### The property

In the one-arena layout every append of DATA, FIELDS, ENV or SLOTS takes the next address of one counter, in step order and, inside a step, in the order of `owner_apply_validated`. Frames and tasks are hash-chain stacks and take no address. The design needs Core's slice indexing `start + k` to stay valid on unified addresses. Stated exactly:

- **(A) step-local**: in every step, the appended cells of each kind form one run of consecutive unified addresses (no kind A, kind B, kind A pattern inside one step).
- **(B) slice contiguity**: for every slice `(start, len)` of kind FIELDS or SLOTS that is reachable from the machine state (field slices of data rows, the partial field block of an active Build, `completed` of `ArgsF` and `ConstructorFieldsF` frames and of a scheduled frame, `args` of `Invoke`, `Enter` and the Bind helper, `fields` of `Materialize`, `src` of the Append and Build helpers, the partial destination block of an active Append, and `AppendDone.seq`), `addr(start + k) = addr(start) + k` for all `k < len`.

ENV cells are linked by `previous` and DATA rows are single records, so no other kind is indexed by `start + k`.

### Premises (all hold)

| Premise | How checked | Result |
|---|---|---|
| S1: `owner_apply_validated` pushes in the order: `alloc_data` fields, `alloc_data` node, `alloc_data_field`, `alloc_constructor`, env, slot | STATIC: positions of the pushes in the function text | holds |
| S2: over all 120 rules, every Append-helper rule (entry and group) appends SLOTS only; every Build-helper rule appends FIELDS and the closing constructor only; only the Append group appends SLOTS; only `XAdtHelperBuildField` writes a Build field or constructor; no rule sets `alloc_data` together with a Build write; the two entry rules append nothing | STATIC, all rules | holds, no counterexample |
| S3: the entry rules start the block at the active length: `BHelperAppendEntry` takes `dest_start` from `active().value_slots`, `XAdtHelperBuildEntry` takes `field_start` from `active().data_fields` | STATIC (text of the sliced code); OBSERVED on every entry step | holds |
| S4: while a helper is active, `owner_select` selects only from `helper_group(kind)` | STATIC (text of `owner_select`); OBSERVED: the fired rule was in the active helper's group on every step | holds |

From S1 and S2, (A) holds for every rule: the only steps with two kinds are `BHelperScalarExit` (fields, then node) and `XAdtHelperBuildField` (last field, then constructor), and each kind is one run. (B) follows by induction over steps (READ): a FIELDS slice is written either in one step by `alloc_data` (consecutive by S1) or by the Build helper, which from entry (S3) to its last field runs only Build-group rules (S4) that append only FIELDS (S2), with `owner_validate_plan` requiring `field_start + src_cursor == active.data_fields` at every field; the constructor follows the last field. A SLOTS slice is written only by one Append run, which from entry to exit appends only SLOTS (S2 to S4); every other slot slice is a copy, a suffix (`legacy_tail`) or the result of such a run. Lane B's concern (`lanes/B/round2.md:35`) does not apply: argument evaluation between two Append runs allocates other kinds, but each Append copies `completed` into a fresh block.

### Dynamic check

On every applied corpus step the test checked (A) and (B) on the post image under the one-arena map: 6,668 steps, 9,031 slices of length 2 or more (including the 10-field constructor across an 8-cell segment), 0 violations of (A), 0 of (B), 0 entry-capture violations, 0 helper-exclusivity violations. Largest step: 3 cells.

### Verdict

The one-arena non-interleaving property holds for all 120 enabled rules. No counterexample. The over-all-rules part is mechanical for S1 to S4; the step from S1 to S4 to (B) is a code reading, and the dynamic check confirms it on 6,668 steps.

## What this means for leaf sizing

- `W_flush = 1` per step is safe in one arena: a step appends at most 3 consecutive cells, so it crosses at most one segment boundary (DERIVED). The per-leaf argument of the run-length AUDIT finding 8 is unchanged.
- At S = 1 one step needs up to 4 old heap records, in up to 4 distinct segments in one arena (READ bound; 2 OBSERVED), or up to 2 per arena in four arenas (DATA 2 and FIELDS 2 for `BHelperEqualField`). So `K_page >= 4` (one arena) is required for a step to fit with line caching, or 4 scalar cold reads per step with `cold_read`. This supersedes `K_page = 4` as a guess: it is now the bound from the rules, not yet a measured leaf.
- ROM: one step reads at most 9 ROM rows by family and ADT call sites (STATIC, `XAdtHelperPlanSwitch`, in up to 4 ADT tables plus the family tables) plus 2 selection expression rows (READ). The per-table block count for `K_rom` is not bounded by this experiment: the 3 tables and 3 blocks are OBSERVED from a log that counts a `program_view()` call as one entry.

## Open

- 18 of the 108 did not fire: `BHelperEnvGetFailure`, `BHelperEnvGetEmpty`, `BHelperFindFailure`, `BHelperFindEmpty`, `BHelperBindFailure`, `BHelperBindArity`, `BHelperBindPropagate`, `BHelperValidateFailure`, `BHelperValidateInvalid`, `BHelperValidateField`, `KMachineEvalLiteralInvalid`, `KMachineEvalVariableAbsent`, `KMachineReturnIfInvalid`, `KMachineReturnCallInvalid`, `KMachineEnterMissing`, `KMachineBoundInvalid`, `KMachineCasesEmpty`, `XAdtReturnCtorFieldsMistyped`. Their values are STATIC only. All have zero appends; `BHelperValidateField` has 2 heap records by code reading. 16 are failure or refusal paths. `BHelperValidateField` and `BHelperBindPropagate` are not: the corpus has no step that validates a field of a structured constant (the `constlist` and `wideconst` programs halt on `Data` references), and no step propagates a Bind. Whether source programs can reach the 18, or only hand-built images can, is open.
- The STATIC read bound is a call-site sum, not a distinct-record bound. The distinct maximum 4 is STATIC for the 116 rules whose call-site bound is at most 4, and READ for `BHelperScalarTag`, `BHelperValidateField`, `BHelperEqualPair` and `BHelperEqualField`. A per-path count (maximum over branches) would make it mechanical.
- The step from premises S1 to S4 to property (B) is a code reading. A Quint model or an assertion in `owner_apply_validated` would mechanise it.
- ROM blocks per table (`K_rom`) are open: the ADT view row sites are counted, but the rows they name (and so the blocks of 8) are not logged. Logging each `view.<table>` row read in `LogView` would give OBSERVED blocks per table.
- Hash counts are DERIVED from the design; no circuit with hash-chain stacks exists to measure them.
- Equality of the backend `supported_rules()` list with the Core groups is READ; the test rebuilds the 108 from the Core tables.
- The clippy errors in untouched `mori-core` files (271 lib, 320 lib test, cargo's own count) and the `mori-lexer` findings under the pinned toolchain are unchanged.
- The wiki lint (`python3 llm-wiki/tools/lint.py`) fails in the agent worktree until this change is merged, because the wiki's `~/Projects/Moriarty/...` source path resolves to the main checkout; with the path mapped to the worktree it passes.
