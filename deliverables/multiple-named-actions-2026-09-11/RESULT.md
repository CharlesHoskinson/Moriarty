# Multiple named actions result

Implemented the bounded source-language capability using Grok 4.6 high
(returned model `grok-4.6-build`) and a fresh GPT-6 Astra medium full-candidate
audit. The new `moriarty-financial-agreement-source/2` profile supports named
actions with shared declarations and ordinary state. All actions are checked;
each owns its argument and local scope. API and CLI simulation select exactly
one action explicitly. The existing /1 contract is preserved.

The actual fixture runs repay 30 followed by installment 20 from debt 100.
Residual debt is 50, payer/lender balances 50/50, allowance remaining/spent 50/50,
with both transfer/allocation identifiers retained. Work use is 43 then 45,
leaving 12 from 100; the closure reserve remains 16. Failed invocations expose
no tentative state or effects. Independent controls cover unselected static
errors, runtime isolation, scope leakage, argument shape, interest-first
repayment, exact work boundaries and old profile diagnostics.

## Acceptance evidence

- Root package verification: 748 tests passed, 0 failed; typecheck exit 0.
- Root independent probes: 23 CLI, 27 API and 48 complete /1 differential comparisons passed.
- Fresh Astra medium audit:approve, no findings;748 tests, typecheck and 109
  independent probes passed, including 192 /1 differential comparisons.
- Full candidate hash:`002106488ff8ce8a1855d408c4417e0d911123dee088873ecb0b9f9e53c44b94`.
  All 165 source/plan files matched before and after the audit and in main
  after integration. The audit reviewed the full affected execution path and
  relevant unchanged dependencies; it was not a diff-only review.
- Main integration reran 23 CLI and 27 API probes successfully. All 52 unrelated
  changed tracked files retained their original bytes. Only task files are
  staged in the isolated branch; copied guidance/plugin changes are excluded.

See `candidate.json`, `verification.json`, `integration.json`, `root-*-final.txt`,
`main-*-final.txt` and `audit-01/review.json` for exact scope and receipts.

## Corrections retained

Independent root comparison found a /1 validation-order regression after
compiler extraction. Grok reproduced it, added failing compound-invalid tests,
and restored parameter checking before protected binding for /1. All 48 root
comparisons now match the reviewed baseline; /2 retains shared binding first.
The initial new snapshot fixture had a trailing newline incompatible with
canonical transport and was corrected before final verification. Failed
receipts remain intact. The auditor also preserved its initial erroneous
work-count oracle and documented its constructor-count correction; no
candidate change was needed for that audit issue.

## Scope and publication

Branch:`feat/multiple-named-actions`, based on reviewed source-defined
repayment commit`ea40ab488d1d056c7192cc4dfd7ae250d44e3d49`.
Publish as a stacked draft PR on`feat/source-defined-repayment` (#2), with
remote confirmation retained in the local publication receipt.
This is local language and funded-preparation acceptance. It does not establish
ledger settlement, formal correspondence or native proofs. Existing live
admission and accounting stops remain unchanged.
