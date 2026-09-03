---
id: experiments.benchmarks
type: benchmark
title: Reproduced benchmarks and experiments
status: active
updated_at: 2026-09-02T18:20:00Z
sources:
  - SRC-0002
  - SRC-0005
  - SRC-0007
---

# Reproduced benchmarks and experiments

## Marlowe structural study

`experiments/structural-benchmarks-2026-09-02.json` analyzes real pinned V1
examples and synthetic stress cases. The recurring-payment family grows
linearly when unrolled:

| Periods | Canonical JSON bytes | Recursive nodes | Maximum depth |
|---:|---:|---:|---:|
| 10 | 3,687 | 241 | 42 |
| 100 | 36,807 | 2,401 | 402 |
| 1,000 | 368,007 | 24,001 | 4,002 |

A high-fan-out state with 10, 100, and 1,000 accounts serialized to 918,
8,568, and 85,068 bytes and implies the same number of abstract refunds. The
zlib figures in the experiment are only a relocation probe; they do not claim
that a compressed representation is ledger-compatible or cheaper to prove.

`experiments/typed_values.py` passed five tests: same-token addition and
timestamp-plus-duration were accepted; cross-token addition,
amount-plus-duration, and timestamp-plus-amount were rejected. This is a small
kind-checking experiment, not a Moriarty type-system implementation.

## Moriarty to Compact and ZKIR

The research escrow at
`experiments/moriarty-compact-escrow/escrow.compact` was generated against
Compact compiler `0.34.100`, language `0.26.0`, runtime `0.19.100`, ledger
`9.1.0.0-rc.3`, with `--feature-zkir-v3 --skip-zk`.

Six source-level acceptance checks passed. The compiler emitted TypeScript,
source maps, contract information, a hashed manifest, and three ZKIR 3.0
circuits. The current ZKIR `mock-compile-many` command accepted all three:

| Entry point | ZKIR bytes | Instructions | Private inputs | Model k | Model rows |
|---|---:|---:|---:|---:|---:|
| `fund` | 3,995 | 49 | 2 | 13 | 2,072 |
| `release` | 5,657 | 70 | 2 | 13 | 2,127 |
| `refundAfterTimeout` | 5,330 | 67 | 0 | 8 | 189 |

The test confirms syntactic and backend feasibility. It does not establish
proof generation, network deployment, fee, proving latency, or semantic
equivalence to a normative Moriarty Core. Full ZKIR proof integration tests
were blocked by the repository's required compile-time `MIDNIGHT_PP` proof
parameter directory. The ZKIR library suite did run independently: 44 tests
passed.
