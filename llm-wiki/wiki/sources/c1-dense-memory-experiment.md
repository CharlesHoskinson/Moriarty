---
title: "Experiment C1: dense tail-buffered memory backend (measured)"
type: source
tags: [memory, dense-tail, experiment, measured, chip-leaf, run-length]
sources: [c1-dense-memory-experiment, runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area2-memory-draft]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/EXPERIMENTS.md
source_url: local
---

# Experiment C1: dense tail-buffered memory backend (measured)

## Summary

First backend of night-plan step C1. A `MemBackend` trait and a dense tail-buffered Merkle backend were built as Halo2 chips in the spike crate `crates/mori-chip` (`membackend.rs`, `dense.rs`), with a harness (MEM window, backend, CORE stand-in) that passes MockProver (9 positive tests) and refuses 21 named mutants. Rows were measured at TOP = 61 and 63. The LogUp backend, the hash-chain stacks and the wiring into the real leaf are not built. Run `20261111T065836Z-c1-rows` ([[DenseTailMemory]], [[final-design-decisions]]).

## Key Claims

- MEASURED: one flush unit 8,541 rows at TOP = 61 and 8,821 at TOP = 63 (9,031 / 9,311 with the segment hash); one cold read 4,621 / 4,761; one window slot 203; tail open 498; `persist(W) = 12,137 + 10,161 (W - 1)` at TOP = 61.
- MEASURED: TOP = 63 costs +280 rows per flush and +140 per cold read, and does not change the key shape (walk gates are per row), so it adds nothing to the node.
- MEASURED: the circuit pays the window capacity (203 rows per slot) whether a slot is used or not; a used slot adds 3 CORE rows.
- MEASURED: four arenas with one flush unit each: 49,172 harness rows against 12,281 for one arena (W = 1).
- ESTIMATE (composed): leaf rows at S = 16 / 64 / 128 with W = 1 / 3 / 5 and no cold read: 14,942 / 36,543 / 58,565 at TOP = 61; each cold read adds 4,751. At S = 16 the gate `k_leaf <= 16` holds up to about 10 cold reads per leaf.
- ESTIMATE (composed): four arenas at S = 16, TOP = 61: 52,205 rows with one flush unit per arena (2 cold reads fit `k_leaf <= 16`), 35,123 with two shared flush units (6 fit).
- ESTIMATE: node about +24,000 rows (net +4 fixed columns at 6,004 each), so about 847,500 against the 950,000 gate; not measured.
- Domain tags sit in the sponge capacity and cost no permutation; first-input tags would double the node hash cost (flush unit 17,080, ESTIMATE).
- The AUDIT finding 8 invariants are gates: both lengths decomposed into TOP + 3 bits, canonical tail (PAD above `r`), tail opened on every leaf and passed through when no append, kind tag in the bus key and as the first record-hash input, tagged hashes, `EMPTY_SEG != 0`.

## Key Quotes

- "TOP does not change the key shape: the walk gates are per row."
- "The circuit pays the window capacity: 203 rows per slot, fixed by W, whether the slot is used or not."

## Connections

- Measures the design of [[DenseTailMemory]] (R-3, R-4, R-6 of [[final-design-decisions]]) from [[runlength-consensus]] with the conditions of [[runlength-consensus-audit]] finding 8; the interface follows [[astra-area2-memory-draft]] section 4.
- Open: LogUp backend; hash-chain FRAMES and TASKS; integration into `RfLeaf` and a measured node; the 108-rule non-interleaving check (R-6); the per-step maximum of cold reads (audit finding 6); write-before-read of scratch (R-8, a CORE obligation).

## Contradictions

- None with the audit. The consensus's 4,480-row cold read omits the record hash (measured 4,621, as [[astra-area2-memory-draft]] said).
