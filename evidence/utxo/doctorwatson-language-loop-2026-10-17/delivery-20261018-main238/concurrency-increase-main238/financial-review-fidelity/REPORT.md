# PR236 whole-opinion fidelity inspection

Exact base `8269b5b7ceeec5169c706aa31ea33f390d49c969`, head `360f32ba5b6680788d1c0f1e7b68c2207a877d47`, tree `66a7d788405966afe87f27e0879332652ee11125`. The original 85,336-byte opinion is retained unchanged in `WHOLE-OPINION.json`, SHA256 `47ff40ae342351e121d6d8035293a525bdb187bff81555a937800fe83aa04ea0`.

The mechanical predicate is not fully satisfied: the packet verifier fails on quotation line attribution. All52 literal quotations occur in the delivered files with the quoted hashes;28 have invalid line attribution. Of77 evidence line references,16 extend beyond the named delivered file. All15 changed-source coverage hashes and158 other mechanical checks passed. Supplemental validation also mechanically checked all873 recorded source hashes against exact-head Git blobs, reconstructed all15 historical patch bodies, and confirmed the one-extra-newline delta and unchanged raw original8manifest objects. This does not semantically audit873source files. These counts concern evidence fidelity, not reviewer adoption.

The six-PASS/eight-filtered result, exact360f candidate bindings, byte-oracle harness and report projection facts agree with the public packet. Twelve public CLI invocations are inferred through passing harness assertions; individual child streams were not retained. No result transfers to later main238.

## Narrative mismatches and required qualifications

### N1-manifest-name-conflation

61561517... is packet/SOURCE-PINS.json, the sourceManifestSha256 field. packet/MANIFEST.json hashes to b4f7705e46db191c3484cf8d09baaf950c871fa57871d6cc682a279eb6a6a35a. The top-level review sourceManifestSha256 and receiptManifestSha256 fields are correctly bound; their prose names are conflated.

### N2-driver-try-finally

Post-run clean-head/status/file-pin assertion occurs in try on driver line49. finally begins line50 and removes the root/writes FINAL-RESULT. The source has no unconditional finally recheck. The successful root-reported driver completion and recorded exit0 still corroborate the successful path.

### N3-prose-line-numbers

Base harness has131 lines; unchanged prefix is lines1-131. New six wrappers occupy lines133-161; current complete harness has161 lines. Cargo Finished test ...52.55s is stderr line134, not131. The recorded wall duration52.61422588999994 and PASS counts are correct. Cargo elapsed build duration does not independently measure only compiler time.

### N4-guideline-scope-inference

The quoted guideline says comments explain code and review/authorship/not-yet-run notes go in commit messages, PRs or docs/audits/, not in source. It does not literally ban such metadata from every path under crates/ or explicitly classify Markdown fixture documentation as Rust source. Applying it to FINANCIAL-ORACLES.md is reviewer interpretation. The stale New case outputs are UNRUN sentence itself exists and conflicts with later six-PASS receipts. The proposed docs/audits cleanup remains a recommendation, not an explicit quoted rule covering all crates documentation.

## All quotation-attribution mismatches

All literals below exist in the full pinned delivered file. Locations are inspection facts; the preserved opinion has not been corrected.

| Axis/check/quote | Delivered path | Claimed lines | Literal starts at |
|---|---|---|---|
| sourceChecks/actual-public-consumer/1 | crates/mori/tests/check_source_dispatch.rs | 134–162 | 133 |
| sourceChecks/core1-financial-report/0 | crates/fixtures/check-source/expected/financial-core1-payment.stdout | 11–28 | 10 |
| sourceChecks/core1-financial-report/1 | crates/fixtures/check-source/expected/financial-core1-loan.stdout | 19–41 | 17 |
| sourceChecks/core2-financial-await-report/0 | crates/fixtures/check-source/expected/financial-core2-escrow.stdout | 29–39 | 27 |
| sourceChecks/core2-financial-await-report/1 | crates/fixtures/check-source/expected/financial-core2-rebate.stdout | 51–59 | 48 |
| sourceChecks/exact-candidate-and-format/1 | crates/mori/tests/check_source_dispatch.rs | 159–162 | 158 |
| sourceChecks/guidelines-and-scope/0 | context-excerpts/RUST_GUIDELINES.md.txt | 93–98 | 101 |
| sourceChecks/oracle-exposure-boundary/1 | author-freeze/EXPECTATIONS.json | 89–90 | 147 |
| sourceChecks/original-eight-and-selected-six/0 | crates/mori/tests/check_source_dispatch.rs | 129–137 | 128 |
| sourceChecks/original-eight-and-selected-six/1 | FORMAT-QUALIFICATION.json | 100–102 | 83 |
| sourceChecks/qualification-boundary/0 | crates/fixtures/check-source/FINANCIAL-ORACLES.md | 22–28 | 22 |
| sourceChecks/qualification-boundary/1 | actual-receipts/CHECK-PLAN.json | 64–64 | 65 |
| sourceChecks/whole-byte-oracles/0 | crates/mori/tests/check_source_dispatch.rs | 55–67 | 56 |
| sourceChecks/whole-byte-oracles/1 | context-excerpts/crates/mori/src/commands/core_io.rs.txt | 23–27 | 25 |
| resultChecks/candidate-and-source-bindings/0 | actual-driver.py | 13–15 | 14 |
| resultChecks/candidate-and-source-bindings/1 | actual-driver.py | 47–47 | 49 |
| resultChecks/financial-beta-invoice/0 | crates/mori/tests/check_source_dispatch.rs | 134–137 | 133 |
| resultChecks/financial-beta-repayment/0 | crates/mori/tests/check_source_dispatch.rs | 139–142 | 138 |
| resultChecks/financial-core1-loan/0 | crates/mori/tests/check_source_dispatch.rs | 149–152 | 148 |
| resultChecks/financial-core1-payment/0 | crates/mori/tests/check_source_dispatch.rs | 144–147 | 143 |
| resultChecks/financial-core2-escrow/0 | crates/mori/tests/check_source_dispatch.rs | 154–157 | 153 |
| resultChecks/financial-core2-rebate/0 | crates/mori/tests/check_source_dispatch.rs | 159–162 | 158 |
| resultChecks/format-qualified-exact-result/1 | actual-receipts/FINAL-RESULT.json | 18–18 | 19 |
| resultChecks/raw-output-boundary/1 | crates/mori/tests/check_source_dispatch.rs | 56–60 | 57 |
| resultChecks/terminal-and-containment/1 | actual-receipts/FINAL-RESULT.json | 15–17 | 16 |
| findings/F1-ORACLE-DOC-UNRUN-AND-PROVENANCE-NOTE-IN-CRATES-FIXTURES/1 | context-excerpts/RUST_GUIDELINES.md.txt | 47–49 | 58 |
| findings/F2-AUTHOR-EXPOSURE-AND-CHECK-REPORT-PROJECTION-BOUNDARY/1 | crates/mori/src/commands/check_source.rs | 69–71 | 70 |
| findings/F3-SUBPROCESS-OUTPUT-RETENTION-AND-FILTERED-BASELINE-BOUNDARY/1 | crates/mori/tests/check_source_dispatch.rs | 62–72 | 63 |

## All out-of-bounds evidence references

| Axis/check | Original reference | Delivered line count |
|---|---|---|
| sourceChecks/actual-public-consumer | crates/mori/tests/check_source_dispatch.rs:L134-L162 | 161 |
| sourceChecks/core1-financial-report | crates/fixtures/check-source/expected/financial-core1-payment.stdout:L1-L36 | 35 |
| sourceChecks/core1-financial-report | crates/fixtures/check-source/expected/financial-core1-loan.stdout:L1-L49 | 47 |
| sourceChecks/core2-financial-await-report | crates/fixtures/check-source/expected/financial-core2-escrow.stdout:L1-L66 | 62 |
| sourceChecks/core2-financial-await-report | crates/fixtures/check-source/expected/financial-core2-rebate.stdout:L1-L60 | 57 |
| sourceChecks/exact-candidate-and-format | FORMAT-QUALIFICATION.json:L1-L103 | 86 |
| sourceChecks/exact-candidate-and-format | crates/mori/tests/check_source_dispatch.rs:L130-L162 | 161 |
| sourceChecks/exact-candidate-and-format | base/crates/mori/tests/check_source_dispatch.rs:L125-L132 | 131 |
| sourceChecks/original-eight-and-selected-six | crates/mori/tests/check_source_dispatch.rs:L1-L162 | 161 |
| sourceChecks/original-eight-and-selected-six | base/crates/mori/tests/check_source_dispatch.rs:L1-L132 | 131 |
| sourceChecks/qualification-boundary | crates/fixtures/check-source/FINANCIAL-ORACLES.md:L22-L28 | 27 |
| resultChecks/financial-core1-loan | crates/fixtures/check-source/expected/financial-core1-loan.stdout:L1-L49 | 47 |
| resultChecks/financial-core1-payment | crates/fixtures/check-source/expected/financial-core1-payment.stdout:L1-L36 | 35 |
| resultChecks/financial-core2-escrow | crates/fixtures/check-source/expected/financial-core2-escrow.stdout:L1-L66 | 62 |
| resultChecks/financial-core2-rebate | crates/fixtures/check-source/expected/financial-core2-rebate.stdout:L1-L60 | 57 |
| resultChecks/financial-core2-rebate | crates/mori/tests/check_source_dispatch.rs:L159-L162 | 161 |

`MECHANICAL-RESULT.json` lists every checked hash/path/quote and failure; `CLAIM-FIDELITY.json` covers all10source/11result assessments and4original findings with literal narrative claims and exact backing anchors. `EVIDENCE-REFERENCE-BOUNDS.json` records all77line references.

Startup used the installed Moriarty develop skill, checkout AGENTS/product contract and inspected status code. Status was loaded with bytecode disabled, SQLite forced to read-only mode, and admin-interval recording disabled in memory because the stock status code otherwise writes shared state. STARTUP-STATUS.json retains observed stale-binding/accounting blockers as operational history; none are reset.

No provider calls, product tests/builds, native/SDK work, Git mutations or shared writes occurred. Only this assigned output directory was written. Public terminal export and packet files were read; private captures/auth/cache were not accessed. The review self-reported requested Gemini4ArgonHigh/HIGH and an unattested backend identity; this inspection adds no identity attestation, security judgment, parent closure or adoption decision.
