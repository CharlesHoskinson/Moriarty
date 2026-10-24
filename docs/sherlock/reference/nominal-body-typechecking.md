# Nominal expression body typing candidate

Historical source-review record frozen before its later qualifications. The
UNRUN and pending-adoption statements below describe that earlier stopping point;
Root later adopted exactly three judgments and observed separate BW27 and NW21
checks. See the [current adoption](nominal-body-compiler-adoption-20261019.md)
and [NW observation](nominal-body-typed-cli-observation-20261019.md).

Source-only, unrun. Base: `79f42d7ab2699bc2e1be47a19c47aa190cc52e8b`.
This additive API does not alter a public compile handler or owner admission.

The callable predicate is `check_nominal_program_bodies(&prepared)`: every
function body has its declared return type under only its parameters and local
binders; entry is closed; calls use the first matching declaration's complete
signature without unfolding bodies. The explicit domain and owner-prepared
T3 canonical type IDs and constructor field types are retained. Recursive types
remain finite references. Declaration validation and instantiation stay with
`prepare_ext_program_types`.

The initial source-only expectation table was frozen before implementation;
its history and subsequent corrections are recorded below:

| Case | Expected |
| --- | --- |
| `fn f(n: UInt): Bool { n }`, including unused f | reject return mismatch |
| undeclared function-body variable or free entry parameter | reject unknown variable |
| let initializer refers to its own new binder | reject unknown variable |
| let shadows an outer binder and restores scope outside the let | accept correct outer type |
| arithmetic on Bool; not on UInt; non-Bool if condition | reject operand mismatch |
| differing if branches or match arm result types | reject result mismatch |
| missing named function; wrong arity; wrong argument type | reject independently |
| direct and mutual recursion with declared signatures | accept without unfolding |
| constructor Box<UInt> receives Bool | reject field mismatch |
| Box<UInt> and Box<Bool> used interchangeably | reject nominal mismatch |
| nested Pair<Bool, Box<UInt>> constructor/pattern | bind Bool and UInt in declared field order |
| constructor pattern for a different nominal instantiation | reject scrutinee mismatch |
| pattern binder referenced outside its arm | reject unknown variable |
| nested repeated pattern binder | revised in `d91f2355`: proposed rejection; agrees with owner G4 on planned Matches |
| mutually recursive nominal declaration references | accept finite signature references |
| dedicated Cons/Leaf/Branch binding types | UInt/List/Tree as existing constructors specify |

The G4 row is a revision, not the original frozen wording. At `9af39187` it
read: `nested repeated pattern binder` / `reject existing nominal G-4 rule`.
The revision corrects that overstatement; the implementation and proposed
static rule have the qualified scope described below. The frozen table has no
Eq row, although its frozen test file contains an Eq function.

Equality's proposed static predicate permits any two individually welltyped
operands and produces Bool. This is an **inference/recommendation** from the
[universal model's value-sum equality](../../../openspec/changes/sherlock-universal-interpreter-design/model-coverage.md)
(unequal variants compare false), the [existing draft checker](../../../crates/mori-core/src/sherlock/draft_typecheck.rs)
and B's `HelperKind::Equal` route in `owner_runtime.rs`. These describe runtime
behavior, not a complete normative nominal static typing judgment. Fresh Opus
design/source review recommends Eq-Any, but spec-owner adoption is still pending
before integration or any typechecked-profile claim. The test name identifies
this proposed rule. Core/2 has a separate equality typing rule. No numeric
coercion or default is added.

Duplicate-parameter rejection is also a proposed static judgment pending
adoption. Actual owner `Bind` scans its partial parameter environment and sets
`BindStatus::Bad` on `GBindDupHit`; it does not choose a winning duplicate.
Owner parameter-row lowering retains source rows without a duplicate-name check.
`EnvPut`, used for lets, instead prepends a cell, and `EnvGet` finds the newest
binding. Runtime call rejection therefore supports rejecting duplicate parameters
statically, but rejecting an unused declaration still needs a static judgment.

Pattern binder uniqueness agrees with the **implementation-only** preorder rule
in `owner_ext_c_expression.rs::check_arm_binders` on **planned Matches**. The
actual `begin_match` plans a Match when any root arm is `Ctor` or `Wildcard`;
dedicated-only Matches keep `plan: None` and bypass this check. `lower_ext_p`
calls `check_arm_binders` for every arm of a planned Match. Qualified builtin
`List`/`Tree` patterns lower to `PatternRowC::Constructor` and are planned too.
This checker also rejects repeated binders in dedicated-only unplanned Matches;
that is a proposed static restriction, pending adoption, while legacy runtime
matching treats the repeated binder as a miss. No separate adopted G4 normative
text was located. The G4 label in an error string establishes no static rule.
No runtime rule is changed. This API checks pattern typing, not exhaustiveness
or reachability.

Test provenance: `9af39187` froze fifteen functions before production. Fourteen
have unchanged expectations; one G4 function was substantively rewritten after
production from a flat duplicate pattern to a nested pattern that asserts the
duplicate name. Four functions were added after production (first-match calls,
pattern shadow restoration, proposed duplicate-parameter rejection and nested
owned literal fields). The harness was adapted after production, including an
explicit `std::result::Result` and an extracted layout helper; the historical
unqualified two-argument Result signature would conflict with the later private
one-argument alias. None of these tests has run. The rewritten G4 test and later
tests are author expectations, not independent pre-production expectations.
The original bytes remain in their commits.

The following label-only renames occurred in `d91f2355`; their expectations did
not change. Per-function result reporting must retain this history:

| Previous function name | Current function name | Origin |
| --- | --- | --- |
| `equality_retains_structural_value_sum_typing` | `proposed_eq_any_rule_accepts_individually_typed_operands` | pre-production function |
| `unused_duplicate_parameter_is_a_structural_typing_error` | `proposed_static_rule_rejects_even_unused_duplicate_parameters` | post-production function |

After fresh review found M1, `1cf9ce5b` froze a new twentieth function before
the M1 production correction. It exercises all four qualified builtin pattern
constructors, nested builtin patterns, wildcard and a type mismatch. The fix
uses the owner's fixed builtin constructor source ordinals instead of sentinel
`name_id: 0`; user nominal constructors still use the interned name binding.
The new discriminator is also unrun: there is no observed red/green result.

Revision02 review R1(a) supplied the missing constructor-use scope. The shared
`fields` helper serves Construct, literal Constructor and constructor patterns.
In the actual T3 walker, both Construct and LiteralCtor become
`T3UseKind::Constructor`; `PassTSession::step` sends both through
`resolve_construct_use`. That resolver already rejects List/Tree with the
existing `BuiltinConstruct` reason. Qualified builtin patterns instead use
`resolve_pattern_ctor_use`, which admits the fixed builtin ordinals. Therefore
the shared helper's builtin branch is reachable through unchanged owner
preparation for patterns, and cannot newly admit builtin constructor values.
No production change or new unsupported-form restriction is needed.

The actual runtime keeps dedicated Cons/Leaf/Branch rows distinct from generic
Constructor rows. Materialization writes `DataRow::Constructor` with its type
and constructor IDs; `adt_checked_ctor_arity` refuses builtin IDs below4 for
that generic kind. Constant constructors similarly use `ConstRow::Constructor`.
This is source representation evidence, not a runtime observation. Do not
fabricate a builtin generic value to assert correspondence through this API.

`d99c4a8c` freezes a twenty-first discriminator while production remains exactly
`d91f2355`: it calls public preparation directly for List/Tree Construct text
and typed List/Tree literal Constructor carriers, expecting the actual owner
`BuiltinConstruct` error. It does not use the success-only `check` fixture
helper. The new test is unrun and no red/green observation is claimed.

Diagnostics use deterministic worklist order: If branches are inferred before
the condition type is required; all call/constructor children are inferred
before their expected types are compared; all arm patterns are bound before
arm bodies are inferred. Pattern errors name the arm-body path. After a Literal
expression, remaining `child_path` indices identify ExtValue constructor fields,
not Expr children or T3 value ordinals. Typed-carrier out-of-range KIntC markers
remain UInt here; owner constant lowering preserves them as U128OutOfRange for
subsequent validation/runtime rejection. Body typing is not literal-range
validation or owner admission.

Implementation plan: freeze executable tests; implement an additive module that
indexes borrowed T3 source uses and infers expression results with an explicit
work stack; type constructor fields and nested patterns from existing tables;
export only that module; request fresh Opus review of all changed bytes before
any Cargo/rustc/checker/provider execution. Existing parser/type depth guards
and native/resource authorities are unchanged. No new budget or H1 guarantee
is supplied. Finance/lifecycle additions have no selected typing rules here;
the API returns an explicit unsupported-domain/form diagnostic instead of
accepting those forms.

The original proposed `cargo test --lib` selection is superseded by an external
consumer proposal. [The retentive shim](../../../crates/mori-core/tests/consumers/nominal_body_types_public.rs)
reuses the exact twenty-one current test bodies by source path and calls the same public
API in a full normal Core rlib. It adds public import aliases only. Internal and
external test modules include the same test file. This consumer exercises public parser/type
preparation, nominal checker success and typed errors; it does not exercise a
new public compile-handler integration or financial effects.

The actual Core manifest has Midnight dev dependencies. Therefore even a
`cargo test --test` target is not the selected proposal. After fresh source
review and a new Root resource disposition, build the full normal library using
`cargo build -p mori-core --lib`; then compile the shim with `rustc --edition=2024
--test`, `--extern mori_core=<that exact rlib>` and `-L
dependency=<that build's actual deps directory>`. Run only that external test
binary. Pin the actual artifact/command/configuration before execution; no
rlib, successful link or passing test result exists for this candidate yet.
The workspace declares edition2024/rust-version1.99 and no panic profile
override; compiler identity and the actual unwind-test linkage remain unobserved.

Do not reuse or raise the historical 30-second wall/25-second CPU/2-GiB address
space/64-MiB file envelope: its older cut failed the wall limit at load31. The
new candidate is unmeasured. A source review does not supply Root's separate
resource disposition. There is no red/green observation, compiler result,
native evidence or EARS closure. The source-only instruction explicitly defers
the TDD execution steps until fresh independent review. Static formatting and
`git diff --check` were inspected without running Cargo, rustc or the checker.
