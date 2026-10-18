Moriarty's strict host FENS-JubJub verifier is a minimal adaptation of
Sig Network's Fenrir signature specification and reference verifier from
https://github.com/sig-net/minocrab.git, available under MIT OR Apache-2.0.
The two original license notices are preserved in this directory.

Observed repository HEAD: 9f4d6a62abeb882fc2987d43cd6e0ba0cfb279cd.
Signature files were untracked; this HEAD does not pin their bytes.
The immutable observed-input manifest is `jubjub-frozen-inputs-v2/manifest.json`
in the October 11 native-liabilities runtime evidence. Frozen SHA256 inputs:

| Inspected source | SHA256 |
| --- | --- |
| crates/fenrir-contracts/src/sig/spec.rs | 5005231a3dc84db06b6984d59b189650eb57c9976be4d864ba9d9a698ee73b85 |
| crates/fenrir-contracts/src/sig/reference.rs | 35d30e4c49f1032bb41cf2037bfa8690511d8c3b1b7d9e7835491dbf9678d51e |
| crates/fenrir-contracts/src/sig/zkir.rs | 1b2510a0ddd2495656ba1b4f3ef44f325fc6484759ae9f3bd9dd50fc901627b5 |
| LICENSE-MIT | 2230c498083dc4b7b5f21d3cc0177a8bb1bfa8dd93ce1f8a02ea906f2cc82af1 |
| LICENSE-APACHE | cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30 |

The host verifier adapted from the reference/specification lives in
`mori-backend/src/schnorr.rs` (see its notice); `src/signed/schnorr.rs` only
wraps it in ledger refusal codes.
`src/signed/constraints.rs` adapts the constraint-only asserted verifier from the frozen `zkir.rs`: typed coordinate extraction, nonidentity, FENS-first challenge, canonical scalar roundtrip and asserted equation. It uses only the ledger Builder allocator and emits no auth Impact or private transcript. No signer, custody scheme or Fenrir application is copied.
The implementation depends only on the already locked Midnight crates.
Historical external audit notes do not certify these observed source bytes or
this adaptation. Host verification does not qualify native constraints,
ledger ownership, deployed identity, history or time.
