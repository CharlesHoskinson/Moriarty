You are a builder lane in a controlled experiment. Work only inside the current
directory. It is a copy of a Rust workspace with one file removed.

# Task: the `c6_call_bind` C6 step relation

Write `crates/mori-backend/src/sherlock_machine/c6_call_bind_step.rs`. It emits the ZKIRv3 instructions of one per-rule C6 step
relation. The relation covers these rules of the Sherlock owner machine: `RuleIdC::KMachineInvokeFind`, `RuleIdC::KMachineEnterBind`, `RuleIdC::KMachineBoundValid`.

## Interface (fixed)

`crates/mori-backend/src/sherlock_machine/c6_call_bind_consumer.rs` builds and checks the circuit. It must compile against your file
without changes. Your file must export:

- `pub const CALL_BIND_RULES: [RuleIdC; N]`: exactly the rules above, in that order.
- `pub fn call_bind_supported(rule: RuleIdC) -> bool`: true exactly for those rules.
- `pub struct C6CallBindEmitter` with
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

- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:256: "KMachineInvokeFind",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:263: "KMachineEnterBind",`
- `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:270: "KMachineBoundValid",`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1488: Self::KMachineInvokeFind => json!("KMachineInvokeFind"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1490: Self::KMachineEnterBind => json!("KMachineEnterBind"),`
- `crates/mori-core/src/sherlock/compile_artifact/wire.rs:1492: Self::KMachineBoundValid => json!("KMachineBoundValid"),`
- `crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:2170: /// The call gives genuine ArgsF and nominal ReturnF pushes. `KMachineBoundValid``
- `crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:3351: // KMachineBoundValid: nominal Pair return, callee env replaces caller env.`
- `crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:3421: "a nominal ReturnF push scheduled by KMachineBoundValid"`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:289: // ROW k.machine.invoke.find`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:290: RuleIdC::KMachineInvokeFind => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:301: // ROW k.machine.enter.bind`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:302: RuleIdC::KMachineEnterBind => {`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:315: // ROW k.machine.bound.valid`
- `crates/mori-core/src/sherlock/helper_machine/fragments/legacy.proposed.rs:316: RuleIdC::KMachineBoundValid => {`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:496: KMachineInvokeFind,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:498: KMachineEnterBind,`
- `crates/mori-core/src/sherlock/helper_machine/raw.rs:500: KMachineBoundValid,`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:884: KMachineInvokeFind => (200, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:886: KMachineEnterBind => (204, 1),`
- `crates/mori-core/src/sherlock/helper_machine/reference_projection.rs:888: KMachineBoundValid => (208, 1),`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:525: | RuleIdC::KMachineInvokeFind`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:526: | RuleIdC::KMachineEnterBind`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:618: KMachineInvokeFind => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:623: KMachineEnterBind => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:634: KMachineBoundValid => matches!(`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1373: | KMachineInvokeFind`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1375: | KMachineEnterBind`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1377: | KMachineBoundValid`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1708: Invoke => &[KMachineInvokeFind],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1709: Enter => &[KMachineEnterMissing, KMachineEnterBind],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1710: Bound => &[KMachineBoundInvalid, KMachineBoundValid],`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1824: R::KMachineInvokeFind => C::KMachineInvokeFind,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1826: R::KMachineEnterBind => C::KMachineEnterBind,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1828: R::KMachineBoundValid => C::KMachineBoundValid,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1963: C::KMachineInvokeFind => R::KMachineInvokeFind,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1965: C::KMachineEnterBind => R::KMachineEnterBind,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:1967: C::KMachineBoundValid => R::KMachineBoundValid,`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3111: | RuleIdC::KMachineInvokeFind`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:3112: | RuleIdC::KMachineEnterBind`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:5825: let sel = fixture_selected(RuleIdC::KMachineInvokeFind, None);`
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:6013: &fixture_selected(RuleIdC::KMachineInvokeFind, None),`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:427: KMachineInvokeFind,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:429: KMachineEnterBind,`
- `crates/mori-core/src/sherlock/helper_machine/shared.rs:431: KMachineBoundValid,`

## Rule description

C6 CallBind control step: the legacy `k.machine.invoke.find`,
`k.machine.enter.bind` and `k.machine.bound.valid` rows as one per-rule
relation over the full C6 carriers (the rows factorial and fold fire).

Each circuit fixes its rule at build time, selects it with the composed
dispatch owner, writes the post control exactly as
`fragments/legacy.proposed.rs` plans it, clears the helper result where
the plan does (InvokeFind, EnterBind), charges LegacyComplete and preserves
every other image, frame, DATA and helper lane. EnterBind reads the entered
function's row (return type, body) from canonical PROG. Canonicality of
META/PROG/CTX/DATA/HELPER is composed; enable = 0 is identity.


## Skeleton

`crates/mori-backend/src/sherlock_machine/c6_call_bind_step.rs` already holds a skeleton: signatures, statement-lane stubs,
per-function contracts (`Requires` / `Ensures`) and a list of global
invariants. Bodies are `todo!()`. Implement the bodies so that every contract
and every global invariant holds. You may add private helpers. Change a
signature only if the interface above forces it, and say so in your summary.

## Rules of this lane

- Change only `crates/mori-backend/src/sherlock_machine/c6_call_bind_step.rs`. Changes to any other file are discarded.
- You may read any file in the current directory.
- You may run `cargo check -p mori-backend --lib --offline`,
  `cargo clippy -p mori-backend --no-deps --lib --offline` and `cargo fmt`.
  Do not run `cargo test` (the circuit tests need about 11 GB per thread).
- Do not read or search outside the current directory. Do not use git, the
  network or any other copy of this repository. Do not look for an existing
  implementation of this file; none is available to you.
- Finish when `cargo check -p mori-backend --lib --offline` passes and you
  believe the relation is complete and sound. End with a short summary: what
  the relation constrains, and anything you are unsure of.
