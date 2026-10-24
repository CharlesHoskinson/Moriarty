# Complete positive META codec — B38

The new frontend and both directed Quint tests passed once under separate Root selections. This describes evidence as of **2026-10-20T07:14:57.448278+00:00**, the publication-preparation timestamp rather than an operation start time.

| Operation | Exit | Wall seconds | Full stdout / stderr |
|---|---:|---:|---:|
| New frontend | 0 | 29.253105225 | 0 / 0 bytes |
| Complete observations | 0 | 28.005832112 | 106 / 0 bytes |
| Roundtrip and span laws | 0 | 31.872367738 | 106 / 0 bytes |

Each directed command used TypeScript, seed 1 and one sample and reported exactly its one named passing test. All 4,682 recorded checks passed before and after each operation, including exact seven regular source entries and the private fixture at mode `0400`. The [metadata readback](METADATA-RECEIPT-READBACK.json), [authoritative Root readback](actual/ROOT-THREE-ACTUAL-RESULTS-FULL-READBACK.json) and complete raw commands/results/selections/acceptances/streams under `actual/` preserve the scope. No retry or passing-prefix replay occurred. These are finite operation durations, not native performance results.

## Model and predicates

The general direct positive META encoder/reader follows the complete CoreHost4 [called definitions](public-reference/): header, full admission identity, 22 admission bounds, complete layout including separate layout bounds and four capacities, run identity and both RunInt values. It preserves original options, ordered duplicate width entries, empty text and selected-empty digests, raw opaque digest bytes, reserved grammar/finance profiles and Negative/Nat/NatAboveU128 representation. Text arrives through an explicit canonical UTF8 byte projection; opaque digest bytes are uninterpreted.

Every observation has all six fields: `encoded_lanes`, `decoded_metadata`, ordered `blob_spans`, `final_blob_bytes`, `final_width_entries`, `final_cursor`. `c6PositiveMetaCompleteObservationsTest` compares the entire expected root, including schema/profile/ordered case IDs and all three complete observations. `c6PositiveMetaRoundtripAndSpanLawsTest` separately checks general roundtrip, re-encode, full consumption and blob/span/width-counter laws. Encoder, reader and observer neither import expected data nor branch on fixture IDs. [Inputs](contract/SAFE-NATIVE-INPUTS.json), [byte projection](contract/SAFE-TEXT-PROJECTION.json), [contract](contract/SAFE-POSITIVE-META-CONTRACT.json) and [observation interface](contract/SAFE-OBSERVATION-INTERFACE.json) preserve the exact finite domain.

The whole equality predicate ran internally. **No actual observation-vector JSON was separately exported or published**; saved stdout is a test summary. This is not an actual Rust encoder/decoder comparison.

## Ownership and reproduction limits

The complete [Muse Wonder 1.0 HIGH source/API opinion](source-operation-review/FINAL-RESPONSE.txt) approved source/API with no mandatory fixes after full public source and called-definition delivery. Its [actual receipt](source-operation-review/RESULT.json) records 41.493478131 seconds and returned identity. That reviewer did **not** read private expected semantics, the installed tool tree or actual later results. Conditional operation eligibility was fulfilled later through separately selected Root operations with preceding actual passes, full durable raw streams and pin checks. The retained caller preserves that sequence.

The independent owner manually froze NEW whole JSON from the SAFE input/contract/declaration closure before model implementation, then independently transported it to exact typed DATA. Its [SAFE descriptor](independent-premise/SAFE-HANDOFF.json) is an external premise with shared-contract/method correlation; JSON and QNT are one derivation, not two votes. Root and source author kept expected bodies semantically unread. Normal Quint execution necessarily consumed the bound fixture.

**These six public modules cannot run the test entry without the withheld seventh `independentC6MetaExpected.qnt`.** Its JSON/QNT, proof, checker, freeze, membership and derivation bodies are excluded. Historical absolute paths do not promise relocatability. [Dependency provenance](DEPENDENCY-PROVENANCE.json) reuses the existing published capture helper by exact path/pin; never run its historical `main` for this batch. No compiled payload is published.

## Scope and evidence maintenance

Canonical native String/UTF8 relations, byte/profile/u32/usize fit, checked arithmetic success, canonical header/tag/boolean images and applicable direct META string/cumulative-byte/cumulative-width limits remain external positive premises. The direct API does not add outerHOST max-total/max-state-lane enforcement. Zero outer limits and reserved or diagnostic RunInt inputs do not establish admission/genesis validity, authenticity, StepInv or runtime semantics.

No malformed/refusal/security/authentication/constraint soundness/sourcechecker/P1/native/SDK/target/universal refinement/EARS claim follows. All retained G/E/D/N and25refinementOPEN remain distinct; [SCOPED-RESULT.json](SCOPED-RESULT.json) gives the finite result. Six full pre/post pin tables are losslessly archived with [inverse metadata](actual/PIN-ARCHIVE-INDEX.json); hashes do not approve semantics. Historical source-stage pending/unbound labels in `provenance/` are unchanged; later binding/review/actual receipts record their resolution. Publication/integration decisions belong in subsequent Root receipts. Change the source/input/contract and freeze independent expectations before testing changed predicates; do not replay unchanged passing prefixes.
