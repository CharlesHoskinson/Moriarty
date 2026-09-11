# Terra completion 01 result

## Outcome

No independently reproduced production defect required a source-runtime repair.
The approved `moriarty-financial-expression-source/1` CLI candidate checks the
published vault quote with the same source-only static result as its public API,
and its formatter re-checks and is idempotent. This completion record adds that
actual fixture path as a subprocess regression and gives developers copyable
commands at the profile entry point.

## Changes

- `tests/expression-cli.test.mjs` now runs the real financial vault-quote
  source and canonical schema through `src/cli.ts`. It compares CLI `check`
  output with `createFinancialExpressionSourceV1(...).check`, checks the
  formatted source again through the CLI, and verifies a second format has
  identical bytes.
- `spec/successor/financial-expression-source.md` now shows the exact explicit
  `check` and `format` commands for that fixture and links the transport/error
  contract.

## Commands and observations

| Command | Outcome |
| --- | --- |
| `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` | The guarded queue reports an unrelated operational-history block. Routine local source/documentation work remains authorized; no campaign or network action ran. |
| `npm --prefix experiments/moriarty-language run build` | exit 0 |
| `npm --prefix experiments/moriarty-language test` | exit 0; 679 tests, 7 suites, 0 failures, skips, or cancellations |
| `node --test experiments/moriarty-language/tests/expression-cli.test.mjs` | exit 0; 13 tests, including the added published-fixture CLI/API/format round trip |
| `node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/financial-vault-quote.schema.json experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori` | exit 0; `SourceChecked`, profile `moriarty-financial-expression-source/1`, static work bound `43` |
| `node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-expression-source/1 experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori` | exit 0; canonical source output, rechecked by the regression test and idempotent |

Adversarial public-path probes also confirmed that a malformed canonical schema
returns the stable `INPUT_SCHEMA` source rejection and that the original
`moriarty-expression-source/1` CLI still checks and formats its own source.
No input file, snapshot, schema, or network state was modified by these probes.

## Limits

This is source-only checking and formatting. It does not establish a financial
transition, source-defined financial schema, simulation, K correspondence,
proof, wallet, native, or Midnight Preview behavior. SP02 remains open pending
its stated specification and acceptance gates, plus fresh independent GPT-6
Astra and Grok 4.6 audits of the exact candidate bytes.
