# Reserve side of the two-program financial regression

UNAPPLIED / UNRUN. Pinned current main `71c22f8a477231be869786eb42f04e3b50d1b3a7`, tree `6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4`.

`candidate.patch` adds eight absent paths: the existing corrected `core_reserve_spend_refund` test and original source/scenario/expectation/custody fixtures. It changes no production code or expectations. The full candidate source equals the frozen failure-shape increment after-file, SHA256 `27c502d178d7422b2c53f03e1b294cdb792dfc719aac1e772ca2cf405b514b90`. The current tree does not contain this test/fixture namespace, so an old one-line modification patch cannot apply directly; restoring the full corrected regression is the necessary delta.

The former adapter error remains corrected: every cumulative observation copies the actual Step.index and its complete ordered effect prefix into `{index,effects}`. The original flat production Execution.effects and per-Step effect deltas remain unchanged. No expected index or effect is stripped or regenerated. Both real library execution and public CLI simulation compare every original typed value, full frame/delta, source kind and indexed cumulative prefix. Prior-source/ and CANDIDATE-COMPARISON.json retain the original author candidate, failed root-formatted source and corrected increment; the old exit101 remains immutable.

Among the inspected production parser/runtime/IR/JSON/CLI/Cargo paths, only parser.rs changed relative to failed commit de1eaf81: language-profile diagnostic text now names both supported versions. The ordinary valid parser body, financial evaluator, DTO, pretty JSON and public simulate source paths otherwise have identical Git blobs. SOURCE-PINS.json and full current-source/ bodies bind these observations. This is source compatibility evidence, not a new runtime result.

All original oracle bytes are unchanged. Source SHA256 `6a816fd9bff3a3f5a0dac9f5391dfeae88d1abbf045a8c4d0ffbce2c05d4da69`; financial execution projection SHA256 `ee754f563ce059663ad6ecdd2595845c37938d0525d56c72367a4f550add8426`; cumulative records SHA256 `649a025c2dbb6319c3d2df4d82d56f8e0d799878b93e38f3ce7503267488be5c`. All 20 records and 17 ordered effects are retained exactly. ORIGINAL-FREEZE-UNCHANGED.json verifies the original input freeze and all 25 whole independent-oracle payloads, including terminal freeze `1b606141dff7ca162bcd132d55d62014187de4f936cfc6528c23c8d22591aefb`.

A data-only comparison over the retained old CLI capture reconfirms all 20 indexed records. It supplies no current-main execution or passing-test claim. Payout is the separate counterpart of the former two-program batch; its source is not duplicated by this lane.

After root integration/audit, the bounded ordinary check is:

```sh
MORI_CORE_REFUND_OBSERVATIONS=/absolute/fresh/main238-reserve-capture cargo test -p mori --no-default-features --locked --test core_reserve_spend_refund -- --nocapture
```

The parent capture directory must exist. This invokes the real library and public CLI through the existing tests. Root owns execution and any new receipt. The payout target requires its separately integrated source before a combined two-program batch can run.

Author validation is offline source/JSON/hash/patch only. No compiler/test/Node/provider/SDK/native/security execution or shared/Git write occurred. Application gross counters remain separate from refund; allowance enforcement, Program/statement ExprIds, logical work, public byte goldens, Target/Artifact/native and original U/M/MC/SP/G/finance/composition/release exits remain open. No parent predicate is narrowed.
