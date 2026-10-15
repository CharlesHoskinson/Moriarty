# Verdict: PASS_SCOPED_CURRENT_REFERENCE

The top finding is that, within the expressly nonsecurity and local-reference scope, the exact C1 mechanical repair is present and the supplied actual executions support Run A currentness and the Run B positive subset. This verdict grants no broad compiler, UTXO, Cardano-target, Preprod, proof, ledger, cryptographic, authentication, authorization, custody, or schema acceptance.

Reviewer identity: Meta Muse Wonder 1.0, high effort, as routed for this fresh independent nonsecurity audit. Opus retains ownership of all security judgments; none is made or approved here. This audit performed no tools, execution, network access, or writes. It reviewed the supplied verbatim source, context, fixtures, freeze records, disclosures, and complete run logs.

## C1 Repair Audit

The C1 delta matches the preauthorized mechanical repair:

- `CoreSourceExportV1` and `export_source` are present in `crates/mori-cardano/src/export.rs`, using public `mori_core::parse`, canonicalization, and hash helpers.
- `crates/mori-core/src/lib.rs` matches the stated restored base hash `9d920393…`, and no Core `export.rs` module remains in the supplied Core library surface.
- The Cardano library, types, and projection code use the relocated Cardano-owned export path. The export record’s field names and field order are unchanged.
- The root `Cargo.lock` retains the unavoidable `mori-cardano` workspace-member entry.
- The retained root-lock change necessarily changes the input to the native `source_closure()` calculation because that closure incorporates the root lock bytes. Consequently, the declared native build-identity strings are altered. No ledger source-closure test, SDK qualification, or native qualification was performed, and the resulting native identity remains **unqualified**.
- No advisory production change beyond the authorized relocation and import-path changes is evident in the supplied production delta.

The PRJ-01 wording disposition is treated as Opus’s non-semantic authorial amendment: the current construction is a source-owned export built from `mori_core::parse` plus the canonical Program hash, not a continuing `mori_core::export_source` API.

## Actual-Result Coverage

Run A is supported by its complete supplied log:

- Base `9c63048a18315d7eb3cf114b983bd68c0d842dc6`, tree `40be71ec3602b3cd25fae44f48b2139bde3ca304`.
- Cargo and rustc were both version 1.99.0.
- The offline, locked consumer command exited 0.
- Nine tests passed and none failed: three metadata tests and six P2 projection tests.
- In Run A, the staged wrapper was compiled but not behaviorally exercised. Run A therefore supplies currentness evidence only, not staged-outcome evidence.

Run B is also supported by its complete supplied log:

- The same base, tree, offline/locked consumer manifest, and fresh target directory `/tmp/mori-cardano-pr54-currentness-B` were used.
- The tree command selected 61 package/version pairs from the 63-package consumer lock, with no Git source, Midnight, proptest, or SDK crate shown.
- The run exited 0 in the supplied result record.
- Eleven tests passed and none failed.

Those eleven are **nine pre-existing currentness tests plus two new chained staged tests**. They must not be reported as twenty distinct tests across Runs A and B, because the same nine currentness tests were executed again in Run B. Nor do the eleven tests close the whole Run B gate.

The two new tests assert eight complete staged outcomes: three delivery outcomes and five reserve outcomes. The assertions use complete serialized JSON equality for metadata, Programs, Plans, continuations, executions, ordered steps, ordered effects, captures, residual duties, boundaries, and attempted/committed logical work. Object-member ordering is not treated as significant, but field membership, values, and array ordering are asserted. This distinction is consistent with the supplied oracle methodology.

## Independence And Provenance

The 51-member independent freeze is recorded as preceding assertion authoring. The supplied chronology states that expectations were manually derived and copied, not generated or fitted from actual output. The first green result occurred against pre-existing implementation code; no red assertion, mutation result, or implementation-under-test discovery is claimed.

The test code passes actual returned continuations into subsequent calls after comparing each returned continuation with its complete independent fixture. It does not substitute a deserialized or reconstructed golden continuation. Shared Program and Execution values are compared as JSON rather than being forced through the `/1`-only deserialization route.

Before execution, the supplied receipts state that all 51 freeze members and all 95 Run A source rows were rehashed unchanged. After execution, all 123 applied rows and the relevant Git status were reported unchanged. Relative to Run A, only the consumer module-declaration change and the reviewed test-only staged additions differ; production libraries, manifests, and locks are reported identical.

Provenance limit: because this audit was prohibited from using tools or execution, it did not independently recompute the live-tree hashes. It relies on the root-rehashed pin statement and the supplied manifests, receipts, stdout, stderr, command records, timestamps, and output digests. The pass is therefore a source-and-actual-log audit of the furnished evidence, not a newly executed reproduction.

## Required Disclosures Preserved

The F10 specimen disclosure remains mandatory with every use of the invoice or reserve results. In particular, the invoice specimen is not a safe escrow: repeated unauthenticated `partial(1)` calls can drain escrow into fees and seller payments, leave the thread unable to progress, and leave the seller and reserve obligations unfunded. The conditional-reserve specimen likewise does not acquire authority from its `approved` input.

The staged disclosures also remain mandatory:

- **F11, delivery:** the `ready` event is caller-supplied and unauthenticated. Anyone possessing or copying the continuation can immediately resume with `ready = true`; the supplied result is not evidence of adversarial conditional settlement.
- **F12, reserve:** a caller can supply an excessive `quote` after the relevant early check, commit the second segment, and leave the action permanently unable to complete. The local reference result neither detects nor prevents that behavior.

The persisted-record limitation is also preserved. Raw `StageOutcome` JSON equality in these tests is an internal assertion made alongside fully asserted source metadata carrying `local-reference-stage`, `none`, and `untracked`. It does not create or qualify a labelled persisted transition record, target record, record codec, or target serialization. Persisted outcome, rejection, continuation, and segment records must still embed the three labels verbatim before any such record consumer is treated as satisfying that separate requirement.

## Open And Unexecuted Scope

The following remain open and unexecuted: NS1–NS17, NT1–NT24, SP6, and the supplementary SP7 compatibility source. The positive subset covers only partial SP1–SP5 evidence.

Also unperformed and outside this verdict are the Stage-to-UTXO codec, layout and layout-hash qualification, target descriptor or comparator qualification, Aiken work, proof generation or verification, authentication, custody, ledger interaction, native closure qualification, SDK execution, and Preprod acceptance. Earlier failures and historical limitations remain preserved and are not erased by this scoped pass.

Accordingly, the correct disposition is **PASS_SCOPED_CURRENT_REFERENCE**, limited to the exact C1-repaired tree, Run A’s nine-test currentness result, and Run B’s eleven-test positive subset with eight complete staged outcomes.