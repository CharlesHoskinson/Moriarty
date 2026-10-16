# Separate profile extensions, conflicts and owner decisions

Status: proposed crosswalk and retained requirements, not adoption or security
analysis. Existing requirements keep their original owners and wording.

## Candidate B is a distinct state, not A plus zero counters

The Candidate B interfaces and source-schema draft contain additional semantics
and fields. The A layout does not erase them, initialize them to invented zeros,
or claim to encode B. A proposed B extension would reuse the ordinary table
mechanisms but add the following explicit root sections and rows. Widths follow
the same capacity-derived rules; counter bounds below are named parameters, not
chosen numeric limits. Every field must remain in B materialized observations.

| B field | Proposed explicit storage / mapping |
|---|---|
| identity | separate exact source/profile/layout/bounds/field metadata plus owner-defined native statement binding; no guessed digest format |
| control / controlOrigin | tagged LegacyControl or HelperControl root; origin ExprId from actual execution, not structural reconstruction |
| environment | complete persistent EnvRoot |
| frames | B logical bottom-to-top sequence with complete payloads and origins; an explicit reversal adapter is required if sharing A physical top-first rows |
| values | explicit root ValueSlice for the interfaces.md value list; it cannot disappear because the older schema draft omitted it |
| outcome | Running/Halted(value)/Rejected(code)/Exhausted(kind), separate from A Outcome |
| limits | exact selected Bounds record, carried by immutable layout identity, available to decoding and each rule |
| remainingWork / consumedWork | separate bounded counters, each retained, with width derived from the declared work envelope |
| helperWork | separate charged-helper-event counter bounded by maxHelperWork |
| helperVisits | separate occurrence-inspection diagnostic, not helperWork and not an inferred counter |
| logical allocations | separate dataNodes, dataFields, environmentCells cumulative counters; never replaced by physical active lengths |
| referenceCursor | explicit logical reference-event cursor, with its own declared maximum including free events |
| physical occupancy | per-table active lengths and live scratch/task counts, distinct from logical allocation history |

Bounds preserves every existing field: codeRows, functionRows, codeChildSlots,
parameterSlots, patternRows, matchPlanRows, matchArmRows, typeRows,
constructorRows, constructorFieldSlots, maxArity, constantNodes,
constantFieldSlots, dataNodes, dataFieldSlots, environmentCells, frames,
helperTasks, serializationBytes, maxReferenceWork, maxHelperWork, stepCap.
Additional realized-layout capacities are needed for names/bytes/occurrence
paths, control/resume rows, task descriptors, value slots and event cursors.
Those are explicit physical envelope parameters, not silently new application
exhaustion reasons. Their relation to the existing Bounds is an owner decision.

B outcome tags could remain 0..3 in the same order Running/Halted/Rejected/
Exhausted, but their profile namespace and Exhausted reason differ from A.
Named reasons from interfaces.md are Fuel, DataNodes, DataFields,
EnvironmentCells, HelperTasks, HelperWork, SerializationBytes (proposed local
reason tags 0..6 in that order). These are storage recommendations only.
Per-rule failure charges, allocations and intra-cap precedence still need the
Opus-owned completed table. Frame overflow remains the recorded error4 arm;
NoMatch maps to recorded error9 and checked arithmetic to error10. Historical
Quint's phase/reason and charged terminal tick are not normalized into this table.

## Concrete helper payload proposal for review

Use a fixed tagged HelperRow with dedicated variant columns (inactive columns
zero), a Resume table, and ordered task-slot tables. Dynamic counts are bounded
by declared capacities. Task order is FIFO/stack as explicitly chosen per
operation manifest; no traversal order is inferred from an arena address.
The recommendation for Validate/Equal/Match is ordered depth-first, left-first
worklists: pop the first task and prepend ordered children. Physical shift/copy
cost remains separate from the source helper visit. This is a design choice
requiring review, not an already adopted helper schedule.

| Proposed tag / helper | Complete retained payload |
|---|---|
| 0 Find | functionName, current function-row cursor, Option<found row>, phase, resumeId |
| 1 EnvGet | sought name, original EnvRoot, Option<current cell>, Option<found ValueRef>, phase, resumeId |
| 2 EnvPut | name, value, original EnvRoot, Option<result root>, phase, resumeId |
| 3 Bind | parameterSlice, argumentValueSlice, parameterCursor, argumentCursor, partialEnv, duplicate-scan EnvRoot/current cell, current name/value/type, validation-task slice, phase, resumeId |
| 4 Append | typed source/destination sequence-space tags, source/destination slices, source/destination cursors, phase, resumeId |
| 5 Validate | ordered ValidationTask slice and cursor, current expected type, accumulated status, phase, resumeId |
| 6 Equal | ordered EqualityTask slice and cursor, accumulated equality Boolean, phase, resumeId |
| 7 Match | ordered arm slice, arm cursor, PatternTask slice and cursor, partialBindings EnvRoot, saved EnvRoot, Option<selected arm>, phase, resumeId |

ResumeRow columns: originRuleId, sourceOccurrenceId, originReferenceCursor,
savedLegacyControlSpine (complete bounded control rows), savedEnvironment,
resultSlot (typed slot-space tag and index), phase. A result slot has an explicit
empty/present discriminator and typed result payload, including successful empty
environment, NoMatch, and errors; its exact result enum follows the helper's
declared output, never a success-only Boolean. Nested helper calls retain their
parent resumeId through an explicit Option<ResumeId>; no unmetered host stack is
an implementation of a suspended helper.

ValidationTask = {ValueRef,expectedType,fieldCursor,occurrencePathSlice}.
EqualityTask = {leftValueRef,rightValueRef,fieldCursor,leftOccurrencePath,rightOccurrencePath}.
PatternTask = {patternOrPlanId,valueRef,fieldCursor,sourceArmIndex,occurrencePathSlice}.
Each fieldCursor from the existing source-schema task record is retained with
its declared sequence bound; a task-list cursor does not replace it.
Paths are bounded ordered field-index arrays with declared physical capacity.
Task descriptors/cursors and all copied values/slices survive cuts. Repeated
references create repeated occurrence tasks. Canonical projection must retain
these tasks and resume roots, not merely the current main value.

Proposed phase vocabularies are explicit enum families rather than unbounded
Nat: Find/EnvGet=Visit|Finish; EnvPut=Insert|Finish;
Bind=CheckCounts|SelectPair|DuplicateScan|Validate|Insert|Advance|Finish;
Append=Visit|Finish; Validate/Equal=Visit|Finish;
Match=SelectArm|VisitPattern|DiscardBindings|SelectBody|Finish. Each family also
needs a located failure completion in its event catalog. No charge/allocation
is assigned here to these phases: exact event rules and owner failure rows must
be accepted before claiming this B payload complete enough for execution.
The proposed enum is a reviewable interface choice, not a disguised B freeze.

Source discrepancies requiring explicit resolution:

- interfaces.md sketches PrimRight(op,left,savedEnvironment), while the source
  schema and actual A Frame::Right have no environment. Recommendation: A stays
  exact; any B PrimRight includes its explicitly required saved environment until
  the B contract is reconciled. Do not copy an arbitrary current map into A.
- interfaces.md includes B values, limits and layoutVersion; the source-schema
  ReferenceState draft omits some of those. Recommendation: retain the interface
  fields in B materialization and reconcile the catalog, rather than deleting
  them from the proposed native image.
- interfaces.md EnvPut includes environmentRoot, while the schema HelperControl
  variant relies on resume context. Recommendation: retain it explicitly as above.
- The source-schema PrimitiveResult draft has separate optional value and error.
  Current A is Result<Value,u8>. Recommendation: one explicit Result discriminator
  in A and typed result slots in B, with no both/neither ambiguity.
- Counter widths, stepCap admission and every failure/allocation event row remain
  the designated owner's unresolved schema-freeze inputs. They cannot be filled
  by observing successful host call-helper examples.

## General nominal data and matching extension

The recommended node fields already separate typeId, ctorId and ordered field
slices. Extending beyond the four A types requires a different reviewed profile
with explicit TypeDecl rows {nominalName,recursiveGroup,ctorSlice}, Ctor rows
{owningType,sourceOrder,fieldTypeSlice}, and bounded declaration-name/group
dictionaries. Source declarations and IDs remain distinct from compact tags.
Closed monomorphic mutually recursive groups are the existing B initial target.
Parameterized types, closures, cycles, GADTs and or-patterns are not smuggled
into the A representation.

Add Construct Expr rows {typeId,ctorId,ordered field-expression slice} and
ConstructorFields Frame rows {typeId,ctorId,completedValues,remainingExpressions,
savedEnvironment}. Pattern extension rows are Bind(name), Wildcard,
Constructor(typeId,ctorId,ordered child-pattern slice). These are proposed B
source/schema additions, not existing A variants. MatchPlan rows have tags
Switch(path,cases,default), Bind(name,path,next), Select(sourceArmId,body),
NoMatch; case and path tables are ordered and capacity bounded. Every plan node
retains source-match/arm/path and reference-work cursor mapping. First matching
source arm remains the result; path sharing does not erase declared occurrences
or helper work. Exact phase/event rules and lowering remain open tasks.

## Financial and proof requirements stay present

The pure A Snapshot has no financial cells. The financial extension must add
the full committed and tentative state, ordered effects, residual liabilities
and duties, pending operations, external observations, phase/cost fields and
continuation identity together with semantics, observations and representation.
It cannot use an empty optional blob, empty effect list or Pure result to claim
coverage. Exact Commit/Yield/Await and target-phase mappings are still open in
the controlling interfaces. A technical cut has none of those source meanings.

Retained clauses include UNI-001 permissionless tooling; UNI-002/003 one semantic
relation/native correspondence; UNI-004 intent/solver completion; UNI-005/006
complete finances and phases; UNI-007/008 conditional settlement/recovery;
UNI-009/010 full history/privacy; UNI-011..017 adapters, AI, services, libraries,
evolution, release and native contract; UNI-018..024 profile admission, scoped
comparisons, independent expectations, parent-evidence distinctions, target
separation, retained membership and developer journey. Existing MOR/AEO/MPLR,
ZR01..16, MNR01..08 and U/M/MC/SP/G obligations are unchanged.

UIREQ-001..011 map to complete program/value/control/frame tables;
012/013 to exact profile outcomes/work; 014..020 to complete checkpoints and the
still-required financial extension; 021..026 to declared cross-layer domain,
compiler/native/Clean correspondence and evidence; 027..035 to occurrences,
helpers, representation, exact arithmetic, separate plans, prefixes,
residualization, ADTs and deterministic witness dataflow. None is marked passed.

The product contract's contract properties, intent refinement, transition
validity and full history compliance remain mandatory acceptance obligations.
Complete effects/fees/gross debit/liabilities/residual duties and permitted
failed-phase outcomes cannot be removed to make a pure image fit. Preview,
Fenrir and required Midnight stagenet evidence remain distinct. No new proof,
key, hash, privacy, ledger or target assumption is supplied by these rows.

## Decisions for the fresh reviewers

| ID | Concrete decision/question | Owner |
|---|---|---|
| L01 | Adopt the full A arena layout versus expanded holder slots, including exact stated closure domain? | Muse + Opus substantive agreement |
| L02 | Accept top-first frames, recursive control spine, persistent map roots, byte scalar baseline and listed tags/column order? | Muse + Opus substantive agreement |
| L03 | Accept boundary traversal/no-interning recommendation and explicit operation-manifest requirement for runtime append/write order? | Muse + Opus substantive agreement |
| L04 | Exact public/private/constant labels, source/program/pre/post identity binding, field embedding and canonicality requirements? | Opus; UISEC-001..009/A3/A4/B002 remain controlling |
| L05 | Which static dimensions/segment envelopes may be realized, and how is capacity refusal separated from A/B application outcomes? | Opus resource/admission owner; no numeric choice here |
| L06 | Counter widths, B stepCap/cap precedence, helper-event and failed-event charge/allocation tables, task/slot lifecycle? | Opus; existing A1/A2/B001..004 dispositions retained |
| L07 | Resolve the concrete B schema/interface discrepancies and accept/revise proposed helper phase/payload rows? | Muse ordinary interface + Opus owned policies |
| L08 | Exact provenance producer and missing-origin reporting; no guessed occurrence from structural equality? | Muse interface + Opus binding scope |
| L09 | Any CTFE/specialization split: static immutable facts versus dynamic state, fuels, frames, helpers and future finance? | Opus for security/CTFE judgments; no specialization selected |
| L10 | Full theorem/hostile-test/native-recursion/privacy/history/ledger/target acceptance obligations and applicable gates? | Opus exclusively; UISEC-010..012/B005..009 retained |

### L3 owner clarification limits

The fresh Opus review `machine-layout-and-rule-manifest-security-opus-r2`
adds the following limits without selecting a mechanism or numeric capacity.
F1(a-c) and F2 are materialized in design.md, interfaces.md and crosswalk.json;
new bytes still need fresh applicable votes before complete codec claims.

- L04 / F4: canonical-ProgramBlock and least-equal enforcement remain
  UNSELECTED. The owner lists a bound prepare-time canonical-representative
  table or verifier-recomputed ProgramBlock; least-equal enforcement must not
  rely on per-step in-circuit search. This is still an owner decision.
- L05 / F3: no-interning boundary projection can unfold exponentially in fuel.
  Checkpoint, chunk and external-claim admission must use the unfolded capacity
  envelope, as well as the existing segment occupancy requirements. Fuel,
  live DAG size and mid-segment high-water occupancy alone do not supply it.
  Capacity refusal never becomes error4, Exhausted or another semantic outcome.
  Maximal-sharing canonicalization is only a future separately reviewed change.
- F5 / H1: host encoders, closure search and unfolding must check capacity
  incrementally and refuse at the first overflow, never after a full unfold.
  Every proposed adapter remains host tooling only, with no untrusted or
  network consumer. No host/resource/native admission follows.

All listed B discrepancies, financial fields and phases, general ADTs, binding,
resource, hostile-case, theorem, native-recursion, privacy, history, ledger and
target obligations above remain. These notes add no substitute judgment.

This author inventories questions and existing owner requirements only. It does
not answer them with safety/absence/soundness claims, author hostile tests or
classify executable admission. No missing reviewer can count as agreement.
