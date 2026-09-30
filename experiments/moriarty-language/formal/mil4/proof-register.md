# MIL/4 proof register candidate

**Status:** obligations only. No K proof has been run or discharged. Claim domains depend on the Source/6/Core/5 grammar and state signature.

| ID | Claim | Minimum domain and assumptions | Status |
| --- | --- | --- | --- |
| M4-P01 | Determinism and first rejection | One well-formed Core/5 S0 stage, fixed authenticated pre-state, fixed judgment order and code precedence | Open |
| M4-P02 | Progress or defined rejection | Every admitted finite Core/5 S0 term reduces to accepted observation or named rejection | Open |
| M4-P03 | Type preservation | Core/5 formation and reduction preserve nominal assets, domains, widths and result sorts | Open |
| M4-P04 | Finite termination | Bounded constructors, collection sizes, work and strictly decreasing stage measure | Open |
| M4-P05 | Per-asset conservation | Complete effect vector including recipient and fee beneficiary, after alias resolution | Open |
| M4-P06 | Debt discharge is funded | Obligation's bound creditor receives exactly the amount discharged; payer debit is matched | Open |
| M4-P07 | Frame and atomic rejection | Every nonwritten cell is unchanged; local rejection publishes no post-state or effects | Open |
| M4-P08 | Budget and allowance monotonicity | Gross debit, fee and spent counters remain within signed limits, even under endpoint aliasing | Open |
| M4-P09 | History non-reuse | Current head, signed intent replay key and atomic successor prevent duplicate acceptance | Open |
| M4-P10 | Arithmetic bounds | All S0 intermediate additions and repayments respect selected UInt128 and signed nominal bounds | Open |
| M4-P11 | Obligation invariant | `outstanding=principal+accrued`; status is Settled exactly when outstanding is zero | Open |
| M4-P12 | Source/Core elaboration | Every admitted Source/6 S0 form lowers to one typed Core/5 form with preserved signed scope | Open |
| M4-P13 | K/evaluator correspondence | Exact source/Core/K observation equality on a declared finite corpus; a universal claim needs a mechanized bridge | Open |
| M4-P14 | Residual duty conservation | Later episode and accepted-failure stages retain all unpaid duties and reserved closure work | Open |
| M4-P15 | Cross-family effect identity | Each first-profile stage authenticates all pre/post cells and applies its prepared complete effect vector | Open |

The old K runner returns `PROOF_UNIMPLEMENTED`. Existing finite expression, lifecycle and surge results are candidate evidence on their original domains only. The first S0 proof domain will include admitted source/Core inputs, named external signature and snapshot-authentication premises, and an atomic ledger compare-and-consume premise. No theorem may treat those premises as proven by K. Every changed state or transition signature requires review of affected claim domains.
