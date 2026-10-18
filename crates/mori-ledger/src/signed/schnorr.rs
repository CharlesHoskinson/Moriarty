//! Strict host FENS-JubJub attachment verification, not native certification.
//! The verifier is `mori_backend::schnorr`; ledger callers see each of its
//! refusals as `strict-fens-refusal`, without the backend's message text.
use crate::{Error, Result};
use midnight_curves::JubjubSubgroup;
use midnight_transient_crypto::curve::Fr;
use mori_backend::schnorr as fens;

pub use fens::{FENS_DOMAIN, SignatureWitness, coordinates, native_hex};

fn refused(_: mori_backend::Error) -> Error {
    Error::new("signature", "strict-fens-refusal")
}

/// Exactly sized lowercase hex. Supplied field integers are never reduced.
pub(crate) fn unhex(text: &str, bytes: usize) -> Result<Vec<u8>> {
    fens::unhex(text, bytes).map_err(refused)
}

/// Decode a canonical 32LE native field element strictly below p.
pub fn decode_native_hex(text: &str) -> Result<Fr> {
    fens::decode_native_hex(text).map_err(refused)
}

pub fn decode_key(hex: &str) -> Result<JubjubSubgroup> {
    fens::decode_key(hex).map_err(refused)
}

pub fn decode_signature(hex: &str) -> Result<SignatureWitness> {
    fens::decode_signature(hex).map_err(refused)
}

pub fn verify_attachment(key_hex: &str, signature_hex: &str, message: Fr) -> Result<()> {
    fens::verify_attachment(key_hex, signature_hex, message).map_err(refused)
}
