# Task: the `c6_scalar_construct` C6 step relation

Write `crates/mori-backend/src/sherlock_machine/c6_scalar_construct_step.rs`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: `RuleIdC::BHelperScalarConstruct`.

## Interface (fixed)

`crates/mori-backend/src/sherlock_machine/c6_scalar_construct_consumer.rs` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const SCALAR_CONSTRUCT_RULES: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn scalar_construct_supported(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct C6ScalarConstructEmitter` with
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

- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:220: "BHelperScalarConstruct",`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1502: Self::BHelperScalarConstruct => json!("BHelperScalarConstruct"),`
- `crates/mori-core/src/sherlock/helper_machine/fragments/scalar.proposed.rs:188: // ROW b.helper.Scalar.construct`
- `crates/mori-core/src/sherlock/helper_machine/fragments/scalar.proposed.rs:189: RuleIdC::BHelperScalarConstruct => {`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:510: BHelperScalarConstruct,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:748: BHelperScalarConstruct => {`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1395: | BHelperScalarConstruct`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1452: | BHelperScalarConstruct`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1491: | BHelperScalarConstruct`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1593: BHelperScalarConstruct,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1838: R::BHelperScalarConstruct => C::BHelperScalarConstruct,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1977: C::BHelperScalarConstruct => R::BHelperScalarConstruct,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:441: BHelperScalarConstruct,`

## Rule description

C6 Scalar construct step: the `b.helper.scalar.construct` row
factorial/tree-fold-style programs fire while building `leaf(..)`,
`cons(..)` and `branch(..)` values, as one per-rule relation over the
full C6 carriers.

The circuit fixes its rule at build time, selects it with the composed
dispatch owner and writes the post state exactly as
`fragments/scalar.proposed.rs` (row `b.helper.Scalar.construct`) plans
it. The Scalar helper (kind 1) carries mode (lane 5), operand_a
(6-7), operand_b (8-10), phase (13) and staged (15-16):
- Leaf at phase First: `kind_of(operand_a) == UInt` stages
`PendingNode { Leaf }`, anything else stages `PrimError { 11 }`;
- Cons at phase Second: `kind_of(operand_b)` in {Nil, Cons} stages
`PendingNode { Cons }`, anything else `PrimError { 11 }`;
- Branch at phase Second: `kind_of(operand_b)` in {Leaf, Branch}
stages `PendingNode { Branch }`, anything else `PrimError { 11 }`.
Every construct step ends at phase Done. Kinds come from the checked
Const/DATA value read of the operand itself, never from a host lane.

Charge is the applied helper charge (visits 1). Not constrained here,
so rejected: the P3 helper-work refusal. Every unnamed lane is
preserved; canonicality is composed; enable = 0 is identity.

