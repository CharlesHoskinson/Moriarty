# Compiler repair v1 — source only

Parent-reported first actual native caller build exited101 after38.36s with9 diagnostics. The complete parent log remains ../native-ledger-v3-build.log. Full pre-edit source, manifest, inventories, lock graph and graph receipts are preserved in ../native-ledger-consumer-source-v4-precompile-repair. No receipt is rewritten; absence of a produced binary is not itself evidence of source identity drift.

Source changes address all reported diagnostics:

- Two seal trait errors: pass owned StdRng to Transaction::seal. Official ledger semantics.rs2028 accepts an owned R: SplittableRng; base-crypto/rng.rs declares StdRng's marker and blanket SeedableRng implementation. The owned RNG is consumed after final input/registration signing; no later RNG consumer exists.
- One SHA formatting error: encode each byte of Sha256::digest into lowercase hex instead of formatting the digest array with LowerHex. Current immutable manifest has no direct hex dependency, so an equivalent exact byte encoder preserves manifest, lock and graph. No hash policy or artifact ceiling changes.
- Five moved-value diagnostics: clone the identical PureGeneratorPedersen before each into() in independent verifier statement and mutation checks. Official transient-crypto/commitment.rs134/142 defines a non-Copy Clone value consumed by conversion. Every mutation still uses the same original commitment value.
- One nonexhaustive pattern error: V2 preimage extraction now has an explicit refusal else branch. Official structure.rs230 marks ProofPreimageVersioned non_exhaustive. Unknown versions cannot bypass equality/preparation guards.

Full source repair diff: COMPILE-REPAIR-v1.patch. Dependency files, configs, target and runtime inputs are unchanged. No build/Cargo/Node/check/parameter/SRS/key generation/proof command was run by this author. Repairs are uncompiled. Future compiler diagnostics, actual no-VK export/prepare, complete preimage equality, costs/partition/native skips, actual finalized proof and strict ledger application remain unperformed after this repair. No additional public API gap is evidenced by the current compiler log; this is not a claim that later compilation or runtime stages will pass.
