---
title: "N1: native Compact circuit sizes"
type: source
tags: [measurement, compact, native, cost]
sources: [n1-native-circuits]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/measured/N1-native-circuits.md
source_url: local
---

# N1: native Compact circuit sizes

## Summary

The lead's first cost-round measurement ([[cost-round-brief]]): `compactc` 0.36.0 and `zkir-v3 mock-compile` on the `tiny`, `election` and `zerocash` examples on this host. All figures measured. Native proving time is not measured yet.

## Key Claims

| Contract | Entry point | k | Rows | Prover key (bytes) |
|---|---|---:|---:|---:|
| tiny | get | 7 | 68 | 113,707 |
| tiny | set | 13 | 3,887 | 11,277,187 |
| tiny | clear | 13 | 3,904 | 11,277,352 |
| election | set_topic | 13 | 3,884 | 11,277,195 |
| election | advance | 13 | 3,898 | 11,277,379 |
| election | add_voter | 13 | 5,972 | 11,277,747 |
| election | vote$reveal | 14 | 9,904 | 22,552,156 |
| election | vote$commit | 14 | 15,552 | 22,552,435 |
| zerocash | zerocash_mint | 13 | 7,784 | 11,277,336 |
| zerocash | spend | 15 | 24,025 | 45,104,355 |

- Verifier keys are 2,121 bytes for every entry checked (zerocash) (measured).
- Whole compile with key generation: tiny 2.8 s, election 5.9 s, zerocash 5.6 s (measured, includes TypeScript generation; not proving time).
- `zerocash` `spend` is k = 15, so "no verifiable circuit exceeds 2^14 rows" does not hold for this example.
- The cheapest entry that touches `persistentHash` is about 3,900 rows: one SHA-256 use sets a floor near k = 13 natively.
- One Sherlock modular leaf is 15,226 rows at S = 16 (DESIGN ESTIMATE), so native `spend` is about 1.6 leaves of rows; the ratio is set by machine steps per operation and the node term, not by the leaf.
- Not measured: native proving time per entry (`zkir-v3` has no prove command) and S1.

## Key Quotes

- "The ratio is set by machine steps per operation and by the node term (node gate 950,000 rows per node step), not by the leaf."

## Connections

[[cost-round-brief]], [[cost-lane-fable]], [[cost-lane-astra]], [[cost-lane-gemini]], [[MidnightLedger9]].

## Contradictions

- The k = 15 `spend` circuit contradicts the ledger 9 comment that no verifiable circuit exceeds 2^14 rows ([[MidnightLedger9]]); whether ledger 9 verifies it is open in [[cost-lane-fable]] and confirmed from sources in [[elastic-chip-gpu-fable]].
