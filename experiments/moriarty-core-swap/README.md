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
The backend machine interprets the manifest's structured public phase, input,
time-guard, timeout-priority, and value-effect table. Witness authorization and
privacy behavior remain compiler/static-review obligations outside this
differential comparison.

The pinned Compact compiler emitted TypeScript, metadata, and four ZKIR 3.0
circuits. The pinned ZKIR mock compiler accepted all four circuits.

A fresh `git archive` of commit
`006c4d91ed09c0a89261861b6e7203b3efa3e2df` reproduced 43 focused tests,
the 1,000-trace certificate, the Compact source, compiler manifest, and all
eight text and binary ZKIR digests. The receipt is
`evidence/wp01/reproduction-receipt.json`.

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

The `negative/` directory preserves the failing source and a receipt with the
exact command, exit code, full-diagnostic digest, and a bounded diagnostic
excerpt. The negative source differs from the passing source only at that
disclosure wrapper.

Moriarty now checks the visibility manifest against every lexical `disclose`
call in generated Compact. The scanner accepts legal whitespace and ignores
comments and string literals. Negative tests remove the required `decision`
disclosure and add an undeclared `phase` disclosure. Both fail before proof
generation or signing. A second check compares the compiler-reported versions,
circuits, arguments, witnesses, and ledger fields with the Moriarty manifest.

## Reproducibility footgun reproduced

The Compact compiler's source-map digest depends on how the input path is
spelled. An absolute-path invocation compiled successfully but changed the
compiler-manifest digest. The recorded relative-path command reproduced the
original digest. The SDK build orchestrator must use a canonical sandbox path
or canonicalize source maps before it claims reproducible builds.

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
- 18 action-by-phase deadline-boundary cells
- terminal expiry rejection from both terminal phases
- zero divergences
- zero invariant failures

Regression certificates also pass for the minimum deadlines `1`, `2`, and `3`
and for party names whose canonical account order is the reverse of their
Alice/Bob role order. Amounts and deadlines must be exact integers within the
declared Compact widths.

The generated contract is an abstract template. Its constructor schema records
the required Compact types and encodings, but no concrete Midnight user
addresses, authority hashes, or token colors are bound. Those values must be
recorded and verified in a separate deployment manifest before any network run.

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
- `evidence/wp01/reproduction-receipt.json` records the clean, pinned-commit
  reproduction.
- `evidence/wp01/current-checkout-validation.json` separately records the
  remediated source-hash validation. It does not claim a fresh archive or commit
  identity.
- `negative/undisclosed-decision.compact` and `negative/compile-result.json`
  preserve the disclosure negative control.
- `output/compiler/contract-manifest.json` is the compiler-generated artifact
  manifest.
- `output/zkir/*.zkir` and `output/zkir/*.bzkir` are the generated circuit
  representations.
