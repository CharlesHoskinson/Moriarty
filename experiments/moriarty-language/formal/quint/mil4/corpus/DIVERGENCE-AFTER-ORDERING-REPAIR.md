# S0 Quint ordering repair witnesses

**Target:** repaired provisional `../s0.qnt`, Quint 0.32.0.  
**Corpus:** `s0_divergence_witnesses.qnt`.  
**Scope:** fourteen fixed witness traces using the TypeScript backend.

The import stipulates `signatureVerified`, `authenticatedSnapshots`,
`nativeQualified`, and `ledgerAtomicReady`. The runs do not establish those
premises. Exact arguments, outputs, exit codes, timestamp and candidate SHA256
values are in [repair-command-results.json](repair-command-results.json).

From the checkout root:

```text
$ quint typecheck experiments/moriarty-language/formal/quint/mil4/s0.qnt
(no output; exit 0)

$ quint typecheck experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt
(no output; exit 0)

$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt --main s0_divergence_witnesses --match '.*Test' --seed 0x5 --backend typescript

  s0_divergence_witnesses
    ok overflowAndMissingFeeTest passed 1 test(s)
    ok repaymentAliasTest passed 1 test(s)
    ok debtorMismatchTest passed 1 test(s)
    ok debtorSignerMismatchTest passed 1 test(s)
    ok debtorPayerMismatchTest passed 1 test(s)
    ok signedDebtorScopeTest passed 1 test(s)
    ok signedCreditorScopeTest passed 1 test(s)
    ok suppliedPayerScopeTest passed 1 test(s)
    ok nonidentityConversionTest passed 1 test(s)
    ok conversionScaleTest passed 1 test(s)
    ok conversionRoundingTest passed 1 test(s)
    ok invalidRoundAndAliasTest passed 1 test(s)
    ok capAndAliasTest passed 1 test(s)
    ok overpayAndMissingCreditorTest passed 1 test(s)

  14 passing (118ms)
(exit 0)
```

| Witness | Competing conditions or scope defect | Observed first decision |
| --- | --- | --- |
| `overflowAndMissingFeeTest` | Receiver credit overflows and vector omits fee credit | Effect, `S0_EFFECT_RANGE` |
| `repaymentAliasTest` | Authenticated obligation creditor equals bound payer | Stage, `S0_STAGE_UNSUPPORTED` |
| `debtorMismatchTest` | Existing debtor differs from bound signer and payer | Stage, `S0_STAGE_UNSUPPORTED` |
| `debtorSignerMismatchTest` | Existing debtor differs from bound signer | Stage, `S0_STAGE_UNSUPPORTED` |
| `debtorPayerMismatchTest` | Existing debtor differs from bound payer | Stage, `S0_STAGE_UNSUPPORTED` |
| `signedDebtorScopeTest` | Signed debtor field differs from valid existing debtor | Intent, `S0_INTENT_SCOPE` |
| `signedCreditorScopeTest` | Signed creditor field differs from authenticated creditor | Intent, `S0_INTENT_SCOPE` |
| `suppliedPayerScopeTest` | Supplied payer substituted while bound intent and typed state are valid | Intent, `S0_INTENT_SCOPE` |
| `nonidentityConversionTest` | Signed conversion mantissa is 2 | Intent, `S0_INTENT_SCOPE` |
| `conversionScaleTest` | Signed conversion scale is 1 | Intent, `S0_INTENT_SCOPE` |
| `conversionRoundingTest` | Signed rounding policy is not none | Intent, `S0_INTENT_SCOPE` |
| `invalidRoundAndAliasTest` | Current round is outside signed interval and transfer endpoints alias | Intent, `S0_INTENT_SCOPE` |
| `capAndAliasTest` | Gross cap below debit and transfer endpoints alias | Intent, `S0_INTENT_SCOPE` |
| `overpayAndMissingCreditorTest` | Repayment exceeds outstanding and vector omits creditor credit | Effect, `S0_EFFECT_RANGE` |

Each witness checks the pre-state decision and a blocked submit action. The
shared rejection predicates also check unchanged balances, obligation,
allowance, work, head, replay and last effects where those cells are present.
Quint's `fail()` ends the fixed trace; it does not sample a successor after a
failed action. A blocked action has no committed successor in this model.

## Repair and retained evidence

The repair binds the existing obligation debtor to the immutable bound signer
and payer at Stage and rejects creditor=payer there. Signed party fields and
identity conversion are checked at Intent. Signed validity, caps and scope
precede alias diagnosis. Operation-specific numeric checks now precede the
submitted-vector comparison at Effect in both transfer and repayment.

The [original two-witness result and unapplied proposal](DIVERGENCE-BEFORE-REPAIR.md)
are retained as historical observations. Before model repair, the eleven-trace
expanded corpus reported two passing validity/cap controls and nine failed
ordering expectations. The final corpus adds three signed/supplied party scope
controls. All fifteen original positive and hostile witnesses still execute
successfully; see [RESULTS.md](RESULTS.md).

The observations above are finite executions of this Quint candidate. They do
not establish exhaustive model checking, a general K/Quint or TypeScript
correspondence theorem, Source/6 admission, native proof qualification, or
ledger admission. No `quint verify` was run. This model has no bounded `step`
relation or configured exhaustive invariant. The default Rust evaluator cannot
represent the UInt128 literals; the TypeScript backend was selected explicitly.
