# Task: the `c6_append_helper` C6 step relation

Write `crates/mori-backend/src/sherlock_machine/c6_append_helper_step.rs`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: `RuleIdC::BHelperAppendEntry`, `RuleIdC::BHelperAppendEmpty`, `RuleIdC::BHelperAppendTail`, `RuleIdC::BHelperAppendExit`.

## Interface (fixed)

`crates/mori-backend/src/sherlock_machine/c6_append_helper_consumer.rs` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const APPEND_HELPER_RULES: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn append_helper_supported(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct C6AppendHelperEmitter` with
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

- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:361: "BHelperAppendEntry",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:368: "BHelperAppendEmpty",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:375: "BHelperAppendTail",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:382: "BHelperAppendExit",`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1531: Self::BHelperAppendEntry => json!("BHelperAppendEntry"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1533: Self::BHelperAppendEmpty => json!("BHelperAppendEmpty"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1534: Self::BHelperAppendTail => json!("BHelperAppendTail"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1536: Self::BHelperAppendExit => json!("BHelperAppendExit"),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:487: (RuleIdC::BHelperAppendEntry, Some(SiteTag::RetCtorFields)) => {`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:493: RuleIdC::BHelperAppendEntry => append_entry(pre, entry_common(sel, pre)?),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:495: RuleIdC::BHelperAppendEmpty => append_empty(pre),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:496: RuleIdC::BHelperAppendTail => append_tail(pre),`
- `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:498: RuleIdC::BHelperAppendExit => append_exit(pre),`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:539: BHelperAppendEntry,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:541: BHelperAppendEmpty,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:542: BHelperAppendTail,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:544: BHelperAppendExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:489: if selected.rule != RuleIdC::BHelperAppendEntry || selected.site != Some(SiteTag::RetCtorFields)`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:737: | BHelperBindEntry | BHelperAppendEntry | BHelperValidateEntry | BHelperEqualEntry`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:907: BHelperAppendEmpty => {`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:911: BHelperAppendTail => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:918: BHelperAppendExit => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1320: HelperKind::Append => RuleIdC::BHelperAppendEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1332: | BHelperBindEntry | BHelperAppendEntry | BHelperValidateEntry | BHelperEqualEntry`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1335: | BHelperBindExit | BHelperAppendExit | BHelperValidateExit | BHelperEqualExit`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1409: | BHelperAppendEmpty`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1410: | BHelperAppendTail`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1462: | BHelperAppendTail`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1502: BHelperAppendEntry | BHelperAppendCell | BHelperAppendEmpty | BHelperAppendTail`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1503: | BHelperAppendCopy | BHelperAppendExit => FamilyId::Append,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1626: BHelperAppendEmpty,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1627: BHelperAppendTail,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1628: BHelperAppendExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1867: R::BHelperAppendEntry => C::BHelperAppendEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1869: R::BHelperAppendEmpty => C::BHelperAppendEmpty,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1870: R::BHelperAppendTail => C::BHelperAppendTail,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1872: R::BHelperAppendExit => C::BHelperAppendExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2006: C::BHelperAppendEntry => R::BHelperAppendEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2008: C::BHelperAppendEmpty => R::BHelperAppendEmpty,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2009: C::BHelperAppendTail => R::BHelperAppendTail,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2011: C::BHelperAppendExit => R::BHelperAppendExit,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:470: BHelperAppendEntry,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:472: BHelperAppendEmpty,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:473: BHelperAppendTail,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:475: BHelperAppendExit,`

## Rule description

C6 Append helper step: the `b.helper.append.{entry,empty,tail,exit}` rows
factorial and fold fire (argument accumulation at a RetArgs site) as one
per-rule relation over the full C6 carriers.

Each circuit fixes its rule at build time, selects it with the composed
dispatch owner and writes the post state exactly as
`families/environment.rs` plans it:
- entry (RetArgs: control Return { value }, top frame ArgsF { completed }):
Append { common(RetArgs), src = completed, value, src_cursor 0,
dest_start = active value slots, staged None, Copy };
- empty (src_cursor == src.len, nothing staged): phase Tail;
- tail: appends `value` as the next DATA value slot (active value slots
and the DATA value-slot count + 1), phase Done;
- exit: result AppendDone { seq = (dest_start, src.len + 1) }, helper
cleared.

Charge is the applied helper charge (visits 1 for tail, 0 otherwise); the
P3 helper-work refusal is rejected, not constrained. The ADT
RetCtorFields entry (`adt_constructor::append_entry`) and the copy/cell
rows are not covered by this fragment. Every unnamed lane is preserved;
canonicality is composed; enable = 0 is identity.

