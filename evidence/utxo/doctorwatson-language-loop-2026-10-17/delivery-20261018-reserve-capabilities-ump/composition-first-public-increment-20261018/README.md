# Smallest existing public composition increment

Source-only, unapplied candidate against exact main `e4d7a2b5709caa18a94f5256d03a0302364b8235`, tree `b44043fba64fa2bd396d18412adb3472434c70ac`. Root integrates. No product/runtime/build/model/SDK/proof/provider execution, shared writes or Git mutation occurred.

The next increment is one ordinary CLI regression in the existing feature-gated `crates/mori/tests/disjoint_test_public.rs`. It exercises a wrong expected observation through actual `mori test` → disjoint consumer → existing Pair. The current successful two-schedule test does not exercise the false comparison/failed aggregate path. The older `stage_test_public` has the analogous ordinary expectation-transcription test for its different `/2` manifest, which does not cover this disjoint branch.

The new test changes only a temporary copy of `left-first-2.json`: expected `/post/state/marker/value` 77 becomes78. It changes no program, scenario, event, continuation or original fixture. The public command must return exit1 and `1 passed; 1 failed`, retain every original actual observation, and continue the two later first-case stages and all second-case stages. The entire failed aggregate JSON and bytes are compared, then all ten complete Pair reports are reconstructed from case-local prefix tables and compared to the original full goldens. No expected financial value is derived from a producer run.

- [Exact one-file patch](candidate.patch), 62 added lines; no Cargo/profile/API change.
- [Full after file](after/crates/mori/tests/disjoint_test_public.rs).
- [Expected-data derivation and custody pins](EXPECTATION-DERIVATION.json).
- [Complete derived failed aggregate](DERIVED-FAILED-PUBLIC-REPORT.json), 839,112 bytes. This changes only three wrapper status/boolean fields from the accepted frozen aggregate; it is not new independent financial custody.
- [Financial/frame/work review view](FINANCIAL-EXPECTATION-VIEW.json); full original reports, source and inputs remain in `before/`.
- [Actual source/line/hash matrix](SOURCE-MATRIX.md), [exact before pins](BEFORE-PINS.json).
- [Separate unadopted shared-state boundary](NEXT-SHARED-STATE-BOUNDARY.md).

Falsifiable predicate: a single public CLI invocation whose expected frame contains this ordinary transcription error returns failure while its complete actual financial histories, later results and second case remain exactly the independently frozen baseline. No runtime pass is claimed. Formatting, compilation and selected test execution are UNRUN.

Oracle qualifications are inherited unchanged: finance/work were independently source-derived with disclosed related-source exposure and a narrow finite Core premise; full report wire is conditional on the authored codec, not an independent compiler-wire proof. Reusing those immutable original observations adds no new financial scenario or operator coverage. This regression observes expected-test comparison/aggregation behavior, not a staged execution rejection, authority, authentication, hostile input or security property.

Sequential full-byte escrow/partial/multi-condition tests and the successful disjoint test already exist in this main. The patch does not duplicate their successful predicates. SI1–SI4 from the separate shared-state design remain unadopted; no shared-head semantics are implemented here. All five full composition operators, U0–U7, M1–M17, MC/SP/G/R7 and native/proof/ledger/resource/target/parent obligations remain open with their existing owners and counts.
