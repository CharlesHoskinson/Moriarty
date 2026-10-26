# ff 0.13.1 batch inversion
> Source: https://docs.rs/crate/ff/0.13.1/source/src/batch.rs
> Collected: 2026-10-26
> Published: Unknown

ff 0.13.1 - Docs.rs

[Docs.rs](/)

* [ff-0.13.1](/crate/ff/0.13.1 "Library for building and interfacing with finite fields")
* [Platform](#)
  + [i686-pc-windows-msvc](/ff/0.13.1/i686-pc-windows-msvc/ff/)
  + [i686-unknown-linux-gnu](/ff/0.13.1/i686-unknown-linux-gnu/ff/)
  + [x86\_64-apple-darwin](/ff/0.13.1/x86_64-apple-darwin/ff/)
  + [x86\_64-pc-windows-msvc](/ff/0.13.1/x86_64-pc-windows-msvc/ff/)
  + [x86\_64-unknown-linux-gnu](/ff/0.13.1/ff/)
* [Feature flags](/crate/ff/0.13.1/features "Browse available feature flags of ff-0.13.1")

* [docs.rs](#)
  + [About docs.rs](/about)
  + [Badges](/about/badges)
  + [Builds](/about/builds)
  + [Metadata](/about/metadata)
  + [Shorthand URLs](/about/redirections)
  + [Download](/about/download)
  + [Rustdoc JSON](/about/rustdoc-json)
  + [Build queue](/releases/queue)
  + [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

* [Rust](#)
  + [Rust website](https://www.rust-lang.org/)
  + [The Book](https://doc.rust-lang.org/book/)
  + [Standard Library API Reference](https://doc.rust-lang.org/std/)
  + [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  + [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  + [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

ff 0.13.1
=========

Library for building and interfacing with finite fields

* [Crate](/crate/ff/0.13.1)
* [Source](/crate/ff/0.13.1/source/src/batch.rs)
* [Builds](/crate/ff/0.13.1/builds)
* [Feature flags](/crate/ff/0.13.1/features)

[Documentation](/ff/0.13.1/ff/)

* Hide files
* [..](../)
* [batch.rs](./batch.rs)
* [helpers.rs](./helpers.rs)
* [lib.rs](./lib.rs)

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
110
111
112
113
114
115
116
117
118
119
120
121
122
123
124
125
126
127
128
129
130
131
```

```
//! Batched field inversion APIs, using [Montgomery's trick].
//!
//! [Montgomery's trick]: https://zcash.github.io/halo2/background/fields.html#montgomerys-trick

use subtle::ConstantTimeEq;

use crate::Field;

/// Extension trait for iterators over mutable field elements which allows those field
/// elements to be inverted in a batch.
///
/// `I: IntoIterator<Item = &'a mut F: Field + ConstantTimeEq>` implements this trait when
/// the `alloc` feature flag is enabled.
///
/// For non-allocating contexts, see the [`BatchInverter`] struct.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub trait BatchInvert<F: Field> {
    /// Consumes this iterator and inverts each field element (when nonzero). Zero-valued
    /// elements are left as zero.
    ///
    /// Returns the inverse of the product of all nonzero field elements.
    fn batch_invert(self) -> F;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, F, I> BatchInvert<F> for I
where
    F: Field + ConstantTimeEq,
    I: IntoIterator<Item = &'a mut F>,
{
    fn batch_invert(self) -> F {
        let mut acc = F::ONE;
        let iter = self.into_iter();
        let mut tmp = alloc::vec::Vec::with_capacity(iter.size_hint().0);
        for p in iter {
            let q = *p;
            tmp.push((acc, p));
            acc = F::conditional_select(&(acc * q), &acc, q.is_zero());
        }
        acc = acc.invert().unwrap();
        let allinv = acc;

        for (tmp, p) in tmp.into_iter().rev() {
            let skip = p.is_zero();

            let tmp = tmp * acc;
            acc = F::conditional_select(&(acc * *p), &acc, skip);
            *p = F::conditional_select(&tmp, p, skip);
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
    /// - `scratch_space` is a slice of field elements that can be freely overwritten.
    ///
    /// Returns the inverse of the product of all nonzero field elements.
    ///
    /// # Panics
    ///
    /// This function will panic if `elements.len() != scratch_space.len()`.
    pub fn invert_with_external_scratch<F>(elements: &mut [F], scratch_space: &mut [F]) -> F
    where
        F: Field + ConstantTimeEq,
    {
        assert_eq!(elements.len(), scratch_space.len());

        let mut acc = F::ONE;
        for (p, scratch) in elements.iter().zip(scratch_space.iter_mut()) {
            *scratch = acc;
            acc = F::conditional_select(&(acc * *p), &acc, p.is_zero());
        }
        acc = acc.invert().unwrap();
        let allinv = acc;

        for (p, scratch) in elements.iter_mut().zip(scratch_space.iter()).rev() {
            let tmp = *scratch * acc;
            let skip = p.is_zero();
            acc = F::conditional_select(&(acc * *p), &acc, skip);
            *p = F::conditional_select(&tmp, &p, skip);
        }

        allinv
    }

    /// Inverts each field element in `items` (when nonzero). Zero-valued elements are
    /// left as zero.
    ///
    /// - `element` is a function that extracts the element to be inverted from `items`.
    /// - `scratch_space` is a function that extracts the scratch space from `items`.
    ///
    /// Returns the inverse of the product of all nonzero field elements.
    pub fn invert_with_internal_scratch<F, T, TE, TS>(
        items: &mut [T],
        element: TE,
        scratch_space: TS,
    ) -> F
    where
        F: Field + ConstantTimeEq,
        TE: Fn(&mut T) -> &mut F,
        TS: Fn(&mut T) -> &mut F,
    {
        let mut acc = F::ONE;
        for item in items.iter_mut() {
            *(scratch_space)(item) = acc;
            let p = (element)(item);
            acc = F::conditional_select(&(acc * *p), &acc, p.is_zero());
        }
        acc = acc.invert().unwrap();
        let allinv = acc;

        for item in items.iter_mut().rev() {
            let tmp = *(scratch_space)(item) * acc;
            let p = (element)(item);
            let skip = p.is_zero();
            acc = F::conditional_select(&(acc * *p), &acc, skip);
            *p = F::conditional_select(&tmp, &p, skip);
        }

        allinv
    }
}
```