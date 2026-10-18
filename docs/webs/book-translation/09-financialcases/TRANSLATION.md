# Seven-layer financial examples for Moriarty Webs

This proposal uses the book’s seven names as a learning map for financial transitions. A Web makes a transition’s required proof roles and financial effects inspectable. The layers help readers ask different questions about that transition; they do not assert seven independent components or a linear pipeline. The book itself calls the model interdependent and later describes the proof core (Layers 4–6) and a causal web with pressures in both directions.

## Five website proposals

1. **A connected seven-layer map.** Place the seven layer names around one transition card and let each highlight a question: Was setup needed? How was the rule expressed? What private inputs were used? What constraints encode the rule? What proof claim is checked? Which cryptographic assumptions support it? What does the consumer verify against the transaction? Connect the highlighted layers, especially 4–6, rather than presenting a one-way staircase. Use the five case cards as selectable examples. Keep actual implementation and trust evidence in the technical section.

2. **Invoice: one partial payment, in full.** This is the detailed financial walkthrough and the best place to teach Layers 2–4 and 7 together. Show seller payment, program fee, gross escrow debit, closing cash, invoice duty, reserve duty, and total duties as separate rows. At a payment of 35 CASH, the seller receives 35, the fee recipient receives 2, and escrow pays 37. The 2 fee discharges no debt. The invoice duty falls from 100 to 65; reserve duty stays 10; total duties are 75. With the source’s 110 CASH opening, 73 remains, leaving a 2 CASH backing gap. With a separately labeled proposed 112 CASH funding mode, 75 remains and matches those 75 duties for this one partial. The 112 mode is a design proposal; S12 does not implement its extra funding. Remaining duty is ordinary financial progress, not a missing proof. A whole payment uses the distinct `finish()` action, and each additional partial needs another 2 CASH fee allowance.

3. **Five cases, five financial questions.** Use one case selector that applies the same layer questions without suggesting that each role is an independent proof producer. DEX: do each owner’s 100 A offer and 90 B minimum fit the pool transition, exact rounding, and actual outputs? Capped issuance: do 300 new units stay under the 1,000 cap and reconcile with 180 + 120 distribution, while 700 unused capacity remains distinct from debt? Loan: does 100 debt plus 5 accrual become 105, and does a 35 repayment leave 70 while a 2 fee remains separate? Treasury: does `allocate(true,23,7,2)` satisfy 23 = 3×7 + 2 and match transfers of 14 A + 4 B, 7 A, and 2 A fees, while reserve duties remain visible? The invoice example supplies the fuller payment, fee, and duty trace. These are teaching examples; DEX, issuance, and loan have no supplied source program, and no live prices or observations are present.

4. **Input, witness, and limits view.** For each selected case, let readers reveal the values used by the calculation, which are meant to stay private, which facts need transaction or external authentication, and which duties continue. This connects Layer 3 to Layer 7 without implying that privacy is automatic: a proof concerns values and constraints, while a consumer must bind the statement to its own transaction and any outside observation needs its named source and freshness rule. Keep owner authorization, exact assets, output allocation, and external data visible as separate questions.

5. **Proof-core and readiness panel.** Pair Layers 4, 5, and 6 in one linked panel: the financial rule is represented as constraints; a proof system checks a claim about those constraints; cryptographic choices shape the available representation and proof. Add a Layer 1 setup link and a Layer 7 consumer link. Do not claim a particular production circuit, setup count, proof cost, or cryptographic advantage from the example. A technical status panel can state what is source reading, local evidence, proposed design, and still open, while the main product story explains the intended Web concept directly.

## Invoice walkthrough copy

**One partial payment, in full**

The invoice starts with 100 CASH of invoice duty and 10 CASH of reserve duty. Choose a partial seller payment of 35 CASH. The seller receives 35 CASH, the fee recipient receives 2 CASH, and the escrow’s gross debit is 37 CASH. Only the 35 CASH payment reduces the invoice duty, leaving 65 CASH. The 10 CASH reserve duty remains, so total duties are 75 CASH.

With 110 CASH in escrow, 73 CASH remains after the debit. That is 2 CASH less than the 75 CASH of duties. With a proposed 112 CASH funding mode, 75 CASH remains against 75 CASH of duties for this single partial. The extra 2 CASH is a proposed funding policy, not an action in S12. It does not cover a second partial, which incurs another 2 CASH fee. Paying the whole invoice uses `finish()`, a separate action with different fee behavior.

A payment can reduce a duty while the Web remains complete for the step. The remaining 65 CASH is debt still owed; it does not mean that a proof is missing. Network fees, collateral, proof-claim min-ADA, and the V2 thread reserve are separate from these CASH amounts.

## Fact-check questions for the security owner

- Which claims in the book’s Layer 1 and Layer 6 descriptions are appropriate for this project’s technical status section, and which should remain general examples rather than product claims?
- Does the supplied V2 source reading support mentioning Groth16 here, and what exact qualification must accompany the current forgeable test setup and unknown setup count?
- Which book statements about a single setup, “trustlessness,” proof size, setup ceremony participants, and Layer 4–6 coupling need correction, qualification, or removal before publication?
- What consumer-verification wording accurately separates proof meaning, statement binding, owner authorization, authenticated ledger inputs, and external observations?

## Source distinctions

- **BOOK_FACT:** `B08-EXACT-MANUSCRIPT-SECTIONS.json`, especially “The Seven Layers at a Glance” (lines 272–298), “The Structured Reference String” (396–420), “Universal versus Circuit-Specific Setups” (609–621), “The Proof Core: Layers 4, 5, and 6” (2340–2353), “The Causal Web: Why It Is a DAG, Not a Stack” (4460–4479), and “Trust Decomposition: Seven Weaker Assumptions” (4502–4527). These are claims made by the manuscript, not independently certified facts in this deliverable.
- **PRIMARY_SOURCE_SUPPORT:** The five case amounts and invoice funding modes come from `scenarios.json` and `invoice-fixtures.json`. The supplied `V2-COPY.json` separates intended product copy from implementation evidence and reports current setup limitations, absent transaction/ledger evidence, and the uncompiled status of V2 source readings.
- **MORIARTY_MAPPING:** The case selector and layer questions are proposed educational mappings to the financial examples and Web design.
- **INFERENCE:** A shared case selector and highlighted layer questions could help readers compare concerns while seeing where layers constrain each other.
- **UNRESOLVED:** The exact adopted security and cryptographic language, setup description, production circuit inventory and count, and evidence status remain for the security owner and project technical record.
