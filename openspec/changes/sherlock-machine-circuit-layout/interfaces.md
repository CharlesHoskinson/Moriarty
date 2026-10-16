# Proposed implementation interfaces and complete comparison

These signatures are design notation, not installed APIs. They use the tables
and widths in design.md; no opaque `fixedPayload` or recursive circuit value is
left to an implementer. Concrete dimensions remain layout parameters.

L2's refusal, dispatch and event-envelope repairs came from Opus owner
guidance; L3 adds the bounded fresh-review clarifications below. See [OWNER-REPAIR-PROVENANCE.md](OWNER-REPAIR-PROVENANCE.md). The changed
candidate remains unadopted and requires fresh substantive reviews.

L4 adds the verbatim Opus G1 field-slot timing and G2 source-path conventions
in design.md and crosswalk.json. ConstantFields/DataFields blocks are appended
with their parent node after its children; fieldless nodes have (0,0) field
slices. Non-Match child positions are zero-based, root paths are (0,0), and
Entry functionIndex is 0. StepInv names only a future row-check obligation,
not an existing component or proved result (G3). These changes require fresh
applicable review before any complete preparation/encoding claim.

```text
prepareA(program: public Program, capacities: ACapacities)
  -> Result<PreparedA {programBlock, fullProgram, occurrenceMap, usage}, EncodeFailure>
encodeObservedA(prepared: PreparedA, observation: public Snapshot)
  -> Result<CanonicalA {stateBlock, usage}, EncodeFailure>
decodeObservedA(prepared: PreparedA, image: AImage, mode: Canonical | Step)
  -> Result<public Snapshot, DecodeFailure>
projectA(prepared: PreparedA, image: AImage)
  -> Result<CanonicalA, EncodeFailure>

StepOperandsA = {program: ProgramBlock, pre: StateBlock, suppliedPost: StateBlock}
emitStepA(layout: ALayout, operands: StepOperandsA)
  -> Result<InstructionFragment, LocatedUnsupported>
packStepWitnessA(layout, suppliedProgram, suppliedPre, suppliedPost, suppliedAux)
  -> Result<OrderedBindings, ShapeOrTypeFailure>

ObservedRecordA = {envelope: {ruleStartLine: enum(39) RuleLineA, charge: u8}, beforeImage: StateBlock, afterImage: StateBlock}  -- envelope outside StateBlock; one shared ProgramBlock
materializeRecordA(prepared, record: ObservedRecordA)
  -> Result<public StepRecord, DecodeFailure>
materializeTraceA(prepared, initialImage, records)
  -> Result<{identity: TraceIdentity, initial: Snapshot, records: [StepRecord]}, DecodeFailure>
compareA(leftCompleteTrace, rightCompleteTrace)
  -> Option<TraceDifference>  -- actual existing first_difference
```

EncodeFailure and DecodeFailure are located tooling results:

- Capacity{table,needed,limit,locator}.
- ProgramMismatch{locator}: the observation's program is not equal to PreparedA.fullProgram, including either side of a record.
- OutsideProgramClosure{field,locator}: a held expression, function, slice or name has no program-closure candidate.
- WrongRepresentation{block,table,row,column,definition}: the image is outside the representation domain, including a non-suffix StateBlock remainingExprSlice or armSlice in either Canonical or Step mode (Opus F2). Empty slices remain (0,0).
- OutsideObservationDomain{condition,locator}: a typed value violates O1-O4, including (Pending, None).
- WrongProfile.
- HostUnrepresentable{field,locator}.

None of these is Outcome::Rejected, Outcome::Exhausted, error4 or another A semantic outcome. None is a StepRecord. Each locator identifies the refused field without dropping or coercing it. The exact adversarial-input and admission contract for these proposed adapters is for Opus. This author does not introduce an untrusted host consumer.

PreparedA retains full Program syntax and occurrence metadata alongside its
ordinary bounded tables. It traverses the whole program, including unused code,
without evaluating expressions or following named calls. Preparing two different
program tables must not require a different interpreter implementation. Compiling
a layout with immutable program specialization remains a later, separately
identified realization of the same relation, not the initial baseline here.

L3 records the Opus F1(a-c) preparation clarifications in design.md: exact
syntax-name membership, Match path numbering (scrutinee 0, arm k at k+1), and
left-to-right post-order constant traversal without interning. F2 requires
suffix membership for decoded state slices in both modes and as a StepInv row
check. These revised definitions require fresh Muse and Opus review before a
complete preparation/decode or determinism claim; no codec is implemented here.

Opus F3/F5 require the canonical unfolded-size capacity envelope and incremental
host capacity refusal at the first overflow, never after full unfolding. H1
keeps every proposed encoder/decoder/adapter host-only, with no untrusted or
network consumer. F4's canonical-representative enforcement remains UNSELECTED
under L04. These requirements grant no execution or resource admission.

SuppliedPost and auxiliaries are candidate witnesses, not host-computed truth.
The proposed emitter must construct actual constraints from pre/program/post
operands. packStepWitnessA only orders supplied data and checks shape/type; it
must not decide transition acceptance, fill missing state from expected JSON,
or turn a host evaluator result into an acceptance bit. The existing fetch
fragment supplies only a read primitive; it does not implement emitStepA.

## Ordered external data interface

For the ordinary comparison candidate, logical input order is ProgramBlock,
pre StateBlock, supplied post StateBlock, then operation-specific auxiliary
records in the operation manifest's dependency order. The proposed output is
the full supplied post image after constraint checks, plus a rule/charge event.
Actual SDK public/private/constant allocation and transcript order are pending
Opus decisions. “Input order” here specifies a data adapter, not a native proof
statement or exposure policy. No public-only or private-only state commitment
is silently inherited from the fetch demonstration wrapper.

The operation manifest must cover distinct components for generic code fetch,
typed control/top-frame dispatch, name lookup, argument binding/accumulation,
type validation, ordered matching, structural equality, scalar arithmetic,
immutable node allocation, environment read/insert, stack/control updates,
fuel/outcome update, and untouched-field preservation. Each operation has an
independent witness producer description and actual Instruction consumer.
The manifest must identify the complete write set for each rule; all other
state rows, program rows and active lengths require preservation wiring.
Metadata or a host comparison is not that wiring.

## A dispatch families and exact boundaries

Dispatch first respects published Outcome: a non-Pending pre-state has no A event and no StepRecord. A (Pending, None) pre-state is outside the observation domain (OutsideObservationDomain, no StepRecord, no accepted post-image); it is not a semantic result. Dispatch then considers the current Control. Return with an empty stack, and Error, publish free before the zero-fuel check. Other pending states with no fuel publish Exhausted at zero charge. Otherwise the source machine rule consumes one A unit. See the pinned reference.rs and main.k for the authoritative transition conditions.

| Control/source family | Actual rule membership to retain | Circuit responsibility |
|---|---|---|
| Eval | literal, var, let, binary, unary, if, call-empty/nonempty, match | Fetch all needed source payload; schedule exact pending frame/control |
| Push | success 135 or frame-cap error4 at 140 | Preserve pending versus installed frame; full environment; exact depth check |
| Return/top frame | all nine Frame variants | Return/pop/replace order, restored environments and typed primitive/result control |
| Invoke | first source-name match, line200 | Ordered scan; Some versus None function result |
| Enter | missing202 / bind204 | Every ordered argument/parameter, duplicate-binding and type behavior |
| Bound | None206 / Some208 | Failure versus even empty successful map; saved caller return frame |
| Cases | empty212 / first-arm214 | First arm only, value and complete saved map; retain remaining arms |
| CaseResult | None217 / Some220 | Retry next arm or schedule Scope with distinct binding and saved map |
| PrimitiveResult | Ok223 / Err225 | Preserve Result discriminator and all value/error data |
| Publication | final80 / error82 / exhaustion84 | Exact complete state and zero-charge event; no later prefix executed |

This is a component coverage request, not a substitute reimplementation of the
source relation. Full A helper work can require bounded scans/structural visits
inside one logical rule. Such physical work must not become B helper charges
or added A reference events. If a proposed realization needs a different source
clock, it must use the separately reviewed B profile instead of relabelling A.

## Prefix materialization and origins

Each comparison includes an initial image and every ordered before/after image;
program, control, current map, every frame and saved map, stack_limit, both
fuel values and outcome are reconstructed. An empty record sequence is a
present zero-step trace, not an absent layer. Replayed deltas are acceptable
only with a complete base and ordered materializer that reconstructs those
same fields; this baseline stores full images and chooses no delta codec.

Event charge is the A source event charge, stored as an exact u8. ruleStartLine is one of the 39 line values in crosswalk rule_line_tags and resolves only under the pinned K source identity. Record condition O4 ties charge 0 to lines 80/82/84 and charge 1 to every other line. A trace envelope records the exact source, profile, proposed layout membership and declared comparison domain. Consecutive records must satisfy previous.after == next.before. The existing comparator does not check this, so future observation consumers must report continuity separately. first_difference's order remains: identity, initial eight fields, record rule/charge/before/after, then length. Never compare only terminal values.

Grouping by m=1/2/4/8 retains every internal record and stops at the earliest
source outcome; padding is not a reference event. prefixWork starts at zero,
has records.length+1 entries, and sums actual charges. Technical cuts retain
the full roots and physical data needed for resumption; they introduce no A
Waiting state or financial commit. Physical projection is an explicit boundary
operation and never supplies missing record data after the fact.

A public Snapshot does not carry control/frame source-occurrence identities.
Identical expressions can occur at distinct sites, and structural equality
cannot recover the original site. encodeObservedA may select the first equal
source occurrence solely to reconstruct the same typed expression. A separate
optional ExecutionOrigins observation must carry live controlOrigin, ordered
frameOrigins and event ordinal from the actual producer when provenance is
claimed. Absence is reported as absent, not zero or a guessed occurrence.
The current private ImageMachine does propagate such origins; its fields cannot
be exported or silently adopted by this document. B requires these fields and
their resume counterparts in its own state.

## Falsifiable normal checks after adoption and separate execution admission

1. For every admitted constructor, decode(encode(observation)) equals the whole
   typed observation. Each exact error u8 and Option/Result arm is retained.
   This is a requested future check, not a proof statement or result.
2. Use the independently frozen eight-program/216-prefix corpus unchanged, with
   its source identities. Compare every materialized initial and record state;
   preserve missing/unexecuted layers and first divergence locations.
3. Independently specified normal cases must include successful empty binding,
   Some(function row0), Nil versus false versus UInt0, pending Push versus an
   installed frame, a saved shadowed local, non-tail recursion with pending
   multiplication, two call arguments of distinct values, and a tree whose
   shared subvalue is visited twice. Root arranges independent expected data;
   this proposal does not generate it from the codec/emitter.
4. Run the same emitter shape with at least distinct factorial and list/tree
   program tables. Actual SDK construction/evaluation and complete output
   comparisons require their own permission and evidence; native proofs and
   ledger/stagenet qualification remain later evidence classes.
