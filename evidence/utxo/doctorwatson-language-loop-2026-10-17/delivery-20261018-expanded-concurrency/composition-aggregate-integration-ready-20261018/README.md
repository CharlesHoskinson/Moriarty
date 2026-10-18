# Integration-ready ordinary composition CLI regression

Candidate reconstructed against K `823ce081c00786a0c55b92af81009f9e7c5e6a16`, tree `aa593783a8717ac838ae556e24c187997ab3547e`, with E4D main `e4d7a2b5709caa18a94f5256d03a0302364b8235` as its context. K was clean at inspection. The original composition-first-public-increment-20261018 packet remains immutable. This new packet contains the full current CLI/local runtime source, manifests/lock/toolchain and original fixture bodies, not merely old path references.

The exact one-file62-line [patch](candidate.patch) and [complete after source](after/crates/mori/tests/disjoint_test_public.rs) are byte-identical to the preserved candidate. The current target file is also byte-identical to its original E4D baseline. No production, manifest, feature, fixture, local composition semantics, authentication or SI1–SI4 change is proposed.

Predicate: one ordinary wrong expected marker value makes actual public `mori test` return failure while retaining the complete aggregate, later results, second case and all ten complete original actual Pair reports. The full financial expectations remain frozen original data. Only three literal wrapper status/boolean fields change in the expected aggregate. [The existing intended contract and complete derivation](INTENDED-CONTRACT.md) establish these expected values without a producer run.

Root's minimal selected command, **UNRUN here**, from K after applying the candidate:

```sh
cargo test --frozen -j1 -p mori --no-default-features --features core2-disjoint-draft --test disjoint_test_public ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports -- --exact --test-threads=1
```

[COMMAND.json](COMMAND.json) pins its argv and expected layers: outer Cargo exit0; one Rust test passes, zero fail and the existing successful test is filtered out; inner CLI exits1 with one case passed/one failed and a complete839112byte stdout aggregate. This command does not rerun the existing successful test function. Root uses its existing ordinary execution/environment/cap/cleanup controls; this packet creates no execution allocation, retry or waiver.

Read [RUNTIME-PATH.md](RUNTIME-PATH.md) and [source entry/line/hash matrix](SOURCE-CALL-MATRIX.md) for main/clap dispatch, manifest/preloading, Pair, stage admission, parser/lexer, scalar statements, logical meter, original envelopes, prefix tables, bounded exporter, stderr status and process exit. [FEATURE-SELECTION.json](FEATURE-SELECTION.json) records the static nondefault-only feature selection. Third-party/std/build-script/proc-macro implementations are not newly audited, and no compiled image or runtime behavior is asserted.

Full [current90 source pins](CURRENT-SOURCE-PINS.json), [E4D→K823 diff](E4D-to-K823.diff), original complete reports and the full derived failed aggregate are included. Only three pinned context source files changed since E4D: check.rs, check_source.rs and commands/mod.rs; the new source-check adapter is not called by Command::Test. The exact context relationship and filesystem/source-copy checks are in INTEGRATION.json. Later real changes require a finite source judgment; no future-base blanket is supplied.

Static source/data verification reconstructs the exact patch, preserves the original packet, matches all original financial goldens to custody, checks all ten lossless whole-report expansions, and verifies only three expected-wrapper changes. Expected per-case JSON values fit the unchanged bounds. These checks do not execute Rust, the CLI, an exporter, a semantic model, SDK/proof/native paths or providers. Formatting, compilation and the selected test remain UNRUN. Root owns integration and actual checks.

All inherited oracle exposure/codec qualifications, caps, counters and all five full operator/parent obligations remain unchanged/open. The separate shared-state production boundary remains in NEXT-SHARED-STATE-BOUNDARY.md; SI1–SI4 are still unadopted, and this regression cannot implement or accept them by implication.
