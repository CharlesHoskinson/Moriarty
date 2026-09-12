A source-defined agreement can now contain multiple named actions that share declarations and ordinary state. The new `moriarty-financial-agreement-source/2` profile checks every action with independent argument/local scopes and requires explicit `--action` selection for simulation. The repayment example pays 30, then a fixed installment of 20, leaving debt 50 while preserving balances, allowance/history, and exact work accounting.

The existing `/1` profile retains its single-action API and diagnostics. Independent differential checks caught a validation-order regression during the shared-compiler extraction; the repair and regression evidence are included.

Validation: 748 package tests, TypeScript typecheck, 23 independent CLI probes, 27 API probes, and 48 comparisons against the reviewed `/1` implementation. Grok 4.6 high authored the change (returned identity `grok-4.6-build`). Fresh Astra medium audit receipt is retained with the complete candidate manifest in `deliverables/multiple-named-actions-2026-09-11/`.

This draft PR is stacked on #2 (`feat/source-defined-repayment`). Scope is local language execution; it adds no public ledger settlement or formal proof claim.
