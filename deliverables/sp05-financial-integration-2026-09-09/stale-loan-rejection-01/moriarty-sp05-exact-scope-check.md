# Exact SP05 acceptance scope check

Read-only review, 2026-09-10. No specification change, new gate, allocation or execution authority.

## Two corrections to earlier summaries

**Independent expected results are required. Independently funded wallets or independently operated counterparties are not stated as SP05 requirements.**

- SP05.1, `openspec/sprints/sp05-financial-integration-on-preview.md:36`: “Loan and swap expected records are derived independently of generated outputs.” Here independence concerns the expected financial result.
- MC02 tasks D.1, `openspec/changes/mc02-preview-financial-operation/tasks.md:43`: “Freeze distinct counterparty identities, funding, complete effects, and the package submission reservation.” This requires explicit identities and funding, not independent funding sources or operators.
- Charter, `openspec/MORIARTY-COMPLETION-PROGRAM.md:377`: “Use distinct principals for loan/swap counterparties and wrong-recipient controls, with external private key storage and explicit fixture funding.” Distinct principals are required; separate humans, wallet processes, funding sources or controllers are not specified.
- Charter `:263`: “Use the existing dedicated Preview wallet and externally stored secrets.” MC02 specification `specs/mc02-preview-financial-operation/spec.md:7` likewise explicitly describes “the dedicated wallet” executing the fixture.
- SP05 refinement `:73` names the actual production obligations: asset IDs, participant roles, UTXO/signed-offer ownership, gross debit, fees, change and canonical finality.

Therefore do not turn the latest result’s honest “no independent counterparty funding/signatures” limitation into an extra gate. The actual obligation is to bind distinct financial principals and actual applicable authority/ownership to complete effects and explicit test funding. One controller may manage distinct role capabilities/payout principals in the fixed I2 fixture. A provider signature is not automatically required for a close with no provider-owned transaction input; the actual declared close capability and principal bindings still must hold. A fee-payer signature alone never supplies some other financial authority. General signed-intent/mandatory-proof acceptance remains later work.

The latest independent swap review already verified its one actual unshielded input signature and trader ownership; close has no unshielded input signature (`local-swap-continuation-01/result-review-gpt6.json:27`). It checked program/network/domain/address/capability commitments and independently derived complete participant deltas (`:35`). The loan review verified the borrower-owned gross input and signature, exact lender payout/change (`local-continuation-02/result-review-gpt6.json:44`). Its initialized mint is explicit test funding, not evidence of actual funded loan origination (`:48`). No reviewed normative MC02 requirement demands an economically independent loan origination demonstration for this fixed repayment case.

**A failed local transaction must leave financial state unchanged. Included fallible rollback is not specifically mandated by the reviewed SP05/MC02/I2 text.**

- SP05.2 `:43`: “Test failed transactions leave no financial state mutation.”
- MC02 tasks `:23`: “Run positive and negative comparisons against local Docker before public submission.”
- SP05 refinement `:74`: “Wrong payer, wrong recipient and omitted-fee controls must exercise the callable production path.”
- MC02 specification `:22–23` requires the complete-effect comparator to reject wrong recipient/domain, excess fee, missing debit, extra approval or undeclared write.

Interpretation: an actual constructed transaction rejected by the real local node for its relevant invalid contract/ledger predicate, with retained transaction identity, exact rejection and complete canonical before/after state/assets, can discharge the **node-rejected transaction / no financial mutation case**. No included fallible-segment campaign is required merely because the words “failed transactions” appear. Label the boundary accurately. Transport failure, unsupported RPC, unknown finality or source-only rejection is not that node rejection result. Observe the financial state at anchors that actually cover the attempted operation and establish its rejection status; a snapshot from before the attempt, queried afterward, is not an after-state. Account for any fees separately and avoid claiming future non-inclusion from an unresolved submit response.

This narrow negative case does not prove included rollback, all possible failures, wrong-payer/recipient/fee behavior or SP09 durable one-time consumption. Those distinguishing controls must still be represented at their declared production boundaries. A stale-state rejection is useful but cannot substitute for missing payer/recipient/fee controls.

## Acceptance checklist and current evidence

### SP05.1 — Freeze independently derived fixtures

- Retain accepted atomic source and RP01-MC02 candidate/profile bindings (`SP05:35`; I2 admission `REPORT-RECONCILIATION:101`). Both stage prerequisites are recorded complete; full SP01/full RP01 is not an entry barrier.
- Bind independently derived expected loan/swap state and effects; reject wrong recipient/token/denomination/fee/debt identity (`SP05:35–36`). Current result audits provide substantive existing scoped positive evidence; reuse it.
- Explicitly bind distinct principals, custody/capabilities, actual input and signed-offer ownership, token IDs versus nominal denomination, funding/mint provenance, before-state, gross debit, fees, change, net credit and residual duties (`SP05:35,73`; MC02 D.1; charter377). Reconcile the existing retained identity/funding evidence to these predicates; do not require independent wallets/controllers as a new condition.
- Preserve exact inputs, checks, source hashes and both current scoped reviews; reserve cleanup and public attempts before public submission (`SP05:34–38`). Local consumed allocations authorize no new public run.

### SP05.2 — Exercise Docker settlement

- **Existing scoped positive evidence:** real local loan and swap traces each have all four financial comparisons and GPT-6/Grok result approvals. Preserve the raw inner INCOMPLETE diagnostics and separate outer containment evidence; do not rerun consumed positive campaigns just to obtain another receipt.
- **Remaining actual negative evidence:** demonstrate a real failed local transaction with unchanged complete financial state/assets. The newly reviewed compiled rejection helper is only pre-submit evidence. The finalized-state reader is only source-tested with controlled RPC; neither is live ledger nonmutation.
- Check complete initial/final state and complete receipt decoding against independent records (`SP05:43–44`); preserve negative controls for the comparator and callable production path, including wrong payer/recipient/omitted fee (`SP05:36,74`; MC02 spec22–23).
- Obtain both current candidate-bound result audits for the completed local predicate and retain exact evidence. Existing scoped reviews must not be generalized to unperformed negative cases.

### SP05.3 — Preview integration

- Requires local Docker checks plus MC02-specific live RP03 campaign admission; no native recursive-proof gate is added (`REPORT-RECONCILIATION:101`).
- Use existing Preview identities and explicitly frozen asset/principal/funding/authority mapping. Retain public before-state and exact transaction bytes, attempts and effects; secrets remain external (`MC02 tasks29`; `SP05:51`).
- Enforce actual applicable submission/gross-debit/DUST/cleanup reservations and the specified at-most-two-per-case public attempt rule. Historical charges are not reset. Deploy/failed submissions still count against applicable program reservations (`SP05:51`; charter250–265).
- Actual loan and swap must each have indexed SUCCESS, canonical node finality, exact readback and complete independent financial comparison (`SP05:51–52`; MC02 spec15–23).
- Publish each actual transaction ID/status; keep I2 uncertified; obtain both current result audits and reconcile the package acceptance evidence (`SP05:53–54,75`; MC02 tasks34–39,59–65).

## Sequencing implication

Continue SP05.2 with the smallest real local negative transaction whose node rejection and complete before/after financial effects can be observed under a valid bounded allocation. Reconcile the already-retained principal/funding evidence rather than creating a second-wallet project. Once required local positive/negative production controls and scoped reviews pass, proceed to the separately admitted Preview campaign. An included-fallible rollback experiment is an additional distinguishing result if pursued, not an invented prerequisite of the current wording.
