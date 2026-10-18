# Scenario contribution

Five proposed application stories and a single-step invoice worksheet. The `strands` arrays contain illustrative future proof roles, not concrete Strands or three independently available producers. Requirements become Strands only when supported proof instances exist. Arithmetic policy checks may share a whole-execution proof; signatures and validator enforcement are separate.

The worksheet derives expected integers directly from S12 text. It imports no Moriarty evaluator, compiler, SDK, or prover. It admits slider values 1–99 only. Refused boundary rows 0 and 100 preserve hypothetical arithmetic for comparison and have no expected source partial effects.

S12 open(90,10) transfers 110 CASH to escrow and charges buyer 3 CASH, gross debit 113. The 112 mode assumes 2 extra CASH outside the supplied source action. No added funding action is represented as existing.

Treasury units 23 is an arithmetic input. Custody debit is 14 A + 7 A + 2 A fee and 4 B; duties created are 2 A and 6 B. No duty is discharged by allocation.

## Source ownership and exact security wording

The statements below are attributed exact Opus 5.5 errata. They are source-concept corrections, not this contribution’s independent security judgments. V2 descriptions remain readings of unadopted/uncompiled source, not observed behavior. Original USE-01 through USE-05 remain PROPOSED; CL-22 through CL-26 retain their conditions.

### CL-22 · PROPOSED

Batched DEX cases explore order execution and pool arithmetic; consuming validators enforce actual output allocation under a profile-selected policy.

Original reference: [{"studyId": "study-04", "proposalId": "USE-01", "status": "PROPOSED"}, {"sourceId": "S02", "pointer": "/11"}, {"sourceId": "A04", "pointer": "/rows/7", "errataId": "ERR-08"}, {"sourceId": "A04", "pointer": "/rows/22", "errataId": "ERR-23"}]

Conditions: Strict one-output-per-order is one option, not a universal rule. Consumer-recomputed amount partition can admit coalesced payments with each obligation allocated exactly once. A proof only attests arithmetic over values the validator recomputes. Reviewed V2 strict allocation covers one S2 script hash, not other scripts or versions. Real producer/consumer and target evidence required.

### CL-23 · PROPOSED

Capped issuance can be explored with supply change, recipient distribution and separate servicing duties.

Original reference: [{"studyId": "study-04", "proposalId": "USE-02", "status": "PROPOSED"}, {"sourceId": "S03", "section": "What provable intention means"}]

Conditions: Mint/burn semantics and actual policy support unestablished here. No legal-compliance claim. Capacity is not debt.

### CL-24 · PROPOSED

Loan servicing can separate principal, accrual, payment, fee and residual debt.

Original reference: [{"studyId": "study-04", "proposalId": "USE-03", "status": "PROPOSED"}, {"sourceId": "S03", "section": "What provable intention means"}]

Conditions: Illustrative debt105 minus payment35 leaves70; separate fee2 is not discharge. Period/rate/rounding/observation rules and actual consumer required. No ACTUS conformance claim.

### CL-25 · PROPOSED

Treasury allocation source needs non-negative custody23A/4B; full backing25A/10B is a separate proposed policy.

Original reference: [{"studyId": "study-04", "proposalId": "USE-04", "status": "PROPOSED"}, {"sourceId": "S13", "section": "action allocate"}, {"sourceId": "A04", "pointer": "/rows/8", "errataId": "ERR-09"}, {"sourceId": "A04", "pointer": "/rows/20", "errataId": "ERR-21"}, {"sourceId": "A04", "pointer": "/rows/23", "errataId": "ERR-24"}]

Conditions: Source arithmetic: transfers14A+7A and4B, fee2A, duties2A/6B. units is checked arithmetically, not itself debited. Neither S13 nor reviewed V2 source requires liability backing. S13 has no allocation_reserve discharge; nonzero reserve duty blocks V2 Close. V2 signed scenario covers approved input through required signers and optional listed advancer; a distinct governance approver needs its own coverage. V2 statements are attributed unadopted/uncompiled source readings, not observed behavior.

### CL-26 · PROPOSED

Staged invoice escrow can be explored as distinct funding, partial payment, completion, reserve-return and recovery steps.

Original reference: [{"studyId": "study-04", "proposalId": "USE-05", "status": "PROPOSED"}, {"sourceId": "S03", "section": "Partial transactions and conditional settlement"}, {"sourceId": "S04", "section": "Supported subset and refusals"}, {"sourceId": "A04", "pointer": "/rows/21", "errataId": "ERR-22"}, {"sourceId": "A04", "pointer": "/rows/24", "errataId": "ERR-25"}]

Conditions: await not lowered in documented experimental compiler. Source funding110 is sufficient for non-negative custody, while backing112 for one partial is PROPOSED. Further partial fees require further backing funding if that policy is adopted. S12 has no reserve-duty-return action; nonzero reserve duty blocks V2 Close. Delivery evidence retains named assumptions; no complete staged-commerce implementation.

### Opus 5.5 ERR-08

> (a) A consumer-recomputed allocation policy, chosen per profile. Strict option: each output index serves at most one obligation, as V2 already enforces for redeemers of one S2 script hash (R25 global_outputs_distinct, settle_payouts), extended across every script and version in the transaction. Partition option: each output's value equals the sum of amounts allocated to it from distinct obligations plus a declared allowance, with every obligation allocated exactly once; this admits legitimate coalesced payments. Input side: no input's value counts toward two obligations (single consumption is a ledger premise). Claim: Under this profile's allocation rule, no payment can be counted toward two obligations; the transaction cannot charge more than the fee and debit you signed or drop a debt you owe.

Preserved assumption: The double-satisfaction target (S02 [11]); the RR-16 cross-version residual is acknowledged.

Required evidence: Double-satisfaction negatives across two scripts and two S2 versions; a coalesced-payment positive under the partition option.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/7

### Opus 5.5 ERR-09

> (d) A lint for Boolean authorization inputs. In S13 text alone, approved is an action input. In the V2 consumer source (R24 scenario_json; R25 settle_scenario_hash, settle_authority_shape), action inputs are inside the scenario every required signer signs and inside the statement body. So approved=true is consented to by the effect-derived required signers (here the debited vault account) and, where the entry lists advancers, by at least one advancer. That does not show that a distinct approver or governance body authorized it. The lint fires when the approving party is not a required signer, an advancer or an authenticated input.

Preserved assumption: Owner authorization remains a separate objective obligation (S03).

Required evidence: Lint result on S13 under a catalogue whose approver is not a required signer; a genuine signature checked under real context.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/8

### Opus 5.5 ERR-21

> S13 and the V2 consumer require only that a custody vault never goes negative. allocate(true,23,7,2) debits 23 A (14 + 7 transferred, 2 fee) and 4 B, so a custody vault needs at least 23 A and 4 B before the step. Neither S13 nor the supplied V2 source requires custody to back liabilities. Holding at least 25 A and 10 B, so that 2 A and 6 B remain against the reserve and rebate duties, is a PROPOSED full-backing policy that a consumer would have to enforce. units is checked only arithmetically (23 = 3×7 + 2): 21 A is transferred and the remainder of 2 becomes a duty, not a transfer. S13 has no action that discharges allocation_reserve, so under V2 Close a nonzero reserve duty blocks closing.

Preserved assumption: Governance authentication remains required (study-04).

Required evidence: Independent custody and duty accounting; a backing-policy negative.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/20

### Opus 5.5 ERR-22

> S12 executes with escrow funded at 110: partial(35) debits 37, leaving 73, and the source checks only non-negative custody. Against outstanding duties of 75 (invoice 65, reserve 10) that is 2 short. Funding 112 restores full backing for one partial; each further partial call charges escrow another 2 that no duty reflects. This backing is a PROPOSED application policy. S12 has no extra transfer and no reserve-return action. After finish, escrow holds 8 against a reserve duty of 10, and under V2 Close the nonzero reserve duty blocks closing.

Preserved assumption: Delivery evidence and dispute rules remain named assumptions.

Required evidence: Independent open/partial/finish/reserve-return accounting.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/21

### Opus 5.5 ERR-23

> Claim specifications cover order execution and pool arithmetic. Per-order output allocation is enforced by the consuming validator over actual transaction outputs (as V2 does for one S2 script); a proof can attest arithmetic only over values the validator recomputes. The illustration uses a strict one-output-per-order policy; a consumer-recomputed amount partition is the alternative that admits coalesced payments. Which to use needs review.

Preserved assumption: Strand cardinality is unresolved (study-04).

Required evidence: Double-satisfaction negatives; a coalesced positive if partition is chosen.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/22

### Opus 5.5 ERR-24

> Keep, optionally adding: In the proposed V2 consumer the Boolean is covered by the required signers' intents; that is the debited account's consent, not a separate governance decision.

Preserved assumption: Governance authentication remains required.

Required evidence: None.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/23

### Opus 5.5 ERR-25

> Append: Each consumed claim output requires its claimant's signature on the settlement transaction and the burn of its token. Each claim output locks min-ADA from issuance until settlement or post-expiry reclaim; network fees and collateral are funded separately from program-asset fees.

Preserved assumption: Issuance route (S09, unadopted).

Required evidence: None beyond the cited source.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/24

### Opus 5.5 ERR-28

> Use 'thread reserve' for V2 lovelace and 'reserve duty' for program liabilities; 'intent revocation (Revoke)' for seq advance and 'key migration' (PROPOSED) for VK change; 'claim output' for the C2 UTxO and 'claim specification' for a reusable type.

Preserved assumption: No ABI renaming.

Required evidence: None.

Exact source: /home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/security-reviewed/SECURITY-ERRATA.json /rows/27

## Source and scope

Exact links and JSON pointers are attached to each scenario row. Pattern-inventory entries are retained requirements, not compatibility results. No source parent was enumerated, and no private capture, provider, native proof, SDK, ledger, or Git operation was used.

Startup read AGENTS.md and guarded status in /home/charl/Moriarty/.worktrees/utxo. Checkout docs/MORIARTY-PRODUCT-CONTRACT.md was absent; the named public source copy S03 was used. Historical guarded campaign status was read only and not dispatched.

Draft data are frozen for integration. Final-page editorial, security-wording, and browser reviews belong to root’s integration task; this contribution is not their substitute.
