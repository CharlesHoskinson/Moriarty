# Positive complete ProgramSnapshot codec — B36

The frontend and both directed Quint tests passed once under separate Root selections. This dated record describes the source and results available as of **2026-10-20 05:21:41 UTC**; that is the publication preparation timestamp, not an operation start timestamp. Raw receipts retain measured durations.

| Operation | Exit | Wall seconds | Complete stdout / stderr |
|---|---:|---:|---:|
| New frontend | 0 | 9.384752683 | 0 / 0 bytes |
| Complete observations | 0 | 11.487500829 | 112 / 0 bytes |
| Count, row and text continuity | 0 | 13.501115351 | 114 / 0 bytes |

Each directed command used the TypeScript backend, seed 1 and one sample, and reported exactly its one named passing test. All 4,682 recorded checks passed before and after each operation, including exact seven-source membership and private-fixture mode `0400`. [The readback](METADATA-RECEIPT-READBACK.json) checks full streams, small receipts, recorded pin rows and the six public source hashes without opening the private expected bodies. No passing prefix was replayed.

## Representation and predicates

The general encoder and positive reader preserve all 22 count-prefixed ProgramSnapshot tables in their declared order, including empty tables, repeated rows, raw slices, options, booleans, marker bits and exact `u128` data. Text uses complete canonical UTF-8 byte lists; recursive TypeExpr uses complete preorder tokens with Apply arity. These are externally supplied lossless projections of native String/AST inputs. The model does not execute the native Rust codec or validate those projection premises.

The two public inputs contrast all-empty tables with a populated multirow specimen, and empty, one-byte and two-byte strings, including two-byte Unicode. The [input forms](contract/SAFE-RUST-PROGRAM-INPUTS.json), [byte-wire inputs](contract/SAFE-BYTE-WIRE-INPUTS.json), [projection](contract/SAFE-TEXT-AND-TYPE-PROJECTION.json) and [positive contract](contract/SAFE-POSITIVE-PROGRAM-CONTRACT.json) specify the exact domain.

`c6PositiveProgramCompleteObservationsTest` compares the entire independently prefrozen observation list: `encoded_lanes`, `decoded_program`, `table_counts`, `table_spans`, `text_spans` and `final_cursor`. Roundtrip and complete-consumption laws are additional conjuncts. `c6PositiveProgramCountRowTextContinuityTest` checks all table/row/text cursor laws, including zero tables and length-prefixed empty text. Neither test replaces whole-observation equality with a subset.

## Evidence and ownership

The [source/API opinion](source-operation-review/FINAL-RESPONSE.txt) is the actual Muse Wonder 1.0 HIGH review, completed in 58.802760055 seconds, with its [identity receipt](source-operation-review/RESULT.json). It found no mandatory source fix and made the later operations conditional on preceding actual passes and Root's durability/pin checks. The [retained caller](actual/ROOT-THREE-SEPARATE-ORDINARY-COMPOSITION.py), three selections, full commands/results/streams and acceptance receipts preserve how those conditions were discharged. The unchanged helper is included as historical source; its `main` belongs to an older task and must not be used here. Its stale internal `root_B35_existing_ordinary_capture_only` run-name is retained history, not this batch's identity.

The independent owner manually froze complete expectations before model implementation, using only the shared SAFE inputs/contracts. The later data-only QNT transport is the same derivation. Its [manual](independent-premise/SAFE-MANUAL-HANDOFF.json) and [transport](independent-premise/SAFE-TRANSPORT-HANDOFF.json) descriptors identify an external premise. The source reviewer did not inspect expected semantics. Root and the source author kept those bodies semantically unread; normal Quint execution necessarily read the private fixture. This is no additional independent vote for Rust or native correspondence.

**The public test entry is not runnable from these six modules alone.** It imports the withheld seventh `independentC6ProgramExpected.qnt` module. No private fixture, expected JSON, proof, checker or derivation is published. Absolute runtime paths in historical receipts are preserved; this package does not claim a relocatable reproduction command.

## Scope and maintenance

This is finite positive representation evidence for two specimens. `caps` is unused by the model: cap satisfaction, host width, cumulative byte/type-node/arity budgets and canonical input formation are external premises. The empty specimen is a structural codec input, not an admitted executable program. There is no malformed/refusal, admission, authority, runtime transition, native/circuit, target, universal refinement or EARS result. All retained G/E/D/N and 25 refinement gaps stay open. Current handwritten PROGCTX production work has its own source/review/validation status.

For reference, [public-reference](public-reference/) retains the actual public Core codec/declaration excerpts and [the source inventory](history/PUBLIC-SOURCE-INVENTORY.json) identifies exact parents and copied ranges. [The archive index](lineage/ARCHIVE-INDEX.json) records lossless gzip copies of complete public source-review deliveries, installed-tool metadata and all six raw pre/post check tables. Their inverses were checked, but huge pin rows are not inline semantic source-review evidence. No private bodies are archived.

Historical `history/` names and statuses such as `pending-root-binding`, `UNRUN` and `source01` describe their original source-stage timestamps; [SCOPED-RESULT.json](SCOPED-RESULT.json) and raw actual receipts record the subsequent passes. Publication review and integration decisions belong in their later Root receipts, without rewriting this dated record. Update the model and independent oracle contracts separately when changing the public grammar or projection, then obtain fresh evidence for changed predicates; do not replay unchanged passing prefixes.
