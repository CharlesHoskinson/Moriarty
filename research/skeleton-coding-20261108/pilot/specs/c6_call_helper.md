# Task: the `c6_call_helper` C6 step relation

Write `crates/mori-backend/src/sherlock_machine/c6_call_helper_step.rs`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: `RuleIdC::BHelperFindEntry`, `RuleIdC::BHelperFindRow`, `RuleIdC::BHelperFindExit`, `RuleIdC::BHelperEnvGetEntry`, `RuleIdC::BHelperEnvGetCell`, `RuleIdC::BHelperEnvGetExit`.

## Interface (fixed)

`crates/mori-backend/src/sherlock_machine/c6_call_helper_consumer.rs` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const CALL_HELPER_RULES: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn call_helper_supported(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct C6CallHelperEmitter` with
  `pub fn new(scope: &str) -> Result<Self, E>` and
  `pub fn emit(&mut self, rule: RuleIdC, inputs: &C6StepInputs) -> Result<B, E>`,
  where `E: std::fmt::Display` and `B` has a field `pub instructions: Vec<Instruction>`.

The statement is the full LocalPublic pre+post carrier statement that
`c6_step_harness::build_step_circuit` allocates (`C6StepInputs`): enable, host
schema, carrier version, then pre META/PROG/CTX/IMAGE/DATA/HELPER and post
META/PROG/CTX/IMAGE/DATA/HELPER lanes. Every lane is public; there are no
private inputs. Shared gate helpers are in `c6_step_kit.rs`. Other per-rule
step files in the same folder show the conventions.

## Correctness

- Completeness: every honest owner step of these rules (pre and post carriers
  encoded independently from the owner's snapshots) is accepted, and with
  enable = 0 the identity pair (post = pre) is accepted.
- Soundness: the prover chooses every lane. With enable = 1 the relation must
  accept a (pre, post) pair only if the owner machine, from that pre-state,
  fires one of these rules and produces exactly that post-state. With
  enable = 0 it must accept only the identity. A lane the rule fixes and the
  circuit does not constrain is a soundness bug.
- Keep the relation no larger than it needs to be; the instruction count is
  recorded.

## Reference machine (the specification)

The owner machine in `crates/mori-core` is the reference semantics. The step
entry is `step` in `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`
(P0/P1/P2 publication and fuel, then `owner_select`, the family plan and the
physical write). These lines name the rules:

- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:277: "BHelperFindEntry",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:284: "BHelperFindRow",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:291: "BHelperFindExit",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:340: "BHelperEnvGetEntry",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:347: "BHelperEnvGetCell",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:354: "BHelperEnvGetExit",`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1505: Self::BHelperEnvGetEntry => json!("BHelperEnvGetEntry"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1506: Self::BHelperEnvGetCell => json!("BHelperEnvGetCell"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1508: Self::BHelperEnvGetExit => json!("BHelperEnvGetExit"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1516: Self::BHelperFindEntry => json!("BHelperFindEntry"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1517: Self::BHelperFindRow => json!("BHelperFindRow"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1519: Self::BHelperFindExit => json!("BHelperFindExit"),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:438: RuleIdC::BHelperEnvGetEntry => env_get_entry(pre, entry_common(sel, pre)?),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:439: RuleIdC::BHelperEnvGetCell => env_get_cell(pre),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:441: RuleIdC::BHelperEnvGetExit => env_get_exit(pre),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:466: RuleIdC::BHelperFindEntry => find_entry(pre, entry_common(sel, pre)?),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:467: RuleIdC::BHelperFindRow => {`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:475: RuleIdC::BHelperFindExit => find_exit(pre),`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:513: BHelperEnvGetEntry,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:514: BHelperEnvGetCell,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:516: BHelperEnvGetExit,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:524: BHelperFindEntry,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:525: BHelperFindRow,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:527: BHelperFindExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:736: BHelperScalarEntry | BHelperEnvGetEntry | BHelperEnvPutEntry | BHelperFindEntry`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:765: BHelperEnvGetCell => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:781: BHelperEnvGetExit => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:809: BHelperFindRow => match pre.helper() {`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:825: BHelperFindExit => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1316: HelperKind::EnvGet => RuleIdC::BHelperEnvGetEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1318: HelperKind::Find => RuleIdC::BHelperFindEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1331: BHelperScalarEntry | BHelperEnvGetEntry | BHelperEnvPutEntry | BHelperFindEntry`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1334: BHelperScalarExit | BHelperEnvGetExit | BHelperEnvPutExit | BHelperFindExit`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1396: | BHelperEnvGetCell`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1399: | BHelperFindRow`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1453: | BHelperEnvGetCell`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1455: | BHelperFindRow`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1494: BHelperEnvGetEntry | BHelperEnvGetCell | BHelperEnvGetEmpty | BHelperEnvGetExit`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1497: BHelperFindEntry | BHelperFindRow | BHelperFindEmpty | BHelperFindExit`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1600: BHelperEnvGetExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1602: BHelperEnvGetCell,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1607: BHelperFindExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1609: BHelperFindRow,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1841: R::BHelperEnvGetEntry => C::BHelperEnvGetEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1842: R::BHelperEnvGetCell => C::BHelperEnvGetCell,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1844: R::BHelperEnvGetExit => C::BHelperEnvGetExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1852: R::BHelperFindEntry => C::BHelperFindEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1853: R::BHelperFindRow => C::BHelperFindRow,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1855: R::BHelperFindExit => C::BHelperFindExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1980: C::BHelperEnvGetEntry => R::BHelperEnvGetEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1981: C::BHelperEnvGetCell => R::BHelperEnvGetCell,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1983: C::BHelperEnvGetExit => R::BHelperEnvGetExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1991: C::BHelperFindEntry => R::BHelperFindEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1992: C::BHelperFindRow => R::BHelperFindRow,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1994: C::BHelperFindExit => R::BHelperFindExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:444: BHelperEnvGetEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:445: BHelperEnvGetCell,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:447: BHelperEnvGetExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:455: BHelperFindEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:456: BHelperFindRow,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:458: BHelperFindExit,`

## Rule description

C6 call helper step: the Find and EnvGet helper rows factorial and fold
fire (`b.helper.find.{entry,row,exit}`, `b.helper.envget.{entry,cell,exit}`)
as one per-rule relation over the full C6 carriers.

Each circuit fixes its rule at build time, selects it with the composed
dispatch owner and writes the post helper/result records exactly as
`families/environment.rs` plans them:
- Find.entry: Find { common(Invoke), function_name, row_cursor 0, Pending };
- Find.row: compares the PROG function row at row_cursor by name; a match
sets found = row_cursor and Positive, otherwise row_cursor + 1;
- Find.exit: result FindFound { found }, helper cleared;
- EnvGet.entry: EnvGet { common(EvalVar), binder of the Var row, cursor =
env root, Pending };
- EnvGet.cell: reads the DATA env cell at cursor; a binder match sets found
= cell value and Positive, otherwise cursor = cell previous;
- EnvGet.exit: result EnvFound { found }, helper cleared.

Charge is the applied helper charge (work 1, helper work 1, visits 1 for
row/cell, 0 otherwise); the P3 helper-work refusal is rejected, not
constrained. Control, frames, tasks, DATA and every unnamed lane are
preserved; canonicality is composed; enable = 0 is identity.

