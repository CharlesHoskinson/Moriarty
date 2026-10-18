//! Exact UInt128, Bool and wide-sum gadgets.
//!
//! Field requirement: the scalar modulus `r` exceeds 2^196 (checked by
//! `circuit::compile` through `MIN_MODULUS_BITS`). Every equation below compares
//! integers whose absolute values stay below 2^195, so equality in the field is
//! equality of integers. No gadget lets a field reduction stand in for u128 overflow.
//!
//! Representation:
//! - [`UIntW`]: `lo`, `hi` linear combinations with `value = lo + 2^64 * hi`. Every
//!   non-constant UIntW is created by [`alloc_uint`], which proves `lo, hi < 2^64` by
//!   128 Boolean bits, so every UInt wire is an integer in `[0, 2^128)` and its limbs
//!   are bit sums (or constants).
//! - [`BoolW`]: a linear combination equal to 0 or 1, by booleanity constraint or by
//!   construction from Boolean wires.
//! - [`WideW`]: an exact sum below 2^144 in three bit-sum limbs (accounting summary).
//!
//! Constants fold at compile time with Core's checked arithmetic. A constant that
//! fails (for example `1 - 2`) is a static failure: the action can never succeed.
//!
//! Honest-value handling: `val` is the proposed assignment. When Core would refuse,
//! the proposal uses wrapping arithmetic so that synthesis proceeds and the
//! constraint system is unsatisfied; the host never decides acceptance. Circuit
//! structure depends only on `konst` (compile-time constants), never on `val`, so setup
//! and proving synthesize the same matrices.
use crate::r1cs::{Cs, Lc, fe, fe_bool, two64, two128};
use crate::statement::Wide;
use ark_ff::PrimeField;
use ark_relations::r1cs::SynthesisError;

#[derive(Debug)]
pub(crate) enum GadgetError {
    Synthesis(SynthesisError),
    /// A constant-only operation refuses for every input.
    Static(String),
}

impl From<SynthesisError> for GadgetError {
    fn from(e: SynthesisError) -> Self {
        Self::Synthesis(e)
    }
}

type G<T> = Result<T, GadgetError>;

#[derive(Clone, Debug)]
pub(crate) struct BoolW<F: PrimeField> {
    pub lc: Lc<F>,
    pub val: Option<bool>,
    pub konst: Option<bool>,
}

#[derive(Clone, Debug)]
pub(crate) struct UIntW<F: PrimeField> {
    pub lo: Lc<F>,
    pub hi: Lc<F>,
    pub val: Option<u128>,
    pub konst: Option<u128>,
}

#[derive(Clone, Debug)]
pub(crate) struct WideW<F: PrimeField> {
    pub lo: Lc<F>,
    pub mid: Lc<F>,
    pub top: Lc<F>,
}

#[derive(Clone, Debug)]
pub(crate) enum CVal<F: PrimeField> {
    U(UIntW<F>),
    B(BoolW<F>),
}

impl<F: PrimeField> BoolW<F> {
    pub fn constant(b: bool) -> Self {
        Self { lc: Lc::constant(fe_bool(b)), val: Some(b), konst: Some(b) }
    }
}

impl<F: PrimeField> UIntW<F> {
    pub fn constant(v: u128) -> Self {
        Self {
            lo: Lc::constant(fe(v & u128::from(u64::MAX))),
            hi: Lc::constant(fe(v >> 64)),
            val: Some(v),
            konst: Some(v),
        }
    }
    /// `lo + 2^64 * hi`.
    pub fn lc(&self) -> Lc<F> {
        self.lo.clone().plus(&self.hi.scaled(two64()))
    }
    fn limb(&self, high: bool) -> Limb<F> {
        let pick = |v: u128| if high { (v >> 64) as u64 } else { v as u64 };
        Limb {
            lc: if high { self.hi.clone() } else { self.lo.clone() },
            val: self.val.map(pick),
            konst: self.konst.map(pick),
        }
    }
}

impl<F: PrimeField> WideW<F> {
    pub fn constant(w: Wide) -> Self {
        Self {
            lo: Lc::constant(fe(w.lo & u128::from(u64::MAX))),
            mid: Lc::constant(fe(w.lo >> 64)),
            top: Lc::constant(fe(u128::from(w.top))),
        }
    }
    /// `lo + 2^64 * mid + 2^128 * top`.
    pub fn lc(&self) -> Lc<F> {
        self.lo.clone().plus(&self.mid.scaled(two64())).plus(&self.top.scaled(two128()))
    }
}

struct Limb<F: PrimeField> {
    lc: Lc<F>,
    val: Option<u64>,
    konst: Option<u64>,
}

/// Boolean witness: `b * b = b`.
pub(crate) fn alloc_bool<F: PrimeField>(cs: &mut Cs<F>, role: &str, val: Option<bool>) -> G<BoolW<F>> {
    let v = cs.witness(role, val.map(fe_bool))?;
    cs.enforce(Lc::var(v), Lc::var(v), Lc::var(v))?;
    Ok(BoolW { lc: Lc::var(v), val, konst: None })
}

/// Range-proven UInt128 witness: 128 Boolean bits, `lo = sum_{i<64} 2^i b_i`,
/// `hi = sum_{i<64} 2^i b_{64+i}`. Cost: 128 constraints.
pub(crate) fn alloc_uint<F: PrimeField>(cs: &mut Cs<F>, role: &str, val: Option<u128>) -> G<UIntW<F>> {
    let mut lo = Lc::zero();
    let mut hi = Lc::zero();
    for i in 0..128u32 {
        let bit = val.map(|v| (v >> i) & 1 == 1);
        let b = cs.witness(&format!("{role}.bit{i}"), bit.map(fe_bool))?;
        cs.enforce(Lc::var(b), Lc::var(b), Lc::var(b))?;
        let weight = fe::<F>(1u128 << (i % 64));
        if i < 64 {
            lo = lo.term(weight, b);
        } else {
            hi = hi.term(weight, b);
        }
    }
    Ok(UIntW { lo, hi, val, konst: None })
}

fn both<A: Copy, B: Copy>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    match (a, b) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

/// Checked addition. Constraint: `a + b = c` with `c` range-proven. Since
/// `a + b < 2^129 < r`, the field equation is an integer equation, and `c < 2^128`
/// excludes overflow. Unsatisfiable exactly when Core's `checked_add` fails.
pub(crate) fn add<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &UIntW<F>, b: &UIntW<F>) -> G<UIntW<F>> {
    if let Some((x, y)) = both(a.konst, b.konst) {
        return x.checked_add(y).map(UIntW::constant).ok_or_else(|| GadgetError::Static("constant UInt addition exceeds u128".into()));
    }
    let val = both(a.val, b.val).map(|(x, y)| x.wrapping_add(y));
    let c = alloc_uint(cs, &format!("{role}.c"), val)?;
    cs.enforce_eq(&a.lc().plus(&b.lc()), &c.lc())?;
    Ok(c)
}

/// Checked subtraction. Constraint: `b + c = a` with `c` range-proven, so `a >= b`
/// and `c = a - b` as integers (`b + c < 2^129 < r`). Unsatisfiable exactly when
/// Core's `checked_sub` fails.
pub(crate) fn sub<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &UIntW<F>, b: &UIntW<F>) -> G<UIntW<F>> {
    if let Some((x, y)) = both(a.konst, b.konst) {
        return x.checked_sub(y).map(UIntW::constant).ok_or_else(|| GadgetError::Static("constant UInt subtraction is negative".into()));
    }
    let val = both(a.val, b.val).map(|(x, y)| x.wrapping_sub(y));
    let c = alloc_uint(cs, &format!("{role}.c"), val)?;
    cs.enforce_eq(&b.lc().plus(&c.lc()), &a.lc())?;
    Ok(c)
}

/// Product of two limbs (< 2^64 each), exact in the field (< 2^128). A constant limb
/// makes it linear; otherwise one witness and one constraint `x * y = p`.
fn mul_limb<F: PrimeField>(cs: &mut Cs<F>, role: &str, x: &Limb<F>, y: &Limb<F>) -> G<Lc<F>> {
    if let Some(k) = x.konst {
        return Ok(y.lc.scaled(fe(u128::from(k))));
    }
    if let Some(k) = y.konst {
        return Ok(x.lc.scaled(fe(u128::from(k))));
    }
    let val = both(x.val, y.val).map(|(p, q)| u128::from(p) * u128::from(q));
    let p = cs.witness(role, val.map(fe))?;
    cs.enforce(x.lc.clone(), y.lc.clone(), Lc::var(p))?;
    Ok(Lc::var(p))
}

/// Checked multiplication with the 256-bit intermediate made explicit.
///
/// With `a = a0 + 2^64 a1`, `b = b0 + 2^64 b1` (limbs < 2^64):
/// `a*b = a0 b0 + 2^64 (a0 b1 + a1 b0) + 2^128 a1 b1`.
/// Constraints: `a1 * b1 = 0` (an integer equation, the product being < 2^128) and
/// `c = a0 b0 + 2^64 (a0 b1 + a1 b0)` with `c` range-proven. The right side is below
/// 2^128 + 2^193 < r, so it is an integer equation; together they give `c = a*b < 2^128`.
/// If `a*b < 2^128` then `a1 = 0` or `b1 = 0`, so the system is satisfiable exactly
/// when Core's `checked_mul` succeeds. A naive `a * b = c` field check would accept
/// `c = a*b mod r` whenever that value is below 2^128 (AUTHOR-EXPECTATIONS A11); this
/// gadget cannot.
pub(crate) fn mul<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &UIntW<F>, b: &UIntW<F>) -> G<UIntW<F>> {
    if let Some((x, y)) = both(a.konst, b.konst) {
        return x.checked_mul(y).map(UIntW::constant).ok_or_else(|| GadgetError::Static("constant UInt multiplication exceeds u128".into()));
    }
    let (a0, a1, b0, b1) = (a.limb(false), a.limb(true), b.limb(false), b.limb(true));
    let p00 = mul_limb(cs, &format!("{role}.p00"), &a0, &b0)?;
    let p01 = mul_limb(cs, &format!("{role}.p01"), &a0, &b1)?;
    let p10 = mul_limb(cs, &format!("{role}.p10"), &a1, &b0)?;
    let high_zero = a1.konst == Some(0) || b1.konst == Some(0);
    if !high_zero {
        let p11 = mul_limb(cs, &format!("{role}.p11"), &a1, &b1)?;
        cs.enforce_zero(p11)?;
    }
    let val = both(a.val, b.val).map(|(x, y)| x.wrapping_mul(y));
    let c = alloc_uint(cs, &format!("{role}.c"), val)?;
    let rhs = p00.plus(&p01.plus(&p10).scaled(two64()));
    cs.enforce_eq(&c.lc(), &rhs)?;
    Ok(c)
}

/// `a < b` for UInt. Witnesses: Boolean `f` and range-proven `t`.
/// Constraint: `f * (2(a - b) + 1) = (a - b) - t`.
/// `f = 0` gives `t = a - b`, forcing `a >= b`; `f = 1` gives `t = b - a - 1`,
/// forcing `a < b`. Because `|a - b| < 2^128` and `r > 2^130`, a wrong `f` would need
/// `t` congruent to a negative integer above `-2^129`, i.e. `t >= r - 2^129 > 2^128`,
/// which the range proof excludes. So `f` is unique.
pub(crate) fn lt<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &UIntW<F>, b: &UIntW<F>) -> G<BoolW<F>> {
    if let Some((x, y)) = both(a.konst, b.konst) {
        return Ok(BoolW::constant(x < y));
    }
    let pair = both(a.val, b.val);
    let f = alloc_bool(cs, &format!("{role}.f"), pair.map(|(x, y)| x < y))?;
    let t_val = pair.map(|(x, y)| if x < y { y - x - 1 } else { x - y });
    let t = alloc_uint(cs, &format!("{role}.t"), t_val)?;
    let d = a.lc().minus(&b.lc());
    let two = fe::<F>(2);
    cs.enforce(f.lc.clone(), d.scaled(two).plus(&Lc::one()), d.minus(&t.lc()))?;
    Ok(f)
}

/// `a == b` for UInt. Witnesses `f`, `inv`; constraints `d * inv = 1 - f`, `d * f = 0`
/// with `d = a - b`. `d != 0` forces `f = 0`; `d = 0` forces `f = 1`. `d = 0` in the
/// field iff `a = b`, because `|a - b| < 2^128 < r`.
pub(crate) fn eq_uint<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &UIntW<F>, b: &UIntW<F>) -> G<BoolW<F>> {
    if let Some((x, y)) = both(a.konst, b.konst) {
        return Ok(BoolW::constant(x == y));
    }
    let pair = both(a.val, b.val);
    let f_val = pair.map(|(x, y)| x == y);
    let inv_val = pair.map(|(x, y)| {
        if x == y { F::from(0u64) } else { (fe::<F>(x) - fe::<F>(y)).inverse().expect("nonzero difference") }
    });
    let f = cs.witness(&format!("{role}.f"), f_val.map(fe_bool))?;
    let inv = cs.witness(&format!("{role}.inv"), inv_val)?;
    let d = a.lc().minus(&b.lc());
    cs.enforce(d.clone(), Lc::var(inv), Lc::one().minus(&Lc::var(f)))?;
    cs.enforce(d, Lc::var(f), Lc::zero())?;
    Ok(BoolW { lc: Lc::var(f), val: f_val, konst: None })
}

pub(crate) fn not<F: PrimeField>(a: &BoolW<F>) -> BoolW<F> {
    BoolW { lc: Lc::one().minus(&a.lc), val: a.val.map(|v| !v), konst: a.konst.map(|v| !v) }
}

/// Product of two Booleans; Boolean when both inputs are.
fn bool_product<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &BoolW<F>, b: &BoolW<F>) -> G<Lc<F>> {
    let val = both(a.val, b.val).map(|(x, y)| x && y);
    let p = cs.witness(role, val.map(fe_bool))?;
    cs.enforce(a.lc.clone(), b.lc.clone(), Lc::var(p))?;
    Ok(Lc::var(p))
}

/// Strict conjunction: both operands are already lowered by the caller (Core evaluates
/// both); only the combination folds when an operand is constant.
pub(crate) fn and<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &BoolW<F>, b: &BoolW<F>) -> G<BoolW<F>> {
    match (a.konst, b.konst) {
        (Some(false), _) | (_, Some(false)) => Ok(BoolW::constant(false)),
        (Some(true), _) => Ok(b.clone()),
        (_, Some(true)) => Ok(a.clone()),
        _ => {
            let p = bool_product(cs, &format!("{role}.p"), a, b)?;
            Ok(BoolW { lc: p, val: both(a.val, b.val).map(|(x, y)| x && y), konst: None })
        }
    }
}

/// Strict disjunction: `a + b - a*b`.
pub(crate) fn or<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &BoolW<F>, b: &BoolW<F>) -> G<BoolW<F>> {
    match (a.konst, b.konst) {
        (Some(true), _) | (_, Some(true)) => Ok(BoolW::constant(true)),
        (Some(false), _) => Ok(b.clone()),
        (_, Some(false)) => Ok(a.clone()),
        _ => {
            let p = bool_product(cs, &format!("{role}.p"), a, b)?;
            let lc = a.lc.clone().plus(&b.lc).minus(&p);
            Ok(BoolW { lc, val: both(a.val, b.val).map(|(x, y)| x || y), konst: None })
        }
    }
}

/// Bool equality: `1 - a - b + 2ab`.
pub(crate) fn eq_bool<F: PrimeField>(cs: &mut Cs<F>, role: &str, a: &BoolW<F>, b: &BoolW<F>) -> G<BoolW<F>> {
    match (a.konst, b.konst) {
        (Some(x), Some(y)) => Ok(BoolW::constant(x == y)),
        (Some(true), _) => Ok(b.clone()),
        (Some(false), _) => Ok(not(b)),
        (_, Some(true)) => Ok(a.clone()),
        (_, Some(false)) => Ok(not(a)),
        _ => {
            let p = bool_product(cs, &format!("{role}.p"), a, b)?;
            let lc = Lc::one().minus(&a.lc).minus(&b.lc).plus(&p.scaled(fe(2)));
            Ok(BoolW { lc, val: both(a.val, b.val).map(|(x, y)| x == y), konst: None })
        }
    }
}

/// `b = 1`. A constant false is a static failure.
pub(crate) fn assert_true<F: PrimeField>(cs: &mut Cs<F>, b: &BoolW<F>, what: &str) -> G<()> {
    match b.konst {
        Some(true) => Ok(()),
        Some(false) => Err(GadgetError::Static(format!("{what} is constantly false"))),
        None => {
            cs.enforce_eq(&b.lc, &Lc::one())?;
            Ok(())
        }
    }
}

/// Exact sum of UInt wires as `lo + 2^64 mid + 2^128 top` with `lo, mid < 2^64` and
/// `top < 2^16`. At most 2^16 terms each below 2^128 sum below 2^144 < r, so the single
/// equation is an integer equation and the 144-bit decomposition makes the three limbs
/// canonical. No overflow refusal exists here: Core's balance semantics never sums
/// effects, so the summary must not be stricter than `simulate`. Cost: 145 constraints.
pub(crate) fn sum_wide<F: PrimeField>(cs: &mut Cs<F>, role: &str, terms: &[&UIntW<F>]) -> G<WideW<F>> {
    if terms.len() > (1 << 16) {
        return Err(GadgetError::Static("accounting summary over more than 2^16 amounts".into()));
    }
    let mut total = Lc::zero();
    let mut val = Some(Wide::ZERO);
    let mut konst = Some(Wide::ZERO);
    for t in terms {
        total = total.plus(&t.lc());
        val = both(val, t.val).map(|(w, x)| w.plus(x));
        konst = both(konst, t.konst).map(|(w, x)| w.plus(x));
    }
    if let Some(k) = konst {
        return Ok(WideW::constant(k));
    }
    let mut lo = Lc::zero();
    let mut mid = Lc::zero();
    let mut top = Lc::zero();
    for i in 0..144u32 {
        let bit = val.map(|w| w.bit(i));
        let b = cs.witness(&format!("{role}.bit{i}"), bit.map(fe_bool))?;
        cs.enforce(Lc::var(b), Lc::var(b), Lc::var(b))?;
        let weight = fe::<F>(1u128 << (i % 64));
        match i / 64 {
            0 => lo = lo.term(weight, b),
            1 => mid = mid.term(weight, b),
            _ => top = top.term(weight, b),
        }
    }
    let w = WideW { lo, mid, top };
    cs.enforce_eq(&total, &w.lc())?;
    Ok(w)
}
