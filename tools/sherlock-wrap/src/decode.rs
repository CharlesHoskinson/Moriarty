//! The E3 map: the D-9 decoder (`sherlock-lfr` `iface::decode`, openvm e3-iface 2f09caf) on
//! either back end. It reads the `L_Fr` public values (the `L` vk pre-hash, then the `L`
//! public words packed 8 per Fr at 31 bits) and gives the call vector (1,026 lanes at
//! `Dims::FULL`), the wrap statement. Every stock check is a constraint:
//!
//! 1. unpack: each word `< P` (BabyBear), limb words `< 2^30`; the packed Fr is the sum of
//!    its words (8 * 31 = 248 bits, no wrap, so no high bits);
//! 2. `dec`: each lane and each logged value is 9 limbs of 30 bits and `< r` (`Canon`, a
//!    borrow chain against `r - 1`);
//! 3. entries: the op is admitted (one-hot over the 7 admitted codes), `lo <= param <= hi`,
//!    no used entry after a `NOP`, offsets and permutation slots, capacity `off <= V`,
//!    `np <= P`; every value after `off_n` is zero;
//! 4. lane map in stock order: a pair `(0, 0)` is skipped; otherwise the entry exists and is a
//!    `DIGEST`, `lane < lanes`, `s[lane] == 0`, then `s[lane] = vs[off + 1]`;
//! 5. each `PUB(param)` entry: `s[param] == vs[off]`.
//!
//! Steps 4 (the value read) and 5 use one log-derivative lookup into the tables
//! `(j, vs_j)` and `(V + l, s_l)` (`s` after the lane map). The multiplicities and the
//! witnessed values are observed into the transcript before the two challenges.
//!
//! The pre-hash word is pinned to a key constant (recommended option, for owner
//! confirmation). The statement head words (0 to 113, other than the lane map) are only
//! range checked; the native-Fr replay of the log values (E3 J6) is not in the wrap.

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};
use num_bigint::BigUint;

use crate::{
    alg::Alg,
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
pub const N_TAGS: u64 = 4;
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

    // 4. Lane map, in stock order. Queries into V: (gate, index, value).
    let vsv = hv(a, &vs);
    let mut qv: Vec<(A::V, A::V, A::V)> = vec![];
    let mut s = lanes;
    let nl = d.lanes() as u64;
    for k in 0..LANE_PAIRS {
        let (lane, ent) = (w[LANE_MAP + 2 * k].clone(), w[LANE_MAP + 2 * k + 1].clone());
        let u = a.lin(&[(F::ONE, lane.clone()), (F::from(nl), ent.clone())], F::ZERO)?;
        let inv = a.wit(a.val(&u).map(|u| u.invert().unwrap_or(F::ZERO)))?;
        let nz = a.mul(&u, &inv)?;
        let z = a.lin(&[(-F::ONE, nz.clone())], F::ONE)?;
        let zu = a.mul(&z, &u)?;
        a.assert_eq_c(&zu, F::ZERO, "lane map skip")?;
        let h = one_hot(a, d.lanes(), &lane, &nz, "lane map pair")?;
        let e = one_hot(a, d.b, &ent, &nz, "lane map entry")?;
        let digs: Vec<A::V> = ops.iter().map(|c| c[I_DIGEST].clone()).collect();
        let dig = dot(a, &e, &digs)?;
        a.assert_eq(&dig, &nz, "lane map pair")?;
        let taken = dot(a, &h, &s)?;
        a.assert_eq_c(&taken, F::ZERO, "lane map pair")?;
        let o = dot(a, &e, &offs)?;
        let idx = a.add_c(&o, F::ONE)?;
        let val = a.wit(a.val(&idx).zip(a.val(&nz)).zip(vsv.clone()).map(|((i, g), vs)| {
            if g == F::ONE { vs.get(low_u64(&i) as usize).copied().unwrap_or(F::ZERO) } else { F::ZERO }
        }))?;
        for (sl, hl) in s.iter_mut().zip(&h) {
            *sl = a.mul_add(hl, &val, sl)?;
        }
        qv.push((nz, idx, val));
    }

    // 5. PUB entries: query (off_i, v_i) into V and (param_i, v_i) into S.
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
