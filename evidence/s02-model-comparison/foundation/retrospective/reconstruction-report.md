# Retrospective process reconstruction

The first-task raw files did not preserve the missing-import RED output or the
deposit-only incremental output. No timestamp or chronology claim is made for
the original process. The records below are fresh reconstructions performed
after commit `d069800` in `/tmp/moriarty-s02-retro.TLqMoD`.

The reconstruction copied the committed `effects.qnt` and current test source.
It used `apply_patch` to change the test import to
`./missing_effects_harness`. It used `apply_patch` to remove refund from the
temporary harness and set `step = deposit`. The temporary source hashes are:

| Input | SHA-256 |
| --- | --- |
| `effects.qnt` | `dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c` |
| `effects_test-missing-import.qnt` | `e79ae106bb8656e4860ab55e04f5b7de98fb731670cea11cfc3c79af0e60bbbe` |
| `effects_harness-deposit-only.qnt` | `d95f9ba61d74d0a7a33fcb5699ca22f5ab716f194523f0a1b523bfda3341e341` |

## Missing-import RED reconstruction

Work directory:

```text
/tmp/moriarty-s02-retro.TLqMoD
```

Command:

```text
/home/charl/.npm-global/bin/quint test effects_test.qnt --main effects_test --match '.*'
```

Exit status: `1`.

Raw stdout was empty. Raw stderr is preserved at
`missing-import.stderr` with SHA-256
`a54191fee00240384c462f15efdb255a92dce160577e4915aebb0bea32d87c65`.
The output contains the missing-module diagnostics `QNT013` and `QNT405`,
followed by the dependent unresolved-name diagnostics, and ends with
`error: parsing failed`.

## Deposit-only incremental reconstruction

The deposit-only harness typechecked with this command:

```text
/home/charl/.npm-global/bin/quint typecheck effects_harness.qnt
```

Exit status: `0`. Stdout and stderr were empty.

The deposit-only run used this command:

```text
/home/charl/.npm-global/bin/quint run effects_harness.qnt --main effects_harness --witnesses deposited --seed 42 --max-samples 10000 --max-steps 1
```

Exit status: `0`. Raw stdout is preserved at `deposit-run.stdout` with
SHA-256 `aa88e85f0ee34889b354d3f4d4801300ddec02e8400bc17b1802012018a30282`.
Raw stderr was empty. The reconstructed run reaches `deposited` in 10,000 of
10,000 traces and reports no invariant violation.

These are retrospective reconstructions. They supplement the fresh final
receipts. They do not claim to be the original execution chronology.
