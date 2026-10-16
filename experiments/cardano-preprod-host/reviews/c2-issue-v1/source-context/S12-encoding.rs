//! Canonical BLS12-381 Groth16 encodings for the Cardano C2 consumer, and strict decoders.
//!
//! Derived from the pinned sources, not from memory:
//! - points: `ark-bls12-381 0.4.0 curves/util.rs:16-72`, `g1.rs:102-134`, `g2.rs:136-177`.
//!   Compressed G1 is 48 bytes, G2 96 bytes; big-endian x (G2: `x.c1 || x.c0`); flag bits in the
//!   first byte: 7 compressed, 6 infinity, 5 lexicographically largest y. This is the Zcash
//!   layout that the Plutus `bls12_381_*_uncompress` builtins read (C2, `groth16.ak`).
//! - VK and proof: field order of `ark-groth16 0.4.0 data_structures.rs:6-42`, `Vec` length as
//!   u64 little-endian (`ark-serialize 0.4.2 impls.rs:527-540`). VK = alpha G1, beta G2,
//!   gamma G2, delta G2, `03 00 00 00 00 00 00 00`, ic0, ic1, ic2 = 488 bytes; proof = A G1,
//!   B G2, C G1 = 192 bytes (`groth16.ak:15-20,52-69`).
//!
//! Strictness beyond arkworks: arkworks' compressed reader returns the identity whenever the
//! infinity bit is set and ignores every other bit (`util.rs:107-109,166-168`). Every decoder
//! here re-encodes and requires byte equality, checks on-curve and subgroup membership, and
//! refuses identity points in a VK (all seven) and in a proof (all three). C2's F-I profile
//! refuses identity only for gamma, delta, ic1 and ic2, so the driver accepts a subset of what
//! C2 accepts; it never accepts more.
use crate::error::DriverError;
use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

pub const G1_LEN: usize = 48;
pub const G2_LEN: usize = 96;
pub const VK_LEN: usize = 488;
pub const PROOF_LEN: usize = 192;
/// Offset of the `gamma_abc_g1` length prefix in a VK (C2 `slice_bytearray(336, 8, vk)`).
pub const VK_IC_OFFSET: usize = 336;
/// u64 little-endian 3: constant term plus two public inputs (C2 `vk_ic_count`).
pub const VK_IC_PREFIX: [u8; 8] = [3, 0, 0, 0, 0, 0, 0, 0];

/// Compressed G1 identity: `c0` followed by 47 zero bytes (C2 `g1_identity_compressed`).
pub fn g1_identity_compressed() -> [u8; G1_LEN] {
    let mut b = [0u8; G1_LEN];
    b[0] = 0xc0;
    b
}

/// Compressed G2 identity: `c0` followed by 95 zero bytes (C2 `g2_identity_compressed`).
pub fn g2_identity_compressed() -> [u8; G2_LEN] {
    let mut b = [0u8; G2_LEN];
    b[0] = 0xc0;
    b
}

fn enc(what: &str, detail: impl std::fmt::Display) -> DriverError {
    DriverError::Encoding(format!("{what}: {detail}"))
}

pub fn encode_g1(p: &G1Affine) -> Result<[u8; G1_LEN], DriverError> {
    let mut out = [0u8; G1_LEN];
    p.serialize_compressed(&mut out[..]).map_err(|e| enc("G1 serialization", e))?;
    Ok(out)
}

pub fn encode_g2(p: &G2Affine) -> Result<[u8; G2_LEN], DriverError> {
    let mut out = [0u8; G2_LEN];
    p.serialize_compressed(&mut out[..]).map_err(|e| enc("G2 serialization", e))?;
    Ok(out)
}

/// Strict compressed G1: exact length, valid flags, coordinate below p, on curve, in the
/// prime-order subgroup and canonical (re-encoding gives the same bytes).
pub fn decode_g1(bytes: &[u8], what: &str) -> Result<G1Affine, DriverError> {
    if bytes.len() != G1_LEN {
        return Err(enc(what, format!("expected {G1_LEN} bytes, got {}", bytes.len())));
    }
    let p = G1Affine::deserialize_compressed(bytes).map_err(|e| enc(what, e))?;
    if !p.is_on_curve() || !p.is_in_correct_subgroup_assuming_on_curve() {
        return Err(enc(what, "point not in the prime-order subgroup"));
    }
    if encode_g1(&p)?[..] != bytes[..] {
        return Err(enc(what, "non-canonical encoding"));
    }
    Ok(p)
}

/// Strict compressed G2, as [`decode_g1`].
pub fn decode_g2(bytes: &[u8], what: &str) -> Result<G2Affine, DriverError> {
    if bytes.len() != G2_LEN {
        return Err(enc(what, format!("expected {G2_LEN} bytes, got {}", bytes.len())));
    }
    let p = G2Affine::deserialize_compressed(bytes).map_err(|e| enc(what, e))?;
    if !p.is_on_curve() || !p.is_in_correct_subgroup_assuming_on_curve() {
        return Err(enc(what, "point not in the prime-order subgroup"));
    }
    if encode_g2(&p)?[..] != bytes[..] {
        return Err(enc(what, "non-canonical encoding"));
    }
    Ok(p)
}

fn is_identity<A: AffineRepr>(p: &A) -> bool {
    p.xy().is_none()
}

/// SettlementHashV2 VK profile: exactly three IC points and no identity point at all.
pub fn vk_profile_ok(vk: &VerifyingKey<Bls12_381>) -> Result<(), DriverError> {
    if vk.gamma_abc_g1.len() != 3 {
        return Err(enc("verifying key", format!("{} IC points; SettlementHashV2 needs 3", vk.gamma_abc_g1.len())));
    }
    let g1 = [("alpha", &vk.alpha_g1), ("ic0", &vk.gamma_abc_g1[0]), ("ic1", &vk.gamma_abc_g1[1]), ("ic2", &vk.gamma_abc_g1[2])];
    for (name, p) in g1 {
        if is_identity(p) {
            return Err(enc("verifying key", format!("{name} is the identity")));
        }
    }
    let g2 = [("beta", &vk.beta_g2), ("gamma", &vk.gamma_g2), ("delta", &vk.delta_g2)];
    for (name, p) in g2 {
        if is_identity(p) {
            return Err(enc("verifying key", format!("{name} is the identity")));
        }
    }
    Ok(())
}

/// The 488-byte C2 VK encoding. The manual layout is cross-checked against ark-groth16's
/// derived serialization of the same value.
pub fn encode_vk(vk: &VerifyingKey<Bls12_381>) -> Result<Vec<u8>, DriverError> {
    if vk.gamma_abc_g1.len() != 3 {
        return Err(enc("verifying key", "SettlementHashV2 needs exactly 3 IC points"));
    }
    let mut out = Vec::with_capacity(VK_LEN);
    out.extend_from_slice(&encode_g1(&vk.alpha_g1)?);
    out.extend_from_slice(&encode_g2(&vk.beta_g2)?);
    out.extend_from_slice(&encode_g2(&vk.gamma_g2)?);
    out.extend_from_slice(&encode_g2(&vk.delta_g2)?);
    out.extend_from_slice(&VK_IC_PREFIX);
    for p in &vk.gamma_abc_g1 {
        out.extend_from_slice(&encode_g1(p)?);
    }
    if out.len() != VK_LEN {
        return Err(enc("verifying key", format!("encoded {} bytes, expected {VK_LEN}", out.len())));
    }
    let mut derived = vec![0u8; vk.compressed_size()];
    vk.serialize_compressed(&mut derived[..]).map_err(|e| enc("verifying key", e))?;
    if derived != out {
        return Err(enc("verifying key", "manual layout differs from the ark-groth16 derived serialization"));
    }
    Ok(out)
}

/// Strict 488-byte VK decoding: fixed offsets (the length prefix is compared, never trusted
/// as an allocation size), strict points, profile, and whole-key round trip.
pub fn decode_vk(bytes: &[u8]) -> Result<VerifyingKey<Bls12_381>, DriverError> {
    if bytes.len() != VK_LEN {
        return Err(enc("verifying key", format!("expected {VK_LEN} bytes, got {}", bytes.len())));
    }
    if bytes[VK_IC_OFFSET..VK_IC_OFFSET + 8] != VK_IC_PREFIX[..] {
        return Err(enc("verifying key", "IC length prefix is not 3 (u64 little-endian)"));
    }
    let mut ic = Vec::with_capacity(3);
    for (k, at) in [344usize, 392, 440].into_iter().enumerate() {
        ic.push(decode_g1(&bytes[at..at + G1_LEN], &format!("vk ic{k}"))?);
    }
    let vk = VerifyingKey::<Bls12_381> {
        alpha_g1: decode_g1(&bytes[0..48], "vk alpha")?,
        beta_g2: decode_g2(&bytes[48..144], "vk beta")?,
        gamma_g2: decode_g2(&bytes[144..240], "vk gamma")?,
        delta_g2: decode_g2(&bytes[240..336], "vk delta")?,
        gamma_abc_g1: ic,
    };
    vk_profile_ok(&vk)?;
    if encode_vk(&vk)? != bytes {
        return Err(enc("verifying key", "non-canonical encoding"));
    }
    Ok(vk)
}

/// The 192-byte C2 proof encoding, cross-checked against the derived serialization.
pub fn encode_proof(proof: &Proof<Bls12_381>) -> Result<Vec<u8>, DriverError> {
    let mut out = Vec::with_capacity(PROOF_LEN);
    out.extend_from_slice(&encode_g1(&proof.a)?);
    out.extend_from_slice(&encode_g2(&proof.b)?);
    out.extend_from_slice(&encode_g1(&proof.c)?);
    let mut derived = vec![0u8; proof.compressed_size()];
    proof.serialize_compressed(&mut derived[..]).map_err(|e| enc("proof", e))?;
    if derived != out || out.len() != PROOF_LEN {
        return Err(enc("proof", "manual layout differs from the ark-groth16 derived serialization"));
    }
    Ok(out)
}

/// Strict 192-byte proof decoding; identity points are refused.
pub fn decode_proof(bytes: &[u8]) -> Result<Proof<Bls12_381>, DriverError> {
    if bytes.len() != PROOF_LEN {
        return Err(enc("proof", format!("expected {PROOF_LEN} bytes, got {}", bytes.len())));
    }
    let proof = Proof::<Bls12_381> {
        a: decode_g1(&bytes[0..48], "proof A")?,
        b: decode_g2(&bytes[48..144], "proof B")?,
        c: decode_g1(&bytes[144..192], "proof C")?,
    };
    if is_identity(&proof.a) || is_identity(&proof.b) || is_identity(&proof.c) {
        return Err(enc("proof", "identity point"));
    }
    Ok(proof)
}

/// Proving key: arkworks compressed serialization (custody material, not a public export).
pub fn encode_proving_key(pk: &ProvingKey<Bls12_381>) -> Result<Vec<u8>, DriverError> {
    let mut out = vec![0u8; pk.compressed_size()];
    pk.serialize_compressed(&mut out[..]).map_err(|e| enc("proving key", e))?;
    Ok(out)
}

/// Proving key decoding with arkworks validation (on-curve and subgroup for every point),
/// exact length and the VK profile.
pub fn decode_proving_key(bytes: &[u8]) -> Result<ProvingKey<Bls12_381>, DriverError> {
    let pk = ProvingKey::<Bls12_381>::deserialize_compressed(bytes).map_err(|e| enc("proving key", e))?;
    if pk.compressed_size() != bytes.len() {
        return Err(enc("proving key", "trailing bytes after the key"));
    }
    vk_profile_ok(&pk.vk)?;
    Ok(pk)
}
