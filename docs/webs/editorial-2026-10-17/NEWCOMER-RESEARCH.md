# Newcomer audit and editorial recommendations

Research and source inspection: 2026-10-17 UTC. Scope: newcomer comprehension, section structure and interaction wording. No application or Git changes. All recommendations below are editorial judgments, not empirical usability findings.

## Result and falsifiable acceptance predicate

A Cardano developer who has never read *Proving Nothing* should be able to explain a Web, a Strand, the relationship between a proof and a transaction, and the difference between a circuit and a proof after reading the page's main path. The introduction must supply the transaction example before requiring the reader to understand the proof machinery. Reading only the headings should reveal the subjects covered.

## Evidence and limits

Repository observations come from the exact current `docs/webs/app/src/App.jsx`, `SevenLayers.jsx`, `Groth16.jsx`, `WebDiagram.jsx`, `presentation.mjs`, and the content those components render in `data.generated.json`. This is a source-based editorial audit; I did not run a participant study or inspect rendered screenshots. The guarded status was read; its historical loan-swap execution blockers do not block this read-only language task. `wiki/index.md` was read before public acquisition. The installed develop skill's requested `docs/MORIARTY-PRODUCT-CONTRACT.md` is absent in this checkout. This report does not establish cryptographic correctness, implementation acceptance or deployment.

Public source facts, acquired with Scrapling 0.4.15:

- Google Technical Writing's [Audience](https://developers.google.com/tech-writing/one/audience) distinguishes a reader's general technical role from familiarity with a particular project. It recommends supplying the knowledge the reader needs while accounting for unfamiliar vocabulary. Applied recommendation: a Cardano developer can know transactions and still need Web, Strand, witness and claim explained.
- Google's [Documents](https://developers.google.com/tech-writing/one/documents) recommends giving essential answers at the beginning and organizing a document around the reader's needs. Applied recommendation: explain the financial transaction before presenting its dependency framework.
- The GOV.UK Design System's [Details](https://design-system.service.gov.uk/components/details/) guidance reserves disclosure for information only some users need and recommends clear, descriptive disclosure text. Applied recommendation: keep the proof-versus-ledger distinction visible; disclose raw evidence, source comparisons and detailed premise authentication.
- W3C WAI's [Headings](https://www.w3.org/WAI/tutorials/page-structure/headings/) tutorial describes headings as the organization of the document and as navigation for assistive technologies. Applied recommendation: replace motivational headings with subjects and keep the hierarchy meaningful.

Exact raw HTML, scoped text, URLs, retrieval timestamps, HTTP status and SHA-256 digests are in this directory, with `receipts.json` and reproducible `acquire.py`. Four public unauthenticated pages were fetched; no login, browser escalation, cookie persistence or access bypass was used. The static-document pattern already exists in the Scrapling skill's site-pattern record, so there is no new general pattern or cookie to save. No external private directory was inspected.

## Where the current explanation loses a newcomer

1. The hero starts with “transition,” then introduces consuming contracts, conditions, Strands and rebuilt statements before giving a concrete action. The word “transition” should first mean a transaction changing contract state; subsequently the technical term has a referent.
2. The definition strip is comprehensive enough to function as a reference, but its first screen requires claim, statement, metadata, sequence number, settling script, catalogue, signed intent and claimant. Those terms are introduced inside definitions of other unfamiliar terms. Split introductory definitions from binding metadata.
3. “Give every proof a defined job,” “Pay part. Keep the remainder visible,” “Make the financial handoff explicit,” and “Keep the distinctions useful” are editorial commands. They do not tell a scanning reader which subject the section explains. “One action. Its complete effects.” and the closing “What moves. What remains owed.” repeat the same cadence without supplying information.
4. The explorer alternates among role, Strand and proof. Three visible roles look like three independently issued proofs, although the text permits one whole-execution proof to cover multiple roles. Explain that distinction before the first diagram. Removing a role is also misleading: the contract's requirement stays fixed; its proof coverage is what becomes absent.
5. The page hides almost the entire proof/transaction distinction inside “How the proof and transaction fit together.” The visible excerpt says what a proof does but omits the essential check that binds that proof to actual submitted outputs. Keep that binding in the ordinary explanation.
6. The seven-layer introduction names *Proving Nothing* as if its purpose were already familiar. It also traverses constraints, witness, Groth16 and consumer in a single sentence. Introduce the book as a source of an architectural framework, identify the framework's purpose, then introduce the seven named concerns. Preserve its layer numbering, while explaining that the numbers are references rather than an execution sequence.
7. The Groth16 section answers setup counts before explaining why the reader encounters setup at all. Start with proof production and verification keys, then explain why those keys depend on the compiled circuit. The assumed circuit counter should clearly separate circuits, proofs, setup stages and ceremonies.
8. “Source comparison · 110 CASH” is an archival distinction presented as a normal financial choice. A newcomer cannot know what “source” means. Keep source provenance in the implementation area; if both controls remain in the worksheet, label them by the financial consequence.

## Recommended reading order

| Position | Section heading | Purpose |
|---|---|---|
| Hero | Webs and Strands for Cardano transactions | Name the system and explain its financial task |
| 1 | An invoice payment and its required checks | Anchor the abstractions in a 35 CASH payment |
| 2 | What a Web contains | Define Strand, Web and reusable claim specification |
| 3 | How the contract checks a Web | Explain required coverage and transaction binding; offer five cases |
| 4 | Payments, fees and remaining duties | Explore partial-payment arithmetic |
| 5 | From owner authorization to settlement | Show the four separate events |
| 6 | Building an application with Webs | Explain developer inputs, artifacts and consuming contract |
| 7 | Seven parts of a zero-knowledge system | Introduce the book framework and connect the pieces |
| 8 | Groth16 proofs and circuit setup | Explain statement, witness, keys, setup and counts |
| 9 | Questions about proofs and settlement | Answer focused questions |
| 10 | Implementation status and evidence | Place actual tooling, evidence and source comparison together |

The first two sections can be compact blocks inside the hero/explorer area; a full visual redesign is unnecessary. If section order must stay unchanged, still introduce the invoice in the hero before the definition strip and replace the existing headings as below.

## Candidate introduction and definitions

These are proposed replacement copy, to be checked against the authoritative technical copy before integration.

**Page title:** Webs and Strands for Cardano transactions

**Introductory paragraph:** A financial contract needs to check both a payment and the obligations that remain afterward. Moriarty lets an application prove the calculations before the contract settles the transaction. A Web collects the proofs the contract requires and binds them to the transaction being settled.

**Concrete example:** Take an invoice with 100 CASH left to pay and a 10 CASH reserve duty. A 35 CASH payment to the seller costs the escrow 37 CASH because the program also charges a 2 CASH fee. The invoice then has 65 CASH left to pay, while the 10 CASH reserve duty continues. The contract checks those calculations and the transaction's actual outputs before accepting the change.

**Audience sentence:** This guide explains that process for Cardano application developers, from the required checks to on-chain settlement.

**Web:** A Web is the complete set of Strands a contract requires for one state change. The contract accepts the change only when every required check has proof coverage and the statements match the transaction.

**Strand:** A Strand is one proof packaged with the claim it proves, the statement being checked and metadata that binds it to a particular contract step.

**Claim specification:** A claim specification defines a reusable kind of check, including its subject, premises and relation. Each new step still needs a proof for its own statement.

**After the definitions:** One proof can cover several required checks. The diagram separates the checks so you can inspect them; it does not prescribe one proof per displayed role.

**Visible binding explanation:** The proof checks the calculations in a statement. The settling validator rebuilds that statement from the submitted transaction and the state being spent, then checks that it matches the statement proved. This is how a valid calculation is tied to the actual payments and continuing duties.

Put the exact binding fields—state reference, sequence, settling script, catalogue, signed intents and claimant—in “How a Strand identifies its contract step.” The fields remain important but are a second-level explanation.

## Headings and transitions for the current component order

| Current heading | Replacement |
|---|---|
| Bring the required proofs to the transition. | Webs and Strands for Cardano transactions |
| Give every proof a defined job. | The checks a financial transaction requires |
| Pay part. Keep the remainder visible. | Payments, fees and remaining duties |
| One partial payment, in full | Calculate a partial invoice payment |
| From signed intent to settled transition | From owner authorization to settlement |
| Make the financial handoff explicit. | Building an application with Webs |
| One action. Its complete effects. | What a DEX order must specify |
| Keep the obligations visible. | Record the obligations that continue |
| Seven layers of a Moriarty Web | Seven parts of a zero-knowledge system |
| How many Groth16 setups a Web needs | Groth16 proofs and circuit setup |
| Keep the distinctions useful. | Questions about proofs and settlement |
| What moves. What remains owed. | Inspect another financial transaction |

**Explorer introduction:** Choose a financial transaction to see the checks its contract requires. Select a check to read its statement, or remove its proof coverage to see why an incomplete Web cannot settle.

**Explorer to worksheet:** The invoice case separates the seller's payment from the fee and the amount still owed. Change the payment below to see those amounts update together.

**Worksheet to lifecycle:** These calculations describe the financial result. The next section follows the separate transactions and signatures needed to settle it.

**Lifecycle introduction:** Owners authorize the financial limits first. The prover produces the required evidence, Issue transactions record the checked proofs on-chain, and a settlement transaction applies the financial change. The application then confirms the result from the ledger.

**Lifecycle to developer handoff:** An application carries the same asset identities, limits and obligations through each stage. These are the inputs and artifacts the developer supplies.

**Developer handoff introduction:** Start with the financial action and the limits its owners approve. Compile the required checks, produce the Strands and provide the Web with the transaction to the consuming contract.

**Seven-layer introduction:** A zero-knowledge application combines a programming language, a way to express its rules as equations, a proof system and an on-chain verifier. The book *Proving Nothing* organizes these design concerns into seven layers. Here, each layer explains one part of a Moriarty Web and how it depends on the others. The layer numbers follow the book; they are not a sequence of transaction steps.

**Seven-layer to Groth16:** The proof-system layer uses Groth16. Its proving and verification keys are specific to a compiled circuit, which determines how setup is shared or repeated.

**Groth16 introduction:** Groth16 lets a prover show that a statement satisfies a compiled program's constraints. A proving key produces the proof, and a verification key checks it. Setup creates those public keys for one exact compiled circuit. Transactions that use that same circuit can reuse its keys while producing proofs for their own statements.

**Counter introduction:** Choose how many distinct circuits share a compatible curve, parameter format and size limit. One shared phase 1 supports a separate phase 2 for each circuit. This counter counts setup stages for that assumption.

**FAQ introduction:** These answers cover authorization, proof claims, settlement and the duties a financial action leaves behind.

**Implementation introduction:** This section records the available commands, observed results and remaining implementation work. It also contains the source comparisons and test-setup trust assumptions behind the examples.

## Interaction and disclosure wording

| Current label | Suggested label | Reason |
|---|---|---|
| Explore | Financial examples | Identifies destination |
| Follow the value | Invoice calculator | Names the interaction |
| Seven layers | System architecture | Gives a reason to visit before knowing the book |
| Groth16 & setup | Proofs and setup | Defines topic before cryptographic name |
| Explore the five cases | Inspect a financial example | Gives a concrete action |
| Required roles / SELECT A ROLE | Required checks / Select a check | Uses an introductory noun |
| Inspect a proof role | Selected check | Corresponds to diagram selection |
| Statement | What the proof checks | Introduces the technical term in the accompanying text |
| Context | Where the inputs come from | Explains why this text matters |
| Omit this required role | Remove proof coverage for this check | Requirement stays in force |
| Restore it to complete the required set. | Restore proof coverage to complete the Web. | Names the effect |
| Complete Web. Every required Strand is present… | Every required check has proof coverage. The contract still checks the statements against the settlement transaction. | Distinguishes completeness from acceptance |
| Incomplete Web… | A required check has no proof coverage, so the contract cannot settle this transaction. | States the financial consequence |
| Source comparison · 110 CASH | Duties funded · fee allowance missing (110 CASH) | Explains what selecting it changes |
| Funded for one partial payment · 112 CASH | Duties and one payment fee funded (112 CASH) | Parallel financial labels |
| How the proof and transaction fit together | How the validator binds a proof to the transaction | Names the specific explanation; core binding remains visible |
| Draws on layers | Depends on | Short relation, layer names visible nearby |
| What this layer produces | Output of this layer | Concrete heading |
| Setup stages for K compatible circuits | Setup stages for compatible circuits | Avoids an unexplained variable in heading |
| Distinct compiled circuits (K) | Number of distinct compiled circuits | Plain quantity |
| The Cardano-wide count | Counting setups across applications | Removes implied known inventory |
| Sources / Source notes | Sources for this section | Clear location and content |

Use one on-page convention in the technical area: this guide's controls explain the design and calculate example amounts; they do not submit transactions or generate proofs. Do not repeat implementation-status caveats in every section. Keep protocol trust requirements near the concepts they qualify: they explain the system, rather than the development stage.

## Essential claims the rewrite must retain

These are observed requirements of the current page's technical account. Preserve them exactly in meaning; this editorial audit does not independently prove them.

- A Web is the contract-defined complete required collection for one transition. Missing required coverage gives no authority over the contract's effects; completeness alone does not establish transaction acceptance.
- A Strand contains one proof, its claim, statement and binding metadata. A role is a required check, and one whole-execution proof can cover multiple roles. Neither an intent nor a UTxO nor a catalogue entry is itself a Strand.
- A proof establishes its relation over the statement's values. It does not authenticate those values as ledger facts. The validator reconstructs statements from the submitted transaction and predecessor state, checks the issued digest/binding and enforces actual financial outputs. External facts retain issuer, authentication and freshness requirements.
- Owner authorization precedes proving and preserves per-asset gross debit caps, separate fee caps, liability caps, net receipt goals, validity and nonce. Refunds do not cancel gross debit limits; fees do not repay debt.
- Each Strand's proof is checked in a separate Issue transaction that creates its proof-claim output. Application-asset issuance is a distinct minting policy. Settlement consumes the required claims, requires claimant signatures and burns claim tokens. Unused claims hold min-ADA until authorized settlement or claimant reclaim after expiry.
- The payment example retains 100 CASH initial invoice duty, 10 CASH reserve duty, 2 CASH program fee, and 112 CASH default escrow funding. For payment p, seller receives p, escrow debit is p+2, invoice duty becomes 100−p and reserve stays 10. CASH is an example unit; network fees/collateral/min-ADA are separate ADA amounts. A remaining duty is compatible with a complete Web.
- Groth16 uses circuit-specific proving and verification keys, both public. New values within an unchanged relation reuse setup; changes to the compiled relation/bounds/public-input layout require the corresponding new setup. Proof count, user count, role count, catalogue count and book-layer count do not determine setup count.
- In the compatible two-phase construction, K distinct circuits use one shared phase 1 and K circuit-specific phase-2 stages. This counts stages, not K+1 complete ceremonies. Compatibility includes curve, format and capacity. Setup randomness must be destroyed; each multiparty phase requires the stated honest-participant and verified-transcript assumptions. Wrappers also contribute circuits.
- The seven-layer framework is attributed to *Proving Nothing* at its pinned source, while the Moriarty mapping is this guide's interpretation. The layers are interdependent design concerns, not seven sequential runtime stages.
- Inclusion, application-required finality and readback of actual effects remain separate observations. A timeout leaves the outcome unknown. Permissionless developer access remains explicit.

## Review predicates for the integrated page

1. A reader encounters a concrete payment before a sentence requiring knowledge of a Strand, catalogue or witness.
2. Every main heading names its section's topic; no main heading relies on paired sentence fragments or an imperative about the author's intentions.
3. Web and Strand are defined once in plain terms before the first interactive use; detailed binding fields remain available below.
4. Required roles/checks and proof instances are distinguished before diagram interaction. Omitting coverage never changes the contract's requirement.
5. The ordinary visible copy explains that submitted transaction outputs and predecessor state supply authenticated premises.
6. A reader can identify *Proving Nothing* as the source book, understand why the framework is useful, and use the layer panel without reading it first.
7. The setup counter explicitly counts compatible compiled circuits and distinguishes setup stages from ceremonies.
8. One technical implementation-status area contains actual execution evidence and tooling limits. Protocol conditions remain precise; repetitive development caveats do not interrupt the explanatory path.
