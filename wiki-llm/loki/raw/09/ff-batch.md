//! Batched field inversion APIs, using [Montgomery's trick].
//!
//! [Montgomery's trick]: https://zcash.github.io/halo2/background/fields.html#montgomerys-trick
use subtle::ConstantTimeEq;
use crate::Field;
/// Extension trait for iterators over mutable field elements which allows those field
/// elements to be inverted in a batch.
///
/// `I: IntoIterator` implements this trait when
/// the `alloc` feature flag is enabled.
///
/// For non-allocating contexts, see the [`BatchInverter`] struct.
#[cfg(feature = "alloc")]
#[cfg\_attr(docsrs, doc(cfg(feature = "alloc")))]
pub trait BatchInvert {
/// Consumes this iterator and inverts each field element (when nonzero). Zero-valued
/// elements are left as zero.
///
/// Returns the inverse of the product of all nonzero field elements.
fn batch\_invert(self) -> F;
}
#[cfg(feature = "alloc")]
#[cfg\_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, F, I> BatchInvert for I
where
F: Field + ConstantTimeEq,
I: IntoIterator,
{
fn batch\_invert(self) -> F {
let mut acc = F::ONE;
let iter = self.into\_iter();
let mut tmp = alloc::vec::Vec::with\_capacity(iter.size\_hint().0);
for p in iter {
let q = \*p;
tmp.push((acc, p));
acc = F::conditional\_select(&(acc \* q), &acc, q.is\_zero());
}
acc = acc.invert().unwrap();
let allinv = acc;
for (tmp, p) in tmp.into\_iter().rev() {
let skip = p.is\_zero();
let tmp = tmp \* acc;
acc = F::conditional\_select(&(acc \* \*p), &acc, skip);
\*p = F::conditional\_select(&tmp, p, skip);
}
allinv
}
}
/// A non-allocating batch inverter.
pub struct BatchInverter {}
impl BatchInverter {
/// Inverts each field element in `elements` (when nonzero). Zero-valued elements are
/// left as zero.
///
/// - `scratch\_space` is a slice of field elements that can be freely overwritten.
///
/// Returns the inverse of the product of all nonzero field elements.
///
/// # Panics
///
/// This function will panic if `elements.len() != scratch\_space.len()`.
pub fn invert\_with\_external\_scratch(elements: &mut [F], scratch\_space: &mut [F]) -> F
where
F: Field + ConstantTimeEq,
{
assert\_eq!(elements.len(), scratch\_space.len());
let mut acc = F::ONE;
for (p, scratch) in elements.iter().zip(scratch\_space.iter\_mut()) {
\*scratch = acc;
acc = F::conditional\_select(&(acc \* \*p), &acc, p.is\_zero());
}
acc = acc.invert().unwrap();
let allinv = acc;
for (p, scratch) in elements.iter\_mut().zip(scratch\_space.iter()).rev() {
let tmp = \*scratch \* acc;
let skip = p.is\_zero();
acc = F::conditional\_select(&(acc \* \*p), &acc, skip);
\*p = F::conditional\_select(&tmp, p, skip);
}
allinv
}
/// Inverts each field element in `items` (when nonzero). Zero-valued elements are
/// left as zero.
///
/// - `element` is a function that extracts the element to be inverted from `items`.
/// - `scratch\_space` is a function that extracts the scratch space from `items`.
///
/// Returns the inverse of the product of all nonzero field elements.
pub fn invert\_with\_internal\_scratch(
items: &mut [T],
element: TE,
scratch\_space: TS,
) -> F
where
F: Field + ConstantTimeEq,
TE: Fn(&mut T) -> &mut F,
TS: Fn(&mut T) -> &mut F,
{
let mut acc = F::ONE;
for item in items.iter\_mut() {
\*(scratch\_space)(item) = acc;
let p = (element)(item);
acc = F::conditional\_select(&(acc \* \*p), &acc, p.is\_zero());
}
acc = acc.invert().unwrap();
let allinv = acc;
for item in items.iter\_mut().rev() {
let tmp = \*(scratch\_space)(item) \* acc;
let p = (element)(item);
let skip = p.is\_zero();
acc = F::conditional\_select(&(acc \* \*p), &acc, skip);
\*p = F::conditional\_select(&tmp, p, skip);
}
allinv
}
}