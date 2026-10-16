# A102 Literal instruction fragment: proposed source mapping

This is the first composable A102 source fragment, not emitStepA, full Step, a native ABI, or an accepted circuit. The exact scoped P1 adoption and envelope/carrier clarification supply its scope. No native execution is authorized. The original worktree now registers the reviewed Rust bytes as a new source integration candidate; compilation and all five wiring tests remain UNRUN.

The callable source path is `mori_backend::sherlock_machine::{literal_schema, emit_literal_step}`. The backend `src/lib.rs` publicly declares `sherlock_machine`; its parent module privately declares `literal_step` and publicly reexports both constructors and all nine public Literal types. The five existing wiring tests are registered in the sibling `literal_step_normal_tests` module under `cfg(test)`. The fragment continues to use the real sibling G3 enabled-fetch implementation and the parent's pinned SDK/core imports. Existing backend compile entry points and CLI paths do not invoke A102; this registration adds no A102 input binding, witness packer, public ABI, execution wrapper or acceptance flag.

The emitter and test files are byte-identical to reviewed manifest `662bc7748f2a98e2186372970951c08ebed0328b04c574f6f7e4f5eab42e7f85`. Their frozen source comments retain the earlier unregistered status as provenance; this paragraph records the new parent registration. The Gemini and Muse `APPROVED_SOURCE_WITH_LIMITS` reviews cover that prior unregistered candidate. This changed integration candidate needs fresh review and has no inferred compiler, SDK or native result. The separate `sherlock-backend-build-20261016` compile worktree and its frozen r2 manifest remain unchanged.

`LiteralCapacities` supplies all 15 ProgramBlock table capacities in L4 order, all six StateBlock table capacities in L4 order, and the independent semantic SL. Dimensions are host usize for finite construction; SL remains u128. No defaults or admitted numeric bounds are chosen. C/X/K must be nonzero for this fragment; L4's builtin T/Q/QF minima remain 4/4/5. A shape/refusal is construction-only, never Outcome.

`literal_schema` constructs exact named flattened logical columns below. `LiteralImage` contains root/header operands and all fixed-capacity rows, including inactive rows. `LiteralStepInputs` contains one shared full ProgramBlock, full pre StateBlock, independent supplied post StateBlock, caller enable, and exactly `LiteralEnvelope { rule_tag, charge }`. No host rule-line operand is present. Host RecordEnvelopeA/StepRecord line 102 remains unchanged; wire tag is 3, charge 1. One immutable ProgramBlock operand image serves both sides; no digest or copied program substitutes for it.

Fuel fields each have 16 individual little-endian u8 operands. Other bounded logical scalars use Native operands with their exact L4 width and range checks. This is an instruction-construction carrier view, not an alternate serialized StateBlock. The sole canonical byte/Boolean representation remains L4. Connecting every nonfuel carrier to ceil(w/8) canonical little-endian bytes, byte decomposition/reconstitution and source/statement bindings is an explicit unwaived OBL.G4@A102/StepInv caller obligation. No public/private binding, serializer, witness packer, host acceptance flag or callback result is introduced.

Every supplied logical lane receives exact ConstrainBits or ConstrainToBoolean; enum lanes additionally exclude unused tags. Header/root counts are bounded by their own capacities. Full active-reference/variant/padding/acyclicity/domain/canonicality checks outside the local rule remain StepInv/F4 obligations. Program typing must establish literal uiValid; no Constants or ConstantFields node is fetched by this fragment.

The caller supplies a constrained exactly-one A102 enable with the complete precheck/dispatch partition. The constructor constrains enable Boolean and implements its local guards: pre Pending with zero outcome payload/error, controlLength=1, CR0=Eval with all unused columns zero, selected Expressions row=Literal/Constant with all inactive columns/slices zero, exact occurrenceId, exprId<activeExpressions and valueRef.index<activeConstants. Real emit_enabled_fetch_block selects the complete Expressions row under the same enable. Full pre-control padding is checked locally for rows >=1.

Enabled post equations constrain controlLength=1; CR0=Return with the exact selected Constant valueRef; all other CR0 columns and every later control row zero. Every other Root column is post=pre except remainingFuel; all Data/DataFields/EnvCells/ValueSlots/FrameSlots rows and columns are post=pre. This includes complete saved environments, recursive frame payload columns, both outcome reference components and all initialFuel bytes. No arena append occurs. Equations use ConstrainEq(CondSelect(enable,left,right),right); disabled rows impose no A102 transition equation on the shared post, so another selected rule can constrain it.

For fuel byte i, zero_i=TestEq(pre_i,0), borrow_0=enable, borrow_(i+1)=CondSelect(borrow_i,zero_i,0). Each borrow is Boolean. Under enable, constrain pre_i+256*borrow_(i+1)=post_i+borrow_i, with explicit 8-bit byte bounds and 9-bit intermediate bounds; constrain final borrow=0. A separate bytewise zero fold constrains all-zero-pre=false when enabled. No packed 128-bit subtraction or narrowed fuel carrier appears. InitialFuel is preserved byte by byte.

All input Variables must start with %. The composer reserves distinct non-nested scopes. This fragment reserves `%{scope}.local.temp.` and G3 `%{scope}.expression_fetch.temp.`; inputs cannot use either prefix. No global namespace registry or bound-input producer is installed. The block returns instructions, the constrained selected-expression operands for composition/inspection, and exact supplied operand aliases; it never evaluates them.

Unwaived: whole StepInv/O1-O4 and representation typing, F4 canonical ProgramBlock/least-equal boundary projection, exact-one dispatch/free publication, statement/program/trace/public-private binding, concrete resource/physical capacity admission and SDK/native correctness. The remaining 38 events, unwritten P1 rows/P2-P6, B, general ADTs and financial/PCD/ledger obligations remain open. No obligation is represented as a passing flag.

Normal symbolic tests inspect actual emitted instruction wiring: complete schema membership, embedded real G3 instruction sequence, Constant reference alias, every unchanged post cell, Return padding and envelope, and all sixteen borrow edges. All tests are UNRUN. Specimen dimensions supply ordinary construction examples only, not a smaller semantic domain or admitted resource cap.

## Complete logical column map

Each composite below expands recursively in exactly the listed L4 order. u8[16] expands to .0 through .15; all other scalar entries are one logical Native operand with the stated width. ProgramBlock tables follow the header in the table order in crosswalk.json; StateBlock tables follow Root. No column is omitted.

- ProgramHeader: entryExprId:id(X), activeNames:count(N), activeNameBytes:count(NB), activeOccurrences:count(O), activeOccurrencePaths:count(OP), activeExpressions:count(X), activeExprChildren:count(XC), activeFunctions:count(F), activeParameters:count(P), activePatterns:count(R), activeArms:count(A), activeTypes:count(T), activeConstructors:count(Q), activeConstructorFieldTypes:count(QF), activeConstants:count(K), activeConstantFields:count(KF).
- Names: byteStart:start(NB), byteLength:count(NB).
- NameBytes: byte:u8.
- Occurrences: ownerTag:enum(Entry,Function), functionIndex:id(F), pathStart:start(OP), pathLength:count(OP).
- OccurrencePaths: childPosition:count(max(XC,A+1)).
- Expressions: tag:enum(Literal,Var,Let,Binary,Unary,If,Call,Match), occurrenceId:id(O), valueRef:ValueRef, nameId:id(N), binaryOp:enum(Add,Sub,Mul,Eq,Lt,Cons,Branch), unaryOp:enum(Leaf,Not), childSlice:Slice(XC), armSlice:Slice(A).
- ExprChildren: expression:id(X).
- Functions: sourceTableIndex:id(F), nameId:id(N), parameterSlice:Slice(P), returnType:id(T), bodyExprId:id(X).
- Parameters: nameId:id(N), typeId:id(T).
- Patterns: tag:enum(Var,Nil,Cons,Leaf,Branch), name0:id(N), name1:id(N).
- Arms: patternId:id(R), bodyExprId:id(X).
- Types: kind:enum(UInt,Bool,List,Tree), ctorStart:start(Q), ctorLength:count(Q).
- Constructors: owningType:id(T), sourceOrder:count(Q), fieldTypeSlice:Slice(QF).
- ConstructorFieldTypes: typeId:id(T).
- ValueRef: space:Bool, index:id(max(K,D)).
- ConstantNode: kind:enum(UInt,Bool,Constructor), typeId:id(T), ctorId:id(Q), uintBytes:u8[16], boolValue:Bool, fieldStart:start(KF), fieldLength:count(KF).
- DataNode: kind:enum(UInt,Bool,Constructor), typeId:id(T), ctorId:id(Q), uintBytes:u8[16], boolValue:Bool, fieldStart:start(DF), fieldLength:count(DF).
- ConstantFields: value:ValueRef.
- DataFields: value:ValueRef.
- EnvRoot: present:Bool, index:id(E).
- EnvCells: nameId:id(N), valueRef:ValueRef, parent:EnvRoot.
- ValueSlots: value:ValueRef.
- FrameRow: tag:enum(Let,Left,Right,Unary,If,Match,Args,Return,Scope), nameId:id(N), binaryOp:enum(Add,Sub,Mul,Eq,Lt,Cons,Branch), unaryOp:enum(Leaf,Not), typeId:id(T), expr0:id(X), expr1:id(X), savedEnv:EnvRoot, valueRef:ValueRef, completedSlice:Slice(V), remainingExprSlice:Slice(XC), armSlice:Slice(A).
- ControlRow: tag:enum(Eval,Return,Push,Invoke,Enter,Bound,Cases,CaseResult,PrimitiveResult,Error), exprId:id(X), valueRef:ValueRef, pendingFrame:FrameRow, environment:EnvRoot, nameId:id(N), valueSlice:Slice(V), functionPresent:Bool, functionId:id(F), bindingsPresent:Bool, bindings:EnvRoot, typeId:id(T), armSlice:Slice(A), primitiveIsOk:Bool, errorCode:u8.
- Root: activeData:count(D), activeDataFields:count(DF), activeEnvCells:count(E), activeValueSlots:count(V), depth:count(S), controlLength:count(C), environment:EnvRoot, stackLimit:count(SL), remainingFuel:u8[16], initialFuel:u8[16], outcomeTag:enum(Pending,Done,Rejected,Exhausted), outcomeValue:ValueRef, outcomeError:u8.

Snapshot mapping: program=one shared whole ProgramBlock; control=controlLength+all ControlRows; environment=Root.environment+all EnvCells; frames=depth+all FrameSlots; stack_limit=stackLimit; remaining_fuel=16 remainingFuel bytes; initial_fuel=16 initialFuel bytes; outcome=outcomeTag/valueRef/error. Envelope remains outside all eight fields.
