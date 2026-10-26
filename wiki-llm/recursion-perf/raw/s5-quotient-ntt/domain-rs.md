# midnight-zk proofs/src/poly/domain.rs (main)
> Source: https://raw.githubusercontent.com/midnightntwrk/midnight-zk/main/proofs/src/poly/domain.rs
> Collected: 2026-10-26
> Published: Unknown

//! Contains utilities for performing polynomial arithmetic over an evaluation
//! domain that is of a suitable size for the application.
use std::marker::PhantomData;
use ff::WithSmallOrderMulGroup;
use group::ff::{BatchInvert, Field};
use midnight\_curves::fft::{best\_fft\_with\_twiddles, compute\_twiddles, fft\_coeff\_to\_extended};
use super::{Coeff, ExtendedLagrangeCoeff, LagrangeCoeff, Polynomial, Rotation};
use crate::utils::{arithmetic::parallelize, rational::Rational};
/// Precomputed twiddle factors for the FFT.
///
/// Twiddle factors are the powers of the root of unity used in the
/// butterfly operations of the Cooley-Tukey FFT:
/// `[1, ω, ω², ..., ω^(n/2 - 1)]`
/// where ω is the n-th root of unity in the field.
///
/// Four sets are stored: forward and inverse for both the base domain (size
/// 2^k) and the extended domain (size 2^extended\_k).
#[derive(Clone, Debug)]
struct CachedTwiddles {
omega: Vec,
omega\_inv: Vec,
extended\_omega: Vec,
extended\_omega\_inv: Vec,
}
/// This structure contains precomputed constants and other details needed for
/// performing operations on an evaluation domain of size $2^k$ and an extended
/// domain of size $2^{k} \* j$ with $j \neq 0$.
#[derive(Clone, Debug)]
pub struct EvaluationDomain {
pub(crate) n: u64,
k: u32,
extended\_k: u32,
omega: F,
omega\_inv: F,
extended\_omega: F,
pub(crate) g\_coset: F,
g\_coset\_inv: F,
quotient\_poly\_degree: u64,
ifft\_divisor: F,
extended\_ifft\_divisor: F,
t\_evaluations: Vec,
barycentric\_weight: F,
twiddles: CachedTwiddles,
}
impl> EvaluationDomain {
/// This constructs a new evaluation domain object based on the provided
/// values $j, k$.
pub fn new(j: u32, k: u32) -> Self {
// quotient\_poly\_degree \* params.n - 1 is the degree of the quotient polynomial
let quotient\_poly\_degree = (j - 1) as u64;
// n = 2^k
let n = 1u64 << k;
// We need to work within an extended domain, not params.k but params.k + i
// for some integer i such that 2^(params.k + i) is sufficiently large to
// describe the quotient polynomial.
let mut extended\_k = k;
while (1 << extended\_k) < (n \* quotient\_poly\_degree) {
extended\_k += 1;
}
// ensure extended\_k <= S
assert!(extended\_k <= F::S);
let mut extended\_omega = F::ROOT\_OF\_UNITY;
// Get extended\_omega, the 2^{extended\_k}'th root of unity
// The loop computes extended\_omega = omega^{2 ^ (S - extended\_k)}
// Notice that extended\_omega ^ {2 ^ extended\_k} = omega ^ {2^S} = 1.
for \_ in extended\_k..F::S {
extended\_omega = extended\_omega.square();
}
let extended\_omega = extended\_omega;
let mut extended\_omega\_inv = extended\_omega; // Inversion computed later
// Get omega, the 2^{k}'th root of unity (i.e. n'th root of unity)
// The loop computes omega = extended\_omega ^ {2 ^ (extended\_k - k)}
// = (omega^{2 ^ (S - extended\_k)}) ^ {2 ^ (extended\_k - k)}
// = omega ^ {2 ^ (S - k)}.
// Notice that omega ^ {2^k} = omega ^ {2^S} = 1.
let mut omega = extended\_omega;
for \_ in k..extended\_k {
omega = omega.square();
}
let omega = omega;
let mut omega\_inv = omega; // Inversion computed later
// The coset evaluation domain is:
// zeta {1, extended\_omega, extended\_omega^2, ...,
// extended\_omega^{(2^extended\_k) - 1}}
let g\_coset = F::ZETA;
let g\_coset\_inv = g\_coset.square();
let mut t\_evaluations = Vec::with\_capacity(1 << (extended\_k - k));
{
// Compute the evaluations of t(X) = X^n - 1 in the coset evaluation domain.
// We don't have to compute all of them, because it will repeat.
let orig = g\_coset.pow\_vartime([n, 0, 0, 0]);
let step = extended\_omega.pow\_vartime([n, 0, 0, 0]);
let mut cur = orig;
loop {
t\_evaluations.push(cur);
cur \*= &step;
if cur == orig {
break;
}
}
assert\_eq!(t\_evaluations.len(), 1 << (extended\_k - k));
// Subtract 1 from each to give us t\_evaluations[i] = t(zeta \* extended\_omega^i)
for coeff in &mut t\_evaluations {
\*coeff -= &F::ONE;
}
// Invert, because we're dividing by this polynomial.
// We invert in a batch, below.
}
let mut ifft\_divisor = F::from(1 << k); // Inversion computed later
let mut extended\_ifft\_divisor = F::from(1 << extended\_k); // Inversion computed later
// The barycentric weight of 1 over the evaluation domain
// 1 / \prod\_{i != 0} (1 - omega^i)
let mut barycentric\_weight = F::from(n); // Inversion computed later
// Compute batch inversion
t\_evaluations
.iter\_mut()
.chain(Some(&mut ifft\_divisor))
.chain(Some(&mut extended\_ifft\_divisor))
.chain(Some(&mut barycentric\_weight))
.chain(Some(&mut extended\_omega\_inv))
.chain(Some(&mut omega\_inv))
.batch\_invert();
// Precompute twiddle factors for all FFT configurations.
let twiddles = CachedTwiddles {
omega: compute\_twiddles(&omega, k),
omega\_inv: compute\_twiddles(&omega\_inv, k),
extended\_omega: compute\_twiddles(&extended\_omega, extended\_k),
extended\_omega\_inv: compute\_twiddles(&extended\_omega\_inv, extended\_k),
};
EvaluationDomain {
n,
k,
extended\_k,
omega,
omega\_inv,
extended\_omega,
g\_coset,
g\_coset\_inv,
quotient\_poly\_degree,
ifft\_divisor,
extended\_ifft\_divisor,
t\_evaluations,
barycentric\_weight,
twiddles,
}
}
/// Obtains a polynomial in Lagrange form when given a vector of Lagrange
/// coefficients of size `n`; panics if the provided vector is the wrong
/// length.
pub fn lagrange\_from\_vec(&self, values: Vec) -> Polynomial {
assert\_eq!(values.len(), self.n as usize);
Polynomial {
values,
\_marker: PhantomData,
}
}
/// Obtains a polynomial in coefficient form when given a vector of
/// coefficients of size `n`; panics if the provided vector is the wrong
/// length.
pub fn coeff\_from\_vec(&self, values: Vec) -> Polynomial {
assert\_eq!(values.len(), self.n as usize);
Polynomial {
values,
\_marker: PhantomData,
}
}
/// Returns an empty (zero) polynomial in the coefficient representation
pub fn empty\_coeff(&self) -> Polynomial {
Polynomial {
values: vec![F::ZERO; self.n as usize],
\_marker: PhantomData,
}
}
/// Returns an empty (zero) polynomial in the Lagrange coefficient
/// representation
pub fn empty\_lagrange(&self) -> Polynomial {
Polynomial {
values: vec![F::ZERO; self.n as usize],
\_marker: PhantomData,
}
}
/// Returns an empty (zero) polynomial in the Lagrange coefficient
/// representation, with deferred inversions.
pub fn empty\_lagrange\_rational(&self) -> Polynomial, LagrangeCoeff> {
Polynomial {
values: vec![F::ZERO.into(); self.n as usize],
\_marker: PhantomData,
}
}
/// Returns a constant polynomial in the Lagrange coefficient representation
pub fn constant\_lagrange(&self, scalar: F) -> Polynomial {
Polynomial {
values: vec![scalar; self.n as usize],
\_marker: PhantomData,
}
}
/// Returns an empty (zero) polynomial in the extended Lagrange coefficient
/// representation
pub fn empty\_extended(&self) -> Polynomial {
Polynomial {
values: vec![F::ZERO; self.extended\_len()],
\_marker: PhantomData,
}
}
/// Returns a constant polynomial in the extended Lagrange coefficient
/// representation
pub fn constant\_extended(&self, scalar: F) -> Polynomial {
Polynomial {
values: vec![scalar; self.extended\_len()],
\_marker: PhantomData,
}
}
/// This takes us from an n-length vector into the coefficient form.
///
/// This function will panic if the provided vector does not have the
/// correct length.
pub fn lagrange\_to\_coeff(&self, mut a: Polynomial) -> Polynomial {
assert\_eq!(a.values.len(), 1 << self.k);
// Perform inverse FFT using cached twiddle factors.
Self::ifft(
&mut a.values,
&self.twiddles.omega\_inv,
self.k,
self.ifft\_divisor,
);
Polynomial {
values: a.values,
\_marker: PhantomData,
}
}
/// This takes us from an n-length coefficient vector into the
/// lagrange evaluation domain.
pub fn coeff\_to\_lagrange(&self, mut a: Polynomial) -> Polynomial {
assert\_eq!(a.values.len(), 1 << self.k);
best\_fft\_with\_twiddles(&mut a.values, &self.twiddles.omega, self.k);
Polynomial {
values: a.values,
\_marker: PhantomData,
}
}
/// This takes us from an n-length coefficient vector into a coset of the
/// extended evaluation domain.
///
/// Uses a pruned DIF FFT that exploits the zero-padded structure
/// (`n\_real = 2^k` coefficients padded to `2^extended\_k`) to skip work on
/// all-zero subtrees. Falls back to standard DIT for non-Fq fields.
pub fn coeff\_to\_extended(
&self,
mut a: Polynomial,
) -> Polynomial {
let n\_real = a.values.len();
assert\_eq!(n\_real, 1 << self.k);
self.distribute\_powers\_zeta(&mut a.values, true);
a.values.resize(self.extended\_len(), F::ZERO);
fft\_coeff\_to\_extended(
&mut a.values,
&self.twiddles.extended\_omega,
self.extended\_k,
n\_real,
);
Polynomial {
values: a.values,
\_marker: PhantomData,
}
}
/// This takes us from the extended evaluation domain and gets us the
/// quotient polynomial coefficients.
///
/// This function will panic if the provided vector does not have the
/// correct length.
pub fn extended\_to\_coeff(&self, mut a: Polynomial) -> Vec {
assert\_eq!(a.values.len(), self.extended\_len());
// Inverse FFT using cached twiddle factors.
Self::ifft(
&mut a.values,
&self.twiddles.extended\_omega\_inv,
self.extended\_k,
self.extended\_ifft\_divisor,
);
// Distribute powers to move from coset; opposite from the
// transformation we performed earlier.
self.distribute\_powers\_zeta(&mut a.values, false);
a.values
}
/// This takes us from the extended evaluation domain and gets us to the
/// small evaluation domain (of size `n`).
///
/// This function will panic if the provided vector does not have the
/// correct length.
pub fn extended\_to\_lagrange(
&self,
mut a: Polynomial,
) -> Polynomial {
assert\_eq!(a.values.len(), self.extended\_len());
Self::ifft(
&mut a.values,
&self.twiddles.extended\_omega\_inv,
self.extended\_k,
self.extended\_ifft\_divisor,
);
a.values.truncate(self.n as usize);
self.distribute\_powers\_zeta(&mut a.values, false);
best\_fft\_with\_twiddles(&mut a.values, &self.twiddles.omega, self.k);
Polynomial {
values: a.values,
\_marker: PhantomData,
}
}
/// This divides the polynomial (in the extended domain) by the vanishing
/// polynomial of the $2^k$ size domain.
pub fn divide\_by\_vanishing\_poly(
&self,
mut a: Polynomial,
) -> Polynomial {
assert\_eq!(a.values.len(), self.extended\_len());
// Divide to obtain the quotient polynomial in the coset evaluation
// domain.
parallelize(&mut a.values, |h, mut index| {
for h in h {
\*h \*= &self.t\_evaluations[index & (self.t\_evaluations.len() - 1)];
index += 1;
}
});
Polynomial {
values: a.values,
\_marker: PhantomData,
}
}
/// Given a slice of group elements `[a\_0, a\_1, a\_2, ...]`, this returns
/// `[a\_0, [zeta]a\_1, [zeta^2]a\_2, a\_3, [zeta]a\_4, [zeta^2]a\_5, a\_6, ...]`,
/// where zeta is a cube root of unity in the multiplicative subgroup with
/// order (p - 1), i.e. zeta^3 = 1.
///
/// `into\_coset` should be set to `true` when moving into the coset,
/// and `false` when moving out. This toggles the choice of `zeta`.
fn ifft(a: &mut [F], twiddles: &[F], log\_n: u32, divisor: F) {
best\_fft\_with\_twiddles(a, twiddles, log\_n);
parallelize(a, |a, \_| {
for a in a {
\*a \*= &divisor;
}
});
}
fn distribute\_powers\_zeta(&self, a: &mut [F], into\_coset: bool) {
let coset\_powers = if into\_coset {
[self.g\_coset, self.g\_coset\_inv]
} else {
[self.g\_coset\_inv, self.g\_coset]
};
parallelize(a, |a, mut index| {
for a in a {
// Distribute powers to move into/from coset
let i = index % (coset\_powers.len() + 1);
if i != 0 {
\*a \*= &coset\_powers[i - 1];
}
index += 1;
}
});
}
/// Get the size of the domain
pub fn k(&self) -> u32 {
self.k
}
/// Get the size of the extended domain
pub fn extended\_k(&self) -> u32 {
self.extended\_k
}
/// Get the size of the extended domain
pub fn extended\_len(&self) -> usize {
1 << self.extended\_k
}
/// Get $\omega$, the generator of the $2^k$ order multiplicative subgroup.
pub fn get\_omega(&self) -> F {
self.omega
}
/// Get $\omega^{-1}$, the inverse of the generator of the $2^k$ order
/// multiplicative subgroup.
pub fn get\_omega\_inv(&self) -> F {
self.omega\_inv
}
/// Get the generator of the extended domain's multiplicative subgroup.
pub fn get\_extended\_omega(&self) -> F {
self.extended\_omega
}
/// Multiplies a value by some power of $\omega$, essentially rotating over
/// the domain.
pub fn rotate\_omega(&self, value: F, rotation: Rotation) -> F {
let mut point = value;
if rotation.0 >= 0 {
point \*= &self.get\_omega().pow\_vartime([rotation.0 as u64]);
} else {
point \*= &self.get\_omega\_inv().pow\_vartime([(rotation.0 as i64).unsigned\_abs()]);
}
point
}
/// Computes evaluations (at the point `x`, where `xn = x^n`) of Lagrange
/// polynomials `l\_i(X)` defined such that `l\_i(omega^i) = 1` and
/// `l\_i(omega^j) = 0` for all `j != i` at each provided rotation `i`.
///
/// # Implementation
///
/// The polynomial
/// $$\prod\_{j=0,j \neq i}^{n - 1} (X - \omega^j)$$
/// has a root at all points in the domain except $\omega^i$, where it
/// evaluates to $$\prod\_{j=0,j \neq i}^{n - 1} (\omega^i -
/// \omega^j)$$ and so we divide that polynomial by this value to obtain
/// $l\_i(X)$. Since $$\prod\_{j=0,j \neq i}^{n - 1} (X - \omega^j)
/// = \frac{X^n - 1}{X - \omega^i}$$
/// then $l\_i(x)$ for some $x$ is evaluated as
/// $$\left(\frac{x^n - 1}{x - \omega^i}\right)
/// \cdot \left(\frac{1}{\prod\_{j=0,j \neq i}^{n - 1} (\omega^i -
/// \omega^j)}\right).$$ We refer to
/// $$1 \over \prod\_{j=0,j \neq i}^{n - 1} (\omega^i - \omega^j)$$
/// as the barycentric weight of $\omega^i$.
///
/// We know that for $i = 0$
/// $$\frac{1}{\prod\_{j=0,j \neq i}^{n - 1} (\omega^i - \omega^j)} =
/// \frac{1}{n}.$$
///
/// If we multiply $(1 / n)$ by $\omega^i$ then we obtain
/// $$\frac{1}{\prod\_{j=0,j \neq 0}^{n - 1} (\omega^i - \omega^j)}
/// = \frac{1}{\prod\_{j=0,j \neq i}^{n - 1} (\omega^i - \omega^j)}$$
/// which is the barycentric weight of $\omega^i$.
pub fn l\_i\_range + Clone>(
&self,
x: F,
xn: F,
rotations: I,
) -> Vec {
let mut results;
{
let rotations = rotations.clone().into\_iter();
results = Vec::with\_capacity(rotations.size\_hint().1.unwrap\_or(0));
for rotation in rotations {
let rotation = Rotation(rotation);
let result = x - self.rotate\_omega(F::ONE, rotation);
results.push(result);
}
results.iter\_mut().batch\_invert();
}
let common = (xn - F::ONE) \* self.barycentric\_weight;
for (rotation, result) in rotations.into\_iter().zip(results.iter\_mut()) {
let rotation = Rotation(rotation);
\*result = self.rotate\_omega(\*result \* common, rotation);
}
results
}
/// Gets the quotient polynomial's degree (as a multiple of n)
pub fn get\_quotient\_poly\_degree(&self) -> usize {
self.quotient\_poly\_degree as usize
}
/// Obtain a pinned version of this evaluation domain; a structure with the
/// minimal parameters needed to determine the rest of the evaluation
/// domain.
pub fn pinned(&self) -> PinnedEvaluationDomain<'\_, F> {
PinnedEvaluationDomain {
k: &self.k,
extended\_k: &self.extended\_k,
omega: &self.omega,
}
}
}
/// Represents the minimal parameters that determine an `EvaluationDomain`.
#[allow(dead\_code)]
#[derive(Debug)]
pub struct PinnedEvaluationDomain<'a, F: Field> {
k: &'a u32,
extended\_k: &'a u32,
omega: &'a F,
}
#[test]
fn test\_rotate() {
use midnight\_curves::Fq as Scalar;
use rand\_core::OsRng;
use crate::utils::arithmetic::eval\_polynomial;
let domain = EvaluationDomain::::new(1, 3);
let rng = OsRng;
let mut poly = domain.empty\_lagrange();
assert\_eq!(poly.len(), 8);
for value in poly.iter\_mut() {
\*value = Scalar::random(rng);
}
let poly\_rotated\_cur = poly.rotate(Rotation::cur());
let poly\_rotated\_next = poly.rotate(Rotation::next());
let poly\_rotated\_prev = poly.rotate(Rotation::prev());
let poly = domain.lagrange\_to\_coeff(poly);
let poly\_rotated\_cur = domain.lagrange\_to\_coeff(poly\_rotated\_cur);
let poly\_rotated\_next = domain.lagrange\_to\_coeff(poly\_rotated\_next);
let poly\_rotated\_prev = domain.lagrange\_to\_coeff(poly\_rotated\_prev);
let x = Scalar::random(rng);
assert\_eq!(
eval\_polynomial(&poly[..], x),
eval\_polynomial(&poly\_rotated\_cur[..], x)
);
assert\_eq!(
eval\_polynomial(&poly[..], x \* domain.omega),
eval\_polynomial(&poly\_rotated\_next[..], x)
);
assert\_eq!(
eval\_polynomial(&poly[..], x \* domain.omega\_inv),
eval\_polynomial(&poly\_rotated\_prev[..], x)
);
}
#[test]
fn test\_l\_i() {
use midnight\_curves::Fq;
use rand\_core::OsRng;
use crate::utils::arithmetic::{eval\_polynomial, lagrange\_interpolate};
let domain = EvaluationDomain::::new(1, 3);
let mut l = vec![];
let mut points = vec![];
for i in 0..8 {
points.push(domain.omega.pow([i]));
}
for i in 0..8 {
let mut l\_i = vec![Fq::ZERO; 8];
l\_i[i] = Fq::ONE;
let l\_i = lagrange\_interpolate(&points[..], &l\_i[..]);
l.push(l\_i);
}
let x = Fq::random(OsRng);
let xn = x.pow([8]);
let evaluations = domain.l\_i\_range(x, xn, -7..=7);
for i in 0..8 {
assert\_eq!(eval\_polynomial(&l[i][..], x), evaluations[7 + i]);
assert\_eq!(eval\_polynomial(&l[(8 - i) % 8][..], x), evaluations[7 - i]);
}
}