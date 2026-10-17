# Layer 5 — Proof System

## Translation boundary

Layer 5 explains how a proof system turns a statement and a witness satisfying a compiled relation into a proof that a verifier can check. The book's “sealed certificate” image is a useful entry point, provided the page says what the certificate is about: a proof is tied to a particular relation and statement. In the Webs design, the proof supports a Strand's computation; the consuming validator must still derive the relevant statement from the transaction and spent state. A proof does not authenticate those premises by itself.

The seven-layer model is an explanatory map. Layer 5 choices are coupled to Layer 4 arithmetization and Layer 6 fields and commitments. Chapter 10 describes a dependency DAG, so the site should show these pressures and cross-links rather than imply either a clean pipeline or independent layers.

## Fact-check ledger

- **BOOK_FACT:** Chapter 6 presents the proof system as “sealing” a certificate, surveys Groth16, PLONK and STARKs, and later contrasts recursive proof composition with folding. See `L5-EXACT-CHAPTER.md`, headings “Sealing the Certificate,” “The Three Families,” and “Recursion vs. Folding: Russian Dolls and Snowballs” (source manuscript lines 2430–3014; supplied excerpt lines 3–170).
- **BOOK_FACT:** Chapter 10 calls Layers 4, 5 and 6 an interdependent proof core and describes cross-layer causal dependencies. `B08-EXACT-MANUSCRIPT-SECTIONS.json`, sections “The Causal Web: Why It Is a DAG, Not a Stack” and “The Proof Core” / “The Proof Core Triad.”
- **PRIMARY_SOURCE_SUPPORT:** Groth's supplied paper describes a relation with public statement values and a witness, a prover that creates a three-group-element proof, and a verifier that checks a pairing equation. It gives the proof as two G1 elements and one G2 element, with three pairings in the stated setup. `G05.txt`, pp. 16–18, especially the QAP relation, `Prove`, `Vfy`, and “Efficiency.”
- **PRIMARY_SOURCE_SUPPORT:** The supplied snarkjs guide says Groth16 needs a trusted ceremony per circuit; it separates proof and public-input files and verifies against a verification key. It explicitly warns that an initial zero-contribution key is unsafe for production. `G02.txt`, §§15, 23–24.
- **MORIARTY_MAPPING:** The v2 page copy defines a Strand as a proof, claim, statement and binding metadata; says the validator rebuilds each statement from the transaction and spent state; and says a proof cannot establish that values are real ledger facts. This is product-design copy, not evidence that the complete Web consumer is implemented. `V2-COPY.json`, `/main/proofRoleExplanation`, `/main/definitions`, `/technical/summary`, `/technical/trustNotes`.
- **MORIARTY_MAPPING:** The same supplied copy records one invoice-partial Groth16 roundtrip and one compiled local C2 wrapper with one acceptance and six mutation rejections; contexts were caller-fabricated, the test setup operator could forge proofs, no on-chain transaction is recorded, and the total required setup count is unknown. It says the complete all-or-nothing Web consumer is not yet built. `V2-COPY.json`, `/technical/summary`, `/technical/trustNotes` (`recorded-evidence`, `setup-keys`, `web-completeness`, `page-scope`).
- **INFERENCE:** An interactive page can make the boundary legible by showing (a) the relation and statement a Strand names, (b) what its proof attests, and (c) which facts the consuming validator must get from the transaction or an authenticated outside source. This explains the supplied design without presenting its open implementation pieces as observed behavior.
- **UNRESOLVED:** Opus should approve exact claims about Groth16 setup assumptions, soundness/knowledge soundness, zero knowledge, proof serialization size, and the precise distinctions among aggregation, recursive composition and folding before adoption. The book's generic or illustrative numbers must not be copied into a Moriarty claim without a pinned curve, serialization and statement.
- **UNRESOLVED / BOOK MISMATCH TO CHECK:** In the supplied excerpt, the 192-byte example is specified for BLS12-381, while the adjacent Ethereum gas/precompile comparison names `alt_bn128`/BN254. These are different curve configurations. Keep proof-size and Ethereum verification-cost examples separate unless Opus confirms a precise comparison and wording.

## Proposed page structure

1. Start with the statement–witness–proof–verification relationship, attached to one concrete Strand from the invoice example.
2. Let readers inspect the required Strand set for a Web and remove one to see the design rule for incompleteness. State in the technical section that all-or-nothing enforcement is not yet implemented.
3. Give Groth16 and setup a dedicated visible section. Pair the general explanation with the limited local evidence and unknown total setup count. Avoid extrapolating from a proof byte count to verifier cost or system-wide setup count.
4. Add a small comparison that keeps Web composition (which proof obligations a transition requires and which one proof may cover) distinct from proof aggregation, recursive verification of earlier proofs, and folding of claims. The supplied chapter explains recursion and folding; exact aggregation terminology remains for the security reviewer.

These are copy and interaction proposals only. They do not establish cryptographic security, implementation behavior, setup completion, ledger acceptance, or measured cost.
