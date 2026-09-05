# S02 installment lifecycle source review

Date: 2026-09-05 UTC. Classification: independent source review. This is a
common-model unit review, not a Council gate or candidate A–D acceptance.

## Identity and source

The implementation author was `/root/execution_adversarial_tests`. A separate
non-author native GPT-6 Astra reviewer, `/root/quint_policy_review`, inspected
the complete three-file unit and the adopted common-design decision. Root also
read the source and independently ran the lifecycle tests and sampled model.
The independent test author `/root/s02_recovery_review` owns the separate
`installment_adversarial_test.qnt`; it is not the implementation author's test.

The reviewed implementation is committed at `8d8e8fe`. Reviewed SHA-256 values:

- `installment_fixtures.qnt`: `e6c54da829a67acef0c9359071363c66924e7ed4c17489fbeaea0281af39f10c`.
- `installment_harness.qnt`: `a0ed856d8e9f1ccd6f7fb0408fde270da341aba4c9a311cf1285d561726920da`.
- `installment_test.qnt`: `0609a869541454f772e7fcfa2b847b466b6241b38e9e28d08b3ec040b32a1af1`.
- Shared `execution.qnt`: `cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928`.

## Source verdict

Clean, with no correctness findings. All 21 actions match the explicit pure
enabledness predicate. There is no blanket stutter. The initial state is unsigned
with prefunded Alice escrow of ten; both profiles execute prepare and sign.
Both initial race contenders must be verified against the same actual context
and parent revision before either can commit. Each successor waits for explicit
stale-loser rejection.

First fill consumes nonce zero while retaining the exact signed parent. Slot two
uses its exact residual. Initial cancellation moves no funds; after a first-fill
winner, fresh cancellation uses the updated parent and evidence. Recovery then
prepares and signs separate nonce one. Actual refund effects read the ledger;
signed policy facts independently require ten or five. Recovery preserves the
cancelled parent and its nonce-zero registry cell.

Conservation, parent paid/escrow/refund coupling, retained recovery authority,
and original verified race-loser evidence are checked by the invariant. Terminal
financial completion additionally requires no pending attempt or signing check.
Cancellation alone and first fill alone are nonterminal.

## Evidence and limits

Full implementation-author development and final outputs are preserved in
`evidence/s02-model-comparison/installment/installment-report.md`. Early scaffold
failures and the shared-source development caveat are explicitly distinguished
from the final stable-source runs. Historical scaffold source was not committed
by the author; its report is not an immutable scaffold-source archive.

The separate adversarial report preserves its exact test command and source pins.
Root found a test-construction issue before final acceptance: slot two was
expected before resolving the stale cancellation. The test was corrected to
respect that boundary. Its recipient mutation also now preserves the attempted
quantity. Neither correction is a product-model defect or implementation RED.

The fixture covers the required initial fill/cancel race and then selects one
successor branch. An additional simultaneous slot-two/fresh-cancellation race is
explicitly outside this fixture. Time remains two; escrow is prefunded. Evidence
validity is a trusted external-verifier abstraction, not effect derivation or
cryptographic proof. No Core interpreter, candidate A–D semantics, independent
correspondence, exhaustive model checking, Council acceptance, or release gate
follows. Exact root runtime receipts are recorded separately from source review.

## Final source and evidence review

The same independent reviewer inspected the separately authored negative tests
and all final receipts at `f4c7bc5`. Verdict: clean, with no source, test-semantic,
or evidence discrepancy. All 18 pins match, and all 11 Quint sources match
`d1c475f86ad0147d0ca1205eab05f2be694cee4f`.

Root receipts support 12 lifecycle tests, nine independently authored negative
tests, and 1,000 sampled executions of `installmentSafety`. All 28 witnesses are
positive. Terminal outcomes are payment after/before 118/135, recovery-ten
after/before 263/249, and recovery-five after/before 124/111; these sum to 1,000.
Fresh Python and local S01 receipts report 286 tests and ten checks respectively.
The development RED limitations and test-construction corrections are accurately
disclosed. No additional runtime tests were performed by this evidence reviewer.
