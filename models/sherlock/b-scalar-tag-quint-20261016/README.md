# Scalar.tag positive Quint projection

Authored source only, GPT-6.1 Sol high. Parse/type/effect checks, all eight normal
model tests, simulation and verification are **UNRUN**. No Quint, K, Lean/Clean,
circuit, SDK, compiler, native or test execution was selected for this task.
The source predicate is: on the declared positive RetRight/Add/First/UInt
cutpoint, the selected operand's full UInt128 integer payload is copied to
helper.regA, helper.phase becomes Second, the four work/visit counters change by
-1/+1/+1/+1, and every other named helper/global field is preserved.

The normative sources are schema `237a3f31d5d93f613aab6a73fbf105373c3842272b6aed7876fb86a0ceb48de5`
and R4 rows `a0dd03b196a247232b2595204645268e368625ce320ccf7d5cc6dc237b7de75e`,
verified against the circuit task's input pins. The row is `/helper_events/1`,
`b.helper.Scalar.tag`, and its branch is
`/effects/1/then/0/then`. The existing public declared interface contract supplies
named preservation and already-declared caller obligations. That proposed
contract does not itself adopt a row, layout, width, field, ABI or circuit.
Root designates the row as adopted R4; historical overlay proposal labels and
qualified HostOnly Stage1 source status do not imply wider adoption.

No circuit Rust, emission sketch, engine/runtime/family source, actual capture,
MAP/column mapping, independent QA output, frozen O directory or expected tuple
was read. The author derived semantics from the normative row/schema. The public
interface declarations were supplied as the authorized caller contract only.
Exact inspected input and authored-file hashes are in SOURCE-MEMBERSHIP.json.

## One explicit positive row

The model has one sequential machine, one atomic tag action and an explicit stop
at Second. There is no network, competing actor, failure scheduling, wall clock,
hidden evaluation or later Add arithmetic. `applyTag` performs just the two row
effects and four deltas. `rowRelation` separately names all preserved helper
fields and compares the complete declared Global record. `step` stutters after
the one-row comparison boundary. This stutter is a model stop, not a normative B
transition or an extra event.

The active prefix of both value spaces is retained as integer UInt rows. The
space discriminator and index select exactly one constant or data row; index 1
is exercised with a different index-0 payload, and opposite-space decoys differ.
UInt tags are implicit in the projection's UIntRow type. Logical kind is UInt
for every row in this finite positive domain; no raw Const/Data tag ordinal,
field packing or hidden UInt conversion is used.

Payloads are Quint mathematical integers constrained to `[0, 2^128)`, including
`18446744073709551623`, `79228162514264337593543950355`, and
`340282366920938463463374607431768211455`. They are not strings, floating-point
numbers, u64 values or limb residues. Ordinary counter samples also exceed 64
bits. This finite sample set is not a restriction on Moriarty's UInt128 language
semantics. Execution support for these concrete integer literals is UNRUN.

## Preconditions and finite projection domain

The already-declared positive premises are: the bound step is an Applied
Scalar.tag row; helper is present Scalar, site RetRight, mode Add, phase First;
operandA selects an active UInt row; remainingWork is positive; helperWork is
below its supplied maxHelperWork; and the four local counter operations fit
UInt128 with no wrap. Whole StepInv, AB admission binding, P0 Running and
constrained exactly-one dispatch are caller obligations, not claims discharged
by this projection. P2/P3/P6 refusal and enable-zero behavior are excluded.

The normal source instances satisfy the visible S1 and site-coherence projection:
regA is initially zero, staged/expected/parentResume are absent, operandB is the
Return value, operandA equals the top RightF.left, originReferenceCursor equals
referenceCursor, result is absent and tasks are empty. Work bookkeeping uses
consumedWork = helperWork + referenceCursor. The model's `positivePre` makes
these finite-domain premises explicit; it is not an implementation of admission,
whole StepInv, error behavior, runtime guards or a new refusal policy.

Six input-only cases choose both value spaces, indices 0/1, payloads and ordinary
counter tuples. Each has two active UInt rows per space, one RightF, one retained
environment cell, two value slots and no tasks. Clocks start at nonzero existing
values; the helper interval origin/reference cursor is 3. Fuel is the sum of
remaining and consumed work. Read-domain capacities are supplied by each input
instance (`limitInput`); values 2/1/0 and the headroom +8 are exploration choices,
not new Bounds, runtime caps, resource authority or serialized widths. The
Limits type is an explicit subset, not all 22 Bounds/four LayoutCapacities.

## Named preservation and row observation

Only helper.regA, helper.phase and remainingWork/consumedWork/helperWork/helperVisits
change. All helper common and scalar fields are named: present/kind/site,
originReferenceCursor/parentResume, mode, operandA, operandB, expected and staged.
They are copied unchanged in `applyTag` and compared individually in
`rowRelation`. No opaque rest-lane partition hides a helper change.

The preserved Global record names control/controlOrigin/envRoot/depth/stackLimit,
outcome/initialFuel/referenceCursor, alloc/active/taskLive/result/segBase,
data/dataFields/env/valueSlots/frames/tasks. Program constants and source metadata
are retained in State. The frame is a single actual RightF-shaped projection;
tasks are empty and their element representation is not a general HelperTask.
The retained source occurrence/control IDs are symbolic metadata, not a checked
ProgramView or a materialized source-identity proof.

The row observation records the exact eleven-position delta vector, rule,
HelperVisit/Applied, First→Second phase, the Scalar phase ordinal cursor 0→1,
origin, frame depth and task counts. **referenceCursor delta is 0**. The public
contract's phrase “cursor delta 0” is ambiguous against the normative schema's
event helper cursor, which maps Scalar phase ordinals and advances 0→1. OBL-06
and D-09R separately preserve referenceCursor. This wording issue has been routed
to Root for the designated Opus owner; no resolution is claimed here. The source
keeps the unambiguous row vector and schema helper-cursor projection separate. No new event,
codec or circuit wiring is adopted. The observation is a declared row projection,
not a complete StepEvent; RunId/stepId, source occurrence materialization,
result/alloc/outcome envelope fields and full event production remain gaps.

## Tests and cross-layer limits

The eight normal `_test.qnt` tests are authored directly from the normative row:
Const zero, Data index selection, payloads above 64/96 bits, full UInt128 copy,
wide ordinary counters, the local observation and sampled normal relation plus
explicit stop. All are UNRUN. NORMAL-WITNESS-INPUTS.json contains inputs only.
No expected output was generated by a host, circuit or model implementation.
These author-local assertions do not replace independently frozen O prefixes.

FIELD-ROW-MAP.json records source pointers and omitted fields by layer. This is
one positive mid-helper row, not full B, a scheduler, complete transition,
StepInv/genesis/preservation/refinement proof, constraint check or native result.
Other operators/sites/phases/branches, other tags, arithmetic, failure/refusal,
publication, admission, full frames/tasks, layouts/widths, ABI/field embedding,
statement binding, generic/mutual ADTs, finance and continuations remain outside
this slice. All actual K/Quint comparisons with independent QA remain future,
separately authorized work. K, Lean/Clean and the actual circuit are not inspected
or compared by this artifact.

Full Sherlock scope and all U/M/MC/SP/G exits remain open, as do ACTUS/DeFi,
Preview/Fenrir and mandatory Midnight stagenet qualification. Stage2 R1–R7,
Stage2/3/untrusted-input blocks, C4 and resource counters remain unchanged.
Native authority remains 0. No public developer approval dependency was added.
Any source, projection, precondition or bound change needs fresh applicable
source audit and separately eligible execution before reusing a comparison.
