---
title: "mori-chip crate"
type: entity
tags: [crate, chip-leaf]
sources: [chip-spike-experiments, quint-chipleaf-final, quint-chipleaf-hash, quint-chipleaf-pairrow-m3, c6-outcome-ledger9-review]
last_updated: 2026-11-10
---

# mori-chip crate

`crates/mori-chip` holds the chip leaf spike: `rf.rs` (32-word register file and spike rules), `rfcore.rs` (layouts and leaf), `poseidon.rs` (Poseidon-only HASH chip), `range.rs`, `alu.rs`, `mem.rs`, `m3.rs` (records, segment root, walker, digests), `vsel.rs` (packed selectors), tests in `tests/e3_spike.rs` ([[chip-spike-experiments]]).

- It is a spike with stand-in rules and no proof ([[c6-outcome-ledger9-review]], Q2 blockers).
- Its circuits are checked by three Gemini Quint lanes: [[quint-chipleaf-pairrow-m3]], [[quint-chipleaf-hash]], [[quint-chipleaf-final]].
- The review asks for a value digest experiment in this crate (rows, columns, node rows, negatives, Quint model, halt traversal beside it) ([[c6-outcome-ledger9-review]]).

See [[ChipLeaf]].
