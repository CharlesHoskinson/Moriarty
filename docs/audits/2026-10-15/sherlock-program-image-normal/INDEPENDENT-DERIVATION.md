# Independent normal ProgramImageDraft expectations

Status: hand-derived source expectations, specified-only and unexecuted. These
artifacts were written without reading `program_image.rs`, its test implementation,
the builder, the evaluator, or their outputs. No builder/evaluator/model/compiler
execution was used. Read-only development-plugin status and file/hash tooling are
administrative observations only. The integrator must freeze these exact bytes
before its first subject execution; later results require a separate record.

The source set consists of the declaration-only `ORACLE-TYPES-AND-CONTRACT.rs.txt`,
`INPUT-AST-TYPES.rs.txt`, and the public `models/sherlock/oracles/extra-normal-programs.json`.
No K implementation read was necessary. Input names are their signed-decimal
strings. The profile is literally `sherlock-k-a/1` throughout. JSON uses numeric
newtype IDs, unit-variant strings and single-key data-variant objects. This is
oracle notation, with no serialization, admission, native ABI or capacity claim.

The callable predicate is exact whole-image equality for each of the four
independently typed inputs returned by `normal_cases()`. It includes every
expression payload and child slice, ordered child entry, function and parameter,
match arm, constant and field, source locator, entry ID and profile ID. The Rust
module supplies no actual invocation or assertion; the integrator adds those
outside the oracle file. All expected data are explicit literals. Small helpers
only wrap literal constructor arguments; they do not traverse or calculate an
expected image.

Contract rule: entry precedes all function bodies in original table order.
Within a root, each syntactic occurrence receives the next expression ID in
child preorder. Each parent reserves its complete child slice before descending.
The current ordered-child table length is the start of an empty slice. Constants
follow occurrence preorder with the same complete-field-slice reservation rule.
Literal field paths are relative to their owning expression. Named calls retain
their name and only their argument children; there is no recursion unrolling.

## N07: weighted three arguments

The independent input is entry `17(1,2,3)` and function 17 with UInt parameters
named 0, 1, 2 and UInt body `(0 + (10 * 1)) + (100 * 2)`, where numeric bare
names in that body denote variables. The source function body is recorded once.
Expression 0 reserves children slots 0..3 for arguments 1, 2 and 3. All three
argument literals therefore have empty child slices starting at 3.

| Expr ID | Owner/path | Payload | Child slice start,len |
|---|---|---|---|
| 0 | Entry / [] | Call 17 | 0,3 |
| 1 | Entry / [0] | Literal C0 = UInt 1 | 3,0 |
| 2 | Entry / [1] | Literal C1 = UInt 2 | 3,0 |
| 3 | Entry / [2] | Literal C2 = UInt 3 | 3,0 |
| 4 | Function(0) / [] | Add | 3,2 |
| 5 | Function(0) / [0] | Add | 5,2 |
| 6 | Function(0) / [0,0] | Var 0 | 7,0 |
| 7 | Function(0) / [0,1] | Mul | 7,2 |
| 8 | Function(0) / [0,1,0] | Literal C3 = UInt 10 | 9,0 |
| 9 | Function(0) / [0,1,1] | Var 1 | 9,0 |
| 10 | Function(0) / [1] | Mul | 9,2 |
| 11 | Function(0) / [1,0] | Literal C4 = UInt 100 | 11,0 |
| 12 | Function(0) / [1,1] | Var 2 | 11,0 |

Root 4 reserves slots 3 and 4 for IDs 5 and 10. Node 5 reserves slots 5 and 6
for IDs 6 and 7; node 7 reserves slots 7 and 8 for IDs 8 and 9. Node 10 later
reserves slots 9 and 10 for IDs 11 and 12. Thus `ordered_children` is literally
`[1,2,3,5,10,6,7,8,9,11,12]`, not a sequence grouped by visiting each edge.
Function row 0 is name 17, parameter slice (0,3), return UInt, body 4. Its three
parameters retain 0,1,2 order and UInt types. Constants are 1,2,3,10,100 at
expressions 1,2,3,8,11; every field path is [] and field slice is (0,0).

## N08: named factorial two

The independent input is entry `18(2)` and function 18, UInt parameter name 0,
UInt return, body `if (0 == UInt(0)) then UInt(1) else (0 * 18(0 - UInt(1)))`.
All bare 0 names represent the parameter variable. Only the literal program
body is expanded. Repeated variables and repeated UInt(1) literals are distinct
source occurrences, yielding thirteen expressions rather than ten interned nodes.

| Expr ID | Owner/path | Payload | Child slice start,len |
|---|---|---|---|
| 0 | Entry / [] | Call 18 | 0,1 |
| 1 | Entry / [0] | Literal C0 = UInt 2 | 1,0 |
| 2 | Function(0) / [] | If | 1,3 |
| 3 | Function(0) / [0] | Eq | 4,2 |
| 4 | Function(0) / [0,0] | Var 0 | 6,0 |
| 5 | Function(0) / [0,1] | Literal C1 = UInt 0 | 6,0 |
| 6 | Function(0) / [1] | Literal C2 = UInt 1 | 6,0 |
| 7 | Function(0) / [2] | Mul | 6,2 |
| 8 | Function(0) / [2,0] | Var 0 | 8,0 |
| 9 | Function(0) / [2,1] | Call 18 | 8,1 |
| 10 | Function(0) / [2,1,0] | Sub | 9,2 |
| 11 | Function(0) / [2,1,0,0] | Var 0 | 11,0 |
| 12 | Function(0) / [2,1,0,1] | Literal C3 = UInt 1 | 11,0 |

Entry 0 reserves slot 0. If 2 reserves slots 1..4 for IDs 3,6,7, before Eq 3
reserves slots 4 and 5 for IDs 4,5. Mul 7 reserves slots 6 and 7 for IDs 8,9;
Call 9 reserves slot 8 for ID 10; Sub 10 reserves slots 9 and 10 for IDs 11,12.
Thus `ordered_children` is `[1,3,6,7,4,5,8,9,10,11,12]`. Function row 0 retains
name 18, parameter slice (0,1), UInt return and body 2. Constants are UInt
2,0,1,1 at expressions 1,5,6,12. All have field path [] and field slice (0,0).

## Shared Arc tree literal

There are no functions. The entry is one literal `Tree::Branch(L,L)`, where L
is the same physical `Arc<Tree>` containing `Leaf(4)` in both positions. It is
one expression, ID 0, with payload Literal C0, children (0,0), Entry/[] locator.
The constant branch C0 reserves its two field slots first. Those slots point to
distinct C1 and C2 leaf occurrences; both empty leaf field slices start at 2.

| Constant ID | Payload | Field slice | Source expression/path |
|---|---|---|---|
| 0 | TreeBranch | 0,2 | 0 / [] |
| 1 | TreeLeaf 4 | 2,0 | 0 / [0] |
| 2 | TreeLeaf 4 | 2,0 | 0 / [1] |

`constant_fields` is [1,2]. Function, parameter, arm and ordered-child tables
are empty. Entry is 0. Physical sharing does not merge the two field occurrences.

## Unused functions in table order

Entry is UInt(7). The input function table is name 8 with body UInt(9), followed
by name 3 with body UInt(4). Both have zero parameters and return UInt. All three
roots are literal expressions: IDs 0,1,2 point respectively to constants C0,C1,C2
with UInt payloads 7,9,4. Their source locators are Entry/[], Function(0)/[],
Function(1)/[]. All expression child slices and constant field slices are (0,0).
Function rows retain source indices 0,1, names 8,3, bodies 1,2 and parameter
slices (0,0). Children, parameters, arms and fields are empty. Entry is 0.
No invocation is needed for the two bodies to occur in the image.

These four exact-image predicates are falsifiable static expectations. No
successful compilation, runtime pass, theorem, circuit, native execution or
target acceptance is claimed by this artifact.
