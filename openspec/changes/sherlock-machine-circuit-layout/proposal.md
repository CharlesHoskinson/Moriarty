# Full Sherlock machine circuit layout proposal

Status: **PROPOSED / UNADOPTED / UNEXECUTED**. This is an ordinary representation
design for review, not a native ABI, schema freeze, implementation, theorem,
resource admission or permission to execute. It changes no existing specification.
The author is Astra, at the user's explicit request. Superseded 2026-10-24: adoption and acceptance follow AGENTS.md (factory predicates + holdouts); no vote or reviewer approval is required.

Revision L4 adds only the verbatim Opus G1/G2 canonical append/path
clarifications and the G3 future-obligation label, and corrects G4 map
provenance. No codec,
ProgramBlock digest, canonical-uniqueness or execution result is claimed.

Historical revision L3 materialized only Opus F1(a-c)/F2 text clarifications and recorded
F3/F5 constraints and F4's UNSELECTED mechanism from the fresh L2 review.
L2 received two substantive ADOPTABLE_WITH_LIMITS votes, but those votes do
not approve these changed bytes, implement a codec or establish determinism.
Complete preparation/decode claims need their own evidence;
this candidate remains unexecuted.

Historical revision L2 materialized the separately supplied Opus 5.5 high owner repairs,
including the exact 39-rule observation envelope and the separation of physical
capacity refusals from semantic observation conditions. The owner guidance is
not a vote or adoption. [Repair provenance](OWNER-REPAIR-PROVENANCE.md) records
its identity and scope; both reviewers must review these final changed bytes.

## Decision requested

Adopt the complete current **K-A structural representation** described in
[design.md](design.md), [interfaces.md](interfaces.md) and the exhaustive
[crosswalk.json](crosswalk.json), subject to the decisions in
[extensions-and-obligations.md](extensions-and-obligations.md). The recommended
starting point is the full public `sherlock-k-a/1` machine, with all eight Snapshot
fields and every current constructor, not a fixed application or literal subset.
The same bounded circuit shape takes different supplied program tables and
machine witnesses. Source recursion remains runtime function lookup and frames.

The recommendation is to make an explicit A layout decision first. Candidate B
adds a different metered helper schedule and state. Its separate extension map
is provided for review, but missing B event/failure policies prevent this package
from being a B freeze. No helper counters are inserted as zero into an A state.
Financial behavior and general nominal ADTs remain required extensions, with
their exact missing interfaces retained rather than represented as empty data.

The implementable A core uses immutable ordered program tables, immutable typed
value nodes/field slots, persistent environment binding chains, a top-first
bounded frame vector, and a bounded control spine for recursive `Push.next`.
It specifies every variant payload and sum/option discriminator, index and
length widths, row order, traversal roots, and complete prefix materialization.
Static capacities remain supplied parameters. This does not choose k, degree,
proof/key formats, public/private bindings or execution budgets.

## Evidence and actual consumers

Source facts: `reference_types.rs` exposes Program/Snapshot and the types listed
in the crosswalk. `reference.rs::Machine::step` implements the A host clock,
including free final/error publication before zero-fuel exhaustion. The existing
`compare::first_difference` compares all eight typed state fields, record rule
line and charge in order; it is explicitly not a validity checker. This is the
target comparison consumer. `reference_display.rs` is diagnostic text, not a
codec. The private test-only ProgramImageDraft/MachineImageDraft remain unchanged
and are not adopted or exposed by this proposal.

The frozen new `sherlock_machine::emit_fetch_block` is a scalar table component,
not this layout. Future adapters can supply byte/Boolean columns from the rows
here. Multi-byte index reconstruction, typed dispatch, equality/validation,
environment updates and stack/control movement remain real circuit work. The
current backend serializes an IrSource and invokes the pinned SDK's load
consumer; this proposal neither runs that consumer nor asserts its acceptance.

The existing Sherlock evidence covers eight normal programs and 216 prefixes
in the checked-out documentation, with separate host and Quint receipts. The
earlier handoff's “14” refers to GAP-01..GAP-14 in a separate Core staged CLI
trace-projection draft, not fourteen Sherlock traces. That Core P-TRACE-PROJ
draft is unadopted and is not a machine profile, state projection or acceptance
rule in this proposal. No Core gap is closed here.

## Real alternative

| Choice | Recommended: typed arenas and root references | Alternative: fully expanded bounded term slots |
|---|---|---|
| Values | Nodes plus ordered child slots; immutable sharing has structural meaning | Each holder contains a fixed-capacity prefix encoding of its entire value |
| Saved environments | Root into immutable binding cells; each frame retains its own complete root | Each frame/control carries a full bounded sorted binding array and expanded values |
| Calls | Top-first frames and completed/remaining slices | Top-first frames with full inline argument/remaining-code blocks |
| Row access | Existing generic fetch can be adapted; reference/range and append constraints needed | Direct slot access; larger copies and repeated structural data |
| Capacity domain | Separate node, field, binding, frame and sequence capacities | Multiplicative per-holder capacities; duplicated values can refuse sooner for the same numeric limits |
| Main tradeoff | More indirection and explicit lifetime/canonicalization obligations | Simpler local payload ownership, but broad copies and larger static shapes |

Neither is a measured row/cost winner. The recommended shape connects naturally
to arbitrary recursive values and saved contexts without expanding each value
at every holder. The alternative is credible for a small initial circuit and
must be compared under the same declared semantic domain, not equal but
incomparable numeric capacity labels. A bottom-first physical stack is another
local optimization: it needs an explicit reversal adapter to A's top-first
observations and is not silently selected here.

## Completion predicate

Reviewers can accept or reject a concrete A representation: every current public
constructor and field has a listed storage path; all tables and row columns have
ordered definitions and symbolic widths; option/sum distinctions and saved
contexts survive decoding; complete records can feed the existing typed
comparator. The separate owner questions name what blocks adoption or a B/native
claim. This package supplies no executed round trip, circuit comparison or proof.
