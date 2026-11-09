# Task: the `{component}` C6 step relation

Write `{target}`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: {rules}.

## Interface (fixed)

`{consumer}` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const {rules_const}: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn {supported_fn}(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct {emitter}` with
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

{pointers}

## Rule description

{prose}
