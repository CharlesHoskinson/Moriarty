// SPDX-License-Identifier: Apache-2.0
//! BLS12-381 G1 (y^2 = x^3 + 4) in affine (blst_p1_affine layout) and XYZZ coordinates.
//! x = X / ZZ, y = Y / ZZZ, ZZ^3 = ZZZ^2; ZZ = 0 is the point at infinity.
//! Formulas: EFD add-2008-s, madd-2008-s, dbl-2008-s-1, mdbl-2008-s-1 (a = 0).
use crate::fp::{Fp, ONE, ZERO};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Affine {
    pub x: Fp,
    pub y: Fp,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Xyzz {
    pub x: Fp,
    pub y: Fp,
    pub zz: Fp,
    pub zzz: Fp,
}

impl Affine {
    /// blst encodes the affine infinity as all-zero limbs.
    #[inline(always)]
    pub fn is_inf(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
    #[inline(always)]
    pub fn neg(&self) -> Affine {
        Affine {
            x: self.x,
            y: self.y.neg(),
        }
    }
}

pub const INF: Xyzz = Xyzz {
    x: ONE,
    y: ONE,
    zz: ZERO,
    zzz: ZERO,
};

impl Xyzz {
    /// Affine form with its own inversion (slow path, rare).
    pub fn to_affine(&self) -> Affine {
        if self.is_inf() {
            return Affine { x: ZERO, y: ZERO };
        }
        Affine {
            x: self.x.mul(&self.zz.inv()),
            y: self.y.mul(&self.zzz.inv()),
        }
    }

    #[inline(always)]
    pub fn is_inf(&self) -> bool {
        self.zz.is_zero()
    }

    #[inline(always)]
    pub fn from_affine(a: &Affine) -> Xyzz {
        if a.is_inf() {
            INF
        } else {
            Xyzz {
                x: a.x,
                y: a.y,
                zz: ONE,
                zzz: ONE,
            }
        }
    }

    #[inline(always)]
    pub fn neg(&self) -> Xyzz {
        Xyzz {
            x: self.x,
            y: self.y.neg(),
            zz: self.zz,
            zzz: self.zzz,
        }
    }

    pub fn dbl(&self) -> Xyzz {
        if self.is_inf() {
            return *self;
        }
        let u = self.y.dbl();
        let v = u.sqr();
        let w = u.mul(&v);
        let s = self.x.mul(&v);
        let x2 = self.x.sqr();
        let m = x2.dbl().add(&x2);
        let x3 = m.sqr().sub(&s.dbl());
        let y3 = m.mul(&s.sub(&x3)).sub(&w.mul(&self.y));
        Xyzz {
            x: x3,
            y: y3,
            zz: v.mul(&self.zz),
            zzz: w.mul(&self.zzz),
        }
    }

    fn mdbl(a: &Affine) -> Xyzz {
        let u = a.y.dbl();
        let v = u.sqr();
        let w = u.mul(&v);
        let s = a.x.mul(&v);
        let x2 = a.x.sqr();
        let m = x2.dbl().add(&x2);
        let x3 = m.sqr().sub(&s.dbl());
        let y3 = m.mul(&s.sub(&x3)).sub(&w.mul(&a.y));
        Xyzz {
            x: x3,
            y: y3,
            zz: v,
            zzz: w,
        }
    }

    /// self + a (mixed addition).
    pub fn add_affine(&self, a: &Affine) -> Xyzz {
        if a.is_inf() {
            return *self;
        }
        if self.is_inf() {
            return Xyzz::from_affine(a);
        }
        let u2 = a.x.mul(&self.zz);
        let s2 = a.y.mul(&self.zzz);
        let p = u2.sub(&self.x);
        let r = s2.sub(&self.y);
        if p.is_zero() {
            return if r.is_zero() { Xyzz::mdbl(a) } else { INF };
        }
        let pp = p.sqr();
        let ppp = p.mul(&pp);
        let q = self.x.mul(&pp);
        let x3 = r.sqr().sub(&ppp).sub(&q.dbl());
        let y3 = r.mul(&q.sub(&x3)).sub(&self.y.mul(&ppp));
        Xyzz {
            x: x3,
            y: y3,
            zz: self.zz.mul(&pp),
            zzz: self.zzz.mul(&ppp),
        }
    }

    /// self + o.
    pub fn add(&self, o: &Xyzz) -> Xyzz {
        if o.is_inf() {
            return *self;
        }
        if self.is_inf() {
            return *o;
        }
        let u1 = self.x.mul(&o.zz);
        let u2 = o.x.mul(&self.zz);
        let s1 = self.y.mul(&o.zzz);
        let s2 = o.y.mul(&self.zzz);
        let p = u2.sub(&u1);
        let r = s2.sub(&s1);
        if p.is_zero() {
            return if r.is_zero() { self.dbl() } else { INF };
        }
        let pp = p.sqr();
        let ppp = p.mul(&pp);
        let q = u1.mul(&pp);
        let x3 = r.sqr().sub(&ppp).sub(&q.dbl());
        let y3 = r.mul(&q.sub(&x3)).sub(&s1.mul(&ppp));
        Xyzz {
            x: x3,
            y: y3,
            zz: self.zz.mul(&o.zz).mul(&pp),
            zzz: self.zzz.mul(&o.zzz).mul(&ppp),
        }
    }

    /// k * self for a small k (double and add, most significant bit first).
    pub fn mul_small(&self, k: u32) -> Xyzz {
        let mut r = INF;
        let mut bit = 32;
        while bit > 0 {
            bit -= 1;
            r = r.dbl();
            if (k >> bit) & 1 == 1 {
                r = r.add(self);
            }
        }
        r
    }
}
