# Midnight proofs 0.8.3 lookup prover
> Source: https://docs.rs/crate/midnight-proofs/0.8.3/source/src/plonk/lookup/prover.rs
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
* [Source](/crate/midnight-proofs/0.8.3/source/src/plonk/lookup/prover.rs)
* [Builds](/crate/midnight-proofs/0.8.3/builds)
* [Feature flags](/crate/midnight-proofs/0.8.3/features)

* Hide files
* [..](../)
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
274
275
276
277
278
279
280
281
282
283
284
285
286
287
288
289
290
291
292
293
294
295
296
297
298
299
300
301
302
303
304
305
306
307
308
309
310
311
312
313
314
315
316
317
318
319
320
321
322
323
324
325
326
327
328
329
330
331
332
333
334
335
336
337
338
339
340
341
342
343
344
345
346
347
348
349
350
351
352
353
354
355
356
357
358
359
360
361
362
363
364
365
366
367
368
369
370
371
372
373
374
375
376
377
378
379
380
381
382
383
384
385
386
387
388
389
390
391
392
393
394
395
396
397
398
399
400
401
402
403
404
405
406
407
408
409
410
411
412
413
414
415
416
417
418
419
420
421
422
423
424
425
426
427
428
429
430
431
432
433
434
435
436
437
438
439
440
441
442
443
444
445
446
447
448
449
450
451
452
453
454
455
456
457
458
459
460
461
```

```
use std::{collections::HashMap, hash::Hash, iter};

use ff::{FromUniformBytes, PrimeField, WithSmallOrderMulGroup};
use group::ff::BatchInvert;
use rand_core::{CryptoRng, RngCore};

use super::{
    super::{circuit::Expression, Error, ProvingKey},
    Argument,
};
use crate::{
    plonk::evaluation::evaluate,
    poly::{
        commitment::PolynomialCommitmentScheme, Coeff, EvaluationDomain, LagrangeCoeff, Polynomial,
        ProverQuery, Rotation,
    },
    transcript::{Hashable, Transcript},
    utils::arithmetic::{eval_polynomial, parallelize},
};

#[cfg_attr(feature = "bench-internal", derive(Clone))]
#[derive(Debug)]
pub(crate) struct Permuted<F: PrimeField> {
    compressed_input_expression: Polynomial<F, LagrangeCoeff>,
    permuted_input_expression: Polynomial<F, LagrangeCoeff>,
    permuted_input_poly: Polynomial<F, Coeff>,
    compressed_table_expression: Polynomial<F, LagrangeCoeff>,
    permuted_table_expression: Polynomial<F, LagrangeCoeff>,
    permuted_table_poly: Polynomial<F, Coeff>,
}

#[cfg_attr(feature = "bench-internal", derive(Clone))]
#[derive(Debug)]
pub(crate) struct Committed<F: PrimeField> {
    pub(crate) permuted_input_poly: Polynomial<F, Coeff>,
    pub(crate) permuted_table_poly: Polynomial<F, Coeff>,
    pub(crate) product_poly: Polynomial<F, Coeff>,
}

pub(crate) struct Evaluated<F: PrimeField> {
    constructed: Committed<F>,
}

impl<F: WithSmallOrderMulGroup<3> + Ord + Hash> Argument<F> {
    /// Given a Lookup with input expressions [A_0, A_1, ..., A_{m-1}] and table
    /// expressions [S_0, S_1, ..., S_{m-1}], this method
    /// - constructs A_compressed = \theta^{m-1} A_0 + theta^{m-2} A_1 + ... +
    ///   \theta A_{m-2} + A_{m-1} and S_compressed = \theta^{m-1} S_0 +
    ///   theta^{m-2} S_1 + ... + \theta S_{m-2} + S_{m-1},
    /// - permutes A_compressed and S_compressed using permute_expression_pair()
    ///   helper, obtaining A' and S', and
    /// - constructs `Permuted<C>` struct using permuted_input_value = A', and
    ///   permuted_table_expression = S'.
    ///
    /// The `Permuted<C>` struct is used to update the Lookup, and is then
    /// returned.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn commit_permuted<
        'a,
        'params: 'a,
        CS: PolynomialCommitmentScheme<F>,
        T: Transcript,
    >(
        &self,
        pk: &ProvingKey<F, CS>,
        params: &'params CS::Parameters,
        domain: &EvaluationDomain<F>,
        theta: F,
        advice_values: &'a [Polynomial<F, LagrangeCoeff>],
        fixed_values: &'a [Polynomial<F, LagrangeCoeff>],
        instance_values: &'a [Polynomial<F, LagrangeCoeff>],
        challenges: &'a [F],
        rng: &mut impl RngCore,
        transcript: &mut T,
    ) -> Result<Permuted<F>, Error>
    where
        F: FromUniformBytes<64>,
        CS::Commitment: Hashable<T::Hash>,
    {
        // Closure to get values of expressions and compress them
        let compress_expressions = |expressions: &[Expression<F>]| {
            let compressed_expression = expressions
                .iter()
                .map(|expression| {
                    pk.vk.domain.lagrange_from_vec(evaluate(
                        expression,
                        domain.n as usize,
                        1,
                        fixed_values,
                        advice_values,
                        instance_values,
                        challenges,
                    ))
                })
                .fold(domain.empty_lagrange(), |acc, expression| {
                    acc * theta + &expression
                });
            compressed_expression
        };

        // Get values of input expressions involved in the lookup and compress them
        let compressed_input_expression = compress_expressions(&self.input_expressions);

        // Get values of table expressions involved in the lookup and compress them
        let compressed_table_expression = compress_expressions(&self.table_expressions);

        // Permute compressed (InputExpression, TableExpression) pair
        let (permuted_input_expression, permuted_table_expression) = permute_expression_pair(
            pk,
            domain,
            rng,
            &compressed_input_expression,
            &compressed_table_expression,
        )?;

        // Closure to construct commitment to vector of values
        let commit_values = |values: &Polynomial<F, LagrangeCoeff>| {
            let poly = pk.vk.domain.lagrange_to_coeff(values.clone());
            let commitment = CS::commit_lagrange(params, values);
            (poly, commitment)
        };

        // Commit to permuted input expression
        let (permuted_input_poly, permuted_input_commitment) =
            commit_values(&permuted_input_expression);

        // Commit to permuted table expression
        let (permuted_table_poly, permuted_table_commitment) =
            commit_values(&permuted_table_expression);

        // Hash permuted input commitment
        transcript.write(&permuted_input_commitment)?;

        // Hash permuted table commitment
        transcript.write(&permuted_table_commitment)?;

        Ok(Permuted {
            compressed_input_expression,
            permuted_input_expression,
            permuted_input_poly,
            compressed_table_expression,
            permuted_table_expression,
            permuted_table_poly,
        })
    }
}

impl<F: WithSmallOrderMulGroup<3>> Permuted<F> {
    /// Given a Lookup with input expressions, table expressions, and the
    /// permuted input expression and permuted table expression, this method
    /// constructs the grand product polynomial over the lookup. The grand
    /// product polynomial is used to populate the `Product<C>` struct. The
    /// `Product<C>` struct is added to the Lookup and finally returned by the
    /// method.
    pub(crate) fn commit_product<CS: PolynomialCommitmentScheme<F>, T: Transcript>(
        self,
        pk: &ProvingKey<F, CS>,
        params: &CS::Parameters,
        beta: F,
        gamma: F,
        rng: &mut (impl RngCore + CryptoRng),
        transcript: &mut T,
    ) -> Result<Committed<F>, Error>
    where
        F: WithSmallOrderMulGroup<3> + FromUniformBytes<64>,
        CS::Commitment: Hashable<T::Hash>,
    {
        let blinding_factors = pk.vk.cs.blinding_factors();
        // Goal is to compute the products of fractions
        //
        // Numerator: (\theta^{m-1} a_0(\omega^i) + \theta^{m-2} a_1(\omega^i) + ... +
        // \theta a_{m-2}(\omega^i) + a_{m-1}(\omega^i) + \beta)
        //            * (\theta^{m-1} s_0(\omega^i) + \theta^{m-2} s_1(\omega^i) + ... +
        //              \theta s_{m-2}(\omega^i) + s_{m-1}(\omega^i) + \gamma)
        // Denominator: (a'(\omega^i) + \beta) (s'(\omega^i) + \gamma)
        //
        // where a_j(X) is the jth input expression in this lookup,
        // where a'(X) is the compression of the permuted input expressions,
        // s_j(X) is the jth table expression in this lookup,
        // s'(X) is the compression of the permuted table expressions,
        // and i is the ith row of the expression.
        let mut lookup_product = vec![F::ZERO; pk.vk.n() as usize];
        // Denominator uses the permuted input expression and permuted table expression
        parallelize(&mut lookup_product, |lookup_product, start| {
            for ((lookup_product, permuted_input_value), permuted_table_value) in lookup_product
                .iter_mut()
                .zip(self.permuted_input_expression[start..].iter())
                .zip(self.permuted_table_expression[start..].iter())
            {
                *lookup_product = (beta + permuted_input_value) * &(gamma + permuted_table_value);
            }
        });

        // Batch invert to obtain the denominators for the lookup product
        // polynomials
        lookup_product.iter_mut().batch_invert();

        // Finish the computation of the entire fraction by computing the numerators
        // (\theta^{m-1} a_0(\omega^i) + \theta^{m-2} a_1(\omega^i) + ... + \theta
        // a_{m-2}(\omega^i) + a_{m-1}(\omega^i) + \beta)
        // * (\theta^{m-1} s_0(\omega^i) + \theta^{m-2} s_1(\omega^i) + ... + \theta
        //   s_{m-2}(\omega^i) + s_{m-1}(\omega^i) + \gamma)
        parallelize(&mut lookup_product, |product, start| {
            for (i, product) in product.iter_mut().enumerate() {
                let i = i + start;

                *product *= &(self.compressed_input_expression[i] + &beta);
                *product *= &(self.compressed_table_expression[i] + &gamma);
            }
        });

        // The product vector is a vector of products of fractions of the form
        //
        // Numerator: (\theta^{m-1} a_0(\omega^i) + \theta^{m-2} a_1(\omega^i) + ... +
        // \theta a_{m-2}(\omega^i) + a_{m-1}(\omega^i) + \beta)
        //            * (\theta^{m-1} s_0(\omega^i) + \theta^{m-2} s_1(\omega^i) + ... +
        //              \theta s_{m-2}(\omega^i) + s_{m-1}(\omega^i) + \gamma)
        // Denominator: (a'(\omega^i) + \beta) (s'(\omega^i) + \gamma)
        //
        // where there are m input expressions and m table expressions,
        // a_j(\omega^i) is the jth input expression in this lookup,
        // a'j(\omega^i) is the permuted input expression,
        // s_j(\omega^i) is the jth table expression in this lookup,
        // s'(\omega^i) is the permuted table expression,
        // and i is the ith row of the expression.

        // Compute the evaluations of the lookup product polynomial
        // over our domain, starting with z[0] = 1
        let z = iter::once(F::ONE)
            .chain(lookup_product)
            .scan(F::ONE, |state, cur| {
                *state *= &cur;
                Some(*state)
            })
            // Take all rows including the "last" row which should
            // be a boolean (and ideally 1, else soundness is broken)
            .take(pk.vk.n() as usize - blinding_factors)
            // Chain random blinding factors.
            .chain((0..blinding_factors).map(|_| F::random(&mut *rng)))
            .collect::<Vec<_>>();
        assert_eq!(z.len(), pk.vk.n() as usize);
        let z = pk.vk.domain.lagrange_from_vec(z);

        #[cfg(debug_assertions)]
        // This test works only with intermediate representations in this method.
        // It can be used for debugging purposes.
        {
            // While in Lagrange representation, check that product is correctly constructed
            let u = (pk.vk.n() as usize) - (blinding_factors + 1);

            // l_0(X) * (1 - z(X)) = 0
            assert_eq!(z[0], F::ONE);

            // z(\omega X) (a'(X) + \beta) (s'(X) + \gamma)
            // - z(X) (\theta^{m-1} a_0(X) + ... + a_{m-1}(X) + \beta) (\theta^{m-1} s_0(X)
            //   + ... + s_{m-1}(X) + \gamma)
            for i in 0..u {
                let mut left = z[i + 1];
                let permuted_input_value = &self.permuted_input_expression[i];

                let permuted_table_value = &self.permuted_table_expression[i];

                left *= &(beta + permuted_input_value);
                left *= &(gamma + permuted_table_value);

                let mut right = z[i];
                let mut input_term = self.compressed_input_expression[i];
                let mut table_term = self.compressed_table_expression[i];

                input_term += &(beta);
                table_term += &(gamma);
                right *= &(input_term * &table_term);

                assert_eq!(left, right);
            }

            // l_last(X) * (z(X)^2 - z(X)) = 0
            // Assertion will fail only when soundness is broken, in which
            // case this z[u] value will be zero. (bad!)
            assert_eq!(z[u], F::ONE);
        }

        let product_commitment = CS::commit_lagrange(params, &z);
        let z = pk.vk.domain.lagrange_to_coeff(z);

        // Hash product commitment
        transcript.write(&product_commitment)?;

        Ok(Committed::<F> {
            permuted_input_poly: self.permuted_input_poly,
            permuted_table_poly: self.permuted_table_poly,
            product_poly: z,
        })
    }
}

impl<F: WithSmallOrderMulGroup<3>> Committed<F> {
    pub(crate) fn evaluate<T: Transcript, CS: PolynomialCommitmentScheme<F>>(
        self,
        pk: &ProvingKey<F, CS>,
        x: F,
        transcript: &mut T,
    ) -> Result<Evaluated<F>, Error>
    where
        F: Hashable<T::Hash>,
    {
        let domain = &pk.vk.domain;
        let x_inv = domain.rotate_omega(x, Rotation::prev());
        let x_next = domain.rotate_omega(x, Rotation::next());

        let product_eval = eval_polynomial(&self.product_poly, x);
        let product_next_eval = eval_polynomial(&self.product_poly, x_next);
        let permuted_input_eval = eval_polynomial(&self.permuted_input_poly, x);
        let permuted_input_inv_eval = eval_polynomial(&self.permuted_input_poly, x_inv);
        let permuted_table_eval = eval_polynomial(&self.permuted_table_poly, x);

        // Hash each advice evaluation
        for eval in iter::empty()
            .chain(Some(product_eval))
            .chain(Some(product_next_eval))
            .chain(Some(permuted_input_eval))
            .chain(Some(permuted_input_inv_eval))
            .chain(Some(permuted_table_eval))
        {
            transcript.write(&eval)?;
        }

        Ok(Evaluated { constructed: self })
    }
}

impl<F: WithSmallOrderMulGroup<3>> Evaluated<F> {
    pub(crate) fn open<'a, CS: PolynomialCommitmentScheme<F>>(
        &'a self,
        pk: &'a ProvingKey<F, CS>,
        x: F,
    ) -> impl Iterator<Item = ProverQuery<'a, F>> + Clone {
        let x_inv = pk.vk.domain.rotate_omega(x, Rotation::prev());
        let x_next = pk.vk.domain.rotate_omega(x, Rotation::next());

        iter::empty()
            // Open lookup product commitments at x
            .chain(Some(ProverQuery {
                point: x,
                poly: &self.constructed.product_poly,
            }))
            // Open lookup input commitments at x
            .chain(Some(ProverQuery {
                point: x,
                poly: &self.constructed.permuted_input_poly,
            }))
            // Open lookup table commitments at x
            .chain(Some(ProverQuery {
                point: x,
                poly: &self.constructed.permuted_table_poly,
            }))
            // Open lookup input commitments at x_inv
            .chain(Some(ProverQuery {
                point: x_inv,
                poly: &self.constructed.permuted_input_poly,
            }))
            // Open lookup product commitments at x_next
            .chain(Some(ProverQuery {
                point: x_next,
                poly: &self.constructed.product_poly,
            }))
    }
}

type ExpressionPair<F> = (Polynomial<F, LagrangeCoeff>, Polynomial<F, LagrangeCoeff>);

/// Given a vector of input values A and a vector of table values S,
/// this method permutes A and S to produce A' and S', such that:
/// - like values in A' are vertically adjacent to each other; and
/// - the first row in a sequence of like values in A' is the row that has the
///   corresponding value in S'.
///
/// This method returns (A', S') if no errors are encountered.
fn permute_expression_pair<F, CS: PolynomialCommitmentScheme<F>>(
    pk: &ProvingKey<F, CS>,
    domain: &EvaluationDomain<F>,
    rng: &mut impl RngCore,
    input_expression: &Polynomial<F, LagrangeCoeff>,
    table_expression: &Polynomial<F, LagrangeCoeff>,
) -> Result<ExpressionPair<F>, Error>
where
    F: WithSmallOrderMulGroup<3> + Hash + Ord + FromUniformBytes<64>,
{
    let blinding_factors = pk.vk.cs.blinding_factors();
    let usable_rows = pk.vk.n() as usize - (blinding_factors + 1);

    let mut permuted_input_expression: Vec<F> = input_expression.to_vec();
    permuted_input_expression.truncate(usable_rows);

    // Sort input lookup expression values
    permuted_input_expression.sort();

    // A HashMap of each unique element in the table expression and its count
    let mut leftover_table_map = HashMap::<F, u32>::with_capacity(table_expression.len());
    table_expression.iter().take(usable_rows).for_each(|coeff| {
        *leftover_table_map.entry(*coeff).or_insert(0) += 1;
    });
    let mut permuted_table_coeffs = vec![F::ZERO; usable_rows];

    let mut repeated_input_rows = permuted_input_expression
        .iter()
        .zip(permuted_table_coeffs.iter_mut())
        .enumerate()
        .filter_map(|(row, (input_value, table_value))| {
            // If this is the first occurrence of `input_value` in the input expression
            if row == 0 || *input_value != permuted_input_expression[row - 1] {
                *table_value = *input_value;
                // Remove one instance of input_value from leftover_table_map
                if let Some(count) = leftover_table_map.get_mut(input_value) {
                    assert!(*count > 0);
                    *count -= 1;
                    None
                } else {
                    // Return error if input_value not found
                    Some(Err(Error::ConstraintSystemFailure))
                }
            // If input value is repeated
            } else {
                Some(Ok(row))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Populate permuted table at unfilled rows with leftover table elements
    for (coeff, count) in leftover_table_map.iter() {
        for _ in 0..*count {
            permuted_table_coeffs[repeated_input_rows.pop().unwrap()] = *coeff;
        }
    }
    assert!(repeated_input_rows.is_empty());

    permuted_input_expression.extend((0..(blinding_factors + 1)).map(|_| F::random(&mut *rng)));
    permuted_table_coeffs.extend((0..(blinding_factors + 1)).map(|_| F::random(&mut *rng)));
    assert_eq!(permuted_input_expression.len(), pk.vk.n() as usize);
    assert_eq!(permuted_table_coeffs.len(), pk.vk.n() as usize);

    #[cfg(debug_assertions)]
    {
        let mut last = None;
        for (a, b) in permuted_input_expression
            .iter()
            .zip(permuted_table_coeffs.iter())
            .take(usable_rows)
        {
            if *a != *b {
                assert_eq!(*a, last.unwrap());
            }
            last = Some(*a);
        }
    }

    Ok((
        domain.lagrange_from_vec(permuted_input_expression),
        domain.lagrange_from_vec(permuted_table_coeffs),
    ))
}
```