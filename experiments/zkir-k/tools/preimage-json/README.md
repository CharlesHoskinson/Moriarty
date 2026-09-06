# preimage-json

Decodes a tagged-serialized `ProofPreimage` (the bytes returned by the onchain
runtime's `proofDataIntoSerializedPreimage`) into the JSON the harness reads:
`inputs`, `binding_input`, `communications_commitment`, `private_transcript`,
`public_transcript_inputs`, `public_transcript_outputs` as decimal strings, plus
`key_location`.

Versioned copy of the crate registered as a workspace member of
`~/Moriarty/repos/_build/ledger-92e8bdd3` (one line added to that workspace's
`Cargo.toml`, after `zkir-circuit-oracle`). Build:

    cd ~/Moriarty/repos/_build/ledger-92e8bdd3 && cargo build --release -p preimage-json --offline

Binary: `target/release/preimage-json FILE.bin` (or `-` for stdin). Used by
`tools/moriarty_preimages.mjs`.
