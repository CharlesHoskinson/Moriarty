# Moriarty Core atomic-swap stop test

<!-- markdownlint-disable MD013 -->

Date: 2026-09-03 UTC  
Status: S3 experimental evidence  
Assignment experiment: E00

## Result

The initial E00 predicate passed at experimental scope. A finite Moriarty Core
atomic swap generated fixed-state Compact. The reference Core and an independent
manifest machine agreed on 1,000 unique deterministic traces. The traces had
zero observable divergence and zero conservation or non-negativity failures.

The pinned Compact compiler emitted TypeScript, metadata, and four ZKIR 3.0
circuits. The pinned ZKIR mock compiler accepted all four circuits.

This result does not prove the Compact compiler, ZKIR implementation, or proof
system correct. It does not establish key generation, proof generation, network
deployment, fees, proving latency, privacy against traffic analysis, economic
safety, or production security.

## Contract behavior

1. Alice deposits token A.
2. Bob deposits token B.
3. Bob submits public choice `0` or `1`.
4. Choice `1` atomically pays token A to Bob and token B to Alice.
5. Choice `0` refunds both deposits.
6. The absolute deadline refunds all assets held in a non-terminal phase.

The structural analyzer reports three accepted inputs, three internal
reductions, two payments, two live accounts, one absolute deadline, and 22 Core
syntax nodes.

## Privacy footgun reproduced

The first generated source used `decision` directly as a branch condition. The
Compact compiler rejected it because exported circuit arguments are private by
default and the branch changed public ledger effects.

The corrected source uses `disclose(decision)`. The artifact manifest also lists
`decision` as public. This is intentional because the Core choice is public.
Moriarty must never infer such disclosure silently from the backend error.

## Reproduce the semantic certificate

```bash
uv sync
uv run python experiments/moriarty-core-swap/generate.py
uv run pytest tests/test_translation_certificate.py -q
```

Expected certificate summary:

- 1,000 traces
- 1,000 unique ordered input sequences
- seven accepted transition classes
- five rejection classes
- both terminal phases
- deadline offsets `-1`, `0`, and `1`
- zero divergences
- zero invariant failures

## Compile to ZKIR

```bash
/nix/store/5h37yza1bii76f71wjpnhpdjs8m2a7pf-compactc/bin/compactc \
  --feature-zkir-v3 --skip-zk \
  experiments/moriarty-core-swap/swap.compact \
  experiments/moriarty-core-swap/output

repos/midnightntwrk/midnight-zkir/target/debug/zkir \
  mock-compile-many experiments/moriarty-core-swap/output/zkir
```

Observed mock-compiler results:

| Circuit | ZKIR bytes | Instructions | Public inputs | Private inputs | Impacts | Model k | Rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `fundAlice` | 4,848 | 60 | 8 | 2 | 39 | 13 | 2,099 |
| `fundBob` | 4,848 | 60 | 8 | 2 | 39 | 13 | 2,099 |
| `decide` | 18,030 | 225 | 25 | 2 | 184 | 13 | 2,507 |
| `expire` | 14,083 | 177 | 21 | 0 | 146 | 9 | 452 |

`toolchain-results.json` records exact binary hashes, repository commits,
commands, artifact hashes, host scope, and the proof-generation limitation.

## Evidence files

- `artifact-manifest.json` records the accepted Core, bounds, public
  disclosures, witnesses, effects, and target tuple.
- `translation-certificate.json` records the trace corpus digest, coverage,
  invariant result, and zero-divergence result.
- `toolchain-results.json` records the Compact and ZKIR execution results.
- `output/compiler/contract-manifest.json` is the compiler-generated artifact
  manifest.
- `output/zkir/*.zkir` and `output/zkir/*.bzkir` are the generated circuit
  representations.
