//! One verifier code base, two back ends. `Alg` is the operation set the `L_Fr` verifier
//! uses; `Native` evaluates it on `Value<F>` and records failed checks (the plain verdict and
//! the witness generator), `Gadget` emits the same operations as `ZkStdLib` cells. The
//! `MockProver` checks that the two agree on a fixture.

use ff::{Field, PrimeField};
use midnight_circuits::{
    instructions::{
        ArithInstructions, AssertionInstructions, AssignmentInstructions, ControlFlowInstructions,
        ConversionInstructions, DecompositionInstructions, RangeCheckInstructions,
    },
    types::{AssignedBit, AssignedNative},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::ZkStdLib;

use crate::native::{self, F, WIDTH};

pub trait Alg {
    type V: Clone + std::fmt::Debug;
    type B: Clone + std::fmt::Debug;

    /// A private witness value (a proof value).
    fn wit(&mut self, v: Value<F>) -> Result<Self::V, Error>;
    /// A key constant.
    fn cst(&mut self, c: F) -> Result<Self::V, Error>;
    fn add(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Error>;
    fn sub(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Error>;
    fn mul(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Error>;
    /// `sum c_i x_i + k`.
    fn lin(&mut self, terms: &[(F, Self::V)], k: F) -> Result<Self::V, Error>;
    /// `a * c` for a constant `c`.
    fn mul_c(&mut self, a: &Self::V, c: F) -> Result<Self::V, Error> {
        self.lin(&[(c, a.clone())], F::ZERO)
    }
    fn add_c(&mut self, a: &Self::V, c: F) -> Result<Self::V, Error> {
        self.lin(&[(F::ONE, a.clone())], c)
    }
    /// `x * y + z` (one native-chip row on the gadget back end).
    fn mul_add(&mut self, x: &Self::V, y: &Self::V, z: &Self::V) -> Result<Self::V, Error>;
    fn assert_eq(&mut self, a: &Self::V, b: &Self::V, what: &str) -> Result<(), Error>;
    fn assert_eq_c(&mut self, a: &Self::V, c: F, what: &str) -> Result<(), Error>;
    fn permute(&mut self, s: &[Self::V; WIDTH]) -> Result<[Self::V; WIDTH], Error>;
    /// Low `n` bits (little-endian) of the canonical representative (canonical decomposition).
    fn low_bits(&mut self, x: &Self::V, n: usize) -> Result<Vec<Self::B>, Error>;
    fn assert_bit_false(&mut self, b: &Self::B, what: &str) -> Result<(), Error>;
    /// `b ? x : y`.
    fn select(&mut self, b: &Self::B, x: &Self::V, y: &Self::V) -> Result<Self::V, Error>;
    fn bit_v(&mut self, b: &Self::B) -> Result<Self::V, Error>;
    /// Host view of a value (known only with a witness).
    fn val(&self, a: &Self::V) -> Value<F>;
    /// `n` little-endian chunks of `bits` bits with `x = sum c_i 2^(bits i)`; refuses `x` at or
    /// above `2^(bits n)` (E6 call-vector unpack, `bits n < 255`).
    fn chunks(&mut self, x: &Self::V, bits: usize, n: usize) -> Result<Vec<Self::V>, Error>;
    /// Asserts `x < 2^n`.
    fn range(&mut self, x: &Self::V, n: usize, what: &str) -> Result<(), Error>;
    /// A private witness bit.
    fn wbit(&mut self, b: Value<bool>) -> Result<Self::B, Error>;
}

fn big(x: &F) -> num_bigint::BigUint {
    num_bigint::BigUint::from_bytes_le(x.to_repr().as_ref())
}

/// Native back end: values and a list of failed checks.
#[derive(Default, Debug)]
pub struct Native {
    pub failures: Vec<String>,
    pub perms: usize,
}

impl Native {
    fn check(&mut self, ok: Value<bool>, what: &str) {
        ok.map(|ok| {
            if !ok {
                self.failures.push(what.to_string());
            }
        });
    }
}

fn low_u64v(v: &Value<F>) -> Value<u64> {
    v.map(|x| native::low_u64(&x))
}

impl Alg for Native {
    type V = Value<F>;
    type B = Value<bool>;

    fn wit(&mut self, v: Value<F>) -> Result<Value<F>, Error> {
        Ok(v)
    }
    fn cst(&mut self, c: F) -> Result<Value<F>, Error> {
        Ok(Value::known(c))
    }
    fn add(&mut self, a: &Value<F>, b: &Value<F>) -> Result<Value<F>, Error> {
        Ok(*a + *b)
    }
    fn sub(&mut self, a: &Value<F>, b: &Value<F>) -> Result<Value<F>, Error> {
        Ok(*a - *b)
    }
    fn mul(&mut self, a: &Value<F>, b: &Value<F>) -> Result<Value<F>, Error> {
        Ok(*a * *b)
    }
    fn lin(&mut self, terms: &[(F, Value<F>)], k: F) -> Result<Value<F>, Error> {
        Ok(terms.iter().fold(Value::known(k), |acc, (c, x)| acc + x.map(|x| x * c)))
    }
    fn mul_add(&mut self, x: &Value<F>, y: &Value<F>, z: &Value<F>) -> Result<Value<F>, Error> {
        Ok(*x * *y + *z)
    }
    fn assert_eq(&mut self, a: &Value<F>, b: &Value<F>, what: &str) -> Result<(), Error> {
        self.check(a.zip(*b).map(|(a, b)| a == b), what);
        Ok(())
    }
    fn assert_eq_c(&mut self, a: &Value<F>, c: F, what: &str) -> Result<(), Error> {
        self.check(a.map(|a| a == c), what);
        Ok(())
    }
    fn permute(&mut self, s: &[Value<F>; WIDTH]) -> Result<[Value<F>; WIDTH], Error> {
        self.perms += 1;
        let st: Value<[F; WIDTH]> = Value::<Vec<F>>::from_iter(s.iter().copied())
            .map(|v| {
                let mut a: [F; WIDTH] = v.try_into().unwrap();
                native::permute(&mut a);
                a
            });
        Ok(std::array::from_fn(|i| st.map(|a| a[i])))
    }
    fn low_bits(&mut self, x: &Value<F>, n: usize) -> Result<Vec<Value<bool>>, Error> {
        assert!(n < 64);
        let lo = low_u64v(x);
        Ok((0..n).map(|i| lo.map(|v| (v >> i) & 1 == 1)).collect())
    }
    fn assert_bit_false(&mut self, b: &Value<bool>, what: &str) -> Result<(), Error> {
        self.check(b.map(|b| !b), what);
        Ok(())
    }
    fn select(&mut self, b: &Value<bool>, x: &Value<F>, y: &Value<F>) -> Result<Value<F>, Error> {
        Ok(b.zip(x.zip(*y)).map(|(b, (x, y))| if b { x } else { y }))
    }
    fn bit_v(&mut self, b: &Value<bool>) -> Result<Value<F>, Error> {
        Ok(b.map(|b| if b { F::ONE } else { F::ZERO }))
    }
    fn val(&self, a: &Value<F>) -> Value<F> {
        *a
    }
    fn chunks(&mut self, x: &Value<F>, bits: usize, n: usize) -> Result<Vec<Value<F>>, Error> {
        assert!(bits * n < 255);
        let b = x.map(|x| big(&x));
        self.check(b.as_ref().map(|b| b.bits() as usize <= bits * n), "chunks: value too large");
        let mask = (num_bigint::BigUint::from(1u8) << bits) - 1u8;
        Ok((0..n)
            .map(|i| {
                b.as_ref().map(|b| {
                    let c = (b >> (bits * i)) & &mask;
                    F::from_u128(c.to_u64_digits().first().copied().unwrap_or(0) as u128)
                })
            })
            .collect())
    }
    fn range(&mut self, x: &Value<F>, n: usize, what: &str) -> Result<(), Error> {
        self.check(x.map(|x| big(&x).bits() as usize <= n), what);
        Ok(())
    }
    fn wbit(&mut self, b: Value<bool>) -> Result<Value<bool>, Error> {
        Ok(b)
    }
}

/// `ZkStdLib` back end.
pub struct Gadget<'a, L: Layouter<F>> {
    pub std: &'a ZkStdLib,
    pub l: &'a mut L,
    pub perms: usize,
}

impl<'a, L: Layouter<F>> Gadget<'a, L> {
    pub fn new(std: &'a ZkStdLib, l: &'a mut L) -> Self {
        Self { std, l, perms: 0 }
    }
}

impl<L: Layouter<F>> Alg for Gadget<'_, L> {
    type V = AssignedNative<F>;
    type B = AssignedBit<F>;

    fn wit(&mut self, v: Value<F>) -> Result<Self::V, Error> {
        self.std.assign(&mut *self.l, v)
    }
    fn cst(&mut self, c: F) -> Result<Self::V, Error> {
        self.std.assign_fixed(&mut *self.l, c)
    }
    fn add(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Error> {
        self.std.add(&mut *self.l, a, b)
    }
    fn sub(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Error> {
        self.std.sub(&mut *self.l, a, b)
    }
    fn mul(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Error> {
        self.std.mul(&mut *self.l, a, b, None)
    }
    fn lin(&mut self, terms: &[(F, Self::V)], k: F) -> Result<Self::V, Error> {
        self.std.linear_combination(&mut *self.l, terms, k)
    }
    fn mul_add(&mut self, x: &Self::V, y: &Self::V, z: &Self::V) -> Result<Self::V, Error> {
        // 0*x + 0*y + 1*z + 0 + 1*x*y
        self.std.native_gadget().add_and_mul(
            &mut *self.l,
            (F::ZERO, x),
            (F::ZERO, y),
            (F::ONE, z),
            F::ZERO,
            F::ONE,
        )
    }
    fn assert_eq(&mut self, a: &Self::V, b: &Self::V, _: &str) -> Result<(), Error> {
        self.std.assert_equal(&mut *self.l, a, b)
    }
    fn assert_eq_c(&mut self, a: &Self::V, c: F, _: &str) -> Result<(), Error> {
        self.std.assert_equal_to_fixed(&mut *self.l, a, c)
    }
    fn permute(&mut self, s: &[Self::V; WIDTH]) -> Result<[Self::V; WIDTH], Error> {
        self.perms += 1;
        self.std.poseidon_chip().permutation(&mut *self.l, s)
    }
    fn low_bits(&mut self, x: &Self::V, n: usize) -> Result<Vec<Self::B>, Error> {
        let all = self.std.assigned_to_le_bits(&mut *self.l, x, None, true)?;
        Ok(all[..n].to_vec())
    }
    fn assert_bit_false(&mut self, b: &Self::B, _: &str) -> Result<(), Error> {
        self.std.assert_equal_to_fixed(&mut *self.l, b, false)
    }
    fn select(&mut self, b: &Self::B, x: &Self::V, y: &Self::V) -> Result<Self::V, Error> {
        self.std.select(&mut *self.l, b, x, y)
    }
    fn bit_v(&mut self, b: &Self::B) -> Result<Self::V, Error> {
        ConversionInstructions::<F, AssignedBit<F>, AssignedNative<F>>::convert(self.std, &mut *self.l, b)
    }
    fn val(&self, a: &Self::V) -> Value<F> {
        a.value().copied()
    }
    fn chunks(&mut self, x: &Self::V, bits: usize, n: usize) -> Result<Vec<Self::V>, Error> {
        assert!(bits * n < 255);
        self.std.assigned_to_le_chunks(&mut *self.l, x, bits, Some(n))
    }
    fn range(&mut self, x: &Self::V, n: usize, _: &str) -> Result<(), Error> {
        self.std.assert_lower_than_fixed(&mut *self.l, x, &(num_bigint::BigUint::from(1u8) << n))
    }
    fn wbit(&mut self, b: Value<bool>) -> Result<Self::B, Error> {
        AssignmentInstructions::<F, AssignedBit<F>>::assign(self.std, &mut *self.l, b)
    }
}
