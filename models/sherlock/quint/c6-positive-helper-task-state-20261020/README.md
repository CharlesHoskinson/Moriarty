# Complete positive HELPER/TASK/STATE codec — B39

The new frontend and both directed Quint tests passed once under separate Root selections. This records evidence as of **2026-10-20T08:27:04.140639+00:00**, the publication-preparation timestamp rather than an operation start time.

| Operation | Exit | Wall seconds | Full stdout / stderr |
|---|---:|---:|---:|
| New frontend | 0 | 7.221999368 | 0 / 0 bytes |
| Complete eight-key root | 0 | 7.387819184 | 107 / 0 bytes |
| Roundtrip and framing laws | 0 | 6.931407742 | 110 / 0 bytes |

Each directed command used TypeScript, seed 1 and one sample and reported exactly its named passing test. All 4,684 recorded checks passed before and after each operation, with exact nine regular source entries and the private fixture at mode `0400`. Full commands/results/raw streams/selections/acceptances and the retained caller are under `actual/`; see [Root's authoritative readback](actual/ROOT-THREE-ACTUAL-RESULTS-FULL-READBACK.json) and [the publication metadata readback](METADATA-RECEIPT-READBACK.json). No retry or passing-prefix replay occurred. These durations are finite operation measurements, not native performance results.

## General source and finite predicates

The pure general writer and positive reader cover all 11 helper, 19 result, 7 ScalarStaged, 3 task, 7 DATA, 10 frame and 11 control variants, all Outcome/ExhaustReason constructors, every root counter and the complete 41-lane STATE header with its 29-cap descriptor. Helper/result/task widths are 23/4/7. STATE derives all three child sections and their offsets from typed inputs; no none-only restriction or supplied expected STATE payload is used. Tasks preserve bottom-to-top order and duplicates. Owned DATA Vec lengths are distinct from active/alloc/segment counters, which remain unchanged diagnostic fields.

[SAFE inputs](contract/SAFE-INPUTS.json) contain 46 selected cases across five groups: 18 helper records, 20 results, three tasks, three helper sections and two full nonempty states. They exercise all helper/result/staged/task constructors and preserve raw u128 values, `None` versus `Some(0)`, `Const` versus `Data`, nonzero-start empty slices, parent-resume, ordered tasks and full retained state. They do not cover every scalar/enum/phase/option cross-product. Many cap fields equal 32, so this finite test alone is not a descriptor-permutation or mutation-adequacy theorem.

`c6PositiveStateCompleteObservationsTest` compares the **entire eight-key root**: schema/profile, all five ordered case-ID lists, and complete helper/result/task/helper-section/STATE observation groups. Full raw vectors, decoded values, all retained fields, context/cap descriptors, child sections/spans and consumed endpoints are compared. `c6PositiveStateRoundtripAndFramingLawsTest` adds ordinary positive roundtrip, re-encode, record-width, consumed-end, task-count and section-framing laws. These same-author laws are not an independent oracle substitute. Encoder/reader/observer modules import neither cases nor expected data and have no case-ID branch.

**No actual observation-vector JSON was exported or captured.** Whole equality ran inside Quint; saved stdout contains exact test summaries. This is not an actual Rust encoder/decoder comparison.

## Ownership and reproduction limits

The complete [Muse Wonder 1.0 HIGH source/API opinion](source-operation-review/FINAL-RESPONSE.txt) approved source/API with no mandatory fix. Its [actual receipt](source-operation-review/RESULT.json) records 40.920944013 seconds and returned identity. It read full public source, SAFE contracts and called positive Rust API material; it did **not** read private expected semantics or audit the installed tool tree. The opinion's UNRUN and conditional-eligibility language describes its review time. Root subsequently selected each operation separately and satisfied actual pin, durability, preceding-pass and full-stream conditions.

An independent owner froze complete NEW JSON from the selected SAFE7 declaration/input contract before model source, followed by exact complete typed DATA transport. The [SAFE descriptor](independent-premise/SAFE-HANDOFF.json) records that one derivation, strict full-input/coverage checks and transport inverse as external premises. Shared source-derived contracts and the same independent author are method correlation, not additional independent adoption votes. Source author and Root kept private expected semantics unread; normal Quint execution necessarily read the bound fixture.

**These eight public modules cannot run the test entry without withheld ninth `independentC6PositiveStateExpected.qnt`.** Its body, original JSON, proof, checker, freeze, membership and derivation are excluded. No compiled payload is published. Historical absolute runtime paths do not promise relocatability. [Dependency provenance](DEPENDENCY-PROVENANCE.json) records exact declaration/root-codec reuse and the existing published capture helper, whose historical `main` must never be used for this batch.

## Scope and maintenance

The model consumes canonical positive images. Native scalar fit, exact u128 ranges, tag/padding/kind images, checked arithmetic, original checked-cap success and lane limits remain external premises. This is tooling `C6StateContext::from_caps_assertion`, not retained-layout authority or admission. STATE has no run identity by the actual API. The source preserves counters independently; it establishes no cross-field membership, consistency or reachable machine state.

No malformed/refusal/security/admission/StepInv, helper/task execution or clocks, Rust correspondence, constraints/P1/native/SDK/target, universal refinement or EARS closure follows. All G/E/D/N and25refinementOPEN remain distinct. Whole-HOST composition remains next work after this prerequisite. [SCOPED-RESULT.json](SCOPED-RESULT.json) states the finite result and limitations.

Six complete recorded pre/post pin tables are losslessly archived with [full byte-inverse metadata](actual/PIN-ARCHIVE-INDEX.json). Hashes do not approve semantic content. Historical source-stage pending/unbound labels in `provenance/` remain unchanged; later binding/review/actual receipts record their resolution. Draft input `prev` was corrected to Core's `previous` before independent prefreeze; the additive [corrigendum](provenance/contract/INPUT-CORRIGENDUM-AND-INVERSES.json) preserves that history. Publication/integration decisions belong in subsequent Root receipts. Freeze new independent expectations before testing changed predicates; do not replay unchanged passing prefixes.
