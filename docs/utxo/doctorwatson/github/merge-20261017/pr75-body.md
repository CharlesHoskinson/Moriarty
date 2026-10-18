This adds public `mori test` support for `moriarty-tests/2` manifests using local Core/2 load, start and resume. The harness compares complete reports. Source fixtures cover two deliveries, a condition that does not advance, and a captured fee calculation.

Earlier fixtures omitted incoming-liability identities. Their blocked inputs, expectations and review dispositions remain preserved as invalidated historical evidence. The revised candidate registers those identities and binds six independently authored reports plus whole-program and plan preimages.

The guide and provenance now point to the current header-correction R3 binding. Source inspection confirms that the selected manifest names the active cases explicitly; the CLI consumer does not recursively discover historical fixture directories. External wrappers and actual runtime behavior have not been validated.

Validation: the recorded independent source reviews approve only their exact source and draft updates. The two-document correction was reviewed and pushed as `26ec28afb31be46ad01cf81f90a9404098b2d6d5`. All Rust/Cargo builds, CLI tests and runtime comparisons remain UNRUN for this cleanup. The user requested merge and fresh ordinary plus Opus 4.8 reviews approve source-only integration. Source review does not establish financial, proof, Cardano or Midnight ledger acceptance.

Tracked under [doctorwatson<UTXO> / doctormoriarty<utxo>](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/ROADMAP-AUDIT.md), with [the intake and remaining checks](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/work-items.json). Before readiness or merge, validate the exact workspace/consumer under the applicable execution boundary and obtain independent actual-result review. Native authority remains zero; the broader Preprod goal remains paused.

The integrated candidate moves the staged-test module into the conventional `commands/test/stage_test.rs` location. Its implementation body and all 90 fixture files are unchanged. The combined source tree is `733c981de96ad6ac280a036355bd81be4bbdd1a3`; Git index reconstruction confirms its identity.

Source-only merge disposition: ordinary Sherlock reporting QA passed 230 tests and plugin guidance passed 10. Cargo/Rust compilation and runtime tests, SDK/proof checks and ledger execution remain UNRUN. No native resource counter or EARS/product acceptance is reset. The optional SDK-linked Cargo test was not admitted by the security reviewer.
