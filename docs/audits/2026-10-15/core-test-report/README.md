# Core CLI aggregate test report repair — 2026-10-15

This repairs the Rust `mori test DIRECTORY` report, a local U7/M10/MC08 child capability. Valid suites previously evaluated all cases and then lost their entire JSON report when the aggregate exceeded the shared single-document limit. The command now exports each complete case with the unchanged exporter and assembles the complete report. Evaluation, comparison, case order, status and exit selection are unchanged.

This is current local reference and tooling evidence. It does not close M10, MC08, U7, any other U milestone, native correspondence, financial-library conformance or target qualification. Native authority remains zero; no SDK relation/check/degree/model, VM, key generation, proof, native verification or ledger action ran.

## Observed behavior

| Frozen suite | Actual cases | Actual stdout bytes | Exit and comparison |
| --- | ---: | ---: | --- |
| Small compatibility | 1 | 28,093 | 0; byte-identical to the independent legacy-format expectation |
| Large positive | 256 | 7,181,098 | 0; every full execution equals the independent expectation |
| Large mixed expectations | 256 | 7,181,101 | 1; only indices 0, 127 and 255 fail comparison; every actual execution remains present |
| FeePool accepted aggregate | 87 | 31,333,417 | 0; original order, complete frozen poststates and ordered effects match |

The manual normal oracle retains alice75, bob25, sixteen untouched observer balances42, an untouched duty11, both execution steps and the one Transfer effect. Each repeated case starts from its own identical input; repetitions do not add financial-family coverage. FeePool selects the 87 accepted cases from the original 162-case corpus. The original 75 rejected cases remain in the frozen membership inventory; they are not newly executed by this positive-only capture. FeePool trace presence is checked; the normal fixture checks every trace field independently.

[Independent expectations](INDEPENDENT-EXPECTATIONS.md) were frozen before product execution. Their JSON SHA-256 is `3911ef8990bbcdf067f641bcae032f48828b2035ddae55f8687969ecc0cb2395`. Expectations were not imported from the evaluator/compiler or tuned to output. Full independent normal expected stdout is retained in the three `independent-*-expected-stdout.json.gz` files. The original source-only check predates formatting and the test digest-format compatibility correction; current executed source identities are below.

[Actual capture summary](actual/SUMMARY.json) records the fresh compile-only command, executable identity, source hashes and all four comparisons. Full exact public outputs are retained as `actual/{small,large,mixed,feepool-positive}/stdout.gz`, with stderr and command/result receipts alongside them. Gzip uses zero mtime and decompresses to the recorded raw output digest. Compressed fixture manifests preserve the exact frozen public inputs; their recorded absolute checkout paths identify this capture. The committed integration test reconstructs portable temporary inputs instead of depending on this runtime directory.

## Source and checks

Nonsecurity implementation and normal regressions: GPT-6.1 Sol high. The aggregate helper and three security tests were authored by Opus 5.5 high and mechanically integrated by Sol; only rustfmt changed their layout and added one trailing comma. [Opus's contract and exact code](OPUS-OUTPUT-BOUNDARY-SNAPSHOT.md) retain its scope and conditions. Its initial broad tool-assisted attempt timed out without a verdict; the smaller supplied-source attempt completed with canonical `claude-opus-5-5` identity. Neither attempt is presented as final candidate/result approval.

The executed production file SHA-256 is `e3b1668ac433debb19a3de0f55ad75556a36d5590abd1179f41e3c889ceb1fd0`; the executed integration test SHA-256 is `ed6c46c3967ebd3562b13fcba4d2c9cde0e11384f1322edb7cd822e884ef914a`. The captured public executable SHA-256 is `89ae66c4f8911adaedfec690a0965a88baffc3601c5765fab137c37244eff28e`. The capture holds the existing build lock and verifies that the executable and named sources do not change.

- [Behavioral red](red-r3-result.json): one small test passes, three aggregate tests fail against unchanged production. The failure output records successful evaluation followed by `JSON exceeds 1 MiB`.
- [Green](green-result.json): all four public CLI regressions pass after the repair.
- [Opus test selection](opus-unit-selection.json): exactly three report-test names were listed before execution. [Execution](opus-unit-result.json) passes all three; 21 other binary unit tests were filtered out.
- [OpenSpec validation](openspec-validation.json): all 14 current packages pass strict validation.
- [Planning retention validation](author-sprint-verify-result.json) and [11 validator tests](author-sprint-test-verify-result.json) pass. Only UNI-024's changed clause hash was refreshed in the requirement index; no IDs, original clauses or acceptance exits were removed.

The first ordinary build reached its 120-second timeout before tests. The next attempt reached a test-only digest `LowerHex` compilation error. Both failed attempts remain in this dossier; neither is counted as a behavioral red. Sol corrected only that formatting incompatibility, preserving the frozen digest, and the subsequent red/green sequence above ran. Builds used the existing shared lock and target, offline locked dependencies, two build jobs and serial selected tests. The later ordinary build bound was 300 seconds; no frozen native resource limit or counter changed.

Shared `mori-core/src/json.rs`, `core_io.rs`, parser/evaluator, signing and native code are unchanged. The manifest retains its existing 256-case limit. Opus's report supplies the output/resource boundary and security interpretation; GPT adds no substitute security verdict. `COMMAND-STARTUP-OBSERVATIONS.json` records the remaining supplied-source observations. In the Opus draft, “7.18 MB” was a predicted normal output size at review time, and `mori-core` does have Midnight dev-dependencies; actual measurements and the normal-versus-development dependency distinction are recorded here and in that observation receipt.

[Scope](scope.json) identifies the complete source change and preserved files. Fresh non-author Muse nonsecurity review and fresh Opus security review of this exact candidate and actual results are separate from authoring and initial command classification. Their final receipts are added after candidate freeze. No plugin source/runtime change is part of this repair; plugin 0.1.1 remains the separately reviewed PR51 delivery.
