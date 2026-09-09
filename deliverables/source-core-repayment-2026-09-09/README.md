# Source, typed Core and funded repayment — SP02/SP03 preparation

This work item connects explicit `.mori` Transfer and Repay emissions to the retained funded-repayment reference kernel. The source path must distinguish nominal debt units from settlement-asset units and preserve the entire supplied financial projection. A bare debt-field subtraction is not a payment.

Status: the source/Core/reference path is implemented and both required audits passed. Fresh GPT-6 review verified a successful build, 235 tests, 24 independent source probes and 64 reviewer probes. Exact Fable 5.1 reviewed the source and complete execution evidence; its initial documentation finding is resolved. GPT-6 reviewed the documentation change and verified all remaining candidate bytes were unchanged. No K execution, proof or Midnight transaction occurred. Full SP02 and SP03 remain open.

## Run the source example

From the checkout root:

```sh
node experiments/moriarty-language/src/successor/simulate-cli.ts simulate experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori experiments/moriarty-language/spec/successor/examples/funded-partial-payment.invocation.json
```

The checked example returns a local Prepared candidate: payer cash 70, creditor cash 30, principal 70, spent allowance 30 and remaining work 98 with closure reserve 16 preserved. It executes the actual `.mori` parser, typed Core elaboration and retained funded-repayment kernel. `settlementAsset` in source lowers to the kernel's `asset` field; the former is used because `asset` is a reserved parser token.

`spec/successor/funded-source.md` under the language package defines the supported forms and public API. Arithmetic, state writes, guards and richer source forms are explicitly unsupported in this slice. Serialized Core is inspectable data and is not accepted as an execution input.

## Independent expectations

`independent-financial-cases.json` contains six independently specified complete results: principal partial repayment, interest-first allocation, principal-first allocation, complete repayment, payment crossing the interest/principal boundary, and transfer40/repay30. These records include every balance, allowance, obligation term, used identifier, work value and ordered effect. The expected results were written separately from the new implementation.

`node deliverables/source-core-repayment-2026-09-09/check-independent-financial-cases.mjs` compares these expectations with the retained reference kernel. This baseline check does not exercise the new source frontend or K. The separate `independent-source-probes.mjs` runs the real source path against all six complete expectations plus rejection, conversion and preservation cases; its initial missing-module failure is retained.

## Scope and acceptance boundaries

The source/runtime input is a supplied local financial projection, not authenticated ledger state. Successful output may be Prepared only. Source types, a parser result, a Core record or an evaluator result do not establish authorization, PCD, ledger acceptance or a successor semantic freeze.

The initial K proposal takes lowered repayment input, with explicit restrictions on its domain. It is not a K interpreter for the full typed Core. Finite three-way observation checks, if executed, will remain separate from the required elaboration, evaluator and ledger correspondence proofs. Unsupported projection cases must be reported explicitly.

`boundary-review.md` is one conditional GPT-6 design/resource vote, not a result audit or execution admission. The old native campaign's stale inputs and unresolved accounting remain untouched. No K compilation, proof or public execution may be inferred from this source preparation.

## Next required checks

- Both result audits are complete for this bounded candidate: `gpt6-audit.json` with `gpt6-doc-addendum.json`, and `fable-final-audit.json`.
- `candidate.json` binds the final 13 files. `candidate-initial.json` and the initial audit receipts preserve the original review and authentication history. Historical publication restrictions in the initial receipts describe that earlier review state; the final addendum and Fable receipt govern this candidate.
- Fable performed static inspection of supplied code and execution evidence. GPT-6 ran the tests and probes; the root separately verified candidate hashes and the real CLI result.
- For K execution, first close actual candidate/resource admission, then retain actual compile/trace output and complete observation comparisons.
- Keep the full SP02/SP03, native, proof and financial Preview acceptance gates open until their own evidence exists.
