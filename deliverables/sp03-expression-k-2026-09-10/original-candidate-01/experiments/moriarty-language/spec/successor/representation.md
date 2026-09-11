# Proposed normative representation and size

Proposed correction for EX-G6-F1. This specifies a mathematical embedding and size
function, not an implemented encoder, decoder or accepted execution profile.
It supplements V/S0 of the expression contract and must be reviewed with them.
No financial-operation semantics or signing hash domain is added.

## Canonical JSON spelling

Let J(x) be the UTF-8 byte string for the **exact JSON tree** below: object keys
sorted in Unicode scalar order (all protocol/field keys here are ASCII), arrays
in listed order, no whitespace/BOM, no normalization. Quote/backslash and
backspace/form-feed/newline/carriage-return/tab use their short JSON escapes;
remaining U+0000..U+001F use lowercase `\u00xx`; other scalar values are emitted
as UTF-8. JSON numbers, null, unpaired surrogates and duplicate object keys do
not occur in these trees. Every integer, including metadata, spans and capacities,
is a canonical decimal string. JSON booleans remain booleans. This determines
spelling; the tables below determine the tree.

`bytes(x)=length(J(x))` counts every key, quote, escape, delimiter and array/object
wrapper. No pretty-printed, tagged, implementation-specific or host-memory size
can substitute for this function. Receiving a different layout rejects
INPUT_SCHEMA before evaluating a term; it cannot become an alternative size
interpretation. A noncanonical spelling rejects INPUT_SCHEMA before evaluation
when a wire input is supplied. Schematic mathematical derivations must state the
same tree even though no transport decoder is implemented in this task.

## Types: t(T)

Types always use the following JSON arrays. Type references are not inlined
through named record/enum definitions. The finite acyclic schema resolves them.
The right side below is a JSON tree: metavariables stand for their specified
JSON subtrees or strings, not text interpolation.

| Type | t(T) |
| --- | --- |
| UInt64, UInt128, SInt128, Bool, Text | `["UInt64"]`, `["UInt128"]`, `["SInt128"]`, `["Bool"]`, `["Text"]` respectively |
| Amount<A> | `["Amount", A]` |
| Shares<V,H> | `["Shares", V, H]` |
| Rate<S> | `["Rate", decimal(S)]` |
| Price<A,B,S> | `["Price", A, B, decimal(S)]` |
| Quantity<U,S> | `["Quantity", [[symbol,decimal(exponent)],...], decimal(S)]` with U in canonical symbol order |
| Record<R>, Enum<E> | `["Record",R]`, `["Enum",E]` |
| Option<T> | `["Option",t(T)]` |
| Collection<T,N> | `["Collection",t(T),decimal(N)]` |
| Operation<O> | `["Operation",O]` |

Unit has no value embedding: it is a statement-result marker, never a valid
input/schema member. The Core annotation token `["Unit"]` is nevertheless
structurally recognized so ConstructNone(Unit) reaches static value-type
resolution and rejects TYPE_NAME. W(Unit,a) is undefined for every a and must
never be measured or admitted. A nominal reference `["Record","AbsentType"]`
is likewise a well-shaped annotation that rejects TYPE_NAME if undeclared. No implicit width/asset/scale/vault conversion is encoded.
Unknown type-annotation tags and wrong array arity reject INPUT_SCHEMA.
Well-shaped but undeclared identifiers reject TYPE_NAME at static resolution.
A resolved t(T) used by W is defined only after those checks.

## Values: v(T,a)

A type T and its schema are always known before interpreting v. Type metadata
is deliberately stored in t and Σ rather than repeated on every scalar value.

| Value type | v(T,a) |
| --- | --- |
| UInt64, UInt128, SInt128 | canonical decimal string for a |
| Bool, Text | JSON boolean or exact decoded JSON string |
| Amount, Shares, Rate, Price, Quantity | canonical decimal string for quanta/mantissa; indices and scale remain in t(T) |
| Record<R> | JSON object `{field:v(R[field],a[field]),...}` with exactly R's fields |
| Enum<E> | member identifier string |
| Option<T> None | `[]` |
| Option<T> Some(a) | `[v(T,a)]` |
| Collection<T,N> | `[v(T,a0),...,v(T,ak)]`, length at most N; an empty collection is `[]` |
| Operation<O> | `{"operation":O,"fields":v(Record<R_O>,a.fields)}` |

Option and collection empty arrays are unambiguous because T is known; a decoder
cannot infer T from an untyped value. The exact type-indexed standalone value
encoding is `W(T,a)={"type":t(T),"value":v(T,a)}`. The complete value byte bound
is **bytes(W)**, including the type wrapper. Mathematical value nodes remain
one scalar/compound constructor plus child nodes. W and t metadata do not add
value nodes. Record fields and collection/Some elements are child occurrences;
None, enum members and numeric/indexed scalar values each have one value node.
An Operation value has one node plus the value nodes of its fields record.
Value depth is one for a scalar/empty constructor and one plus maximum child
depth for a nonempty constructor. Snapshot and descriptor-sequence roots add
one depth level. The bound is64 for each individually checked value/aggregate;
type metadata and W do not add mathematical value depth.
Repeated occurrences count separately, even if the host shares a reference.

Every individual input field value and constructed value must fit 65,536 bytes
as W and 4,096 value nodes. Literal values use the same rule; simpler literal
limits remain in force. Aggregate snapshot/state bounds below are additional,
not a replacement for the individual value bound.

## Σ and aggregate snapshot trees

Σ is exactly the following object. Each map is a JSON object keyed by the
relevant declared identifier. Each shown key is required, including empty maps.
Roster arrays are sorted unique identifiers. Enum member arrays are sorted
unique identifiers. Declaration/field order for schema validation is increasing
identifier order. Source expression/statement evaluation order is unchanged.

```text
{
  "units": [unitSymbol,...],
  "assets": [asset,...],
  "vaults": [vault,...],
  "parties": [party,...],
  "recordTypes": {R: {field:t(T),...},...},
  "enumTypes": {E:[member,...],...},
  "fields": {field:{"type":t(T),"writeClass":"ordinary"|"financial"},...},
  "args": {name:t(T),...},
  "observations": {name:t(T),...},
  "operations": {O:R_O,...}
}
```

Every Quantity unit symbol must occur in Σ.units, including the symbols in
literal units and derived unit vectors. Unit atoms are not free text or implicit
asset names: the future source elaborator must bind source `unit` declarations
to this exact roster. An undeclared unit rejects TYPE_NAME during type/literal
checking. Symbols eliminated by normalization still must be declared in the
operand types; cancellation cannot hide an undeclared unit.

For schema acyclicity, recursively traverse every Option/Collection type to its
element and every record field to its type. Treat each named record R and
operation O as a dependency node: Record<R> references R; Operation<O> references
O; O has an edge to its operand record R_O. Record fields recursively contribute
all such edges even through Option or Collection. Reject any directed cycle
before value admission with TYPE_SCHEMA_CYCLE, including R→Option<Operation<O>>→O→R. Enum and indexed
scalar types are leaves; field/argument/observation types are roots. Finite
capacities or a None value do not permit recursive schemas.

This is a meta-notation for a closed object, not a JSON document with `...` or
union bars. R_O is a declared record type identifier; no host callback/function,
financial effect equation or signature is encoded there. Profiles still owe
those independent financial semantics. Σ cannot be extended by a caller merely
to classify a debt field ordinary; the complete reviewed profile must own it.

`Args`, `Obs` and `Pre` are each exactly `{name:v(T_name,value),...}`, using
Σ.args, Σ.observations and Σ.fields respectively. All declared fields occur
exactly once. `Post` has the identical field/type layout to Pre, with staged
ordinary values substituted and all other values unchanged. Neither snapshot
repeats Σ or wraps field values in W. Each snapshot has its own 65,536-byte and
4,096-value-node bound: one outer record node plus the field value nodes.
The input schema, Args, Obs and Pre are checked separately; there is no unstated
single 65,536-byte bound on their concatenation. Σ has its own 65,536-byte bound.

Schema-size nodes are exactly JSON-tree nodes (one for each object, array and
scalar occurrence; object keys do not add nodes). Σ must have at most 4,096 such
nodes and maximum JSON-tree depth64 (root depth1). Schema metadata limits remain
in force. These schema-size nodes are not expression work or financial values.

`Descriptors` is one array of v(Operation<O>,descriptor) trees in emitted order;
its size includes its outer brackets/commas, operation names and field wrappers.
It has at most128 entries, at most65,536 bytes and at most4,096 value nodes,
counting one outer sequence node plus each Operation value's nodes. Empty is
`[]`, two bytes and one sequence node. Each descriptor must also fit W's
individual bound. Exceeding an append's individual or aggregate size rejects
DESCRIPTOR_BOUND; other constructed values use VALUE_BOUND. No financial effect
list is substituted for this array.

An ExpressionPrepared presentation tree is exactly
`{"status":"ExpressionPrepared","post":Post,"descriptors":Descriptors,"workRemaining":decimal(workRemaining)}`.
A Rejected presentation tree is exactly
`{"status":"Rejected","code":code,"span":P,"nodePath":[decimal(index),...],"workUsed":decimal(workUsed)}`.
These result wrappers do not impose a new aggregate65,536-byte gate: the bound
is on Post/Descriptors individually, as stated above. Their presentation byte
size is nevertheless exactly bytes(tree), and their finite size follows from
the component limits. nodePath length is at most65 (statement plus constructor
depth), and indexes are bounded by the corresponding operand/list cardinality.
No caller can use a smaller presentation to waive the component bounds.

Locals and tentative writes are internal finite maps, not separately admitted
wire snapshots. Each stored value satisfies its individual bound, and there
are at most256 binders/writes by the statement bound. This gives a finite
product bound on these intermediate maps, not an additional 65,536-byte rejection
at each Let/NextWrite. Post assembly and Descriptors have the aggregate limits
above. Host memory usage and native proof cost are not established by these
logical bounds.

## Core and span trees

Core metadata operands are encoded by role:

- Identifier, enum member, view and field operands: the exact strings.
- Width/capacity/scale/integer literal operands: canonical signed decimal
  strings at structural admission. The constructor's static rule then enforces
  its width, sign and range; e.g. LitAmount(A,-1) is well-shaped but TYPE_LITERAL.
  A JSON number or noncanonical string such as "+1" is INPUT_SCHEMA instead.
- Boolean/Text literal operands: the JSON boolean/string itself.
- Type operands: the structurally recognized type-annotation arrays above,
  then resolve to t(T) during checking; recordType/enumType operands use the
  named identifier. No arbitrary free type-name string stands for a type array.
- Quantity units metadata: U as the type table's ordered array of pairs.
- Evaluated expression operand: recursively N(child).
- Expression list: array of N(children) in lexical order.
- Named expression list: array `[{"name":field,"value":N(child)},...]` in
  lexical order, never sorted as a substitute for evaluation order.

A node N is exactly `{"constructor":name,"operands":{operandName:encodedOperand,...},"span":P}`,
with operand names exactly as expression-signatures.json. No inferred tag,
unknown operand, extra type annotation or missing span is allowed. Shared AST
objects are encoded as repeated occurrences, not pointer IDs.

P is exactly `{"kind":"source"|"synthetic","start":decimal(start),"end":decimal(end)}`.
Source is a separate supplied well-formed UTF-8 byte string of at most65,536
bytes. A source span requires that artifact and is within its actual byte
length, as specified by E; a caller-supplied length without the source cannot
establish a source span. Range validation alone is not source/Core correspondence.
A synthetic span is **always start=end=0** and establishes no source location.
Reject a synthetic span claiming a nonzero source offset with INPUT_SPAN.
Span integers use signed canonical strings structurally; negative/out-of-order
values reject INPUT_SPAN, not a late expression error. A pure fragment
judgment can use synthetic spans; only a future source elaborator can produce
source span evidence. Spans and constructor/operand keys are included in
canonical Core byte size but are not extra AST/work nodes.

An action A is exactly `{"statements":[N(statement),...],"span":P}`. The final
Ensure suffix is part of this same lexical array. A's canonical bytes must fit
65,536; its AST occurrence count must fit4,096; constructor nesting depth64;
statement count256. Action/root/span wrappers add bytes, but not work nodes.
The source's own 65,536-byte bound is separate. Size failure of a supplied Core
node/action is INPUT_BOUND at admission, workUsed0. Literal values and metadata
still require their finite-type checks. This does not install a trusted Core
input API in the existing funded-source runtime.

Admitted schema/input bounds are tested before reduction. For pure construction
or candidate finalization, compute this exact mathematical embedding and size;
if it exceeds its bound, reject VALUE_BOUND at the constructor/action span with
no published post-state. Emit append size failure instead uses DESCRIPTOR_BOUND.
No implementation may return UNKNOWN, omit metadata, substitute a tagged layout
or defer the size check until after publication. An unknown wire type tag rejects INPUT_SCHEMA; a well-shaped reference with
missing schema definition rejects TYPE_NAME during schema/static checking.
Neither can reach value sizing. The old proposal's wire is not implicitly inherited.


## Structural versus static failure ordering

Schema/metadata validation first checks JSON tree shape, permitted tags/keys,
scalar kinds, canonical integer spelling, raw aggregate bytes, schema/Core
structural nodes and spans. Type-dependent W sizes and value node/depth counts
are computed only after type resolution.
It does not check a constructor literal's numeric domain, Text1024-byte bound,
unit-vector ordering/uniqueness, declaration membership, or an expression's
operand/result type until static checking. Thus a LitUInt with canonical2^64
passes shape and fails TYPE_LITERAL; LitText1025 ASCII characters can pass the
whole-action65,536-byte bound and then fail TYPE_LITERAL. A unit-vector pair
array of valid identifiers/integer strings is structurally valid; duplicated
unit symbols on LitQuantity fail TYPE_LITERAL, undeclared symbols TYPE_NAME.
Values in supplied snapshots are different: after resolving their declared
types, wrong representation shapes reject INPUT_SCHEMA, individual W or typed
node/depth bounds reject INPUT_BOUND, and mathematical domain violations reject
INPUT_VALUE, in that order in S0's snapshot stage. Raw snapshot byte bounds were
already checked structurally. All of this occurs before reduction.

A named expression list is an array, so duplicate field *names* are structurally
representable and reach ConstructRecord's static TYPE_DUPLICATE_FIELD rule.
Duplicate keys in any JSON object still reject INPUT_SCHEMA. ReadPre's view
is a string at shape validation: pre/post/next are recognized source/Core view
spellings; next fails TYPE_NEXT_READ statically, post outside Ensure fails
TYPE_POST_SCOPE. Other view strings reject INPUT_SCHEMA. Unknown operation or
local/argument/observation/field names that are valid identifier strings reach
TYPE_NAME, rather than being disguised as malformed wire trees.

These rules resolve the phase for the existing specified cases. Cases are
mathematical terms, not serialized input captures: their assumed well-formed
metadata must be embedded exactly as above to reproduce the named predicate.
A contradictory earlier admission defect would invalidate that case isolation.
