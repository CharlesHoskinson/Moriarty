# Internal L4 preparation and typed state codec source

Status: source-only proposal. The private `#[cfg(test)] mod layout_a` connects
this source to the test source graph. Compilation and all 25 normal tests
are UNRUN. Core test dependencies include SDK code; no test command is admitted
or implied. This is H1 host tooling only. The decoder consumes an internal typed
image with a retained PreparedA; no public byte importer or network consumer is
exposed. Module registration is source integration, not an observed compile.

`prepare_a(Arc<Program>, Capacities)` retains the complete source Program by Arc,
its syntax occurrence metadata, exact sorted names, and a complete ProgramBlock.
It does not evaluate syntax, follow named calls, statically reject runtime type
errors, search for a function by name, or reuse the older ProgramImageDraft.
Duplicate function/parameter/binder occurrences remain in source order. Only
the Names table deduplicates strings, with exact UTF-8 byte sorting.

Capacities has no Default implementation. `tables` follows the fifteen TableId
variants in this order: Names, NameBytes, Occurrences, OccurrencePaths,
Expressions, ExprChildren, Functions, Parameters, Patterns, Arms, Types,
Constructors, ConstructorFieldTypes, Constants, ConstantFields. `data` supplies
D for the specified ValueRef id(max(K,D)) width; preparation allocates no Data
table. Capacities are u128 mathematical inputs within that host carrier; values
that cannot be represented as a host table length receive HostUnrepresentable.
No numeric capacities or resource admission are selected by this module.

Every table uses a named row type. ProgramHeader is entryExprId followed by
fifteen active lengths in the same order. All inactive table rows and unused
variant columns are zero. Root source paths are (0,0); Entry functionIndex=0 is
padding, including when Functions is empty. Function-owned occurrences retain
the actual source function index. Match paths use scrutinee 0 and arm k+1;
other child positions are zero-based in source order. ExprChildren and arm
blocks are written in ExprId order after preorder identities are assigned.

Explicit task stacks avoid Rust recursive traversal of input Expr and Tree.
Occurrence/path/child/arm counts are reserved before pending traversal expansion.
Names and byte counts are checked before copying a newly discovered name.
Literal nodes and field counts are reserved incrementally before expansion.
Pending reservations count toward the same supplied capacity, including shared
subtrees visited twice. This does not measure host work or establish resource
adequacy. The caller still owes the F3 unfolded envelope and existing admission.

Literal UInt128 uses sixteen little-endian bytes. Lists become UInt/Cons/Nil;
Tree Leaf wraps a UInt and Branch holds two Tree references. Children are
visited left-to-right, nodes appended postorder, with no interning. Each node's
field-reference block is appended with that node after its children, as L4 G1
requires; fieldless slices are (0,0). Four type declarations, four constructors
and five constructor-field types are always included.

`program_bytes()` expands header/table rows and subrecords in L4 order. Unsigned
lanes use the supplied capacity-derived widths and little-endian bytes; enum
and Boolean lanes occupy their specified byte expansion. It supplies neither
a native embedding nor statement binding/digest. It does not range-check an
unused zero id against an empty active table.

`state_encoding::observed_state_bytes_a(&PreparedA, &StateCapacities, &Snapshot)`
calls the real canonical state encoder, then expands the complete fixed-table
StateBlock to bytes through a private helper. Root, Data, DataFields, EnvCells,
ValueSlots, FrameSlots and ControlRows follow the L4 order, including inactive
rows and inline pending-frame columns. Id/count/slice widths derive from their
own capacities; ValueRef uses max(K,D), EnvRoot uses E, and stackLimit uses SL
independently of physical frame capacity S. UInt and both fuel fields retain
sixteen little-endian bytes. Allocation or host-width failures use the existing
located refusal class. The raw-StateBlock helper is not exposed. This source
provides StateBlock byte expansion; byte parsing and a complete image wire
interface remain absent. No digest, statement binding or native ABI is added.

Located preparation failures are Capacity and HostUnrepresentable, the two L4
classes relevant to this phase. Capacity records table, needed, limit and source
location. Pending child locations name the parent expression and child position;
other locations name expression/function and the affected field. Checked host
arithmetic and fallible vector/string reservation precede materialization.
A refused preparation returns no ProgramBlock or semantic Outcome. The typed
state interfaces use all seven L4 refusal classes: Capacity, ProgramMismatch,
OutsideProgramClosure, WrongRepresentation, OutsideObservationDomain,
WrongProfile and HostUnrepresentable. These are tooling failures, never semantic
Outcome substitutions.

`state_encode::encode_observed_a(&PreparedA, &StateCapacities, &Snapshot)` builds
a complete padded logical StateBlock. All nine Frame, ten Control, four Outcome
forms and eight Snapshot fields have explicit source mappings. Canonical roots
visit the control spine first, each Push pending frame then its environment,
terminal control fields, current environment, top-first installed frames and
finally Done value. Values use noninterned postorder and parent-time field
blocks; each environment occurrence independently appends its sorted bindings.
Value vectors append their values before their contiguous reference slots.
Absent options, successful empty bindings, function row zero, every u8 error,
exact fuel and stack limit remain distinct.

The encoder compares the complete Program structurally and finds the least
equal prepared expression, function or permitted Call/Match suffix. Its explicit
Expr/Tree comparison stacks use prepared occurrence/node counts as equality
bounds. No pointer identity, evaluator or decoder call supplies acceptance.
State capacities have no defaults; D must agree with preparation. Pending
growth is checked incrementally, including canonical shared-tree unfolding.
O1/O2/O3 observation checks remain separate from capacity and semantic outcomes.

`state_decode::decode_observed_a(&PreparedA, &StateCapacities, &AImage, DecodeMode)`
checks the profile, full structural Program, complete supplied ProgramBlock,
fixed table lengths, active rows, zero padding, active references, constructor
fields, environments, source suffixes, control spine and observation domain.
It reconstructs a typed Snapshot. Canonical mode then calls the actual encoder
and compares the entire resulting logical StateBlock. Step mode permits the
specified noncanonical representation without that final equality. It does not
run a machine step or establish transition validity. No callback or success
stub replaces either implementation. This remains uncompiled source, not an
observed roundtrip or a verified decoder.

The normal test source uses literal expected table orders from the independent
normal packet and complete typed specimens. Coverage includes all eight Expr,
five Pattern, seven BinaryOp, two UnaryOp and four Value forms, duplicate source
rows, identical Arc expressions with distinct occurrence IDs, repeated Tree
children without interning, fixed declarations, explicit field-slot timing,
root/nonzero function ownership, deep paths, scalar bytes and zero padding.
Six preparation tests were authored before implementation; the independent depth-three
example and shared-tree test were added during source inspection. No RED/GREEN
or actual round-trip/equivalence result is claimed. Test dimensions are examples,
not defaults. Expected values do not enter preparation. Eight encoder tests
(seven authored first) and five decoder tests (four authored first) add literal
row/whole-Snapshot specimens, all frame/control/outcome forms, multi-Push order,
current/saved environments, shared trees, options, suffixes and error payloads.
The decoder tests supply hand-authored images; encoder tests call the real
Canonical decoder. Four byte-expansion tests compare full byte vectors and
exact lengths against independently hand-derived literal expectations. Three
were authored before the serializer; a fourth independently supplied example
was added afterward. They cover pending Eval, full scalar bytes, a two-byte
stack limit with full fuel, and a pending saved frame distinct from current
environment and installed frame. This is partial positive coverage. All
8 + 8 + 5 + 4 tests are UNRUN. No owner rejection-test module is present or
registered in this candidate.

Future work still includes StateBlock byte parsing, reviewed boundary
projection, complete record/trace materialization and comparison, owner refusal
test sources and executable normal evidence. The current-A constructor mappings
do not discharge the full 55-variant/general-ADT roadmap. Retained PreparedA is
trusted internal preparation output; accepting an arbitrary mutated preparation
object is not an interface promise. Standard Arc/Box/BTreeMap allocation and
host Clone/Drop/Debug/derived equality outside the explicit traversal helpers
retain allocation and recursive-depth limitations.
F4 representative enforcement, physical P1-P6 masks/append plans, StepInv,
G.4 components, B helpers/counters/ADTs and full financial phase/effect/liability/
await state remain separate. All U0-U7/M1-M17/MC/SP/G, native/recursive/private
history/proof/ledger/target obligations remain. No codec completeness,
determinism, canonical uniqueness or product acceptance is claimed.
