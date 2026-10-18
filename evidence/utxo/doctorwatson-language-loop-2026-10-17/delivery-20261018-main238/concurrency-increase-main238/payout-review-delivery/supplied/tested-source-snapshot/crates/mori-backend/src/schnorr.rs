//! Strict host FENS-JubJub attachment verification, not native certification.
//! Minimal adaptation of Sig Network's Fenrir reference; see
//! `third-party/jubjub-fens/NOTICE.md` for frozen source hashes and licenses.
use crate::{Error, error};
use midnight_curves::pairing::group::{Group, cofactor::CofactorGroup, ff::Field};
use midnight_curves::{Fq, Fr as Scalar, JubjubAffine, JubjubExtended, JubjubSubgroup};
use midnight_transient_crypto::{curve::Fr, hash::transient_hash};

pub const FENS_DOMAIN: u64 = 0x46454e53;

/// Exactly sized lowercase hex. Supplied field integers are never reduced.
pub fn unhex(text: &str, bytes: usize) -> Result<Vec<u8>, Error> {
    if text.len() != 2 * bytes
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(error(
            "signature",
            "Expected exactly sized lowercase hexadecimal.",
        ));
    }
    (0..text.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&text[i..i + 2], 16)
                .map_err(|_| error("signature", "Invalid hexadecimal."))
        })
        .collect()
}

/// Decode a canonical 32LE native field element strictly below p.
pub fn decode_native_hex(text: &str) -> Result<Fr, Error> {
    native(&unhex(text, 32)?)
}
fn native(bytes: &[u8]) -> Result<Fr, Error> {
    Fr::from_le_bytes(bytes)
        .ok_or_else(|| error("signature", "Native field integer is not below p."))
}
/// Canonical 32LE encoding, also for ranged UInt assignments.
pub fn native_hex(value: Fr) -> String {
    mori_core::intent::hex(&value.as_le_bytes())
}
fn point(bytes: &[u8]) -> Result<JubjubSubgroup, Error> {
    let u = native(&bytes[..32])?.0;
    let v = native(&bytes[32..])?.0;
    let d = -(Fq::from(10240) * Fq::from(10241).invert().expect("10241 != 0"));
    let (u2, v2) = (u.square(), v.square());
    if v2 - u2 != Fq::ONE + d * u2 * v2 {
        return Err(error("signature", "Point is off the JubJub curve."));
    }
    let extended = JubjubExtended::from(JubjubAffine::from_raw_unchecked(u, v));
    let subgroup = Option::<JubjubSubgroup>::from(extended.into_subgroup())
        .ok_or_else(|| error("signature", "Point is not in the prime-order subgroup."))?;
    if bool::from(subgroup.is_identity()) {
        return Err(error("signature", "Identity point is forbidden."));
    }
    Ok(subgroup)
}
pub fn decode_key(hex: &str) -> Result<JubjubSubgroup, Error> {
    point(&unhex(hex, 64)?)
}
#[derive(Debug, Clone)]
pub struct SignatureWitness {
    pub r: JubjubSubgroup,
    pub response: Fr,
}
pub fn decode_signature(hex: &str) -> Result<SignatureWitness, Error> {
    let bytes = unhex(hex, 96)?;
    let r = point(&bytes[..64])?;
    let response_bytes: &[u8; 32] = bytes[64..].try_into().expect("exact signature length");
    Option::<Scalar>::from(Scalar::from_bytes(response_bytes))
        .ok_or_else(|| error("signature", "Response integer is not below r."))?;
    Ok(SignatureWitness {
        r,
        response: native(response_bytes)?,
    })
}
/// Affine native coordinates of a decoded subgroup point.
pub fn coordinates(point: JubjubSubgroup) -> (Fr, Fr) {
    let affine = JubjubAffine::from(JubjubExtended::from(point));
    (Fr(affine.get_u()), Fr(affine.get_v()))
}
/// Exact attached bytes checked under the caller's reconstructed message.
/// This establishes a host signature equation, not account-label authority.
pub fn verify_attachment(key_hex: &str, signature_hex: &str, message: Fr) -> Result<(), Error> {
    let key = decode_key(key_hex)?;
    let signature = decode_signature(signature_hex)?;
    let (rx, ry) = coordinates(signature.r);
    let (pkx, pky) = coordinates(key);
    let h = transient_hash(&[Fr::from(FENS_DOMAIN), rx, ry, pkx, pky, message]);
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(&h.as_le_bytes());
    let challenge = Scalar::from_bytes_wide(&wide);
    let response_bytes: [u8; 32] = signature
        .response
        .as_le_bytes()
        .try_into()
        .expect("native32LE");
    let response = Option::<Scalar>::from(Scalar::from_bytes(&response_bytes))
        .ok_or_else(|| error("signature", "Response integer is not below r."))?;
    if JubjubSubgroup::generator() * response != signature.r + key * challenge {
        return Err(error("signature", "FENS signature equation refused."));
    }
    Ok(())
}
