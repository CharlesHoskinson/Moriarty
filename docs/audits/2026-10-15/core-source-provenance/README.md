# Mapped-role compiler source provenance

Status: implemented and ten exact selected tests passed; fresh full-candidate Muse and Opus audits are pending. Base is merged PR52 `eb2577d6c67684b950288d769ea2fcb91e731bf1`. [Scope](scope.json) records source hashes, profiles and limitations. This is a compiler metadata capability, not native or semantic certification.

The existing public mapped-role `compile-call --all-actions` output now includes `statement_emissions`: each source statement/root expression, its half-open local instruction/group ranges, and actual template-replacement field coordinates. The existing public reopen consumer recompiles and compares complete leaves. No general compile command or grammar arm was missing in this repair. Generated IR stays byte-identical for the independently specified five-step specimen.

The mapped relation leaf format is `/2`; bundle profile `/1` and qualification strings remain unchanged. Opus owns [compatibility and integrity interpretation](OPUS-SOURCE-ADDITIONS-DETAIL.md). Source-closure hashes change, including ordinary/signed manifest bytes; old bundles require re-export and native requalification remains separate and unperformed.

## Actual verification

[Verified complete result summary](ACTUAL-CHECK-SUMMARY.json). Full stdout/stderr are lossless gzip captures; each directory retains exact argv, source fingerprints and exit result.

| Capture | Selected tests passed | Observed predicate |
|---|---:|---|
| [Public CLI](actual/normal-cli-01/result.json) | 1 | Export/cold reopen; full expected metadata; direct IR byte equality with pinned prechange executable |
| [Decoded IR](actual/normal-ir-01/result.json) | 1 | 58 Impact indices/widths, 52 frozen guards, 13 replacement operands and all five statement rows |
| [Opus public regressions](actual/security-cli-01/result.json) | 3 | H2/H3/H4 test functions passed; security interpretation reserved for fresh Opus review |
| [Opus validator units](actual/security-unit-01/result.json) | 5 | Exactly five selected test functions passed; 84 other ledger units were filtered out |

No failed product check or retry occurred. Source fingerprints and baseline executable SHA stayed unchanged. The [normal oracle](ORACLE-DESIGN.md) and [Opus expectations](OPUS-PUBLIC-EXPECTATIONS.json) were frozen before execution; none were tuned to results. Financial amounts in the oracle remain predictions, not observed execution. The five-step specimen is not rule-level coverage of the full grammar. Temporary exported leaves were deleted by test fixtures; individual IR/bundle digests were not separately persisted. The [post-check binary observation](POST-CHECK-BINARY.json) is separate from the pinned [baseline](BASELINE-BINARY.json).

## Review history and limits

Sol authored nonsecurity code, tests and developer documentation; Opus authored the validator, security requirements and test source. [Bounded execution classification](OPUS-COMMAND-BOUNDED.md) approved only the four exact ordinary commands, with unchanged-source review premises and all residuals explicit. It is not source/result/security approval.

The H2-H4 source-writing job timed out after writing code and frozen expectations, without a final report or returned canonical identity. Its requested model was Opus5.5. The later large full-source command review also timed out without a verdict. Their terminal receipts are preserved verbatim. The smaller bounded review completed with canonical Opus5.5 high. Original design mistakes, corrections, failed attempts and reviewer limitations remain in their original records. No timeout, requested identity or green test substitutes for the fresh required audits.

All U0-U7, M1-M17 and MC/SP/G exits remain open. Native authority is zero, gross4334/remaining0 is unchanged, and no Fenrir identity or target tuple is invented. Metadata does not establish dependencies, activity, value equivalence, kind/payer enforcement, debit custody, authorization, native execution, ledger effects or full security certification.
