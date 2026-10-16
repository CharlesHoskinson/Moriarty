# Proposed A layout: ordered typed tables and complete roots

Every choice in this file is a recommendation awaiting review. Field names are
descriptive proposal names, not new public Rust types. `crosswalk.json` gives
the exact constructor payload map. No existing source enum is extended.

The historical L2 domain, canonical-order, code-reference, header and binding
repairs were materialized verbatim from Opus owner guidance. L3 clarifies only
the F1-F5 matters directed by the fresh Opus review, as recorded in
[OWNER-REPAIR-PROVENANCE.md](OWNER-REPAIR-PROVENANCE.md). It is not an approving
vote, adoption, executable validation or a new security judgment by Astra.

## Domain and parameters

Represent finite typed A programs and A observations in three explicitly checked domains. (1) Program domain: one Program prepared by prepareA. Every Snapshot carried by an observation must satisfy `snapshot.program == PreparedA.fullProgram` by typed structural Program equality: entry, then every function, parameter, type and body, in order. Arc pointer identity and digest equality are never used. A violation is `ProgramMismatch`, located at that Snapshot.program. Every held expression, function, argument-expression slice, arm slice and name must have a candidate in the program's structural closure, as defined under Runtime code references. Otherwise the result is `OutsideProgramClosure`, and the payload is never dropped, coerced or replaced. (2) Representation domain: images satisfying the Representation-domain definitions below. A violation is `WrongRepresentation`. (3) Observation domain: typed Snapshots and StepRecords satisfying the Observation-domain conditions below. A violation is `OutsideObservationDomain`. These refusals and `Capacity` are located tooling results. None of them is Outcome::Rejected, Outcome::Exhausted, error4 or any other semantic result. Arbitrary-state import remains a separate unsupported consumer, even where the public Rust constructor permits a value. No reachability or coverage proof is supplied. The domains are definitions, not theorems. They do not narrow A machine semantics, and A remains an incremental representation layer, not the final interpreter's scope.

Choose static mathematical capacities, with no numeric defaults:

| Symbol | Fixed table/limit |
|---|---|
| N, NB | distinct names and UTF-8 name bytes |
| X, XC | expression occurrences and ordered expression-child cells |
| F, P | source-ordered functions and parameters |
| A, R | source-ordered arms and pattern rows |
| K, KF | immutable constant nodes and their ordered field slots |
| D, DF | runtime immutable data nodes and their ordered field slots |
| E | persistent environment cells |
| V | ordered runtime value-reference slots for argument/completed lists |
| S | physical frame slots |
| C | control-spine rows, including a final non-Push row |
| SL | maximum representable source stack_limit, independent of S |
| O, OP | source-occurrence records and path components |
| T, Q, QF | type declarations, constructor declarations and constructor-field type slots |

The A builtin schema requires T>=4, Q>=4 and QF>=5, with four declared
constructor-field lists; other static capacities can be zero when their table is unused. A source
stack limit is stored exactly and is not replaced by S. Realization admission
must establish adequate physical capacity for its declared semantic domain;
a physical capacity refusal remains tooling-only, not error4 or A Exhausted.
No smaller hidden physical limit may be silently passed off as the source limit.
This design states the representation problem; Opus owns admission/resource
judgments. Fuel is exactly UInt128, not a bound derived from a test's fuel value.

For every capacity Z, `id(Z)` has width max(1,ceil(log2(max(1,Z)))) and denotes
an index below the corresponding active length. `count(Z)` and `start(Z)` have
width max(1,ceil(log2(Z+1))); lengths/start positions can equal Z. A slice is
`{start:start(Z),len:count(Z)}`, with start+len<=activeLength; empty slices have
start=0. Arithmetic here is mathematical, never overflowing host addition.
`Option<Id>` is `{present:Bool,index:id(Z)}`; absent index is zero. Index zero
is a valid present row. Every optional environment uses this same distinction.

### Representation-domain definitions

An image (ProgramBlock, StateBlock or ObservedRecordA envelope) is in the representation domain iff every item below holds. Each item is a decode-domain definition, not a proof. The circuit components that enforce them (enable guard, w-bit lanes, multi-byte id reconstitution, padding checks) are operation-manifest obligations.

- References: every present id is below its table's active length. Every ValueRef index is below the active length of its selected space. Every slice has start+len<=active length, and empty slices are (0,0). A nonempty StateBlock remainingExprSlice must be a suffix of one Call row's childSlice; a nonempty StateBlock armSlice must be a suffix of one Match row's armSlice. A non-suffix decoded state slice, including one spanning separate source slices, is WrongRepresentation in both Canonical and Step decode modes. This suffix-membership condition is a StepInv row-check obligation (Opus F2). Every active length is <= its capacity. ConstantFields and Literal Expressions use space=Constant. At a canonical boundary, every ValueRef reachable from a StateBlock root uses space=Data.
- Lanes: every enum lane holds a listed tag; unused codes of a non-power-of-two enum are invalid. Every unsigned lane has zero bits above its width. Every Bool lane is 0 or 1. u8 and u8[16] lanes accept every byte value.
- Padding: rows at or above their active length are zero. Columns unused by the active variant are zero recursively, including the whole pendingFrame unless tag=Push. An absent option has present=0 and index 0. Inactive Root outcome columns are zero. Zero is never itself a discriminator.
- Program rows: Expressions[i].occurrenceId=i. Child count is 0 for Literal/Var, 2 for Let/Binary, 1 for Unary/Match, 3 for If, and any count for Call. nameId is used only by Var/Let/Call, and armSlice only by Match. Functions[i].sourceTableIndex=i. Names are distinct, valid UTF-8, strictly ascending by bytes, and contiguous in NameId order; the empty name is permitted. Types, Constructors and ConstructorFieldTypes equal the fixed A rows. Header entryExprId=0 and activeExpressions>=1. The complete ProgramBlock must equal prepareA(decoded Program) bit for bit.
- Typing:
  - A UInt node has kind=0, typeId=0 and zero ctor/field/bool columns.
  - A Bool node has kind=1, typeId=1, zero uint bytes and boolValue in {0,1}.
  - A Constructor node has kind=2 and zero scalar columns. Constructors[ctorId].owningType=typeId. fieldLength equals the declared field count. Each field node's typeId equals the declared field type: Nil:[], Cons:[UInt,List], Leaf:[UInt], Branch:[Tree,Tree].
  - Pattern rows use name0/name1 exactly as listed. Type lanes are 0..3.
- Acyclicity, as checkable strict orders replacing `must unfold finitely`: every Expressions child and arm bodyExprId is greater than its parent's ExprId. Every ConstantFields entry is an earlier Constant. Every DataFields entry is an earlier Data node or a Constant. Every EnvCell parent is an earlier cell.
- Spines: when controlLength=n>0, rows 0..n-2 are Push and row n-1 is not Push. Rows >=n are zero. FrameSlots rows >=depth are zero. Every typed Control unfolds to such a spine, so this excludes no typed value.

### Observation-domain conditions

StepInv is only a label for a future row-check obligation. It is not an
existing component, implemented invariant checker or proved result (Opus G3).

These typed conditions hold for every Snapshot and StepRecord produced by `reference.rs::Machine` from `Machine::new`. This is a source reading, not a proof. A violating hand-constructed value receives `OutsideObservationDomain{condition,locator}` and is never coerced.

- O1: `remaining_fuel <= initial_fuel`. This is the documented machine invariant; fuel is decremented only after the line-84 zero check.
- O2: `frames.len() <= stack_limit`. Line 135 installs a frame only when `frames.len() < stack_limit`, and replacements keep depth.
- O3: `outcome == Pending` iff `control` is Some. `new` starts with Some(Eval). Lines 80/82/84 set None together with a non-Pending outcome. Every other step sets Some.
- O4: a StepRecord has `before.outcome == Pending`, a `ruleStartLine` listed in rule_line_tags, and the `charge` given for that tag (0 for 80/82/84, 1 otherwise). `before.program` and `after.program` both equal fullProgram; otherwise the result is ProgramMismatch at that side.

`(Pending, None)` violates O3. Encoding refuses it at `Root.controlLength`. `Machine::step` returns None there, so no StepRecord, record image or accepted post-image exists for it. It is not Rejected, Exhausted or error4. A terminal Snapshot that satisfies O3 likewise has no StepRecord; that is the source's absence of a transition, not a refusal.

The following are not observation conditions, because genuinely produced observations can violate them when capacities are small: `depth <= S`, `controlLength <= C`, `stack_limit <= SL`, and arena active lengths <= capacities. They yield `Capacity{table,needed,limit,locator}`. Error payloads are unrestricted: Error, PrimitiveResult Err and Rejected keep all 256 u8 values. No condition requires nonempty or distinct names, distinct parameters, `stack_limit>0`, nonzero fuel, `controlLength<=2` or a whitelist of error codes.

### Scalar lanes and record ordering

Logical scalar lanes are UInt128, Bool, bounded unsigned integers, and enums.
UInt128 is sixteen little-endian bytes, consistent with the current source-schema
comparison baseline. Other unsigned lanes of width w use ceil(w/8) little-endian
bytes, with the unused high bits zero. Booleans use one Boolean lane. An enum
with v variants has max(1,ceil(log2(v))) bits and the listed tags only. The A
error payload is all eight bits of u8, not an invented whitelist. No packed
field/limb optimization or cryptographic field assumption is adopted. Native
embedding/range constraints remain a separately reviewed lowering obligation.

Record columns are flattened in listed order, recursively expanding subrecords;
arrays are ascending-index row major. Offsets are the sum of the preceding
column widths. All columns exist at fixed width; inactive variant columns and
inactive table rows are zero. Inactive zero is not a sum discriminator. The
variant mask and active length determine whether a row/column is meaningful.
These are proposed encoding rules, not a claim they are constrained already.

## Immutable program block

Every step receives the same logical ProgramBlock. It contains the entire entry
and source-ordered function table, including unused bodies, not a chosen path.
There is no hash substituted for Program structural equality in the host
comparison. Binding supplied program rows to a proof statement is unresolved
and Opus-owned; copying a source digest into metadata does not solve it.

ProgramBlock order is: ProgramHeader, Names, NameBytes, Occurrences, OccurrencePaths, Expressions, ExprChildren, Functions, Parameters, Patterns, Arms, Types, Constructors, ConstructorFieldTypes, Constants, ConstantFields. ProgramHeader has sixteen ordered columns. The first is `entryExprId:id(X)`. It is followed by fifteen active lengths, in the table order above, each `count(Z)` of its own capacity: activeNames:count(N), activeNameBytes:count(NB), activeOccurrences:count(O), activeOccurrencePaths:count(OP), activeExpressions:count(X), activeExprChildren:count(XC), activeFunctions:count(F), activeParameters:count(P), activePatterns:count(R), activeArms:count(A), activeTypes:count(T), activeConstructors:count(Q), activeConstructorFieldTypes:count(QF), activeConstants:count(K), activeConstantFields:count(KF). In A, entryExprId=0, activeTypes=4, activeConstructors=4 and activeConstructorFieldTypes=5. The header carries no profile, layout, source digest or rule metadata.

| Table | Ordered columns / meaning |
|---|---|
| Names | byteStart, byteLength; exact UTF-8 bytes in NameBytes |
| Occurrences | ownerTag(Entry=0/Function=1), functionIndex, pathStart, pathLength |
| Expressions | tag, occurrenceId, valueRef, nameId, binaryOp, unaryOp, childSlice, armSlice |
| ExprChildren | ExprId; each source node's children in order |
| Functions | sourceTableIndex, nameId, parameterSlice, returnType, bodyExprId |
| Parameters | nameId, typeId |
| Patterns | tag, name0, name1 |
| Arms | patternId, bodyExprId; each Match's arms occupy an ordered slice |
| Types | kind(UInt=0/Bool=1/List=2/Tree=3), ctorStart, ctorLength |
| Constructors | owningType, sourceOrder, fieldTypeSlice |
| ConstructorFieldTypes | typeId |
| Constants | ValueNode below; constant-space children only |
| ConstantFields | ValueRef below, with space=Constant |

Names are distinct exact strings sorted by UTF-8 lexicographic order, not parsed
as numbers. This supports the host Name type; a K/Quint common-domain adapter
additionally requires canonical signed-decimal spellings, without changing names.
Names contains exactly the distinct names occurring anywhere in Program syntax:
function names, parameter names, Var/Let/Call names and pattern binders, including
unused syntax. It contains no other names. Sorting and deduplication affect the
Names table only; every syntax occurrence and duplicate binder remains in its
own source position. No hashing or comparison of arena addresses defines name
equality. This exact membership clarification is Opus F1(a).

Assign expression occurrences in preorder: entry first, then each function body
in original function order, visiting each node's children in their semantic
source order. Let value/body, binary left/right, unary operand, if condition/
then/else, call arguments and match scrutinee/arm bodies have that order.
Every occurrence gets its own expression row even if Arc nodes or syntax are
equal. Prepared expression children obey the Representation-domain strict
orders above. Calls name functions; they never point to an unrolled body.
SourceOccurrence owner and AST path are retained, distinct from execution ordinal.
For a Match node, OccurrencePaths childPosition is 0 for its scrutinee and k+1
for the body of its zero-based arm k, in source arm order (Opus F1(b)). These
source-path child positions are not execution-event ordinals.

For non-Match nodes, childPosition is the child's zero-based index in the order listed above. A root occurrence (entry or function body) has pathStart=0, pathLength=0, and an Entry occurrence has functionIndex=0.

The preceding paragraph is the verbatim Opus G2 path clarification.

Immutable prepared links are occurrence links. Every childSlice cell, arm bodyExprId, function bodyExprId and entryExprId names the occurrence produced by this preorder, so each child keeps its own, greater ExprId. Preparation never redirects an immutable child, arm or body to an earlier structurally equal occurrence. It never shares child or arm cells between occurrences. The least-equal choice under Runtime code references applies only to references held in StateBlock rows.

After assigning expression IDs, append each expression's child cells in ExprId
order; append Match arm rows in that same order and their pattern rows in source
arm order. Append parameter slices in function order, and constant subtrees in
literal ExprId order, each in left-to-right post-order: visit constructor fields
left to right before their parent, with no interning (Opus F1(c)). Occurrences
follow expression order; append each complete path in occurrence order. Name
byte slices follow sorted NameId order. Builtin Types/Constructors use the fixed
orders below; constructor field-type slices follow constructor order. These
rules determine all immutable table ordering independently of evaluated paths.

Expressions use tags Literal=0, Var=1, Let=2, Binary=3, Unary=4, If=5, Call=6,
Match=7. Literal uses valueRef; Var uses nameId; Let uses nameId and two child
IDs; Binary uses binaryOp and two; Unary uses unaryOp and one; If uses three;
Call uses nameId and its ordered child slice; Match uses one child and armSlice.
The other columns are zero. Binary tags Add/Sub/Mul/Eq/Lt/Cons/Branch=0..6;
Unary Leaf/Not=0..1; Type UInt/Bool/List/Tree=0..3. Pattern Var/Nil/Cons/Leaf/
Branch=0..4; Var/Leaf use name0, Cons/Branch use name0 then name1, Nil none.
Duplicate binders/names are not silently removed during preparation. Existing
binding behavior, not a new uniqueness assumption, determines their outcome.

Function lookup scans original order and selects the first matching name. sourceTableIndex equals its array index and is retained explicitly. Parameters stay in source order. No sorted or direct-name map changes duplicate-function behavior. `Control::Enter(Some(function))` is encoded as functionPresent=1 and functionId = the least row r with `program.functions[r] == function`, by exact typed Function equality: name bytes, ordered parameter names and types, return type, and structural body. A same-name row that differs in any of these is not a match. If no row matches, the result is `OutsideProgramClosure{field:Enter.function}`. For machine-produced observations, this least row is the row selected at line 200. `find` returns the first row with that name, and any structurally equal row has the same name, so no earlier equal row exists. In general, standalone encoding claims structural identity, not table-index identity.

### Runtime code references

A StateBlock reference to immutable code selects the least prepared position whose decoded value is structurally equal to the held source value:

- Eval, Bound and CaseResult exprId, and Frame expr0/expr1: the least ExprId whose prepared subtree equals the ExprRef.
- Args remainingExprSlice: the least start among suffixes of a Call row's childSlice whose cells decode, element by element, equal to the held Vec<ExprRef>.
- Frame Match, Cases and CaseResult armSlice: the least start among suffixes of a Match row's armSlice whose arms (pattern and structural body) are equal.

Empty slices are (0,0). Every produced slice is such a suffix (`args[1..]`, `rest.remove(0)`, `arms.clone()`, `arms.remove(0)`). When encoding a held typed source slice, absence of a program-closure candidate
is `OutsideProgramClosure`. When decoding an image, a non-suffix state slice is
instead `WrongRepresentation` in both modes, as required above by Opus F2.
Names held in state (Invoke, Args, environment keys) use their unique NameId; a name absent from Names is `OutsideProgramClosure`. Least selection is a canonical-boundary rule. Mid-segment images may hold any candidate that meets the same suffix and equality condition, and the operation manifest must state each rule's exact reference output. Equality is structural, never Arc identity, so encoding or boundary projection may move a reference to a different but equal occurrence. Provenance exists only in a separately supplied ExecutionOrigins observation, and is reported as absent otherwise.

## Values, recursive structure and argument sequences

ValueRef is `{space:Bool(Constant=0/Data=1),index:id(max(K,D))}`. The selected
space supplies the actual active-length bound; an absent reference is represented
by the enclosing option/mask, never a sentinel value. ValueNode column order:
`kind(UInt=0/Bool=1/Constructor=2), typeId, ctorId, uintBytes[16], boolValue,
fieldStart, fieldLength`. Constant and runtime field slices address their own
respective field-slot tables; each slot is a ValueRef.

When a node is appended to Constants (respectively Data), its ordered field references are appended at that same moment, as one contiguous block at the current end of ConstantFields (respectively DataFields), in field order. A node with no fields has fieldStart=0 and fieldLength=0.

The preceding paragraph is the verbatim Opus G1 field-slot timing clarification.

Builtin constructor IDs are Nil=0, Cons=1, Leaf=2, Branch=3, with declarations:
Nil:List[], Cons:List[UInt,List], Leaf:Tree[UInt], Branch:Tree[Tree,Tree]. In A,
active Types=4, Constructors=4 and ConstructorFieldTypes=5 even if capacities
are larger. UInt/Bool have empty constructor slices; List uses (0,2) and Tree
uses (2,2). Constructor sourceOrder is 0/1 within its owning type. UInt
and Bool nodes use their matching type IDs, zero ctor/field columns and only
their own scalar payload. Constructor nodes have zero scalar columns, the
declared type/ctor pair and exact ordered field count. `Value::List` expands to
ordered Cons/Nil structure; its elements remain UInt values. `Tree::Leaf` wraps
an UInt node; Branch retains left then right. Empty list, UInt0, Bool false and
absence remain distinct. There is no nested-list/general-ADT support claim.

Constants and data use child-before-parent construction. Constant references
remain within Constants; a runtime child may reference any constant or an
earlier runtime node. All scalar values and complete structures survive; field
order is source order, not arena-address order. Equality and validation are
structural operations, not equality of ValueRef pairs. Shared nodes can be read
more than once; occurrence accounting is a separate profile matter.

ValueSlots stores ordered ValueRefs for `Invoke`, `Enter` and completed Args.
It is separate from constructor fields and cannot change a node's children.
Slices retain empty versus nonempty lists explicitly. Runtime source expression
and arm slices reference the immutable program's ordered cells, including suffix
slices. A located closure refusal replaces any attempt to guess a foreign slice.

## Environments and saved environments

EnvRoot is Option<EnvCellId>; None is the empty map. EnvCell column order is
`nameId,valueRef,parent:EnvRoot`. Cells are immutable and append-only within
a segment; a parent index is lower than its child. Decoding visits newest to
oldest and retains the first binding for each name. Thus an insertion creates
one cell with parent=old root, including shadowing. An existing saved root still
decodes the full old map. Fresh function binding begins from the empty root.
No mutation of shared cells changes a caller's saved environment.

This physical chain may retain shadowed bindings, while an A Environment is a map. Insertion (Let at line 145, bind, matching) appends one cell whose parent is the old root, so every parent index is lower than its child's. Decoding walks newest to oldest and keeps the first binding of each name, which equals BTreeMap insert and shadow semantics. At a canonical boundary, each root occurrence is encoded independently as its distinct-key map. For each binding, in ascending exact-Name byte order: first append its value to Data in post-order, then append one EnvCell {nameId, valueRef, parent}. The parent is the previous cell of this same root, or present=0 for the first cell. The root points at the last cell. An empty map is EnvRoot{present=0,index=0} and appends nothing. No cell or value is shared between root occurrences in the baseline. The decoder's map meaning is compared, not chain identity or history. The representation does not assert that all encoding refinements are proved.

Every Frame and Control field named savedEnvironment/bindings/environment gets
its own complete EnvRoot. `Option<Environment>` adds a separate present bit:
None = failed/absent binding; Some(EnvRoot=None) = successful empty map. These
must never collapse, notably in zero-argument calls and matching.

## Frames and recursive control

FrameSlots is a fixed S vector with dynamic depth. Row0 is the next frame to
consume, exactly matching public A Snapshot. Push shifts the prior active rows
one position toward higher indices and installs row0; pop shifts toward zero;
replacement changes row0 at the same depth. Those are candidate circuit
operations over bounded rows, not host-computed accepted after-state flags.
S and stack_limit remain different fields. A scheduled Push carries a pending
frame in control but does not increment depth until the source push rule.

FrameRow columns: `tag,nameId,binaryOp,unaryOp,typeId,expr0,expr1,savedEnv,
valueRef,completedSlice,remainingExprSlice,armSlice`. Tags and used columns:

| Tag | Source Frame | Payload in order |
|---|---|---|
| 0 | Let | nameId, expr0=body, savedEnv |
| 1 | Left | binaryOp, expr0=right, savedEnv |
| 2 | Right | binaryOp, valueRef=left; no invented environment |
| 3 | Unary | unaryOp |
| 4 | If | expr0=then, expr1=else, savedEnv |
| 5 | Match | armSlice, savedEnv |
| 6 | Args | nameId, completedSlice, remainingExprSlice, savedEnv |
| 7 | Return | typeId=expected, savedEnv |
| 8 | Scope | savedEnv |

Control's sole recursive field is Push.next. Store it as a fixed C vector in
root-to-leaf order with dynamic controlLength: zero means Snapshot.control=None;
otherwise all rows except the final row are Push and their next is row i+1.
The final row is one of the nine non-Push constructors. No internal reference
cycle or missing terminal row is part of this declared encoding domain.

ControlRow columns: `tag,exprId,valueRef,pendingFrame:FrameRow,environment:EnvRoot,
nameId,valueSlice,functionPresent,functionId,bindingsPresent,bindings:EnvRoot,
typeId,armSlice,primitiveIsOk,errorCode`. PendingFrame is inline and distinct
from FrameSlots; zero inactive fields apply recursively. The complete crosswalk
uses tags Eval=0, Return=1, Push=2, Invoke=3, Enter=4, Bound=5, Cases=6,
CaseResult=7, PrimitiveResult=8, Error=9. In Push, next is the next active
control row; environment is the environment to install. In CaseResult the
optional bindings and saved environment occupy distinct columns. PrimitiveResult
uses primitiveIsOk plus either valueRef or errorCode, never two independent
options whose both-present/both-absent meanings are unclear.

## State root and complete physical order

StateBlock order is: Root, Data, DataFields, EnvCells, ValueSlots, FrameSlots,
ControlRows. Root column order:

`activeData,activeDataFields,activeEnvCells,activeValueSlots,depth,controlLength,
environment:EnvRoot,stackLimit:count(SL),remainingFuel:UInt128,
initialFuel:UInt128,outcomeTag,outcomeValue:ValueRef,outcomeError:u8`.

Outcome tags Pending=0, Done=1, Rejected=2, Exhausted=3. Only Done uses value;
only Rejected uses error. Exhausted has no A reason payload. A carries no B
referenceCursor, consumedWork or helperWork field. Charged work is a checked
diagnostic `initialFuel-remainingFuel` on observations satisfying the existing
counter relation, never a substitute for retaining both fuel values.

SL is a mathematical bound, not a platform-sized serialized integer. Encoding retains the producing Snapshot's exact usize stack_limit. stack_limit>SL is Capacity. Decoding into a host that cannot hold the value is a located HostUnrepresentable failure. Physical S bounds installed frame rows; it is not the source limit. The step relation must have no accepting post-image, and tooling reports Capacity, when either of these occurs:

- a Push would make depth exceed S while depth<stack_limit;
- any arena or control row would exceed its capacity.

Neither case may ever produce error4 (line 140), Exhausted or any other outcome. Fuel is exact UInt128 and is never replaced by a bound derived from a test's fuel. Admission must state the semantic domain in which S and the other capacities suffice. No numeric capacity, cost or k is chosen here.

The paired observation interface supplies one ProgramBlock. It is shared by reference for pre and post, and for every record of a trace. The machine never changes Snapshot.program. Any observed Snapshot whose program differs from PreparedA.fullProgram is ProgramMismatch. Materialization reconstructs the same complete Program for each Snapshot, including the entry and every unused function, parameter and body. The remaining seven Snapshot fields are explicit roots above. Physical active lengths are not semantic allocation counters. StepRecord ruleStartLine and charge live only in the ObservedRecordA envelope (crosswalk observation_envelope), outside ProgramBlock and StateBlock. The envelope does not freeze trace identity, exposure or statement binding.

The following are retained Opus obligations, not adopted solutions:

- binding the ProgramBlock to the native statement so a prover cannot choose it (public data, or a committed digest with an in-circuit opening; unselected);
- binding each step's post to the next step's pre across proofs;
- binding profile, layout and K source identity;
- public/private labels;
- where canonical-ProgramBlock and least-equal canonicality are enforced (Opus
  F4). A bound prepare-time canonical-representative table or a
  verifier-recomputed ProgramBlock are the owner-listed alternatives. The
  mechanism remains UNSELECTED; least-equal enforcement must not rely on
  per-step in-circuit search.

Metadata equality or a copied digest satisfies none of these, and an unknown binding is never declared verified. The baseline treats ProgramBlock as a dynamic operand of one interpreter shape. CTFE or program specialization is a later, separately identified realization of the same relation (L09).

## Canonical boundaries and mid-segment images

Recommended boundary encoding uses no structural interning. Program constants are visited in expression occurrence order. A StateBlock is rebuilt from empty Data, DataFields, EnvCells and ValueSlots by visiting roots in Snapshot field order:

1. ControlRows row-major, for i=0..controlLength-1. If row i is Push, visit its pendingFrame payload in Frame source order, then Push.environment, then continue with row i+1. The environment is never visited after the entire tail. The terminal row visits its fields in Control source order.
2. Root.environment.
3. FrameSlots from row 0 (top) to depth-1, each in Frame source order.
4. The Done value.

Field visits:

- An expression, arm slice, function or name is a program reference and appends nothing.
- A Value is appended in post-order: each Constructor's fields left to right, then the node. For example, List [x0,x1] appends UInt x0, UInt x1, Nil, Cons(x1,Nil), Cons(x0,..).
- An Environment or Some(Environment) is appended by the canonical map rule. None appends nothing.
- A Vec<Value> (Invoke, Enter, Args completed) first appends each value left to right, then reserves one contiguous ValueSlots slice holding their refs. An empty list is (0,0).

Root active lengths equal the appended counts. Inactive storage and unused variant columns are zero. No pointer identity affects traversal.

Mid-segment data/field/environment/value-slot arenas may retain unreachable
old rows; appends do not rewrite prior active rows. Frames/control rows are
reusable vector positions with complete overwrite and release zeroing. An
ordered operation manifest must specify concrete append order and full pre/post
write masks for each rule before circuit implementation can claim determinism.
This representation proposal does not fill that future circuit obligation with
a host decoder or a source-metadata assertion. Boundary projection may rebuild
reachable roots only through a separately reviewed bounded adapter; it is not
an implicit operation inside a machine step. The applicable B A3/A4 requirements
remain owner-governed; A has no invented allocation-history counters to project.

Opus F3 adds a separate canonical-unfolding requirement to L05. With no
interning, repeated Branch(t,t) can produce exponentially many canonical Data
nodes in the number of charged steps, even when the mid-segment DAG shares
only linearly many nodes. Canonical-boundary admission for checkpoints, chunks
and external claims must cover the unfolded capacity envelope, not just fuel,
live DAG size or mid-segment high-water occupancy. Overflow is Capacity, never
a semantic Outcome. Deterministic maximal-sharing canonicalization would be a
separately reviewed profile change; it is not selected here.

Opus F5 requires host encoders, including closure search and canonical
unfolding, to check capacity incrementally and refuse at the first overflow,
never after completing a full unfold. H1 applies to every proposed adapter:
host tooling only, with no untrusted or network consumer. This records the
owner requirement; it introduces no host admission or numeric bound.

Capacity reporting must distinguish physical high-water occupancy from live
reachable size. A trace envelope must cover append/copy high-water usage across
its chosen segment, not only its largest decoded snapshot. Splitting a segment
can invoke a reviewed boundary operation; it cannot alter fuel, outcome or
introduce a source commit. No quantitative cost or admission result is implied.
