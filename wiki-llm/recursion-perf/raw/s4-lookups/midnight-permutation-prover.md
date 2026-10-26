# Midnight 0.8.3 copy permutation prover
> Source: https://docs.rs/crate/midnight-proofs/0.8.3/source/src/plonk/permutation/prover.rs
> Collected: 2026-10-26
> Published: Unknown

midnight-proofs 0.8.3 - Docs.rs

[Docs.rs](/)

* [midnight-proofs-0.8.3](/crate/midnight-proofs/0.8.3 "Fast PLONK-based zero-knowledge proving system
  ")

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

midnight-proofs 0.8.3
=====================

Fast PLONK-based zero-knowledge proving system

* [Crate](/crate/midnight-proofs/0.8.3)
* [Source](/crate/midnight-proofs/0.8.3/source/src/plonk/permutation/prover.rs)
* [Builds](/crate/midnight-proofs/0.8.3/builds)
* [Feature flags](/crate/midnight-proofs/0.8.3/features)

* Hide files
* [..](../)
* [keygen.rs](./keygen.rs)
* [prover.rs](./prover.rs)
* [verifier.rs](./verifier.rs)

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
132
133
134
135
136
137
138
139
140
141
142
143
144
145
146
147
148
149
150
151
152
153
154
155
156
157
158
159
160
161
162
163
164
165
166
167
168
169
170
171
172
173
174
175
176
177
178
179
180
181
182
183
184
185
186
187
188
189
190
191
192
193
194
195
196
197
198
199
200
201
202
203
204
205
206
207
208
209
210
211
212
213
214
215
216
217
218
219
220
221
222
223
224
225
226
227
228
229
230
231
232
233
234
235
236
237
238
239
240
241
242
243
244
245
246
247
248
249
250
251
252
253
254
255
256
257
258
259
260
261
262
263
264
265
266
267
268
269
270
271
272
273
```

```
use std::iter::{self, ExactSizeIterator};

use ff::{PrimeField, WithSmallOrderMulGroup};
use group::ff::BatchInvert;
use rand_core::{CryptoRng, RngCore};

use super::{super::circuit::Any, Argument, ProvingKey};
use crate::{
    plonk::{self, Error},
    poly::{
        commitment::PolynomialCommitmentScheme, Coeff, LagrangeCoeff, Polynomial, ProverQuery,
        Rotation,
    },
    transcript::{Hashable, Transcript},
    utils::arithmetic::{eval_polynomial, parallelize},
};

#[cfg_attr(feature = "bench-internal", derive(Clone))]
#[derive(Debug)]
pub(crate) struct CommittedSet<F: PrimeField> {
    pub(crate) permutation_product_poly: Polynomial<F, Coeff>,
}

#[cfg_attr(feature = "bench-internal", derive(Clone))]
#[derive(Debug)]
pub(crate) struct Committed<F: PrimeField> {
    pub(crate) sets: Vec<CommittedSet<F>>,
}

pub(crate) struct Evaluated<F: PrimeField> {
    constructed: Committed<F>,
}

impl Argument {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn commit<
        F: WithSmallOrderMulGroup<3>,
        CS: PolynomialCommitmentScheme<F>,
        T: Transcript,
    >(
        &self,
        params: &CS::Parameters,
        pk: &plonk::ProvingKey<F, CS>,
        pkey: &ProvingKey<F>,
        advice: &[Polynomial<F, LagrangeCoeff>],
        fixed: &[Polynomial<F, LagrangeCoeff>],
        instance: &[Polynomial<F, LagrangeCoeff>],
        beta: F,
        gamma: F,
        rng: &mut (impl RngCore + CryptoRng),
        transcript: &mut T,
    ) -> Result<Committed<F>, Error>
    where
        CS::Commitment: Hashable<T::Hash>,
    {
        let domain = &pk.vk.domain;

        // How many columns can be included in a single permutation polynomial?
        // We need to multiply by z(X) and (1 - (l_last(X) + l_blind(X))). This
        // will never underflow because of the requirement of at least a degree
        // 3 circuit for the permutation argument.
        assert!(pk.vk.cs_degree >= 3);
        let chunk_len = pk.vk.cs_degree - 2;
        let blinding_factors = pk.vk.cs.blinding_factors();

        // Each column gets its own delta power.
        let mut deltaomega = F::ONE;

        // Track the "last" value from the previous column set
        let mut last_z = F::ONE;

        let mut sets = vec![];

        for (columns, permutations) in
            self.columns.chunks(chunk_len).zip(pkey.permutations.chunks(chunk_len))
        {
            // Goal is to compute the products of fractions
            //
            // (p_j(\omega^i) + \delta^j \omega^i \beta + \gamma) /
            // (p_j(\omega^i) + \beta s_j(\omega^i) + \gamma)
            //
            // where p_j(X) is the jth column in this permutation,
            // and i is the ith row of the column.

            let mut modified_values = vec![F::ONE; domain.n as usize];

            // Iterate over each column of the permutation
            for (&column, permuted_column_values) in columns.iter().zip(permutations.iter()) {
                let values = match column.column_type() {
                    Any::Advice(_) => advice,
                    Any::Fixed => fixed,
                    Any::Instance => instance,
                };
                parallelize(&mut modified_values, |modified_values, start| {
                    for ((modified_values, value), permuted_value) in modified_values
                        .iter_mut()
                        .zip(values[column.index()][start..].iter())
                        .zip(permuted_column_values[start..].iter())
                    {
                        *modified_values *= &(beta * permuted_value + &gamma + value);
                    }
                });
            }

            // Invert to obtain the denominator for the permutation product polynomial
            modified_values.batch_invert();

            // Iterate over each column again, this time finishing the computation
            // of the entire fraction by computing the numerators
            let omega = domain.get_omega();
            for &column in columns.iter() {
                let values = match column.column_type() {
                    Any::Advice(_) => advice,
                    Any::Fixed => fixed,
                    Any::Instance => instance,
                };
                parallelize(&mut modified_values, |modified_values, start| {
                    let mut deltaomega = deltaomega * &omega.pow_vartime([start as u64, 0, 0, 0]);
                    for (modified_values, value) in
                        modified_values.iter_mut().zip(values[column.index()][start..].iter())
                    {
                        // Multiply by p_j(\omega^i) + \delta^j \omega^i \beta
                        *modified_values *= &(deltaomega * &beta + &gamma + value);
                        deltaomega *= &omega;
                    }
                });
                deltaomega *= &F::DELTA;
            }

            // The modified_values vector is a vector of products of fractions
            // of the form
            //
            // (p_j(\omega^i) + \delta^j \omega^i \beta + \gamma) /
            // (p_j(\omega^i) + \beta s_j(\omega^i) + \gamma)
            //
            // where i is the index into modified_values, for the jth column in
            // the permutation

            // Compute the evaluations of the permutation product polynomial
            // over our domain, starting with z[0] = 1
            let mut z = Vec::with_capacity(domain.n as usize);
            z.push(last_z);
            for row in 1..(domain.n as usize) {
                let tmp = z[row - 1] * &modified_values[row - 1];
                z.push(tmp);
            }
            let mut z = domain.lagrange_from_vec(z);
            // Set blinding factors
            for z in &mut z[domain.n as usize - blinding_factors..] {
                *z = F::random(&mut *rng);
            }
            // Set new last_z
            last_z = z[domain.n as usize - (blinding_factors + 1)];

            let permutation_product_commitment = CS::commit_lagrange(params, &z);
            let permutation_product_poly = domain.lagrange_to_coeff(z);

            // Hash the permutation product commitment
            transcript.write(&permutation_product_commitment)?;

            sets.push(CommittedSet {
                permutation_product_poly,
            });
        }

        Ok(Committed { sets })
    }
}

impl<F: PrimeField> super::ProvingKey<F> {
    pub(crate) fn open(&self, x: F) -> impl Iterator<Item = ProverQuery<'_, F>> + Clone {
        self.polys.iter().map(move |poly| ProverQuery { point: x, poly })
    }

    pub(crate) fn evaluate<T: Transcript>(&self, x: F, transcript: &mut T) -> Result<(), Error>
    where
        F: Hashable<T::Hash>,
    {
        // Hash permutation evals
        for eval in self.polys.iter().map(|poly| eval_polynomial(poly, x)) {
            transcript.write(&eval)?;
        }

        Ok(())
    }
}

impl<F: WithSmallOrderMulGroup<3>> Committed<F> {
    pub(crate) fn evaluate<T: Transcript, CS: PolynomialCommitmentScheme<F>>(
        self,
        pk: &plonk::ProvingKey<F, CS>,
        x: F,
        transcript: &mut T,
    ) -> Result<Evaluated<F>, Error>
    where
        F: Hashable<T::Hash>,
    {
        let domain = &pk.vk.domain;
        let blinding_factors = pk.vk.cs.blinding_factors();

        {
            let mut sets = self.sets.iter();

            while let Some(set) = sets.next() {
                let permutation_product_eval = eval_polynomial(&set.permutation_product_poly, x);

                let permutation_product_next_eval = eval_polynomial(
                    &set.permutation_product_poly,
                    domain.rotate_omega(x, Rotation::next()),
                );

                // Hash permutation product evals
                for eval in iter::empty()
                    .chain(Some(&permutation_product_eval))
                    .chain(Some(&permutation_product_next_eval))
                {
                    transcript.write(eval)?;
                }

                // If we have any remaining sets to process, evaluate this set at omega^u
                // so we can constrain the last value of its running product to equal the
                // first value of the next set's running product, chaining them together.
                if sets.len() > 0 {
                    let permutation_product_last_eval = eval_polynomial(
                        &set.permutation_product_poly,
                        domain.rotate_omega(x, Rotation(-((blinding_factors + 1) as i32))),
                    );

                    transcript.write(&permutation_product_last_eval)?;
                }
            }
        }

        Ok(Evaluated { constructed: self })
    }
}

impl<F: WithSmallOrderMulGroup<3>> Evaluated<F> {
    pub(crate) fn open<'a, CS: PolynomialCommitmentScheme<F>>(
        &'a self,
        pk: &'a plonk::ProvingKey<F, CS>,
        x: F,
    ) -> impl Iterator<Item = ProverQuery<'a, F>> + Clone {
        let blinding_factors = pk.vk.cs.blinding_factors();
        let x_next = pk.vk.domain.rotate_omega(x, Rotation::next());
        let x_last = pk.vk.domain.rotate_omega(x, Rotation(-((blinding_factors + 1) as i32)));

        iter::empty()
            .chain(self.constructed.sets.iter().flat_map(move |set| {
                iter::empty()
                    // Open permutation product commitments at x and \omega x
                    .chain(Some(ProverQuery {
                        point: x,
                        poly: &set.permutation_product_poly,
                    }))
                    .chain(Some(ProverQuery {
                        point: x_next,
                        poly: &set.permutation_product_poly,
                    }))
            }))
            // Open it at \omega^{last} x for all but the last set. This rotation is only
            // sensical for the first row, but we only use this rotation in a constraint
            // that is gated on l_0.
            .chain(
                self.constructed.sets.iter().rev().skip(1).flat_map(move |set| {
                    Some(ProverQuery {
                        point: x_last,
                        poly: &set.permutation_product_poly,
                    })
                }),
            )
    }
}
```