# Candidate A native case transport

## ADDED Requirements

These prospective EARS/OpenSpec obligations refine A4-R01–R05. They do not
mark A4 accepted. Exact implementation interface: experimental commit5829639,
`docs/superpowers/plans/2026-09-05-candidate-a-case-sharded-transport.md`,
SHA256 `29dcf47fc7a487ca57da8f2fc23b0c5eb9b97bf84c44e3b2c96137af4b24b7ae`.
The schema2 inventory remains78cases/1557events; package transport is schema3.

### Requirement: CT-001: Unchanged semantics

When a shard executes, the system SHALL execute the exact existing case descriptor, ordered selectors, generic/lifecycle guards, independent signing profile, calculations and state updates. It SHALL retain full programs, maps, plans, proof bindings, parent/nonce history, raw calculations and rejected/losing records. No field may be replaced by a hash, shared ID, summary or omitted value inside the raw event/state carrier.

#### Scenario: CT-001: Preserve complete semantics

**WHEN** an honest full case is exported through its wrapper, **THEN** its ordered semantic events, full observations and retained records equal the unchanged case contract; changing any full program node or proof binding is rejected.

### Requirement: CT-002: Closed native inventory

When export is requested, the producer SHALL execute all78 fixed native case wrappers, one successful complete run each. It SHALL not select a runtime-supplied scenario or seed successful authority state. A failed, diagnostic, incomplete or oversized run SHALL remain a failed gate with its original receipt; it SHALL not be replaced with independent checker output.

#### Scenario: CT-002: Require every native case

**WHEN** a submitted package omits one wrapper's raw trace or duplicates another case, **THEN** admission fails even if submitted totals or hashes are rewritten.

### Requirement: CT-003: Independent initialization

When a wrapper initializes, it SHALL assign that case's actual unsigned state, case-start event, fixed global caseIndex and cursor0. Every shard SHALL have exactly one case-start at local index0 and exactly one case-end at its final index. There SHALL be no intra-shard reset, omitted transition or cross-shard carry of authority state.

#### Scenario: CT-003: Native unsigned initialization

**WHEN** wrapper014 starts, **THEN** its first authority state is unsigned, caseIndex is14 and cursor is0; a seeded signed state or wrong ordinal is rejected.

### Requirement: CT-004: Explicit reset semantics

Because cases are already independent, the transport SHALL replace the former between-case reset transition with a native initialization for each case. Each first exported event SHALL bind before=after to that shard's actual unsigned first state. This is an explicit change of reset observation, not byte equivalence with the former whole-loop trace. No formerly required authority transition or case event is removed; each prior case-start is still present. Cross-case adjacency SHALL not be asserted or fabricated.

#### Scenario: CT-004: Explicit independent resets

**WHEN** a second case begins in its own native shard, **THEN** before=after binds that shard's unsigned first state, with no asserted cross-shard edge; inserting case-start later within either shard is rejected.

### Requirement: CT-005: Complete local provenance

For local state index k, each event SHALL reference its exact original raw shard path/hash, after_index=k and before_index=max(0,k-1). The raw state metadata index, cursor and event sequence SHALL equal k; raw caseIndex SHALL equal the fixed global inventory ordinal. All latest-event fields and full before/after states SHALL remain value-identical to their original raw positions, apart from the existing explicitly permitted outer integer sequence conversion.

#### Scenario: CT-005: Exact local provenance

**WHEN** an event claims a nonadjacent previous index, substituted latest field or foreign raw hash, **THEN** linkage fails; the original event with exact local indices and bytes passes.

### Requirement: CT-006: Complete global admission

When the checker returns success, it SHALL have validated the exact ordered78 descriptors and1557 events, every source/input/case/receipt pin, every case computation and authority transition, every local adjacency, and every terminal status. No subset, missing-case default, skip, resume-as-acceptance or count-only shortcut is permitted.

#### Scenario: CT-006: Complete admission

**WHEN** all78 original native shards and case documents satisfy every semantic, pin and inventory check, **THEN** success reports78/1557; **WHEN** the last shard fails, **THEN** the overall result is failure, not partial success.

### Requirement: CT-007: Bounded processing

While validating or exporting, Python SHALL hash files in chunks, parse one event/raw state at a time, and write incrementally. It SHALL not load all raw bytes, all parsed cases, all receipts or canonical full-package strings into memory. Persistent processing windows SHALL contain only the previous/current raw states, current submitted event and expected record. Transient codec buffers and encoding/decoding copies bounded by this event window are permitted; this is not a literal four-Python-object limit. Existing independent history construction may retain one case's immutable expected history; it SHALL not retain all78 histories. Peak memory SHALL be measured in the native pilot and bounded Python admission rather than inferred from serialized byte size.

#### Scenario: CT-007: Bounded memory windows

**WHEN** many cases are processed sequentially, **THEN** completed cases/raw file byte buffers are released before subsequent cases, while permitted one-event codec copies and one-case expected history remain bounded; no whole-package canonical string is constructed.

### Requirement: CT-008: Strict stream parsing

While parsing JSON incrementally, the system SHALL preserve duplicate-key rejection, exact field inventories, integer/boolean distinction, tuple/list/unit distinction, duplicate map/set detection, UTF-8 validity, trailing-data rejection and ordered arrays. Chunk boundaries SHALL not change parsing results. JSON strings with braces, escapes or split multibyte characters SHALL not be treated as record boundaries. A resource-limit failure SHALL be reported explicitly and SHALL not be accepted as a semantic mutation kill.

#### Scenario: CT-008: Strict streaming JSON

**WHEN** duplicate keys, malformed UTF-8, illegal numbers or trailing data cross a reader chunk boundary, **THEN** parsing fails just as for contiguous input; honest escaped strings split across boundaries pass unchanged.

### Requirement: CT-009: Original evidence

For each native shard, the recorder SHALL retain original source/import/tool bytes, command, cwd, environment, raw stdout/stderr, terminal status, elapsed time and emitted raw artifact hash. Repeated source/runtime closures may be content-addressed once, but every receipt SHALL identify the exact archive hash and verify unchanged bytes during its command. No reconstructed original run or rewritten successful receipt is allowed.

#### Scenario: CT-009: Original execution evidence

**WHEN** a shard receipt points to a different tool/source archive or changed terminal stream, **THEN** root byte admission fails; a producer-created receipt alone never counts as proof of execution.

### Requirement: CT-010: Unchanged mutation obligations

Every adopted Task6 semantic and provenance mutant SHALL retain its effective-mutation check, independently rebound raw/hash/indices where required, semantic rejection, corrected counterpart pass and unrelated honest pass. Complete-package mutation controls SHALL still check the actual admitted package and recheck the unmodified package. Process isolation, streaming or disk-backed storage SHALL change resource lifetime only, not verdict criteria.

#### Scenario: CT-010: Effective mutation triples

**WHEN** a semantic mutant is fully rebound to consistent raw hashes and local indices, **THEN** provenance passes but independent semantics rejects it; the corrected and unrelated honest controls pass, and a no-op mutation fails setup rather than claiming a kill.
