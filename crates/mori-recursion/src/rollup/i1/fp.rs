// Adapted from the pinned Fenrir B2 oracle; see ../sources.json.
//! Field, hash, domains and encodings.
//!
//! `F` is the BLS12-381 scalar field (the Midnight circuit field).
//! In `midnight-curves` this type is exported as `Fq`.
//! `JFr` is the Jubjub scalar field, exported as `Fr`.
//! `H` is the Poseidon sponge hash of `midnight-circuits`
//! (`PoseidonChip` as `HashCPU`), which is the same function the
//! ledger exposes as `transientHash` (see transient-crypto `hash.rs`).

use ff::PrimeField;
use midnight_circuits::{hash::poseidon::PoseidonChip, instructions::hash::HashCPU};

pub type F = midnight_curves::Fq;
pub type JFr = midnight_curves::Fr;

pub const CM_DEPTH: usize = 32;
pub const NF_DEPTH: usize = 32;
pub const RH_DEPTH: usize = 16;
pub const WD_DEPTH: usize = 20;
/// Root-history window (DL-23, ID-118 `W_RH`, R6-AW-002, R6-AW-007): exactly
/// `W_RH` anchors are usable, `idx >= rh_count - W_RH`, and the recording
/// header's authenticated time must be at most `RH_RETENTION_MS` before the
/// applying header's. One root is recorded per committed header. These are the
/// ID-118 planning values (1 h over 6 s headers = 600 headers); a ledger takes
/// both values from the parameter registry and refuses to start when either is
/// unset (ID-49).
pub const RH_RETENTION_MS: u64 = 3_600_000;
pub const HEADER_PERIOD_MS: u64 = 6_000;
pub const W_RH: u32 = (RH_RETENTION_MS / HEADER_PERIOD_MS) as u32;
/// Flat fee for every non-deposit transaction: 0.01 USD in micro units.
pub const FLAT_FEE: u64 = 10_000;
/// Synthetic USD token id (6 decimals). NIGHT is only a reserved id here.
pub const TOK_USD: u64 = 1;
pub const TOK_NIGHT: u64 = 2;
/// Genesis time (unix seconds). Header h has time T0 + 6*h.
pub const T0: u64 = 1_760_000_000;
pub const HEADER_SECS: u64 = 6;

pub fn header_time(height: u64) -> u64 {
    T0 + HEADER_SECS * height
}

/// The one hash function used everywhere in this lane.
pub fn hash(input: &[F]) -> F {
    <PoseidonChip<F> as HashCPU<F, F>>::hash(input)
}

pub fn f_u64(x: u64) -> F {
    F::from(x)
}

/// Pack an ASCII tag into one field element: tag bytes as a
/// little-endian integer (byte 0 is least significant).
pub fn field_from_tag(tag: &str) -> F {
    let bytes = tag.as_bytes();
    assert!(bytes.len() < 31, "tag too long");
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut()[..bytes.len()].copy_from_slice(bytes);
    F::from_repr(repr).expect("tag value below modulus")
}

/// Domain separators. The STRAWMAN I-DOM names are packed with
/// `field_from_tag`; the two tree domains keep the numeric values
/// of the spike (`fenrir-recursive-rollup/src/trees.rs`) so the
/// tree logic is unchanged.
pub mod dom {
    use super::{F, f_u64, field_from_tag};
    pub fn node() -> F {
        f_u64(0x4652_4e4f_4445)
    }
    pub fn nf_leaf() -> F {
        f_u64(0x4652_4e55_4c4c)
    }
    pub fn cm() -> F {
        field_from_tag("DOM_CM")
    }
    pub fn nf() -> F {
        field_from_tag("DOM_NF")
    }
    pub fn nk() -> F {
        field_from_tag("DOM_NK")
    }
    pub fn key() -> F {
        field_from_tag("DOM_KEY")
    }
    pub fn sig() -> F {
        field_from_tag("DOM_SIG")
    }
    pub fn txmsg() -> F {
        field_from_tag("DOM_TXMSG")
    }
    pub fn tx() -> F {
        field_from_tag("DOM_TX")
    }
    pub fn in4() -> F {
        field_from_tag("DOM_IN4")
    }
    pub fn in2() -> F {
        field_from_tag("DOM_IN2")
    }
    pub fn pad() -> F {
        field_from_tag("DOM_PAD")
    }
    pub fn dep() -> F {
        field_from_tag("DOM_DEP")
    }
    pub fn wd() -> F {
        field_from_tag("DOM_WD")
    }
    pub fn state() -> F {
        field_from_tag("DOM_S")
    }
    pub fn stmt() -> F {
        field_from_tag("DOM_STMT")
    }
    pub fn ab() -> F {
        field_from_tag("DOM_AB")
    }
    /// Root-history entry domain: entry = H(DOM_RH, era, root) (NF-048).
    /// Not in `all()`, which lists the dataset-v1 domain table.
    pub fn rh() -> F {
        field_from_tag("DOM_RH")
    }
    pub fn all() -> Vec<(&'static str, F)> {
        vec![
            ("DOM_NODE", node()),
            ("DOM_NF_LEAF", nf_leaf()),
            ("DOM_CM", cm()),
            ("DOM_NF", nf()),
            ("DOM_NK", nk()),
            ("DOM_KEY", key()),
            ("DOM_SIG", sig()),
            ("DOM_TXMSG", txmsg()),
            ("DOM_TX", tx()),
            ("DOM_IN4", in4()),
            ("DOM_IN2", in2()),
            ("DOM_PAD", pad()),
            ("DOM_DEP", dep()),
            ("DOM_WD", wd()),
            ("DOM_S", state()),
            ("DOM_STMT", stmt()),
            ("DOM_AB", ab()),
        ]
    }
}

pub fn network_tag() -> F {
    field_from_tag("FENRIR-L2-STRAW")
}

/// Reduce a field element to the 248-bit nullifier domain:
/// keep the low 31 little-endian bytes, clear byte 31.
pub fn trunc248(x: F) -> F {
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(x.to_repr().as_ref());
    repr.as_mut()[31] = 0;
    F::from_repr(repr).expect("248-bit value below modulus")
}

/// The reserved maximal nullifier key, 2^248 - 1.
pub fn max_key() -> F {
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut()[..31].fill(255);
    F::from_repr(repr).unwrap()
}

pub fn is_reserved_nullifier(x: F) -> bool {
    use ff::Field;
    x == F::ZERO || x == max_key()
}

/// Interpret the 32 little-endian bytes of a BLS field element as
/// an integer reduced modulo the Jubjub scalar order. This matches
/// `from_bytes_wide` on a 64-byte buffer whose high half is zero,
/// as in transient-crypto `schnorr.rs`.
pub fn reduce_to_jfr(x: F) -> JFr {
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(x.to_repr().as_ref());
    JFr::from_bytes_wide(&wide)
}

/// Embed a Jubjub scalar into the BLS field by its little-endian
/// bytes. Sound because the Jubjub order is below the BLS modulus.
pub fn jfr_to_f(x: JFr) -> F {
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(x.to_repr().as_ref());
    F::from_repr(repr).expect("jubjub scalar below BLS modulus")
}

pub fn f_to_hex(x: F) -> String {
    hex::encode32(x.to_repr().as_ref())
}

pub fn f_from_hex(s: &str) -> Option<F> {
    let bytes = hex::decode32(s)?;
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&bytes);
    F::from_repr(repr).into()
}

pub fn jfr_to_hex(x: JFr) -> String {
    hex::encode32(x.to_repr().as_ref())
}

pub fn jfr_from_hex(s: &str) -> Option<JFr> {
    let bytes = hex::decode32(s)?;
    let mut repr = <JFr as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&bytes);
    JFr::from_repr(repr).into()
}

pub mod hex {
    pub fn encode32(bytes: &[u8]) -> String {
        assert_eq!(bytes.len(), 32);
        let mut out = String::with_capacity(64);
        for b in bytes {
            out.push_str(&format!("{b:02x}"));
        }
        out
    }
    pub fn decode32(s: &str) -> Option<[u8; 32]> {
        if s.len() != 64 {
            return None;
        }
        let mut out = [0u8; 32];
        for i in 0..32 {
            out[i] = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()?;
        }
        Some(out)
    }
}

/// Serde helpers: fields are 32-byte little-endian hex (64 chars, no prefix).
pub mod serde_f {
    use super::{F, f_from_hex, f_to_hex};
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(x: &F, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&f_to_hex(*x))
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<F, D::Error> {
        let s = String::deserialize(d)?;
        f_from_hex(&s).ok_or_else(|| serde::de::Error::custom("bad field hex"))
    }
}

pub mod serde_f_vec {
    use super::{F, f_from_hex, f_to_hex};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(x: &[F], s: S) -> Result<S::Ok, S::Error> {
        let v: Vec<String> = x.iter().map(|e| f_to_hex(*e)).collect();
        v.serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<F>, D::Error> {
        let v = Vec::<String>::deserialize(d)?;
        v.iter()
            .map(|s| f_from_hex(s).ok_or_else(|| serde::de::Error::custom("bad field hex")))
            .collect()
    }
}

pub mod serde_jfr {
    use super::{JFr, jfr_from_hex, jfr_to_hex};
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(x: &JFr, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&jfr_to_hex(*x))
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<JFr, D::Error> {
        let s = String::deserialize(d)?;
        jfr_from_hex(&s).ok_or_else(|| serde::de::Error::custom("bad jubjub scalar hex"))
    }
}
