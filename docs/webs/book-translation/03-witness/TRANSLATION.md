# Layer 3 — Witness

**Role:** Source-grounded website translation proposal. This is a design contribution, not a security review or evidence that Moriarty generates private proofs for the invoice example.

## Reader-facing explanation

A witness is the data and intermediate values used to show that a computation can satisfy a set of rules. A proof system can let a verifier check a statement about that computation without receiving the witness itself. The boundary depends on the statement, the constraints, and the proof system; calling data a “witness” does not make it private by itself.

For the invoice illustration, a user chooses a partial payment such as 35 CASH. The worksheet can show the corresponding arithmetic: from a 100 CASH invoice duty, 35 goes to the seller, 2 goes to the fee recipient, and 65 of invoice duty remains; the 10 CASH reserve duty remains too. This is an example of inputs and computed values. It is not a claim that these values are currently hidden, proved, authenticated from chain state, or settled by Moriarty. The supplied worksheet is local arithmetic and explicitly does not execute Moriarty, generate or verify a proof, or create ledger state.

A useful explanation separates three things:

1. **User input:** the selected payment amount and any other values an application supplies.
2. **Witness:** candidate values and intermediate results that a prover uses for a particular computation. Which values belong here must be established by the actual relation and implementation.
3. **Statement and constraints:** the claim to be checked, its public inputs, and the rules that relate them. For this illustrative arithmetic, the relation might check that the payment is strictly between 0 and the opening invoice duty, that the fee is accounted separately, and that the unpaid invoice and reserve duties are carried forward. This is a proposed teaching example, not a claim about an implemented invoice circuit.

Information already present in a public transaction, public history, or public outputs does not become private because a proof is involved. A proof about supplied values does not authenticate those values or establish that they came from an outside observation. The exact public/private boundary for any Moriarty Web must be tied to its compiled relation, verifier inputs, data sources, and consumer behavior. Process leakage such as timing is a separate concern from what the proof reveals mathematically.

## Proposed interactive treatment

Use a two-column “Backstage / What the verifier checks” explainer with a **Show example values / Show the boundary** toggle. In the first view, a user-selected 35 CASH flows through the illustrative arithmetic. In the second, the page labels candidate witness values, public statement inputs, and constraints separately, with a conspicuous note that the example values may already be public and that this control changes the explanation only. A separate “What can still be observed?” drawer can name public transaction/history/output data and implementation channels such as timing without implying a finding about the invoice implementation.

The worksheet should keep its existing arithmetic behavior. Do not label its slider a privacy control, proof button, or witness-privacy guarantee. Do not present a complete Web display as evidence that a proof was checked or a transition settled.

## Cross-layer links

Layer 2 determines how the computation and its witness are expressed. Layer 4 explains how rules over witness values become constraints. Layer 5 explains how a proof attests to satisfaction of those constraints. Layer 7 explains what an actual consumer verifies and what transaction data is public. These are related concerns, not a claim that each layer is a separate execution stage.

## Questions for the security fact-check

- For the relevant Moriarty target and relation, which values are private witness inputs, which are public inputs, and which values are public transaction or output data?
- What exact evidence connects a public statement to the spent thread state, catalogue entry, claimant, and transaction? Which facts remain caller-supplied or depend on a source/consumer premise?
- Does the current invoice example have any implemented relation or proof producer? The supplied page materials describe it as illustrative, and the supplied evidence concerns one invoice-partial relation in a different, scoped record.
- Which data can be inferred from transaction shape, public history, outputs, or timing even when the proof itself is zero-knowledge?
- What wording accurately separates mathematical zero-knowledge from process privacy and from authentication of ledger or external facts?

## Evidence labels and limits

- **BOOK_FACT:** Chapter 4 describes a witness as an execution recording, distinguishes a proof’s mathematical privacy from the proof-generation process, and discusses performance, memory, side-channel security, and correctness gaps.
- **PRIMARY_SOURCE_SUPPORT:** The supplied invoice fixture defines a local arithmetic example and its limitations. The supplied V2 copy says a proof over supplied values does not authenticate those bytes or make them an authenticated ledger fact.
- **MORIARTY_MAPPING:** The invoice payment amount and resulting duty arithmetic provide a familiar way to explain candidate inputs, intermediate values, a statement, and constraints.
- **INFERENCE:** A two-column interactive boundary view could help readers distinguish what the prover uses from what a verifier checks.
- **UNRESOLVED:** The exact witness/public-input split and proof guarantees of a deployed Moriarty Web are not established by these supplied files. No claim is made about privacy of public-chain data, history, outputs, or implementation side channels.
