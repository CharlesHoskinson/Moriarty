# Independent RP01 revised-case review

**APPROVED for the fixed partial design case.** This is a substantive independent GPT-6 Astra design vote, not full RP01 acceptance. No blocking defect was established within the declared three-step domain. This reviewer did not author or previously review this case and did not read other independent reviews; the author's disposition was inspected.

Exact candidate pins:

- Case: `74e3a5506a775c260ed5471a273343bb0a8b840c3f628a7a83b4ca818a2edd93`.
- Derivation: `4a86e4ac3b0394f8746c5efa8cbedc4950181fbc5b8ed44abf5edda1b172db0a`.
- Checker: `38c985ce97cb53f4d86fbeb7148f7b6dde3e932b50ad05d98f79b0af169af16b`.

The offline checker passed all 3 steps, 34 rejected mutations, 2 direct conservation controls, 1 ordinary-work positive boundary and 1 domain exclusion. It checked input immutability on success and every rejection, verified 9 local source hashes and used 35/64 logical verification units. I independently recomputed the accounting and token totals using integer/Fraction arithmetic.

The financial sequence is consistent. Impairment 600 preserves nominal debt 1000. Actual 400 Cash sale proceeds reduce debt to 600; Pool pays its own fee 10. Later Borrower pays 150 to Pool and 5 to FeeCollector; only 150 reduces principal, with a matching 150 impairment reversal. Final Pool cash/NAV is 640, remaining nominal recourse and allowance are 450 each, carrying receivable is 0 and current netloss is 460. Holder book values 384/256 and loss allocations 276/184 sum correctly. Cash remains 700 and Collateral remains 4. Book loss is not erased by token conservation or hidden by zero carrying value.

The authority repair checks the actual repayment grant denomination, payee and duty against both the obligation and executed same-step funding. Consistent grant asset/payee substitutions now reject AUTHORITY. Source grants and external observations remain assumed records, not authenticated authority. Gross borrower debit 155 is kept distinct from nominal recovery 150; exhausted grants are retained and not replenished by residual cash.

**I agree with the author on the reserve dispute.** The model has 12 ordinary units plus a separate 2-unit reserve, totaling 14. Ten events leave 2 ordinary and 2 reserve. The positive 10 ordinary boundary leaves 0 ordinary while retaining 2 reserve; the ordinary-shortfall control still rejects despite available reserve. Subtracting that reserve from ordinary remaining again would charge it twice. The pinned repayment-kernel work section independently uses the same additive convention.

The chosen recourse, impairment reversal, fee incidence and 600/400 pari-passu policy form a coherent hypothetical design case. They are not established accounting/legal rules or Morpho/other protocol conformance. Local survey pages 4-5 and secondary bad-debt material motivate distinctions without supplying this policy. The source provenance is appropriately limited.

**RP01-LOSS-FULL-RECOVERY remains REQUIRED**, owned by SP01, SP02, SP03, SP07 and SP09. It must establish actual further 450 Cash funding and new bounded authority, retain Loan1 at zero with coherent discharged status, reconcile impairment/cash/fees/history and test funded discharge separately from row deletion. Borrower currently has 45 Cash and an exhausted debit grant; no 450 payment may be invented. The present zero-duty scope exclusion is not a funded complete-recovery demonstration and closes no full RP01 gate.

No compiler, wallet, network, proof or generated-language execution was performed. This fixed-case checker does not implement a general closed financial language schema; dedicated branch controls and authenticated observation/custody/admission evidence remain limited as disclosed. Source/Core/K/Compact correspondence, actual rollback, network fees, protocol policy and mandatory proofs remain open. The guarded operational-history stop is unchanged.
