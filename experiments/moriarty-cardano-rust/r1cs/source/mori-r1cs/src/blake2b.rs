//! BLAKE2b-256 (RFC 7693; `nn = 32`, no key) on the host and as an R1CS gadget.
//!
//! P3S1, PROPOSED, unreviewed, never compiled or run. Used only by the
//! `SettlementHashV2` adapter (`abi2.rs`) to bind the ABI v2 statement bytes to the two
//! public inputs `int_be(digest[0..16])`, `int_be(digest[16..32])`.
//!
//! Gadget representation: a bit is a linear combination that is 0 or 1 (a booleanized
//! witness, a constant, an allocated XOR output, or `1 - x` of one of these); a 64-bit
//! word is 64 bits, least significant first. Rotations are free (a permutation of bits).
//! XOR of two non-constant bits costs one witness `c` and one constraint
//! `(2a) * b = a + b - c` (P3S1-R1G1, review E-01: returning `a + b - 2ab` unallocated made
//! every stored row grow with the XOR chain and the block count). A sum of k words
//! modulo 2^64 costs `64 + ceil(log2 k)` booleanized result bits plus one equation; the
//! integer sum is below 3 * 2^64 < r, so the decomposition is unique and the low 64 bits
//! are exactly the modular sum. Constants fold at synthesis time; the circuit structure
//! depends only on the message length, never on values.
use crate::r1cs::{Cs, Lc, fe, fe_bool};
use ark_ff::PrimeField;
use ark_relations::r1cs::SynthesisError;

pub(crate) const IV: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

pub(crate) const SIGMA: [[usize; 16]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

/// Parameter word for an unkeyed 32-byte digest: `0x0101kknn` with kk = 0, nn = 32.
const PARAM: u64 = 0x0101_0020;

// ## Host implementation

fn g_host(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(32);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(24);
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

fn compress_host(h: &mut [u64; 8], block: &[u8; 128], t: u128, last: bool) {
    let mut m = [0u64; 16];
    for (i, w) in m.iter_mut().enumerate() {
        let mut b = [0u8; 8];
        b.copy_from_slice(&block[8 * i..8 * i + 8]);
        *w = u64::from_le_bytes(b);
    }
    let mut v = [0u64; 16];
    v[..8].copy_from_slice(h);
    v[8..].copy_from_slice(&IV);
    v[12] ^= t as u64;
    v[13] ^= (t >> 64) as u64;
    if last {
        v[14] = !v[14];
    }
    for r in 0..12 {
        let s = &SIGMA[r % 10];
        g_host(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
        g_host(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
        g_host(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
        g_host(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]]);
        g_host(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
        g_host(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
        g_host(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
        g_host(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
    }
    for i in 0..8 {
        h[i] ^= v[i] ^ v[i + 8];
    }
}

/// Number of 128-byte blocks for a message of `len` bytes (at least one).
pub fn block_count(len: usize) -> usize {
    if len == 0 { 1 } else { len.div_ceil(128) }
}

/// BLAKE2b-256 of `msg`.
pub fn blake2b256(msg: &[u8]) -> [u8; 32] {
    let mut h = IV;
    h[0] ^= PARAM;
    let blocks = block_count(msg.len());
    for b in 0..blocks {
        let mut block = [0u8; 128];
        let start = 128 * b;
        let end = msg.len().min(start + 128);
        if start < end {
            block[..end - start].copy_from_slice(&msg[start..end]);
        }
        compress_host(&mut h, &block, end as u128, b + 1 == blocks);
    }
    let mut out = [0u8; 32];
    for i in 0..4 {
        out[8 * i..8 * i + 8].copy_from_slice(&h[i].to_le_bytes());
    }
    out
}

// ## Gadget

/// A bit: a linear combination equal to 0 or 1.
#[derive(Clone, Debug)]
pub(crate) struct Bit<F: PrimeField> {
    pub lc: Lc<F>,
    pub val: Option<bool>,
    pub konst: Option<bool>,
}

impl<F: PrimeField> Bit<F> {
    pub fn constant(b: bool) -> Self {
        Self { lc: Lc::constant(fe_bool(b)), val: Some(b), konst: Some(b) }
    }
}

/// A booleanized witness bit: `b * b = b`.
pub(crate) fn alloc_bit<F: PrimeField>(cs: &mut Cs<F>, role: &str, val: Option<bool>) -> Result<Bit<F>, SynthesisError> {
    let v = cs.witness(role, val.map(fe_bool))?;
    cs.enforce(Lc::var(v), Lc::var(v), Lc::var(v))?;
    Ok(Bit { lc: Lc::var(v), val, konst: None })
}

fn not<F: PrimeField>(a: &Bit<F>) -> Bit<F> {
    Bit { lc: Lc::one().minus(&a.lc), val: a.val.map(|x| !x), konst: a.konst.map(|x| !x) }
}

fn xor<F: PrimeField>(cs: &mut Cs<F>, a: &Bit<F>, b: &Bit<F>) -> Result<Bit<F>, SynthesisError> {
    match (a.konst, b.konst) {
        (Some(x), Some(y)) => Ok(Bit::constant(x ^ y)),
        (Some(false), _) => Ok(b.clone()),
        (Some(true), _) => Ok(not(b)),
        (_, Some(false)) => Ok(a.clone()),
        (_, Some(true)) => Ok(not(a)),
        _ => {
            let val = match (a.val, b.val) {
                (Some(x), Some(y)) => Some(x ^ y),
                _ => None,
            };
            // P3S1-R1G1 (review E-01): the output is allocated, with (2a) * b = a + b - c,
            // so c = a + b - 2ab = a XOR b for Boolean a and b. Still one witness and one
            // constraint; the returned bit is one variable, so no later row carries the
            // chain of XORs (and blocks) behind it.
            let c = cs.witness("b2.xor", val.map(fe_bool))?;
            cs.enforce(a.lc.scaled(fe(2)), b.lc.clone(), a.lc.clone().plus(&b.lc).minus(&Lc::var(c)))?;
            Ok(Bit { lc: Lc::var(c), val, konst: None })
        }
    }
}

pub(crate) type Word<F> = Vec<Bit<F>>;

fn word_const<F: PrimeField>(w: u64) -> Word<F> {
    (0..64).map(|i| Bit::constant((w >> i) & 1 == 1)).collect()
}

fn word_xor<F: PrimeField>(cs: &mut Cs<F>, a: &Word<F>, b: &Word<F>) -> Result<Word<F>, SynthesisError> {
    a.iter().zip(b).map(|(x, y)| xor(cs, x, y)).collect()
}

/// Rotate right by `n`: result bit i is input bit (i + n) mod 64.
fn rotr<F: PrimeField>(w: &Word<F>, n: usize) -> Word<F> {
    (0..64).map(|i| w[(i + n) % 64].clone()).collect()
}

fn word_value<F: PrimeField>(w: &Word<F>) -> Option<u64> {
    let mut v = 0u64;
    for (i, b) in w.iter().enumerate() {
        if b.val? {
            v |= 1 << i;
        }
    }
    Some(v)
}

fn word_konst<F: PrimeField>(w: &Word<F>) -> Option<u64> {
    let mut v = 0u64;
    for (i, b) in w.iter().enumerate() {
        if b.konst? {
            v |= 1 << i;
        }
    }
    Some(v)
}

/// Sum of 2 or 3 words modulo 2^64.
fn add_words<F: PrimeField>(cs: &mut Cs<F>, ws: &[&Word<F>]) -> Result<Word<F>, SynthesisError> {
    debug_assert!(ws.len() == 2 || ws.len() == 3);
    if let Some(ks) = ws.iter().map(|w| word_konst(w)).collect::<Option<Vec<u64>>>() {
        let s = ks.iter().fold(0u64, |a, k| a.wrapping_add(*k));
        return Ok(word_const(s));
    }
    let mut total = Lc::zero();
    for w in ws {
        for (i, b) in w.iter().enumerate() {
            total = total.plus(&b.lc.scaled(fe(1u128 << i)));
        }
    }
    let val: Option<u128> = ws.iter().map(|w| word_value(w).map(u128::from)).sum();
    let width = 66;
    let mut out = Vec::with_capacity(64);
    let mut recomposed = Lc::zero();
    for j in 0..width {
        let bit = alloc_bit(cs, "b2.sum", val.map(|v| (v >> j) & 1 == 1))?;
        recomposed = recomposed.plus(&bit.lc.scaled(fe(1u128 << j)));
        if j < 64 {
            out.push(bit);
        }
    }
    cs.enforce_eq(&total, &recomposed)?;
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
fn g<F: PrimeField>(
    cs: &mut Cs<F>,
    v: &mut [Word<F>],
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    x: &Word<F>,
    y: &Word<F>,
) -> Result<(), SynthesisError> {
    v[a] = add_words(cs, &[&v[a], &v[b], x])?;
    v[d] = rotr(&word_xor(cs, &v[d], &v[a])?, 32);
    v[c] = add_words(cs, &[&v[c], &v[d]])?;
    v[b] = rotr(&word_xor(cs, &v[b], &v[c])?, 24);
    v[a] = add_words(cs, &[&v[a], &v[b], y])?;
    v[d] = rotr(&word_xor(cs, &v[d], &v[a])?, 16);
    v[c] = add_words(cs, &[&v[c], &v[d]])?;
    v[b] = rotr(&word_xor(cs, &v[b], &v[c])?, 63);
    Ok(())
}

fn compress<F: PrimeField>(cs: &mut Cs<F>, h: &mut [Word<F>], m: &[Word<F>], t: u128, last: bool) -> Result<(), SynthesisError> {
    let mut v: Vec<Word<F>> = Vec::with_capacity(16);
    v.extend(h.iter().cloned());
    v.extend(IV.iter().map(|w| word_const(*w)));
    v[12] = word_xor(cs, &v[12], &word_const(t as u64))?;
    v[13] = word_xor(cs, &v[13], &word_const((t >> 64) as u64))?;
    if last {
        v[14] = v[14].iter().map(not::<F>).collect();
    }
    for r in 0..12 {
        let s = &SIGMA[r % 10];
        g(cs, &mut v, 0, 4, 8, 12, &m[s[0]], &m[s[1]])?;
        g(cs, &mut v, 1, 5, 9, 13, &m[s[2]], &m[s[3]])?;
        g(cs, &mut v, 2, 6, 10, 14, &m[s[4]], &m[s[5]])?;
        g(cs, &mut v, 3, 7, 11, 15, &m[s[6]], &m[s[7]])?;
        g(cs, &mut v, 0, 5, 10, 15, &m[s[8]], &m[s[9]])?;
        g(cs, &mut v, 1, 6, 11, 12, &m[s[10]], &m[s[11]])?;
        g(cs, &mut v, 2, 7, 8, 13, &m[s[12]], &m[s[13]])?;
        g(cs, &mut v, 3, 4, 9, 14, &m[s[14]], &m[s[15]])?;
    }
    for i in 0..8 {
        let x = word_xor(cs, &h[i], &v[i])?;
        h[i] = word_xor(cs, &x, &v[i + 8])?;
    }
    Ok(())
}

/// In-circuit BLAKE2b-256 of a message given as bits (8 per byte, least significant
/// first). Returns the 256 digest bits in the same byte/bit order.
pub(crate) fn blake2b256_gadget<F: PrimeField>(cs: &mut Cs<F>, msg: &[Bit<F>]) -> Result<Vec<Bit<F>>, SynthesisError> {
    assert!(msg.len() % 8 == 0, "message bits come in whole bytes");
    let len = msg.len() / 8;
    let mut h: Vec<Word<F>> = IV.iter().map(|w| word_const(*w)).collect();
    h[0] = word_const(IV[0] ^ PARAM);
    let blocks = block_count(len);
    cs.push("blake2b".into());
    for b in 0..blocks {
        let mut words: Vec<Word<F>> = Vec::with_capacity(16);
        for wi in 0..16 {
            let mut w = Vec::with_capacity(64);
            for byte in 0..8 {
                let pos = 128 * b + 8 * wi + byte;
                for bit in 0..8 {
                    w.push(if pos < len { msg[8 * pos + bit].clone() } else { Bit::constant(false) });
                }
            }
            words.push(w);
        }
        let t = len.min(128 * (b + 1)) as u128;
        compress(cs, &mut h, &words, t, b + 1 == blocks)?;
    }
    cs.pop();
    let mut out = Vec::with_capacity(256);
    for w in h.iter().take(4) {
        out.extend(w.iter().cloned());
    }
    Ok(out)
}

/// Linear combination of a big-endian integer formed by digest bytes `[from, from+16)`.
pub(crate) fn digest_half_lc<F: PrimeField>(digest: &[Bit<F>], from: usize) -> Lc<F> {
    let mut lc = Lc::zero();
    for k in 0..16 {
        let byte = from + k;
        let weight_byte = 8 * (15 - k) as u32;
        for bit in 0..8u32 {
            let b = &digest[8 * byte + bit as usize];
            lc = lc.plus(&b.lc.scaled(pow2::<F>(weight_byte + bit)));
        }
    }
    lc
}

/// Test hook: synthesizes the gadget over `msg` given as booleanized witness bits in a
/// fresh prove-mode constraint system. Returns (satisfied, in-circuit digest from the
/// witness values, constraint count). The digest is read from bit values, so equality
/// with [`blake2b256`] is internal agreement; the frozen hashlib vectors are the
/// independent reference.
#[doc(hidden)]
pub fn gadget_check<F: PrimeField>(msg: &[u8]) -> Result<(bool, [u8; 32], usize), SynthesisError> {
    use ark_relations::r1cs::ConstraintSystem;
    let cs_ref = ConstraintSystem::<F>::new_ref();
    let mut cs = Cs::new(cs_ref.clone(), crate::r1cs::Tamper::none());
    let mut bits = Vec::with_capacity(8 * msg.len());
    for byte in msg {
        for j in 0..8 {
            bits.push(alloc_bit(&mut cs, "msg", Some((byte >> j) & 1 == 1))?);
        }
    }
    let digest = blake2b256_gadget(&mut cs, &bits)?;
    let mut out = [0u8; 32];
    for (k, o) in out.iter_mut().enumerate() {
        for j in 0..8 {
            if digest[8 * k + j].val == Some(true) {
                *o |= 1 << j;
            }
        }
    }
    Ok((cs_ref.is_satisfied()?, out, cs.num_constraints()))
}

/// Test hook (P3S1-R1G1, review E-01): synthesizes the gadget over `msg` (witness bits,
/// no values) in setup mode, finalized as the compile-time probe does, and returns
/// (the most nonzero entries in one A, B or C row, the nonzero entries of all three
/// matrices). Structure only: it shows how large the stored linear combinations get.
#[doc(hidden)]
pub fn gadget_weight<F: PrimeField>(msg: &[u8]) -> Result<(usize, usize), SynthesisError> {
    use ark_relations::r1cs::{ConstraintSystem, OptimizationGoal, SynthesisMode};
    let cs_ref = ConstraintSystem::<F>::new_ref();
    cs_ref.set_optimization_goal(OptimizationGoal::Constraints);
    cs_ref.set_mode(SynthesisMode::Setup);
    let mut cs = Cs::new(cs_ref.clone(), crate::r1cs::Tamper::none());
    let mut bits = Vec::with_capacity(8 * msg.len());
    for _ in 0..8 * msg.len() {
        bits.push(alloc_bit(&mut cs, "msg", None)?);
    }
    blake2b256_gadget(&mut cs, &bits)?;
    cs_ref.finalize();
    let m = cs_ref.to_matrices().expect("setup mode constructs matrices");
    let widest = m.a.iter().chain(&m.b).chain(&m.c).map(|row| row.len()).max().unwrap_or(0);
    Ok((widest, m.a_num_non_zero + m.b_num_non_zero + m.c_num_non_zero))
}

pub(crate) fn pow2<F: PrimeField>(e: u32) -> F {
    if e < 128 { fe(1u128 << e) } else { fe::<F>(1u128 << 64) * fe::<F>(1u128 << 64) * pow2::<F>(e - 128) }
}
