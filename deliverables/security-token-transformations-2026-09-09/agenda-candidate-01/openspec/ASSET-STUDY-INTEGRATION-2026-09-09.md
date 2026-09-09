# Asset-study requirements in the Moriarty roadmap

The [user's integration instruction](../raw/assignments/moriarty-asset-study-integration-2026-09-09.md) makes the recommendations below required remaining roadmap work. The [machine-readable crosswalk](sprints/asset-study.json) assigns stable AS01–AS11 requirements and AT01–AT08 acceptance cases to existing tasks. Read it alongside each affected sprint specification. The MC01–MC08 program remains the acceptance authority.

Source: SRC-0110, [security-token report and graph](../deliverables/security-token-transformations-2026-09-09/README.md). The source report was fully read, but its opaque external citations and claims about current standards, audits, deployments and law remain unverified. Its examples motivate requirements; they do not establish an ERC or Cardano implementation in Moriarty.

## What changes

Moriarty must describe what asset a position represents, what the holder is entitled to, which obligations remain, and what each transformation changes. Token-indexed quantities alone are insufficient for wrapper receipts, pledges, pending redemptions, servicing and exceptional recovery.

| Requirement | Existing implementation location and evidence |
| --- | --- |
| AS01 Asset, position and entitlement | SP01 financial contract and SP02 types; SP08 product profiles and SP12 examples distinguish identifier/domain/unit, holder, obligor, underlying, rights and exit conditions. |
| AS02 Transformation and claim continuity | SP01 independent traces, SP03 reductions/K and SP07–08 libraries preserve input/output rights, fees/rounding, authorization and residual obligations. |
| AS03 Encumbrance and restriction | SP01–03 and SP08 distinguish pledge, settlement hold and freeze, with precise quantity/priority and partial release/liquidation accounting. |
| AS04 Complete operation policies | SP01/SP08/SP09/SP11 cover normal and exceptional transfer, issuance/burn, receipt transfer, redemption, liquidation, recovery and migration paths. |
| AS05 Scoped exceptional authority | SP01/SP02/SP06/SP08/SP09 bind actor, asset, operation, quantity, recipient, time, policy version and reason. A new recovery transition preserves prior history. |
| AS06 Identity, freshness and revocation | SP01/SP03/SP08–10 distinguish key control, credential predicates and beneficial ownership; reject stale/replayed policy evidence under explicit issuer assumptions. |
| AS07 Financial servicing and persistent claims | SP07 owns schedules, capitalization, coupon/record-date/default/redemption semantics; SP08 owns pending requests and residual rights. |
| AS08 Bounded composition and exit paths | SP03/SP08/SP10/SP11 test custody, wrapper, batch, liquidation and migration compatibility under finite position/dependency/nesting bounds. |
| AS09 History, ledger and trust | SP04/SP06/SP09 bind the accepted asset/claim/policy statement. Custody, oracle truth, finality and legal assumptions remain explicit. |
| AS10 Disclosure and developer explanation | SP02/SP08/SP10/SP12 specify private/public fields, diagnostics, complete canonical signing display, administrator powers and exit constraints. |
| AS11 Actual staged target evidence | SP05/SP11/SP12 require Docker first, then separately admitted Midnight Preview evidence, with actual transaction IDs, complete effects and retained duties. |

The preferred implementation is a small bounded semantic model with reusable financial and policy profiles. Do not create a Core constructor merely because a standard names a feature. If an existing operation safely expresses the behavior, reuse it. If a required behavior needs a new primitive, keep it open until its semantics, bounds, lowering and acceptance obligations are reviewed. Arbitrary hooks or unrestricted TypeScript execution are not authorized by this integration.

The source uses compact TypeScript-style syntax; SP02 owns the accepted EBNF, lexical/static rules and diagnostics. SP03 owns Felleisen–Hieb stateful reductions and executable K correspondence. The syntax/DX research branch supplies proposals for review, not a silent change to the accepted language.

## Required acceptance cases

All cases currently remain unimplemented and unaccepted in the complete required scope. The JSON crosswalk supplies each positive and distinguishing invalid case, primary task and contributing tasks.

- [ ] **AT01 Restricted wrapper:** preserve the declared entitlement policy through deposit, receipt transfer and redemption. A successful deposit alone does not prove an available exit.
- [ ] **AT02 Partial pledge and liquidation:** pledge six of ten receipt units, liquidate two, retain four pledged and four unpledged; preserve the independently denominated remaining debt and priority.
- [ ] **AT03 Pending redemption:** retain the unfulfilled claim/duty after request and partial payment; a quoted NAV or token burn is not completed cash redemption.
- [ ] **AT04 Scoped exceptional recovery:** preserve history and encumbrances while enforcing a specific exceptional grant. Reject cross-asset authority and implicit seizure permission.
- [ ] **AT05 Record-date servicing:** pay the entitled holder once under explicit snapshot/event-order rules; test transfers across the record date.
- [ ] **AT06 Stale policy and migration:** preserve identity, claim lineage, authority and work across an authorized migration; reject old-policy replay, duplicate successors and unrestricted abandoned representations.
- [ ] **AT07 Mixed-policy settlement:** every bounded leg satisfies its own policy and the stated atomicity boundary; reject conflicting policies without partial unintended effects.
- [ ] **AT08 Private eligibility:** prove the declared scoped predicate/freshness with the specified disclosure; reject replay, stale credentials and diagnostic identity leakage.

A prior fixture can satisfy a new case only after a reviewed equivalence map covers the complete behavior, state/effects and negative controls. The existing partial/default/full-recovery arithmetic does **not** by itself implement AT02's receipt encumbrance behavior. Record the overlap and the remaining gap.

## Implementation order and Midnight checkpoints

1. Preserve and finish the currently admitted SP05 loan/swap work. Asset-study intake does not restart consumed builds or reopen unrelated accepted source bytes.
2. Add AS01–AS05 to SP01's financial contract/challenge map and canonical signing/display decisions. Start the new challenge with AT02; retain the existing loss/residual-debt case as supporting evidence without claiming equivalence.
3. Carry the accepted case through SP02 typing/authoring and SP03 reduction semantics, K and evaluator with complete post-state/effect comparison. Requalify affected proof statements in SP04/SP06/SP09.
4. After its native implementation and execution admission, add the supported asset-transformation profile to SP05's **Docker positive and rejection tests**. Then run its separately admitted **Midnight Preview checkpoint**. Record actual finalized transaction IDs, fees, ownership, complete effects, remaining claims and cleanup. Source tests, compiler keys and network connectivity do not satisfy this checkpoint.
5. Implement AT01/03/04/06 in SP08, AT05 in SP07, and AT07/08 in SP10 with their contributing tasks. Repeat Docker-to-Preview qualification when newly admitted behavior changes the ledger statement. SP11 closes complete conformance and SP12 releases the readable source/signing examples and reproducible evidence.

SP07 retains first ownership of shared financial Core/K additions. SP08 drafts independent libraries/expectations and integrates those shared files after SP07 review. Assign one writer per shared file. Preserve all existing 277 ACTUS fixtures, 32 dispositions, 18 executable types, 72 DeFi rows, DA01–DA24, eight intents, three held-outs, eight regression classes, five composition operators and TX/VX cases.

## Evidence and decision boundary

This integration makes the work required; it does not claim a semantic freeze, new runtime allocation, accepted schema, proof or financial settlement. Product-specific legal interpretations, wrapper policy inheritance, credential revocation timing, priority rules and any new financial/resource conventions remain explicit design decisions under existing reviews. Primary standard behavior must be tied to an exact source/revision before claiming implementation conformance. Use Scrapling for that acquisition when needed.

Current routing is GPT-6 implementation with fresh independent GPT-6 Astra and Grok 4.6 high audits. Missing reviewers do not approve work. Existing MC/RP admission and acceptance gates remain in force; no SP13 or additional blockchain backend is introduced.
