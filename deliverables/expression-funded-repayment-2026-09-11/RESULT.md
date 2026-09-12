# Computed funded repayment: local implementation result

Grok 4.6 implemented the source adapter and normal CLI integration. Fresh GPT-6 Astra medium approved the final candidate with no unresolved findings.

A `.mori` action can now compute a nominal repayment, check guards, stage ordinary updates, and prepare funded Transfer/Repay effects together. The adapter uses the existing expression evaluator and one complete repayment-kernel invocation. Rejection exposes no tentative ordinary or financial state.

## Run

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/expression-funded-payment.schema.json --snapshots experiments/moriarty-language/spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/expression-funded-payment.mori
```

The default snapshot supplies two nominal Quantity arguments, 10 and 20. The program adds them and derives the settlement amount. The result transfers30, leaves principal70, records allowance remaining70/spent30, and preserves the separate closure reserve16. The final example consumes45 expression reductions and2 kernel actions, leaving work53 from100. These work figures apply to this exact example source.

Changing the arguments to3 and4 with initial principal100/accrued10 executes the same source and pays interest first: principal100, accrued3, outstanding103. Insufficient funding rejects with empty CLI stdout. [Independent CLI outcomes](independent-cli-probes.json) retain all three cases.

## Verification

| Check | Actual outcome |
| --- | --- |
| Baseline language suite | 682 passed; 0 failed |
| Final full language suite | 698 passed; 0 failed; 0 skipped |
| Final TypeScript check | exit0 |
| Root independent API probes | 19 passed |
| Root independent CLI cases | 3 passed |
| Fresh Astra focused suite | 113 passed; 0 failed |
| Fresh Astra independent cases | 44 passed:40 API and4 CLI |
| Final candidate hashes | all149 files matched before and after review |

The final candidate manifest SHA256 is `569da1d040f964acc9b422745ab0f277d1bd27fe2839287155f3066e23261800`.
[The implementation plan](../../docs/superpowers/plans/2026-09-11-expression-funded-repayment.md), [candidate](candidate.json), [final audit](audit-01/review.json), [full tests](root-tests-final.txt), and [typecheck](root-typecheck-final.txt) record scope and checks.
The focused, root and auditor counts overlap; they are not additive coverage totals.

## Authorship and corrections

Requested author: Grok4.6 high. Returned identity: `grok-4.6-build`; terminal status:`end_turn`; session:`01a09248-fc3e-7b71-b023-077691ce7337`. [Author receipt](author-receipt.json) preserves returned usage and identity. Grok completed this in one author session, including repairs.

The preimplementation Astra review corrected a proposed double reservation of closure work. Grok used `transferAmount` because `amount` is reserved source metadata. Two initial adapter type errors were corrected before audit. Root independent probes initially used unsupported syntax and were corrected without weakening their expected financial outcomes. Root requested a dynamic-argument example before freeze. Final Astra review found one README working-directory error; root made that editorial correction and Astra reviewed the final candidate. Historical candidate and audit receipts are retained locally.

## Acceptance and limits

This closes the bounded SP02/SP03 source-expression-to-funded-repayment integration predicate. It does not close either whole sprint. The original pure expression APIs and CLI forms keep their behavior; the funded mode requires an explicit repayment-state option.

Nominal payment uses nonnegative signed128 Quantity with scale0 and one denomination. Transfer amount uses a distinct Amount asset index. Exact operation schemas, runtime units, funding and kernel relationships are checked. Snapshot workInitial must equal carried remaining work. Both stages debit the same work pool; prior spent, allowances, used IDs, residual debt and reserve survive continuation.

The supplied financial projection is local input, not authenticated ledger state. Source ensures checks ordinary application post; it does not assert final financial postconditions. Source-defined schemas, multiple actions, full financial-operation coverage, K correspondence, authenticated authority, proofs and Preview acceptance remain open. No public transaction or proving campaign was run. Existing live admission/accounting stops remain unchanged.
