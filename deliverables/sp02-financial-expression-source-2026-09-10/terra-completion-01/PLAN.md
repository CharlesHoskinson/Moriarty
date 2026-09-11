# Terra completion 01 plan

## Scope

Verify and, only if independently reproduced, repair the approved
`moriarty-financial-expression-source/1` source-profile and its explicit
`check`/`format` CLI adapter. This task covers source parsing, schema transport,
static checking, formatting, diagnostics, and CLI/API agreement. It does not
change financial expression semantics, financial transition support, source-owned
schemas, K, proofs, wallets, network operations, or any SP02 completion gate.

The current user routing is Terra implementation, then fresh GPT-6 Astra at
medium effort and Grok 4.6 at high effort for independent result audits. Earlier
review artifacts are preserved as historical evidence and do not by themselves
identify a product defect or approve this candidate.

## Paths

- `experiments/moriarty-language/src/cli.ts`
- `experiments/moriarty-language/src/successor/financial-expression-source-v1.ts`
- `experiments/moriarty-language/src/successor/financial-expression-source-frontend.ts`
- `experiments/moriarty-language/src/successor/format.ts`
- `experiments/moriarty-language/tests/financial-expression-source-*.test.mjs`
- `experiments/moriarty-language/tests/expression-cli.test.mjs`
- `experiments/moriarty-language/spec/successor/financial-expression-source.md`

## Observable checks

1. Build and run the package test suite before and after any repair.
2. Run CLI `check` and `format` on `financial-vault-quote.mori`; compare check
   output with the public API and verify format parse meaning plus idempotence.
3. Exercise hostile malformed source, schema, CLI argument, and file input
   paths. Each must return the documented stable rejection without a crash.
4. Confirm old `expression-source/1` CLI behavior remains available.
5. Retain exact commands and outputs in `RESULT.md`, then commit only this
   scoped record, regression evidence, and any required source repair.

## Decision rule

Do not modify implementation merely to enlarge the candidate. A reproduced
production-path defect receives a narrow regression test and repair. If no
defect reproduces, retain adversarial verification and developer-facing evidence
instead.
