# Hand-derived raw A102 Step expectation

This packet supplies one ordinary source expectation, not an execution result. The typed snapshots and all logical image rows are literal data. No prospective literal fragment, fragment tests or fragment mapping was read. No candidate encoder, decoder, evaluator or emitter was invoked. The author previously wrote host codec source; this is output-independent data, not a blinded codec audit.

## Rule and complete typed cells

K `main.k` rule starting at line 102 changes `eval(lit(V))` to `ret(V)` for a valid value and positive fuel, and decrements fuel. Rust `reference.rs` implements the Literal arm by cloning the value into Return; its step fuel accounting decrements the positive fuel. The selected value is UInt7. The complete snapshots retain the identical full Program, current environment `ignored -> UInt42`, installed top-first frame `Right(Mul, UInt9)`, stack limit 5, initial fuel 257 and Pending outcome. Only control Eval(Literal(UInt7)) becomes Return(UInt7), and remaining fuel 256 becomes 255. A102 does not consume the installed Right frame or multiply anything. There is no Done result in this step.

The host observation envelope is source rule line 102 with charge 1. L4's separate wire enum tag is 3. These identities are not interchangeable, and the envelope lies outside StateBlock. No native envelope carrier or binding is selected here.

## Full immutable ProgramBlock

The Program is Literal(UInt7) with one unused source function `unused(ignored:UInt) -> UInt99`. Its unused function and parameter remain present. The complete syntax name closure is exactly `ignored`, `unused`, ordered by exact UTF-8 bytes. The concatenated NameBytes contain 13 active bytes. Function name id is 1 and parameter name id is 0.

Entry-first source traversal assigns entry occurrence/expression id 0 and the function-body occurrence/expression id 1. Both root paths are empty `(pathStart,pathLength)=(0,0)`; entry has ownerTag 0/functionIndex 0, while the function body has ownerTag 1/actual source functionIndex 0. No child, arm or pattern rows are active. Literal traversal produces Constant0 UInt7, Constant1 UInt99, without interning. Scalar node ctor/bool/field columns are zero. There are no active constant fields.

Fixed declarations retain all four Types and four Constructors, including unused List and Tree: UInt and Bool have empty constructor slices; List uses Nil/Cons rows 0/1 and Tree uses Leaf/Branch rows 2/3. Constructor field types in order are `[UInt,List,UInt,Tree,Tree]`. Fieldless Nil has `(0,0)`; the remaining field type slices are `(0,2)`, `(2,1)`, `(3,2)`. The header includes entryExprId plus all fifteen active counts. Every table includes explicitly listed padding. Zero-valued active rows (for example parameter name0/typeUInt) remain active according to header counts; padding must not be inferred from row contents.

## Full raw Step images

Program tables are shared and unchanged. Physical state dimensions are D=4, DF=2, E=3, V=3, S=3, C=3, SL=9. They are literal specimen dimensions only. The semantic stack limit is 5, independent of physical frame capacity S=3; the actual depth is 1. These numbers are not defaults, resource admission or a guarantee about any future step.

Both images have activeData=3, activeDataFields=0, activeEnvCells=2, activeValueSlots=2, depth=1 and controlLength=1. Data0 is UInt42, Data1 UInt9, Data2 UInt88; Data3 is padding. EnvCell0 binds source name0 to Data0 and has no parent. Root.environment is Some(0). EnvCell1 is an unreachable old extension that binds the same name to Data2 and has prior parent0. It does not change the current environment because current root0 does not include it. ValueSlot0 holds Constant1 (UInt99), and ValueSlot1 holds Data2 (UInt88); neither is currently referenced by a control/frame slice. All these active old rows are retained unchanged.

L4 design explicitly permits mid-segment data/field/environment/value-slot arenas to retain unreachable old rows. This is a Step image, not a canonical boundary. The retained references are within their selected active tables and the environment parent is earlier. No inactive frame/control row contains retained data: all released/padding rows are zero. There are no active field slots because all specimen values are scalar.

FrameSlot0 has Right tag2, Mul operator2 and ValueRef(Data,1); all other columns are zero. Both spare frame rows are zero. Pre ControlRow0 has Eval tag0 and exprId0, with all other columns zero. Post ControlRow0 has Return tag1 and copies Expressions[0].valueRef exactly: `(space=Constant=0,index=0)`. Its other columns are zero. No Data append, constant unfolding, arena compaction or frame/environment update occurs. Both spare control rows remain zero.

This Constant-space reference decodes to UInt7 by the literal Constant0 row. Canonical boundary encoding instead requires reachable value references to use Data space and can rebuild reachable roots. No canonical image is generated here. Data0 deliberately holds UInt42, so replacing the Constant0 reference with Data0 is not equivalent. Although the active post ValueRef has zero numeric lanes, its meaning is established by the Return tag and Constant-space interpretation; it is not absent padding.

## Fuel, preservation and observation domain

UInt128 little-endian pre remaining fuel is `[0,1,0,...,0]`, post is `[255,0,0,...,0]`, and preserved initial fuel is `[1,1,0,...,0]`. This includes a real byte borrow at the 256-to-255 boundary. The only changed raw numeric lanes are remainingFuel bytes 0/1 and ControlRow0.tag. The logical control changes from an expression reference to the active Constant0 value reference even though both indices are numerically zero.

O1 has 256 <= 257 and 255 <= 257. O2 has depth1 <= semantic limit5. O3 has Some(control) and Pending on each side. O4 uses before Pending, exact listed host line102/charge1 and the identical full Program. Valid UInt7 and positive fuel select the ordinary A102 case. This packet assumes an arbitrary representable Step state, not startup reachability or trace continuity. All complete typed cells, active table rows, inactive columns and padding are explicit in expected.json.

Static JSON checks compare authored literals, column lists from frozen L4, padding and preservation only. They are not a parser/decoder implementation, candidate comparison, semantic evaluator, circuit check or security judgment. No byte-stream/native ABI result is claimed. Full39 rules, P1-P6, F4, G4, B, general ADTs, financial, proof, native and stagenet obligations remain open in their existing scopes.
