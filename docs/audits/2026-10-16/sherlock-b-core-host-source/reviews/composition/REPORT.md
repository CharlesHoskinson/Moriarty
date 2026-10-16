SCOPED VERDICT: PASS (source-only, mechanical composition only) — the eleven-method forwarding, module binding, and snapshot membership in the supplied bundle satisfy the stated composition predicate as written, subject to the compiler-needed concerns below. No compilation, execution, security, guard-meaning, selection, StepInv, ABI, or adoption judgment is made or implied.

Top finding first: I found no forwarding mismatch, no accidental self-recursion, no second `FamilyPlans` implementation, and no glue-level recomputation of `Selected` metadata. The remaining risks are ordinary Rust borrow/name-resolution points that only a compiler can settle, plus owner bodies that are absent by design.

## 1. Snapshot and membership closure

- `SOURCE-COMPOSITION.json` as supplied declares SHA256 `5a8950587b7e50029bcb4a049f21ee36763498c07ef9dc2a3aace13fa509553d`, ten files, and 139554 total bytes. The declared per-file byte counts sum to that total (15873+315+2545+315+22406+22227+13694+18821+43046+312 = 139554). I could not recompute hashes — no tool execution was authorized — so byte-identity is checked only against the membership text itself, not independently re-hashed.
- Six exact snapshots are declared and internally consistent between the `files` list and the `exact_snapshots` list: `inputs/raw.rs` (ef9c70a4…), `inputs/shared.rs` (b91a6078…), `fragments/legacy.proposed.rs` (96498b8a…), `fragments/scalar.proposed.rs` (beda415a…), `families/environment.rs` (1d7d7835…), `families/structural.rs` (b356f61e…). The four new Root glue files are `lib.rs`, `families/mod.rs`, `families/legacy.rs`, `families/scalar.rs`, matching the manifest's `new_root_glue`.
- The `FamilyPlans` trait text inside `inputs/shared.rs` contains exactly the eleven methods in the expected contract, each with signature `(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>`.

## 2. Eleven-method forwarding

One implementation only: `impl FamilyPlans for HostFamilyPlans` in `families/mod.rs`. Each method body is a single tail call passing `sel, pre, guards` unchanged, with no site, rule, charge, cap, or guard recomputation in the glue:

- `plan_legacy` → `legacy::dispatch(sel, pre, guards)` → fragment free fn `plan_legacy` — matches expected.
- `plan_scalar` → `scalar::dispatch(sel, pre, guards)` → fragment free fn `plan_scalar` — matches expected.
- `plan_env_get` → `environment::plan_env_get` — matches.
- `plan_env_put` → `environment::plan_env_put` — matches.
- `plan_find` → `environment::plan_find` — matches.
- `plan_bind` → `structural::plan_bind` — matches.
- `plan_append` → `environment::plan_append` — matches.
- `plan_validate` → `structural::plan_validate` — matches.
- `plan_equal` → `structural::plan_equal` — matches.
- `plan_match` → `structural::plan_match` — matches.
- `plan_bookkeeping_release` → `environment::plan_bookkeeping_release` — matches.

No method is duplicated, omitted, or routed to a wrong family. Parameter names differ from the trait (`guards` vs `g`), which Rust permits in trait implementations.

## 3. Module, import, and visibility closure

- `lib.rs`: `#[path = "inputs/raw.rs"] pub mod raw;`, `#[path = "inputs/shared.rs"] pub mod shared;`, private `mod families;`, and `pub use families::HostFamilyPlans;`. Because both modules sit at crate root, `shared.rs`'s `use super::raw::{ConstRow as RawConstRow, DataRow as RawDataRow, RuleId as RawRuleId};` resolves to `crate::raw`. Correct in this layout.
- `families/mod.rs` uses `use crate::shared::*;` and declares private child modules `legacy`, `scalar`, `environment`, `structural`. `families/environment.rs` and `families/structural.rs` use `super::super::shared`, which from `crate::families::<file>` also resolves to `crate::shared`. Two path styles coexist but both resolve under the actual placement.
- `families/legacy.rs` / `families/scalar.rs`: `use crate::shared::*;` then `include!("../fragments/legacy.proposed.rs")` (resp. scalar), which resolves relative to `families/` to the declared fragment paths. The included free functions `plan_legacy` / `plan_scalar` are private to their adapter module; `dispatch` is `pub(super)`, i.e. visible to `families/mod.rs` and nowhere wider. No self-recursion: `dispatch` and `plan_legacy` are distinct names, and the unqualified call cannot resolve to the trait method (no receiver).
- Privacy factoring is preserved: `families` is a private module, so the `pub fn plan_*` functions in `environment.rs`/`structural.rs` are effectively crate-internal; only `HostFamilyPlans` is re-exported. Family modules are siblings of `shared`, not descendants, and touch `shared` only through its public items and the `PreImageView`/`GuardEvaluator` trait objects. Private carriers (`AdmittedBody`, `MachineImage` fields, `PreImage` fields, `ValidatedPlan::from_validated`, `OwnerGuards::new_owner`) remain unconstructible from family code as written.
- Foreign-rule literal coherence: `environment::foreign()`, `structural::foreign_rule()`, and the legacy/scalar fallback arms all construct `HostViolation { kind: WrongVariant, clause: None, rule: None, locator: Some("foreign rule for family") }`, matching the normative literal. Legacy/scalar spell it via their local `*_payload_error(WrongVariant, "foreign rule for family")` helpers; the produced value is the same.

## 4. Row-coverage claim check (against the supplied literal maps, not the sum)

- Legacy fragment: 37 match arms, `KMachineEvalLiteralValid` through `KMachinePrimitiveError`, agreeing with the 37 entries in `ROW-FIELD-MAPPING.json`.
- Scalar fragment: 6 arms (`BHelperScalarEntry/Tag/Arithmetic/Construct/Exit/Failure`), agreeing with the same map.
- Environment: EnvGet 5 + EnvPut 3 enabled (Entry/Insert/Exit) + Find 5 + Append 6 + `BBookkeepingTaskRelease` 1 = 20, agreeing with `ROW-POINTER-MAP.json`. Disabled EnvPut Cell/Empty/Failure and Append Failure have no branch and fall to the foreign literal.
- Structural: Bind 10 + Validate 7 enabled + Equal 6 enabled + Match 10 enabled = 33, agreeing with `ROW-EFFECT-MAPPING.json`. `BHelperValidateDeclaration`, `BHelperMatchDiscard` (ExtensionOnly), `BHelperEqualFailure`, `BHelperMatchEmpty` (Disabled) have no branch and fall to the foreign literal.
- Total enabled branches present: 96. I did not reclassify any Disabled/ExtensionOnly row; the absence of those branches is an owner-selector seam (selection must never present them), not a forwarding defect. Note for the record: if a Disabled/ExtensionOnly `RuleIdC` were ever presented to these functions, the composition returns the foreign-rule `WrongVariant` literal rather than `DisabledRuleReached`/`ExtensionRuleReached` — consistent with the supplied `shared.rs` family contract text, but worth Root's awareness since it originates in the worker fragments, not the glue.

## 5. Findings

F-1 (Medium, compiler-needed, origin: unchanged worker snapshot `families/structural.rs`). Borrow/move patterns that a compiler must settle, e.g. in `plan_validate_nonentry` and `plan_equal_nonentry`: a `status` reference is taken via `let HelperControlC::Validate { status, .. } = &mut helper`, mutated (`*status = TriStatus::Positive`), and then `helper` is moved into `Write::Set(Some(helper))` inside the same match arm. Similar shapes appear in `plan_match_postarm` (many `&mut` field bindings, then `Write::Set(Some(helper))` after the match) and in `scalar.proposed.rs` `BHelperScalarTag` TestType arm (`*staged = … pre.scalar_test_type_ok(operand_a, expected)?` with `operand_a`, `expected`, `staged` all borrowed from one `&mut h` pattern). These may well pass under disjoint-field borrow rules, but I cannot certify them without `rustc`.

F-2 (Medium, compiler-needed, origin: unchanged worker/shared snapshots). Several let-else and match-ergonomics forms need compiler confirmation: e.g. `environment.rs` `let id = cursor.ok_or_else(|| wrong("EnvGet.cell cursor absent"))?;` where `cursor` is a `&mut Option<usize>` binding from a `&mut h` destructure; `structural.rs` `pre.field_at(scrutinee, &usize::from(*field_idx))?` passing a temporary reference; and `&mut RefC` bindings passed where `&RefC` is expected (`kind_of`, `scalar_test_type_ok`). Each is plausibly fine under autoderef/reborrow rules; none is verifiable by reading alone.

F-3 (Low, glue/layout fragility). The composition binds the crate-root layout declared in `lib.rs`, not the `crates/mori-core/src/sherlock/helper_machine/` layout proposed in the `shared.rs` header comment. If Root later moves these files under a `helper_machine` parent, `families/legacy.rs`/`scalar.rs`'s `use crate::shared::*;` and the `../fragments/…` include paths break and would need re-pointing. Not a defect in the frozen candidate; a constraint on future relocation.

F-4 (Low, cosmetic). Mixed module-path styles (`crate::shared` in glue/fragments vs `super::super::shared` in environment/structural) and `use RuleIdC as RuleId;` plus inner `use RuleId::*;` / `use RuleIdC::*;` in `structural.rs` are all resolvable as written but make the binding harder to audit mechanically. No collision found: `RuleIdC` variants are the only names imported by those inner uses.

F-5 (Informational, origin: unchanged `inputs/shared.rs`). `StepEventC` as supplied derives `Clone, PartialEq, Eq, Debug` (no `Copy`), consistent with the owner errata; its fields (`Arc<RunIdC>`, `String`, `Vec`) support those derives. The errata's other items (foreign-rule comment text) are also reflected in the supplied text. No event-policy body is implemented anywhere in the composition, as required.

No High-severity composition defect found in the glue itself.

## 6. Missing bodies versus defects

Absent by design (declared owner seams, not defects): any `PreImageView` implementation for `PreImage`, any `GuardEvaluator` implementation (`OwnerGuards` has no body here), every `OwnerEngine` method (selector, `validate_plan`, `cap_refusal`, `physical_check`, `apply_validated`, `step`, `run`, `check_step_inv`, admission), and therefore any driver, charge/cap application, or `apply` path. Consequence: the crate as composed declares the full contract and one forwarding implementation, but nothing in it can yet be executed against a real image; that is a coverage limit of the candidate, not a glue error. Nothing in the four glue files fabricates a substitute body.

## 7. Compiler-needed concerns (summary)

No compiler, test, or version run was performed or authorized. Before any adoption, a compiler pass must at minimum settle: F-1/F-2 borrow and reborrow forms; let-else pattern ergonomics against `Option<&FrameRowC>`/`&ControlC` in environment and structural entry payloads; the `usize::from(*field_idx)` temporary in `plan_match_postarm`; glob-import name resolution under `use crate::shared::*;` in the two fragment adapters (notably `pure_leaves::checked_add`/`CheckedArith` in scalar, and `FrameWrite`, `BoundBinding`, `CaseMatching` in legacy); and unused-import warnings (warnings only). Security-owned preflight for any such compiler diagnosis is a separate task; I make no judgment on its authority or policy.

## 8. Identity, dissent, and unchanged gates

Reviewer identity: requested and configured as muse-wonder-1.0 at HIGH reasoning effort; no backend attestation is available or claimed. This is a fresh, non-author, nonsecurity review; the three family-lane audits were treated as scoped to their own sources and not as approval of this composition.

Unchanged acceptance state, none of which this report alters: qualified R4 HOST Stage1 adoption remains a normative-source adoption distinct from the proposed v3a Rust rendering, which remains unadopted (`adopted: false`, `compiled: false`, `native_authority: 0` in the v3a membership); I-B-EVENT adoption remains pending its separate Root decision; the disabled/extension row dispositions, owner errata scope (StepEvent Copy/foreign comments plus event-side text only), and all parent phase gates remain exactly as supplied. Dissent recorded in the underlying R4 materials is preserved unaltered and is not adjudicated here.

Tail canary, quoted to show full bundle coverage: `MUSE_ROOT_COMPOSITION_AUDIT_FULL_TAIL_20261016`
