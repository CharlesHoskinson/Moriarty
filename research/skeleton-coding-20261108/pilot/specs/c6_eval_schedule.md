# Task: the `c6_eval_schedule` C6 step relation

Write `crates/mori-backend/src/sherlock_machine/c6_eval_schedule_step.rs`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: `RuleIdC::KMachineEvalLiteralValid`, `RuleIdC::KMachineEvalVariablePresent`, `RuleIdC::KMachineEvalBinary`, `RuleIdC::KMachineEvalUnary`, `RuleIdC::KMachineEvalIf`, `RuleIdC::KMachineEvalMatch`, `RuleIdC::KMachineEvalCallArguments`.

## Interface (fixed)

`crates/mori-backend/src/sherlock_machine/c6_eval_schedule_consumer.rs` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const EVAL_SCHEDULE_RULES: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn eval_schedule_supported(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct C6EvalScheduleEmitter` with
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

- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:135: "KMachineEvalLiteralValid",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:142: "KMachineEvalVariablePresent",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:149: "KMachineEvalBinary",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:156: "KMachineEvalUnary",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:163: "KMachineEvalIf",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:170: "KMachineEvalMatch",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:177: "KMachineEvalCallArguments",`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1462: Self::KMachineEvalLiteralValid => json!("KMachineEvalLiteralValid"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1464: Self::KMachineEvalVariablePresent => json!("KMachineEvalVariablePresent"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1467: Self::KMachineEvalBinary => json!("KMachineEvalBinary"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1468: Self::KMachineEvalUnary => json!("KMachineEvalUnary"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1469: Self::KMachineEvalIf => json!("KMachineEvalIf"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1470: Self::KMachineEvalMatch => json!("KMachineEvalMatch"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1472: Self::KMachineEvalCallArguments => json!("KMachineEvalCallArguments"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:2204: events_per_visit: vec![RuleIdC::KMachineEvalMatch, RuleIdC::KMachineCasesTry],`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:2218: "events_per_visit":["KMachineEvalMatch","KMachineCasesTry"]},`
- `crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:3339: // KMachineEvalCallArguments: empty prefix, caller env kept.`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:54: // ROW k.machine.eval.literal.valid`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:55: RuleIdC::KMachineEvalLiteralValid => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:67: // ROW k.machine.eval.variable.present`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:68: RuleIdC::KMachineEvalVariablePresent => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:89: // ROW k.machine.eval.binary`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:90: RuleIdC::KMachineEvalBinary => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:99: // ROW k.machine.eval.unary`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:100: RuleIdC::KMachineEvalUnary => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:108: // ROW k.machine.eval.if`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:109: RuleIdC::KMachineEvalIf => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:118: // ROW k.machine.eval.match`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:119: RuleIdC::KMachineEvalMatch => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:135: // ROW k.machine.eval.call.arguments`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:136: RuleIdC::KMachineEvalCallArguments => {`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:470: KMachineEvalLiteralValid,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:472: KMachineEvalVariablePresent,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:475: KMachineEvalBinary,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:476: KMachineEvalUnary,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:477: KMachineEvalIf,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:478: KMachineEvalMatch,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:480: KMachineEvalCallArguments,`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:859: KMachineEvalLiteralValid => (102, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:860: KMachineEvalVariablePresent => (108, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:863: KMachineEvalBinary => (117, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:864: KMachineEvalUnary => (120, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:865: KMachineEvalIf => (123, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:866: KMachineEvalMatch => (126, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:868: KMachineEvalCallArguments => (131, 1),`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:511: RuleIdC::KMachineEvalLiteralValid`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:513: | RuleIdC::KMachineEvalVariablePresent`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:549: KMachineEvalLiteralValid => matches!(res(pre), Some(HelperResultC::Validated { ok: true })),`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:553: KMachineEvalVariablePresent => matches!(res(pre), Some(HelperResultC::EnvFound { .. })),`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:555: KMachineEvalLet | KMachineEvalBinary | KMachineEvalUnary | KMachineEvalIf`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:556: | KMachineEvalMatch | KMachineReturnLeft | KMachineReturnMatch | KMachineReturnScope => {`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:565: KMachineEvalCallArguments => match pre.control() {`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1347: KMachineEvalLiteralValid`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1349: | KMachineEvalVariablePresent`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1352: | KMachineEvalBinary`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1353: | KMachineEvalUnary`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1354: | KMachineEvalIf`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1355: | KMachineEvalMatch`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1357: | KMachineEvalCallArguments`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1678: EvalLiteral => &[KMachineEvalLiteralValid, KMachineEvalLiteralInvalid],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1679: EvalVar => &[KMachineEvalVariablePresent, KMachineEvalVariableAbsent],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1681: EvalBinary => &[KMachineEvalBinary],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1682: EvalUnary => &[KMachineEvalUnary],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1683: EvalIf => &[KMachineEvalIf],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1684: EvalMatch => &[KMachineEvalMatch],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1685: EvalCall => &[KMachineEvalCallEmpty, KMachineEvalCallArguments],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1798: R::KMachineEvalLiteralValid => C::KMachineEvalLiteralValid,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1800: R::KMachineEvalVariablePresent => C::KMachineEvalVariablePresent,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1803: R::KMachineEvalBinary => C::KMachineEvalBinary,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1804: R::KMachineEvalUnary => C::KMachineEvalUnary,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1805: R::KMachineEvalIf => C::KMachineEvalIf,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1806: R::KMachineEvalMatch => C::KMachineEvalMatch,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1808: R::KMachineEvalCallArguments => C::KMachineEvalCallArguments,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1937: C::KMachineEvalLiteralValid => R::KMachineEvalLiteralValid,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1939: C::KMachineEvalVariablePresent => R::KMachineEvalVariablePresent,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1942: C::KMachineEvalBinary => R::KMachineEvalBinary,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1943: C::KMachineEvalUnary => R::KMachineEvalUnary,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1944: C::KMachineEvalIf => R::KMachineEvalIf,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1945: C::KMachineEvalMatch => R::KMachineEvalMatch,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1947: C::KMachineEvalCallArguments => R::KMachineEvalCallArguments,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3097: RuleIdC::KMachineEvalLiteralValid`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3099: | RuleIdC::KMachineEvalVariablePresent`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:401: KMachineEvalLiteralValid,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:403: KMachineEvalVariablePresent,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:406: KMachineEvalBinary,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:407: KMachineEvalUnary,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:408: KMachineEvalIf,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:409: KMachineEvalMatch,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:411: KMachineEvalCallArguments,`

## Rule description

C6 Eval schedule step: the legacy `k.machine.eval.*` rows that factorial and
tree fold exercise, as one per-rule relation over the full C6 carriers.

Each circuit fixes its rule at build time and:
- selects it with the composed dispatch owner (`c6_dispatch_step`), so the
pre state must make Core select exactly this rule;
- reads the Eval expression row (and, for CallArguments, the first
argument child) from canonical PROG;
- writes the post control exactly as `fragments/legacy.proposed.rs` plans it
(Return, or ScheduleNext with the rule's frame, next and env root);
- clears the helper result where the legacy plan does (Literal and
Variable rules) and preserves every other helper lane;
- charges LegacyComplete (work 1, reference cursor 1) and preserves all
other image, frame and DATA lanes;
- composes META/PROG/CTX/DATA/HELPER canonicality; enable = 0 is identity.

