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

[Current actual capture summary](actual-r2/SUMMARY.json) records the fresh compile-only command, executable identity, source hashes and all four comparisons. Full exact public outputs are retained as `actual-r2/{small,large,mixed,feepool-positive}/stdout.gz`, with stderr and command/result receipts alongside them. Gzip uses zero mtime and decompresses to the recorded raw output digest. Compressed fixture manifests preserve the exact frozen public inputs; their recorded absolute checkout paths identify this capture. The committed integration test reconstructs portable temporary inputs instead of depending on this runtime directory.

## Source and checks

Nonsecurity implementation and normal regressions: GPT-6.1 Sol high. The aggregate helper, four unit security tests and two CLI security tests were authored by Opus 5.5 high and mechanically integrated by Sol; only rustfmt changed their layout and added one trailing comma. The last three security regressions and the reader-profile documentation were supplied by the [fresh Opus review of the initial candidate](OPUS-REVIEW-69f3258d.md). [Opus's contract and exact code](OPUS-OUTPUT-BOUNDARY-SNAPSHOT.md) retain its scope and conditions. Its initial broad tool-assisted attempt timed out without a verdict; the smaller supplied-source attempt completed with canonical `claude-opus-5-5` identity. Neither attempt is presented as final candidate/result approval.

The executed production file SHA-256 is `4d8bc18746dab1ce89247c7a6e30b9eb376e912c5afe26ce6f367516b7409dc7`; the executed integration test SHA-256 is `74e021ed57129647354f3ec226d5d3acdee314091d9ed77870b67c3b76ac2387`. The captured public executable SHA-256 is `89ae66c4f8911adaedfec690a0965a88baffc3601c5765fab137c37244eff28e`. The capture holds the existing build lock and verifies that the executable and named sources do not change.

- [Behavioral red](red-r3-result.json): one small test passes, three aggregate tests fail against unchanged production. The failure output records successful evaluation followed by `JSON exceeds 1 MiB`.
- [Initial green](green-result.json): all four normal public CLI regressions pass after the repair. [Current CLI checks](review-followup-cli-result.json) pass six tests: the original four plus the two Opus CLI regressions.
- [Current Opus test selection](review-followup-unit-selection.json): exactly four report-test names were listed before execution. [Execution](review-followup-unit-result.json) passes all four; 21 other binary unit tests were filtered out. The initial three-test run remains separately retained.
- [OpenSpec validation](openspec-validation.json): all 14 current packages pass strict validation.
- [Planning retention validation](author-sprint-verify-result.json) and [11 validator tests](author-sprint-test-verify-result.json) pass. Only UNI-024's changed clause hash was refreshed in the requirement index; no IDs, original clauses or acceptance exits were removed.

The first ordinary build reached its 120-second timeout before tests. The next attempt reached a test-only digest `LowerHex` compilation error. Both failed attempts remain in this dossier; neither is counted as a behavioral red. Sol corrected only that formatting incompatibility, preserving the frozen digest, and the subsequent red/green sequence above ran. Builds used the existing shared lock and target, offline locked dependencies, two build jobs and serial selected tests. The later ordinary build bound was 300 seconds; no frozen native resource limit or counter changed.

Shared `mori-core/src/json.rs`, `core_io.rs`, parser/evaluator, signing and native code are unchanged. The manifest retains its existing 256-case limit. Opus's report supplies the output/resource boundary and security interpretation; GPT adds no substitute security verdict. `COMMAND-STARTUP-OBSERVATIONS.json` records the remaining supplied-source observations. In the Opus draft, “7.18 MB” was a predicted normal output size at review time, and `mori-core` does have Midnight dev-dependencies; actual measurements and the normal-versus-development dependency distinction are recorded here and in that observation receipt.

[Scope](scope.json) identifies the complete source change and preserved files. Fresh non-author Muse nonsecurity review and fresh Opus security review of this exact candidate and actual results are separate from authoring and initial command classification. Initial source/result approvals for `69f3258d` are retained as [Muse](MUSE-REVIEW-69f3258d.md) and [Opus](OPUS-REVIEW-69f3258d.md), with [initial scope](scope-69f3258d.json). They do not alone approve the added tests/documentation; fresh applicable final receipts follow the updated candidate freeze. No plugin source/runtime change is part of this repair; plugin 0.1.1 remains the separately reviewed PR51 delivery.

Raw Rust test stdout includes its original final blank lines. `green.stdout.gz`, `opus-unit.stdout.gz` and `red-r3.stdout.gz` preserve those exact bytes losslessly; original stdout digests remain in their result receipts. No captured output was trimmed to satisfy whitespace checks.

## Incorporated review additions

The current six CLI/four unit results include Opus-authored tests for hostile case-name output, a final individually unexportable case with empty stdout, and the retained per-case depth boundary. The public guide now states Opus's reader-profile distinction explicitly. Sol's normal reporting paragraph qualifies its status/exit promises with “When every case report is exportable.” No production runtime logic changed after the initial approved candidate: its complete prefix before `#[cfg(test)]` is byte-identical, as recorded in `AUTHOR-OPUS-ADDITIONS.json`. All four original normal integration tests also remain byte-identical.

The refreshed [capture](actual-r2/SUMMARY.json) binds the updated complete source/test hashes. The compiled public executable remains byte-identical to the initial captured image. [Output equality](final-normal-output-equality.json) confirms exact repeated normal outputs, with no expected-value changes. Repeated captures add provenance, not new financial-family coverage. The initial `actual/` evidence remains unchanged.

The initial Opus memory summary is superseded by F2 and residual R2 in its later review. That is an Opus source analysis, not an observed heap benchmark. Its result review used a supplied-source snapshot and did not recompute repository hashes; Muse's read-only review and root's publication checks record those separate checks. Two positive red tests explicitly print the JSON-size error; the mixed red specifically fails on missing stdout, after exit/count assertions.

## Final source and result reviews

Fresh [Muse nonsecurity review](MUSE-FINAL-REVIEW-R2.md) and [Opus security review](OPUS-FINAL-REVIEW-R2.md) both approve source and recorded results for `95df73558ab4fb0b6d80618a76e88c3a52c38888` within their stated scopes. [Binding and identities](FINAL-REVIEW-BINDING.json) retain the exact seven-file source projection. Publication adds only review/delivery evidence; product source is unchanged.

Opus retains nonblocking N1, D1 and S1-S3 and the documented residuals. `AUTHOR-OPUS-ADDITIONS.json` is a pre-qualifier, pre-execution author receipt; current source hashes and actual executions supersede that temporary state without rewriting it. No missing test, native/target result or parent exit is promoted to a pass.
