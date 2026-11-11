---
title: "N1b: native Compact proving time per entry point"
type: source
tags: [measurement, compact, native, cost, ledger9]
sources: [n1b-native-proving-time]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/measured/N1b-native-proving-time.md
source_url: local
---

# N1b: native Compact proving time per entry point

## Summary

The lead's second cost-round measurement ([[cost-round-brief]]): real proofs with honest witnesses for all ten entry points of [[n1-native-circuits]], on the ledger 9 proof-server pin, CPU only, 20 rayon threads. Native proving takes 0.55 s for `tiny` `set` (k = 13), 1.02 s for `vote$commit` (k = 14) and 1.76 s for `spend` (k = 15), all measured. These replace every estimate of native prover time in the cost lanes. A synthetic SHA-256 circuit series extends the k range. As read (file time 12:26), the rows for k = 20 unthrottled, 21 and 22 and the curve section are unfilled placeholders.

## Key Claims

- Prover stack: `midnight-zkir` tag `zkir-3.2.0` (named by ledger 9 `proof-server/Cargo.toml`), on `midnight-proofs` 0.8.2, `midnight-zk-stdlib` 2.4.0, `midnight-circuits` 7.3.0; the ledger 9 `zkir-v3` crate (3.0.0) refuses zkir 3.1, which `compactc` 0.36.0 emits.
- Public parameters: the Midnight ceremony files; the 2p21 and 2p22 files were downloaded and their SHA-256 matches the hashes pinned in ledger 9.
- Medians over 7 timed runs (measured): `tiny` `get` 0.031 s; k = 13 entries 0.54 to 0.58 s; `vote$reveal` 0.932 s; `vote$commit` 1.022 s (326 MB); `spend` 1.755 s (581 MB peak RSS). Proofs are 5,392 bytes (3,824 for `get`). Every warm-up proof verified with the matching verifier key.
- Time is set almost entirely by k; rows inside one k change it little.
- Synthetic SHA-256 series (measured, stdlib column layout, not the node's): k = 16 3.316 s; k = 17 5.359 s; k = 18 10.880 s (3.9 GB); k = 19 19.510 s (7.5 GB). Synthetic k = 13 to 15 agree with the real entries within 4 percent.
- The k = 20 row at a 16 GB cap (132.8 s median, 14.9 GB peak) is throttled by the cgroup limit "and that number should not be used"; an unthrottled k = 20 row at a 40 GB cap is referred to but not filled in as read.
- Peak memory is about 14.6 KB per row of 2^k for stdlib-shaped circuits; memory, not time, decides which k fits one job on a shared host.
- One deviation: `election.compact` has no constructor, so the driver sets ledger field 0 (`authority`) before the first call.
- Not measured: thread scaling, GPU proving, proof-server HTTP overhead, verification time, the Sherlock node's own column layout.

## Key Quotes

- "This replaces any ESTIMATE of native prover time in the decomposition."
- "k = 20 at the 16G cap is throttled, and that number should not be used."

## Connections

[[n1-native-circuits]], [[cost-round-brief]], [[cost-lane-fable]], [[cost-lane-astra]], [[cost-lane-gemini]], [[MidnightLedger9]].

## Contradictions

- The measured 1.755 s for `spend` falls inside the [[cost-lane-fable]] estimate (1 to 2 s) and is below the [[cost-lane-gemini]] estimate (4.8 s, range 2.8 to 6.5 s); the Gemini time ratios rest on the higher figure.
- The file is incomplete as read: placeholder rows `K20ROW`, `K21ROW`, `K22ROW` and `CURVE` remain.
