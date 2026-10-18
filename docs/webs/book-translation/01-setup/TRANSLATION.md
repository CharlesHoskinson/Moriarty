# Layer 1 — Setup: the stage and its provenance

## Product explanation

A proof system may need public mathematical parameters before a prover can create proofs and a verifier can check them. Setup is the work that produces those parameters. The key question is how the parameters were produced, what computation they support, and what evidence lets someone check their origin and consistency.

For a Groth16 circuit, setup is circuit-specific: changing the circuit means producing parameters for the changed circuit. In the supplied snarkjs example, a circuit's R1CS and a prepared powers-of-tau file are inputs to setup; the resulting key artifact is checked against both. The artifact can then yield a verification key. Filecoin's documentation illustrates how a project can publish phase-two parameter files, contribution records, signatures, and verification instructions. These examples explain the idea; they do not establish which setup or artifacts Moriarty or Cardano use.

In Webs, show the provenance connection at the Strand that depends on a parameter set: identify the circuit or compiled artifact, the parameter-set identifier, the proving and verification key identifiers, and the evidence linking the parameters to their source and contributions. A user should be able to follow that connection from a Strand to the relevant source and verification record. Mark missing provenance as unknown rather than implying a ceremony or trust property.

Setup belongs to Layer 1 in the teaching order, but it is not an independent floor or a mandatory first engineering decision. The book describes the layers as interdependent and notes that primitive, proof-system, arithmetization, language, and setup choices constrain one another. The product can introduce Setup first, then link it to the other layers where those dependencies matter.

## Concrete website proposals

1. **A Strand-level setup provenance panel.** For each Strand with a proof dependency, show the named circuit/compiled artifact, parameter-set identifier, proving-key identifier, verification-key identifier, and available provenance records. Show each link's evidence state. This is a product mapping proposal, not a claim that current Strands expose these fields.
2. **An explorable parameter lineage.** Let a reader follow the documented example from circuit description and shared phase-one material through circuit-specific setup, contribution records, final parameters, and exported verification key. Explain that this is a Groth16 example from snarkjs/Filecoin material, not the asserted Cardano or Moriarty pipeline.
3. **A visible setup and Groth16 explainer.** Give setup a dedicated section that explains why Groth16 setup is circuit-specific and what “verify against the circuit” means at a high level. Include a count only when a reviewed inventory names its scope, source, and date; the material reviewed here does not establish a Moriarty/Cardano total.
4. **A trust-model comparison.** Contrast circuit-specific Groth16 setup with a shared/universal setup example, while showing that “shared” does not mean “no setup” and that the setup model depends on the proof system. Keep the comparison labeled as an explanation of the cited examples rather than a statement of Moriarty deployment.

## Source distinctions and review questions

- **BOOK_FACT:** Layer 1 is “The Setup”; the chapter describes parameters as enabling proving and verification, and frames setup trust as part of the system's guarantees (B08, lines 272–298). It describes an SRS as shared parameters and distinguishes an SRS from a proof (B08, lines 396–420). It says Groth16 setup is circuit-specific and describes Midnight's separate universal PLONK-family setup (B08, lines 609–621).
- **PRIMARY_SOURCE_SUPPORT:** The pinned snarkjs README describes a shared phase-one powers-of-tau input, circuit-specific Groth16 setup, checking a zkey against the circuit and powers-of-tau file, and exporting a verification key (G02, “Setup,” “Verify the latest zkey,” and “Export the verification key”). The Filecoin phase-two README describes per-circuit parameter ceremonies and published contribution/attestation files (G04, “Phase2 Ceremonies,” “Phase2 Validation,” and “Description of Trusted-Setup Phases”). These support the examples only.
- **MORIARTY_MAPPING:** Add provenance fields and evidence links to the relevant Strand view. No inspected source established current Strand schemas or actual Moriarty setup artifacts.
- **INFERENCE:** A navigable chain from a Strand to the circuit, setup artifact, keys, and published verification evidence would make setup provenance understandable to Webs readers. The exact UI and field names need product-owner alignment.
- **UNRESOLVED:** Which Cardano proof relation(s) in Moriarty map to which concrete Strands? Which circuit/compiled artifact and parameter set does each use? What are the source, ceremony/transcript, contribution evidence, artifact digests, and key identifiers? Which verification procedures have actually been run, and against which exact circuit/artifact? What count can be stated, and what inventory defines it? Route these fact and cryptographic wording questions to Opus 5.5.

## Limits

This is an explanatory proposal, not a setup audit or security conclusion. The supplied Groth16 references are examples from snarkjs and Filecoin. Do not transfer their setup procedure or provenance to Cardano or Moriarty without primary evidence. Do not infer a total setup count, cite Ethereum KZG participant counts as Groth16 evidence, or assert that any specific Strand has a verified parameter lineage.
