//! The E3 map: the D-9 decoder (`sherlock-lfr` `iface::decode`, openvm e3-iface 9539418) on
//! either back end. It reads the `L_Fr` public values (the `L` vk pre-hash, then the `L`
//! public words packed 8 per Fr at 31 bits) and gives the call vector (1,026 lanes at
//! `Dims::FULL`), the wrap statement. Each step below is a constraint:
//!
//! 1. unpack: each word `< P` (BabyBear), limb words `< 2^30`; the packed Fr is the sum of
//!    its words (8 * 31 = 248 bits, no wrap, so no high bits);
//! 2. `dec`: each lane and each logged value is 9 limbs of 30 bits and `< r` (`Canon`, a
//!    borrow chain against `r - 1`);
//! 2b. (T9) `X` (head words 97 to 105) is 9 limbs of 30 bits with `Canon` and equals the key
//!    constant `x` (the cached-trace commitment of `L` that `L_Fr` pins, the DAG key check);
//!    `layout_d` (head words 89 to 96) equals the key constant (ENC-3, V+);
//! 3. entries: the op is admitted (one-hot over the 7 admitted codes), `lo <= param <= hi`
//!    (`DIGEST` tags 0 to 5), no used entry after a `NOP`, offsets and permutation slots,
//!    capacity `off <= V`, `np <= P`; every value after `off_n` is zero;
//! 4. `R` and the anchor rule (plan D-10): `mode` 0 or 1, the `j`, `k` words u16, a chunk
//!    `1 <= j <= k`; entries 0 to 4 (5 for a chunk) are `DIGEST(PROGC)`, `DIGEST(INPUT)`,
//!    `DIGEST(OUTCOME)`, `SPONGE(3)`, `DIGEST(t_in)` [, `DIGEST(t_out)`]; (T9) or, in the
//!    opening-proof form `open = [param_4 = SHK_SET]`, `DIGEST(SHK_SET)` at entry 4 and the
//!    anchors at 5 [and 6]; the anchor entry words `e_in = 4 + open`, `e_out = e_in + mode`;
//!    the `claim_pub` inputs are the three run outputs;
//! 5. lane map in stock order (`apply_lane_map`): a pair `(0, 0)` is skipped; otherwise lane
//!    and entry are u16 words, the entry exists and is a `DIGEST` or a `SPONGE(3)`,
//!    `lane < lanes`, (T9) `lane` is not 0 or 1, the lane is not already mapped and
//!    `s[lane] == 0`, then `s[lane] = vs[off + vals - 1]`;
//! 5b. (T9) the N-11 anchor prologue: the template lanes 2 to 9 equal key constants, lane 10
//!    (the `popeq` of the anchor cell) equals `a_in`; for a chunk the anchor-write template lanes
//!    (11 to 19, 21) equal key constants and lane 20 equals `a_out`;
//! 6. each `PUB(param)` entry: `s[param] == vs[off]`;
//! 7. the native replay of every `PERM`, `H2`, `SPONGE`, `DIGEST` and (T9, D-11) `PATH` entry: a
//!    tape of `P` permutation slots tied to the entries by a lookup of packed tuples with the
//!    fixed multiplicities `perms_i`, the sponge state carried from slot to slot; a `PATH(d)` slot
//!    `m` hashes `H2(l, r)` of the current node and the wrap-private sibling `m`, the index bit
//!    `m` choosing the order (node hash: the E3 `iface.rs` placeholder, LEAD), the first slot reads
//!    the leaf, the last checks the bits against the logged index and the output against the root.
//!
//! Steps 5, 6 and 7 read values through one log-derivative lookup into the tables
//! `(j, vs_j)` and `(V + l, s_l)` (`s` after the lane map). The multiplicities, the
//! witnessed values and the slot tuples are observed into the transcript before the two
//! challenges.
//!
//! The pre-hash word is pinned to a key constant (recommended option, for owner
//! confirmation). The other statement head words are only range checked.

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};
use num_bigint::BigUint;

use crate::{
    alg::Alg,
    native::{F, cap_fixed, low_u64, to_big},
    transcript::Transcript,
};

pub const STMT: usize = 116;
pub const LANE_MAP: usize = 106;
/// Owner ruling O7 (2026-11-13): 5 pairs.
pub const LANE_PAIRS: usize = 5;
pub const LAYOUT_WORD: usize = 89;
pub const X_WORD: usize = 97;
/// The N-11 prologue lanes (compiled `tools/sherlock-sdk/sherlock-ledger.compact`, compactc
/// 0.36.0, MEASURED; `sherlock-lfr` `iface.rs` holds the same constants).
pub const LANE_POPEQ: usize = 10;
pub const LANE_A_OUT: usize = 20;
/// Template lanes `(lane, value)` of every entry; a value `-k` is `r - k`.
pub const TPL_ORD: [(usize, i64); 8] = [(2, 0x30), (3, 0x50), (4, 1), (5, 1), (6, 0), (7, 0x0c), (8, 1), (9, -2)];
/// Template lanes of a chunk's anchor write.
pub const TPL_CHUNK: [(usize, i64); 10] =
    [(11, 0x10), (12, 1), (13, 1), (14, 1), (15, 0), (16, 0x11), (17, 1), (18, 1), (19, -2), (21, 0x91)];
pub fn tpl_f(v: i64) -> F {
    if v < 0 { -F::from((-v) as u64) } else { F::from(v as u64) }
}
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
pub const SHK_SET: u64 = 5;
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
pub const R_OFF: [usize; 7] = [0, 2, 4, 6, 10, 12, 14];
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
    /// `X`: the cached-trace commitment of `L` (T9; the DAG key check of ARCHITECTURE 1.11).
    pub x: F,
    /// `layout_d` (ENC-3), 8 BabyBear words.
    pub layout_d: [u64; 8],
}

impl DecodeShape {
    /// From a fixture's `decode` section (`dims`, `keys.x` hex, `keys.layout_d`) and the pinned
    /// pre-hash.
    pub fn from_doc(dd: &serde_json::Value, pre_hash: F) -> Self {
        let g = |k: &str| dd["dims"][k].as_u64().unwrap() as usize;
        let d = Dims { b: g("b"), v: g("v"), p: g("p"), l_tx: g("l_tx") };
        let k = &dd["keys"];
        let x = crate::native::f_from_hex(k["x"].as_str().expect("decode.keys.x")).expect("decode.keys.x hex");
        let ld: Vec<u64> = k["layout_d"].as_array().expect("decode.keys.layout_d").iter().map(|v| v.as_u64().unwrap()).collect();
        DecodeShape { d, pre_hash, x, layout_d: ld.try_into().expect("8 layout_d words") }
    }
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

/// A witnessed read of `vs[idx]` (0 when the gate is off); the caller adds the query.
fn read<A: Alg>(a: &mut A, vsv: &Value<Vec<F>>, idx: &A::V, gate: &A::V) -> Result<A::V, Error> {
    a.wit(a.val(idx).zip(a.val(gate)).zip(vsv.clone()).map(|((i, g), vs)| {
        if g == F::ONE { vs.get(low_u64(&i) as usize).copied().unwrap_or(F::ZERO) } else { F::ZERO }
    }))
}

/// Bit offsets of the packed replay tuple: bits 0 to 5 are the `PERM`, `H2`, `SPONGE`, `DIGEST`
/// selectors, the parity of `param` and the `PATH` selector; then `param` (8 bits), `o`, `poff`,
/// `perms` (9 bits each).
const T_PARAM: usize = 6;
const T_O: usize = 14;
const T_POFF: usize = 23;
const T_PERMS: usize = 32;

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

/// The decoder with no `PATH` siblings (a log with a `PATH` entry is refused).
pub fn decode<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    sh: &DecodeShape,
    pw: &[A::V],
) -> Result<Vec<A::V>, Error> {
    decode_with(a, t, sh, pw, &Value::known(vec![]))
}

/// The decoder. `pw` are the `L_Fr` public-value cells; `t` is the transcript that has
/// already observed them (the wrap: after `part4`); `sibs` are the wrap-private siblings, one
/// list per `PATH` entry in log order. Returns the call vector `s`.
pub fn decode_with<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    sh: &DecodeShape,
    pw: &[A::V],
    sibs: &Value<Vec<Vec<F>>>,
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
            if 8 * i + j >= d.values_at() || (X_WORD..X_WORD + LIMBS).contains(&(8 * i + j)) {
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
    // 2b. X with Canon (T10: `L_Fr` checks X against the L cached commitment it reads, lead
    // pick at T7c; on path C X differs per program); layout_d pinned (T9).
    let _xv = dec(a, &w[X_WORD..X_WORD + LIMBS], &r1)?;
    for i in 0..8 {
        a.assert_eq_c(&w[LAYOUT_WORD + i], F::from(sh.layout_d[i]), "layout_d is not the key constant")?;
    }

    // 3. Entries.
    let zero = a.cst(F::ZERO)?;
    let (mut off, mut np) = (zero.clone(), zero.clone());
    let mut offs = vec![];
    let mut ops: Vec<Vec<A::V>> = vec![];
    let mut params = vec![];
    let (mut poffs, mut permss, mut pars) = (vec![], vec![], vec![]);
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
        let chk = a.lin(&[(F::from(2u64), q.clone()), (-F::ONE, param.clone()), (-F::ONE, ev.clone())], F::ZERO)?;
        a.assert_eq_c(&chk, F::ZERO, "sponge slots")?;
        a.range(&q, &pow2(16), "sponge slots")?;
        let spq = a.mul(&c[I_SPONGE], &q)?;
        let pp = a.mul(&c[I_PATH], &param)?;
        let perms = a.lin(
            &[(F::ONE, c[1].clone()), (F::ONE, c[2].clone()), (F::ONE, c[I_DIGEST].clone()), (F::ONE, spq), (F::ONE, pp)],
            F::ZERO,
        )?;
        offs.push(off.clone());
        poffs.push(np.clone());
        off = a.add(&off, &vals)?;
        np = a.add(&np, &perms)?;
        permss.push(perms);
        pars.push(ev);
        ops.push(c);
        params.push(param);
    }
    let cap_v = a.lin(&[(-F::ONE, off.clone())], F::from(d.v as u64))?;
    a.range(&cap_v, &pow2(17), "capacity")?;
    let cap_p = a.lin(&[(-F::ONE, np.clone())], F::from(d.p as u64))?;
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
    assert!(d.b >= 7 && d.v >= R_OFF[6] + 2);
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
    // Entry 4: DIGEST in both forms; the opening-proof form (T9) is `open = [param_4 = SHK_SET]`
    // (t_in is 0 or 1, never 5), with the anchors at entries 5 and 6.
    a.assert_eq_c(&ops[4][I_DIGEST], F::ONE, "R entry 4")?;
    let p4 = a.add_c(&params[4], -F::from(SHK_SET))?;
    let open = is_zero(a, &p4, "R entry 4")?;
    let nopen = a.lin(&[(-F::ONE, open.clone())], F::ONE)?;
    let x4 = a.sub(&params[4], &t_in)?;
    let x4 = a.mul(&nopen, &x4)?;
    a.assert_eq_c(&x4, F::ZERO, "R entry 4")?;
    let om = a.mul(&open, &mode)?;
    let need5 = a.lin(&[(F::ONE, open.clone()), (F::ONE, mode.clone()), (-F::ONE, om.clone())], F::ZERO)?;
    let c5 = a.add_c(&ops[5][I_DIGEST], -F::ONE)?;
    let c5 = a.mul(&need5, &c5)?;
    a.assert_eq_c(&c5, F::ZERO, "R entry 5")?;
    let dt = a.sub(&t_in, &t_out)?;
    let want5 = a.mul_add(&open, &dt, &t_out)?;
    let p5 = a.sub(&params[5], &want5)?;
    let p5 = a.mul(&need5, &p5)?;
    a.assert_eq_c(&p5, F::ZERO, "R entry 5")?;
    let c6 = a.add_c(&ops[6][I_DIGEST], -F::ONE)?;
    let c6 = a.mul(&om, &c6)?;
    a.assert_eq_c(&c6, F::ZERO, "R entry 6")?;
    let p6 = a.sub(&params[6], &t_out)?;
    let p6 = a.mul(&om, &p6)?;
    a.assert_eq_c(&p6, F::ZERO, "R entry 6")?;
    let ei = a.sub(&w[E_IN_WORD], &open)?;
    a.assert_eq_c(&ei, F::from(ANCHOR_IN), "anchor entry words")?;
    let eo = a.lin(&[(F::ONE, w[E_OUT_WORD].clone()), (-F::ONE, mode.clone()), (-F::ONE, open.clone())], F::ZERO)?;
    a.assert_eq_c(&eo, F::from(ANCHOR_IN), "anchor entry words")?;
    // a_in: the output of entry 4 + open; a_out: of entry 5 + open for a chunk, else a_in.
    let d45 = a.sub(&vs[R_OFF[5] + 1], &vs[R_OFF[4] + 1])?;
    let a_in = a.mul_add(&open, &d45, &vs[R_OFF[4] + 1])?;
    let d56 = a.sub(&vs[R_OFF[6] + 1], &vs[R_OFF[5] + 1])?;
    let a_next = a.mul_add(&open, &d56, &vs[R_OFF[5] + 1])?;
    let dn = a.sub(&a_next, &a_in)?;
    let a_out = a.mul_add(&mode, &dn, &a_in)?;
    let sp = R_OFF[3];
    for i in 0..3 {
        a.assert_eq(&vs[sp + i], &vs[R_OFF[i] + 1], "claim_pub inputs are not the run DIGEST outputs")?;
    }
    // 5. Lane map, in stock order (`apply_lane_map`, E3 9539418). Queries into V:
    // (gate, index, value).
    let vsv = hv(a, &vs);
    let mut qv: Vec<(A::V, A::V, A::V)> = vec![];
    let mut s = lanes;
    let nl = d.lanes() as u64;
    let digs: Vec<A::V> = ops.iter().map(|c| c[I_DIGEST].clone()).collect();
    let sps: Vec<A::V> = ops.iter().map(|c| c[I_SPONGE].clone()).collect();
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
        // Lanes 0 (binding_input) and 1 (comm) are not effect lanes (T9).
        a.assert_eq_c(&h[0], F::ZERO, "lane map pair: lane 0 or 1")?;
        a.assert_eq_c(&h[1], F::ZERO, "lane map pair: lane 0 or 1")?;
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
        let val = read(a, &vsv, &idx, &nz)?;
        for (sl, hl) in s.iter_mut().zip(&h) {
            *sl = a.mul_add(hl, &val, sl)?;
        }
        qv.push((nz.clone(), idx, val));
    }

    // 5b. The N-11 anchor prologue (T9): template lanes, popeq lane = a_in; a chunk's anchor
    // write template and value lane = a_out.
    for (l, v) in TPL_ORD {
        a.assert_eq_c(&s[l], tpl_f(v), "anchor template lane")?;
    }
    a.assert_eq(&s[LANE_POPEQ], &a_in, "popeq lane is not a_in")?;
    for (l, v) in TPL_CHUNK {
        let e = a.add_c(&s[l], -tpl_f(v))?;
        let e = a.mul(&mode, &e)?;
        a.assert_eq_c(&e, F::ZERO, "anchor write template lane")?;
    }
    let e = a.sub(&s[LANE_A_OUT], &a_out)?;
    let e = a.mul(&mode, &e)?;
    a.assert_eq_c(&e, F::ZERO, "anchor write is not a_out")?;

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

    // 7. Native replay of every entry (stock: PERM, H2, SPONGE, DIGEST; PATH needs the
    // wrap-private siblings and is not replayed). A tape of P permutation slots: slot t is block
    // m = t - poff_i of the entry i with poff_i <= t < poff_i + perms_i. Each active slot
    // (t < np) carries the packed tuple (op bits, parity, param, o, poff, perms) of its entry;
    // a lookup with the fixed multiplicities perms_i ties the slots to the entries, and
    // 0 <= m < perms_i places them. The values are read through the V lookup.
    let tup = |a: &mut A, b: [&A::V; 6], param: &A::V, o: &A::V, poff: &A::V, prm: &A::V| -> Result<A::V, Error> {
        let mut t: Vec<(F, A::V)> = b.iter().enumerate().map(|(k, x)| (f2(k), (*x).clone())).collect();
        t.push((f2(T_PARAM), param.clone()));
        t.push((f2(T_O), o.clone()));
        t.push((f2(T_POFF), poff.clone()));
        t.push((f2(T_PERMS), prm.clone()));
        a.lin(&t, F::ZERO)
    };
    let mut rows = vec![];
    for i in 0..d.b {
        let c = &ops[i];
        rows.push(tup(a, [&c[1], &c[2], &c[I_SPONGE], &c[I_DIGEST], &pars[i], &c[I_PATH]], &params[i], &offs[i], &poffs[i], &permss[i])?);
    }
    // Host view of the PATH witness of each slot: (sibling, index bit) from the wrap-private
    // siblings and the logged index vs[o + 1] (honest witness; zero elsewhere).
    let host_ent: Value<Vec<(bool, usize, u64, u64)>> = Value::from_iter((0..d.b).map(|i| {
        a.val(&ops[i][I_PATH]).zip(a.val(&offs[i])).zip(a.val(&poffs[i])).zip(a.val(&permss[i])).map(
            |(((pb, o), p), q)| (pb == F::ONE, low_u64(&o) as usize, low_u64(&p), low_u64(&q)),
        )
    }));
    let path_wit: Value<Vec<(F, bool)>> = host_ent.zip(vsv.clone()).zip(sibs.clone()).map(|((ents, vs), sb)| {
        let mut out = vec![(F::ZERO, false); d.p];
        let mut k = 0usize;
        for &(is_path, o, p, q) in &ents {
            if !is_path {
                continue;
            }
            let idx = vs.get(o + 1).map(low_u64).unwrap_or(0);
            for m in 0..q {
                let t = (p + m) as usize;
                if t < d.p {
                    let sib = sb.get(k).and_then(|v| v.get(m as usize)).copied().unwrap_or(F::ZERO);
                    out[t] = (sib, m < 64 && (idx >> m) & 1 == 1);
                }
            }
            k += 1;
        }
        out
    });
    // Host view of the slot of each t (honest witness).
    let host_rows: Value<Vec<u64>> = Value::from_iter(rows.iter().map(|r| a.val(r).map(|x| low_u64(&x))));
    let host_pp: Value<Vec<(u64, u64)>> = Value::from_iter(
        poffs.iter().zip(&permss).map(|(p, q)| a.val(p).zip(a.val(q)).map(|(p, q)| (low_u64(&p), low_u64(&q)))),
    );
    let slot_tuples: Value<Vec<u64>> = host_rows.zip(host_pp).map(|(r, pp)| {
        (0..d.p as u64)
            .map(|t| pp.iter().position(|&(p, q)| p <= t && t < p + q).map(|i| r[i]).unwrap_or(0))
            .collect()
    });
    let npv = a.val(&np).map(|x| low_u64(&x) as usize);
    let (cap02, cap42) = (a.cst(cap_fixed(0, 2))?, a.cst(cap_fixed(4, 2))?);
    let mut prev = [zero.clone(), zero.clone(), zero.clone()];
    let (mut pw_prev, mut acc_prev) = (zero.clone(), zero.clone());
    let mut slots: Vec<(A::V, A::V)> = vec![];
    let mut act_prev: Option<A::V> = None;
    let mut act_sum = vec![];
    for ti in 0..d.p {
        let tf = F::from(ti as u64);
        let ab = a.wit_bit(npv.map(|n| ti < n))?;
        let act = a.bit_v(&ab)?;
        if let Some(p) = &act_prev {
            // Monotone: act_t = 1 implies act_(t-1) = 1.
            let m = a.mul(p, &act)?;
            a.assert_eq(&m, &act, "replay slots")?;
        }
        act_prev = Some(act.clone());
        act_sum.push(act.clone());
        let tv = slot_tuples.as_ref().map(|v| v[ti]);
        let field = |x: u64, sh: usize, w: usize| (x >> sh) & ((1u64 << w) - 1);
        let mut bits = vec![];
        for k in 0..6 {
            let b = a.wit_bit(tv.map(|x| field(x, k, 1) == 1))?;
            bits.push(a.bit_v(&b)?);
        }
        let param = a.wit(tv.map(|x| F::from(field(x, T_PARAM, 8))))?;
        a.range(&param, &pow2(8), "replay slots")?;
        let o = a.wit(tv.map(|x| F::from(field(x, T_O, 9))))?;
        let poff = a.wit(tv.map(|x| F::from(field(x, T_POFF, 9))))?;
        let prm = a.wit(tv.map(|x| F::from(field(x, T_PERMS, 9))))?;
        for x in [&o, &poff, &prm] {
            a.range(x, &pow2(9), "replay slots")?;
        }
        let pk = tup(a, [&bits[0], &bits[1], &bits[2], &bits[3], &bits[4], &bits[5]], &param, &o, &poff, &prm)?;
        // An inactive slot holds the zero tuple.
        let na = a.lin(&[(-F::ONE, act.clone())], F::ONE)?;
        let z = a.mul(&na, &pk)?;
        a.assert_eq_c(&z, F::ZERO, "replay slots")?;
        let [c_perm, c_h2, c_sp, c_dig, par, c_path] = [&bits[0], &bits[1], &bits[2], &bits[3], &bits[4], &bits[5]];
        // m = t - poff with 0 <= m < perms (active slots).
        let tm = a.lin(&[(-F::ONE, poff.clone())], tf)?;
        let d1 = a.mul(&act, &tm)?;
        a.range(&d1, &pow2(9), "replay slots")?;
        let rem = a.lin(&[(F::ONE, prm.clone()), (-F::ONE, tm.clone())], -F::ONE)?;
        let d2 = a.mul(&act, &rem)?;
        a.range(&d2, &pow2(9), "replay slots")?;
        let first = is_zero(a, &tm, "replay slots")?;
        let last = is_zero(a, &rem, "replay slots")?;
        let nf = a.lin(&[(-F::ONE, first.clone())], F::ONE)?;
        // has1: the block has a second input (not the last block of an odd SPONGE).
        let lp = a.mul(&last, par)?;
        let has1 = a.lin(&[(-F::ONE, lp)], F::ONE)?;
        let sp1 = a.mul(c_sp, &has1)?;
        let spl = a.mul(c_sp, &last)?;
        // PATH (T9): the first slot reads the leaf vs[o], the last the index vs[o + 1] and the
        // root vs[o + 2].
        let pf = a.mul(c_path, &first)?;
        let pl = a.mul(c_path, &last)?;
        // Reads: y_k = vs[o + 2m + k] (k < 3), y3 = vs[o + 3] (PERM) or vs[o + param] (SPONGE
        // output) or vs[o + 2] (PATH root), y4 = vs[o + 4] (PERM) or vs[o + 1] (PATH index),
        // y5 = vs[o + 5] (PERM).
        let g0 = a.lin(
            &[(F::ONE, c_perm.clone()), (F::ONE, c_h2.clone()), (F::ONE, c_sp.clone()), (F::ONE, c_dig.clone()), (F::ONE, pf.clone())],
            F::ZERO,
        )?;
        let g1 = a.lin(&[(F::ONE, c_perm.clone()), (F::ONE, c_h2.clone()), (F::ONE, c_dig.clone()), (F::ONE, sp1.clone())], F::ZERO)?;
        let g2 = a.add(c_perm, c_h2)?;
        let g3 = a.lin(&[(F::ONE, c_perm.clone()), (F::ONE, spl.clone()), (F::ONE, pl.clone())], F::ZERO)?;
        let g4 = a.add(c_perm, &pl)?;
        let i0 = a.lin(&[(F::ONE, o.clone()), (F::from(2u64), tm.clone())], F::ZERO)?;
        let i1 = a.add_c(&i0, F::ONE)?;
        let i2 = a.add_c(&i0, F::from(2u64))?;
        let op3 = a.add_c(&o, F::from(3u64))?;
        let pm3 = a.mul(c_perm, &op3)?;
        let opp = a.add(&o, &param)?;
        let i3 = a.mul_add(c_sp, &opp, &pm3)?;
        let o2 = a.add_c(&o, F::from(2u64))?;
        let i3 = a.mul_add(&pl, &o2, &i3)?;
        // vs[o + 4] for PERM, vs[o + 1] for the last PATH slot.
        let i4 = a.lin(&[(F::ONE, o.clone()), (-F::from(3u64), pl.clone())], F::from(4u64))?;
        let i5 = a.add_c(&o, F::from(5u64))?;
        let mut y = vec![];
        for (g, i) in [(&g0, &i0), (&g1, &i1), (&g2, &i2), (&g3, &i3), (&g4, &i4), (c_perm, &i5)] {
            let v = read(a, &vsv, i, g)?;
            qv.push((g.clone(), i.clone(), v.clone()));
            y.push(v);
        }
        // DIGEST tag.
        let pd = a.mul(c_dig, &param)?;
        let tb = one_hot(a, N_TAGS as usize, &pd, c_dig, "replay slots")?;
        let tt: Vec<(F, A::V)> = tb.iter().enumerate().map(|(k, b)| (tag_fr(k as u64), b.clone())).collect();
        let tag = a.lin(&tt, F::ZERO)?;
        // Permutation input.
        let pse = a.lin(&[(F::ONE, c_perm.clone()), (F::ONE, c_h2.clone()), (F::ONE, c_sp.clone())], F::ZERO)?;
        let spn = a.mul(c_sp, &nf)?;
        let x = a.mul(&pse, &y[0])?;
        let x = a.mul_add(c_dig, &tag, &x)?;
        let in0 = a.mul_add(&spn, &prev[0], &x)?;
        let ph = a.add(c_perm, c_h2)?;
        let x = a.mul(&ph, &y[1])?;
        let x = a.mul_add(c_dig, &y[0], &x)?;
        let x = a.mul_add(&sp1, &y[1], &x)?;
        let in1 = a.mul_add(&spn, &prev[1], &x)?;
        let x = a.mul(c_perm, &y[2])?;
        let x = a.mul_add(c_h2, &cap02, &x)?;
        let x = a.mul_add(c_dig, &cap42, &x)?;
        let sf = a.mul(c_sp, &first)?;
        let x = a.mul_add(&sf, &param, &x)?;
        let in2 = a.mul_add(&spn, &prev[2], &x)?;
        // PATH slot m: cur = leaf (first) or the previous slot's output; (l, r) = (cur, sib) or
        // (sib, cur) by the index bit b_m; input (l, r, cap(0, 2)); the bits sum to the index.
        let pwv = path_wit.as_ref().map(|v| v[ti]);
        let sib = a.wit(pwv.map(|x| x.0))?;
        let bb = a.wit_bit(pwv.map(|x| x.1))?;
        let b = a.bit_v(&bb)?;
        let dcur = a.sub(&y[0], &prev[0])?;
        let cur = a.mul_add(&first, &dcur, &prev[0])?;
        let ds = a.sub(&sib, &cur)?;
        let xx = a.mul(&b, &ds)?;
        let pl_l = a.add(&cur, &xx)?;
        let pl_r = a.sub(&sib, &xx)?;
        let in0 = a.mul_add(c_path, &pl_l, &in0)?;
        let in1 = a.mul_add(c_path, &pl_r, &in1)?;
        let in2 = a.mul_add(c_path, &cap02, &in2)?;
        // index accumulator: pw_m = 2^m (1 on the first slot), acc_m = acc_(m-1) + b_m pw_m.
        let two = a.lin(&[(F::from(2u64), pw_prev.clone())], F::ZERO)?;
        let om1 = a.lin(&[(-F::from(2u64), pw_prev.clone())], F::ONE)?;
        let pw_m = a.mul_add(&first, &om1, &two)?;
        let ac = a.mul(&nf, &acc_prev)?;
        let acc = a.mul_add(&b, &pw_m, &ac)?;
        pw_prev = pw_m;
        acc_prev = acc.clone();
        // A bit outside a PATH slot is zero (no free witness on other slots).
        let npb = a.lin(&[(-F::ONE, c_path.clone())], F::ONE)?;
        let zb = a.mul(&npb, &b)?;
        a.assert_eq_c(&zb, F::ZERO, "native replay")?;
        let out = a.permute(&[in0, in1, in2])?;
        // Outputs.
        let chk = |a: &mut A, g: &A::V, u: &A::V, v: &A::V| -> Result<(), Error> {
            let dd = a.sub(u, v)?;
            let dd = a.mul(g, &dd)?;
            a.assert_eq_c(&dd, F::ZERO, "native replay")
        };
        for k in 0..3 {
            chk(a, c_perm, &out[k], &y[3 + k])?;
        }
        chk(a, c_h2, &out[0], &y[2])?;
        chk(a, c_dig, &out[0], &y[1])?;
        chk(a, &spl, &out[0], &y[3])?;
        chk(a, &pl, &out[0], &y[3])?;
        chk(a, &pl, &acc, &y[4])?;
        prev = out;
        slots.push((act, pk));
    }
    let asum = sum(a, &act_sum)?;
    a.assert_eq(&asum, &np, "replay slots")?;

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
    for (_, pk) in &slots {
        t.observe(a, pk)?;
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
    // Slots against entries: sum_t act_t / (alpha - pk_t) = sum_i perms_i / (alpha - row_i).
    let inv1 = |a: &mut A, x: &A::V| -> Result<A::V, Error> {
        let den = a.sub(&alpha, x)?;
        let inv = a.wit(a.val(&den).map(|x| x.invert().unwrap_or(F::ZERO)))?;
        let one = a.mul(&den, &inv)?;
        a.assert_eq_c(&one, F::ONE, "lookup denominator")?;
        Ok(inv)
    };
    let mut l2 = a.cst(F::ZERO)?;
    for (act, pk) in &slots {
        let inv = inv1(a, pk)?;
        l2 = a.mul_add(act, &inv, &l2)?;
    }
    let mut r2 = a.cst(F::ZERO)?;
    for (r, m) in rows.iter().zip(&permss) {
        let inv = inv1(a, r)?;
        r2 = a.mul_add(m, &inv, &r2)?;
    }
    a.assert_eq(&l2, &r2, "native replay")?;
    Ok(s)
}
