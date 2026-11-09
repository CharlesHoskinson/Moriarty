You are a skeleton-author lane in a controlled experiment. Work only inside the
current directory. It is a copy of a Rust workspace with one file removed.

# Task: the `c6_admin_publish` C6 step relation

Write `crates/mori-backend/src/sherlock_machine/c6_admin_publish_step.rs`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: `RuleIdC::KAdminPublishValue`.

## Interface (fixed)

`crates/mori-backend/src/sherlock_machine/c6_admin_publish_consumer.rs` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const ADMIN_PUBLISH_RULES: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn admin_publish_supported(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct C6AdminPublishEmitter` with
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

- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:121: rule: "KAdminPublishValue",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:131: names: "k.admin.publish.value",`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1571: Self::KAdminPublishValue => json!("KAdminPublishValue"),`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:579: KAdminPublishValue,`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:895: KAdminPublishValue => (80, 0),`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1345: KAdminPublishValue | KAdminPublishError | KAdminPublishExhausted => EventKind::Publish,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1784: | RuleIdC::KAdminPublishValue`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1907: R::KAdminPublishValue => C::KAdminPublishValue,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2046: C::KAdminPublishValue => R::KAdminPublishValue,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3730: RuleIdC::KAdminPublishValue,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:510: KAdminPublishValue,`

## Rule description

C6 admin publish step: the `k.admin.publish.value` row every
factorial and tree fold run fires last, as one per-rule relation
over the full C6 carriers.

The circuit fixes its rule at build time and selects it with the
composed dispatch owner (P1 publication: outcome Running, helper
and result absent, control `Return { value }`, depth 0). The only
write is the outcome: `Halted { value }` (image lanes 23-25), with
the value reference copied from the preserved Return control.
Charge is 0 (publication fires before the P2 fuel refusal, so a
zero remaining-work pre-state is accepted); every other image,
DATA and helper lane is preserved exactly. Canonicality is
composed; enable = 0 is identity.


## Your job

Write a skeleton (not an implementation) of `crates/mori-backend/src/sherlock_machine/c6_admin_publish_step.rs`. Another lane will
implement it later from your skeleton and the task above.

The reference implementation of this file is in
`reference/c6_admin_publish_step.rs`. Derive the skeleton from it: keep its
public items, its split into functions, the statement lanes each part reads
and writes, and state each contract from what the reference body enforces.
Do not copy body code.

The skeleton contains:

1. Every public item of the interface, with its real signature.
2. The private functions you would split the relation into, with real
   signatures and `todo!()` bodies. One function per independent concern
   (for example: selection, each written lane group, preservation, canonical
   checks, the disabled identity).
3. Statement-lane stubs: named constants or comments that list which
   `C6StepInputs` lanes each function reads and which post lanes it fixes.
4. A contract on each function as doc comments: `Requires:` (what callers
   guarantee), `Ensures:` (what every satisfying assignment then satisfies),
   and `Check:` (which reference-machine line, from the pointers above, states
   the rule this function encodes).
5. A module-level `Global invariants:` list: the properties the whole
   relation must have that no single function owns (for example: every post
   lane is either written by a rule or preserved; enable = 0 is identity).

Constraints:

- Bodies are `todo!()` only. Do not write any gate, `em.*` call or
  instruction in a body.
- The file must pass `cargo check -p mori-backend --lib --offline`
  (unused-variable warnings are fine). You may run that command, and
  `cargo fmt`, nothing else.
- Change only `crates/mori-backend/src/sherlock_machine/c6_admin_publish_step.rs`. Do not read or search outside the current
  directory, and do not use git or the network.
- Keep it under about 250 lines. End with one line: the number of functions
  and the number of global invariants.
