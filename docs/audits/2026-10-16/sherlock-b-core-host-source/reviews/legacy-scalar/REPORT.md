# Independent Source Review — Legacy and Scalar Row Effects

## Scope and identity limitation

Reviewer: fresh independent non-author `muse-wonder-1.0`, high-effort, non-security source review.

Reviewed as supplied text only:

- `legacy.proposed.rs` — private `plan_legacy`, 37 legacy rows.
- `scalar.proposed.rs` — private `plan_scalar`, 6 Scalar rows.
- `ROW-FIELD-MAPPING.json`, `OPEN-INTERFACES.json`, `SOURCE-INSPECTION.json`, `REPORT.md`, candidate metadata.
- Normative comparison basis: the supplied HOST R4 row effects for the 37 legacy rows and 6 Scalar rows.
- Rust comparison basis: the supplied proposed v3a shared interface, including `Selected`, `PreImageView`, `GuardEvaluator`, `EffectPlan`, `PlannedWrites`, `PlanDelta`, `Write`, `FrameWrite`, `AllocDataWrite`, carriers, accessors, and `pure_leaves::CheckedArith`.

This is a source-text review of uninstalled fragments. Root placement, imports/scope, the sole delegating `FamilyPlans` implementor, owner engine bodies, and adoption of the shared contract are explicitly deferred and are not counted as authored-body defects.

No tools, compiler, Rust tests, SDK, native process, or execution were used. Candidate byte counts and hashes are therefore taken as supplied labels, not independently recomputed in this review. All Rust correctness statements below are source-level inferences from the supplied declarations, not compiler observations.

No security judgment is made. This report does not certify selection, admission, charging, capacity, refusal policy, plan validation, `StepInv`, guards as security policy, or engine correctness.

## Verdict

**PASS — source-only**

- `plan_legacy`: **PASS** for all 37 rows.
- `plan_scalar`: **PASS** for all 6 rows.

This PASS applies only to the two private row-effect function bodies against the supplied R4 row effects and proposed v3a interface. It does not approve adoption of the shared contract, Root integration, compilation, runtime behaviour, or any owner/security mechanism.

## Findings

**No findings.**

Explicitly, within the authorized source-only scope I found none of the following in either body:

1. No omitted normative row.
2. No extra authored row branch claiming an unlisted row effect.
3. No wrong control, frame, environment, helper, result, allocation, or origin payload field.
4. No wrong error code in a produced row payload.
5. No wrong depth or allocation delta assertion.
6. No missing `clearResult` where R4 specifies it, and no added `clearResult` where R4 omits it.
7. No incorrect foreign-family error rendering.
8. No recomputation of `Selected`, site, helper plan, event metadata, or `select_when`.
9. No fabricated payload default.
10. No direct unchecked table indexing or table escape.
11. No loop, arena allocation, counter mutation, index mutation, `MachineImage` mutation, charge/cap/refusal/policy logic, or owner-engine behaviour in these bodies.

## Direct row coverage

### Legacy — 37/37 rows verified

| Rows | Direct source check |
|---|---|
| `k.machine.eval.literal.valid`, `k.machine.eval.literal.invalid` | Valid row reads the checked expression row and produces `Return{Const(constant)}`; invalid row produces `Error{2}`. Both clear result. Depth remains 0. |
| `k.machine.eval.variable.present`, `k.machine.eval.variable.absent` | Present consumes `EnvFound.value` into `Return`; absent produces `Error{3}`. Both clear result. |
| `k.machine.eval.let` | Produces `ScheduleNext` with `LetF{binder, body, saved_env: pre.env_root(), origin}`, `next = value`, and `env = pre.env_root()`. No result clear and no depth change. |
| `k.machine.eval.binary` | Produces `LeftF{op, right, saved_env, origin}`, schedules `left`, and preserves the current environment. |
| `k.machine.eval.unary` | Produces `UnaryF{op, origin}` and schedules `operand`. The frame shape matches the supplied carrier, which has no saved environment for `UnaryF`. |
| `k.machine.eval.if` | Produces `IfF{then_expr, else_expr, saved_env, origin}` and schedules `cond`. |
| `k.machine.eval.match` | Produces `MatchF{arms, saved_env, origin}` and schedules `scrutinee`. |
| `k.machine.eval.call.empty` | Produces `Invoke{function_name, args: SliceC::empty()}`. The empty slice is the contract representation `(0,0)`. |
| `k.machine.eval.call.arguments` | Produces `ArgsF` with empty completed slice, checked tail of `args`, saved environment, and origin. The next expression is obtained through checked `pre.child(&args.start)`. |
| `k.machine.push.accept` | Pushes the staged frame, sets environment from `ScheduleNext.env`, sets `Eval{next}`, sets control origin to `next`, and asserts `depth = +1`. |
| `k.machine.push.refuse` | Produces only `Error{4}`. It does not pop, push, clear result, or change depth. |
| `k.machine.return.let` | Rewrites the top frame to `ScopeF{saved_env, origin}`, sets environment to `Some(EnvUpdated.cell)`, schedules `body`, sets origin to `body`, clears result, and keeps depth unchanged. |
| `k.machine.return.left` | Rewrites the top frame to `RightF{op, left: Return.value, origin}`, restores saved environment, schedules `right`, and sets origin to `right`. No result clear is specified or produced. |
| `k.machine.return.right` | Pops, produces `PrimResult` from `ScalarError`, `ScalarValue`, or `EqualDone`, sets origin from the frame, clears result, and asserts `depth = -1`. |
| `k.machine.return.unary` | Pops, produces `PrimResult` through the same scalar-result conversion, sets origin, clears result, and asserts `depth = -1`. The shared conversion also syntactically accepts `EqualDone`; under the authoritative `Selected`/site result membership for `RetUnary`, that extra accepted shape is not a produced row payload for the normative row input. |
| `k.machine.return.if.true`, `k.machine.return.if.false` | Each pops, restores saved environment, schedules the selected branch expression, sets origin to that expression, clears result, and asserts `depth = -1`. |
| `k.machine.return.if.invalid` | Produces `Error{5}` and clears result. It does not pop and does not change depth. |
| `k.machine.return.match` | Pops, restores saved environment, produces `Cases{value, arms, saved_env}`, sets origin from the frame, and asserts `depth = -1`. No result clear is specified or produced. |
| `k.machine.return.arguments.next` | Rewrites `ArgsF` using `AppendDone.seq` as completed, checked tail of `rest`, saved environment, and origin. It restores saved environment, schedules checked `child(rest.start)`, clears result, and keeps depth unchanged. |
| `k.machine.return.arguments.final` | Pops, restores saved environment, produces `Invoke{function_name, args: AppendDone.seq}`, sets origin, clears result, and asserts `depth = -1`. |
| `k.machine.return.call.valid` | Pops, restores saved environment, sets control origin, clears result, asserts `depth = -1`, and leaves `writes.control` unassigned. Under the supplied default semantics that is `Write::Keep`, as required. |
| `k.machine.return.call.invalid` | Produces `Error{6}` and clears result. It does not pop and does not change depth. |
| `k.machine.return.scope` | Pops, restores saved environment, sets control origin, asserts `depth = -1`, and leaves control unassigned as `Keep`. It does not clear result, matching the row effect list. |
| `k.machine.invoke.find` | Converts `FindFound(function)` to `Some(function)` and `FindMissing` to `None`, preserves the complete invocation argument slice unchanged, produces `Enter`, and clears result. |
| `k.machine.enter.missing` | Produces only `Error{7}`. |
| `k.machine.enter.bind` | Reads the function through checked `pre.function_row(&function)`. It maps `BindOk{env}` to `BoundBinding::Ok{env}` without unwrapping the environment option, maps `BindBad` to `Bad`, copies `return_type` and `body` from the function row, and clears result. An `Ok` binding with `env = None` remains distinct from `Bad`. |
| `k.machine.bound.invalid` | Produces only `Error{8}`. |
| `k.machine.bound.valid` | Produces `ScheduleNext` with `ReturnF{return_type, saved_env: pre.env_root(), origin: pre.control_origin()}`, schedules `body`, and uses the positive binding environment unchanged, including `None`. |
| `k.machine.cases.empty` | Produces only `Error{9}`. |
| `k.machine.cases.try` | Produces `CaseResult` with matching converted from `MatchMatched`/`MatchNoMatch`, preserving a matched environment including `None`; body is read through checked `pre.arm(&arms.start)`; the rest slice is the checked normalized tail; result is cleared. |
| `k.machine.cases.miss` | Re-enters `Cases` using the stored value, `CaseResult.rest` as the complete remaining arm slice, and saved environment. |
| `k.machine.cases.hit` | Produces `ScheduleNext` with `ScopeF{saved_env, origin: pre.control_origin()}`, schedules the stored body, and uses the matched environment unchanged, including `None`. |
| `k.machine.primitive.value` | Produces `Return{value}` from `PrimOutcome::Value`. |
| `k.machine.primitive.error` | Produces `Error{code}` from `PrimOutcome::Error`, preserving the carried code. |
| Foreign legacy rule | Falls through to exactly `HostViolationKind::WrongVariant`, `clause: None`, `rule: None`, `locator: Some("foreign rule for family")`. |

### Scalar — 6/6 rows verified

| Row | Direct source check |
|---|---|
| `b.helper.Scalar.entry` | Uses `sel.site` as authoritative and does not recompute the site. It binds `pre.reference_cursor()` into `HelperCommon.origin_reference_cursor`, sets `parent_resume: None`, `phase: First`, `reg_a: 0`, and `staged: ScalarStaged::None`. `RetRight` uses the right-frame operator and left operand, with `operand_b = Some(Return.value)`. `RetUnary` maps `Leaf`/`Not`; `RetIf` uses `TestBool`; `RetCall` uses `TestType` with `expected = Some(ReturnF.return_type)`. An `Eq` right operator is rejected as an undefined Scalar entry payload rather than converted into a Scalar mode. |
| `b.helper.Scalar.tag` | Copies the pre-image Scalar helper and patches only the row-specified fields. Arithmetic/`Lt` and `Cons` first-operand checks use `LogicalKind::UInt`; `Branch` accepts `Leaf`/`Branch`; mismatches stage `PrimError{11}` and `Done`. Successful first binary operands move to `Second`, with `reg_a` set only for arithmetic/`Lt`. `TestBool` stages the exact `True`/`False`/`NotBool` selector. `TestType` calls `pre.scalar_test_type_ok(operand_a, expected)` and does not convert a missing `Option` to `false`. `Leaf`/`Not` are rejected at this row rather than silently handled. |
| `b.helper.Scalar.arithmetic` | `Not` uses the Boolean payload or stages code `11`. Binary modes require `operand_b`, reject a non-`UInt` second operand with code `11`, implement `Lt` as `reg_a < u128(b)`, and use checked addition/subtraction/multiplication for the other modes. `pure_leaves::CheckedArith::Overflow` maps only to code `10`; a checked value maps to `U128Value`. Phase becomes `Done`. |
| `b.helper.Scalar.construct` | `Leaf` requires a `UInt` first operand; `Cons` requires the second operand kind to be `Nil` or `Cons`; `Branch` requires `Leaf` or `Branch`. Success stages the corresponding `PendingNode`; failure stages code `11`. Phase becomes `Done`. Other modes are rejected for this row. |
| `b.helper.Scalar.exit` | Queries exactly `GScalarAllocNode`, then `GScalarFields1`, then `GScalarFields2`, in that order. It asserts node delta `1/0`, field delta from `1/0 + 2/0`, stages the typed allocation write, sets the result, and clears the helper. |
| `b.helper.Scalar.failure` | Extracts the exact `PrimError.code`, produces `HelperResultC::ScalarError{code}`, and clears the helper. It does not substitute code `10` or `11`. |
| Foreign Scalar rule | Falls through to exactly `HostViolationKind::WrongVariant`, `clause: None`, `rule: None`, `locator: Some("foreign rule for family")`. |

## Ownership and method/type review

The two authored functions have the owner-authorized private free-function shape:

- `plan_legacy(&Selected, &dyn PreImageView, &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>`
- `plan_scalar(&Selected, &dyn PreImageView, &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>`

They do not claim to implement `FamilyPlans` themselves. Delegation by Root’s sole implementor is future integration, not a missing body in these fragments.

Ownership is consistent with the supplied interface:

- Inputs are borrowed; no `MachineImage`, `Admitted`, program table, or arena is owned or mutated.
- Copy carriers such as `RefC`, `SliceC`, frame rows, control rows, helper rows, and result rows are copied by value where the interface derives `Copy`.
- Scalar tag/arithmetic/construct copy the complete pre-image helper first, patch only authorized fields, and then write the whole helper back. Unpatched helper fields are therefore preserved rather than reconstructed.
- Complete slices are passed unchanged where required: invocation arguments, append-completed sequences, and append result sequences are copied as whole `SliceC`/`ValSliceC` values.
- Positive optional environments are not unwrapped: `BoundBinding::Ok{env}`, `CaseMatching::Matched{env}`, bound-valid scheduling, and case-hit scheduling preserve `env = None`.

Method usage matches the supplied v3a declarations at source level:

- Legacy checked reads use `pre.expr`, `pre.child`, `pre.function_row`, and `pre.arm`.
- Other legacy reads use borrowed accessors such as `pre.control`, `pre.result`, `pre.top_frame`, `pre.env_root`, and `pre.control_origin`.
- Scalar entry uses `pre.reference_cursor`.
- Scalar value inspection uses `pre.kind_of`, `pre.u128_of`, and `pre.bool_of`.
- Scalar `TestType` uses the sole supplied seam `pre.scalar_test_type_ok`.
- Scalar exit is the only body using `eval_guard`, and it uses exactly the three supplied `GScalar*` identifiers in the required order.
- Slice tails use checked `+1` and checked `-1`, followed by `SliceC::normalized`, which maps an empty result to `(0,0)`.
- Arithmetic uses `pure_leaves::checked_add`, `checked_sub`, and `checked_mul`, and matches the supplied `CheckedArith::{Value, Overflow}` carrier.

No authored body calls selection, helper-plan derivation, admission, plan validation, charging, capacity, physical-check, apply, outcome-publication, or `StepInv` methods. Legacy ignores the guard evaluator; Scalar uses it only for the exit delta assertions. That matches the limited role assigned to these fragments.

The unqualified names and `pure_leaves::...` paths depend on Root supplying the shared imports/scope, exactly as the candidate states. That is a future placement obligation, not an authored-body defect under the review instruction.

## Write, delta, and field review

`EffectPlan` is used as a payload-write plus delta-assertion carrier only.

For both bodies:

- `PlannedWrites::default()` supplies `Write::Keep` for written-register fields and `None` for optional write slots.
- `PlanDelta::default()` supplies zero for all six assertion fields.
- Those defaults are the supplied contract’s literal no-write/zero representations. They are not treated here as fabricated payload data.

Legacy write behaviour is exact:

- All specified `set` effects are represented as `Write::Set`.
- Frame pushes, rewrites, and pops use `FrameWrite::Push`, `RewriteTop`, and `Pop`.
- `return.call.valid` and `return.scope` correctly leave control as `Keep`.
- Invalid `if`/`call` rows and `push.refuse` do not pop.
- `clearResult` appears exactly in the rows whose R4 effects specify it, represented by `writes.result = Write::Set(None)`.
- No legacy row writes helper, task, allocation, environment-cell allocation, or value-slot payloads.

Legacy delta behaviour is exact:

- `push.accept` asserts `depth = +1`.
- Every normative legacy pop asserts `depth = -1`.
- Rewrites and all non-pop/non-push rows assert `depth = 0` through the default.
- All other five PlanDelta fields remain zero for every legacy row. In particular, charge-like counters and visit counters are not represented in `PlanDelta` and are not computed by these bodies.

Scalar write behaviour is exact:

- Entry writes only the new Scalar helper.
- Tag, arithmetic, and construct write back the copied helper with only the authorized staged/phase/register fields changed.
- Exit writes the typed `AllocDataWrite` only for `U128Value`, `BoolValue`, and `PendingNode`.
- For allocating staged values, the result reference is exactly `RefC::Data(pre.active().data)`. This is a prospective data reference derived from the pre-image active length; the body does not allocate an arena row, compute an index by mutating state, or update a counter.
- `PendingNode` allocation fields preserve order: `Leaf{field: operand_a}`, `Cons{fields: [operand_a, operand_b]}`, and `Branch{fields: [operand_a, operand_b]}`.
- Selector and type-test exits produce `ScalarSelector` and `ScalarTypeTest` without an allocation write.
- Exit and failure both clear the helper with `writes.helper = Write::Set(None)`.
- Failure carries the staged error code unchanged.

Scalar delta behaviour is exact:

- Entry, tag, arithmetic, construct, and failure leave all six delta assertions at zero.
- Exit asserts only `alloc_data_nodes` and `alloc_data_fields`, using the three owner guard queries in order.
- The field assertion is the supplied combination of the one-field and two-field guard results, not a locally recomputed staged-tag policy.
- No depth, task, environment-cell, or value-slot delta is asserted by Scalar exit.

No body contains a loop. No body escapes to a raw program table. No body mutates a machine image or performs an actual allocation.

## Evidence limitations

This review is text-only. The following were **not** observed:

- No Rust compiler result.
- No Rust test result.
- No runtime row application.
- No SDK or native execution.
- No independent recomputation of candidate hashes.
- No producer-independent oracle output.
- No proof that Root’s eventual imports, module placement, delegating implementor, concrete `PreImageView`, or concrete `GuardEvaluator` will compile or behave as intended.

Accordingly, statements about Rust validity are **source inferences** from names, fields, variants, signatures, binding shapes, and Copy/ownership structure in the supplied v3a text. They are not compiler findings.

The supplied Python inspection artifact is consistent with the direct review on row labels, legacy write-target sets, and legacy depth values, but this PASS does not rest solely on that artifact; the row effects above were checked directly against the two Rust fragments and the R4 records.

## Dissent and integration distinctions

No dissent is recorded against the two authored bodies within this non-security source scope.

The following are **future-integration obligations**, not task-blocking source findings:

1. Root must adopt or reject the proposed v3a shared contract separately.
2. Root must choose the module path and supply the promised imports/scope.
3. Root must provide the sole `FamilyPlans` implementor delegating to these private free functions.
4. The designated owners must supply concrete `PreImageView`, `GuardEvaluator`, selection, validation, admission, charging/capacity, physical application, and engine bodies.
5. Compilation, tests, runtime conformance, and oracle comparison remain unobserved.

Unrelated defects, if any, in the proposed shared contract or future owner/engine bodies are referred to Root. This PASS does not imply approval of the shared contract as a whole and does not certify any security, admission, cap, charge, or `StepInv` property.

## Final disposition

- `plan_legacy` — **PASS**, source-only, 37/37 rows.
- `plan_scalar` — **PASS**, source-only, 6/6 rows.
- Task-blocking findings against these two authored bodies: **none**.

ROW_EFFECTS_LEGACY_SCALAR_REVIEW_FULL_TAIL_20261016
