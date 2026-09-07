> For the complete documentation index, see [llms.txt](/llms.txt)

# Zero-knowledge proofs

A zero-knowledge proof (ZKP) lets someone prove knowledge of a secret without revealing the secret itself. For example, a ZKP can prove that an attribute lies within a range without revealing its exact value. A client of a dApp or service can use ZKPs to selectively disclose information from a self-sovereign identity while keeping other attributes private.

For example, consider a person voting in a local election. They must reside within the electorate, be on the voter roll, and not have voted already. In a conventional process, they may need to show documents that disclose their home address, date of birth, and full name. In addition, each voter is marked on the roll as they vote, which can introduce coercion risk under poor governance.

In a zero-knowledge setting, using a digital ID and an appropriate application, the citizen can prove that their address is within the required area, they are registered to vote, and they have not voted already, without disclosing personally identifiable information.

In general, ZKPs can prove statements such as:

* A company is not on a sanctions list
* A customer is over 28 years of age
* A customer has purchased over €1m of product in the past year.

In cases like these, the assertion can be proven without revealing other information.

## ZK Snarks[​](#zk-snarks "Direct link to ZK Snarks")

Midnight uses zero-knowledge succinct non-interactive arguments of knowledge (zk-SNARKs), a class of ZKPs designed for compact proofs and efficient verification.

Two key properties are:

* **Succinctness**: Proof size remains small relative to the size of the statement, which enables efficient verification and reduced data transfer.
* **Non-interactivity**: The prover can generate a proof without back-and-forth interaction with the verifier, unlike interactive proof systems that require multiple communication rounds.

### How do ZK Snarks work?[​](#how-do-zk-snarks-work "Direct link to How do ZK Snarks work?")

Because ZK Snarks are non-interactive arguments of knowledge, they let a prover demonstrate a statement's validity without ongoing interaction with a verifier. Proof generation and verification use advanced cryptographic constructions, including elliptic-curve-based techniques.

The high-level process looks as follows:

![zk-snarks](https://ucarecdn.com/bacf5357-29d7-490e-8b13-ed13c671efaf/)

The process typically includes:

* **Setup phase**: The system establishes public parameters used by the proving and verification algorithms. These parameters are critical to scheme security and correctness.
* **Key components**: ZK Snarks use specialized cryptographic components (for example, elliptic-curve-based constructions and hashing primitives) to enable compact proofs.
* **Circuit construction**: The statement is encoded as an arithmetic circuit that defines operations and constraints.
* **Witness and proof generation**: The prover uses the private witness, the circuit, and public parameters to generate a proof that the statement is valid without revealing witness data.
* **Verification**: The verifier uses the proof, public parameters, and statement to efficiently check validity.

## Related comparisons[​](#related-comparisons "Direct link to Related comparisons")

* [ZK vs FHE vs MPC](/concepts/zk-vs-fhe-vs-mpc.md): how zero-knowledge proofs compare to fully homomorphic encryption and secure multi-party computation.
* [zk-SNARK vs zk-STARK](/concepts/zk-snark-vs-zk-stark.md): a deep dive into the two main zero-knowledge proof systems.
* [Fully homomorphic encryption](/concepts/fully-homomorphic-encryption.md): how FHE works and how it compares to the zero-knowledge approach.
