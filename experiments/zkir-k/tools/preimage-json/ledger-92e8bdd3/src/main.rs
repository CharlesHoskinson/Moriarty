//! Decode a tagged-serialized `ProofPreimage` (as produced by the onchain
//! runtime's `proofDataIntoSerializedPreimage`) into the JSON shape the
//! ZKIR-in-K harness reads: decimal field elements.
//! Usage: preimage-json FILE.bin   (or `-` for stdin)
use std::io::Read;

use num_bigint::BigUint;
use serde::Serialize;
use serialize::tagged_deserialize;
use transient_crypto::curve::Fr;
use transient_crypto::proofs::ProofPreimage;

#[derive(Serialize)]
struct PreimageJson {
    inputs: Vec<String>,
    binding_input: String,
    communications_commitment: Option<(String, String)>,
    private_transcript: Vec<String>,
    public_transcript_inputs: Vec<String>,
    public_transcript_outputs: Vec<String>,
    key_location: String,
}

fn dec(f: &Fr) -> String {
    BigUint::from_bytes_le(&f.as_le_bytes()).to_string()
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut buf = Vec::new();
    if args.len() < 2 || args[1] == "-" {
        std::io::stdin().read_to_end(&mut buf)?;
    } else {
        buf = std::fs::read(&args[1])?;
    }
    let p: ProofPreimage = tagged_deserialize(&buf[..])?;
    let out = PreimageJson {
        inputs: p.inputs.iter().map(dec).collect(),
        binding_input: dec(&p.binding_input),
        communications_commitment: p.communications_commitment.as_ref().map(|(a, b)| (dec(a), dec(b))),
        private_transcript: p.private_transcript.iter().map(dec).collect(),
        public_transcript_inputs: p.public_transcript_inputs.iter().map(dec).collect(),
        public_transcript_outputs: p.public_transcript_outputs.iter().map(dec).collect(),
        key_location: p.key_location.0.to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
