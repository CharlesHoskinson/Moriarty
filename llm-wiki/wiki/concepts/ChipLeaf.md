---
title: "Chip leaf"
type: concept
tags: [chip-leaf, design]
sources: [design-2-7-chip-leaf, chip-spike-experiments, quint-chipleaf-final, quint-chipleaf-hash, quint-chipleaf-pairrow-m3, c6-outcome-ledger9-review, lookup-cache-brief, runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area1-processor-draft]
last_updated: 2026-11-10
---

# Chip leaf

The proposed universal Sherlock leaf: one hand-written circuit with a CPU core and the chips DEC, PAGE, MEM, ALU with RANGE, and HASH, reached through one lookup bus of tuples `(chip, key, a, b, c)`. Status: **proposed 2026-11-10, not adopted**. Until adoption, design sections 2.2 to 2.4 stand ([[design-2-7-chip-leaf]]).

## Mechanism

- Write-once, append-only memory. Memory grows in aligned segments of C = 8 cells per leaf (D3, M3), with each segment root appended to a depth-17 top tree ([[design-2-7-chip-leaf]], [[chip-spike-experiments]]).
- Soundness constraints D1 to D8 (provider-gated table side, one provider per key, aligned segments, gate-pinned chip tags, one enable per step, one MEM base, cached frame protocol, no tag equal to the padding tuple) ([[design-2-7-chip-leaf]]).
- The node's in-circuit verifier only supports rotations -1, 0 and +1, so the leaf uses narrow pair rows (layout B). The node pays for leaf columns and lookups, not leaf rows ([[chip-spike-experiments]]).

## Figures

All figures below are `cost_model` rows over keys made on an unsafe SRS; no proof was made. The design table heads them "Measured"; the experiment files define MEASURED as a `cost_model` count ([[chip-spike-experiments]], [[design-2-7-chip-leaf]]).

| Item | Figure | Gate |
|---|---|---|
| Leaf, S = 16 (final key) | 6,511 rows, k = 13 | `k_leaf <= 16` |
| Node over the leaf key | 823,466 rows, k = 20 | <= 950,000 |
| Decider over that node | 929,166 rows, k = 20 | <= 1,000,000 |
| HASH work per leaf | 87 Poseidon permutations, 6,090 rows | |

- ESTIMATE from the review: the value digest of [[ValueDigest]] adds about +1,680 rows in a worst leaf of 8 data appends, which could move the leaf to k = 14 ([[c6-outcome-ledger9-review]]).

## Verification state

- Packed selectors SOUND WITH CONDITIONS, running-sum RANGE SOUND, M3 as built SOUND WITH CONDITIONS with C6 open ([[quint-chipleaf-final]]).
- HASH chip SOUND with conditions C1 to C6 ([[quint-chipleaf-hash]]).
- Pair rows SOUND; M3 sound only with C2 to C6 ([[quint-chipleaf-pairrow-m3]]).
- Still stand-ins: the spike rules, no PAGE, PROG, TRAP or Init region, test siblings, a leaf statement that does not cover `prog_root` ([[design-2-7-chip-leaf]]).

## Open

- SI-12 under C = 8 blocks D3 adoption for the C6 machine (D-1, [[dissent-c6-section]]).
- The D3 run ceiling against GP-1 ([[RunLengthCeiling]]). Lead decision R-3, pending owner review, replaces D3 with dense tail-buffered memory and hash-chain stacks ([[DenseTailMemory]], [[final-design-decisions]]).
- The Astra area 1 draft proposes the processor modules CORE, DEC, ALU with RANGE, HASH and BUS ([[astra-area1-processor-draft]]); a draft for a later merge.
- A lookup cache study is pending ([[lookup-cache-brief]]).

## Contradictions

- [[design-2-7-chip-leaf]] still marks a Quint lane over the packed-selector circuit as OPEN, while [[quint-chipleaf-final]] reports it done.
- The D3 memory and the D7 cached frame protocol of the mechanism above are what the run-length study replaces. The audit notes that the new hash-chain stacks are not D7 (D7 links frames by an address in memory, and only that form is modelled) and that new chip models are a condition of adoption ([[runlength-consensus-audit]] finding 8, [[runlength-consensus]]).

See also [[C6OutcomeOnlyPublication]], [[MoriChip]], [[IvcNode]], [[RunDecider]].
