# Midnight proofs 0.8.3 proving-key serialization and reconstruction
> Source: https://docs.rs/crate/midnight-proofs/0.8.3/source/src/plonk/mod.rs
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
* [Source](/crate/midnight-proofs/0.8.3/source/src/plonk/mod.rs)
* [Builds](/crate/midnight-proofs/0.8.3/builds)
* [Feature flags](/crate/midnight-proofs/0.8.3/features)

* Hide files
* [..](../)
* [bench](./bench/)
* [lookup](./lookup/)
* [permutation](./permutation/)
* [trash](./trash/)
* [vanishing](./vanishing/)
* [circuit.rs](./circuit.rs)
* [error.rs](./error.rs)
* [evaluation.rs](./evaluation.rs)
* [keygen.rs](./keygen.rs)
* [lookup.rs](./lookup.rs)
* [mod.rs](./mod.rs)
* [permutation.rs](./permutation.rs)
* [prover.rs](./prover.rs)
* [traces.rs](./traces.rs)
* [trash.rs](./trash.rs)
* [vanishing.rs](./vanishing.rs)
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
462
463
464
465
466
467
468
469
470
471
472
473
474
475
476
477
478
479
480
481
482
483
484
485
486
487
488
489
490
491
492
493
494
495
496
497
498
499
500
501
502
503
504
505
506
507
508
509
510
511
512
513
514
515
516
517
518
519
520
521
522
523
524
525
526
527
528
529
530
531
532
533
534
535
536
537
538
539
540
541
542
543
544
545
546
547
548
549
550
551
552
553
554
555
556
557
558
559
560
561
562
563
564
565
566
567
568
569
570
571
572
573
574
575
576
577
578
579
580
```

```
//! This module provides an implementation of a variant of (Turbo)[PLONK][plonk]
//! that is designed specifically for the polynomial commitment scheme described
//! in the [Halo][halo] paper.
//!
//! [halo]: https://eprint.iacr.org/2019/1021
//! [plonk]: https://eprint.iacr.org/2019/953

use blake2b_simd::Params as Blake2bParams;
use group::ff::FromUniformBytes;

use crate::{
    plonk::{
        permutation::{expressions, verifier::CommonEvaluated},
        vanishing::verifier::{Evaluated, PartiallyEvaluated},
    },
    poly::{
        Coeff, EvaluationDomain, ExtendedLagrangeCoeff, LagrangeCoeff, PinnedEvaluationDomain,
        Polynomial,
    },
    transcript::{Hashable, Transcript},
    utils::{
        helpers::{
            byte_length, polynomial_slice_byte_length, read_polynomial_vec, write_polynomial_slice,
            ProcessedSerdeObject,
        },
        SerdeFormat,
    },
};

mod circuit;
mod error;
pub(crate) mod evaluation;
mod keygen;
pub(crate) mod lookup;
pub mod permutation;
pub(crate) mod traces;
pub(crate) mod trash;
pub(crate) mod vanishing;

#[cfg(feature = "bench-internal")]
pub mod bench;

mod prover;
mod verifier;

use std::{io, iter};

pub use circuit::*;
pub use error::*;
pub(crate) use evaluation::Evaluator;
use ff::{PrimeField, WithSmallOrderMulGroup};
pub use keygen::*;
use midnight_curves::serde::SerdeObject;
pub use prover::*;
pub use verifier::*;

use crate::poly::commitment::PolynomialCommitmentScheme;

/// This is a verifying key which allows for the verification of proofs for a
/// particular circuit.
#[derive(Clone, Debug)]
pub struct VerifyingKey<F: PrimeField, CS: PolynomialCommitmentScheme<F>> {
    domain: EvaluationDomain<F>,
    fixed_commitments: Vec<CS::Commitment>,
    permutation: permutation::VerifyingKey<F, CS>,
    cs: ConstraintSystem<F>,
    /// Cached maximum degree of `cs` (which doesn't change after construction).
    cs_degree: usize,
    /// The representative of this `VerifyingKey` in transcripts.
    transcript_repr: F,
}

// Current version of the VK
const VERSION: u8 = 0x03;

impl<F, CS> VerifyingKey<F, CS>
where
    F: WithSmallOrderMulGroup<3> + FromUniformBytes<64>,
    CS: PolynomialCommitmentScheme<F>,
{
    /// Returns `n`
    pub fn n(&self) -> u64 {
        self.domain.n
    }
    /// Writes a verifying key to a buffer.
    ///
    /// Writes a curve element according to `format`:
    /// - `Processed`: Writes a compressed curve element with coordinates in
    ///   standard form. Writes a field element in standard form, with
    ///   endianness specified by the `PrimeField` implementation.
    /// - Otherwise: Writes an uncompressed curve element with coordinates in
    ///   Montgomery form Writes a field element into raw bytes in its internal
    ///   Montgomery representation, WITHOUT performing the expensive Montgomery
    ///   reduction.
    pub fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
        // Version byte that will be checked on read.
        writer.write_all(&[VERSION])?;
        let k = &self.domain.k();
        assert!(*k <= F::S);
        // k value fits in 1 byte
        writer.write_all(&[*k as u8])?;
        writer.write_all(&(self.fixed_commitments.len() as u32).to_le_bytes())?;
        for commitment in &self.fixed_commitments {
            commitment.write(writer, format)?;
        }
        self.permutation.write(writer, format)?;

        Ok(())
    }

    /// Reads a verification key from a buffer for the associated [Circuit].
    ///
    /// Reads a curve element from the buffer and parses it according to the
    /// `format`:
    /// - `Processed`: Reads a compressed curve element and decompresses it.
    ///   Reads a field element in standard form, with endianness specified by
    ///   the `PrimeField` implementation, and checks that the element is less
    ///   than the modulus.
    /// - `RawBytes`: Reads an uncompressed curve element with coordinates in
    ///   Montgomery form. Checks that field elements are less than modulus, and
    ///   then checks that the point is on the curve.
    /// - `RawBytesUnchecked`: Reads an uncompressed curve element with
    ///   coordinates in Montgomery form; does not perform any checks.
    pub fn read<R: io::Read, ConcreteCircuit: Circuit<F>>(
        reader: &mut R,
        format: SerdeFormat,
        #[cfg(feature = "circuit-params")] params: ConcreteCircuit::Params,
    ) -> io::Result<Self> {
        let mut cs = ConstraintSystem::default();
        #[cfg(feature = "circuit-params")]
        let _config = ConcreteCircuit::configure_with_params(&mut cs, params);
        #[cfg(not(feature = "circuit-params"))]
        let _config = ConcreteCircuit::configure(&mut cs);

        Self::read_from_cs(reader, format, cs)
    }

    /// Reads a verification key from a buffer, using the provided
    /// [ConstraintSystem].
    ///
    /// Reads a curve element from the buffer and parses it according to the
    /// `format`:
    /// - `Processed`: Reads a compressed curve element and decompresses it.
    ///   Reads a field element in standard form, with endianness specified by
    ///   the `PrimeField` implementation, and checks that the element is less
    ///   than the modulus.
    /// - `RawBytes`: Reads an uncompressed curve element with coordinates in
    ///   Montgomery form. Checks that field elements are less than modulus, and
    ///   then checks that the point is on the curve.
    /// - `RawBytesUnchecked`: Reads an uncompressed curve element with
    ///   coordinates in Montgomery form; does not perform any checks.
    pub fn read_from_cs<R: io::Read>(
        reader: &mut R,
        format: SerdeFormat,
        cs: ConstraintSystem<F>,
    ) -> io::Result<Self> {
        let mut version_byte = [0u8; 1];
        reader.read_exact(&mut version_byte)?;
        if VERSION != version_byte[0] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected version byte",
            ));
        }

        let mut k = [0u8; 1];
        reader.read_exact(&mut k)?;
        let k = u8::from_le_bytes(k);
        if k as u32 > F::S {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("circuit size value (k): {} exceeds maxium: {}", k, F::S),
            ));
        }

        let domain = EvaluationDomain::new(cs.degree() as u32, k.into());

        let mut num_fixed_columns = [0u8; 4];
        reader.read_exact(&mut num_fixed_columns)?;
        let num_fixed_columns = u32::from_le_bytes(num_fixed_columns);

        let fixed_commitments: Vec<_> = (0..num_fixed_columns)
            .map(|_| CS::Commitment::read(reader, format))
            .collect::<Result<_, _>>()?;

        let permutation = permutation::VerifyingKey::read(reader, &cs.permutation, format)?;

        // we still need to replace selectors with fixed Expressions in `cs`
        let fake_selectors = vec![vec![]; cs.num_selectors];
        let (cs, _) = cs.directly_convert_selectors_to_fixed(fake_selectors);

        Ok(Self::from_parts(domain, fixed_commitments, permutation, cs))
    }

    /// Writes a verifying key to a vector of bytes using [`Self::write`].
    pub fn to_bytes(&self, format: SerdeFormat) -> Vec<u8> {
        let mut bytes = Vec::<u8>::with_capacity(self.bytes_length(format));
        Self::write(self, &mut bytes, format).expect("Writing to vector should not fail");
        bytes
    }

    /// Reads a verification key from a slice of bytes using [`Self::read`].
    pub fn from_bytes<ConcreteCircuit: Circuit<F>>(
        mut bytes: &[u8],
        format: SerdeFormat,
        #[cfg(feature = "circuit-params")] params: ConcreteCircuit::Params,
    ) -> io::Result<Self> {
        Self::read::<_, ConcreteCircuit>(
            &mut bytes,
            format,
            #[cfg(feature = "circuit-params")]
            params,
        )
    }
}

impl<F: WithSmallOrderMulGroup<3>, CS: PolynomialCommitmentScheme<F>> VerifyingKey<F, CS> {
    /// Return the bytes_length of a VerifyingKey
    pub fn bytes_length(&self, format: SerdeFormat) -> usize {
        10 + (self.fixed_commitments.len() * byte_length::<CS::Commitment>(format))
            + self.permutation.bytes_length(format)
    }

    fn from_parts(
        domain: EvaluationDomain<F>,
        fixed_commitments: Vec<CS::Commitment>,
        permutation: permutation::VerifyingKey<F, CS>,
        cs: ConstraintSystem<F>,
    ) -> Self
    where
        F: FromUniformBytes<64>,
    {
        // Compute cached values.
        let cs_degree = cs.degree();

        let mut vk = Self {
            domain,
            fixed_commitments,
            permutation,
            cs,
            cs_degree,
            // Temporary, this is not pinned.
            transcript_repr: F::ZERO,
        };

        let mut hasher =
            Blake2bParams::new().hash_length(64).personal(b"Halo2-Verify-Key").to_state();

        // We serialise the commitments of the VK to get the `transcript_repr`.
        let mut buffer = Vec::new();
        buffer.push(VERSION);
        let k = &vk.domain.k();
        assert!(*k <= F::S);
        buffer.push(*k as u8);
        buffer.extend_from_slice(&(vk.fixed_commitments.len() as u32).to_le_bytes());
        for commitment in &vk.fixed_commitments {
            commitment
                .write(&mut buffer, SerdeFormat::RawBytesUnchecked)
                .expect("Failed to write to buffer - this is a bug.");
        }

        buffer.extend_from_slice(&(vk.permutation.commitments().len() as u32).to_le_bytes());
        for commitment in vk.permutation.commitments() {
            commitment
                .write(&mut buffer, SerdeFormat::RawBytesUnchecked)
                .expect("Failed to write to buffer - this is a bug.");
        }

        // We use the debug implementation to add the gates and domain to the hashed
        // buffer. We should eventually move away from debug implementation for
        // this purpose. See https://github.com/midnightntwrk/halo2/issues/5
        buffer.extend_from_slice(format!("{:?}", vk.get_domain().pinned()).as_bytes());
        buffer.extend_from_slice(format!("{:?}", vk.cs().pinned()).as_bytes());

        hasher.update(&buffer);

        // Hash in final Blake2bState
        vk.transcript_repr = F::from_uniform_bytes(hasher.finalize().as_array());

        vk
    }

    /// Hashes a verification key into a transcript.
    pub fn hash_into<T: Transcript>(&self, transcript: &mut T) -> io::Result<()>
    where
        F: Hashable<T::Hash>,
    {
        transcript.common(&self.transcript_repr)?;

        Ok(())
    }

    /// Obtains a pinned representation of this verification key that contains
    /// the minimal information necessary to reconstruct the verification key.
    pub fn pinned(&self) -> PinnedVerificationKey<'_, F, CS> {
        PinnedVerificationKey {
            domain: self.domain.pinned(),
            fixed_commitments: &self.fixed_commitments,
            permutation: &self.permutation,
            cs: self.cs.pinned(),
        }
    }

    /// Returns commitments of fixed polynomials
    pub fn fixed_commitments(&self) -> &Vec<CS::Commitment> {
        &self.fixed_commitments
    }

    /// Returns `VerifyingKey` of permutation
    pub fn permutation(&self) -> &permutation::VerifyingKey<F, CS> {
        &self.permutation
    }

    /// Returns `ConstraintSystem`
    pub fn cs(&self) -> &ConstraintSystem<F> {
        &self.cs
    }

    /// Returns representative of this `VerifyingKey` in transcripts
    pub fn transcript_repr(&self) -> F {
        self.transcript_repr
    }
}

/// Minimal representation of a verification key that can be used to identify
/// its active contents.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PinnedVerificationKey<'a, F: PrimeField, CS: PolynomialCommitmentScheme<F>> {
    domain: PinnedEvaluationDomain<'a, F>,
    cs: PinnedConstraintSystem<'a, F>,
    fixed_commitments: &'a Vec<CS::Commitment>,
    permutation: &'a permutation::VerifyingKey<F, CS>,
}
/// This is a proving key which allows for the creation of proofs for a
/// particular circuit.
#[derive(Clone, Debug)]
pub struct ProvingKey<F: PrimeField, CS: PolynomialCommitmentScheme<F>> {
    pub(crate) vk: VerifyingKey<F, CS>,
    pub(crate) l0: Polynomial<F, ExtendedLagrangeCoeff>,
    pub(crate) l_last: Polynomial<F, ExtendedLagrangeCoeff>,
    pub(crate) l_active_row: Polynomial<F, ExtendedLagrangeCoeff>,
    pub(crate) fixed_values: Vec<Polynomial<F, LagrangeCoeff>>,
    pub(crate) fixed_polys: Vec<Polynomial<F, Coeff>>,
    pub(crate) fixed_cosets: Vec<Polynomial<F, ExtendedLagrangeCoeff>>,
    pub(crate) permutation: permutation::ProvingKey<F>,
    pub(crate) ev: Evaluator<F>,
}

impl<F: WithSmallOrderMulGroup<3>, CS: PolynomialCommitmentScheme<F>> ProvingKey<F, CS>
where
    F: FromUniformBytes<64>,
{
    /// Get the underlying [`VerifyingKey`].
    pub fn get_vk(&self) -> &VerifyingKey<F, CS> {
        &self.vk
    }

    /// Gets the total number of bytes in the serialization of `self`
    pub fn bytes_length(&self, format: SerdeFormat) -> usize {
        self.vk.bytes_length(format)
            + 12 // bytes used for encoding the length(u32) of "l0", "l_last" & "l_active_row" polys
            + polynomial_slice_byte_length(&self.fixed_values)
            + self.permutation.bytes_length()
    }
}

impl<F: WithSmallOrderMulGroup<3>, CS: PolynomialCommitmentScheme<F>> ProvingKey<F, CS>
where
    F: PrimeField + FromUniformBytes<64> + SerdeObject,
{
    /// Writes a proving key to a buffer.
    ///
    /// Writes a curve element according to `format`:
    /// - `Processed`: Writes a compressed curve element with coordinates in
    ///   standard form. Writes a field element in standard form, with
    ///   endianness specified by the `PrimeField` implementation.
    /// - Otherwise: Writes an uncompressed curve element with coordinates in
    ///   Montgomery form Writes a field element into raw bytes in its internal
    ///   Montgomery representation, WITHOUT performing the expensive Montgomery
    ///   reduction. Does so by first writing the verifying key and then
    ///   serializing the rest of the data (in the form of field polynomials)
    pub fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
        self.vk.write(writer, format)?;
        write_polynomial_slice(&self.fixed_values, writer)?;
        self.permutation.write(writer)?;
        Ok(())
    }

    /// Reads a proving key from a buffer.
    /// Does so by reading verification key first, and then deserializing the
    /// rest of the file into the remaining proving key data.
    ///
    /// Reads a curve element from the buffer and parses it according to the
    /// `format`:
    /// - `Processed`: Reads a compressed curve element and decompresses it.
    ///   Reads a field element in standard form, with endianness specified by
    ///   the `PrimeField` implementation, and checks that the element is less
    ///   than the modulus.
    /// - `RawBytes`: Reads an uncompressed curve element with coordinates in
    ///   Montgomery form. Checks that field elements are less than modulus, and
    ///   then checks that the point is on the curve.
    /// - `RawBytesUnchecked`: Reads an uncompressed curve element with
    ///   coordinates in Montgomery form; does not perform any checks
    pub fn read<R: io::Read, ConcreteCircuit: Circuit<F>>(
        reader: &mut R,
        format: SerdeFormat,
        #[cfg(feature = "circuit-params")] params: ConcreteCircuit::Params,
    ) -> io::Result<Self> {
        let vk = VerifyingKey::<F, CS>::read::<R, ConcreteCircuit>(
            reader,
            format,
            #[cfg(feature = "circuit-params")]
            params,
        )?;
        let [l0, l_last, l_active_row] = compute_lagrange_polys(&vk, &vk.cs);
        let fixed_values = read_polynomial_vec(reader, format)?;
        let fixed_polys: Vec<_> = fixed_values
            .iter()
            .map(|poly| vk.domain.lagrange_to_coeff(poly.clone()))
            .collect();
        let fixed_cosets = fixed_polys
            .iter()
            .map(|poly| vk.domain.coeff_to_extended(poly.clone()))
            .collect();
        let permutation =
            permutation::ProvingKey::read(reader, format, &vk.domain, &vk.cs.permutation)?;
        let ev = Evaluator::new(vk.cs());
        Ok(Self {
            vk,
            l0,
            l_last,
            l_active_row,
            fixed_values,
            fixed_polys,
            fixed_cosets,
            permutation,
            ev,
        })
    }

    /// Writes a proving key to a vector of bytes using [`Self::write`].
    pub fn to_bytes(&self, format: SerdeFormat) -> Vec<u8> {
        let mut bytes = Vec::<u8>::with_capacity(self.bytes_length(format));
        Self::write(self, &mut bytes, format).expect("Writing to vector should not fail");
        bytes
    }

    /// Reads a proving key from a slice of bytes using [`Self::read`].
    pub fn from_bytes<ConcreteCircuit: Circuit<F>>(
        mut bytes: &[u8],
        format: SerdeFormat,
        #[cfg(feature = "circuit-params")] params: ConcreteCircuit::Params,
    ) -> io::Result<Self> {
        Self::read::<_, ConcreteCircuit>(
            &mut bytes,
            format,
            #[cfg(feature = "circuit-params")]
            params,
        )
    }
}

impl<F: PrimeField, CS: PolynomialCommitmentScheme<F>> VerifyingKey<F, CS> {
    /// Get the underlying [`EvaluationDomain`].
    pub fn get_domain(&self) -> &EvaluationDomain<F> {
        &self.domain
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn evaluate_identities<'a, F, CS>(
    vk: &'a VerifyingKey<F, CS>,
    fixed_evals: &'a [F],
    instance_evals: &'a [Vec<F>],
    advice_evals: &'a [Vec<F>],
    permutation_evals: &'a [permutation::verifier::Evaluated<F, CS>],
    lookup_evals: &'a [Vec<lookup::verifier::Evaluated<F, CS>>],
    trashcan_evals: &'a [Vec<trash::verifier::Evaluated<F, CS>>],
    permutations_common: &'a CommonEvaluated<F>,
    x: F,
    xn: F,
    beta: F,
    gamma: F,
    theta: F,
    trash_challenge: F,
    challenges: &'a [F],
    y: F,
    vanishing: PartiallyEvaluated<F, CS>,
) -> Evaluated<F, CS>
where
    F: WithSmallOrderMulGroup<3> + FromUniformBytes<64>,
    CS: PolynomialCommitmentScheme<F>,
{
    let blinding_factors = vk.cs.blinding_factors();
    let l_evals = vk.domain.l_i_range(x, xn, (-((blinding_factors + 1) as i32))..=0);
    assert_eq!(l_evals.len(), 2 + blinding_factors);
    let l_last = l_evals[0];
    let l_blind: F =
        l_evals[1..(1 + blinding_factors)].iter().fold(F::ZERO, |acc, eval| acc + eval);
    let l_0 = l_evals[1 + blinding_factors];

    // Compute the expected value of h(x)
    let expressions = advice_evals
        .iter()
        .zip(instance_evals.iter())
        .zip(permutation_evals.iter())
        .zip(lookup_evals.iter())
        .zip(trashcan_evals.iter())
        .flat_map(
            |((((advice_evals, instance_evals), permutation), lookups), trash)| {
                let challenges = &challenges;
                let fixed_evals = &fixed_evals;
                iter::empty()
                    // Evaluate the circuit using the custom gates provided
                    .chain(vk.cs.gates.iter().flat_map(move |gate| {
                        gate.polynomials().iter().map(move |poly| {
                            poly.evaluate(
                                &|scalar| scalar,
                                &|_| panic!("virtual selectors are removed during optimization"),
                                &|query| fixed_evals[query.index.unwrap()],
                                &|query| advice_evals[query.index.unwrap()],
                                &|query| instance_evals[query.index.unwrap()],
                                &|challenge| challenges[challenge.index()],
                                &|a| -a,
                                &|a, b| a + &b,
                                &|a, b| a * &b,
                                &|a, scalar| a * &scalar,
                            )
                        })
                    }))
                    .chain(expressions(
                        &permutation.sets,
                        vk,
                        &vk.cs.permutation,
                        permutations_common,
                        advice_evals,
                        fixed_evals,
                        instance_evals,
                        l_0,
                        l_last,
                        l_blind,
                        beta,
                        gamma,
                        x,
                    ))
                    .chain(lookups.iter().zip(vk.cs.lookups.iter()).flat_map(
                        move |(p, argument)| {
                            p.evaluated.expressions(
                                l_0,
                                l_last,
                                l_blind,
                                argument,
                                theta,
                                beta,
                                gamma,
                                advice_evals,
                                fixed_evals,
                                instance_evals,
                                challenges,
                            )
                        },
                    ))
                    .chain(trash.iter().zip(vk.cs.trashcans.iter()).flat_map(
                        move |(p, argument)| {
                            p.evaluated.expressions(
                                argument,
                                trash_challenge,
                                advice_evals,
                                fixed_evals,
                                instance_evals,
                                challenges,
                            )
                        },
                    ))
            },
        );

    vanishing.verify(expressions, y, xn)
}
```