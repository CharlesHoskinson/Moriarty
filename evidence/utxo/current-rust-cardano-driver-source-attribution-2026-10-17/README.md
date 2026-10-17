# Current Rust Cardano driver source path

A fresh Gemini review checked 34 source claims tracing the CoreReference proposal path through witness mapping, statement construction, the Groth16 call sites and public proof-export construction. It also checked the separate Records branch and the distinction between a Core1 selector check and the compiler's StagedNotLowered branch. All 78 cited literals match the captured source. This establishes source topology, with no execution or cryptographic judgment.

The driver and vendored Rust files are pinned to utxo00ffdca38a30ea98c91a6731e2d6bdb373d16f0b. Three main8174e52975d8db9bf59636fea0fa338c0f47c855 files provide a separate packaging comparison. Main has a root crates/mori-core path; this utxo snapshot has its Core under experiments/moriarty-cardano-rust/moriarty. Both captured Cargo pairs match byte-for-byte, while lib.rs differs in module declarations. Relative Cargo paths were normalized as strings; Cargo resolution and builds were not run.

The previously missing CoreReference caller body is located in this separately pinned packet. The original 61-claim packet and its W05 observation retain their original scope. Twenty input bodies were read fully; the circuit body was reviewed only through line330, covering the cited API/caller ranges. The Core simulator implementation and external dependency bodies are absent from this driver packet.

The review preserves all 20 UTXO families and the financial, ACTUS, DeFi, recursion, private-handoff, multiparent and Preprod obligations as open. It neither approves proof soundness nor establishes an authenticated ledger claim, VM result or transaction. Requested Gemini identity and returned AGY metadata are recorded without independent provider-identity attestation.

Source snapshots retain exact reviewed bytes. CALL-PATH.md has a final blank line; the publication preserves it and records the resulting Git whitespace diagnostic as the sole check exception.
