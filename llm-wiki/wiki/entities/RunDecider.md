---
title: "Run decider"
type: entity
tags: [recursion, decider, on-chain]
sources: [c6-outcome-ledger9-review, chip-spike-experiments, e2-e4-experiments, design-2-7-chip-leaf]
last_updated: 2026-11-10
---

# Run decider

The terminal proof of a run (`crates/mori-recursion/src/run_decider.rs`). It verifies the last node proof and is the proof Midnight verifies on chain, with a Blake2b transcript ([[c6-outcome-ledger9-review]]).

- 929,166 rows, k = 20, over the chip-leaf node key: "cost model over the chip-leaf node key, unsafe SRS" ([[c6-outcome-ledger9-review]]); also 929,166 over the C6 digest leaf node, unchanged across leaf k ([[e2-e4-experiments]]). Gate: at most 1,000,000 rows ([[design-2-7-chip-leaf]]).
- The same log line gives 1,032,467 table rows of the 1,048,576 in the domain; whether the tables grow with the circuit was not checked ([[c6-outcome-ledger9-review]]).
- The decider does not depend on the leaf; it consumes only the node key ([[design-2-7-chip-leaf]]).
- Under C6 (SRC-006) it exposes exactly `[prog_d, input_d, outcome_d, n_steps]` and no value that depends on a memory address ([[C6OutcomeOnlyPublication]]).
- Proof size: 6,432 bytes for the k = 20 terminal decider on stagenet; ESTIMATE 6,320 bytes for the chip-key decider ([[c6-outcome-ledger9-review]]).

See also [[IvcNode]], [[AcceptRunContract]], [[MidnightLedger9]].
