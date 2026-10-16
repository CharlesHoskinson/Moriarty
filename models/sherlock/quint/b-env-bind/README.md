# EnvGet/Bind finite source projection

This is a source-only B-T11 component for the existing Sherlock correctness loop.
It models the adopted R4 HOST EnvGet and Bind rows, pending results, their legacy
completions, free value/error publication and earliest fuel exhaustion. It is not
a public profile, an implementation of all B, a refinement claim or acceptance.
Typecheck, tests, simulation and verification are **UNRUN** by explicit assignment.
The skill's normal incremental execution is superseded by that source-only limit.

The model follows the qualified R4 HOST source dependency for the existing
`sherlock-helper-microstep/1` / `DedicatedSorts` context. This publication creates
no profile adoption, schema freeze or execution grant. The public source basis
contains the [schema](../../../../docs/audits/2026-10-16/sherlock-b-core-host-source/source-basis/r4/schema-supplement.json),
[rows](../../../../docs/audits/2026-10-16/sherlock-b-core-host-source/source-basis/r4/rows-supplement.json)
and [engineering handoff](../../../../docs/audits/2026-10-16/sherlock-b-core-host-source/source-basis/r4/ENGINEERING-HANDOFF.md).
JSON governs prose. The exact digest of each file and of the governing revised
catalog overlay is recorded in [SOURCE-RESULT-SCOPE.json](SOURCE-RESULT-SCOPE.json).
The overlay and qualified adoption records are not bundled in this public
selection; the public source closure is therefore incomplete for independently
reconstructing adoption and the entire revised event schedule. No private
runtime location is needed to read or inspect the standalone model source.
The raw [rules catalog](../../reference/rules.json) is historical context;
the revised R4 rows and overlay supplied the model schedule. No B engine or
B row implementation, and no O expected states/counts/results, were consulted.

The callable source predicate is: `advance(initial(input, limits))` and each
successive `advance` implement the selected R4 EnvGet/Bind event, preserving
all fields in the declared projection until a published outcome or the explicit
successful-Bind boundary. This predicate remains unobserved.

## Finite instance and input witnesses

One deterministic machine operates on immutable input and explicit lexical cells;
there are no actors, messages, concurrency, clocks or failure assumptions beyond
the source-defined machine outcomes. One `step` is one expanded R4 event. `cases`
contains ten independently authored component inputs, not expected outputs.
They exercise empty/one/deeper/shadow/missing lookup, empty and three-parameter
binding, and ordinary arity/sort/duplicate semantic error paths. These are normal
row witnesses, with no hostile input, security judgment or mutation harness.
`NORMAL-WITNESS-INPUTS.json` repeats the input packet in neutral data form.

All bounds here are MODEL-ONLY exploration choices: three names `x,y,z`, four
constant values `UInt(0),UInt(1),Bool(false),Bool(true)`, at most three parameters
and arguments in the listed input corpus, initial cells of length 0–2, component
fuel 0–32, helper allowance in `{0,1,4,16,32}`, environment capacity 0–5 and task
capacity 0–3. Initialization filters environment capacity below existing rows.
This instance supplies every constant; no uninitialized module parameters exist.
These choices impose no public name, numeric, resource, target or language cap.
The resulting reachable set is finite: cells append within capacity, tasks are
bounded, cursors range over finite input rows and counters increase only through
finite fuel. No execution has established its cardinality or explored that set.

These are **component entry images**, not genesis programs or complete AST
correspondence witnesses. An EnvGet image starts at Eval(Var); a Bind image starts
at Enter with an already found function and already evaluated arguments. Upstream
Find/Append/Validate/call evaluation is outside this model. Prefix reachability
from an admitted whole program is an open obligation. No AST input was borrowed
from implementation or oracle output.

For future comparison choose an actual bound HOST image at a corresponding entry
with helper/result/tasks absent and a fixed baseline `(C0,H0,V0,R0,F0)` for
consumed work, helper work, visits, reference cursor and remaining work. Project
the first four counters by subtracting their baselines; retain F0 as component
initial fuel and retain the full environment arena with its original indices.
The component helper allowance is `host.maxHelperWork-H0`; whole initial fuel is
`C0+F0`. Only images whose normalized input/allowances belong to this finite
instance compare. Helper origin cursors are normalized by R0. A whole admission,
identity, initial fuel, other counters and tables stay in the comparison context;
the model does not reset or mutate them. This normalization is an explicit
projection premise, not an asserted correspondence theorem.

## Fields and reconstruction premises

Compare the entire declared projection at every event: immutable input/limits,
`machine` and the complete `lastEvent`. `machine` includes every active lexical
cell (including unreachable cells), ordered live pair tasks, control, helper,
pending result, env root, outcome and all changing counters. No prefix is skipped.
`lastEvent` is a projected event, not a canonical full StepEvent codec.

| Normative field / rule | Quint field / operation | Projection premise / omitted obligation |
| --- | --- | --- |
| `img.env`, `reg.active.env`, `reg.alloc.environmentCells` | `cells`, list length, `allocEnvironmentCells` | Full original-index arena; allocation starts at existing cell count. Previous index is smaller. Unreachable cells remain present and counted. |
| `EnvCell.binder/value/previous`, `reg.envRoot` | `Cell`, `envRoot` | Names map injectively to the three local names; all values are constant refs 0–3. `At` is reused as an optional value ref only in `GetState.found`; it is not a common host index type. |
| `prog.constants`, `ValueRef` | `values`, integer ref | All refs are `Const(i)`; no Data values, UInt arithmetic, lists, trees, markers or structural equality. Distinct constant refs remain distinct refs. |
| `prog.functions`, params/args slices | `Input.params/args`, `EnterFunction` | One fixed function 0, body expression 0 and UInt return type; ordered slices start at 0. Retain full immutable host tables externally and establish this fixture mapping. No function search/order claim. |
| `reg.control`, `controlOrigin` | `Control` | Eval is expression 0 with the input binder. Enter uses function 0 and the input args. Bound retains fixed return type/body implicitly. Source origin is fixed 0; no occurrence-identity correspondence claimed. |
| `ScheduleNext` with `ReturnF` | `ScheduleBody(root)` | Reconstruct frame `{returnType: UInt, savedEnv: machine.envRoot, origin:0}`, next/body 0, new env root `root`. The caller root remains unchanged. This is a comparison cutpoint before Push, not a terminal outcome. |
| `reg.depth`, frames, stack limit | fixed depth 0, empty frames | Stack limit may be fixed to 1 with physical capacity >=1; no push happens. No caller restoration, frame refusal or recursive call execution model. |
| `reg.helper.EnvGet` | `GetState` | Site EvalVar, parentResume None; binder/cursor/found/status/origin explicit. |
| `reg.helper.Bind` | `BindState` plus immutable input | Site Enter, parentResume None; fixed function/params/args; cursors/partial/duplicate cursor/phase/status/origin explicit. |
| `reg.result` | `Result` | Absent, EnvFound, EnvMissing, BindOk and BindBad remain distinct, including BindOk(None). |
| `img.tasks`, `taskLive` | ordered `tasks`, list length | Full live stack, LIFO. Inactive slots are abstracted as zero; separate reconstruction/codec obligation. |
| initial/remaining/consumed/helper work, visits, cursor | `Limits.initialFuel`, machine counters | Relative component counters as defined above. Logical reference cursor advances only on applied legacy completion. |
| other arenas/allocation, value-slot writes, segBase | fixed external context | No writes; data/dataFields empty, arg slots equal input args, segBase zero. These fields must match externally; no erasure of changed fields is permitted. |
| full identity, host binding, Bound/Run input admission | external pinned context | Assumed valid, same bound admission, HostOnly, DedicatedSorts, finance Absent, no resume, stepCap>=1 and sufficient code/parameter/constant/value-slot capacities. No admission, AB/PS/StepInv proof or codec claim. |
| StepEvent rule/kind/disposition and deltas | `Event` | Raw attempted rule retained on refusal; full source event also has site/helperKind/stepId/resultTag/sourceExprId/exhaustedKind/outcome/boundary. Their mapping is given below; no canonical event export implemented. |

Event reconstruction: site is EvalVar for Get rows and lookup completions, Enter
for Bind rows/enter.bind, Bound for bound.valid/invalid, absent for Publish.
HelperKind is EnvGet/Bind where applicable; resultTag is the staged exit/failure
result or the pre-result consumed by a completion. Source expression is fixed 0.
Data/dataField allocation and valueSlotWrites are 0; depth before/after is 0.
Outcome/payload are the complete `post.machine.outcome`; exhausted kind follows
its constructor, zero otherwise. Boundary is Terminal only on non-Running outcome.
Step ordinal and run identity remain external, bound to each actual event and the
qualified HOST context. `originReferenceCursor` in this projected Event is
explicitly **pre-helper** origin; entry therefore has Absent, even though its
post-helper has origin 0. A future full event adapter must reconcile that choice
with the source event observer contract before comparing a full event codec.
The string `"absent"` denotes optional phase absence only, not a host phase tag.

## Rule correspondence

Array pointers below address the pinned R4 `rows-supplement.json`; stable rule IDs
are also present as literal strings in the model. The source template supplies
charge/preservation; row effects supply the cursor/task/env changes.

| R4 row ID(s), pointer | Model source operation | Work / visits / completion |
| --- | --- | --- |
| `b.helper.EnvGet.entry`, `/helper_events/6` | `startOrBound` | helper charge 1; cursor=envRoot; pending; no task |
| `.cell`, `/helper_events/7` | `applyGet` | helper charge 1, visit 1; first matching newest cell stages found, otherwise follow previous |
| `.empty`, `/helper_events/8` | `applyGet` | helper charge 1, visit 0; stage Negative |
| `.exit` / `.failure`, `/helper_events/9` / `10` | `applyGet` | helper charge 1, visit 0; stage distinct result, clear helper |
| `b.helper.Bind.entry`, `/helper_events/22` | `startOrBound` | helper charge 1; empty partial/tasks; Pairing |
| `.pair`, `/helper_events/23` | `applyBind` | helper charge 1, visit 1; pair push and both cursors +1 fused |
| `.empty` / `.arity`, `/helper_events/24` / `25` | `applyBind` | helper charge 1, visit 0; choose unwind or Bad propagation |
| `.type`, `/helper_events/26` | `applyBind` | helper charge 1, visit 1; test constant kind directly; reject pops one task |
| `.duplicate`, `/helper_events/27` | `applyBind` | helper charge 1, visit 1; scan fresh partial only; duplicate pops and stages Bad |
| `.insert`, `/helper_events/28` | `applyBind` | helper charge 1, visit 1; append one cell and release task fused |
| `.propagate`, `/helper_events/29` | `applyBind` | helper charge 1, visit 1; release one pending task |
| `.exit` / `.failure`, `/helper_events/30` / `31` | `applyBind` | helper charge 1, visit 0; stage BindOk/BindBad; clear helper; retain all allocated cells |
| `k.machine.eval.variable.present/absent` | `consumeResult` | legacy charge 1, cursor +1; Return or Error(3); clear result |
| `k.machine.enter.bind` | `consumeResult` | legacy charge 1, cursor +1; BoundOk/BoundBad; clear result |
| `k.machine.bound.valid/invalid` | `startOrBound` | legacy charge 1, cursor +1; ScheduleBody cutpoint or Error(8) |
| `k.admin.publish.value/error/exhausted` | `advance` | free publication; cursor unchanged; raw state retained |
| P3 helper allowance; P4; P6 task/env capacity | `advance` | P3 free; P4 charges; P6 retains only P4, no visits, cursor/task/env/result changes |

Selection follows source helper priority, then pending result, then entry/Bound.
For this domain the per-phase guards partition the reachable component states;
that assertion has not been typechecked or proved. Private fallback arms in pure
functions only totalize off-domain carrier expressions; they are not a host
HostViolation implementation. No caller-built arbitrary machine is admitted by
`initCase`, which accepts only the listed inputs and selected local capacities.

P1 examines Return/Error before P2 fuel. P2 preserves helper/result/tasks at the
earliest zero-work prefix. P3 precedes P4. P6 can apply only to Bind.pair or
Bind.insert here; its charged refusal leaves helpers/tasks/arenas untouched.
Helper visits count only applied rows, never charges or refused visits. Bind's
inline type and insertion steps do not activate Scalar or EnvPut; there is no
task.enqueue or separate task.release in this component.

## Stuttering, gaps and evidence

At a published outcome, `step` stutters on the entire Machine and clears the
diagnostic lastEvent to NoEvent, modelling P0's no-event response. A consumer
must stop at the first non-Running outcome. At `ScheduleBody` the same no-event
stutter is an **artificial component stop**, not a source terminal step: compare
through the preceding bound.valid event and stop. Do not treat a later actual
Push/fuel event as a matching stutter. No stutter hides an internal helper event.

`workAccounting`, `arenaAccounting`, `allInv` and `comparisonBoundary` are
proposed bookkeeping observations only. No property has been checked. No
expectation was derived from an evaluator, and no trace export, framework,
runner, comparator or public schema was added. Future work must independently
freeze complete expected prefixes and establish a full input/field mapping,
then obtain separately authorized exact-source typecheck/execution evidence.

K B, Lean B, Clean and handwritten circuit correspondence for this slice remain
unimplemented/unobserved in this delivery. Whole-program genesis/reachability,
additional helper kinds, UInt128/full DedicatedSorts data, function ordering,
mutual recursion, caller restoration, generic/mutual ADTs, financial/effect and
continuation semantics, chunk/resume, all circuit/native/proof obligations,
ACTUS/DeFi coverage, Preview, Fenrir and required Midnight stagenet end-to-end
evidence remain mandatory and open. This finite component replaces none of them.
Native authority remains 0; limits, counters and C4 are unchanged. Security
requirements/tests/judgments and execution authority stay with Root/Opus.

## Retained source-review limitations F-01–F-04

- **F-01:** `Event.originReferenceCursor` means the pre-helper origin in this
  projection. Entry therefore records Absent. The exact full-event convention
  remains unresolved; the field cannot be compared naively to host StepEvent.
- **F-02:** pure functions contain off-domain fallback arms, including unchanged
  states and an `"outside"` rule string. They are not a HostViolation
  implementation, and their declared use is confined to the listed initial
  inputs and transitions. No arbitrary-image totality is claimed.
- **F-03:** `allInv` combines only work and arena bookkeeping. It does not state
  or establish full StepInv, helper/result coherence, chain coherence or
  preservation. The predicates have not been checked.
- **F-04:** Quint parsing, typing, effects, tests, simulation and verification
  remain UNRUN. Source inspection supplies no executable success. Any later
  correction changes the candidate and requires fresh applicable review.

No numeric illustrations from source audits are published as expected outputs.
The input packet contains no expected states, work counts, golden traces or
independent O corpus. Complete expected prefixes remain separate future work.
