# Exact selected normative/current-owner excerpts

Full pinned readset snapshots are retained; excerpts do not replace full requirements or claim ABI adoption.


## openspec/changes/sherlock-universal-interpreter-design/interfaces.md:1..8; SHA256 bee739da090c41230adc6ebd711075d842ef127d7fed8d54276e9fb484d19536

1: # Sherlock Candidate B interface contract
2: 
3: Status: FINAL DESIGN CANDIDATE B / SPECIFIED-ONLY unless an evidence row explicitly says otherwise. These are proposed engineering interfaces, not implemented APIs or accepted correspondence results. This file owns shared names and field meanings used by [design.md](design.md), requirements and the implementation plan. Security requirements, theorem statements, tests and judgments remain with Opus in the unchanged [UISEC document](security-requirements.md) and separate [Candidate B delta](security-candidate-b.md); interface completeness does not assess them. The Opus-owned [Candidate B A1–A4/B002 dispositions](security-candidate-b.md) supply the governing profile, accounting, NoMatch, codec and provenance dispositions reflected below.
4: 
5: ## Schema maturity and remaining freeze
6: 
7: This is a proposed interface vocabulary and field-meaning contract, not a complete implementable schema. No B schema freeze is permitted before Phase 0 defines all row/control payload types, tag cases, bounded widths/slices, helper phases/resume slots, the full outcome/error-code table, `Bounds.stepCap`, the `admit` signature and per-profile `isResourceLimit` table, `StepInv` fields, per-rule charge/allocation rows including failures, and intra-cap precedence, with the Opus review required by its A1–A4 handoff. Names such as `CodeRow`, `DataRow`, `EnvCell`, `FrameRow`, `HelperTask`, typed cursors and `fixedPayload` below identify future typed schema fields; they are not existing Lean/Clean declarations. The known K/B mappings error4 (frame cap), error9 (no matching arm) and error10 (checked arithmetic) are explicit; frozen Quint instead retains its actual Rejected phase with StackBound reason. These profile-specific observations do not substitute for that full table.
8: 


## openspec/changes/sherlock-universal-interpreter-design/interfaces.md:90..126; SHA256 bee739da090c41230adc6ebd711075d842ef127d7fed8d54276e9fb484d19536

90: `ScheduleNext` is the proposed scheduling-control constructor; it is distinct from realization `Family.Schedule` and the example source ADT `Schedule`. Its fields above are semantic types; their fixed bounded frame/control/environment-root encodings remain Phase 0 schema work. Other abbreviated constructor arguments in these sketches likewise require typed payload definitions before freeze.
91: 
92: The first implemented model fragment has dedicated list/tree constructors, not this general declaration table. The proposed B reference retains their meanings. `frames` are ordered bottom-to-top; the final element is the next return context. Each frame saves the complete semantic environment it needs, including shadowed locals. Physical liveness/slot analysis may compact this payload only if decoding reconstructs those fields. Calls evaluate arguments left-to-right, push the pending return and install a fresh parameter environment; return restores the caller environment. Physical activation reuse retains every logical frame and its original cap check. Source values are immutable finite terms; DAG references unfold to structural occurrences, with no source address equality or cycles.
93: 
94: One helper transition inspects at most one function row, environment cell, parameter/argument pair, append cell, value occurrence, equality pair or pattern occurrence, then advances its stored cursor. A step may manipulate only a fixed maximum of bounded-width fields under the selected layout; fields/children exceeding that fixed payload are visited in subsequent cursor steps. Binding phases expose duplicate-binder scanning, one binding insertion and type checking; no recursive duplicate search is hidden in `Bind`. Validation and equality preserve ordered occurrence tasks; repeated references create repeated tasks. Match tries arms in order, with fresh tentative bindings discarded on failure. Type/declaration traversal, variable lookup, argument accumulation and structural equality may not call unmetered recursive helpers in B. Fixed-width UInt128 arithmetic is a bounded primitive, independently of variable-size structural traversal.
95: 
96: The initial fixed-step contract is relative to a frozen `Bounds` and field/layout configuration. It does not promise constant cost across capacities or claim that an indexed read is free in a circuit. No whole-state decoding or whole-value revalidation is allowed as an implicit implementation of a B step. Encoding/decoding is a separate bounded traversal interface; its host work is reported independently. Actual bounded helper rules, payloads and rule inventory remain implementation tasks.
97: 
98: `controlOrigin` follows the source expression that scheduled the current operation; helper/resume/frame records retain it through returns, errors and terminal publication. It begins at the entry expression. Physical virtual rows use their own indices alongside that origin.
99: 
100: ### Named schedules and outcomes
101: 
102: | Profile | Charged work | Publication and capacity behavior |
103: |---|---|---|
104: | `sherlock-k-a/1` | Exactly the existing K machine-section schedule; pure helpers unmetered | Startup/final-value/error publication zero; nonterminal zero fuel exhausts; stack-limit push rejects error4 |
105: | `sherlock-quint-a/1` | Existing algorithm-specific ticks, including terminal-return tick | Observation/stutter uncharged; frame overflow retains phase=Rejected, reason=StackBound, and its own cap comparison/charges; scalar/frame/fuel and attemptedWork remain separate |
106: | `sherlock-helper-microstep/1` | Each completed legacy machine transition costs 1; each explicit helper transition costs 1 | Startup and terminal/error publication zero; nonterminal zero fuel exhausts; legacy frame-cap error4 retained; added dynamic image capacities exhaust by the named kind |
107: 
108: 
109: The profile-aware engineering classifier follows exactly the Opus A1 table and is total over observations of each of the three named profiles:
110: 
111: ```text
112: Observation = profile-native outcome/phase/reason observation
113: isResourceLimit(profile:ProfileId,observation:Observation) -> Bool
114: ```
115: 
116: | Profile | `isResourceLimit(profile,observation)` |
117: |---|---|
118: | `sherlock-k-a/1` | `Exhausted(_)` or `Rejected(error4)` |
119: | `sherlock-quint-a/1` | `phase=Exhausted` or `(phase=Rejected and reason=StackBound)` |
120: | `sherlock-helper-microstep/1` | `Exhausted(_)` or `Rejected(error4)` |
121: 
122: Only these arms are resource limits; ScalarBound, ConstructorMismatch, MissingReference and every other error are not, per Opus A1. The classifier neither rewrites raw observations nor interprets resource limits as semantic/typing/financial rejection. Quint's `push` uses its frozen `depth > MAX_FRAMES` refusal boundary and retains its actual overflow charge; no shared numeric error4 or matching K/B cap/charge is inferred. Cap-boundary and overflow-charge correspondence require explicit differential predicates. The full observation/error-code and transition tables remain Phase 0 work.
123: 
124: B decomposes a legacy rule into bounded helper work and a separately identified completion event carrying that legacy rule ID. Entering/finishing a helper is an explicit helper transition if not combined with one bounded helper visit or the legacy completion. Empty/missing scans and helper failure exits likewise have explicit finite events. The complete rule inventory must enumerate these events before implementation claims or exact-fuel oracles. Zero-cost recursive helper closure is prohibited in B. Failure precedence is: publish already-final value/error first; otherwise zero fuel yields `Exhausted(Fuel)`; otherwise select the charged rule, perform its existing logical frame-cap check, then the declared data/env/helper-cap check before its update. A frame-cap error4 is a charged failing push followed by free error publication, as in K-A; this is not a claim about Quint overflow charging. Serialization capacity refusal occurs at a requested encoding/checkpoint operation, not spontaneously inside pure evaluation.
125: 
126: `eraseHelpers` maps an in-progress B helper state to its saved legacy pre-state and maps a completed legacy event to its legacy post-state. For a fully mapped interval with identical admitted inputs, no intervening capacity failure and completed helper work, the proposed accounting equation is `W_B = W_A + H_B`; it is a correspondence target, not a proved result. Equal numeric fuel does not imply equal stopping points. A-preserving physical helper rewrites retain `W_A` and count helper visits only in diagnostics; they are not B metering. Added admission/image capacities narrow a domain explicitly. `AdmissionRejected`, `CapacityNotAdmitted` and backend `UnsupportedFeature` are tooling outcomes outside `Outcome`; none may masquerade as an application result or a legacy exhaustion.


## openspec/changes/sherlock-universal-interpreter-design/interfaces.md:292..308; SHA256 bee739da090c41230adc6ebd711075d842ef127d7fed8d54276e9fb484d19536

292: ## Declaration-driven ADT extension and financial boundary
293: 
294: ```text
295: TypeDecl = {nominalId:TypeId,parameters,recursiveGroup,constructors:[CtorDecl]}
296: CtorDecl = {ctorId:CtorId,sourceOrder,fields:[{fieldId,type}]}
297: CtorLayout = {nominalId:TypeId,ctorId:CtorId,tag,fields:[{fieldId,type,slotKind}],width}
298: MatchPlan = {sourceMatchId:SourceOccurrenceId,scrutineeType:TypeId,
299:              nodes:[PlanNode],entry,armTable,workMap}
300: PlanNode = Switch(occurrencePath,cases,default) | Bind(binderId,occurrencePath,next)
301:          | Select(sourceArmId,bodyExprId) | NoMatch
302: ```
303: 
304: `NoMatch` is a match-plan/helper result that maps to application `Rejected(error9)`, following the Opus-owned reconciliation; it adds no `Outcome` variant. The exact transition and full error-code table remain Phase 0 work.
305: 
306: First extend to closed monomorphic nominal recursive groups, including mutual references. Assign stable type/constructor/field/binder/occurrence IDs; derive a plain tagged-node layout; compile source-ordered pattern matrices using first-column specialization/default decomposition. Field slot `i` always means source field `i`. Evaluate constructor fields strictly left-to-right before allocation and the scrutinee once before matching. Select the minimum matching source arm index, bind in its fresh arm scope and execute only its body. Pattern occurrences use `(sourceMatchId,sourceArmId,fieldPath)`, never arena addresses. `workMap` carries an ordered reference-event cursor; shared tests/suffixes cannot erase charges or stop positions. Heuristic columns, packing and test sharing are later comparisons. Parameterized types need separately bounded instantiation; closures, GADTs, mutation/cycles and or-patterns remain additional extensions. Existing dedicated K list/tree sorts do not discharge this work.
307: 
308: The financial extension must add complete committed/tentative state, ordered effects, residual duties, pending operations/observations and phase/cost fields to `ReferenceState`, images and observations together. Existing Core/2 successful-prefix typed await remains controlling. Exact universal `Commit`/`Yield`/`Await` opcodes and target-phase mapping stay OPEN for integration/review; no opcode is supplied here. Chunk cuts, caches, match plans and residual blocks cannot create financial commits or move source boundaries. Existing UISEC clauses remain unchanged, with all new security implications referred to their Opus owner.


## openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md:246..265; SHA256 e593ecbe48e902ee430fa7c9d14a681100e27f71df297c5161c5881e3fb6311b

246: ### Requirement: UIREQ-034 Declaration-driven ordered matching
247: WHEN the generic ADT extension elaborates an admitted closed nominal declaration group, it SHALL produce TypeDecl/CtorLayout/MatchPlan mappings preserving the first matching source arm.
248: 
249: #### Scenario: New recursive Schedule declaration
250: - **WHEN** Schedule = Done | Pay(UInt128,Schedule) | Both(Schedule,Schedule) is matched by ordered arms Both(Pay(x,_),_)→x; Both(_,r)→sum(r); Pay(x,t)→x+sum(t); Done→0 on Both(Pay(4,Done),Pay(5,Done)).
251: - **THEN** The selected arm is source arm one and the result is 4; a separate structural sum is 9. Nominal/constructor identity, source field order, binder occurrence paths and workMap are retained independently of compact tags or arena indices. Closed monomorphic mutually recursive groups are the initial extension; parameterized specialization needs its own finite bound, and closures/GADTs/or-patterns remain outside this adapter. Existing dedicated list/tree forms do not count as this extension's implementation.
252: 
253: ### Requirement: UIREQ-035 Deterministic witness dataflow
254: WHEN a WitnessPlan is evaluated, its baseline SHALL execute a typed dependency DAG in a deterministic uncached order to materialize complete StepRecords.
255: 
256: #### Scenario: Independent normal comparison baseline
257: - **WHEN** The same admitted factorial/list/tree or pending-caller state is materialized without caching, with cold caches and with warm caches.
258: - **THEN** Stable dependency ordering and explicit bindings produce identical decoded reference prefixes, ordered frames/environments, outcomes and reference-work observations against independently authored expectations. Cache/lifetime/streaming variants retain the uncached baseline; reusable keys include exact program/profile/layout and immutable inputs, while helper visits, copies, live values and memory remain separate diagnostics. Neither those expectations nor semantic behavior are generated from the materializer/DAG metadata.


## crates/mori-core/src/sherlock/helper_machine/shared.rs:18..62; SHA256 8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5

18: //!   shared::owner) + the small implemented constructors/normalizers/leaves
19: //!   in this file. No engine, admission, StepInv, or apply body exists yet.
20: 
21: use std::sync::Arc;
22: use super::raw::{ConstRow as RawConstRow, DataRow as RawDataRow, RuleId as RawRuleId};
23: 
24: // ---------- plain shared enums: Copy, no raw-104 types inside ----------
25: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
26: pub enum GrammarDomain { DedicatedSorts, GenericNominalExtensionReserved }
27: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
28: pub enum FinanceInput { Absent, TypedExtensionReserved }
29: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
30: pub enum SortTag { UInt, Bool, List, Tree }
31: // Runtime signatures share the canonical identity of the existing ADT tables.
32: pub use adt::TypeIdC;
33: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
34: pub enum BinaryOp { Add, Sub, Mul, Eq, Lt, Cons, Branch }
35: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
36: pub enum UnaryOp { Leaf, Not }
37: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
38: pub enum PatternTag { PVar, PNil, PCons, PLeaf, PBranch,
39:     WildcardExtensionReserved, ConstructorExtensionReserved } // L1 restored
40: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
41: pub enum BoundaryKind { None, Terminal, TechnicalCut,
42:     SourceSuspensionExtensionReserved } // L1 restored; still refused in B-core
43: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
44: pub enum LogicalKind { UInt, Bool, Nil, Cons, Leaf, Branch, Marker,
45:     ConstructorExtensionReserved }
46: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
47: pub enum ScalarMode { Add, Sub, Mul, Lt, Cons, Branch, Leaf, Not, TestBool, TestType }
48: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
49: pub enum ScalarPhase { First, Second, Done }
50: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
51: pub enum NodeKind { Cons, Leaf, Branch }
52: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
53: pub enum BoolSelector { True, False, NotBool }
54: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
55: pub enum TriStatus { Pending, Positive, Negative }
56: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
57: pub enum BindPhase { Pairing, UnwindType, UnwindDup, UnwindInsert, Propagate, Finished }
58: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
59: pub enum BindStatus { Ok, Bad }
60: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
61: pub enum AppendPhase { Copy, Tail, Done }
62: #[derive(Clone, Copy, PartialEq, Eq, Debug)]


## crates/mori-core/src/sherlock/helper_machine/shared.rs:148..228; SHA256 8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5

148: pub struct BoundsC { pub code_rows: usize, pub function_rows: usize,
149:     pub code_child_slots: usize, pub parameter_slots: usize,
150:     pub pattern_rows: usize, pub match_plan_rows: usize,
151:     pub match_arm_rows: usize, pub type_rows: usize,
152:     pub constructor_rows: usize, pub constructor_field_slots: usize,
153:     pub max_arity: usize, pub constant_nodes: usize,
154:     pub constant_field_slots: usize, pub data_nodes: usize,
155:     pub data_field_slots: usize, pub environment_cells: usize,
156:     pub frames: usize, pub helper_tasks: usize,
157:     pub serialization_bytes: usize, pub max_reference_work: u128,
158:     pub max_helper_work: u128, pub step_cap: u128 } // all 22, schema #/bounds
159: #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
160: pub struct LayoutCapacitiesC { pub name_rows: usize, pub name_bytes: usize,
161:     pub occurrence_path_slots: usize, pub value_slots: usize }
162: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
163: pub enum RefC { Const(usize), Data(usize) } // shared owned ref; owner maps to raw
164: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
165: pub struct SliceC { pub start: usize, pub len: usize }
166: impl SliceC {
167:     pub fn normalized(start: usize, len: usize) -> Self {
168:         if len == 0 { Self { start: 0, len: 0 } } else { Self { start, len } }
169:     }
170:     pub fn empty() -> Self { Self { start: 0, len: 0 } }
171: }
172: pub type ValSliceC = SliceC; pub type FieldSliceC = SliceC;
173: pub type EnvRootC = Option<usize>;
174: #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
175: pub struct AllocCounters { pub data_nodes: usize, pub data_fields: usize,
176:     pub environment_cells: usize }
177: #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
178: pub struct ActiveLengths { pub data: usize, pub data_fields: usize,
179:     pub env: usize, pub value_slots: usize }
180: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
181: pub struct HelperCommon { pub site: SiteTag, pub origin_reference_cursor: u128,
182:     pub parent_resume: Option<usize> }
183:     // ResumeId HOST carrier = usize in Option; always None in B-core,
184:     // Some(_) = SI-08. Bounded width DEFERRED to X-05, not frozen here.
185: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
186: pub enum ScalarStaged { None, U128Value { n: u128 }, BoolValue { b: bool },
187:     PendingNode { tag: NodeKind }, PrimError { code: u8 }, // 10|11, owner-checked
188:     Selector { sel: BoolSelector }, TypeTest { ok: bool } }
189: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
190: pub enum HelperControlC {
191:     Scalar { common: HelperCommon, mode: ScalarMode, operand_a: RefC,
192:         operand_b: Option<RefC>, expected: Option<TypeIdC>,
193:         phase: ScalarPhase, reg_a: u128, staged: ScalarStaged },
194:     EnvGet { common: HelperCommon, binder: usize, cursor: Option<usize>,
195:         found: Option<RefC>, status: TriStatus },
196:     EnvPut { common: HelperCommon, binder: usize, value: RefC, base: EnvRootC,
197:         new_root: Option<usize>, status: TriStatus },
198:     Find { common: HelperCommon, function_name: usize, row_cursor: usize,
199:         found: Option<usize>, status: TriStatus },
200:     Bind { common: HelperCommon, function: usize, params: SliceC, args: ValSliceC,
201:         p_cursor: usize, a_cursor: usize, partial: EnvRootC,
202:         dup_cursor: Option<usize>, phase: BindPhase, status: BindStatus },
203:     Append { common: HelperCommon, src: ValSliceC, value: RefC,
204:         src_cursor: usize, dest_start: usize, staged: Option<RefC>,
205:         phase: AppendPhase },
206:     Validate { common: HelperCommon, root: RefC, status: TriStatus },
207:     Equal { common: HelperCommon, left: RefC, right: RefC, status: TriStatus },
208:     Match { common: HelperCommon, scrutinee: RefC, arm: usize,
209:         saved_env: EnvRootC, partial: EnvRootC, pat_tag: PatternTag,
210:         x: usize, y: usize, field_idx: u8, staged: Option<RefC>,
211:         phase: MatchPhase, status: TriStatus },
212: }
213: // FrameRow: exactly 9 core variants. ConstructorFields is NOT a variant:
214: // a reserved ConstructorFields tag at any owner boundary is refused
215: // HostViolation{WrongVariant}/C-INV, its payload is never carried here.
216: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
217: pub enum FrameRowC {
218:     LetF { binder: usize, body: usize, saved_env: EnvRootC, origin: usize },
219:     LeftF { op: BinaryOp, right: usize, saved_env: EnvRootC, origin: usize },
220:     RightF { op: BinaryOp, left: RefC, origin: usize },
221:     UnaryF { op: UnaryOp, origin: usize },
222:     IfF { then_expr: usize, else_expr: usize, saved_env: EnvRootC, origin: usize },
223:     MatchF { arms: SliceC, saved_env: EnvRootC, origin: usize },
224:     ArgsF { function_name: usize, completed: ValSliceC, rest: SliceC,
225:         saved_env: EnvRootC, origin: usize },
226:     ReturnF { return_type: TypeIdC, saved_env: EnvRootC, origin: usize },
227:     ScopeF { saved_env: EnvRootC, origin: usize },
228: }


## crates/mori-core/src/sherlock/helper_machine/shared.rs:294..322; SHA256 8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5

294: // ProgramView: Vec lengths ARE active lengths (no stored duplicate).
295: pub enum ExpressionRowC { Literal { constant: usize }, Var { binder: usize },
296:     Let { binder: usize, value: usize, body: usize },
297:     Binary { op: BinaryOp, left: usize, right: usize },
298:     Unary { op: UnaryOp, operand: usize },
299:     If { cond: usize, then_expr: usize, else_expr: usize },
300:     Call { function_name: usize, args: SliceC },
301:     Match { scrutinee: usize, arms: SliceC } }
302: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
303: pub struct FunctionRowC { pub function_name: usize, pub params: SliceC, pub return_type: TypeIdC, pub body: usize }
304: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
305: pub struct ParameterRowC { pub binder: usize, pub sort: TypeIdC }
306: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
307: pub struct ArmRowC { pub pattern: usize, pub body: usize }
308: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
309: pub enum PatternRowC { PVar { x: usize }, PNil, PCons { x: usize, y: usize }, PLeaf { x: usize }, PBranch { x: usize, y: usize } }
310: #[derive(Clone, PartialEq, Eq, Debug)]
311: pub struct OccurrenceRowC { pub owner_is_function: bool, pub function: usize, pub path: SliceC }
312:     // owner_is_function=false is Entry with function=0; path slices occurrence_paths.
313: pub struct ProgramView {
314:     pub entry: usize,
315:     pub expressions: Vec<ExpressionRowC>, pub occurrences: Vec<OccurrenceRowC>, pub occurrence_paths: Vec<usize>,
316:     pub children: Vec<usize>, pub functions: Vec<FunctionRowC>,
317:     pub parameters: Vec<ParameterRowC>, pub arms: Vec<ArmRowC>,
318:     pub patterns: Vec<PatternRowC>,
319:     pub constants: Vec<RawConstRow>, pub constant_fields: Vec<usize>,
320:     pub names: Vec<String>,
321: }
322:     // Vec lengths are the sole active lengths. constants hold markerFree in RawConstRow; constant_fields are Const ids in field order.
