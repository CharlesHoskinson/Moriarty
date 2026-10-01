//! Experimental native verification; never financial or ledger qualification.
pub mod codec;
#[cfg(feature = "native-proof")]
pub mod financial_transfer;
pub mod intent;
pub use codec::{canonical_message, wallet_message};
use midnight_base_crypto::{ecdsa, schnorr};
use midnight_serialize::Deserializable;
/// Verify exact raw signature/key encodings through Midnight's native Rust APIs.
pub fn verify(scheme: &str, message: &[u8], key: &[u8], sig: &[u8]) -> Result<bool, String> {
    if sig.len() != 64 {
        return Err("signature must contain exactly64 bytes".into());
    }
    match scheme {
        "schnorr_bip340" => {
            if key.len() != 32 {
                return Err("Schnorr key must contain exactly32 bytes".into());
            }
            let vk =
                schnorr::VerifyingKey::deserialize(&mut &key[..], 0).map_err(|e| e.to_string())?;
            let s = schnorr::Signature::deserialize(&mut &sig[..], 0).map_err(|e| e.to_string())?;
            Ok(vk.verify(message, &s))
        }
        "ecdsa_secp256k1_sha256" => {
            if key.len() != 33 {
                return Err("ECDSA compressed key must contain exactly33 bytes".into());
            }
            let vk =
                ecdsa::VerifyingKey::deserialize(&mut &key[..], 0).map_err(|e| e.to_string())?;
            let s = ecdsa::Signature::deserialize(&mut &sig[..], 0).map_err(|e| e.to_string())?;
            Ok(vk.verify(message, &s))
        }
        _ => Err("unsupported signature scheme".into()),
    }
}
