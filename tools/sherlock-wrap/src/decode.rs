//! The E3 map: the D-9 decoder (`sherlock-lfr` `iface::decode`, openvm e3-iface 9539418) on
//! either back end. It reads the `L_Fr` public values (the `L` vk pre-hash, then the `L`
//! public words packed 8 per Fr at 31 bits) and gives the call vector (1,026 lanes at
//! `Dims::FULL`), the wrap statement. Each step below is a constraint:
//!
//! 1. unpack: each word `< P` (BabyBear), limb words `< 2^30`; the packed Fr is the sum of
//!    its words (8 * 31 = 248 bits, no wrap, so no high bits);
//! 2. `dec`: each lane and each logged value is 9 limbs of 30 bits and `< r` (`Canon`, a
//!    borrow chain against `r - 1`);
//! 3. entries: the op is admitted (one-hot over the 7 admitted codes), `lo <= param <= hi`
//!    (`DIGEST` tags 0 to 5), no used entry after a `NOP`, offsets and permutation slots,
//!    capacity `off <= V`, `np <= P`; every value after `off_n` is zero;
//! 4. `R` and the anchor rule (plan D-10): `mode` 0 or 1, the `j`, `k` words u16, a chunk
//!    `1 <= j <= k`; entries 0 to 4 (5 for a chunk) are `DIGEST(PROGC)`, `DIGEST(INPUT)`,
//!    `DIGEST(OUTCOME)`, `SPONGE(3)`, `DIGEST(t_in)` [, `DIGEST(t_out)`]; the anchor entry words;
//!    the `claim_pub` inputs are the three run outputs; the native replay of these entries;
//! 5. lane map in stock order (`apply_lane_map`): a pair `(0, 0)` is skipped; otherwise lane
//!    and entry are u16 words, the entry exists and is a `DIGEST` or a `SPONGE(3)`,
//!    `lane < lanes`, the lane is not already mapped and `s[lane] == 0`, then
//!    `s[lane] = vs[off + vals - 1]`; the mapped entry is replayed natively;
//! 6. each `PUB(param)` entry: `s[param] == vs[off]`.
//!
//! Steps 5 (the value reads) and 6 use one log-derivative lookup into the tables
//! `(j, vs_j)` and `(V + l, s_l)` (`s` after the lane map). The multiplicities and the
//! witnessed values are observed into the transcript before the two challenges.
//!
//! One stock check is not here: stock replays every entry (`PERM`, `H2`, `SPONGE`, `DIGEST`);
//! the wrap replays `R` and the mapped entries only (open, E3 J6 in the wrap; FOR OWNER
//! CONFIRMATION). The pre-hash word is pinned to a key constant (recommended option, for owner
//! confirmation). The other statement head words are only range checked.

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};
use num_bigint::BigUint;

use crate::{
    alg::Alg,
    merkle::hash_slice,
    native::{F, low_u64, to_big},
    transcript::Transcript,
};

pub const STMT: usize = 114;
pub const LANE_MAP: usize = 106;
pub const LANE_PAIRS: usize = 4;
pub const LIMBS: usize = 9;
pub const LIMB_BITS: usize = 30;
pub const BB_P: u64 = 0x7800_0001;

pub const NOP: u64 = 0;
pub const PERM: u64 = 1;
pub const H2: u64 = 2;
pub const SPONGE: u64 = 3;
pub const PATH: u64 = 4;
pub const PUB: u64 = 7;
pub const DIGEST: u64 = 8;
/// `DIGEST` tag indices (PROPOSAL 3.6 order; E3 6d70a45): `SHK_PROGC`, `SHK_LOCK`,
/// `SHK_INPUT`, `SHK_OUTCOME`, `SHK_REG`, `SHK_SET`.
pub const N_TAGS: u64 = 6;
pub const TAG_NAMES: [&str; 6] = ["SHK_PROGC", "SHK_LOCK", "SHK_INPUT", "SHK_OUTCOME", "SHK_REG", "SHK_SET"];
pub const SHK_PROGC: u64 = 0;
pub const SHK_LOCK: u64 = 1;
/// Anchor statement words (E3 6d70a45, 95ed47b): `mode`, the `u32` `j` and `k` as two 16-bit
/// words low first, the anchor entry indices.
pub const MODE_WORD: usize = 74;
pub const J_WORDS: usize = MODE_WORD + 1;
pub const K_WORDS: usize = MODE_WORD + 3;
pub const E_IN_WORD: usize = MODE_WORD + 5;
pub const E_OUT_WORD: usize = MODE_WORD + 6;
/// `R` of plan D-10: `DIGEST` of these tags at entries 0 to 2, `SPONGE(3)` at entry 3, the
/// `a_in` anchor `DIGEST` at entry 4 and, for a chunk, the `a_out` anchor at entry 5. The values
/// of entry `i` start at `R_OFF[i]`.
pub const R_RUN: [u64; 3] = [0, 2, 3];
pub const R_OFF: [usize; 6] = [0, 2, 4, 6, 10, 12];
pub const ANCHOR_IN: u64 = 4;

/// The tag value: the ASCII name as a little-endian `u128` (ENC-1 rule P-1).
pub fn tag_fr(k: u64) -> F {
    let name = TAG_NAMES[k as usize].as_bytes();
    let mut b = [0u8; 16];
    b[..name.len()].copy_from_slice(name);
    let v = u128::from_le_bytes(b);
    F::from((v >> 64) as u64) * F::from(2u64).pow_vartime([64]) + F::from(v as u64)
}
pub const PATH_MAX: u64 = 32;
/// The admitted op codes (`ECMUL`, `ECMULG` never admitted while `E = 0`).
pub const ADMITTED: [u64; 7] = [NOP, PERM, H2, SPONGE, PATH, PUB, DIGEST];
const I_SPONGE: usize = 3;
const I_PATH: usize = 4;
const I_PUB: usize = 5;
const I_DIGEST: usize = 6;

/// Capacities (`iface::Dims`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dims {
    pub b: usize,
    pub v: usize,
    pub p: usize,
    pub l_tx: usize,
}

impl Dims {
    pub const FULL: Dims = Dims { b: 256, v: 256, p: 256, l_tx: 1024 };
    pub fn values_at(&self) -> usize {
        STMT + 2 * self.b
    }
    pub fn call_at(&self) -> usize {
        self.values_at() + LIMBS * self.v
    }
    pub fn lanes(&self) -> usize {
        2 + self.l_tx
    }
    pub fn words(&self) -> usize {
        self.call_at() + LIMBS * self.lanes()
    }
    /// `L_Fr` public values: the pre-hash and the packed words.
    pub fn n_public(&self) -> usize {
        1 + self.words().div_ceil(8)
    }
    /// `(low, high)` of the admitted op at index `k` of `ADMITTED`.
    fn bounds(&self, k: usize) -> (u64, u64) {
        match ADMITTED[k] {
            NOP | PERM | H2 => (0, 0),
            SPONGE => (1, self.v as u64 - 1),
            PATH => (1, PATH_MAX),
            PUB => (0, self.lanes() as u64 - 1),
            DIGEST => (0, N_TAGS - 1),
            _ => unreachable!(),
        }
    }
}

/// The decoder's key constants.
#[derive(Clone, Debug)]
pub struct DecodeShape {
    pub d: Dims,
    /// The `L` vk pre-hash (`L_Fr` public value 0), pinned.
    pub pre_hash: F,
}

fn pow2(n: usize) -> BigUint {
    BigUint::from(1u8) << n
}

fn f2(n: usize) -> F {
    F::from(2u64).pow_vartime([n as u64])
}

fn hv<A: Alg>(a: &A, xs: &[A::V]) -> Value<Vec<F>> {
    Value::from_iter(xs.iter().map(|x| a.val(x)))
}

/// `sum x_i` with unit coefficients.
fn sum<A: Alg>(a: &mut A, xs: &[A::V]) -> Result<A::V, Error> {
    let t: Vec<(F, A::V)> = xs.iter().map(|x| (F::ONE, x.clone())).collect();
    a.lin(&t, F::ZERO)
}

/// `sum x_i y_i` by a `mul_add` chain.
fn dot<A: Alg>(a: &mut A, xs: &[A::V], ys: &[A::V]) -> Result<A::V, Error> {
    let mut acc = a.cst(F::ZERO)?;
    for (x, y) in xs.iter().zip(ys) {
        acc = a.mul_add(x, y, &acc)?;
    }
    Ok(acc)
}

/// `[x == 0]` as a constrained bit: `z = 1 - x * inv`, `x * z = 0`.
fn is_zero<A: Alg>(a: &mut A, x: &A::V, what: &str) -> Result<A::V, Error> {
    let inv = a.wit(a.val(x).map(|x| x.invert().unwrap_or(F::ZERO)))?;
    let xi = a.mul(x, &inv)?;
    let z = a.lin(&[(-F::ONE, xi)], F::ONE)?;
    let xz = a.mul(x, &z)?;
    a.assert_eq_c(&xz, F::ZERO, what)?;
    Ok(z)
}

/// `PoseidonFr_4(tag, x)`: the `DIGEST` replay (`sponge_u(4, [tag, x])`).
fn digest<A: Alg>(a: &mut A, tag: &A::V, x: &A::V) -> Result<A::V, Error> {
    hash_slice(a, 4, &[tag.clone(), x.clone()])
}

/// One-hot bits for `idx` over `0..n`, gated: `sum b = gate`, `sum i b_i = idx`.
fn one_hot<A: Alg>(
    a: &mut A,
    n: usize,
    idx: &A::V,
    gate: &A::V,
    what: &str,
) -> Result<Vec<A::V>, Error> {
    let iv = a.val(idx).zip(a.val(gate)).map(|(i, g)| if g == F::ONE { low_u64(&i) as usize } else { usize::MAX });
    let mut bs = vec![];
    for i in 0..n {
        let b = a.wit_bit(iv.map(|j| j == i))?;
        bs.push(a.bit_v(&b)?);
    }
    let s = sum(a, &bs)?;
    a.assert_eq(&s, gate, what)?;
    let t: Vec<(F, A::V)> = bs.iter().enumerate().map(|(i, b)| (F::from(i as u64), b.clone())).collect();
    let w = a.lin(&t, F::ZERO)?;
    a.assert_eq(&w, idx, what)?;
    Ok(bs)
}

/// The 9 limbs of 30 bits of `r - 1`.
pub fn r_minus_1_limbs() -> Vec<u64> {
    let r1 = to_big(&-F::ONE);
    (0..LIMBS)
        .map(|k| ((&r1 >> (LIMB_BITS * k)) % pow2(LIMB_BITS)).to_u64_digits().first().copied().unwrap_or(0))
        .collect()
}

/// `dec`: 9 limbs of 30 bits (already range checked) to Fr, with `Canon` (`< r`).
pub fn dec<A: Alg>(a: &mut A, limbs: &[A::V], r1: &[u64]) -> Result<A::V, Error> {
    let t: Vec<(F, A::V)> = limbs.iter().enumerate().map(|(k, l)| (f2(LIMB_BITS * k), l.clone())).collect();
    let v = a.lin(&t, F::ZERO)?;
    // Borrow chain of (r - 1) - L: d_k = R_k - l_k - c_{k-1} + 2^30 c_k in [0, 2^30), c_8 = 0.
    let lv = hv(a, limbs);
    let borrows = lv.map(|l| {
        let mut b = 0i128;
        let mut out = vec![];
        for k in 0..LIMBS {
            let d = r1[k] as i128 - low_u64(&l[k]) as i128 - b;
            b = if d < 0 { 1 } else { 0 };
            out.push(b == 1);
        }
        out
    });
    let mut prev: Option<A::V> = None;
    for k in 0..LIMBS {
        let mut terms = vec![(-F::ONE, limbs[k].clone())];
        if let Some(p) = &prev {
            terms.push((-F::ONE, p.clone()));
        }
        let ck = if k + 1 < LIMBS {
            let b = a.wit_bit(borrows.as_ref().map(|v| v[k]))?;
            let bv = a.bit_v(&b)?;
            terms.push((f2(LIMB_BITS), bv.clone()));
            Some(bv)
        } else {
            None
        };
        let d = a.lin(&terms, F::from(r1[k]))?;
        a.range(&d, &pow2(LIMB_BITS), "range or Canon")?;
        prev = ck;
    }
    Ok(v)
}

/// The decoder. `pw` are the `L_Fr` public-value cells; `t` is the transcript that has
/// already observed them (the wrap: after `part4`). Returns the call vector `s`.
pub fn decode<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    sh: &DecodeShape,
    pw: &[A::V],
) -> Result<Vec<A::V>, Error> {
    let d = sh.d;
    let n = d.words();
    assert_eq!(pw.len(), d.n_public());
    a.assert_eq_c(&pw[0], sh.pre_hash, "pre-hash")?;

    // 1. Unpack.
    let mut w: Vec<A::V> = Vec::with_capacity(n);
    let lt_p = BigUint::from(BB_P);
    for (i, x) in pw[1..].iter().enumerate() {
        let k = (n - 8 * i).min(8);
        let xv = a.val(x);
        let mut terms = vec![];
        for j in 0..k {
            let c = a.wit(xv.map(|x| {
                let b = (to_big(&x) >> (31 * j)) % pow2(31);
                F::from(b.to_u64_digits().first().copied().unwrap_or(0))
            }))?;
            if 8 * i + j >= d.values_at() {
                a.range(&c, &pow2(LIMB_BITS), "limb range")?;
            } else {
                a.range(&c, &lt_p, "word not BabyBear")?;
            }
            terms.push((f2(31 * j), c.clone()));
            w.push(c);
        }
        let s = a.lin(&terms, F::ZERO)?;
        a.assert_eq(&s, x, "packed word")?;
    }

    // 2. Lanes and logged values.
    let r1 = r_minus_1_limbs();
    let mut lanes = vec![];
    for i in 0..d.lanes() {
        let at = d.call_at() + LIMBS * i;
        lanes.push(dec(a, &w[at..at + LIMBS], &r1)?);
    }
    let mut vs = vec![];
    for j in 0..d.v {
        let at = d.values_at() + LIMBS * j;
        vs.push(dec(a, &w[at..at + LIMBS], &r1)?);
    }

    // 3. Entries.
    let zero = a.cst(F::ZERO)?;
    let (mut off, mut np) = (zero.clone(), zero.clone());
    let mut offs = vec![];
    let mut ops: Vec<Vec<A::V>> = vec![];
    let mut params = vec![];
    let mut prev_nop: Option<A::V> = None;
    for i in 0..d.b {
        let (op, param) = (w[STMT + 2 * i].clone(), w[STMT + 2 * i + 1].clone());
        let opv = a.val(&op).map(|x| low_u64(&x));
        let mut c = vec![];
        for &code in &ADMITTED {
            let b = a.wit_bit(opv.map(|o| o == code))?;
            c.push(a.bit_v(&b)?);
        }
        let one = sum(a, &c)?;
        a.assert_eq_c(&one, F::ONE, "op not admitted")?;
        let tc: Vec<(F, A::V)> = ADMITTED.iter().zip(&c).map(|(&k, b)| (F::from(k), b.clone())).collect();
        let opc = a.lin(&tc, F::ZERO)?;
        a.assert_eq(&opc, &op, "op not admitted")?;
        let lo: Vec<(F, A::V)> = (0..ADMITTED.len()).map(|k| (F::from(d.bounds(k).0), c[k].clone())).collect();
        let hi: Vec<(F, A::V)> = (0..ADMITTED.len()).map(|k| (F::from(d.bounds(k).1), c[k].clone())).collect();
        let lo = a.lin(&lo, F::ZERO)?;
        let hi = a.lin(&hi, F::ZERO)?;
        let x = a.sub(&param, &lo)?;
        a.range(&x, &pow2(11), "param out of bound")?;
        let y = a.sub(&hi, &param)?;
        a.range(&y, &pow2(11), "param out of bound")?;
        if let Some(p) = &prev_nop {
            let m = a.mul(p, &c[0])?;
            a.assert_eq(&m, p, "used entry after NOP")?;
        }
        prev_nop = Some(c[0].clone());
        // vals: PERM 6, H2 3, SPONGE param + 1, PATH 3, PUB 1, DIGEST 2.
        let sp = a.mul_add(&c[I_SPONGE], &param, &c[I_SPONGE])?;
        let vals = a.lin(
            &[
                (F::from(6u64), c[1].clone()),
                (F::from(3u64), c[2].clone()),
                (F::from(3u64), c[I_PATH].clone()),
                (F::ONE, c[I_PUB].clone()),
                (F::from(2u64), c[I_DIGEST].clone()),
                (F::ONE, sp),
            ],
            F::ZERO,
        )?;
        // perms: PERM, H2, DIGEST 1, SPONGE ceil(param / 2), PATH param.
        let pv = a.val(&param).map(|x| low_u64(&x));
        let q = a.wit(pv.map(|p| F::from(p.div_ceil(2))))?;
        let e = a.wit_bit(pv.map(|p| p % 2 == 1))?;
        let ev = a.bit_v(&e)?;
        let chk = a.lin(&[(F::from(2u64), q.clone()), (-F::ONE, param.clone()), (-F::ONE, ev)], F::ZERO)?;
        a.assert_eq_c(&chk, F::ZERO, "sponge slots")?;
        a.range(&q, &pow2(16), "sponge slots")?;
        let spq = a.mul(&c[I_SPONGE], &q)?;
        let pp = a.mul(&c[I_PATH], &param)?;
        let perms = a.lin(
            &[(F::ONE, c[1].clone()), (F::ONE, c[2].clone()), (F::ONE, c[I_DIGEST].clone()), (F::ONE, spq), (F::ONE, pp)],
            F::ZERO,
        )?;
        offs.push(off.clone());
        off = a.add(&off, &vals)?;
        np = a.add(&np, &perms)?;
        ops.push(c);
        params.push(param);
    }
    let cap_v = a.lin(&[(-F::ONE, off.clone())], F::from(d.v as u64))?;
    a.range(&cap_v, &pow2(17), "capacity")?;
    let cap_p = a.lin(&[(-F::ONE, np)], F::from(d.p as u64))?;
    a.range(&cap_p, &pow2(17), "capacity")?;
    // Values after off_n are zero: g_j = [j >= off_n] (monotone, count V - off_n).
    let offv = a.val(&off).map(|x| low_u64(&x) as usize);
    let mut g = vec![];
    for j in 0..d.v {
        let b = a.wit_bit(offv.map(|o| j >= o))?;
        let bv = a.bit_v(&b)?;
        if let Some(p) = g.last() {
            let m = a.mul(p, &bv)?;
            a.assert_eq(&m, p, "value row after off_n not zero")?;
        }
        let z = a.mul(&bv, &vs[j])?;
        a.assert_eq_c(&z, F::ZERO, "value row after off_n not zero")?;
        g.push(bv);
    }
    let gs = sum(a, &g)?;
    a.assert_eq(&gs, &cap_v, "value row after off_n not zero")?;

    // 4. R and the anchor rule (plan D-10; E3 6d70a45, 95ed47b), at static positions.
    assert!(d.b >= 6 && d.v >= R_OFF[5] + 2);
    let mode = w[MODE_WORD].clone();
    let mm = a.mul(&mode, &mode)?;
    let m2 = a.sub(&mm, &mode)?;
    a.assert_eq_c(&m2, F::ZERO, "mode")?;
    for i in 0..4 {
        a.range(&w[J_WORDS + i], &pow2(16), "mode")?;
    }
    let j = a.lin(&[(F::ONE, w[J_WORDS].clone()), (f2(16), w[J_WORDS + 1].clone())], F::ZERO)?;
    let k = a.lin(&[(F::ONE, w[K_WORDS].clone()), (f2(16), w[K_WORDS + 1].clone())], F::ZERO)?;
    let j1 = a.add_c(&j, -F::ONE)?;
    let kj = a.sub(&k, &j)?;
    for x in [&j1, &kj] {
        // A chunk: 1 <= j <= k (each difference in [0, 2^32)).
        let g = a.mul(&mode, x)?;
        a.range(&g, &pow2(32), "mode")?;
    }
    let eq_j1 = is_zero(a, &j1, "mode")?;
    let eq_jk = is_zero(a, &kj, "mode")?;
    // Anchor tags (PROGC 0, LOCK 1): t_in = mode * [j != 1], t_out = [j != k].
    let me = a.mul(&mode, &eq_j1)?;
    let t_in = a.sub(&mode, &me)?;
    let t_out = a.lin(&[(-F::ONE, eq_jk)], F::ONE)?;
    for (i, &t) in R_RUN.iter().enumerate() {
        a.assert_eq_c(&ops[i][I_DIGEST], F::ONE, &format!("R entry {i}"))?;
        a.assert_eq_c(&params[i], F::from(t), &format!("R entry {i}"))?;
    }
    a.assert_eq_c(&ops[3][I_SPONGE], F::ONE, "R entry 3")?;
    a.assert_eq_c(&params[3], F::from(3u64), "R entry 3")?;
    a.assert_eq_c(&ops[4][I_DIGEST], F::ONE, "R entry 4")?;
    a.assert_eq(&params[4], &t_in, "R entry 4")?;
    let c5 = a.add_c(&ops[5][I_DIGEST], -F::ONE)?;
    let c5 = a.mul(&mode, &c5)?;
    a.assert_eq_c(&c5, F::ZERO, "R entry 5")?;
    let p5 = a.sub(&params[5], &t_out)?;
    let p5 = a.mul(&mode, &p5)?;
    a.assert_eq_c(&p5, F::ZERO, "R entry 5")?;
    a.assert_eq_c(&w[E_IN_WORD], F::from(ANCHOR_IN), "anchor entry words")?;
    let eo = a.sub(&w[E_OUT_WORD], &mode)?;
    a.assert_eq_c(&eo, F::from(ANCHOR_IN), "anchor entry words")?;
    let sp = R_OFF[3];
    for i in 0..3 {
        a.assert_eq(&vs[sp + i], &vs[R_OFF[i] + 1], "claim_pub inputs are not the run DIGEST outputs")?;
    }
    // Native replay of R (stock replays every entry; the entries outside R and outside the lane
    // map are not replayed here: open, E3 J6 in the wrap).
    let tags: Vec<A::V> = (0..N_TAGS).map(|t| a.cst(tag_fr(t))).collect::<Result<_, _>>()?;
    for (i, &t) in R_RUN.iter().enumerate() {
        let y = digest(a, &tags[t as usize], &vs[R_OFF[i]])?;
        a.assert_eq(&y, &vs[R_OFF[i] + 1], &format!("entry {i}: native replay"))?;
    }
    let y = hash_slice(a, 0, &vs[sp..sp + 3])?;
    a.assert_eq(&y, &vs[sp + 3], "entry 3: native replay")?;
    let anchor_tag = |a: &mut A, t: &A::V| a.lin(&[(tag_fr(SHK_LOCK) - tag_fr(SHK_PROGC), t.clone())], tag_fr(SHK_PROGC));
    let tg = anchor_tag(a, &t_in)?;
    let y = digest(a, &tg, &vs[R_OFF[4]])?;
    a.assert_eq(&y, &vs[R_OFF[4] + 1], "entry 4: native replay")?;
    let tg = anchor_tag(a, &t_out)?;
    let y = digest(a, &tg, &vs[R_OFF[5]])?;
    let dy = a.sub(&y, &vs[R_OFF[5] + 1])?;
    let dy = a.mul(&mode, &dy)?;
    a.assert_eq_c(&dy, F::ZERO, "entry 5: native replay")?;

    // 5. Lane map, in stock order (`apply_lane_map`, E3 9539418). Queries into V:
    // (gate, index, value).
    let vsv = hv(a, &vs);
    let mut qv: Vec<(A::V, A::V, A::V)> = vec![];
    let mut s = lanes;
    let nl = d.lanes() as u64;
    let digs: Vec<A::V> = ops.iter().map(|c| c[I_DIGEST].clone()).collect();
    let sps: Vec<A::V> = ops.iter().map(|c| c[I_SPONGE].clone()).collect();
    let read = |a: &mut A, idx: &A::V, gate: &A::V| -> Result<A::V, Error> {
        a.wit(a.val(idx).zip(a.val(gate)).zip(vsv.clone()).map(|((i, g), vs)| {
            if g == F::ONE { vs.get(low_u64(&i) as usize).copied().unwrap_or(F::ZERO) } else { F::ZERO }
        }))
    };
    let mut done: Vec<(A::V, A::V)> = vec![];
    for k in 0..LANE_PAIRS {
        let (lane, ent) = (w[LANE_MAP + 2 * k].clone(), w[LANE_MAP + 2 * k + 1].clone());
        // u16 words (finding 3). The gated one-hots below also bound them (lane < lanes,
        // entry < B); the range checks keep the stock order of refusals. (0, 0) passes.
        a.range(&lane, &pow2(16), "lane map pair: not u16")?;
        a.range(&ent, &pow2(16), "lane map pair: not u16")?;
        let u = a.lin(&[(F::ONE, lane.clone()), (F::from(nl), ent.clone())], F::ZERO)?;
        let inv = a.wit(a.val(&u).map(|u| u.invert().unwrap_or(F::ZERO)))?;
        let nz = a.mul(&u, &inv)?;
        let z = a.lin(&[(-F::ONE, nz.clone())], F::ONE)?;
        let zu = a.mul(&z, &u)?;
        a.assert_eq_c(&zu, F::ZERO, "lane map skip")?;
        let h = one_hot(a, d.lanes(), &lane, &nz, "lane map pair")?;
        let e = one_hot(a, d.b, &ent, &nz, "lane map entry")?;
        // The entry is a DIGEST or a SPONGE(3) (E3 099d6d8).
        let sel_dig = dot(a, &e, &digs)?;
        let sel_sp = dot(a, &e, &sps)?;
        let p = dot(a, &e, &params)?;
        let ds = a.add(&sel_dig, &sel_sp)?;
        a.assert_eq(&ds, &nz, "lane map pair")?;
        let p3 = a.add_c(&p, -F::from(3u64))?;
        let sp3 = a.mul(&sel_sp, &p3)?;
        a.assert_eq_c(&sp3, F::ZERO, "lane map pair")?;
        // The lane holds zero limbs and is set at most once (a flag, finding 2a: a mapped
        // output can be 0): lane_i != lane_j for every earlier active pair.
        let taken = dot(a, &h, &s)?;
        a.assert_eq_c(&taken, F::ZERO, "lane map pair")?;
        for (pl, pg) in &done {
            let dl = a.sub(&lane, pl)?;
            let inv = a.wit(a.val(&dl).map(|x| x.invert().unwrap_or(F::ZERO)))?;
            let m = a.mul(&dl, &inv)?;
            let g = a.mul(&nz, pg)?;
            let gm = a.mul(&g, &m)?;
            a.assert_eq(&gm, &g, "lane map pair: lane mapped twice")?;
        }
        done.push((lane.clone(), nz.clone()));
        // The output: vs[o + vals - 1], o + 1 for a DIGEST, o + 3 for a SPONGE(3).
        let o = dot(a, &e, &offs)?;
        let idx = a.lin(&[(F::ONE, o.clone()), (F::from(2u64), sel_sp.clone())], F::ONE)?;
        let val = read(a, &idx, &nz)?;
        // Native replay of the mapped entry (its inputs read through the same lookup).
        let x0 = read(a, &o, &nz)?;
        let i1 = a.add_c(&o, F::ONE)?;
        let i2 = a.add_c(&o, F::from(2u64))?;
        let x1 = read(a, &i1, &sel_sp)?;
        let x2 = read(a, &i2, &sel_sp)?;
        let pd = a.mul(&sel_dig, &p)?;
        let tb = one_hot(a, N_TAGS as usize, &pd, &sel_dig, "lane map entry")?;
        let tt: Vec<(F, A::V)> = tb.iter().enumerate().map(|(t, b)| (tag_fr(t as u64), b.clone())).collect();
        let tag = a.lin(&tt, F::ZERO)?;
        let yd = digest(a, &tag, &x0)?;
        let ys = hash_slice(a, 0, &[x0.clone(), x1.clone(), x2.clone()])?;
        let dd = a.sub(&yd, &val)?;
        let dd = a.mul(&sel_dig, &dd)?;
        a.assert_eq_c(&dd, F::ZERO, "lane map entry: native replay")?;
        let ds = a.sub(&ys, &val)?;
        let ds = a.mul(&sel_sp, &ds)?;
        a.assert_eq_c(&ds, F::ZERO, "lane map entry: native replay")?;
        for (sl, hl) in s.iter_mut().zip(&h) {
            *sl = a.mul_add(hl, &val, sl)?;
        }
        qv.push((nz.clone(), idx, val));
        qv.push((nz.clone(), o, x0));
        qv.push((sel_sp.clone(), i1, x1));
        qv.push((sel_sp, i2, x2));
    }

    // 6. PUB entries: query (off_i, v_i) into V and (param_i, v_i) into S.
    let mut qs: Vec<(A::V, A::V, A::V)> = vec![];
    for i in 0..d.b {
        let gate = ops[i][I_PUB].clone();
        let v = a.wit(a.val(&offs[i]).zip(a.val(&gate)).zip(vsv.clone()).map(|((o, g), vs)| {
            if g == F::ONE { vs.get(low_u64(&o) as usize).copied().unwrap_or(F::ZERO) } else { F::ZERO }
        }))?;
        qv.push((gate.clone(), offs[i].clone(), v.clone()));
        qs.push((gate, params[i].clone(), v));
    }

    // Multiplicities (host count of gated query indices).
    let count = |a: &A, q: &[(A::V, A::V, A::V)], n: usize| -> Value<Vec<F>> {
        let gi: Vec<Value<(F, F)>> = q.iter().map(|(g, i, _)| a.val(g).zip(a.val(i))).collect();
        Value::<Vec<(F, F)>>::from_iter(gi).map(|gi| {
            let mut m = vec![0u64; n];
            for (g, i) in gi {
                if g == F::ONE {
                    if let Some(x) = m.get_mut(low_u64(&i) as usize) {
                        *x += 1;
                    }
                }
            }
            m.into_iter().map(F::from).collect()
        })
    };
    let mvv = count(a, &qv, d.v);
    let msv = count(a, &qs, d.lanes());
    let mut mv = vec![];
    for j in 0..d.v {
        mv.push(a.wit(mvv.as_ref().map(|m| m[j]))?);
    }
    let mut ms = vec![];
    for l in 0..d.lanes() {
        ms.push(a.wit(msv.as_ref().map(|m| m[l]))?);
    }
    for x in mv.iter().chain(&ms) {
        t.observe(a, x)?;
    }
    for (_, _, v) in qv.iter().chain(&qs[..]) {
        t.observe(a, v)?;
    }
    let alpha = t.sample(a)?;
    let beta = t.sample(a)?;

    // 1 / (alpha - (key + beta * val)), checked.
    let term = |a: &mut A, key: &A::V, val: &A::V| -> Result<A::V, Error> {
        let kv = a.mul_add(&beta, val, key)?;
        let den = a.sub(&alpha, &kv)?;
        let inv = a.wit(a.val(&den).map(|x| x.invert().unwrap_or(F::ZERO)))?;
        let one = a.mul(&den, &inv)?;
        a.assert_eq_c(&one, F::ONE, "lookup denominator")?;
        Ok(inv)
    };
    let mut lhs = zero.clone();
    for (g, i, v) in &qv {
        let inv = term(a, i, v)?;
        lhs = a.mul_add(g, &inv, &lhs)?;
    }
    for (g, i, v) in &qs {
        let key = a.add_c(i, F::from(d.v as u64))?;
        let inv = term(a, &key, v)?;
        lhs = a.mul_add(g, &inv, &lhs)?;
    }
    let mut rhs = zero;
    for j in 0..d.v {
        let key = a.cst(F::from(j as u64))?;
        let inv = term(a, &key, &vs[j])?;
        rhs = a.mul_add(&mv[j], &inv, &rhs)?;
    }
    for l in 0..d.lanes() {
        let key = a.cst(F::from((d.v + l) as u64))?;
        let inv = term(a, &key, &s[l])?;
        rhs = a.mul_add(&ms[l], &inv, &rhs)?;
    }
    a.assert_eq(&lhs, &rhs, "PUB differs from its lane")?;
    Ok(s)
}
