---
id: experiments.benchmarks
type: benchmark
title: Reproduced benchmarks and experiments
status: active
updated_at: 2026-09-03T05:39:34Z
sources:
  - SRC-0002
  - SRC-0005
  - SRC-0007
  - SRC-0019
---

<!-- markdownlint-disable MD013 MD025 MD060 -->

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

## E00 Core atomic swap

**CLM-0117.** The canonical E00 swap passed its initial translation-validation
predicate on 2026-09-03. This is an experiment observation from `SRC-0019` at
status S3. Reproduction is complete for the recorded host and pinned binaries.
Confidence is high for the tested predicate.

The finite Core has two deposits, one public choice in `0..1`, one absolute
deadline, and two atomic settlement payments. Its structural bounds are three
accepted inputs, three internal reductions, two payments, two live accounts,
one maximum timeout, and 22 syntax nodes.

The reference interpreter and independent manifest machine agreed on 1,000
unique deterministic traces. Coverage included seven accepted transition
classes, five rejection classes, both terminal phases, and deadline offsets
`-1`, `0`, and `1`. The run found zero divergence, zero conservation failures,
and zero negative balances.

Compact compiler `0.34.100` emitted four ZKIR 3.0 circuits. The pinned ZKIR mock
compiler accepted each circuit:

| Circuit | ZKIR bytes | Instructions | Public inputs | Private inputs | Model k | Rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `fundAlice` | 4,848 | 60 | 8 | 2 | 13 | 2,099 |
| `fundBob` | 4,848 | 60 | 8 | 2 | 13 | 2,099 |
| `decide` | 18,030 | 225 | 25 | 2 | 13 | 2,507 |
| `expire` | 14,083 | 177 | 21 | 0 | 9 | 452 |

**CLM-0118.** The first compile attempt failed because the public Core decision
was still a private Compact circuit argument. This is a reproduced experiment
observation from `SRC-0019`, status S3, with high confidence. The compiler
identified an undeclared indirect disclosure through branch-dependent ledger
effects.

The corrected source uses `disclose(decision)`. The artifact manifest lists the
decision as public. This correction is not cosmetic. A Moriarty visibility type
must decide disclosure before lowering and must reject any backend-driven silent
change to that decision.

The E00 run did not generate keys or proofs. It did not execute on a network or
measure fees and proving latency. It did not prove the Core-to-Compact compiler,
Compact compiler, ZKIR implementation, or proof system correct.
