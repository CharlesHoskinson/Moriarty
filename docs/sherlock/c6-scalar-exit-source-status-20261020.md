# C6 ScalarExit source status — 2026-10-20

This source adds the full C6 `BHelperScalarExit` emitter and its host-pair, owner-record, recorder, statement-builder and SDK preprocessing-checker APIs. The staged branches are U128, Bool, Leaf, Cons, Branch, Selector and TypeTest. The existing physical frame/control representation and retained config/admission-bound interfaces remain in use.

The source includes 35 unrun tests: 19 admitted-owner test sources and 16 raw statement test sources. The raw tests include ranges, disabled gates, derived ends, namespace composition, canonicality and preservation mutations, counter/refusal cases, both statement modes, and recomputed communications commitments. Source fixtures are test inputs and assertions; they are not captured runtime results.

All implementation and security-test changes in this addition were supplied literally by Muse Wonder 1.0 at high effort under the recorded Opus-outage fallback. Independent reviews approved the bounded source/API and the latest staged-companion test source. The earlier full review B abstained on production/API/runtime qualification and the inherited semantic bridges; that dissent remains part of the record. This is not a new full-leaf approval.

Five staged-domain cases now have derivation-consistent post companions. Selector and TypeTest coverage isolates the same coupled pre/post payload-domain family; it does not establish pre-only canonicality isolation. The old-DATA and disabled-PendingNode raw fixtures are not owner-admitted states.

No Rust typecheck, compilation, SDK preprocessing run, test execution, constraint synthesis, MockProver run, proof, native execution or Midnight stagenet qualification is established by this source addition. All 35 tests remain **UNRUN**.

SI08 site/frame denotation, SI21 staged arithmetic/selector/type/pending denotation, SI22 register meaning, StepInv cross-section relations, program membership and native P1 issuance remain open. The remaining 39-selector/refinement work, required witnesses/constraints/exports, grouping 1/2/4/8, residual and financial obligations, persistence, recursion and native/stagenet qualification remain open.

Source-review receipts (SHA-256): full A `436ba0008c899410f14be6cbb83f0c24f7c6df3b1a93e3c8dff7a40bb5268a57`; full B `87c2311232dd231f3fa38ff0000685bb975a5be481925da93acdfed937f8b8a1`; raw V9 `8d444177b352ae797fc1739caac60f4ed115ad7dc763c03e50b45577513fa787`; staged C10 `6351aca92462ff72159ced4e10b77b5a91f72dacc43abce874dd283c37a3565f`.
