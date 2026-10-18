# SECURITY-NOTES: role06 security copy for the Webs and Strands page draft

Status: PROPOSED local draft presentation strings. Authored by role06, requested Opus 5.5 at HIGH effort; actual provider, model and effort are recorded only by the wrapper. Independence from earlier Opus reviews is same-provider fresh context only.

## 1. What this contribution is

- Two files: SECURITY-COPY.json (strings for the React owner) and these notes.
- All security wording is taken from security-reviewed/SECURITY-REVIEW.md (A03) and SECURITY-ERRATA.json (A04), quoted exactly or condensed with every assumption kept. Each card carries its exact errata IDs, matching claim-register rows and, where a SEC06 proposal is mentioned, its PROPOSED status and remaining acceptance evidence, per the application rule.
- It adopts no mechanism and makes no new security conclusion. Genuinely new questions are listed as OPEN referrals in section 6.

## 2. What it is not

- Not a review of any page. No final page source or artifact was supplied to this role. Fresh review of the integrated page remains pending: a nonauthor Muse editorial/result review and a separate Opus review of the exact final security wording. Changed bytes need fresh applicable review.
- Not security approval, implementation, product release, publication approval, operator readiness or native authority.
- No tool, build, test, browser check, proof, VM, ledger, wallet or network action was performed.

## 3. Integration rules for the React owner and other authors

1. Quote a card, rule or string exactly, or omit it. Keep its assumptions, evidenceStage and sourceRefs adjacent or one disclosure away; an accessible expandable section is acceptable, but nothing hover-only.
2. Do not drop an assumption, upgrade a stage label, merge cards into a broader claim or add a security conclusion.
3. Use statusScope.stageLabels for every V2 consumer statement. Claims-register rows whose status says SOURCE FACT but whose conditions say uncompiled-source reading (CL-36, CL-45 to CL-48, CL-50, CL-52) must render as V2_SOURCE_READING, not as observed fact.
4. The maturity box belongs near the hero and in the evidence drawer.
5. Field shapes are content contracts for this draft only. The owner may restructure in app/ source as long as the strings and their adjacency are preserved; record any change for review.

## 4. Card to errata map

| Card | Errata | Claims | SEC06 (PROPOSED) |
| --- | --- | --- | --- |
| proof-meaning | ERR-16, ERR-17, ERR-01, ERR-11 | CL-27, CL-45, CL-49 | P06 |
| recorded-evidence | ERR-18, ERR-17, ERR-05, ERR-19 | CL-11–CL-14, CL-31–CL-34 | P08 |
| setup-keys | ERR-14, ERR-28, ERR-17 | CL-14, CL-32, CL-43 | P07 |
| binding-chain | ERR-02, ERR-03, ERR-13, ERR-07, ERR-16 | CL-36, CL-39, CL-40, CL-45, CL-46 | P03, P04 |
| issuance-route | ERR-04, ERR-06, ERR-07, ERR-12, ERR-25, ERR-26, ERR-27 | CL-02, CL-30, CL-47, CL-50 | P04 |
| web-completeness | ERR-01, ERR-15, ERR-10, ERR-20, ERR-27 | CL-03, CL-15–CL-17, CL-21, CL-37, CL-38 | P01, P02 |
| effects-authority | ERR-08, ERR-09, ERR-10, ERR-23, ERR-24 | CL-22, CL-25, CL-41, CL-48, CL-51 | P05 |
| duties-backing-fees | ERR-21, ERR-22, ERR-25, ERR-28 | CL-19, CL-20, CL-25, CL-26, CL-52 | P05 |

All 28 errata rows remain authoritative. A card that does not cite a row does not relax it.

## 5. Invoice worksheet: worked values (local arithmetic, not execution)

Opening per S12 open(90,10): invoice duty 100, reserve duty 10, escrow 110; buyer gross debit 113 (110 transfer + 3 program-asset fee).

| amount | escrow debit | source cash (110) | PROPOSED cash (112) | invoice duty | total duties | source gap |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | refused: requires amount > 0 | – | – | – | – | – |
| 1 | 3 | 107 | 109 | 99 | 109 | 2 short |
| 35 | 37 | 73 | 75 | 65 | 75 | 2 short |
| 99 | 101 | 9 | 11 | 1 | 11 | 2 short |
| 100 | refused: requires amount < opening (100) | – | – | – | – | – |

Notes:

- The slider allows only 1 to 99 because S12 partial literally requires amount > 0 and amount < opening. Paying the whole balance is the separate finish action, which has no fee call; it is not part of this one-partial worksheet.
- S12 open funds exactly face + reserve, and ERR-22 confirms S12 has no extra transfer. The 112 view is therefore a PROPOSED application policy, not an S12 action, and the source enforces only non-negative custody.
- 112 backs one partial only; each further partial charges another 2 that no duty reflects.
- After finish, the source path leaves cash 8 against reserve duty 10, and the proposed 112 path leaves 10 against 10. Either way S12 has no reserve-duty-return action and the nonzero reserve duty blocks V2 Close (uncompiled-source reading).
- Program-asset fees are separate from network fees, collateral, claim-output min-ADA and the V2 thread reserve, none of which the worksheet models.
- Financial partial progress is not proof incompleteness. The worksheet must never change, or be changed by, the Web explorer's completeness state.

## 6. OPEN referrals (not adopted)

- OR-01 Backing mechanism. If a future program adopts full backing, how it is funded (an explicit fee-budget transfer, a separate fee-reserve duty, or another mechanism) and how a consumer would enforce it is an OPEN design question. It belongs with the SEC06-P05 PROPOSED backing-policy negative and needs Opus security review. This page adopts no mechanism.
- OR-02 Incomplete-state depiction of issued claims. The facts are in ERR-06 and ERR-15. Whether the explorer shows already-issued claim outputs graphically (locked min-ADA, claimant-only release, orphaning after Revoke or competing settlement) is a presentation choice for root. The safe default is the one-line text note in DR-03, with no animated on-chain depiction.
- OR-03 Required-set digest. SEC06-P02 proposes a displayable set digest. It is not adopted; the page must not show one, even illustratively, as if it were an existing artifact.

## 7. Preserved caveat checklist

- Unadopted, uncompiled V2 source readings are not exact-source evidence for the recorded S07 roundtrip or the S08 compiled C2-wrapper (K10) runs. The dispatch also uses the label K6; that label is not defined in the supplied inputs, so these files cite S07 by source ID.
- Source claimant binding at consumption is separate from T3, which is OPEN; the claimant is prover-chosen bytes.
- The class-0 test setup is operator-forgeable.
- A proof creates no authenticated ledger fact and does not make outside observations true.
- Setup is per distinct compiled circuit; the total is unknown; no production ceremony is recorded in the supplied sources.
- Issue is distinct from consumption, claimant settlement signature, burn, liveness (Revoke, competing settlement, orphaning) and min-ADA locking.
- Allocation policy is chosen per profile (strict, scoped to one S2 hash, or amount partition); ledger single-spend is not a value partition.
- Custody, duty, backing and fees are separate: non-negative custody is source behavior, full backing is PROPOSED, reserve duties have no discharge path, and Close is blocked.
- Conditional T1 only; no T2, T3, full WP migration or builder reliance.
- 20 families are requirements; 235 IDs are pre-dedup review identifiers; Midnight remains the broader target; Preview first, then Preprod end-to-end, remain required.

## 8. Flags

- sourceReady: true, only for these local draft security presentation strings with their scope and errata.
- operatorSourceReady, ordinarySelftestReady, authorityProposalReady, nativeRoundtripPrerequisiteClosed, publicRecordExportPrerequisiteClosed, publicVerificationInputsPublishable: false.
- ordinarySelftestScope: none; nothing was run.
- positions: none.
