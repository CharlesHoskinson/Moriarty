# Editorial review of the revised page

Independent nonauthor prose review, 2026-10-17 UTC. Read current App.jsx, editorial-2026-10-17/COPY.json and active data.generated.json. Inspected the built index.html opening and the desktop-top/mobile-top browser screenshots. No page or Git edits. This review makes no native, cryptographic or security conclusions.

The opening now supplies context and a useful path into the payment example. The desktop and mobile screenshots show readable, natural headings without the old forced fragments. The invoice-first order, later definitions and source-only book attribution address the principal problems in the first audit. Keep that structure. Most remaining work is in the case copy that the first overlay did not replace.

## Priority 1: integrate the remaining case rewrite

`REVISION-CASE-COPY.json` beside this report is a complete replacement `caseCopy` array, ready for the overlay. It preserves all five case IDs and fifteen role IDs. Authentication fields are verbatim. Numeric statement fields are verbatim except the treasury division explanation described below.

The concrete defects it removes include:

- Loan summary: “Service a loan payment with principal, accrual, payment, fee and remaining debt as separate amounts.” Replacement: “A debt of 100 CASH grows by 5 CASH for the period. A 35 CASH repayment reduces it to 70 CASH. The borrower also pays a 2 CASH fee, which does not repay any debt.” The paragraph now follows one calculation.
- DEX claim: “The pool continuation follows the pool’s relation.” Replacement: “The updated pool reserves follow the pool’s rules for fees and rounding.” The action and governing rules are named.
- Treasury statement: “For allocate(true,23,7,2): 23 = 3 × 7 + 2 and 2 < 3.” Replacement: “Dividing 23 by 3 gives 7 with a remainder of 2: 23 = 3 × 7 + 2, and the remainder is less than 3. The program call is allocate(true,23,7,2), where true supplies the approval flag.” The notation follows its meaning. Existing authentication text still explains signed authority and the distinction from a separate governance approver.
- Treasury claim: “The allocation amounts match the quotient.” Replacement: “The beneficiary payments and fee match the allocation calculation.”

These replacements preserve the technical statements and their premises while improving the text a reader encounters first.

## Priority 2: small exact replacements in the current overlay and JSX

1. **Do not call paying the invoice a closing action.** Current `/main/worksheet/amountRangeNote`: “Choose 1–99 CASH for a partial payment. Paying the full invoice uses a separate closing action.” Replace with: **“Choose 1–99 CASH for a partial payment. Paying the full invoice uses a separate action.”** “Closing” invites a beginner to infer that every remaining obligation disappears. This is an editorial terminology correction; it does not assert anything new about execution.

2. **Name the contract’s job consistently.** Current explorer heading: “What does each contract need to prove?” Replace with: **“What must the contract check?”** The page itself assigns proof production to the prover and checks to the validator. The new heading also matches the control text beneath it.

3. **Use the payment as the subject.** Current `/main/worksheet/feeRule`: “The seller’s {amount} CASH reduces the invoice balance. The {fee} CASH fee does not.” Replace with: **“The {amount} CASH payment reduces the invoice balance. The separate {fee} CASH fee goes to the fee recipient.”** This supplies a complete second thought and avoids suggesting the seller is making the repayment.

4. **Make the source comparison result intelligible without the noun “backing.”** Current JSX conditional string: “{gap} CASH backing gap”. Replace with: **“Cash is {gap} CASH short of the remaining obligations.”** Preserve the actual variable interpolation. The default view uses ordinary “cash covers what is owed”; the alternative should use the same vocabulary.

5. **Keep the explorer’s financial result consistent with the worksheet vocabulary.** Current JSX invoice result: “Invoice duty {invoiceDuty} CASH and reserve duty {reserveDuty} CASH continue: total {duties} CASH.” Replace with: **“The unpaid invoice is {invoiceDuty} CASH and the reserve obligation is {reserveDuty} CASH: {duties} CASH remains owed.”** Values stay unchanged. This avoids introducing “duty” again immediately after teaching “still owed.”

6. **Explain why the late catalogue counts appear, or remove them from introductory prose.** Current `/groth16/cardanoCount` sentence: “The catalogue’s 235 review identifiers, 20 pattern families and seven architectural concerns are not circuit counts.” Suggested replacement: **“A list of claim types or application patterns is not an inventory of compiled circuits.”** Keep the surrounding answer that the total requires a frozen circuit inventory. Exact historical counts remain suitable for technical notes; they are unexplained trivia in this beginner answer.

## Lower priority: one remaining terminology bump

`/main/lifecycleSteps/1/body` still asks the reader to absorb “state being spent, its sequence number, the settlement script, the catalogue, signed intents and claimant” in one sentence. This is a legitimate metadata inventory, so it should not be removed merely because it is a list. If another pass is useful, introduce the purpose before the inventory: **“The claim identifies the particular state and transaction it belongs to.”** Retain the full existing binding list in the next sentence or the connected technical explanation. There is no need to rewrite the lifecycle again to address this.

## Review boundary

The upper-page screenshots support the readability observations above; they do not show the entire page or every application state. The built artifact contains the revised hero text and current application copy. The case-array candidate was checked as valid JSON with five cases and fifteen roles. It remains a proposed prose replacement until the integrator applies it and regenerates the page. The initial research report contains the editorial source rationale; this follow-up adds no new web claims.
