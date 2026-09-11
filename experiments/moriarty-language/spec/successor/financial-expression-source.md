# Financial expression source /1

Implementation candidate; independent source/result audits remain pending.
The accepted design is the preserved
[proposal02](../../../../deliverables/sp02-expression-source-2026-09-10/pure48-source-proposal-01/PROPOSAL-02.md)
and its [decision](../../../../deliverables/sp02-expression-source-2026-09-10/pure48-source-proposal-01/DECISION-02.json).
The [complete grammar](financial-expression-source-grammar.ebnf) adds conditional
expressions and fixed generic calls to the separately preserved source/1 grammar.

```javascript
import { createFinancialExpressionSourceV1 } from '../../src/successor/financial-expression-source-v1.ts';
const language = createFinancialExpressionSourceV1(schemaCanonicalJSON);
language.check(source);
language.elaborate(source);
language.evaluate(source, snapshotCanonicalJSON);
```

From the repository root, developers can check or format the published vault
quote fixture with the explicit profile:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/financial-vault-quote.schema.json experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-expression-source/1 experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori
```

`check` performs the same source-only static judgment as `language.check` and
requires the trusted canonical schema. `format` uses no schema and writes its
canonical source to stdout. The complete CLI transport, errors, and exit-code
contract is in [expression-cli.md](expression-cli.md).

The source header must be `profile "moriarty-financial-expression-source/1";`.
The factory binds only `moriarty-financial-expression-contract/1`, including its
required `variantTypes` schema map. Neither older profile is upgraded. APIs accept
primitive source/schema/snapshot strings; every invocation owns new input and
result trees. Returned Core is inspectable output, never accepted as evaluation
input. Check and elaborate perform complete static checking with no reduction or
typed snapshot certification. The additive financial runtime `.check()` uses that
same static path and retains `.evaluate()` behavior and its result union.

SourceChecked includes the sourceProfile and staticWorkBound. SourceElaborated
also includes the runtime contract, agreement/action names and checked Core.
Evaluate returns the financial expression runtime's existing result records.
Snapshots have exactly Pre, Args, Obs and workInitial. The outer canonical JSON
transport cap is2000000 UTF-8 bytes before parsing; source, schema and individual
Pre/Args/Obs/Core components retain independent65536-byte caps. Profile lexical
and AST limits remain unchanged; no caller may supply an optimistic work bound.

## Values and source forms

All forty original forms retain their spelling and node count. Added types are
UInt256, Variant<F>, AmountProduct<A,B>, ScaledAmount<A,S>,
SignedScaledAmount<A,S>, SignedAmount<A> and NetAmount<A>, also nested inside
Option/Collection or named records. Indices are written directly in source and
resolve in trusted Σ. AmountProduct type indices must be authored in sorted
order, allowing equal assets; scale is0..18. No host type-alias registry exists.

| Source | Core constructor | Meaning |
| --- | --- | --- |
| amount<A>(q) | ConstructAmount | UInt128 quanta into a pure Amount<A> value |
| shares<V,H>(q) | ConstructShares | UInt128 units into a pure Shares<V,H> value |
| variant<F,T>(x) | ConstructVariant | Exact declared payload into Variant<F> |
| project_variant<T>(x) | ProjectVariant | Exact case projection; wrong active case rejects |
| some_value(x) | ProjectSome | Option payload; None rejects |
| to_uint<64/128/256>(x) | ConvertUInt | Explicit checked unsigned width conversion |
| quanta/mantissa/is_negative/magnitude(x) | ScalarValue | Exact closed scalar projection overload |
| condition ? consequent : alternative | Select | Lazy exact-type conditional expression |

### F — financial constructor rules

The rows below follow the L table shape of [static-semantics.md](static-semantics.md)
and are derived from the runtime's admission, typing and reduction code. Every
constructor is a closed tagged node, costs one unit of work on entry, and rejects
VALUE_BOUND if its result exceeds the value bounds, except Select, whose result
is the already-checked value of the selected arm. Operands before the semicolon
are metadata and are not evaluated; operands after it are evaluated children in
listed order. Static rejections use the original span and node path of the
constructor; runtime rejections use the same provenance and consumed work.

| Rule | Constructor (metadata; children) | Typing | Reduction | Rejections |
| --- | --- | --- | --- | --- |
| F-CA | ConstructAmount(asset; value) | asset declared in Σ.assets; value:UInt128; result Amount\<asset\> | returns the integer as quanta; range check on Amount | TYPE_NAME, TYPE_MISMATCH, ARITH_RANGE |
| F-CS | ConstructShares(vault,holder; value) | vault in Σ.vaults and holder in Σ.parties; value:UInt128; result Shares\<vault,holder\> | returns the integer as share quanta; range check on Shares | TYPE_NAME, TYPE_MISMATCH, ARITH_RANGE |
| F-CV | ConstructVariant(family,tag; value) | Σ.variantTypes[family][tag] declared; value has exactly that payload type; result Variant\<family\> | returns the tagged payload {tag, value} | TYPE_NAME, TYPE_MISMATCH |
| F-PV | ProjectVariant(tag; value) | value:Variant\<F\>; tag declared in F; result is F's payload type for tag | active tag must equal tag, else VARIANT_CASE; returns the payload | TYPE_MISMATCH, TYPE_NAME, VARIANT_CASE |
| F-PS | ProjectSome(value) | value:Option\<T\>; result T | None rejects OPTION_NONE; Some(v) returns v | TYPE_MISMATCH, OPTION_NONE |
| F-CU | ConvertUInt(width; value) | width in {64,128,256}; value:UInt64, UInt128 or UInt256, including the same width; result UInt\<width\> | returns the integer unchanged; must fit the target width, else ARITH_RANGE | TYPE_LITERAL, TYPE_MISMATCH, ARITH_RANGE |
| F-SV | ScalarValue(component; value) | component in {quanta, mantissa, negative, magnitude}; exactly one admitted overload per component and child type (table below) | quanta and mantissa return the underlying integer; negative returns mantissa \< 0; magnitude returns the absolute value | TYPE_LITERAL, TYPE_MISMATCH |
| F-SEL | Select(condition, consequent, alternative) | condition:Bool; consequent and alternative have the identical complete type T, with T not Unit or Operation; result T | after entry evaluate condition once, then enter only the selected arm at child path 1 or 2 | TYPE_MISMATCH, WORK_EXHAUSTED at the selected arm |

ScalarValue overloads are exactly: quanta on Amount or Shares gives UInt128;
mantissa on Price gives UInt128 and on Rate or Quantity gives SInt128; negative
on Rate, Quantity, SignedAmount or NetAmount gives Bool; magnitude on Rate,
Quantity or NetAmount gives UInt128 and on SignedAmount gives UInt256. Any other
pairing rejects TYPE_MISMATCH. The source spelling `is_negative` lowers to the
Core component name `negative`; the prose above uses the source spelling.

Related admitted extensions in this profile, not new constructors: LitUInt
accepts width 256 (the `u256(q)` literal); Add, Sub, Mul, FloorDiv and CeilDiv
admit identical UInt256 operands with a UInt256 result; Mul admits Amount×UInt128
in either operand order to Amount,
Amount×Amount to AmountProduct with sorted indices, Amount×Price to ScaledAmount
when the price's quote matches the amount's asset, and Amount×Rate to
SignedScaledAmount; FloorDiv and CeilDiv admit AmountProduct÷Amount to the other
Amount index, and ScaledAmount or SignedScaledAmount divided by a literal UInt128
exactly 10^scale to Amount or SignedAmount, else TYPE_SCALE_DIVISOR. Comparison
Lt/Lte/Gt/Gte accept every numeric type listed in the runtime's NUMERIC set with
identical operand types. These are the code's overloads; the prose sections below
describe the same set.

`u256(7)` is a one-node UInt256 literal. Its operand must be an integer literal;
`u256(q)` does not convert a variable. Bare positive integers remain UInt128 and
bare negative integers remain SInt128. `amount(5,A)` remains one LitAmount node;
`amount<A>(5)` always retains ConstructAmount plus its LitUInt child. The same
distinction applies to literal and generic shares. There is no folding, hidden
width conversion or generic host function dispatch.

ConvertUInt accepts unsigned integers only, with explicit range-checked narrowing
and metered widening, including same-width conversion. To erase asset/holder
indices, source must say so: `to_uint<256>(quanta(a))`, for a:Amount<A>, returns
raw UInt256 quanta; `to_uint<256>(a)` rejects. ScalarValue overloads are exactly:
quanta on Amount/Shares→UInt128; mantissa on Price→UInt128 or Rate/Quantity→SInt128;
is_negative on Rate/Quantity/SignedAmount/NetAmount→Bool; magnitude on
Rate/Quantity/NetAmount→UInt128 or SignedAmount→UInt256. No other pairing applies.
No new signed256 literal, dynamic rate constructor, match statement or tag test
is introduced. Variant family/tag metadata uses separate bare identifiers, never
dotted metadata, runtime-selected names or implicitly refined payload types.

The existing `*`, floor_div and ceil_div source forms select the published closed
dimensional overloads. Scaled division requires a literal UInt128 exactly10^scale;
an equal-valued local or conversion node does not satisfy TYPE_SCALE_DIVISOR.
No plain division operator, implicit widening, multiply/divide fusion, asset cast,
or extra arithmetic on SignedAmount/NetAmount is added by their type spellings.

## Conditional evaluation and errors

Conditional precedence is below `or`, associates right, and retains all three
children. Condition must be Bool; arms must have the identical complete type,
excluding top-level Unit/Operation, even when one arm would never run. Static
checking visits condition, consequent and alternative. Dynamic execution charges
Select once, evaluates condition once and enters only the chosen arm. Child paths
are0,1,2 and errors retain their child's span/path. Both arms count toward B.

`let x = true ? 7 : floor_div(1,0);` has B7 and actual work4. Replacing true with
false rejects ARITH_DENOMINATOR after6 work at path[0,0,2]. Budget3 on the true
case rejects entering its selected literal. Source spans are half-open UTF-8 byte
offsets; formatting preserves operand order, metadata and every conversion node.

The inherited phase order is transport, parse, schema/source declarations,
lowering, whole-Core typing, typed snapshots, reduction. All pre-reduction errors
have workUsed0. Wrong metadata shape/arity uses existing SOURCE_* diagnostics;
valid unknown identifiers and wrong exact types reach the runtime's static codes.
Financial generic/type/intrinsic term reservations are local to this profile.
The original /1 still admits its previous binders, including u256. Financial
metadata excludes the new fixed generic names as well as the old keyword/generic
exclusions; unsupported identities reject SOURCE_SCHEMA_NAME without renaming.
The corrected access_field operand order is retained in both source profiles.

## Runnable source and remaining authoring

Run `npm --prefix experiments/moriarty-language run financial-expression-demo`.
The [vault quote](examples/financial-vault-quote.mori) explicitly widens its
UInt128 inputs, floors4×3/10 and narrows to1, constructs holder-indexed shares and
a variant, updates an ordinary counter and emits a local Notice. The demo also
selects an absent optional input, which rejects OPTION_NONE without publishing
state or descriptors. Its successful run has B43 and actual work41.
[financial-all48.mori](examples/financial-all48.mori) is an executable fixture
covering all constructor names; the tests separately check indexed overloads,
all new type forms, scalar bounds, independent vault equations, rollback, spans,
lazy work, formatter preservation and CLI/API equality.

The [CLI adapter](expression-cli.md) supports explicit check/format selection for
this profile as well as original expression-source/1. This is local expression
evaluation: constructing amounts or shares and emitting a descriptor confers no
funding, minting, availability, debit, signing or transfer authority. Ordinary
writes remain schema-controlled and financial writes reject. No funded or ledger
path is called.

Full SP02 still requires source-defined schema and financial declarations with
identity/denomination authority, genesis/constants/state initialization, multiple
actions and deterministic selection, identity escaping, full obligation/request/
time/valuation authoring, complete CLI surface and actual-participant syntax
evaluation. The one-action trusted-schema component is not the final language.
Financial transitions, K/proof correspondence, mandatory PCD, ACTUS/DeFi and
Preview acceptance retain their independent gates.
