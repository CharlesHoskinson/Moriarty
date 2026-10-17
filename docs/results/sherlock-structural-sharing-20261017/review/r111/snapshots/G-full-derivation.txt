# Original UIREQ-009 independent fixed normal host oracle

The exact original scenario is frozen in `inputs/UIREQ-009-original.md`: a tree has two occurrences of Leaf(4), either separate or one shared node; its fold returns8 and charges both semantic visits under the same traversal schedule. This packet proposes exactly one external positive test, with those two representations. It executes neither the subject nor K. Expected values/work come from manual composition of the pinned normative K source, not from Machine output or a total-prefix generator.

## Input and fixed domain

Both fixtures use the public typed `sherlock-k-a/1` host API with the exact ordinary R82 settings: fuel512, stack32 and collection budget513. No profile/default/cap changes are introduced. There is one same recursive `Tree -> UInt` function in both fixtures. Numeric symbol10 denotes sumtree,11 its Tree parameter,12 the Leaf payload binder,13/14 the Branch children. These are canonical signed-decimal names, within the K common symbol domain. They are source identifiers, not invented runtime record identities.

The function matches its parameter with Leaf12 ->12, then Branch13,14 -> Add(call10(13),call10(14)). Entry calls10 with one literal Branch(Leaf4,Leaf4). Both Programs share the exact function-body Arc. In one input the root's child Arc pointers are equal; in the other they are distinct. Fixture assertions inspect that actual physical fact before execution. There are2 versus3 physical tree nodes, but3 semantic occurrences in each. Program/AST/value structural observations coincide. The snapshots retain their exact input Program Arc; no fabricated heap addresses are compared.

The function is a recursive sum over the existing dedicated host Tree type. This is not a GenericNominal type/declaration runtime claim and not arbitrary-input evidence. Only the exact finite positive pair is proposed. All values4/8 are valid UInt128 and the observed domain carries no financial effects.

## Normative schedule

`inputs/main.k` hashes to b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41. Public Snapshot/StepRecord and Machine consumer source were read as requested to build a test against the actual API; that source supplies no observed expected data. K is the semantic/work authority. Startup is already reflected by `Machine::new` and is not an event. Each selected machine rule charges1; terminal publication at line80 charges0. Pure helpers uiValid/uiFind/uiBind/uiMatching/type/arithmetic checks do not add A fuel events.

The top call with one literal argument takes8 charged events: EvalCall131, Push135, Literal102, RetArgs181, Invoke200, Enter204, Bound208, Push135. The root Match takes9:126,135,108,173,214,217,214,220,135. Its Leaf arm first fails, then its Branch arm succeeds. These helper decisions charge only their selected machine rows.

Binary117 and Push135 establish a pending Left frame (2 events). The left recursive Leaf call takes18 events: argument/call entry8; matching126,135,108,173,214,220,135 (7); payload variable108, Scope195 and Return185 (3). ReturnLeft149 then rewrites the existing Left frame to Right(Add,UInt4) and evaluates the right Call directly; it creates no Push or Scope itself. The right Leaf call takes the same18. ReturnRight153 and Primitive223 compute8 (2), then the outer selected-case Scope195 and function Return185 restore the empty outer environment (2).

Thus8+9+2+18+1+18+2+2=60 charged transitions. One free publication yields61 events, remaining fuel512-60=452. The complete61-rule vector is literal in Rust and `EXPECTED.json`. There are13 real Push events and13 frame pops; the fixed per-prefix depth vector has peak5 and final0. No phantom Push is inserted after ReturnLeft or return/type completion. These numbers were checked only with fixed integer/JSON/source-vector arithmetic.

## Both semantic occurrences are discriminated

Invoke ordinals are4(root),23(left Leaf) and42(right Leaf), binding ordinals5/24/43, and successful Return185 ordinals36/55/59. Selected Leaf CaseResult snapshots are31 and50. The Leaf body evaluates its payload variable independently at34 and53; both return UInt4 with separate charged events. The left invocation retains Left(Add,right-call,branch-env), while the right retains Right(Add,UInt4). This discriminates left-to-right occurrence multiplicity even though both call arguments are structurally equal Leaf4.

Each callee binding contains only parameter11=Leaf4, with caller branch bindings13/14 stored in a Return frame. Selected complete staged Bound/Push/entered states verify installation order. Leaf matching adds12=UInt4 only inside that selected Scope. Scope return restores the callee's parameter map, and Return185 restores the caller's branch map. The final outer Scope restores11=root before the last function return restores the empty map. These are full typed control/environment/top-first-frame expectations, not value-only checks.

A pointer-keyed shortcut that drops the second required call/body visits or their charges would fail the literal sequence, two leaf-body ordinals, return/binding checkpoints and work/fuel counters even if it still returned8. This does not prove the implementation contains no invisible cache that preserves every declared observation. Heap allocation/time/validation-helper traversal cost remains unmetered by this A profile.

## Completeness and limits

The test independently compares all8 Snapshot fields at post-startup initial state, terminal state, the prepublication state and32 selected call/binding/match/return checkpoints. The Program field binds to the exact immutable literal input Arc and its complete structural code; global retention and pointer identity are checked for every observed boundary. Every event has independent rule/charge/fuel/outcome/stack-limit/initial-fuel/depth expectations. `EXPECTED.json` mirrors these with lossless typed-symbolic data and named input expression/function references; it is not a new serde or checkpoint ABI.

Unlisted prefix controls, environment maps and full frame payloads do not have an independently complete expected value here. Adjacent actual-state continuity and pairwise complete actual structural trace equality supplement the independent oracle; neither is used to manufacture expected states or to claim total-prefix correctness. `FIELD-AND-SCOPE-ACCOUNTING.json` makes that boundary explicit. The proposed test has not been Rust-compiled or run in this lane, so compile/type integration and actual observations remain unestablished.

## Execution handoff

Current develop/AGENTS/product/status were read. The old loan-swap queue was not followed. No Machine/K import or execution, Cargo, SDK/native/provider/model action or W/Git change occurred. Only the local fixed literal writer/JSON/hash/arithmetic checks ran. The proposal and all independent expected bytes are frozen before subject execution. Root must obtain existing designated-owner classification for this exact new test and transitive host closure before any run. This packet supplies no security judgment or execution grant, and cannot close native/target/full-interpreter or full retained requirement acceptance from source alone.
