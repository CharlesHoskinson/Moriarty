# Moriarty Webs editorial research and copy audit

Source review only, 2026-10-17 UTC. No application or Git changes. Working checkout: `<checkout>`.

## Result and limits

The page needs an explanatory narrative, concrete headings and fewer piled-up assertions. Its problem is broader than forbidden vocabulary: nearly every section assumes that the reader already understands transactions, proofs, statements and the Web metaphor. A vocabulary blacklist alone would leave that structure intact.

**Falsifiable editorial predicate:** a newcomer who understands an invoice but has not studied zero-knowledge proofs should be able to read the opening, name what a Web contains, describe a Strand, and explain why the proof and the transaction both matter before reaching the explorer. The heading outline alone should tell that reader what each section teaches.

These are editorial recommendations and repository observations. They are not security judgments. Candidate copy preserves the existing conceptual subject matter but does not establish its correctness or implementation status; existing security wording and proposed/observed distinctions require the assigned security owner's treatment. In particular, edits must not remove a necessary scope label to improve rhythm.

Startup: loaded `moriarty-dev:develop`; read checkout AGENTS and wiki/index before acquisition; guarded status returned the historical blocked SP01 loan-swap campaign. The requested `docs/MORIARTY-PRODUCT-CONTRACT.md` was absent. Inspected the exact five assigned site files. `presentation.mjs` applies exact substitutions, so copy changes must preserve or intentionally update its one-occurrence contract. No native commands, wallet work or provider directories were used.

## Primary sources acquired with Scrapling

Public unauthenticated requests only, Scrapling 0.4.15, static Fetcher. Receipts beside this report contain requested/canonical URL, UTC retrieval, response digest, selector, status and small excerpts. Full copyrighted articles were not retained. Search was used for discovery only; evidence came from Scrapling. Robots were fetched for acquisition domains. No cookies were saved and no new login or site pattern requires persistence.

- **G-TONE:** [Google developer documentation: voice and tone](https://developers.google.com/style/tone). Advice directly relevant here includes avoiding choppy and long-winded sentences, repetitive sentence openings, jargon and distracting cultural references. This is editorial guidance, not a claim that every instance identifies AI authorship.
- **G-AUDIENCE:** [Google Technical Writing One: audience](https://developers.google.com/tech-writing/one/audience). Start from the difference between what the reader already knows and what they need for the task. A developer can still be new to this project's terminology.
- **G-PARAGRAPHS:** [Google Technical Writing One: paragraphs](https://developers.google.com/tech-writing/one/paragraphs). Keep a paragraph on a central topic, establish it early and maintain a readable flow.
- **G-HEADINGS:** [Google developer documentation: headings and titles](https://developers.google.com/style/headings). Headings should describe the topic or task and create a usable hierarchy.
- **NN-WEB:** [Nielsen Norman Group: How Users Read on the Web](https://www.nngroup.com/articles/how-users-read-on-the-web/). The original study supports informative subheadings, one main idea per paragraph and factual web copy. Its age and small early-web setting limit generalization; no study percentage is needed to justify the specific repairs below.
- **LLM-PARALLEL:** [PLOS One: A comparative analysis of syntactic complexity in argumentative essays from rhetorical perspective: ChatGPT vs. English native speakers](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0329410). This study reports more coordination and parallel constructions in its ChatGPT essays. This is a bounded corpus finding, not proof that lists or conjunctions are inherently poor writing. It supports checking the user's observed repetition rather than asserting a universal detector.
- **LLM-GRAMMAR-ABSTRACT:** [Do LLMs write like humans? Variation in grammatical and rhetorical styles](https://arxiv.org/abs/2410.16107). Inspected the authors' abstract, not the full paper: it reports systematic stylistic differences in compared human/LLM corpora and difficulty reproducing human variation. The PMC page returned HTTP 203 with no article body; it was not relied on.
- **Access limitation:** the old GOV.UK writing-guide URL redirected to a response without substantive extractable body. No recommendation below depends on that response. The failed/empty receipt is retained.

## Twenty concrete editorial rules

The bad examples below are exact substrings of the assigned site files or generated data. Candidate rewrites are proposed editorial text, not quotations from sources.

1. **Open with the reader's problem before naming the architecture.** Existing hero: “Bring the required proofs to the transition.” Neither required nor transition has a referent yet. Candidate: **“How Moriarty Webs organize proofs for a Cardano payment”**. Follow with the invoice situation below, then name the design. Basis: [G-AUDIENCE](https://developers.google.com/tech-writing/one/audience).

2. **Define a new project term when it first becomes necessary.** Existing: “accepts the transition only when a Strand covering each one is present”. Candidate: “The design groups the required proofs into a Web. Each proof, together with the statement it supports and the information tying it to this payment, is a Strand.” Basis: [G-AUDIENCE](https://developers.google.com/tech-writing/one/audience). Keep any required proposed-design label adjacent.

3. **Give “transition” an everyday referent.** Existing: “Choose one of five financial transitions.” Candidate: “Choose an action: pay an invoice, exchange assets, issue tokens, repay a loan or allocate treasury funds.” Use “transaction” for an actual transaction and reserve “state transition” for the technical explanation. Basis: [G-AUDIENCE](https://developers.google.com/tech-writing/one/audience).

4. **Replace slogan fragments with headings that name their contents.** Existing worksheet heading: “Pay part. Keep the remainder visible.” Candidate: **“What remains after a partial invoice payment”**. Basis: [G-HEADINGS](https://developers.google.com/style/headings).

5. **Do not use forced line breaks as punctuation for an incomplete thought.** Existing developer heading: “One action.” / “Its complete effects.” Candidate: **“The amounts an exchange order must carry”**. Existing closing: “What moves.” / “What remains owed.” Candidate: **“Try another financial example”**. Allow CSS to wrap a natural heading. Basis: [G-TONE](https://developers.google.com/style/tone).

6. **Make a section heading useful in a heading-only scan.** Existing FAQ title: “Keep the distinctions useful.” Candidate: **“Questions about proofs, payments and settlement”**. The original tells the author what to do rather than telling the reader what is here. Basis: [G-HEADINGS](https://developers.google.com/style/headings).

7. **Do not stack parallel predicates to impersonate explanation.** Existing hero: “Owners sign exact limits on what can leave their accounts, fees stay separate from repayment, and what is still owed stays on the record.” Candidate: “An invoice payment changes more than the seller's balance. The contract also needs to account for its fee and preserve the amount still owed. The owner's signed limits define which changes are authorized.” The second sentence develops the first. Basis: [G-PARAGRAPHS](https://developers.google.com/tech-writing/one/paragraphs); bounded structural concern supported by [LLM-PARALLEL](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0329410).

8. **Preserve useful lists; remove list cadence when the reader needs cause and effect.** Existing seven-layer intro: “financial rules become constraints, a private witness satisfies them, Groth16 seals a proof for each Strand, and a Cardano consumer accepts the transition”. Candidate: “The compiler expresses the program's rules as equations. The prover supplies values that satisfy those equations and produces a proof. The contract then checks the proof's connection to the transaction it is being asked to accept.” Basis: [G-PARAGRAPHS](https://developers.google.com/tech-writing/one/paragraphs). Technical mechanics remain subject to existing reviewed wording.

9. **Do not mistake many short sentences for readable prose.** Existing Groth16 statement explanation: “Together they form the statement. They are not secret.” Candidate: “These public values form the statement that the proof is checked against.” Keep a following sentence explaining the transaction connection; do not turn the whole section into one-liners. Basis: [G-TONE](https://developers.google.com/style/tone).

10. **Connect successive sentences instead of restating the same imperative.** Existing developer copy: “Invoices retain unpaid amounts. Loans retain debt after fees. Treasury allocations retain reserve and rebate duties in their own asset units.” Candidate: “The same handoff must describe what remains owed after the action. For an invoice, that is the unpaid balance; for a loan, it is the debt left after repayment. Treasury allocations may also create reserve or rebate duties.” Basis: [G-PARAGRAPHS](https://developers.google.com/tech-writing/one/paragraphs).

11. **Use concrete descriptions instead of metaphorical accomplishment verbs.** Existing: “Groth16 seals a proof for each Strand”; layer 7: “Proof meets ledger”; layer 6: “Hard problems underneath”. Candidates: “Groth16 produces the proof”; **“Checking a proof against the transaction”**; **“The cryptographic assumptions”**. Basis: [G-TONE](https://developers.google.com/style/tone).

12. **Explain a technical noun before piling on adjacent ones.** Existing first Strand definition: “its claim, its statement and the metadata that binds it to a single step: the state being spent, its sequence number, the settling script, the catalogue, the signed intents and the claimant.” Candidate first sentence: “A Strand contains a proof and identifies the particular payment that proof supports.” Put the exact binding inventory in an immediately available technical disclosure. Basis: [G-AUDIENCE](https://developers.google.com/tech-writing/one/audience). Do not delete technical obligations from the full definition.

13. **Give an interactive control a prediction before asking the reader to operate it.** Existing: “Remove a role's Strand to see how the consuming contract treats an incomplete Web.” Candidate: “Select a proof requirement to read what it checks. Then omit it: the illustration will show that the required set is incomplete.” The action and observed page result are explicit. Basis: [NN-WEB](https://www.nngroup.com/articles/how-users-read-on-the-web/). Preserve the approved distinction between illustration and proof verification.

14. **Use UI state language that describes the UI's actual observation.** Existing assembly status begins “Complete Web. Every required Strand is present, so the transition can go to settlement”. Editorial candidate: “All illustrated proof requirements are included.” Keep the designated security owner's actual scope sentence with this state. This recommendation concerns comprehensibility and visible context, not a substitute acceptance judgment. Basis: [NN-WEB](https://www.nngroup.com/articles/how-users-read-on-the-web/).

15. **Explain ordinary financial vocabulary before adopting project vocabulary.** Existing: “100 CASH of invoice duty and 10 CASH of reserve duty”. Candidate: “The buyer still owes 100 CASH on the invoice. The program also records a separate reserve obligation of 10 CASH; this example calls those amounts duties.” Explain escrow as the account holding the money. Basis: [G-AUDIENCE](https://developers.google.com/tech-writing/one/audience).

16. **Keep source credit out of the instructional lead.** Existing seven-layer kicker: “Proving Nothing · seven layers”; intro begins “Proving Nothing describes every zero-knowledge system through seven layers”. Candidate kicker: **“How the system works”**; intro: “A payment proof depends on several pieces of work. These seven views explain the program, the calculation, the proof and the contract that uses it.” Keep the original attribution and pinned reference in Sources. Basis: [G-TONE](https://developers.google.com/style/tone). Preserve the seven-layer content rather than replacing it with a book discussion.

17. **Name each layer in language a newcomer can connect to its job.** Existing selector entries: “Arithmetization”, “Cryptographic Bedrock”. Candidate visible labels: **“Equations”** with “Arithmetization” as secondary terminology, and **“Cryptographic assumptions”**. Existing heading “Every rule becomes an equation” is a clearer starting point than the selector label. Basis: [G-HEADINGS](https://developers.google.com/style/headings).

18. **Explain the reader's reason to care before the formal mechanism.** Existing setup opening: “Groth16 derives proving and verifying keys from secret randomness for one compiled circuit.” Candidate lead: “A Groth16 proof is checked with keys prepared for the exact circuit being proved. Reusing that circuit lets later payments reuse its keys.” Then explain randomness and its assumptions in the existing designated trust text. Basis: [NN-WEB](https://www.nngroup.com/articles/how-users-read-on-the-web/).

19. **Replace algebraic labels in introductory controls with reader terms.** Existing counter title: “Setup stages for K compatible circuits”; label: “Distinct compiled circuits (K)”. Candidate title: **“How the setup count changes”**; label: **“Number of distinct circuits”**; initial explanation: “Assume these circuits can share the same first setup phase. Each still needs its own second phase.” Keep the exact compatibility and trust conditions beside the result. Basis: [G-HEADINGS](https://developers.google.com/style/headings).

20. **Delete invented abstractions that repeat the surrounding point.** Existing: “Make the financial handoff explicit.” and “Use the same financial terms from the first application input through the artifacts delivered to the consuming contract.” Candidate: **“What the application sends to the contract”**. Body: “The application must supply the action's amounts, the owner's signed limits and the required proof information. The table shows where each item is used.” Basis: [NN-WEB](https://www.nngroup.com/articles/how-users-read-on-the-web/).

## Proposed beginner narrative

Recommended progression: payment problem → Web/Strand terms → worked invoice → proof requirements → lifecycle → developer artifacts → seven technical layers → Groth16 setup → implementation evidence. Keep the explorer accessible from the hero, but do not make its terminology the reader's first lesson.

Candidate opening, to be reconciled with the existing reviewed design-status wording:

> Suppose an application pays 35 CASH toward a 100 CASH invoice. The seller must receive 35, the fee recipient must receive 2, and the record must show that 65 is still owed on the invoice. If the program also has a reserve obligation, that remains part of the next state.
>
> A financial contract needs evidence that those changes follow its rules. Moriarty Webs is a design for organizing that evidence for Cardano applications. The contract defines which proofs an action requires; the collection is a Web. A Strand contains one of those proofs, its statement, and the information tying it to the intended action.
>
> The proof checks a calculation. The contract must also connect that calculation to the transaction's actual inputs, outputs and authorization. This guide follows the invoice through those two jobs, then shows how the same approach applies to exchanges, token issuance, loans and treasury allocations.

This opening intentionally leaves exact assembly/settlement claims to the existing reviewed scope language. It teaches the nouns before expecting the visitor to interpret the Web diagram.

Suggested headings in reading order:

1. How Moriarty Webs organize proofs for a Cardano payment
2. A payment has several requirements
3. What remains after a partial invoice payment
4. From owner authorization to settlement
5. What the application sends to the contract
6. Seven parts of the proof system
7. How Groth16 keys and setup are reused
8. Questions about proofs, payments and settlement
9. Implementation status and evidence

## Review procedure for the revision

Read only the heading outline first. It should explain the page without prose beneath it. Next read the opening aloud and mark each unexplained project noun. For each paragraph, write its single point in the margin; split or remove material when no single point emerges. Underline coordinated predicates and repeated sentence openings, then retain only those that help comparison. Finally, operate each illustration and verify that visible text describes the action actually taken on the page. This is an editorial check; the assigned owner retains security review of statement scope and status wording.

No rule bans conjunctions, lists, short sentences, metaphors or parallelism categorically. The concrete test is whether the form helps this reader understand this particular financial example.
