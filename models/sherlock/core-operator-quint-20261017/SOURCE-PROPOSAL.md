# Independent Core operator K-A QA source, version02

Source-only normal QA proposal. All parsing/type/effect checking, model tests,
simulation, K/Core/adapter/compiler/SDK/native execution and provider audits are
**UNRUN** in this assignment. Root owns later audit and eligible checks. No
interpreter semantics, repository source, guard, resource policy or public
language restriction was changed.


Version02 adds38 named deterministic prefix tests to the unchanged31 original
run definitions: `andFuel00DeterministicTest` through `andFuel15DeterministicTest`
and `leFuel00DeterministicTest` through `leFuel21DeterministicTest`. There are69
named definitions. The original40 complete Snapshot declarations, rule/charge
lists, two prefix expectation functions, inputs and interpreter are unchanged.
No expectation was regenerated from an observed output.

With max-samples1, each new named test has exactly its declared fixed fuel. The
two original `AllFuelPrefixesTest` definitions still choose nondeterministically
and therefore exercise only one fuel each under a one-sample invocation; they
must not be reported as all38 inputs. Root reported31 predecessor tests passing
under that invocation shape. This packet is the source-only correction to that
coverage gap. Version02 remains wholly UNRUN here; Root owns new execution.

`coreOperatorQaTest.qnt` imports the exact unchanged generic
`kProfileMachine.qnt` copy in `imports/`. This is the existing `sherlock-k-a/1`
interpreter, not the historical scalar15 algorithm instance. The new source
contains only finite program tables, independently intended lowering equations,
normal expected states and tests. It introduces no second evaluator. Neither
`advance`, `evaluate`, `ordinaryStep`, the Rust adapter nor another producer is
used to construct expected values, states or rule sequences.

## Declared equations and actual-lowering gap

For each source binary expression, independently choose fresh model names l/r:
`let l = LEFT in let r = RIGHT in BODY(l,r)`. This forces eager left-before-right
source evaluation, including boolean cases whose result is already determined
by l. The body equations are:

| Source | K-A body after both bindings |
| --- | --- |
| Ne | Not(Eq(l,r)) |
| Le | Not(Lt(r,l)) |
| Gt | Lt(r,l) |
| Ge | Not(Lt(l,r)) |
| And | If(l,r,false) |
| Or | If(l,true,r) |

These are independent intended semantic expansions. The task supplies ordinary
Core eager ordered binary meaning, the K-A rules and public fixture inputs as
normative input. Root confirms no additional exact lowering contract is
established. The actual Rust `source_adapter` was not read. Fresh-name choice,
AST shape, parameter entry, ExprIds, inserted scope work and exact fuel charges
are therefore **not asserted equal to the actual adapter or B implementation**.
Later comparison must inspect the actual selected lowering and declare a
shape/action/clock mapping. This packet cannot establish that correspondence.

The pure `lowered` helper only constructs syntax; existing K-A actions perform
all evaluation. Name ranges are disjoint from the named public fixture locals;
ExprId blocks are finite, closed and acyclic. Input literals preserve exact
UInt128 mathematical integer semantics. The finite values are ordinary examples,
not a new bound on Moriarty's supported numbers or programs.

## Concrete normal inputs and expectations

The public `.mori` and `.scenario.json` fixtures are transcribed into explicit
finite K-A programs. Parameters are modeled as outer Lets with the declared
scenario values. `emit` is projected to its final scalar argument only; no
ordered emit, effect, Core ledger result or event envelope is modeled.

| Declared public fixture | Independent local values | Scalar expectation | Chosen-AST charged work |
| --- | --- | --- | --- |
| ascending, left7/right11 | different T, low T, high T, inclusive F, both T, either T, result T | true | 164 |
| equal, left13/right13 | different F, low T, high F, inclusive T, both F, either T, result F | false | 178 |
| subtract-positive, 11−7 | difference4, difference==0 F | false | 28 |
| subtract-zero, 13−13 | difference0, difference==0 T | true | 28 |

The 31 original named test sources contain 17 derived-operator normal truth cases, six
Add/Sub/Mul/Eq/Lt/Not baselines, four public fixture projections, two complete
hand-declared traces and two bounded fuel-prefix families. Expected final tests
compare the complete existing Snapshot, not only the result. All are UNRUN.
The two prefix families use nondeterministic finite selection; their 16 and22
possible fuel inputs are declared coverage, not observed runs or exhaustive
checking results.

`DECLARED-EXPECTATIONS.json` is frozen before any subject execution. Its control,
environment and frame entries are literal declarations transcribed into the test
source. Python was used only to format these authored declarations and inventory
file hashes; it did not interpret the programs or run the imported subject.

The false-And complete trace has 15 charged rules plus free publication. Its
right literal has returned by charged boundary7, before the condition lookup at
boundary11, although the left value is false. The Le7/11 trace has 21 charged
rules plus free publication. Its original left7 returns at boundary3 and right11
at boundary7; only after both are bound does the comparison read r before l.
This distinguishes original operand evaluation order from variable-read order
inside a swapped primitive comparison.

Every boundary in both traces compares all Snapshot fields plus `lastRule` and
`lastCharge`. Fuel f below completion compares the complete published Exhausted
state from the declared pre-exhaustion boundary f, retaining that boundary's
environment and ordered frames. Exact completion compares free Done publication
with zero remaining fuel. No expected state is obtained by running model helpers.

## Field and clock map

REQUIREMENTS-AND-COVERAGE.json gives the exact mapping and exclusions. Snapshot
contains immutable program/code/functions, control phase and typed value, full
Map environment, ordered top-first frames with saved environments, stack limit,
initial/remaining fuel and raw outcome. Logical depth is frames.length().
`lastRule` is the pinned K source line and `lastCharge` is 0/1. Post-startup
initialization is74/0; it is not an actual K startup execution.

The charged primitive path uses binary evaluation117, push135, literal102 or
variable108, left-to-right handoff149, primitive result153, completion223;
Not uses120/135/157/223. Eager binding uses114/135/145 and restoration195;
If uses123/135 and branch161 or165. Free publication80 and exhaustion84 do not
consume fuel. A helper normalization creates no additional K-A work events.
Chosen lowered Ne/Le/Ge cost21, Gt17, And/Or15 for two one-step operands; direct
binary baselines cost7 and unary Not5. Fixture work additionally includes its
explicit source and parameter Lets. These are independently expected K-A counts,
not measured B helper work, circuit cost, source-adapter work or a model pass.

## Scope gaps and independence

The assignment reports an owner pipeline with all11 binary operators plus Not
and eight differential cases. Those scopes are distinct. Neither count is
K/B correspondence, formal acceptance or evidence that these new K-A tests ran.
This packet independently describes normal model behavior across those operator
forms in its own declared domain. It does not reuse the owner report or its
outputs as expectations.

No Rust source_adapter, runtime evaluator, private O, captured trace or actual47
report output was read. A broad docs-only search incidentally displayed one
public core47 README coverage sentence listing operators; no report was opened,
no expected/actual value or trace was exposed or used, and subsequent searches
excluded result-report paths. This exposure is retained in provenance rather
than claimed absent.

Actual source parsing, adapter hygiene and exact emitted AST, Core-to-K
translation, B helper/source clocks and allocation, original argument/function
entry behavior, emissions/financial effects and continuations, K execution,
Lean/Clean and handwritten constraints, native/target/proof/ledger evidence all
remain gaps. Error/refusal/hostile cases and security judgments are outside this
ordinary normal source assignment. Full B, general/mutual ADTs, all financial
and release scope, U/M/MC/SP/G, ACTUS/DeFi, Preview/Fenrir and required stagenet
acceptance remain open. Native authority0, C4, existing counters and limits are
unchanged. A passing future model run would establish only its recorded finite
K-A source predicate and would not close those gaps.
