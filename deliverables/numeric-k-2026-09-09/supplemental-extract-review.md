# Supplemental raw-token extract verification

**PASS.** Independent GPT-6 Astra reviewer `/root/numeric_audit`, 2026-09-09. This is a bounded supplement to the existing result audit, not a new full review or execution allocation. The Moriarty development skill remains loaded; refreshed guarded status preserves the old SP01 operational-history block and reports no pending transactions. No K command or source edit was performed.

Exact SHA-256 identities:

- `raw-token-review-extract.json`: `25f55b09ced787143b0d99cced9b5c47d5d30df5596567c66fdca1aed6e39118`.
- Original `result-candidate.json`: `38e6bf4661a31f0a502f4545d22f48f292ea8f80c83f1469464dca7e9baf5f8b`.
- Preserved `gpt6-result.md`: `ff50a6d8bb7708692c99a9243f6d563c75b7a1346557246582e4961ac0b7093b`.

I independently parsed all 64 retained raw KAST stdout files and compared each row's ordered trace identity, raw stdout SHA-256, constructor label, arity and literal token list directly with the extract. Every field matches. All 64 raw stdout hashes and all 65 compile/trace command-receipt hashes also match the original result manifest. The comparison did not invoke the codec or trust the extract's summary counts.

The independently counted constructors are 22 `prepared`, six `preparedTransfer` and 36 `rejected`: 28 successful preparations in total, 36 rejections, 64 outputs. The explicit debt-status token is present in the 22 sixteen-argument `prepared` outputs. The six nine-argument `preparedTransfer` outputs contain a digest and eight integer tokens, with no debt-status token; the decoder preserves their unchanged debt metadata. A statement that all 28 successful raw outputs emit debt status would be incorrect.

All extracted timing statistics exactly match the retained command receipts:

| Statistic | Seconds |
| --- | --- |
| Compile elapsed | 12.329168309999659 |
| Minimum krun elapsed | 2.273216802001116 |
| Maximum krun elapsed | 6.286211976999766 |
| First trace aggregate | 18.000994734000415 |
| Last trace aggregate | 328.6530731849998 |

Thus 36 is the correct rejection count, and the first trace aggregate is 18.000994734000415 seconds, not approximately 12.355 seconds. These are factual corrections to the described Opus prose, consistent with the original raw receipts and my existing PASS. This supplement verifies the extract and these facts; it does not preapprove an unseen Opus amendment. Preserve the original audit and record its author's amendment separately. No full-stage, universal correspondence, proof or ledger acceptance follows.
