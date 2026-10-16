# Bounded scalar row-fetch source candidate

Status: handwritten source only; not registered in `mori-backend`; all Rust,
SDK and circuit execution is UNRUN. This module is a reusable prerequisite for
machine work, not a machine-step relation or an adopted machine representation.

Iteration r2 repairs the identifier issue G-D1 identified by the independent
Opus and Muse r1 audits. Their r1 findings remain attached to the original frozen
bytes; this revision requires fresh applicable review. The relation and fixture
are unchanged, but no execution result establishes SDK acceptance of this repair.

## Existing interfaces and the missing decision

The public reference has `Program`, `Snapshot`, `Control`, `Frame` and recursive
`Value` types in `crates/mori-core/src/sherlock/reference_types.rs`. The proposed
`ProgramImageDraft` and `MachineImageDraft` are private host-only test modules.
They are not a public bounded native layout. Candidate B `interfaces.md` says
the complete row/control payload schema, widths, slices, helper slots, bounds
and outcomes are still required before schema freeze. It also proposes a frame
order different from the current A snapshot order. This source chooses neither.

The missing interface decision is an adopted mapping from those full reference
types to ordered circuit fields: all constructor tags and payloads, widths,
capacity/padding rules, function-table identity and lookup order, control and
environment encoding, frame order and payloads, and all outcomes. Once that
mapping is reviewed, program/state adapters can supply operands to
`emit_fetch_block`. No serializer, parallel machine IR, tag table, function
lookup rule or frame mover is added here.

## Reused types and actual compiler boundary

`Column.kind` uses the existing `mori_core::Type::{UInt,Bool}` scalar components;
`FetchWitness.rows` uses `mori_core::Value`. UInt uses 128 bits; Bool uses one
Boolean constraint. This reuse does not assign the Moriarty core language
profile to Sherlock. Recursive list/tree values require a later adopted map.

The emitter constructs `midnight_zkir_v3::{Instruction,IrSource}` from the
workspace-pinned Midnight revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`.
`Operand` and `IrType` come from that SDK's `ir` and `ir_types` modules.
The existing backend emits public inputs with a range constraint and `Impact`,
then serializes its `IrSource`, supplies version 3.0 and calls `IrSource::load`.
The new constructor stops before serialization/load. It performs none of those
SDK consumer operations. Its public wrapper reuses that input convention;
binding and security judgments remain with their owners.

`emit_fetch_block(shape, inputs, scope)` is the integration point for an
enclosing circuit builder: bind the supplied Native operands first, reserve
`%{scope}.temp.` for this block, append `instructions`, use the typed `selected`
operands, and retain `preserved` aliases. The fragment declares no public inputs,
outputs or transcript policy. The standalone `emit_public_fetch` wrapper is for
future normal comparison after executable admission, not a registered CLI path.

Generated public input names are `%row_fetch.input.{position}`; generated
temporaries are `%{scope}.temp.{serial}`. `scope` is inserted literally after
the leading `%` and is normally supplied without that prefix. Every caller
Variable must already start with `%`; an unprefixed Variable returns
`ConstructionError::UnprefixedVariable`. Input Variables in the final reserved
temporary namespace return `TemporaryNameCollision`. Immediate operands do not
have variable names. The actual backend serializes identifiers unchanged, writes
the version header, and loads JSON; the pinned SDK writes variable names verbatim
and requires `%` when deserializing operands. This is source inspection, not an
executed serialization/load check.

The composing caller must reserve distinct, nonoverlapping final namespace
prefixes across all fragments and enclosing instructions. Equality and nesting
are both forbidden by that composition contract: `%a.temp.` overlaps
`%a.temp.1.temp.`. This module checks its own supplied operands only; it has no
global registry and does not inspect another fragment's definitions. The fixed
standalone wrapper namespace must also be remapped/reserved by any future
composition adapter rather than duplicated in one enclosing circuit.

## Exact generic relation and field map

Let N be the caller's nonzero capacity and C its nonzero ordered column count.
These are construction parameters, not newly admitted machine resource bounds.
Column names must be nonempty and distinct; names do not encode tags or fields.
Host-size arithmetic overflow is a construction error. Allocation/resource
admission of a concrete shape remains external.

| Position | Public input | Type/constraint |
|---|---|---|
| 0 | active_rows | UInt128 |
| 1 | index | UInt128 |
| 2 + r*C + c | rows[r][c], for every r < N and c < C | caller's column c kind |

All PublicInput instructions use Native, guard None, then the scalar constraint
and `Impact(1,[operand])`. The reusable fragment constrains its own inputs too,
so it also works over independently bound operands; this repeats range checks
in the standalone wrapper. No new private/public machine-state choice is made.

The fragment emits `LessThan(N,active_rows,128) == 0` and
`Assert(LessThan(index,active_rows,128))`. Each comparison result is explicitly
constrained Boolean. Every row selector is `TestEq(index,r)`, constrained Boolean,
and their sum is constrained to one. For each column in order, a CondSelect
chain starts at zero and visits every row; selector one chooses the supplied
cell and zero keeps the previous accumulator. The final cell gets the original
column's scalar constraint. The zero accumulator is internal construction data,
not a default accepted row for an out-of-prefix index.

| Position | Output | Source |
|---|---|---|
| c < C | selected[c] | final CondSelect operand for column c |
| C | preserved.active_rows | exact input operand 0 |
| C + 1 | preserved.index | exact input operand 1 |
| C + 2 + r*C + c | preserved.rows[r][c] | exact input operand 2 + r*C + c |

There are C+2+N*C Native outputs. Every row, including inactive rows, is retained
without a host rewrite or canonical-padding assumption. No replacement row,
claimed after-state, expected result, success flag or transition predicate is a
witness input. `pack_witness` checks only dimensions and scalar kinds, then
encodes count/index and each supplied cell in order. It does not fetch a row,
test index membership or evaluate instructions.

This is ordered row access, not name lookup or language dispatch. In particular,
it does not resolve duplicate function names or assume table indices are names.

## State coverage and reserved work

No `Snapshot` field is partially projected into this block. `program`, `control`,
`environment`, `frames`, `stack_limit`, `remaining_fuel`, `initial_fuel` and
`outcome` all remain outside this generic component. No claim that they are
preserved by a machine transition is made. The preservation claim in the source
map is only identity of all supplied table/count/index operands.

Typed dispatch, recursive call/return, frame movement, environment lookup,
fuel/outcome changes and bounded program preparation wait for the adopted
interface. The current block provides the actual dynamic scalar table access
those operations will need, without choosing their semantics.

## Independent normal data and future comparison

`expected.json` is an exact copy of Root's independent frozen data, SHA256
`1c0319aa6c447599eadbfb2020a60d66effc01001282470429d610e3d5867ee7`.
It was frozen before execution; no claim is made that it predates all author
source work. The supplied columns are amount:UInt128 and enabled:Bool; rows are
(7,true), (9,false), (12,true). With active_rows=3, supplied indices 0, 1 and 2
have independent expected selected rows in the fixture. False remains cell data.

`normal_tests.rs` contains UNRUN normal constructor/witness-order tests and a
second generic shape. These check instruction wiring and exact preserved
operand aliases, not circuit acceptance. The expected file is read only by a
fixture-shape test; its expected values never feed witness packing or emission.

After separate owner admission, a real SDK consumer must load/check/evaluate
the emitted IR and compare every actual output. For the three independent cases,
positions 0 and 1 must equal the fixture's selected row, positions 2 and 3 must be
3 and the supplied index, and positions 4 through 9 must equal all original rows.
No such consumer or execution is supplied or claimed here. Native cost, SDK
acceptance, compiled constraint meaning and security/theorem judgments are open.

The normal construction tests now lock every TestEq row immediate, both Add
operands in the selector sum, its final equality to one, each CondSelect bit,
cell and accumulator edge, and the capacity comparison's equality to zero.
They also specify the final generated identifier strings. These are UNRUN
constructor assertions, not an instruction evaluator or observed output check.
`identifier_rejection_tests.rs` implements only the two rejection tests prescribed
by Opus G-D1(b,c): unprefixed caller variables and collisions with the final
temporary namespace. Their test requirements/security design remain Opus-owned;
this revision does not add broader negative-test policies.

## Remaining G.4 requirements before machine reuse

The r1 Opus review identifies separate required components. They are unchanged
gaps, not features of this repaired generic fetch:

1. An explicit enable/guard relation for a fetch in an inactive dispatch branch.
   This block's prefix assertion and sum-to-one remain unconditional.
2. Narrow byte/enum/w-bit lane checks, either new lane kinds or explicitly owned
   adapter constraints. Existing UInt means 128 bits and Bool means Boolean.
3. Multi-byte ID reconstitution with each byte and the reconstructed width checked.
4. Canonical inactive-row/payload zeroing. This block preserves all rows and
   imposes no zero-padding convention.
5. Composition-wide namespace allocation as specified above; no global collision
   enforcement has been added.
6. Reviewed public/private/constant and statement binding for actual machine
   state. The wrapper's public input convention remains a demonstration.

The actual SDK preimage/transcript adapter is also still missing: pack_witness
orders scalars only. SDK load/check/evaluation, 128-bit LessThan support and
transcript/output behavior require their own admitted executable checks.
