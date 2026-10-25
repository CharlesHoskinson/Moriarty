// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
//! Fp12 products computed "flat": every output coordinate (over Fp) is one
//! signed sum of Fp products, read off the multiplication tensor of the tower.
//! With the zk-stdlib's emulated-field chip a multiplication costs about 15
//! rows while every addition costs 7 and every normalization about 14, so
//! schoolbook products with a single sum per output (and one normalization
//! per output) beat Karatsuba towers, whose savings in multiplications are
//! paid back in additions and normalizations.

use std::sync::OnceLock;

use ff::Field;
use midnight_curves::{
    Fp,
    bls12_381::{Fp2, Fp6, Fp12},
};
use midnight_proofs::{circuit::Layouter, plonk::Error};

use crate::tower::{AFp, AFp2, AFp6, AFp12, F, Tower};

/// A coordinate that is either an assigned value or a constant.
#[derive(Clone, Debug)]
pub enum Co {
    V(AFp),
    K(Fp),
}

/// Coordinate index: 6*c + 2*j + t for Fp12 = (c0, c1), Fp6 = (j=0,1,2),
/// Fp2 = (t=0,1).
pub fn to_coords(f: &Fp12) -> [Fp; 12] {
    let mut out = [Fp::ZERO; 12];
    for (c, f6) in [f.c0(), f.c1()].iter().enumerate() {
        for (j, f2) in [f6.c0(), f6.c1(), f6.c2()].iter().enumerate() {
            out[6 * c + 2 * j] = f2.c0();
            out[6 * c + 2 * j + 1] = f2.c1();
        }
    }
    out
}

pub fn from_coords(x: &[Fp; 12]) -> Fp12 {
    let f6 = |o: usize| {
        Fp6::new(
            Fp2::new(x[o], x[o + 1]),
            Fp2::new(x[o + 2], x[o + 3]),
            Fp2::new(x[o + 4], x[o + 5]),
        )
    };
    Fp12::new(f6(0), f6(6))
}

fn small_int(x: Fp) -> i64 {
    for n in 0..64i64 {
        if x == Fp::from(n as u64) {
            return n;
        }
        if x == -Fp::from(n as u64) {
            return -n;
        }
    }
    panic!("tensor entry is not a small integer");
}

/// T[i][j] = list of (k, coefficient) with e_i e_j = sum_k coefficient e_k.
pub type ProductTensor = Vec<Vec<Vec<(usize, i64)>>>;

pub fn tensor() -> &'static ProductTensor {
    static T: OnceLock<ProductTensor> = OnceLock::new();
    T.get_or_init(|| {
        let basis = |i: usize| {
            let mut c = [Fp::ZERO; 12];
            c[i] = Fp::ONE;
            from_coords(&c)
        };
        (0..12)
            .map(|i| {
                (0..12)
                    .map(|j| {
                        let p = to_coords(&(basis(i) * basis(j)));
                        (0..12)
                            .filter_map(|k| {
                                let n = small_int(p[k]);
                                (n != 0).then_some((k, n))
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect()
    })
}

pub fn afp12_coords(a: &AFp12) -> [AFp; 12] {
    let mut v = Vec::with_capacity(12);
    for f6 in [&a.c0, &a.c1] {
        for f2 in [&f6.c0, &f6.c1, &f6.c2] {
            v.push(f2.c0.clone());
            v.push(f2.c1.clone());
        }
    }
    v.try_into().unwrap()
}

pub fn afp12_from_coords(x: [AFp; 12]) -> AFp12 {
    let x: Vec<AFp> = x.into();
    let f2 = |o: usize| AFp2 {
        c0: x[o].clone(),
        c1: x[o + 1].clone(),
    };
    let f6 = |o: usize| AFp6 {
        c0: f2(o),
        c1: f2(o + 2),
        c2: f2(o + 4),
    };
    AFp12 {
        c0: f6(0),
        c1: f6(6),
    }
}

impl<'a> Tower<'a> {
    /// out = x * y in Fp12, flat. If `square`, x and y must be the same and
    /// only the products x_i x_j with i <= j are computed.
    pub fn flat_mul(
        &self,
        l: &mut impl Layouter<F>,
        x: &[Co; 12],
        y: &[Co; 12],
        square: bool,
    ) -> Result<AFp12, Error> {
        let t = tensor();
        let nf = |l: &mut _, c: &Co| -> Result<Co, Error> {
            Ok(match c {
                Co::V(a) => Co::V(self.fp_nf(l, a)?),
                Co::K(k) => Co::K(*k),
            })
        };
        let mut xs = Vec::with_capacity(12);
        for c in x {
            xs.push(nf(l, c)?);
        }
        let ys = if square {
            xs.clone()
        } else {
            let mut ys = Vec::with_capacity(12);
            for c in y {
                ys.push(nf(l, c)?);
            }
            ys
        };
        let mut pos: Vec<Vec<AFp>> = vec![vec![]; 12];
        let mut neg: Vec<Vec<AFp>> = vec![vec![]; 12];
        let mut konst = [Fp::ZERO; 12];
        for i in 0..12 {
            for j in 0..12 {
                if square && j < i {
                    continue;
                }
                // coefficients of x_i y_j (and x_j y_i when squaring)
                let mut coefs: Vec<(usize, i64)> = t[i][j].clone();
                if square && j != i {
                    coefs.extend(t[j][i].iter().cloned());
                }
                let mut acc = [0i64; 12];
                for (k, c) in coefs {
                    acc[k] += c;
                }
                if acc.iter().all(|c| *c == 0) {
                    continue;
                }
                // the product, as a term with an integer or constant factor
                let (term, kval): (Option<AFp>, Fp) = match (&xs[i], &ys[j]) {
                    (Co::K(a), Co::K(b)) => (None, *a * *b),
                    (Co::V(a), Co::K(b)) | (Co::K(b), Co::V(a)) => {
                        if *b == Fp::ZERO {
                            continue;
                        } else if *b == Fp::ONE {
                            (Some(a.clone()), Fp::ONE)
                        } else {
                            (Some(self.fp_mul_const(l, a, *b)?), Fp::ONE)
                        }
                    }
                    (Co::V(a), Co::V(b)) => (Some(self.fp_mul(l, a, b)?), Fp::ONE),
                };
                for k in 0..12 {
                    let c = acc[k];
                    if c == 0 {
                        continue;
                    }
                    match &term {
                        None => {
                            konst[k] += kval
                                * if c > 0 {
                                    Fp::from(c as u64)
                                } else {
                                    -Fp::from((-c) as u64)
                                };
                        }
                        Some(v) => {
                            let list = if c > 0 { &mut pos[k] } else { &mut neg[k] };
                            for _ in 0..c.unsigned_abs() {
                                list.push(v.clone());
                            }
                        }
                    }
                }
            }
        }
        let mut out = Vec::with_capacity(12);
        for k in 0..12 {
            let s = if pos[k].is_empty() && neg[k].is_empty() {
                self.fp_fixed(l, konst[k])?
            } else {
                self.fp.sum(l, &pos[k], &neg[k], konst[k])?
            };
            out.push(self.fp_nf(l, &s)?);
        }
        Ok(afp12_from_coords(out.try_into().unwrap()))
    }

    pub fn flat_fp12_mul(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp12,
        b: &AFp12,
    ) -> Result<AFp12, Error> {
        self.counts.borrow_mut().fp12_mul += 1;
        let x = afp12_coords(a).map(Co::V);
        let y = afp12_coords(b).map(Co::V);
        self.flat_mul(l, &x, &y, false)
    }

    pub fn flat_fp12_square(&self, l: &mut impl Layouter<F>, a: &AFp12) -> Result<AFp12, Error> {
        self.counts.borrow_mut().fp12_sqr += 1;
        let x = afp12_coords(a).map(Co::V);
        self.flat_mul(l, &x, &x, true)
    }

    /// f * ((b0 + b1 v) + v w).
    pub fn flat_fp12_mul_by_line(
        &self,
        l: &mut impl Layouter<F>,
        f: &AFp12,
        b0: &AFp2,
        b1: &AFp2,
    ) -> Result<AFp12, Error> {
        self.counts.borrow_mut().fp12_sparse += 1;
        let x = afp12_coords(f).map(Co::V);
        let z = || Co::K(Fp::ZERO);
        let y = [
            Co::V(b0.c0.clone()),
            Co::V(b0.c1.clone()),
            Co::V(b1.c0.clone()),
            Co::V(b1.c1.clone()),
            z(),
            z(),
            z(),
            z(),
            Co::K(Fp::ONE),
            z(),
            z(),
            z(),
        ];
        self.flat_mul(l, &x, &y, false)
    }

    /// f * s, s in Fp6 (c1 = 0).
    pub fn flat_fp12_mul_by_fp6(
        &self,
        l: &mut impl Layouter<F>,
        f: &AFp12,
        s: &AFp6,
    ) -> Result<AFp12, Error> {
        let x = afp12_coords(f).map(Co::V);
        let z = || Co::K(Fp::ZERO);
        let y = [
            Co::V(s.c0.c0.clone()),
            Co::V(s.c0.c1.clone()),
            Co::V(s.c1.c0.clone()),
            Co::V(s.c1.c1.clone()),
            Co::V(s.c2.c0.clone()),
            Co::V(s.c2.c1.clone()),
            z(),
            z(),
            z(),
            z(),
            z(),
            z(),
        ];
        self.flat_mul(l, &x, &y, false)
    }
}
