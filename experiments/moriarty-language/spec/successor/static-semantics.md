# Proposed static semantics and expression rules

Status: **PROPOSAL**, companion to [semantic-contract.md](semantic-contract.md).
Rules apply to the complete 40-constructor expression inventory, not to a newly
accepted execution profile. Financial operation equations and full signing
remain outside this contract. Rule IDs below are stable references for the
[machine-readable signatures](expression-signatures.json).

## S0 — judgments and admission order

Σ is the bounded acyclic schema registry from V. Γ contains four disjoint
namespaces: immutable locals L, typed arguments A, typed observations O and
persistent fields F. Names are identifiers; no implicit fallback across
namespaces. T includes the finite ground and recursively constructed types in V.
Unit is a statement-result marker, excluded from T and every Σ data schema,
input, field, record member, option and collection element. Asking for Unit as a
value type (for example ConstructNone(Unit)) rejects TYPE_NAME. It cannot be
introduced by a read or projection.

Exact type equality includes every asset, holder, vault, scale, unit vector,
record/enum identity and collection capacity. No subtyping or implicit casts.

`Σ;Γ;phase ⊢ e:T` is total as an algorithm on the admitted finite syntax:
visit constructor operands in their named/source order, recursively check all
children (including both Boolean branches), then apply the listed rule. If no
listed overload applies, reject TYPE_MISMATCH. Unknown tags reject
TYPE_CONSTRUCTOR. Unrecognized metadata/fields reject INPUT_SCHEMA. Bounds or
malformed spans reject INPUT_BOUND/INPUT_SPAN before typing. Malformed literal
values reject TYPE_LITERAL. Missing names/types/fields reject TYPE_NAME.

Admission precedes reduction:

1. Validate closed structure, duplicate-key freedom, canonical scalar metadata,
   raw aggregate JSON byte bounds, schema JSON-tree bounds, Core AST bounds and
   spans in preorder, first defect wins. Type-dependent individual W/value-node
   bounds wait until types are resolved; no undefined type-size function is called. Structural
   parsing itself is an implementation obligation; these rules do not claim a
   supplied hostile object is already safe.
2. Validate Σ in identifier order: acyclic declared references and bounded shapes;
   validate action parameter/field/observation declarations. Field registries,
   not callers, assign ordinary/financial write classes.
3. Typecheck the complete action in lexical order, including unexecuted statements.
   At each statement check its placement/target/binder before its expression,
   then check expression operands recursively before its result constraint.
4. Validate snapshots in order Pre, Args, Obs, each in identifier order. Missing/
   extra names or a value tree differing from its resolved type shape rejects
   INPUT_SCHEMA. Then check individual W and typed aggregate node/depth bounds
   (INPUT_BOUND), then mathematical value domains (INPUT_VALUE). In each stage
   inspect children in the type-defined field/list order. No missing read can
   surface after a preceding guard. This is before all reduction, not a late read.
5. Start work-metered reduction. The exact shape, size, annotation-versus-value distinction and admission
   phase are fixed by [representation.md](representation.md). Static checks consume no semantic work; bounded
   validation overhead still exists and is not a native-cost estimate.

Thus a later static error precedes an earlier *runtime* guard failure. Lexical
first-failure means first in the relevant admission/static/runtime phase, not
an excuse to skip checking later source. No diagnostics list is authoritative
for deciding which side effects to commit: all rejection paths publish none.

Actions are statement lists `Require|Let|NextWrite|Emit` followed by zero or more
Ensure. Those five nodes cannot nest as expression operands. A pure expression
never returns Unit or mutates a frame. Statement results below are Unit, including
Let (it binds a typed value but is not an expression returning that value).
This corrects the earlier candidate's ambiguous Let result T and is a new proposal.

## L — literals

All literals take metadata only, have no evaluated child and cost one on entry.
The indicated type is determined statically; literal domain validity is checked
there after structural admission. Wrong JSON scalar kinds fail INPUT_SCHEMA
earlier; in particular LitBool(1) cannot reach TYPE_LITERAL. Only true/false
have a structurally valid Boolean literal representation.

| Rule | Constructor metadata | Type and exact value |
| --- | --- | --- |
| L-U | LitUInt(width,value) | width 64 or 128; type UInt64/UInt128; value in that unsigned range |
| L-I | LitSInt(value) | SInt128, exact signed integer |
| L-B | LitBool(value) | Bool, literal true/false only |
| L-T | LitText(value) | Text, exact decoded bounded scalar string |
| L-A | LitAmount(asset,value) | Amount<asset>, declared asset, UInt128 quanta |
| L-Q | LitQuantity(units,scale,mantissa) | Quantity<units,scale>, canonical declared units and SInt128 mantissa |
| L-S | LitShares(vault,holder,value) | Shares<vault,holder>, declared identities, UInt128 quanta |
| L-R | LitRate(scale,mantissa) | Rate<scale>, scale 0..18, SInt128 mantissa |
| L-P | LitPrice(base,quote,scale,mantissa) | Price<base,quote,scale>, distinct declared assets, UInt128 mantissa |

There is no LitDebt. A financial record read from a declared schema is not a
primitive numeric amount. No arithmetic overload operates on a Debt record.

## READ — immutable reads

ReadLocal(name), ReadArg(name) and ReadObs(name) return exactly L[name], A[name]
and O[name], with their declared types. Names must exist at that lexical point.
ReadObs is a pure read of the supplied snapshot: it emits no Observe effect and
makes no oracle truth/currentness assertion. All read nodes cost one.

ReadPre(view,field) accepts metadata view `pre|post` only. `pre` selects F in
immutable pre; `post` selects the assembled overridden F. Type is F[field].
`post` is permitted anywhere inside an Ensure condition and nowhere else;
violation rejects TYPE_POST_SCOPE. `next` rejects TYPE_NEXT_READ. Both views
have the same declared field type. A private/unauthenticated observation remains
private/unauthenticated; these semantics contain no implicit declassification.

## ACCESS — projection

ProjectField(record,field) and AccessField(record,field) require
record:Record<R> and metadata field in R. Result type is R[field], result value
is that exact field, no copy-as-authority or ownership claim. A wrong field is
TYPE_NAME; a non-record child is TYPE_MISMATCH.

ProjectIndex(collection,index) and AccessIndex(collection,index) require
Collection<T,N> and UInt64. Result type T. After both children evaluate, require
index < actual length; otherwise INDEX_RANGE. This is runtime length, not just
capacity. The finite index check never performs an unchecked host array read.
Empty collections therefore reject every index. Aliases have identical rules.

## BUILD — finite constructors

ConstructRecord(recordType,fields): metadata recordType R must be declared.
First resolve R. Then scan the entire field-name list in lexical order for
duplicates, rejecting TYPE_DUPLICATE_FIELD at the first repeated name. Only
after that scan succeeds compare the set to R; missing/extra fields reject
TYPE_RECORD_FIELDS. Perform both checks before any child typing/evaluation.
Each named expression field therefore occurs once and exactly covers R. Evaluate field expressions in lexical order; require each type equal
to R[field]. Return Record<R> sorted by name for representation. Extra evaluation
order is not introduced by sorting. Aggregate value bounds checked last.

ConstructEnum(enumType,member): both metadata; member must belong to declared E;
return Enum<E>(member), otherwise TYPE_ENUM_MEMBER. No runtime payload or effect.

ConstructSome(elementType,value): explicit T metadata; require value:T;
return Some<T>(value). ConstructNone(elementType): return None<T>, no child.
T must be a finite registered type; never infer None's type from a later use.

ConstructCollection(elementType,capacity,items): metadata T,N (0≤N≤128), ordered
expression list length≤N; check shape before children, else TYPE_COLLECTION_BOUND.
All children have exactly T. Return Collection<T,N> preserving order. Any result
of BUILD exceeding the aggregate value/byte/depth bounds rejects VALUE_BOUND
at that constructor, after its children and before returning the value.
A well-shaped undeclared nominal type reference rejects TYPE_NAME during
schema/static resolution. An unknown wire tag rejects INPUT_SCHEMA. A cyclic
type dependency rejects TYPE_SCHEMA_CYCLE during schema validation, including
Operation<O> through its operand Record and any Option/Collection wrappers.

## ARITH — exact overloads and guard order

Arithmetic is strict. Evaluate left then right, then apply the following rules.
Unless listed, an operand-type combination rejects TYPE_MISMATCH statically.
For scalar integer operations, both operands and result have the identical type
T in UInt64, UInt128 or SInt128. Add/Sub/Mul use mathematical +/−/× and reject
ARITH_RANGE when the result does not fit T. There is no widening then truncation.

| Operation | Additional admitted overloads | Result and checks after children |
| --- | --- | --- |
| Add/Sub | Amount<A>, Shares<V,H>, Rate<S>, Quantity<U,S>, with exactly equal indices | Same type; operate on quanta/mantissas and check its underlying UInt128/SInt128 range |
| Mul | Quantity<U1,S1>, Quantity<U2,S2> | Quantity<normalize(U1+U2), S1+S2>; result type must satisfy unit/scale bounds; SInt128 mantissa product must fit |
| FloorDiv/CeilDiv | Quantity<U1,S1>, Quantity<U2,S2> | Quantity<normalize(U1−U2), S1−S2>; result scale must be 0..18 and units bounded; denominator mantissa positive; floor/ceil of the mantissa quotient, checked SInt128 |

For Quantity Mul/Div, type-level exponent and scale arithmetic uses exact
mathematical integers during static checking; no runtime numeric widening is
implied. Normalize merges equal symbols and removes zero exponents; reject
TYPE_QUANTITY_DOMAIN if the result has >8 symbols, exponent outside −128..127,
or scale outside 0..18. These are representation restrictions, not a claim every
mathematically meaningful dimensional operation is representable. In particular,
no implicit rescale repairs a negative division scale. A later rescale operation
requires explicit source requirements and cannot be silently introduced here.

Scalar FloorDiv/CeilDiv require denominator >0, otherwise ARITH_DENOMINATOR
before quotient computation. For integer numerator n and positive d, compute
unique integers q,r with n=q*d+r and 0≤r<d. FloorDiv returns q; CeilDiv returns
q if r=0, otherwise q+1. Check result range, else ARITH_RANGE. This includes
negative SInt128 numerators: −5/2 gives floor −3, ceil −2. No floating point.
Quantity division uses the same rule on mantissas, retaining the derived scale.

No plain `/`; no Rate*Amount or Amount/Price implicit money conversion;
no Price arithmetic, Amount multiplication, Shares across different holders,
Width mixing, comparison-driven casts or Debt-as-Amount. Missing target-required
overloads stay explicit SP01 decisions; these signatures do not silently remove
those financial requirements from RP01. Conversion operations require their
separate financial contracts.

## CMP — comparisons and Booleans

Eq requires identical value types T; admits scalar values and finite Record,
Enum, Option and Collection recursively, including indexed financial *values*.
Operation descriptors may be compared structurally but comparison grants no
financial authority. Records compare the same named fields; collection elements
compare in order; None equals None of the same T, never Some. Text equality is
exact decoded scalar equality. Result Bool. No pointer or host object equality.

Lt/Lte/Gt/Gte accept identical scalar integer types or equal-index Amount,
Shares, Rate, Quantity or Price. Compare underlying integer quanta/mantissas.
Result Bool. They reject Bool, Text, Record, Enum, Option, Collection and Operation;
there is no implicit ordering on identities or records.

Not requires Bool, returns logical negation. And/Or require two Bool children,
evaluate both strictly in order, then return conjunction/disjunction. Runtime
failure in a child wins over the eventual Boolean result. No control-flow or
financial composition operator is encoded by these nodes.

## STMT — staging and descriptors

Require(cond): require Bool. Evaluate cond; false rejects GUARD_FAILED, true
returns Unit with unchanged frames. Ensure(cond): same, but legal only in the
final suffix and false rejects ENSURES_FAILED. Ensure changes no state.

Let(name,value): name must be fresh in L and not shadow any argument, declaration
or reserved pre/post/next name; violation TYPE_DUPLICATE_BINDER. No self reference:
check/evaluate value under the old L, then extend L once. Type of binding is
value's type. Statement returns Unit. Later locals can read it.

NextWrite(field,value): field declared ordinary; financial fields reject
TYPE_FINANCIAL_WRITE. Check field exists and has no previous NextWrite in this
action before checking value; duplicate rejects TYPE_DUPLICATE_WRITE. Require
value's type equals field type; evaluate value against immutable pre, prior
locals/args/obs, never tentative writes. Stage writes[field]=value; return Unit.
No state mutation is externally visible. Whole-action static tracking detects
all duplicate writes even if an earlier runtime guard would have failed.

Emit(operation,fields): metadata operation O must be registered with exact
operand Record<R_O>; require fields:Record<R_O>. Evaluate fields; append
Operation<O>(fields), preserving order. Check descriptor count/aggregate bound,
else DESCRIPTOR_BOUND. Return Unit. The operation registry supplies an operand
schema, not a callable semantic function. No balance, debt, allowance or work
transition is inferred from the descriptor. Executable operation interpretation
requires a separate reviewed relation and may not treat this return as success
of the corresponding financial action.

After all statements and Ensure conditions succeed, finalize the ordinary
post-state and check aggregate bounds, otherwise VALUE_BOUND. Every non-written
field is exactly retained. Stage assembly has no extra expression node charge.
A failure's local work count remains diagnostic only, as defined in E.

## Cases and acceptance limits

Every declared constructor has a positive and a distinguishing rejection case
in expression-cases.json, derived from these equations by the author separately
from the structural validator. Additional cases cover x=10→11, false ensures12,
duplicate writes, next reads, misplaced post, mixed assets, strict Boolean and
two failures. The validator checks their record completeness and references;
it neither interprets terms nor proves that a derivation is correct. Fresh
reviewers must examine those derivations and the missing financial interfaces.
