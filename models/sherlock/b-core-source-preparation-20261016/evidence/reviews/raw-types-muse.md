## Audit outcome

**APPROVE_WITH_SCOPE** for the frozen 104 Rust declarations as a provisional, uninstalled, uncompiled raw-data preparation. This is not schema adoption, B-core adoption, a two-vote decision, compilation evidence, or a product/security claim.

## Scope of approval

Approval covers only:

- Correspondence between the supplied `types.proposed.rs` declarations and the pinned R2 schema/handoff material supplied in the prompt.
- The declaration inventory and named-reference closure as represented in `TYPE-MAPPING.json`.
- The stated omission of the acknowledged blocked closures as omissions, not stubs.

It does **not** cover or imply:

- Rust parsing, compilation, `rustfmt`, Cargo, tests, formal/native execution, or hash recomputation — none was performed.
- Installation, module registration, public API/export status, wire ABI, serialization, `repr`/discriminants, `Eq`/`Hash`/`Default`, validation, admission, constructors enforcing policy, guards, `StepInv`, allocation/capacity enforcement, or behaviour.
- Any security judgment. Security and ambiguous representation decisions remain referred to Opus.
- Adoption of the nominal Rust representations as a frozen interface.

## Source findings

### 1. Declaration inventory matches the stated 104

The supplied source contains the following inventory:

- 8 primitive declarations.
- 24 derived-width nominal carriers.
- 35 tag/rule vocabulary enums.
- `Bounds` and `LayoutCapacities`.
- 21 value, slice, environment, constant/data, helper and outcome declarations.
- 14 program-view, identity and source-occurrence declarations.

Total: 104 declarations.

The 24 derived carriers remain nominally distinct, including `ExprId`, `ConstId`, `DataId`, `EnvId`, `NameId`, `FunctionRowId`, `PatternRowId`, and the separate position/count/work carriers. `ValueRef` correctly preserves the two arenas as `Const { id: ConstId }` and `Data { id: DataId }`.

### 2. Dedicated-sort and indexed/source distinctions are preserved

Examples checked against the schema pointers in the source comments:

- `ConstRow` repeats `marker_free: Bool` in each declared variant and has no generic constructor payload.
- `DataRow` has no marker field and no sequence row.
- `PatternRow` uses `NameId`; `CorePattern` uses `Text`. The mapping correctly keeps source names distinct from indexed names.
- `ExpressionRow` has the eight core expression alternatives only; `ExprTag::ConstructExtensionReserved` remains tag vocabulary and is not converted into an expression payload.
- `ProgramActiveLengths` has the eleven table lengths in the mapped table order.
- `Identity` has the eight schema fields; `RunId` and `SourceOccurrenceId` are finite records only.

### 3. Reserved tags are vocabulary only

Reserved labels such as `GenericNominalExtensionReserved`, `TypedExtensionReserved`, `ConstructExtensionReserved`, `ConstructorExtensionReserved`, `ConstructorFieldsExtensionReserved`, and `SourceSuspensionExtensionReserved` appear only as unit enum variants. No reserved payload, handler, admission behaviour, generic ADT behaviour, or finance behaviour is implemented in the supplied declarations. No security disposition is judged here.

### 4. Blocked closures are omitted, not stubbed

The mapping’s omission entries correspond to the supplied source:

- `B-REP-01`: `HelperControl` / `ResumeId` closure omitted.
- `B-REP-02`: recursive `CoreTree`/`CoreValue`/`CoreExpr`/`CoreFunction`/`CoreProgram` omitted; independently closed `CorePattern` retained.
- `B-REP-03`: `FrameRow` and dependent `ControlRow` omitted; closed `BoundBinding`, `CaseMatching`, and `PrimOutcome` retained.
- `B-REP-04`: `WidthTable`, `BoundaryCapacities`, and `LayoutSchema` omitted; independent `Bounds` and `LayoutCapacities` retained.
- `B-REP-05`: `ExpandedStepId` omitted because ownership of `RunId` is unresolved.
- `B-REP-06`: allocation/segment-base records omitted.
- `B-REP-07`: `StepEvent`, `StepResult`, and `RunSummary` omitted.
- `B-SCOPE-01`/`B-SCOPE-02`: admission, live-state, API/guard, and related vocabularies omitted.

These remain Opus/owner decisions. They are not treated as accidental missing declarations.

## Precise nonsecurity defects / ambiguities

### A. Mapping-note inaccuracy for several slices — documentation defect

In `TYPE-MAPPING.json`, the shared slice note says, in substance, “len `0..2` is a host length `usize`” for entries whose mapped lengths are **not** `0..2` and are not plain Rust `usize`:

- `ValSlice`: `len: ArityCount`
- `ExprSlice`: `len: ArityCount`
- `ArmSlice`: `len: ArmPos`
- `ParamSlice`: `len: ArityCount`
- `OccPathSlice`: `len: OccPathPos`

The `0..2` wording is accurate only for `FieldSlice` and `ConstFieldSlice`, whose Rust fields are `len: usize`.

This does not change the Rust declarations, but the mapping note is imprecise at those five entries and should be corrected in a separately versioned mapping/report, not by silently altering the frozen candidate.

### B. `Identity` field-name ambiguity between schema and handoff sketch — referred, not decided

- Schema pointer: `/identity/Identity/record` uses `profileId`.
- Candidate: `profile_id: ProfileId`.
- ENGINEERING-HANDOFF §3 sketch uses `profile: ProfileId`.

The candidate follows the JSON naming more closely, and the handoff states JSON governs disagreements. Nevertheless, because the handoff also presents a Rust sketch, the exact installed field name must remain an owner/Opus-confirmed interface question before any installation. This audit does not adopt either spelling as a public API.

### C. Host-representation choices remain unadopted

The candidate necessarily chooses raw host shapes such as:

- nominal newtype wrappers around `usize`/`u128` for derived widths;
- `String` for `Text`;
- `Vec<u8>` for digest bytes;
- `Vec<ChildPosition>` for `SourceOccurrenceId::ast_path`;
- `usize` for the `0..2` lengths of `FieldSlice` and `ConstFieldSlice`;
- a named `OccurrenceOwner` enum for the schema’s anonymous `Entry | Function { index }` sum.

These are coherent raw-preparation choices on the supplied text, but they are **not** approved as frozen representation, ABI, allocation, ownership, or public-interface decisions. Any consequential representation decision is referred to Opus. Root integration remains separate and must occur only under exact voted interfaces.

### D. Closed-reference claim is limited to named types

`TYPE-MAPPING.json` lists external named types as `Option`, `String`, and `Vec`, with no undefined named type references. That claim should be read as covering named Rust types only; primitive carriers `bool`, `u8`, `u128`, and `usize` are builtin types and are not separately inventoried as named dependencies. This is a limitation of the inventory wording, not a dangling-reference finding.

## Limitations

Because no tools, execution, compilation, or hash recomputation were used, this audit verifies correspondence only within the bodies and pins supplied in the prompt. It does not independently prove byte counts, hashes, Rust syntactic validity, or absence of files/declarations outside the supplied text.

No replacement code is proposed, and approval does not authorize Root to integrate or adopt the frozen bytes automatically.

VERDICT: APPROVE_WITH_SCOPE
TYPE_CANDIDATE_MANIFEST_SHA256: a242a689888e5962da16696fa009e41949c6eccbda0bcd3e22098c09f26ef4c9
