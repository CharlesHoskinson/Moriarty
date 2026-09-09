# Funded repayment reference transition

The new local kernel moves settlement assets before it discharges debt. Paying 30 against principal 100 leaves principal 70 and moves 30 from payer to creditor. Paying 7 against principal 100 and accrued interest 10 under `AccrualFirst` leaves principal 100 and accrued interest 3. Spending allowances record gross debits, and a transfer cannot fund repayments beyond its unallocated amount.

This is the isolated `moriarty-funded-repayment/0` projection for SP03.1. It does not connect the `.mori` parser to an evaluator, implement K, authenticate financial state, sign a transaction, or establish native proof or Midnight acceptance. It does not close SP03, CM04/CM09, full RP01, or any other whole sprint. The full roadmap retains its gates.

## Run and inspect

The [kernel contract](../../experiments/moriarty-language/spec/successor/repayment-kernel.md) defines every input field, transition, bound and rejection code. The [compact fixture](../../experiments/moriarty-language/spec/successor/examples/funded-repayment.json) supplies due100/pay30. From the repository root:

```sh
npm --prefix experiments/moriarty-language run build
npm --prefix experiments/moriarty-language test
node deliverables/funded-repayment-2026-09-09/independent-probes.mjs "$PWD"
node deliverables/funded-repayment-2026-09-09/error-code-probes.mjs "$PWD"
```

[Recorded checks](checks.json) show the TypeScript build and all 178 tests passing (117 existing and 61 new). Additional independent controls cover 42 financial/input cases and 32 stable rejection-code/index cases. The [candidate manifest](candidate.json) binds the reviewed files. The independent GPT-6 result is recorded in `gpt6-audit.md`; a verdict applies only to that candidate and its stated scope.

## Financial limits

Input is supplied local state, not an authenticated ledger snapshot or a complete successor Debt record. Third-party repayment consumes that payer's allowance; this differs from the retained F.2 debtor-funded sentence and makes no full correspondence claim. Conversion rounds per allocation, so splitting payments can change rounding. Creditor refunds remain separate transfers: they preserve gross debit consumption but this kernel does not check creditor net goals or fees. Richer records need an explicit preserving adapter. K rules, Core elaboration, nominal-debt authority and ledger acceptance remain subsequent obligations.

The [retained failure observation](unfunded-existing-repro.json) comes from the round-08 composition model at `evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py`, SHA-256 `f6f5ead61868a4b693bc4bf710123120fa00fda9857b37ca3911a5a6bc6e43fa`. Removing all Transfer effects from `commonAcceptance.discriminators.partialDuty50to30.completeLegalState` still let that model reduce the duty 50→30 while payer/creditor cash stayed 50/0. That model remains failing; this isolated implementation does not repair or promote it.

## Authorship and review

[Authorship metadata](authoring.json) records actual Grok 4.6 high requests and the observed serving model `grok-4.6-build`. GPT-6 mechanically materialized those responses and authored independent probes and evidence/status prose. A separate fresh GPT-6 Astra reviewed the source and financial behavior.

The first runtime suite passed, but TypeScript compilation exposed a result type that declared `effect` while its producer and consumer used `value`. Grok supplied the [one-line type correction](repair-01-replacements.json). GPT-6 also found a hypothetical error-code catalog in the initial documentation; Grok replaced it with the [actual catalog and scope boundaries](repair-docs-01-replacements.json). The initial failures are not counted as successful runs. [The initial red run](red.txt) records the missing module before implementation materialization; it is distinct from the older model's financial failure. Provider private reasoning is excluded from publication.

No native/K campaign or Midnight transaction was run. The existing dependent campaign remains blocked by stale bindings and missing current accounting/operational evidence. This source slice changes no resource history, accepted atomic behavior or campaign admission.
