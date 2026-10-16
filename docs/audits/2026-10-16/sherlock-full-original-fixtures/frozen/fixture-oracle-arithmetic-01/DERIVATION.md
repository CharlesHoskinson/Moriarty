# Full original arithmetic fixtures: provisional independent expectations

Status: hand-derived source expectations, specified-only and unexecuted. No
independent audit, producer agreement, formal correspondence, Candidate B
adoption, circuit result, financial result, native result or product exit is
claimed. Sole write boundary is this runtime directory; root owns integration
and Git. The baseline commit is `c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429`,
reported by root. Source-byte pins govern this packet's exact inputs.

The falsifiable preparation predicate is: each assigned original frozen fixture
has its entire source-literal state sequence, zero-charge startup/publication,
all ordinary initial-fuel budgets zero through exact completion plus one, and
its unchanged original fuel-1000 run specified independently before any future
producer comparison. Each configured K state cell must reconstruct exactly.
Missing/wrong controls, maps, frame payloads, rule identities, work or prefix rows
falsify that predicate. Actual implementation agreement remains unobserved.

## Inputs and read boundary

`INPUT-PINS.json` was written before reading the bodies of pinned `main.k`, the
two fixtures and existing literal expectations. The rule catalog was additionally
pinned before its label records were inspected. Normative main.k SHA256 is
`b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`.
The original factorial fixture SHA256 is
`0622e01b0b7ba1ee588ff648a9299840fd97e64375fd5aad3d6393ed349b6592`;
the original sum fixture SHA256 is
`661bd0d819885de654e5605635e10231bd587c4113547db696f4aaabd67ec351`.

The development skill, checkout AGENTS/product contract, implementation plan
(independent Oracle O and Phase 0 requirements), reference-status document and
shared interface state/profile text were read. No guarded status CLI was run:
this assignment permits ordinary reads and Python literal inventories only.
Historical execution/status prose supplies no new execution evidence.

Existing normal input/prefix JSON and the beginning of the independent extra
normal derivation were inspected for serialization, state-overlay and source
label conventions. No existing factorial-two state sequence was inspected or
copied. No reference/runtime/circuit/witness/evaluator/compiler implementation,
producer output, actual K capture or private source was read. No subject, build,
model, SDK, native, key or version command was executed. No oracle builder from
the repository was read or executed. Existing eight cases/216 prefixes are
unchanged. The literal serializer here expands source-text abbreviations and
fixed row data; it neither parses/evaluates the fixtures nor dispatches on K
rules or evolves an arbitrary input state.

## Fixed program meanings and source work

F01factorial retains exactly the original ordered singleton function table:
function1, parameter0 UInt128, UInt128 return, test `n == 0`, base1 and
`n * factorial(n - 1)`. Entry is the call on literal3. Its hand arithmetic is
factorial(0)=1, 1*1=1, 2*1=2, 3*2=6.

F02sum retains exactly the original ordered singleton table: function2,
parameter0 UInt128, UInt128 return, the same test, base0 and
`n + sum(n - 1)`. Entry is the call on literal3. Its hand arithmetic is sum(0)=0,
1+0=1, 2+1=3, 3+3=6.

Both original files specify fuel1000 and stackLimit64. `newprograms.json` also
names exact completion fuel113 for minimal normal prefixes; this is metadata,
not a rewrite of either fixture. `original-fixture-runs.json` holds the separate
full fuel1000 traces. Source equality is checked only by whitespace removal
against the immutable fixture bytes.

Every machine edge consumes one A unit. Pure helpers do not introduce another
A event: validity230, argument/return UInt128 typing238, equality258,
subtraction255, checked arithmetic250, multiplication256 or addition254,
lookup282, append286, binding290/291/294 normalize within their respective
machine edges. B helper visits/work are unmeasured; no zero is substituted for
missing helper accounting. No admission, B representation or finance semantics
is added.

The complete charged count for either fixed program is
`8 + 3*28 + 12 + 3*3 = 113`:

| Charged boundary interval | Count | Fixed source sequence |
| --- | --- | --- |
| 0 to 8 | 8 | 131,135,102,181,200,204,208,135: argument3, lookup/bind, enter body3 |
| 8 to 36 | 28 | body3 false test, retain operand3, evaluate subtraction3-1, invoke/bind/enter body2 |
| 36 to 64 | 28 | body2 false test, retain operand2, evaluate subtraction2-1, invoke/bind/enter body1 |
| 64 to 92 | 28 | body1 false test, retain operand1, evaluate subtraction1-1, invoke/bind/enter body0 |
| 92 to 104 | 12 | true test at0, literal base, restore caller1 |
| 104 to 107 | 3 | apply pending operand1, normalize result, restore caller2 |
| 107 to 110 | 3 | apply pending operand2, normalize result, restore caller3 |
| 110 to 113 | 3 | apply pending operand3, normalize result6, restore empty outer caller |

Each 28-step interval has this exact ordered incoming-rule sequence:
`123,135,117,135,108,149,102,153,223,165,117,135,108,149,131,135,117,135,108,149,102,153,223,181,200,204,208,135`.
The 12-step base interval is
`123,135,117,135,108,149,102,153,223,161,102,185`.
Each 3-step unwind interval is `153,223,185`.
This fixed arithmetic/source count precedes serialization and any future subject
execution. It is not a count obtained by running a second interpreter.

## Complete state and lexical restoration

The JSON state overlay matches the existing normal prefix convention. Each
state explicitly stores control, current map, every top-first frame, depth,
charged_steps, remaining_fuel and incoming rule. Fixed fields supply initial
fuel113, stackLimit64, pending output and exact code_ref. Code_ref resolves the
single immutable `code_k` identified by case/input digest in `newprograms.json`.
Frame strings contain their full expression/value/map payloads. No frame or map
is reconstructed from an abbreviated depth or recursion counter.

At startup74 the control becomes entry evaluation, the immutable function table
is installed, and the original or prefix fuel/limit is installed. Its charge is
zero. The explicit pre-start configuration retains empty env/stack, depth0,
stackLimit0, no code, both configured fuel counters0, and pending output.

After entering body3 (state8), current map is `0 |-> uiU(3)` and the return frame
saves `.Map`. At state36 current map is `0 |-> uiU(2)`; top-first stack is the
new return frame saving map3, then the pending arithmetic operand3, then the
outer return frame saving `.Map`. State64 adds the return saving map2 and
pending operand2 above those frames. State92 adds return saving map1 and pending
operand1, yielding seven frames with current map0. Condition evaluation in that
body adds its if frame then binary-left frame, reaching depth9 at state96.
Depth9 also appears at states97,98,99. No push reaches the original limit64.

Every nonzero false branch restores its own full map from its if frame. Each
subtraction is evaluated under the caller's map before the recursive callee's
fresh parameter map is installed. The final-argument181 boundary pops the
argument frame and restores the caller map. Bound208 schedules the return frame
while that caller map remains current; only push135 installs the callee map.
Thus scheduled pushes and actual pushed frames remain distinct snapshots.

At state104, return185 restores map1 and exposes pending operand1. State107
restores map2 and exposes pending operand2. State110 restores map3 and exposes
pending operand3. State113 restores `.Map` and empties the stack at depth0.
The value6 is still in `uiRet(uiU(6))` until publication80 clears control to
`.K` and publishes `uiDone(uiU(6))`, charging zero.

## Every fuel prefix and unchanged original run

`prefixes.json` contains all 115 literal five-column rows per program, for
initial fuel0 through114. Rows0..112 identify the corresponding nonterminal
boundary state, remaining fuel0 and publication84 `uiExhausted()`. The full
preceding charged sequence overlays initial fuel from that row and remaining
fuel equal to that budget minus each state's charged_steps. Publication clears
only control and changes output, retaining the full interrupted map, frames,
depth, code, limits and counters. Prefix startup also substitutes that budget in
the uiRun control; pre-start configured cells remain zero.

Budget113 reaches state113 with remaining fuel0, and rule80 freely publishes
the result instead of exhausting. Budget114 reaches the identical semantic
boundary with remaining fuel1. Both have work113. Neither row creates an
attempted-work diagnostic absent from K A.

`original-fixture-runs.json` repeats both complete source-literal traces with
fixed initial fuel1000 and explicit remaining fuel1000-i at every state i.
Startup retains the original `uiRun(..., 1000, 64)` term. The terminal state has
work113, remaining887, code unchanged, env `.Map`, no frames, depth0, limit64
and output `uiDone(uiU(6))`. The original fuel1000 run is outside the ordinary
minimal-prefix array and has not been silently changed to fuel113.

External source labels are listed in `source_rule_labels` in both trace files,
with source-line identity on every incoming edge and explicit administrative
edges. These are external catalog labels, not labels declared on the K rules.
The serialized files are provisional fixtures for future producer comparison;
they are not a public ABI, adoption decision or execution/review claim.
