//! Selectors packed into fixed columns (experiment E8).
//!
//! The keygen of `midnight-proofs` 0.8.3 does not compress selectors: every
//! selector is its own fixed column, and every filled fixed column costs the node
//! about 6,460 rows (`experiments-3.md` 3.3). A gate that sits on rows disjoint
//! from other gates can share one fixed column with them: the column holds a code
//! `c` in `1..=n` on the gate's rows and 0 elsewhere, and the gate is multiplied by
//! the Lagrange polynomial that is 1 at `c` and 0 at every other value of
//! `0..=n`. That polynomial has degree `n`, so a gate of body degree `d` fits a
//! column of `n` codes when `n + d` is at most the circuit degree (6).
//!
//! The codes are fixed by the circuit, never by the prover, so a packed selector
//! is as sound as a plain one: on a row with code `c` exactly the gates of code `c`
//! are on.
use ff::Field;
use midnight_proofs::{
    circuit::{Region, Value},
    plonk::{Column, ConstraintSystem, Error, Expression, Fixed, Selector, VirtualCells},
    poly::Rotation,
};

use crate::F;

/// The degree bound of the leaf (the Poseidon round gates are degree 6).
pub const MAX_DEG: usize = 6;

/// A gate selector: a plain selector, or code `code` of a column of `n` codes.
#[derive(Clone, Copy, Debug)]
pub enum VSel {
    Simple(Selector),
    Code {
        col: Column<Fixed>,
        code: u64,
        n: u64,
    },
}

impl VSel {
    /// The selector expression: 1 on the gate's rows, 0 on every other row.
    pub fn expr(&self, m: &mut VirtualCells<'_, F>) -> Expression<F> {
        match *self {
            VSel::Simple(s) => m.query_selector(s),
            VSel::Code { col, code, n } => lagrange(m.query_fixed(col, Rotation::cur()), code, n),
        }
    }

    /// Degree of [`Self::expr`].
    pub fn degree(&self) -> usize {
        match *self {
            VSel::Simple(_) => 1,
            VSel::Code { n, .. } => n as usize,
        }
    }

    pub fn enable(&self, region: &mut Region<'_, F>, row: usize) -> Result<(), Error> {
        match *self {
            VSel::Simple(s) => s.enable(region, row),
            VSel::Code { col, code, .. } => region
                .assign_fixed(|| "code", col, row, || Value::known(F::from(code)))
                .map(|_| ()),
        }
    }
}

/// `L_c(q)`: 1 at `q = c`, 0 at every other value of `0..=n`.
pub fn lagrange(q: Expression<F>, c: u64, n: u64) -> Expression<F> {
    let mut e = Expression::Constant(F::ONE);
    let mut den = F::ONE;
    for j in 0..=n {
        if j == c {
            continue;
        }
        e = e * (q.clone() - Expression::Constant(F::from(j)));
        den *= F::from(c) - F::from(j);
    }
    e * Expression::Constant(den.invert().unwrap())
}

/// A code column with `n` codes: the selectors of codes `1..=n`.
pub fn code_column(meta: &mut ConstraintSystem<F>, n: u64) -> Vec<VSel> {
    let col = meta.fixed_column();
    (1..=n).map(|code| VSel::Code { col, code, n }).collect()
}

/// Pack gates of body degrees `degs` into as few code columns as the degree
/// bound allows (first fit, largest degree first). Returns one selector per gate.
pub fn pack(meta: &mut ConstraintSystem<F>, degs: &[usize]) -> Vec<VSel> {
    let mut order: Vec<usize> = (0..degs.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(degs[i]));
    // Columns as (members, max degree).
    let mut cols: Vec<(Vec<usize>, usize)> = Vec::new();
    for i in order {
        let d = degs[i];
        assert!(d < MAX_DEG, "gate body of degree {d} cannot be packed");
        match cols
            .iter_mut()
            .find(|(m, md)| m.len() + 1 + (*md).max(d) <= MAX_DEG)
        {
            Some((m, md)) => {
                m.push(i);
                *md = (*md).max(d);
            }
            None => cols.push((vec![i], d)),
        }
    }
    let mut out = vec![None; degs.len()];
    for (members, _) in cols {
        let col = meta.fixed_column();
        let n = members.len() as u64;
        for (k, &i) in members.iter().enumerate() {
            out[i] = Some(VSel::Code {
                col,
                code: k as u64 + 1,
                n,
            });
        }
    }
    out.into_iter().map(|v| v.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(e: &Expression<F>, q: F) -> F {
        e.evaluate(
            &|c| c,
            &|_| unreachable!(),
            &|_| q,
            &|_| unreachable!(),
            &|_| unreachable!(),
            &|_| unreachable!(),
            &|a| -a,
            &|a, b| a + b,
            &|a, b| a * b,
            &|a, s| a * s,
        )
    }

    #[test]
    fn lagrange_is_an_indicator() {
        let mut cs = ConstraintSystem::<F>::default();
        let col = cs.fixed_column();
        for n in 1..=5u64 {
            for c in 1..=n {
                let mut got = None;
                cs.create_gate("probe", |m| {
                    let e = lagrange(m.query_fixed(col, Rotation::cur()), c, n);
                    got = Some(e.clone());
                    crate::gate(vec![e])
                });
                let e = got.unwrap();
                assert_eq!(e.degree(), n as usize);
                for v in 0..=n {
                    let want = if v == c { F::ONE } else { F::ZERO };
                    assert_eq!(eval(&e, F::from(v)), want, "n {n} c {c} v {v}");
                }
            }
        }
    }

    #[test]
    fn pack_respects_the_degree_bound() {
        let mut cs = ConstraintSystem::<F>::default();
        let degs = [3, 3, 3, 3, 3, 3, 1, 2];
        let v = pack(&mut cs, &degs);
        for (s, d) in v.iter().zip(degs) {
            assert!(s.degree() + d <= MAX_DEG);
        }
        // Six degree-3 gates take two columns of 3; the rest fit in a third.
        assert_eq!(cs.num_fixed_columns(), 3);
    }
}
