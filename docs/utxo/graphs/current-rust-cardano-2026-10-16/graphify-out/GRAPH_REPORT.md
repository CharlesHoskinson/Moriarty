# Current Rust Cardano publication graph fragment — R3 documentation

As of 2026-10-16, this source-attributed Graphify fragment is a navigation aid for the published Rust Cardano prototype and its scoped evidence. It extends the earlier dead-end graph query. R3 makes documentation changes only. Its `graphify-out/graph.json` is byte-identical to R2: SHA-256 `879d2a8cf4d18827c34fb5fe686726d928db9a28cd72caec13b5e540264c00c1`. No source extraction, graph rebuild, graph merge or graph edit occurred.

## Published graph structure

The root read-only structural check records 1,356 nodes and 3,620 directed links, with unique IDs and no dangling links. Its classification totals are 3,401 EXTRACTED, 216 INFERRED and 3 AMBIGUOUS under this consumer rule: use `evidence_type` when present; otherwise use `type`. The structural check found 165 links whose classification appears only in `type`. A consumer that requires a literal `evidence_type` field must apply that fallback. This is a read-time interoperability rule; it does not repair the graph or signal semantic approval. The root check records `semanticApproval: false`.

The earlier R1/R2 narrative reported a raw Graphify extraction output of 1,542 nodes and 5,627 edges for 80 Rust inputs. R3 withdraws those output counts as unverified provenance for the published graph. The extraction JSON/cache and transformation counts needed to reconcile that output with the final graph are absent from the retained R2 outputs and named public generation records. No deduplication, filtering or collapsing explanation is asserted. The published graph counts above come from the root structural check; the earlier raw extraction pair must not be used to explain or reproduce the published node/link set. The manifest-listed Rust input count is 80; it is an input count only.

## Source paths and inferred relationships

Graph source paths use two bases. AST links use package-relative `source_file` paths resolved against `SOURCE-MANIFEST.json` `packageRoot`. Source-file nodes instead prefix paths with `experiments/moriarty-cardano-rust/`, so resolve those from the repository root. Do not use one base for both node and link paths.

AST `calls` links are name-based Graphify inferences. The sampled call links carry INFERRED classification and confidence 0.8; same-named symbols can produce an ambiguous target. Treat those links as navigation hypotheses, not established call facts or reviewed semantics. Other structural relationships likewise describe source structure; they do not establish compiler correctness, runtime behavior or proof correspondence.

## Current evidence status, after the frozen R2 graph snapshot

K4H's actual-result review is complete and its captured finite ordinary evidence is adopted narrowly as a prerequisite for K5 source preparation. The adoption covers 107 finite cases, 29/46-slot financial receipts and 72 reviewed public control rows. It grants no setup, proof, key or ledger authority and is not an arbitrary-witness theorem. The package README and inventory contain author-time statements that this review was pending; those statements describe their earlier snapshot and are superseded for current review status by the pinned K4H adoption and actual-result review.

The four-way run remains FAILED(prediction) and unadopted. Its frozen run record reports 129/147 tests passing, 18 failures, OPX2 exit 1, predictions T-147, PC4-4 and RANGE-vs1 false, 15 calls and zero retries. An independent Muse actual-result review and initial diagnosis are now complete. That review attributes all 18 failures to a malformed harness identifier (`vs2_t` equals `c2_hash`); it reports that the production distinctness guard consequently rejects delegation and genesis while piece predicates pass. The proposed single test-only identifier correction has not been run and needs its own fresh Gemini review, resource agreement and root authorization. No author self-review of that repair is acceptance of the repair.

The R2 graph still has the R1 `raw partial capture` size status and two `RAW_PARTIAL_CAPTURE` link qualifications that say the run is still in OPX2 testing and review pending. Those strings are historical and are superseded on run state by R2's completed-failed metadata and the later pinned actual-result review/diagnosis. R2 metadata also says actual-result review and diagnosis are pending; that status is historical now. Consumers must not treat these older strings as current run state. The `sizes.json` and `chain.json` receipts remain raw, unadopted captures from the completed run. All seven applied scripts were under the 14,336-byte working target, but aggregate script bytes increased by 18,615. The hash-chain check passed. Failed positive settlement/genesis cases remain failures; passing hostile rejections do not establish design validity. These captures do not establish transaction size, ExUnits, fees, proof or ledger acceptance.

## Scope and limits

The package's Rust `moriarty/1` → `moriarty-core/1` → `SettlementHashV2` profile remains distinct from the older Source/6 and TypeScript Core/5 profile. `compile` and `inspect` are the documented ordinary workflow. `setup`, `prove` and `verify` are source interfaces only. No genuine current-language setup/key/proof/keyed verification or Moriarty Preprod transaction is established. Package source identity does not identify a running binary.

All 20 UTxO families, ACTUS (277 fixtures, 18 executable types, 32 taxonomy dispositions, DS01–07), DeFi (72 rows, DA01–24, held-outs/intents/regressions), general financial obligations, recursion, private handoff, multi-parent composition and U0–U7/M1–M17/MC/SP/G exits remain open. No compatibility percentage, security judgment or ledger/proof acceptance is made. Native platform token accounting is unavailable and unknown.

## Provenance

The R2 graph SHA is recorded above and in `SOURCE-MANIFEST.json`. The Muse review and five findings, root structural check/relation slice, completed four-way run observation/manifest, Muse diagnosis report, and K4H adoption/review are hash-pinned in `SOURCE-MANIFEST.json` and itemized in `AUDIT-DISPOSITION.md`. The root graph check is structural only; it did not modify the graph and does not approve its semantics.
