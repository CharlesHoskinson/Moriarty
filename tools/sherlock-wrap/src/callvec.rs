//! E6: the call vector of the wrap statement, read from the `L_Fr` public words (the E3 map in
//! place of the E5 `PublicMap` placeholder).
//!
//! `L_Fr` public words (E3 D-2'): word 0 is the pre-hash of the `L` key, then the 12,164 public
//! words of `L` packed 8 per Fr (31 bits each, little-endian). In the `L` index table (E3
//! `iface.rs`) the call vector is words 2,930 to 12,163: 1,026 lanes of 9 limbs of 30 bits.
//!
//! The wrap circuit:
//! - pins word 0 to the `L` key pre-hash of the keyed fixture;
//! - unpacks each packed word that holds a lane-map or call-vector word into 8 (or 4) chunks of
//!   31 bits (`x < 2^248 < r`, so the chunks are unique);
//! - per lane: each limb below `2^30`, `Canon` (the 270-bit integer at most `r - 1`, by a
//!   borrow chain whose differences are each below `2^30`), and the lane is the integer;
//! - refuses a non-empty lane map (words 106 to 113 zero).
//!
//! Not in this map (named open, FZ-3 production): applying the lane map, the `PUB(i)` rule and
//! the replay of the deferred field log. So the wrap accepts only runs whose lane map is empty,
//! and its statement is the call-vector lanes of `L` exactly.

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};
use num_bigint::BigUint;

use crate::{alg::Alg, native::F};

pub const WORDS: usize = 12_164;
pub const PACKED: usize = 1 + WORDS.div_ceil(8);
pub const CALL_AT: usize = 2_930;
pub const LANES: usize = 1_026;
pub const LIMBS: usize = 9;
pub const LIMB_BITS: usize = 30;
pub const LANE_MAP: usize = 106;
pub const LANE_MAP_WORDS: usize = 8;

/// BLS12-381 scalar field order `r`.
pub fn r_big() -> BigUint {
    BigUint::parse_bytes(b"73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001", 16).unwrap()
}

/// 30-bit limbs of `r - 1`.
fn r_minus_1_limbs() -> [u64; LIMBS] {
    let x = r_big() - 1u8;
    let mask = (BigUint::from(1u8) << LIMB_BITS) - 1u8;
    std::array::from_fn(|k| ((&x >> (LIMB_BITS * k)) & &mask).to_u64_digits().first().copied().unwrap_or(0))
}

fn low(v: Value<F>) -> Value<u64> {
    v.map(|x| crate::native::low_u64(&x))
}

/// The statement (1,026 lanes) from the `L_Fr` public words.
pub fn call_vector<A: Alg>(a: &mut A, pv: &[A::V], pre_hash: F) -> Result<Vec<A::V>, Error> {
    assert_eq!(pv.len(), PACKED, "L_Fr public words");
    a.assert_eq_c(&pv[0], pre_hash, "L key pre-hash")?;
    // Unpack the packed words that hold L words in [lo, hi).
    let unpack = |a: &mut A, lo: usize, hi: usize| -> Result<Vec<(usize, A::V)>, Error> {
        let mut out = vec![];
        for j in lo / 8..hi.div_ceil(8) {
            let n = (WORDS - 8 * j).min(8);
            let cs = a.chunks(&pv[1 + j], 31, n)?;
            for (i, c) in cs.into_iter().enumerate() {
                let w = 8 * j + i;
                if (lo..hi).contains(&w) {
                    out.push((w, c));
                }
            }
        }
        Ok(out)
    };
    for (w, c) in unpack(a, LANE_MAP, LANE_MAP + LANE_MAP_WORDS)? {
        a.assert_eq_c(&c, F::from(0u64), &format!("lane map word {w} not zero"))?;
    }
    let words = unpack(a, CALL_AT, WORDS)?;
    assert_eq!(words.len(), LIMBS * LANES);
    let rl = r_minus_1_limbs();
    let two30 = F::from(1u64 << LIMB_BITS);
    let mut lanes = Vec::with_capacity(LANES);
    for i in 0..LANES {
        let c: Vec<A::V> = words[LIMBS * i..LIMBS * (i + 1)].iter().map(|(_, c)| c.clone()).collect();
        let mut borrow: Option<(A::V, Value<u64>)> = None;
        for k in 0..LIMBS {
            a.range(&c[k], LIMB_BITS, &format!("lane {i} limb {k} range"))?;
            // d_k = R_k - c_k - b_k + 2^30 b_(k+1), each d_k below 2^30; no borrow out of limb 8.
            let (bv, bval) = match &borrow {
                Some((v, x)) => (Some(v.clone()), *x),
                None => (None, Value::known(0)),
            };
            let cv = low(a.val(&c[k]));
            let mut terms = vec![(-F::from(1u64), c[k].clone())];
            if let Some(v) = bv {
                terms.push((-F::from(1u64), v));
            }
            if k + 1 < LIMBS {
                let out = cv.zip(bval).map(|(c, b)| (rl[k] as i128) - (c as i128) - (b as i128) < 0);
                let ob = a.wbit(out)?;
                let ov = a.bit_v(&ob)?;
                terms.push((two30, ov.clone()));
                let d = a.lin(&terms, F::from(rl[k]))?;
                a.range(&d, LIMB_BITS, &format!("lane {i} canon limb {k}"))?;
                borrow = Some((ov, out.map(|o| o as u64)));
            } else {
                let d = a.lin(&terms, F::from(rl[k]))?;
                a.range(&d, LIMB_BITS, &format!("lane {i} canon"))?;
            }
        }
        let terms: Vec<(F, A::V)> =
            c.iter().enumerate().map(|(k, x)| (F::from(2u64).pow_vartime([(LIMB_BITS * k) as u64]), x.clone())).collect();
        lanes.push(a.lin(&terms, F::from(0u64))?);
    }
    Ok(lanes)
}

/// Native statement and failed checks.
pub fn call_vector_native(pv: &[F], pre_hash: F) -> (Vec<F>, Vec<String>) {
    let mut n = crate::alg::Native::default();
    let vals: Vec<Value<F>> = pv.iter().map(|x| Value::known(*x)).collect();
    let lanes = call_vector(&mut n, &vals, pre_hash).unwrap();
    let mut out = vec![];
    for l in lanes {
        l.map(|x| out.push(x));
    }
    (out, n.failures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ff::PrimeField;

    fn pack(words: &[u64], pre: F) -> Vec<F> {
        let mut v = vec![pre];
        for ch in words.chunks(8) {
            let mut b = BigUint::from(0u8);
            for (i, w) in ch.iter().enumerate() {
                b += BigUint::from(*w) << (31 * i);
            }
            v.push(from_big(&b));
        }
        v
    }
    fn from_big(b: &BigUint) -> F {
        let mut bytes = b.to_bytes_le();
        bytes.resize(32, 0);
        let mut r = <F as PrimeField>::Repr::default();
        r.as_mut().copy_from_slice(&bytes);
        F::from_repr(r).unwrap()
    }
    fn limbs(b: &BigUint) -> Vec<u64> {
        let mask = (BigUint::from(1u8) << 30) - 1u8;
        (0..9usize).map(|k| ((b >> (30 * k)) & &mask).to_u64_digits().first().copied().unwrap_or(0)).collect()
    }
    fn words(lanes: &[BigUint]) -> Vec<u64> {
        let mut w = vec![5u64; CALL_AT];
        for x in &mut w[LANE_MAP..LANE_MAP + 8] {
            *x = 0;
        }
        for l in lanes {
            w.extend(limbs(l));
        }
        w
    }

    #[test]
    fn round_trip_and_refusals() {
        let pre = F::from(77u64);
        let mut lanes: Vec<BigUint> = (0..LANES as u64).map(|i| BigUint::from(i * 1_000_003 + 1) << (i % 200)).collect();
        lanes[3] = r_big() - 1u8;
        let (s, f) = call_vector_native(&pack(&words(&lanes), pre), pre);
        assert!(f.is_empty(), "{f:?}");
        assert_eq!(s, lanes.iter().map(from_big).collect::<Vec<_>>());
        // Wrong pre-hash.
        assert!(!call_vector_native(&pack(&words(&lanes), pre), F::from(78u64)).1.is_empty());
        // Non-empty lane map.
        let mut w = words(&lanes);
        w[LANE_MAP + 1] = 3;
        assert!(!call_vector_native(&pack(&w, pre), pre).1.is_empty());
        // A lane equal to r (not Canon).
        let mut w = words(&lanes);
        let rl = limbs(&r_big());
        w[CALL_AT + 9 * 5..CALL_AT + 9 * 6].copy_from_slice(&rl);
        assert!(!call_vector_native(&pack(&w, pre), pre).1.is_empty());
        // A limb at 2^30.
        let mut w = words(&lanes);
        w[CALL_AT + 9 * 7] = 1 << 30;
        assert!(!call_vector_native(&pack(&w, pre), pre).1.is_empty());
    }
}
