# Independent R01 design review

**APPROVED for the exact generic JSON design oracle only.** This is a substantive vote for the proposed recourse, impairment, fee and fixed-share loss policy. It does not approve full RP01 or any language, protocol, proof or ledger implementation. Reviewer `/root/rp01_loss_audit` is independent of author `/root/numeric_expectations`; provider-canonical model metadata was not exposed.

Impairment of 600 leaves nominal debt 1000. The collateral sale delivers four units and transfers 400 Cash before discharging 400 principal. Pool pays its own fee 10, leaving NAV 490 and debt 600. The later Borrower payment transfers 150 to Pool and five separately to FeeCollector; principal and allowance both become 450. Pool NAV is 640 and cumulative loss is 460. Holder book values are 384/256 and losses 276/184. These quantities reconcile independently. The remaining zero-carrying-value duty remains Defaulted with recourse, despite exhausted spending authority.

The checker passed the complete trace and all 22 named negative controls. Separate Python integer checks reproduced the three state boundaries, both holders’ values and token conservation. A separate Node harness confirmed that the positive input and all 22 failing inputs remain unchanged. Static review found bounded grant consumption, same-step Cash funding, consumed allocation identities, gross fee effects, exact successor comparisons and recomputed footprints consistent with the proposal. Work ends at ordinary 2, spent 10, reserve 2. Failure means no returned successor; host costs are outside this model.

All nine source digests passed. The original R01 rejection and RP01 requirement support this challenge. The local survey pages 4–5 discuss repayment, liquidation and partial liquidation; they do not establish the proposed recourse or accounting policy. The derivation correctly marks those policies as hypothetical and declines to promote the secondary Morpho discussion to conformance.

No blocking finding in this scoped candidate. The original canonical map is not changed by this review. TX02/TX05/TX10/VX05 remain related obligations: this trace does not test interest-first allocation, close-factor limits, governance transitions or repeated rounding residue. Initial authorization, custody and holder identities are assumptions, and this fixed trace checker is not a general hostile-input/genesis validator. Logical verification units are not a cryptographic cost result. Complete source signing/display, revocation, four-predicate acceptance, full crosswalk coverage and the other required audit remain open.

Candidate bindings:

- `case.json`: `97e4cbf714e8490c0a26b7bd1badf392f8b2246c4c2be530349758343b7287f2`
- `DERIVATION.md`: `231943d241ec7813849ed463e0251eb980860d8d0a3ee1a8ceeb3552268ab35c`
- `check-case.mjs`: `ed0cb754480763af4283f0fef76d2d28b1811086d71ade0b4486e0f956f75b69`
