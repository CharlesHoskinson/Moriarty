# Four-piece Cardano experiment evidence — 2026-10-16

The [root adoption](root/ACTUAL-ADOPTION.json) adopts exactly the finite local measurement closed by the fresh independent [Opus actual-result review](reviews/actual-opus/REPORT.md): M2 compiled, **147/147 frozen expected unit outcomes passed**, the tool-derived seven-script parameter/hash chain was consistent, and every applied script was within the 14,336 B working target. All 15 actual steps exited zero, all 16 root operator-integrity checks matched, and [OPX7f.exit](run/OPX7f.exit) was zero. This closes the recorded final-hygiene condition. It establishes no proof, compiled-wrapper execution or ledger qualification.

The [Aiken package](../../../experiments/cardano-four-piece-settlement/README.md) is separate from the [Rust experiment](../../../experiments/moriarty-cardano-rust/README.md) published at `515bd5c5`; the first Rust setup source was published at `82c20ad4`. Current-Rust proof-to-these-validators integration remains unproven. There is no current-Rust proof, validated proving-key decode, independent keyed verification or Preprod transaction here. Full Moriarty UTxO support and end-to-end integration remain open.

## Reading the records

| Record | Meaning |
| --- | --- |
| [ACTUAL-ADOPTION](root/ACTUAL-ADOPTION.json), SHA-256 `a6a9b2ec8c6551d2b0a142aecf55bbdec765abc1542580e4e47d2f576cb49e14` | Current root acceptance of the narrow completed result; no new authority |
| [Actual Opus review](reviews/actual-opus/REPORT.md), SHA-256 `437c7e7f20b21c54fe951d2a10049a525c7ddef0840d7138e5fa55b372b33c7c` | Independent actual-result verdict `ACTUAL_PREDICATE_CLOSED`, with limitations and carried findings |
| [RESULT-MANIFEST](root/RESULT-MANIFEST.json) and [OPERATOR-RECEIPT](root/OPERATOR-RECEIPT.json) | Pins for 114 public result files; actual step exits, timing and integrity checks |
| [Comparison](run/receipts/OPX2-compare.json), [test rows](run/receipts/OPX2-tests.tsv), [sizes](run/receipts/sizes.json), [outcome](run/receipts/outcome.json) | Actual observations against the frozen [EXPECTATIONS](candidate-docs/EXPECTATIONS.json) |
| [Source Opus review](reviews/source-opus/REPORT.md), SHA-256 `ee130e2a41aeb52b9d4036db3af39a512105857a2863f5a641ec0ff24d1eecce` | Earlier source/effect readiness for one bounded run; it was not an actual-result review or authorization |
| [AUTHORIZATION](root/AUTHORIZATION.json), [operator source](operator-source/REPORT.md), [Muse operator review](reviews/operator-muse/REPORT.md) | Scope and controls of the completed maintainer experiment; source review scope stays explicit |

The current actual review and adoption resolve only their stated predicate. [RAW-OBSERVATION](root/RAW-OBSERVATION.json) still says independent review is pending; `outcome.json` predates OPX7f and records 415 charged seconds; PRE-LAUNCH says its target was absent. Those immutable snapshots describe their recording times. The dated adoption and terminal receipt supply the later result; no historical field was rewritten.

The tests comprise 125 `shard4_test` cases with nine `fail` annotations and 22 cost benches. All 147 are unit tests; property count is zero. A passing annotated case means expected refusal, not an ignored failure, and refusal alone need not identify a failed conjunct. The 472 retained parent tests were not rerun. The real source partition is sixteen VS components split **6/3/4/3**, plus three VI components, as described by the source review and package README. There are no sixteen catalogue claim IDs or `piece_1` functions in this design.

`allCommandsExit0 = true` and `allPredictionsHeld = false` are both preserved. The sole remaining prediction miss is informative **RANGE-vs1 = FALSE**: 6,859 B is below the frozen 7,000–10,500 B range. No all-green prediction claim is made.

Applied sizes are C2 2,770; VI 12,784; VS1 6,859; VS2 9,940; VS3 10,832; VS4 10,825; S2′ 13,018 bytes. The pieces total **38,456 B**; all seven total **67,028 B**, versus the prior four-script **48,413 B** (**+18,615 B**). Each piece is smaller than monolithic VS at 20,503 B. P01 per-piece host-CEK costs are lower, while aggregate piece costs are higher for P01 and P07. These are library-predicate costs through `test_ctx` with approximate decode subtraction. They are not compiled-wrapper costs or ledger ExUnits. The 14,336 B threshold is a working per-script target, not a qualified protocol parameter, transaction fit, fees, budget fit or deployability conclusion. No VI signatures or C2 Groth16 pairing ran; independent script-hash verification (M-HASH) is unperformed.

## Preserved history

The [first four-piece outcome](history/first-four-way/outcome.json), [test rows](history/first-four-way/OPX2-tests.tsv), [log](history/first-four-way/OPX2.log) and [manifest](history/first-four-way/RESULT-MANIFEST.json) retain **129/147 passed, 18 FAIL**, with T-147, PC4-4 and RANGE-vs1 false. The source Opus review traced the fixture alias: the test-only VS2 identifier `c2×28` collided with the C2 fixture identifier. The [exact repair](root/EXACT-HARNESS-DIFF.patch) changes only that test constant to `d2×28`. Production guards, assertions and frozen expectations/gold remain unchanged; the actual review records unchanged compiled validators. The old refused-path S2′ cost is not an accepting-settlement cost.

The [Gemini report](reviews/unadopted-gemini/REPORT.md) `c611776c` is **UNADOPTED**. The [root discrepancies](root/GEMINI-FULL-REVIEW-DISCREPANCIES.json) and source Opus review record wrong commands, function descriptions and sizes despite matching clause hashes. Its presence preserves review history; it supplies no accepted judgment. The later Opus result supersedes historical pending or contradictory status only within the adopted scope.

The current run took **328.423 s actual wall** and **416 charged seconds** against its 1,800-second cap. Prior lineages remain 43 calls/1,470 charged seconds (14/90 parser-failed; 14/934 prior-layout measurement; 15/446 first four-piece failure). Cumulative records are **58 calls/1,886 charged seconds**. No prior charge or cap was reset.

## Limits and archive contents

The adopted Opus conditions remain: **FV-01**, bind the exact `C2 → VI(c2) → VS3(c2, vi) → S2′(c2, vi, vs1..4)` parameter DAG; **FV-02**, shared unparameterized VS1/VS2/VS4 restrict atomic settlement across S2′ deployments. The actual review also carries AR-1: journal integrity rests on the root result manifest, not the operator's final freeze. AR-2 preserves stale non-gating identifier text; AR-3 preserves historical pending/absent labels. Opaque fixture literals and compiled code were not independently decoded or cryptographically verified by that review.

The first unproved boundary is execution of the compiled applied B9 wrappers with genuine hashes on ledger-shaped contexts (X1/PX04, M-EU), alongside independent hash recomputation, then FX1 signatures, actual proofs, Conway transaction size/fees/ExUnits, registration and real Preprod. New K6 concrete-operator work is source-only. These documents authorize no operation.

All 20 UTxO families; ACTUS 277/18/32/DS01–07; DeFi 72/DA01–24, held-outs/intents/regressions; general financial obligations including gross debit, fees, net receipts, liabilities and residual duties; real recursion/PCD; private handoff; bounded multi-parent composition; universal certificates; and real Preprod remain open. Midnight U0–U7, M1–M17 and every MC/SP/G exit remain distinct. There is no compatibility percentage or external developer clean-clone validation.

[INVENTORY](INVENTORY.md) maps all **240 selected payloads / 5,349,415 bytes**: 33 package files and 207 evidence files. It records each exact path, byte count and SHA-256. The [selection control](SOURCE-PUBLICATION-SELECTION.json) is outside that count. The bundle includes public source, synthetic/caller-declared unit fixtures, compiled blueprint data and receipts; it includes no runtime binary, proving key, private witness, wallet material or provider raw capture. These fixtures authenticate no real chain.

Archived scripts and absolute paths are provenance for a spent maintainer experiment, not build dependencies, portable tools or retry instructions. Use the package README's normal Aiken commands for local reproduction. Maintainer reviews and resource controls never become public developer permission requirements. Publication itself supplies no new install/rebuild, proof, ledger or Preprod evidence.
