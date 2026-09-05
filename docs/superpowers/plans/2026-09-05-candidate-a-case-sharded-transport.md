# Candidate A Case-Sharded Transport Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this addendum task-by-task. Root assigns each bounded task and owns admission; this draft authorizes no exports or implementation.

**Goal:** Preserve all78 independent authority cases and1557 events while replacing infeasible whole-loop JSON transport with native case shards and bounded-memory admission.

**Architecture:** Each reviewed literal Quint wrapper runs one unchanged case from its unsigned state. Small schema3 package/admission manifests bind78 original native ITFs and78 per-case JSON documents; independent Python validation processes the complete inventory in order with bounded event windows. Existing agreement and authority semantics, event payloads and schema2 inventory remain fixed.

**Tech Stack:** Installed Quint0.32.0 Rust backend, Node24.18.1, existing Python3.13 environment and standard library. No tool upgrade, custom Quint fork or new runtime dependency.

## Global constraints and adoption status

This is a proposed transport/interface addendum, pending root and non-author review. It supplements adopted producer/checker plans and ab7c827 without rewriting their historical bytes. It replaces their exact-two-raw-file transport, whole-document Python loading and Task6 whole-package copying requirements only as stated below. It does not accept A4 or waive any XML obligation. Root must admit exact implementing code, original RED/GREEN receipts and complete actual package results before acceptance.

Inventory remains schema2 with ordered descriptor fields `case_id,lifecycle,profile,scenario,control` and `event_count`. Its canonical SHA256 remains `8a1a6136afc9fda3c29e050df8b80144750cc21365e9194924401c155dc6402b`. Counts remain installment32 cases/638events and swap46cases/919events, total78/1557. Case ordering and individual event schedules do not change.

## 1. Local feasibility evidence

1.1 Installed Node reports MAX_STRING_LENGTH536870888 and heap_size_limit4496293888. Read-only command: `node -p 'JSON.stringify({node:process.version,maxString:require("buffer").constants.MAX_STRING_LENGTH,heapLimit:require("v8").getHeapStatistics().heap_size_limit})'`.

1.2 Installed Quint's `dist/src/rust/commandWrapper.js:261–273` collects the Rust evaluator's result as one readline string; its simulate path parses the whole result at line81. `dist/src/cliReporting.js:203–214` constructs the entire JSON string before writeFileSync. Both are whole-string boundaries, so a final-writer-only repair is insufficient.

1.3 Exact local tool bytes:

| File | SHA256 |
| --- | --- |
| `/usr/local/bin/node` | `f3432a45b03b2da0d270095fdd8813dc34cbea73f5fc8b18c7a384b7cf9b333a` |
| `/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/package.json` | `08355263da6adb7d5578c2cddb9b0e9634d79cfb2f73e4679888031067c332a4` |
| Same package `dist/src/rust/commandWrapper.js` | `1f26d3f1c31529572b5be32276d2d94c19a3d0298dbf444891802f744e337e1c` |
| Same package `dist/src/cliReporting.js` | `823889ffe67fa0132f51f2a40d40bb663d8b6280c4c1077dfe9dfb77be9ba66e` |

1.4 Root supplied state-only compact ASCII estimates: installment1545385554bytes and swap546949423bytes, excluding latest-event computations and formatting; maximum state8086640/2396701bytes. Both complete-loop lower bounds exceed the Node string cap. These are independent expected-state estimates, not actual native exports or model-check results.

1.5 Read-only resource snapshot:31GiB total RAM,23GiB available,643GiB disk free. This is not a memory reservation or measured export peak. Disk capacity is not the present hard blocker. Larger heap alone cannot fix the string limit; lower trace coverage is forbidden.

1.6 A read-only enumeration of all78 independent expected histories found the following largest sums of compact serialized raw-state objects including full authorityState and latestEvent:

| Loop | Global ordinal | Largest case | Events | Sum bytes | Largest individual raw state |
| --- | ---: | --- | ---: | ---: | ---: |
| Installment | 14 | recover-r1-timeout100 / ordinary / SignAfterResolve | 28 | 153736575 | 8743267 |
| Swap | 44 | funded2-timeout100 / ordinary / SignAfterResolve | 27 | 34024584 | 2992825 |

Each value was computed from `encode(event['after'])` and `encode(event['latest'])`, with compact JSON, state metadata/local cursor, and one separator byte per state. This estimate used caseIndex0 and Python's ordinary integer encoding. Nominal headroom below the536870888-character cap is383134313 for installment and502846304 for swap; it is not guaranteed native headroom. The estimate excludes actual native bigint-carrier overhead, top-level metadata and the Rust result envelope, including any duplicated trace references materialized by serialization. Installed cliReporting203 uses compact stringify without indentation. It is not an actual Quint file measurement. The original read-only tool terminal was chunk997fce, exit0. A native largest-case pilot remains mandatory; if it still exceeds the cap, stop for a new reviewed transport decision rather than silently dropping fields or dividing a lifecycle mid-case. The pilot must also confirm each wrapper's selected main module and actual source-order vars array equals the required authorityState/latestEvent/caseIndex/cursor order.

## 2. Numbered EARS requirements

**CT-001 — Unchanged semantics.** When a shard executes, the system SHALL execute the exact existing case descriptor, ordered selectors, generic/lifecycle guards, independent signing profile, calculations and state updates. It SHALL retain full programs, maps, plans, proof bindings, parent/nonce history, raw calculations and rejected/losing records. No field may be replaced by a hash, shared ID, summary or omitted value inside the raw event/state carrier.

**CT-002 — Closed native inventory.** When export is requested, the producer SHALL execute all78 fixed native case wrappers, one successful complete run each. It SHALL not select a runtime-supplied scenario or seed successful authority state. A failed, diagnostic, incomplete or oversized run SHALL remain a failed gate with its original receipt; it SHALL not be replaced with independent checker output.

**CT-003 — Independent initialization.** When a wrapper initializes, it SHALL assign that case's actual unsigned state, case-start event, fixed global caseIndex and cursor0. Every shard SHALL have exactly one case-start at local index0 and exactly one case-end at its final index. There SHALL be no intra-shard reset, omitted transition or cross-shard carry of authority state.

**CT-004 — Explicit reset semantics.** Because cases are already independent, the transport SHALL replace the former between-case reset transition with a native initialization for each case. Each first exported event SHALL bind before=after to that shard's actual unsigned first state. This is an explicit change of reset observation, not byte equivalence with the former whole-loop trace. No formerly required authority transition or case event is removed; each prior case-start is still present. Cross-case adjacency SHALL not be asserted or fabricated.

**CT-005 — Complete local provenance.** For local state index k, each event SHALL reference its exact original raw shard path/hash, after_index=k and before_index=max(0,k-1). The raw state metadata index, cursor and event sequence SHALL equal k; raw caseIndex SHALL equal the fixed global inventory ordinal. All latest-event fields and full before/after states SHALL remain value-identical to their original raw positions, apart from the existing explicitly permitted outer integer sequence conversion.

**CT-006 — Complete global admission.** When the checker returns success, it SHALL have validated the exact ordered78 descriptors and1557 events, every source/input/case/receipt pin, every case computation and authority transition, every local adjacency, and every terminal status. No subset, missing-case default, skip, resume-as-acceptance or count-only shortcut is permitted.

**CT-007 — Bounded processing.** While validating or exporting, Python SHALL hash files in chunks, parse one event/raw state at a time, and write incrementally. It SHALL not load all raw bytes, all parsed cases, all receipts or canonical full-package strings into memory. Persistent processing windows SHALL contain only the previous/current raw states, current submitted event and expected record. Transient codec buffers and encoding/decoding copies bounded by this event window are permitted; this is not a literal four-Python-object limit. Existing independent history construction may retain one case's immutable expected history; it SHALL not retain all78 histories. Peak memory SHALL be measured in the native pilot and bounded Python admission rather than inferred from serialized byte size.

**CT-008 — Strict stream parsing.** While parsing JSON incrementally, the system SHALL preserve duplicate-key rejection, exact field inventories, integer/boolean distinction, tuple/list/unit distinction, duplicate map/set detection, UTF-8 validity, trailing-data rejection and ordered arrays. Chunk boundaries SHALL not change parsing results. JSON strings with braces, escapes or split multibyte characters SHALL not be treated as record boundaries. A resource-limit failure SHALL be reported explicitly and SHALL not be accepted as a semantic mutation kill.

**CT-009 — Original evidence.** For each native shard, the recorder SHALL retain original source/import/tool bytes, command, cwd, environment, raw stdout/stderr, terminal status, elapsed time and emitted raw artifact hash. Repeated source/runtime closures may be content-addressed once, but every receipt SHALL identify the exact archive hash and verify unchanged bytes during its command. No reconstructed original run or rewritten successful receipt is allowed.

**CT-010 — Unchanged mutation obligations.** Every adopted Task6 semantic and provenance mutant SHALL retain its effective-mutation check, independently rebound raw/hash/indices where required, semantic rejection, corrected counterpart pass and unrelated honest pass. Complete-package mutation controls SHALL still check the actual admitted package and recheck the unmodified package. Process isolation, streaming or disk-backed storage SHALL change resource lifetime only, not verdict criteria.

## 3. Exact native wrapper and source contract

### OpenSpec requirements and acceptance scenarios

**Requirement CT-001: Preserve complete semantics.** Scenario: WHEN an honest full case is exported through its wrapper, THEN its ordered semantic events, full observations and retained records equal the unchanged case contract; changing any full program node or proof binding is rejected.

**Requirement CT-002: Require every native case.** Scenario: WHEN a submitted package omits one wrapper's raw trace or duplicates another case, THEN admission fails even if submitted totals or hashes are rewritten.

**Requirement CT-003: Native unsigned initialization.** Scenario: WHEN wrapper014 starts, THEN its first authority state is unsigned, caseIndex is14 and cursor is0; a seeded signed state or wrong ordinal is rejected.

**Requirement CT-004: Explicit independent resets.** Scenario: WHEN a second case begins in its own native shard, THEN before=after binds that shard's unsigned first state, with no asserted cross-shard edge; inserting case-start later within either shard is rejected.

**Requirement CT-005: Exact local provenance.** Scenario: WHEN an event claims a nonadjacent previous index, substituted latest field or foreign raw hash, THEN linkage fails; the original event with exact local indices and bytes passes.

**Requirement CT-006: Complete admission.** Scenario: WHEN all78 original native shards and case documents satisfy every semantic, pin and inventory check, THEN success reports78/1557; WHEN the last shard fails, THEN the overall result is failure, not partial success.

**Requirement CT-007: Bounded memory windows.** Scenario: WHEN many cases are processed sequentially, THEN completed cases/raw file byte buffers are released before subsequent cases, while permitted one-event codec copies and one-case expected history remain bounded; no whole-package canonical string is constructed.

**Requirement CT-008: Strict streaming JSON.** Scenario: WHEN duplicate keys, malformed UTF-8, illegal numbers or trailing data cross a reader chunk boundary, THEN parsing fails just as for contiguous input; honest escaped strings split across boundaries pass unchanged.

**Requirement CT-009: Original execution evidence.** Scenario: WHEN a shard receipt points to a different tool/source archive or changed terminal stream, THEN root byte admission fails; a producer-created receipt alone never counts as proof of execution.

**Requirement CT-010: Effective mutation triples.** Scenario: WHEN a semantic mutant is fully rebound to consistent raw hashes and local indices, THEN provenance passes but independent semantics rejects it; the corrected and unrelated honest controls pass, and a no-op mutation fails setup rather than claiming a kill.

Global ordinal g is the descriptor's index in the unchanged inventory,0 through77. Installment uses g0–31; swap uses g32–77 and local table index g-32. File paths use zero-padded decimal width3:

```text
specs/quint/s02/candidate_a_integrated_case_000.qnt
...
specs/quint/s02/candidate_a_integrated_case_077.qnt
raw/case-000.itf.json
cases/case-000.json
case-000-export/receipt.json
```

The path pattern above is a closed generated set, not an open glob accepted from submissions. Each wrapper module is named `candidate_a_integrated_case_000` through077. Flat source paths preserve the existing `./` Quint import resolution rule. Producer author confirmed this interface does not conflict with Task1 observer code.

The planned `candidate_a_integrated_driver.qnt` takes `const CASE_A4: A4Case` and `const CASE_INDEX_A4: int`, replacing CASES_A4. Its complete machine-body replacement is:

```quint
const CASE_A4: A4Case
const CASE_INDEX_A4: int
var authorityState: AAuthorityExecution
var latestEvent: A4Event
var caseIndex: int
var cursor: int
action init = all {
  authorityState' = initialA4(CASE_A4), latestEvent' = startA4(CASE_A4),
  caseIndex' = CASE_INDEX_A4, cursor' = 0
}
action step = {
  if (latestEvent.kind == "diagnostic") all {
    false, authorityState' = authorityState, latestEvent' = latestEvent,
    caseIndex' = caseIndex, cursor' = cursor
  } else if (cursor < CASE_A4.steps.length()) {
    val event = observeA4(authorityState, CASE_A4, cursor + 1, CASE_A4.steps.nth(cursor))
    all {
      latestEvent' = event,
      authorityState' = if (Set("transition", "adversarial-derivation").contains(event.kind))
        applyA4(authorityState, CASE_A4, event.arguments) else authorityState,
      cursor' = cursor + 1, caseIndex' = caseIndex
    }
  } else if (cursor == CASE_A4.steps.length()) all {
    latestEvent' = endA4(authorityState, CASE_A4, latestEvent), authorityState' = authorityState,
    cursor' = cursor + 1, caseIndex' = caseIndex
  } else all {
    false, authorityState' = authorityState, latestEvent' = latestEvent,
    caseIndex' = caseIndex, cursor' = cursor
  }
}
val noDiagnosticA4 = latestEvent.kind != "diagnostic"
val sourceInvariantA4 = caseIndex == CASE_INDEX_A4 and
  (if (CASE_A4.descriptor.lifecycle == "installment") safetyI(authorityState) else safetyS(authorityState))
val completeA4 = caseIndex == CASE_INDEX_A4 and latestEvent.kind == "case-end"
```

The existing common/observer imports and types remain as specified in the producer plan. No exported state acquires a payload summary or successful status shortcut. Ordinary and negative terminal behavior is unchanged.

Exact wrapper rendering, added to the existing producer inventory generator (not imported by the checker):

```python
def case_wrapper(global_index):
    rows = inventory()["cases"]
    if type(global_index) is not int or not 0 <= global_index < len(rows):
        raise ValueError("fixed case index")
    installment = global_index < 32
    table = "INSTALLMENT_CASES_A4" if installment else "SWAP_CASES_A4"
    local_index = global_index if installment else global_index - 32
    name = f"candidate_a_integrated_case_{global_index:03d}"
    return (f"module {name} {{\n"
            '  import candidate_a_integrated_cases.* from "./candidate_a_integrated_cases"\n'
            f"  import candidate_a_integrated_driver(CASE_A4 = {table}.nth({local_index}), "
            f"CASE_INDEX_A4 = {global_index}).* from \"./candidate_a_integrated_driver\"\n"
            "}\n")
```

All78 generated wrapper bytes SHALL be reviewed and pinned, along with their generator and transitive imports. The two former full-loop entry modules are no longer export entries. They SHALL not remain hidden expected inputs in producer receipt validation, checker ENTRIES, source inventory or actual-package tests. If existing supplemental tests retain such modules, they remain explicitly pinned test-only roots and cannot substitute for78 native entries.

Each native command uses its fixed wrapper and bound event_count-1, `--backend=rust --seed=42 --max-samples=1 --n-traces=1`, invariants `noDiagnosticA4 sourceInvariantA4` and witness `completeA4`. Example for g0, whose unchanged count is17:

```bash
quint run specs/quint/s02/candidate_a_integrated_case_000.qnt --backend=rust --seed=42 --max-samples=1 --n-traces=1 --max-steps=16 --invariants noDiagnosticA4 sourceInvariantA4 --witnesses completeA4 --out-itf .superpowers/sdd/a4-producer-receipts/case-000-export/case-000.itf.json
```

The recorder resolves and pins actual Node/Quint paths as before. Root stages the exact emitted bytes at `raw/case-000.itf.json`; copying does not reserialize them. Source metadata must name the corresponding native wrapper. Each receipt's artifact map is exactly `{case-000.itf.json: raw_sha256}` for its ordinal, with normal full terminal witness/invariant evidence. No final exports are authorized until this addendum and implementing code are admitted.

## 4. Exact package and admission schema changes

Package and root admission use integer schema_version3 and string transport `case-sharded-itf-v1`. This prevents a schema2 consumer from silently misreading the new transport. Root inventory remains schema2 and unchanged.

The small `cases.json` package manifest has exactly these keys:

```text
schema_version, transport, inventory_sha256,
source_pins, input_pins, case_pins, receipt_pins, shards
```

Root `admission.json` has exactly:

```text
schema_version, transport, inventory_sha256,
source_pins, input_pins, case_pins, receipt_pins, entries
```

Each pin map is a nonempty mapping from canonical relative path to lowercase64-digit SHA256. Source pins resolve under source_root; the other three pin maps resolve under input_root. Input, case and receipt path sets are pairwise disjoint. All four submitted maps SHALL exactly equal root-admitted maps. Neither package nor admission has permission to define a smaller required case/input/source inventory.

To avoid a circular case-file hash dependency, root may first issue `capture-admission.json` with exactly `schema_version,transport,inventory_sha256,source_pins,input_pins,receipt_pins,entries`, the same schema3 values and exact78 source/raw/receipt closure but no case_pins. This document authorizes staging only and SHALL be rejected by final checker/package admission. The producer stages all78 case files from these admitted native bytes, emits their actual hashes, and root then independently reviews those bytes and issues full admission.json with case_pins. No case file or pin map is self-admitted by its producer.

`input_pins` is exactly78 raw paths `raw/case-{g:03d}.itf.json`; `case_pins` is exactly78 paths `cases/case-{g:03d}.json`. `entries` maps each of the78 exact case_id strings to its fixed wrapper source path. `receipt_pins` is the exact root-reviewed original receipt/archive/member set for all78 runs plus required tool/source archives. Its required set is established by external root receipt admission, not by scanning a submitted receipt as proof of execution.

`shards` is an ordered array of78 records. Each record has exactly the five unchanged descriptor fields plus `global_index,event_count,entry,input_path,case_path`. global_index is an integer, never bool; event_count is the unchanged root inventory value. Entry/raw/case paths are independently derived from the index as above. No shard supplies a free-form event offset or sequence base.

Each `cases/case-{g:03d}.json` document has exactly the old single-case fields: the five descriptor strings plus `events`. Each event has unchanged exact fields `case_id,profile,sequence,kind,arguments,observed_guard,computations,before,after,provenance`. Nested ITF carrier and computation extraction unions remain unchanged. The per-case document has no package-level pin map and cannot be accepted independently as the full package.

Native ITF retains exact top metadata, vars and raw state fields from Task5. Metadata source is the fixed wrapper. vars remains ordered `authorityState,latestEvent,caseIndex,cursor`. Every raw state is accounted for, every local state index equals its position, and every caseIndex equals global_index. The local before/after rule applies even at case0 of swap; there is no loop-local caseIndex reset.

## 5. Bounded Python producer and checker changes

### Producer Task2 replacement

- [ ] Replace only the planned multi-case driver body with the complete single-case body in section3; retain all observer logic and frozen cases/selectors.
- [ ] Add and review all78 generated wrappers. Change driver tests to explicit single-case instantiations and retain their original assertions over ordinary, stale and negative executions. Any assertion that previously tested between-case reset SHALL now test independent unsigned initialization and fixed global ordinal; no authority assertion may be deleted.
- [ ] Add a compiling wrong-global-index control: wrapper g32 passes31 into CASE_INDEX_A4 while its descriptor remains g32. A producer/checker positional test must reject the mismatch; restore32 before GREEN. Record recursive typechecks for every wrapper/import closure, all existing producer tests and added positional tests.
- [ ] Measure the largest complete expected case including authorityState/latestEvent and exact scalar/carrier representation before native export. Then run a bounded native largest-case pilot under the same invariants/witness, retaining full original outputs. A pilot failure is investigated; it does not authorize missing case export or arbitrary subdivision within authority history.

### Producer Task3 replacement

- [ ] Keep the existing CLI flags input-root, source-root, inventory, admission, output and quint; add required `--mode` with exactly `stage-cases` or `seal-manifest`. In stage-cases, admission must be capture-admission.json and output is a staging-only report. In seal-manifest, admission must have the full schema3 admission fields and output is the small final package manifest. Case documents reside at their fixed case paths under input_root. Exclusive creation and failure cleanup rules must not overwrite admitted bytes.
- [ ] Replace full ITF `load(read_bytes())` with streaming strict top-level/state iteration. Validate metadata and fixed counters before converting each event. Retain prior/current raw authority state only; write each case document incrementally with the unchanged event field mapping. Compute case-file SHA256 over actual emitted bytes and require exact root admission before final package output; use a separately identified staging-export phase to obtain bytes for root admission, never a self-admitted final package.
- [ ] Retain original raw-byte SHA256 as provenance anchor. Incrementally hash raw/source/receipt files; never keep complete pin-file byte arrays solely to hash them. Preserve the existing typed producer parser, raw value comparisons, selector shape checks, actual computations and denied/end atomicity checks.
- [ ] Emit the manifest only after all78 case exports and all required pins/records are complete. If any shard fails, return nonzero with its path/index and do not print complete success.

The staging report has exactly `ok,scope,cases,events,inventory_sha256,case_pins`; success means ok=true, scope=`staging-only-not-package-admission`, cases78 and events1557. It is neither a package nor a root admission. Seal-manifest revalidates the exact source/input/receipt/case hashes and complete shard fields without rewriting case files. Exact CLI forms are:

```bash
/home/charl/Moriarty/.venv/bin/python scripts/export_s02_candidate_a_integrated.py --mode stage-cases --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission evidence/s02-candidate-a-completion/a4/capture-admission.json --output evidence/s02-candidate-a-completion/a4/staging-report.json
/home/charl/Moriarty/.venv/bin/python scripts/export_s02_candidate_a_integrated.py --mode seal-manifest --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission evidence/s02-candidate-a-completion/a4/admission.json --output evidence/s02-candidate-a-completion/a4/cases.json
```

### Checker Task5 transport supplement

- [ ] Preserve a4_carrier, a4_agreement, a4_authority, a4_cases and a4_inventory semantics unchanged. Do not import producer fixture/render/export code.
- [ ] Independently derive the fixed entries, raw/case paths, schema3 fields and78 shard records from descriptors()/history() and the global ordinal. Update PYTHON_SOURCES only for actual new implementation modules/tests, identically reflected in reviewed producer closure. Keep every frozen source pin, including moriarty/__init__.py.
- [ ] Replace byte-returning pin_bytes in package admission with streaming digest validation. Replace two-file RAW_NAMES/loop grouping with the exact78 raw and78 case-file sets. Verify all pins before accepting any result; avoid holding receipt archive bytes.
- [ ] Retain bind_event's semantic contract but consume previous/current raw state arguments from the bounded reader. Keep the focused raw-field equality check mandatory. Expose a bounded `check_package(manifest_path,admission_path,source_root,input_root,inventory_path)` entry that returns the existing finite-record report only after complete validation. The CLI flags remain cases, admission, inventory, source-root, input-root, report; cases points to the schema3 manifest. A whole-document in-memory compatibility function may remain for focused units but SHALL not be the actual-package path.
- [ ] Iterate case JSON and raw ITF state arrays together. Compare exact fields, independent expected history, counters, observed raw computations, before/after state and local provenance; reject unequal lengths and trailing data. Per-case sequence0 requires initial before=after and unsigned expected state. Every later event requires actual local adjacency; case-start later than0 is forbidden.
- [ ] Final report remains cases78/events1557 with finite-record/symbolic-premise limitations, plus transport `case-sharded-itf-v1`, shards78 and validated input/case/receipt counts. Neither streaming completion nor artifact hashes prove cryptographic signatures or independent model checking.

The streaming reader/writer implementation is a new reviewable transport unit. Root SHALL require its complete code and exact additional source/test closure before dispatch. This requirement/interface addendum does not authorize improvising an unchecked parser during a producer export window.

## 6. Task6 sequential file-backed strategy

6.1 Preserve all27 named semantic mutants, the raw latest-field substitution triple, all five provenance locator cases, duplicate JSON/map/set/unit controls, path/symlink controls, and all seven complete-package mutation controls. No test may be skipped because the full transport is now sharded.

6.2 Synthetic semantic controls SHALL use one case at a time. Write honest, mutated and rebound raw/case artifacts to distinct temporary files using incremental serialization. Rebinding SHALL recompute every local index, mirrored latest event, adjacent before/after pair and actual raw SHA256. Mutations that change record size or remove events SHALL rebuild the complete affected case stream, not patch byte offsets in place.

6.3 First assert at least one intended logical mutation occurred. Then establish successful raw linkage of the bad rebound artifacts before expecting semantic rejection. An input parse failure, digest mismatch, resource cap, subprocess crash or timeout is not a semantic kill. Corrected artifacts and the unchanged unrelated honest case SHALL pass the same bounded semantic/provenance API.

6.4 Run mutant triples sequentially, without pytest-xdist or other parallel case retention. A helper subprocess may contain one complete triple to release allocator memory; its exact source, arguments, captured streams and exit SHALL be pinned. Test names and explicit mutant identity remain in the parent report. Completion requires every triple, not only a representative subset.

6.5 Actual complete-package controls SHALL start from admitted native shard bytes. Create small mutated manifest/admission copies and a separate overlay root referencing verified unchanged files through explicit copying or hard links; do not introduce symlinks because admission forbids them. Copy-on-write the one affected file; never write through a hard link to original evidence. Rehash/rebind the changed case/raw stream for semantic controls. Missing/duplicate/extra case, missing transitive pin, changed frozen pin, missing input and changed receipt controls all remain required.

6.6 After each package mutant is rejected, run the complete unmodified package admission again. A valid run SHALL visit all78 shards. Reusing immutable parsed source metadata is permitted only with unchanged source hashes; skipping already-checked case semantics is not. Keep the separate unrelated honest case control without presenting it as full-package acceptance.

6.7 Retain Task5's omitted raw-field equality behavioral RED against the Task6 substitution triple; restore it and finish all tests and actual-package admission with no exclusions. Root schedules the brief source-mutation window so no producer command observes moving checker pins.

## 7. Independent review and acceptance checklist

- [ ] Review exact schema3 literals, flat wrapper paths, ordinal mapping, native command bounds, source/import closure and root receipt inventory before implementation.
- [ ] Compare the producer and checker independent enumerations against all78 unchanged root inventory records and canonical hash, not only totals.
- [ ] Prove streaming parser equivalence on honest small cases and controls split at every byte boundary around escapes, UTF-8, strings, arrays and duplicate keys; preserve strict JSON/carrier rejection behavior.
- [ ] Compare bounded and existing in-memory semantic/provenance results on complete small honest and rebound-mutant cases before using the bounded path for the actual package.
- [ ] Record all native78 terminal outcomes and exact original raw bytes. Record each case's actual byte count and peak RSS for the largest-case pilot and final Python admission; do not infer feasibility from expected compact sizes alone.
- [ ] Admit complete final source and environment closure, run all producer/checker tests including complete-package and mutation checks, then perform non-author source and original receipt review. Root owns commits and final scope status.

This draft records a transport change, not A4 completion, Council approval or authorization to reduce any semantic obligation.
