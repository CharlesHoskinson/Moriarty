# S02 effect foundation report

## Status

The effect foundation is implemented and executable. The work covers pure
effect arithmetic and a non-candidate deposit and refund harness only.

The work does not implement a candidate architecture, full intent
authorization, signed-policy verification, Core correspondence, signing, a
proof, production ledger execution, or the S02 gate.

## Contract-first evidence

The required Python contract test failed before the supplement binding existed.
The corrected design binds the supplement at SHA-256
`1adc668e970d35a492e272f4f22ac468dccfd690b2c2e1a0ce3ac62030d3ae25`.
The design and architecture specification name both recovery subscenarios and
the pre-sign distinction. The focused contract result was `3 passed`.

## Quint implementation

The model provides the requested `Principal`, `Asset`, `Location`, `Transfer`,
and `Ledger` types. It provides `canApply`, `applyTransfers`, `policyAllows`,
and `totalAsset`. Wallet and escrow remain distinct. Prefix overdraft checks,
ordered transfer application, and transfer multiplicity remain explicit.

The harness instantiates `INITIAL_A = 10` and `INITIAL_B = 20`. The deposit
action and refund action have separate guards and updates. Each action assigns
the complete state. The terminal phase is phase 2.

Quint 0.32.0 does not re-export imported definitions through the wrapper module.
The test imports the instantiated machine and imports pure effects separately.
Quint 0.32.0 also evaluates imported definitions when `--match '.*'` selects
tests. That literal command attempts 34 non-test definitions and fails. The
corrected discovery command uses `--match 'Test$'` and passes ten tests.

## Commands and results

Tool versions:

- Quint `0.32.0`
- Node `v24.18.1`
- Python virtual environment at `/home/charl/Moriarty/.venv`
- Base commit `aea316c518451039ea61c14dd86e07f2ac9395eb`

Successful commands:

```text
/home/charl/.npm-global/bin/quint typecheck specs/quint/s02/effects.qnt
/home/charl/.npm-global/bin/quint typecheck specs/quint/s02/effects_harness.qnt
/home/charl/.npm-global/bin/quint typecheck specs/quint/s02/effects_test.qnt
/home/charl/.npm-global/bin/quint test specs/quint/s02/effects_test.qnt --main effects_test --match 'Test$'
/home/charl/.npm-global/bin/quint run specs/quint/s02/effects_harness.qnt --main effects_harness --invariant safety --witnesses deposited refunded --seed 42 --max-samples 10000 --max-steps 4
/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py
/home/charl/Moriarty/.venv/bin/python -m pytest -q
git diff --check
```

Results:

- The three Quint typechecks passed.
- Ten Quint tests passed.
- The safety invariant passed with no violation.
- `deposited` was reached in 10,000 of 10,000 traces.
- `refunded` was reached in 10,000 of 10,000 traces.
- The read-only S01 validator passed S01-01 through S01-10.
- Python tests passed with `285 passed in 9.50s`.
- `git diff --check` passed.

The required literal discovery command was also executed. It failed because
Quint selected imported definitions in addition to the ten named tests. This
failure is a tool-selection issue, not a model predicate failure.

## Evidence files

The evidence directory is `evidence/s02-model-comparison/foundation/`.

| File | SHA-256 |
| --- | --- |
| `effects-typecheck.txt` | `1a18a0378f71046cafecf8395840d7a5121a1cf100a0af4ed269a6dfa189da24` |
| `effects-harness-typecheck.txt` | `d065d573effb102427834b62312dd98a055752f2ec80636d806d145269088e07` |
| `effects-test-typecheck.txt` | `b06ee47b230094e3367ff2ab78dedbb048b430aac85efa621021fe34153484a6` |
| `effects-test-10.txt` | `e36c7c0cd5e1a7592b3c9c530a48ed345520c1956ac6a56938db784bb7eb7b27` |
| `effects-run-10000.txt` | `90a1dbab2badb599a2424fdee1f37958ef33c3a586ccae407f8e895dd9e1ea04` |
| `effects-receipt_0.itf.json` | `04c3b3d8febde81855662e440b7471cfa6f281248d6643d0c5edac878aed2378` |

The ITF receipt contains the initial ledger, the deposit state, and the refund
terminal state. It has three states and uses the deterministic seed 42.

## Source hashes

| File | SHA-256 |
| --- | --- |
| `specs/quint/s02/effects.qnt` | `dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c` |
| `specs/quint/s02/effects_harness.qnt` | `9882eb281b4a45628501106d2a09a0c4c69ea05d46e70bab9ae41940dc85a9dc` |
| `specs/quint/s02/effects_test.qnt` | `4990c486644b2ffb0027a0a1ca580bfd15bc918feaf6faabe5bca7ebe6d117ff` |
| `specs/quint/s02/README.md` | `d4b9f2bf045a681d2e795cdf4ba1aade49fd241b54092cb265f4d933028fce53` |
| `tests/test_s02_contract.py` | `8f56b44dd1b01c67292195b04c37c0a7a13762a7a4b0d271fbf90a5a260aee0c` |
| `openspec/changes/s02-model-comparison/design.md` | `96f101c1886f8b4514cb71d1ba6dd93bee9c059b92ccc4d9c4a7843650232d04` |
| `openspec/changes/s02-model-comparison/specs/architecture-comparison/spec.md` | `18afb70f6d2267fe8b31b103644bb6edd156d6ec6d30ec1e4aefc00e3bc7d41a` |

## Concerns

The Quint 0.32.0 discovery behavior requires the `Test$` regex for the ten
tests. The report preserves that correction and the failed literal command.

The generated run is sampled simulation. It is not Apalache model checking.
This foundation does not provide candidate or S02-gate evidence.

## Evidence correction

The first-task `.txt` files were generated with Quint `--out`. They are
structured compiler JSON, not raw console receipts. I retain those files and
do not use them as command provenance.

Fresh command provenance and separate raw stdout and stderr are preserved in
`evidence/s02-model-comparison/foundation/fresh-receipts.md`. The matching
manifest is `fresh-manifest.json`. The fresh receipt uses the exact worktree,
argv, exit status, safety invariant, witness names, 10,000 sample count, seed,
and ITF output path. The fresh ITF is
`effects-fresh-0.itf.json` with SHA-256
`ed144b8e17e85e4cdd7cbd22db8e9102fb66f56ab7f67c50fae4a4cb6a4e7fb1`.

The fresh corrected test command passes ten tests. The fresh literal
`--match '.*'` command exits 1 because Quint selects imported definitions.
The report records that diagnostic as an actual tool behavior.

The original process did not retain raw missing-import RED or deposit-only
incremental output. I do not claim reconstructed output as original history.
The retrospective reconstruction and its exact temporary inputs, commands,
exit statuses, and raw output hashes are in
`evidence/s02-model-comparison/foundation/retrospective/reconstruction-report.md`.
The reconstructed missing-import stderr and deposit-only stdout are preserved
under that directory.
