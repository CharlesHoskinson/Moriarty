// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Uses the pinned stock 0.8.3 graph; terminal validity and gadget soundness remain separate.
//! Small helpers: field hex encoding (32-byte little-endian, the I1 rule),
//! length-prefixed IO, SHA-256.

use std::io::{self, Read, Write};

use ff::PrimeField;
use midnight_proofs::utils::SerdeFormat;
use midnight_zk_stdlib::MidnightVK;
use sha2::{Digest, Sha256};

pub type F = midnight_curves::Fq;

pub fn f_to_hex(x: &F) -> String {
    hex::encode(x.to_repr())
}

pub fn f_from_hex(s: &str) -> io::Result<F> {
    let s = s.trim_start_matches("0x");
    let bytes = hex::decode(s).map_err(io::Error::other)?;
    if bytes.len() != 32 {
        return Err(io::Error::other(format!(
            "field hex must be 32 bytes, got {}",
            bytes.len()
        )));
    }
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&bytes);
    Option::from(F::from_repr(repr)).ok_or_else(|| io::Error::other("non-canonical field element"))
}

pub fn write_bytes<W: Write>(w: &mut W, b: &[u8]) -> io::Result<()> {
    w.write_all(&(b.len() as u32).to_le_bytes())?;
    w.write_all(b)
}

pub fn read_bytes<R: Read>(r: &mut R, max: usize) -> io::Result<Vec<u8>> {
    let n = read_u32(r)? as usize;
    if n > max {
        return Err(io::Error::other(format!("length {n} exceeds {max}")));
    }
    let mut v = vec![0u8; n];
    r.read_exact(&mut v)?;
    Ok(v)
}

pub fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

pub fn write_f<W: Write>(w: &mut W, x: &F) -> io::Result<()> {
    w.write_all(x.to_repr().as_ref())
}

pub fn read_f<R: Read>(r: &mut R) -> io::Result<F> {
    let mut repr = <F as PrimeField>::Repr::default();
    r.read_exact(repr.as_mut())?;
    Option::from(F::from_repr(repr)).ok_or_else(|| io::Error::other("non-canonical field element"))
}

pub fn vk_bytes(vk: &MidnightVK) -> Vec<u8> {
    let mut v = Vec::new();
    vk.write(&mut v, SerdeFormat::Processed).expect("vk write");
    v
}

/// Read a MidnightVK exactly as ledger 9 does (Processed, no trailing bytes).
pub fn vk_from_bytes(b: &[u8]) -> io::Result<MidnightVK> {
    let mut s = b;
    let vk = MidnightVK::read(&mut s, SerdeFormat::Processed)?;
    if !s.is_empty() {
        return Err(io::Error::other("trailing bytes after verifying key"));
    }
    Ok(vk)
}

pub fn sha256_hex(b: &[u8]) -> String {
    hex::encode(Sha256::digest(b))
}

/// Peak resident set size of this process so far, in kB (Linux VmHWM).
pub fn peak_rss_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|n| n.parse().ok()))
        })
        .unwrap_or(0)
}
