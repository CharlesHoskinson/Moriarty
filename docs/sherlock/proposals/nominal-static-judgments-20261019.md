# Proposed nominal static judgments — revision 02

Status: **proposal, not adoption**. Nonsecurity language design only. The two
fresh Opus 5.5 design opinions recommend ADOPT for the same three rules, with
mandatory rationale and integration amendments. This revision supplies those
amendments for a fresh Muse applicability review during the reported Opus quota
outage. Root still must count the opinions and record adoption. This file does
not amend standing OpenSpec, select a public typechecked profile, authorize
execution or supply security approval.

The production checker remains byte-identical to `d91f2355ee52dc55dccba96b2c8cfbc116b6f059`.
The independent M1 discriminator was frozen in `1cf9ce5bd4d1d99d3bbc8135d353de03396d5723`.
The current 21 functional test functions are all UNRUN; their provenance remains
in [the reference](../reference/nominal-body-typechecking.md). This documentation
revision adds no production code or tests. The proposal at `dc21c6a5948db2514ac062330785eb873edeabae`
and both original opinions remain historical inputs, without retroactive edits.

The intended capability is a selected public nominal compiler that checks every
function body and the closed entry before owner admission and truthfully reports
completed body typing. The actual route, failure stages and open obligations are
specified in the [revision 02 integration proposal](nominal-public-integration-contract-20261019.md).
That connection is not implemented by this proposal.

## Proposed rules

These judgments apply only to `ExtDomain::GENERIC_ABSENT`: pure generic nominal
source with ground types resolved by owner Pass T. Types are canonical `TypeIdC`
references, including finite recursive nominal references; declarations are not
infinitely expanded. Let `Σ` select the first matching function declaration in
unchanged source-table order. Let `Γ` map lexical names to ground types. Each
function body starts with only its own parameters; entry starts with empty `Γ`.
Named calls use signatures without unfolding bodies.

**Eq-Any.** If `Σ; Γ ⊢ left : τ₁` and `Σ; Γ ⊢ right : τ₂`, then
`Σ; Γ ⊢ left == right : Bool`. There is no premise `τ₁ = τ₂`. An illtyped
operand remains an error; Eq does not bypass typing either operand. This is a
static judgment, not a constant-folding instruction.

**Unique parameters in every declaration.** For each function
`f(p₁ : τ₁, …, pₙ : τₙ) : ρ { body }`, require pairwise distinct parameter names
and `Σ; {p₁ : τ₁, …, pₙ : τₙ} ⊢ body : ρ`. Check every declaration, including
unused declarations and later declarations with the same function name.
Duplicate function names retain first-match lookup in this isolated predicate;
this rule does not impose unique function names. Reject the second occurrence
of a repeated parameter name with `DuplicateParameter`.

**Unique binders in every arm.** For each arm pattern, collect its binders once
in original source preorder: PVar; PCons head then tail; PLeaf payload; PBranch
left then right; qualified constructor fields left to right recursively;
Wildcard contributes none. Require pairwise distinct names in that one list.
Require this for every arm, including unselected arms and dedicated-only
unplanned Matches. Reusing a name in another arm or shadowing an outer lexical
binding is allowed. Instantiate each field type using the owner's canonical
constructor table, extend only that arm's lexical environment, and check the
arm body. All arm result types must agree.

These are the same three judgments reviewed in votes A and B. The following
rationale and integration qualifications do not add another language rule.

## Rationale and actual owner boundary

Eq-Any follows the selected value-sum structural equality contract: individually
welltyped operands yield Bool, including comparisons whose types differ.
The selected universal-interpreter `design.md` calls this structural equality;
its `model-coverage.md` states value-sum equality with unequal variants false.
UIREQ-009 preserves structural
observations and traversal accounting. UIREQ-006 supplies the scalar primitive
context, but its scenario names arithmetic and lt, not an Eq typing judgment.
The existing text did not itself state a general nominal static Eq judgment;
Eq-Any remains this proposal until adoption. It deliberately admits always-false
comparisons such as UInt versus Bool or `Box<UInt>` versus `Box<Bool>`.

The source-expected runtime result for different types is false, not an observed
result or a correspondence theorem. The previously missing dependency is now
included: `owner_select_when` enables Equal Pair only for an Unvisited pair with
`!shallow_differ`, Different only for an Unvisited pair with `shallow_differ`,
and Field only for a Fields task. A kind mismatch cannot simultaneously satisfy
Pair or Field at that Unvisited task. `shallow_differ` compares the complete
logical kind, including instantiated constructor type and constructor IDs;
Different leads to negative status, and Equal exit produces Bool. This is a
source basis for the false expectation, including unequal-arity mismatches; it
does not establish a run or global runtime correctness. No Core/2 same-sort
restriction is imported. A different lowering needs its own correspondence.

The parameter rule is consistent with an **existing owner restriction**. V4 in
`check_program_view_ext` iterates parameters of every function row, including
unused rows, and rejects duplicate binder IDs. C5 retains every declaration and
maps parameter text to its interned binder ID. `pv_malformed("V4")` is
`CoreTooling(AdmissionRejected { reason: ProgramViewMalformed, detail: "V4",
locator: None })`. The static rule supplies an earlier typed and located error
and does **not** narrow that owner-admitted domain. Raw dynamic Bind fails only
when such a function is called; that is a separate, weaker fact, not the basis
for the declaration-level rule. The frozen checker's Bind-only comment should
be updated to cite V4 in a later reviewed source revision; this revision changes
no production bytes.

Owner V3 separately rejects duplicate function names. The isolated checker can
accept two individually valid same-named functions using the first signature
for calls, and then owner admission fails V3. Body success does not imply owner
admissibility. Checking later same-named declarations remains required; no new
DuplicateFunction judgment is proposed here.

The all-arm rule **narrows the current owner-admitted generic domain**. Owner
G4 and V16 enforce binder uniqueness only when a Match is planned. A root
Wildcard or qualified Ctor in any sibling arm causes planning; a Match with
only dedicated roots stays unplanned. In the latter domain, the only possible
repeated binders are flat `cons(x,x)` and `branch(x,x)`. Runtime
`BHelperMatchDistinct` compares binder IDs and puts those arms in NoMatchReady,
so they can never be selected. This is not a nonlinear value-equality pattern.

The proposed rule removes that class of dead arms and makes legality independent
of an unrelated sibling or surface spelling. Replacing an unused PVar fallback
with `wildcard` currently changes owner acceptance; replacing `cons(x,x)` with
`List::Cons(x,x)` also changes it because qualified Ctor roots are planned.
The all-arm rule rejects both forms consistently. Removing a dead unplanned arm
can preserve its selected value meaning, but changes source membership and may
change helper-work accounting. This is no runtime-domain or same-work equivalence
claim. The rule removes one class of unreachable arms through binder linearity;
it supplies neither general reachability nor exhaustiveness analysis. No K/B
agreement for duplicate dedicated binders is claimed.

## EARS proposal and retained trace

These are proposed refinements under existing requirements, without a new
denominator or adopted identifier:

| Existing trace | Proposed WHEN / SHALL behavior |
| --- | --- |
| UIREQ-027, nominal source profile | WHEN a caller requests body typing, preparation SHALL inspect every function body under its fresh declared parameter context and the closed entry; calls SHALL retain first-match signature references without source unrolling. |
| UIREQ-009, UIREQ-006 primitive context, selected structural equality | WHEN both Eq operands individually typecheck, the checker SHALL return Bool whether or not their canonical types agree; WHEN either operand is illtyped, it SHALL retain that typing failure. |
| UIREQ-027, declared parameter bindings and owner V4 | WHEN any declaration repeats a parameter name, body typing SHALL reject it even if unused, with an earlier typed diagnostic consistent with owner V4. |
| UIREQ-007 / UIREQ-008, declaration-driven patterns | WHEN any arm repeats a source-preorder binder, body typing SHALL reject it, including unplanned dedicated Matches; distinct arms and outer lexical bindings SHALL retain separate binder scopes. |
| UIREQ-002 / UIREQ-027, truthful profile and provenance | WHEN a selected public report says body checking completed, the same unchanged owner-prepared source SHALL have passed every body and entry under the adopted judgments; parsing, first preparation, body typing, owner admission and genesis failures SHALL remain distinct. |

All original 35 UIREQ and revision03/additive requirements remain retained.
These clauses establish no whole-language or requirement-closure result.

## Source-confirmed syntax and boundaries to freeze later

These are **source-derived expectations, all UNRUN**, not newly frozen tests.
Each fragment is inside `language "moriarty-sherlock-nominal/1"; program p { … }`.
Actual `nominal_source.rs::Parser` bodies confirm: `==` consumes two Eq tokens;
`nil` is a Core List literal; `cons` and `branch` expressions are dedicated
Binary constructors and `leaf` is Unary, not named Calls. Dedicated pattern
forms become CorePattern; `wildcard` becomes ExtPattern::Wildcard; an explicit
`::`/type application makes a recursive ExtPattern::Ctor; bare `fallback` is
Dedicated PVar. No casing or declaration lookup guesses that distinction.

| Judgment | Positive boundary | Negative boundary and exact expected diagnostic |
| --- | --- | --- |
| Eq-Any | `entry 1 == true;` has Bool (source-expected runtime false). | `entry (true + 1) == 0;`: `Expected { expected: UInt, actual: Bool }`, Entry child path `[0,0]`. |
| Unique parameters | `fn f(x: UInt): UInt { x } entry 0;` passes despite f being unused. | `fn f(x: UInt, x: UInt): UInt { x } entry 0;`: `DuplicateParameter { name: "x", index: 1 }`, function `Some(0)`, path `[]`; no body traversal. |
| Unique arm binders | `entry match cons(1,nil) { cons(h,t) => h, nil => 0 };` has UInt with an unplanned dedicated Match. | `entry match cons(1,nil) { cons(x,x) => 0, fallback => 1 };`: `DuplicatePatternBinder("x")`, Entry path `[1]` (arm 0 body); source-expected legacy miss then fallback value 1. |

Additional boundaries must be frozen with their exact payload/location before
execution, keeping each test independently interpretable:

- `(1 == true) + 1` rejects `Expected { UInt, Bool }` at Entry `[0]`.
- Replacing `fallback` in the duplicate-arm case with `wildcard`, or replacing
  `cons(x,x)` with `List::Cons(x,x)`, keeps the proposed body diagnostic at `[1]`.
  The owner already rejects those planned counterparts through G4; exact owner
  payload assertions must use its actual G4 error source, not invent an error.
- `match nil { nil => 0, cons(x,x) => 1 }` rejects the unselected duplicate at
  Entry `[2]`. This checks all arms, without general reachability analysis.
- `match cons(1,nil) { cons(x,y) => x, cons(y,x) => y }` has UInt; each arm has
  its own seen set. `let x = 1 in match cons(2,nil) { cons(x,t) => x, nil => x }`
  has UInt and permits outer shadowing.
- A second same-named declaration `fn f(y: UInt, y: UInt): UInt { y }` after
  valid `fn f(x: UInt): UInt { x }` rejects DuplicateParameter at `Some(1), []`,
  name `y`, index 1, even with `entry f(1)`. Owner V3 independently rejects the
  duplicate function name. With two individually valid signatures, body success
  followed by V3 owner failure must remain distinguishable.

Parser `match_expression_inner` orders expression children as scrutinee 0,
arm bodies 1..; `collect` records exactly those paths, with pattern field paths
separate. The integration proposal specifies diagnostic span mapping, including
Literal value fields. Freeze assertions against the actual parsed AST and
typed error payload, not merely `is_err()`. The 21 existing test functions and
their original pre/post-production history remain unchanged.

## Source evidence and acceptance

Relevant actual definitions at the frozen source cut are:

- [Selected universal design](../../../openspec/changes/sherlock-universal-interpreter-design/design.md), structural equality; [selected model scope](../../../openspec/changes/sherlock-universal-interpreter-design/model-coverage.md), value-sum equality; [UIREQ clauses](../../../openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md), UIREQ-006/007/008/009/027. [Revision03](../design.md) and its adoption record retain the whole scope; none supplies the proposed nominal static Eq judgment.
- [Checker](../../../crates/mori-core/src/sherlock/nominal_body_typecheck.rs), all-body/entry traversal, signature lookup, Binary Eq and per-arm preorder binders.
- [Owner selection](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs), `owner_helper_plan` and `owner_select_when`; [structural helpers](../../../crates/mori-core/src/sherlock/helper_machine/families/structural.rs), Equal Bool exit and MatchDistinct miss; [shared pure leaves](../../../crates/mori-core/src/sherlock/helper_machine/shared.rs), `shallow_differ`.
- [Owner view checks](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_ext_admit.rs), `pv_malformed`, V3/V4 and planned V16; [C lowering](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_ext_c_expression.rs), parameter rows, planning and G4; [P traversal](../../../crates/mori-core/src/sherlock/helper_machine/shared/ext_program_p.rs), planned-only G4 calls.
- [Actual nominal parser/provenance](../../../crates/mori-core/src/sherlock/nominal_source.rs), `atom_inner`, `binary`, `pattern_inner`, `match_expression_inner`, `collect` and public data types; [environment helpers](../../../crates/mori-core/src/sherlock/helper_machine/families/environment.rs), prepend/first-cell shadowing.

Acceptance requires fresh applicability review of this exact amendment, Root's
explicit adoption/scope record, separately reviewed source/public integration,
and later frozen ordinary assertions using the FULL normal Core rlib under
Root's new resource disposition. The selected successful report must identify
completed adopted judgments, including the all-arm restriction. Raw compile
and structural-only preparation gain no such qualification.

Two T3 passes, environment clones (potential quadratic work for deep lets) and
linear function lookup remain open resource facts. The parser budget is not a
global bound on them. No guard, cap, financial/lifecycle domain, public developer
permission or SDK/native operation changes here. Owner/resource/security,
compiler correspondence, formal, circuit, financial, native, target and all EARS
exits retain their actual independent evidence requirements.
