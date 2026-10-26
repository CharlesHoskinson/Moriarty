# Midnight 0.8.3 internal prover benchmark
> Source: https://docs.rs/crate/midnight-proofs/0.8.3/source/src/plonk/bench/prover.rs
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
* [Source](/crate/midnight-proofs/0.8.3/source/src/plonk/bench/prover.rs)
* [Builds](/crate/midnight-proofs/0.8.3/builds)
* [Feature flags](/crate/midnight-proofs/0.8.3/features)

* Hide files
* [..](../)
* [mod.rs](./mod.rs)
* [prover.rs](./prover.rs)

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
581
582
583
584
585
586
587
588
589
590
591
592
593
594
595
596
597
598
599
600
601
602
603
604
605
606
607
608
609
610
611
612
613
614
615
616
617
618
619
620
621
622
623
624
```

```
//! Benchmarking utilities for the PLONK prover.

use std::hash::Hash;

use criterion::BenchmarkGroup;
use ff::{FromUniformBytes, WithSmallOrderMulGroup};
use rand_core::{CryptoRng, RngCore};

use crate::{
    plonk::{
        circuit::Circuit,
        lookup, permutation,
        prover::{
            compute_instances, compute_nu_poly, compute_queries, parse_advices,
            write_evals_to_transcript,
        },
        traces::ProverTrace,
        trash, vanishing, Error, ProvingKey,
    },
    poly::commitment::PolynomialCommitmentScheme,
    transcript::{Hashable, Sampleable, Transcript},
};

/// This computes a proof trace for the provided `circuits` when given the
/// public parameters `params` and the proving key [`ProvingKey`] that was
/// generated previously for the same circuit. The provided `instances`
/// are zero-padded internally.
///
/// The trace can then be used to finalise proofs, or to fold them.
///
/// Benchmarks individual internal steps using the provided `group`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_trace<
    F,
    CS: PolynomialCommitmentScheme<F>,
    T: Transcript,
    ConcreteCircuit: Circuit<F>,
>(
    params: &CS::Parameters,
    pk: &ProvingKey<F, CS>,
    circuits: &[ConcreteCircuit],
    // The prover needs to get all instances in non-committed form. However,
    // the first `nb_committed_instances` instance columns are dedicated for
    // instances that the verifier receives in committed form.
    #[cfg(feature = "committed-instances")] nb_committed_instances: usize,
    instances: &[&[&[F]]],
    transcript: &mut T,
    rng: &mut (impl RngCore + CryptoRng),
    group: &mut BenchmarkGroup<criterion::measurement::WallTime>,
) -> Result<ProverTrace<F>, Error>
where
    CS::Commitment: Hashable<T::Hash>,
    F: WithSmallOrderMulGroup<3>
        + Sampleable<T::Hash>
        + Hashable<T::Hash>
        + Hash
        + Ord
        + FromUniformBytes<64>,
{
    #[cfg(not(feature = "committed-instances"))]
    let nb_committed_instances: usize = 0;

    if circuits.len() != instances.len() {
        return Err(Error::InvalidInstances);
    }

    for instances in instances.iter() {
        if instances.len() != pk.vk.cs.num_instance_columns
            || instances.len() < nb_committed_instances
        {
            return Err(Error::InvalidInstances);
        }
    }

    // Hash verification key into transcript
    group.bench_function("Hash VK", |b| {
        b.iter_batched(
            || transcript.clone(),
            |mut t| {
                let _ = pk.vk.hash_into(&mut t);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    pk.vk.hash_into(transcript)?;

    let domain = &pk.vk.domain;

    let instance = {
        let instances_clone = instances.to_vec();
        group.bench_function("Compute instances", |b| {
            b.iter_batched(
                || (transcript.clone(), instances_clone.clone()),
                |(mut t, inst)| {
                    let _ = compute_instances::<F, CS, T>(
                        params,
                        pk,
                        &inst,
                        nb_committed_instances,
                        &mut t,
                    );
                },
                criterion::BatchSize::SmallInput,
            )
        });
        compute_instances(params, pk, instances, nb_committed_instances, transcript)?
    };

    let (advice, challenges) = {
        group.bench_function("Parse advices", |b| {
            b.iter_batched(
                || transcript.clone(),
                |mut t| {
                    let _ = parse_advices::<F, CS, ConcreteCircuit, T>(
                        params, pk, circuits, instances, &mut t, rng,
                    );
                },
                criterion::BatchSize::LargeInput,
            )
        });
        parse_advices(params, pk, circuits, instances, transcript, rng)?
    };

    // Sample theta challenge for keeping lookup columns linearly independent
    let theta: F = transcript.squeeze_challenge();

    // Construct and commit to permuted lookup columns
    let lookups: Vec<Vec<lookup::prover::Permuted<F>>> = {
        group.bench_function("Commit lookup permuted", |b| {
            b.iter_batched(
                || (transcript.clone(), instance.clone(), advice.clone()),
                |(mut t, inst, adv)| {
                    let _: Result<Vec<Vec<_>>, _> = inst
                        .iter()
                        .zip(adv.iter())
                        .map(|(instance, advice)| -> Result<Vec<_>, Error> {
                            pk.vk
                                .cs
                                .lookups
                                .iter()
                                .map(|lookup| {
                                    lookup.commit_permuted(
                                        pk,
                                        params,
                                        domain,
                                        theta,
                                        &advice.advice_polys,
                                        &pk.fixed_values,
                                        &instance.instance_values,
                                        &challenges,
                                        rng,
                                        &mut t,
                                    )
                                })
                                .collect()
                        })
                        .collect();
                },
                criterion::BatchSize::LargeInput,
            )
        });
        instance
            .iter()
            .zip(advice.iter())
            .map(|(instance, advice)| -> Result<Vec<_>, Error> {
                // Construct and commit to permuted values for each lookup
                pk.vk
                    .cs
                    .lookups
                    .iter()
                    .map(|lookup| {
                        lookup.commit_permuted(
                            pk,
                            params,
                            domain,
                            theta,
                            &advice.advice_polys,
                            &pk.fixed_values,
                            &instance.instance_values,
                            &challenges,
                            rng,
                            transcript,
                        )
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    // Sample beta challenge
    let beta: F = transcript.squeeze_challenge();

    // Sample gamma challenge
    let gamma: F = transcript.squeeze_challenge();

    // Commit to permutations
    let permutations: Vec<permutation::prover::Committed<F>> = {
        group.bench_function("Commit permutations", |b| {
            b.iter_batched(
                || (transcript.clone(), instance.clone(), advice.clone()),
                |(mut t, inst, adv)| {
                    let _: Result<Vec<_>, _> = inst
                        .iter()
                        .zip(adv.iter())
                        .map(|(instance, advice)| {
                            pk.vk.cs.permutation.commit(
                                params,
                                pk,
                                &pk.permutation,
                                &advice.advice_polys,
                                &pk.fixed_values,
                                &instance.instance_values,
                                beta,
                                gamma,
                                rng,
                                &mut t,
                            )
                        })
                        .collect();
                },
                criterion::BatchSize::LargeInput,
            )
        });
        instance
            .iter()
            .zip(advice.iter())
            .map(|(instance, advice)| {
                pk.vk.cs.permutation.commit(
                    params,
                    pk,
                    &pk.permutation,
                    &advice.advice_polys,
                    &pk.fixed_values,
                    &instance.instance_values,
                    beta,
                    gamma,
                    rng,
                    transcript,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    // Construct and commit to lookup product polynomials
    let lookups: Vec<Vec<lookup::prover::Committed<F>>> = {
        group.bench_function("Commit lookup products", |b| {
            b.iter_batched(
                || (transcript.clone(), lookups.clone()),
                |(mut t, lkps)| {
                    let _: Result<Vec<Vec<_>>, _> = lkps
                        .into_iter()
                        .map(|lookups| -> Result<Vec<_>, _> {
                            lookups
                                .into_iter()
                                .map(|lookup| {
                                    lookup.commit_product(pk, params, beta, gamma, rng, &mut t)
                                })
                                .collect::<Result<Vec<_>, _>>()
                        })
                        .collect();
                },
                criterion::BatchSize::LargeInput,
            )
        });
        lookups
            .into_iter()
            .map(|lookups| -> Result<Vec<_>, _> {
                // Construct and commit to products for each lookup
                lookups
                    .into_iter()
                    .map(|lookup| lookup.commit_product(pk, params, beta, gamma, rng, transcript))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    // Trash argument
    let trash_challenge: F = transcript.squeeze_challenge();

    let trashcans: Vec<Vec<trash::prover::Committed<F>>> = {
        group.bench_function("Commit trash arguments", |b| {
            b.iter_batched(
                || (transcript.clone(), instance.clone(), advice.clone()),
                |(mut t, inst, adv)| {
                    let _: Result<Vec<Vec<_>>, _> = inst
                        .iter()
                        .zip(adv.iter())
                        .map(|(instance, advice)| -> Result<Vec<_>, Error> {
                            pk.vk
                                .cs
                                .trashcans
                                .iter()
                                .map(|trash| {
                                    trash.commit::<CS, _>(
                                        params,
                                        domain,
                                        trash_challenge,
                                        &advice.advice_polys,
                                        &pk.fixed_values,
                                        &instance.instance_values,
                                        &challenges,
                                        &mut t,
                                    )
                                })
                                .collect()
                        })
                        .collect();
                },
                criterion::BatchSize::LargeInput,
            )
        });
        instance
            .iter()
            .zip(advice.iter())
            .map(|(instance, advice)| -> Result<Vec<_>, Error> {
                pk.vk
                    .cs
                    .trashcans
                    .iter()
                    .map(|trash| {
                        trash.commit::<CS, _>(
                            params,
                            domain,
                            trash_challenge,
                            &advice.advice_polys,
                            &pk.fixed_values,
                            &instance.instance_values,
                            &challenges,
                            transcript,
                        )
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    // Commit to the vanishing argument's random polynomial for blinding h(x_3)
    group.bench_function("Commit vanishing random poly", |b| {
        b.iter_batched(
            || transcript.clone(),
            |mut t| {
                let _ = vanishing::Argument::<F, CS>::commit(params, domain, rng, &mut t);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    let vanishing = vanishing::Argument::<F, CS>::commit(params, domain, rng, transcript)?;

    // Obtain challenge for keeping all separate gates linearly independent
    let y: F = transcript.squeeze_challenge();

    let (instance_polys, instance_values) =
        instance.into_iter().map(|i| (i.instance_polys, i.instance_values)).unzip();

    let advice_polys = advice
        .into_iter()
        .map(|a| {
            a.advice_polys
                .into_iter()
                .map(|p| domain.lagrange_to_coeff(p))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(ProverTrace {
        advice_polys,
        instance_polys,
        instance_values,
        vanishing,
        lookups,
        trashcans,
        permutations,
        challenges,
        beta,
        gamma,
        theta,
        trash_challenge,
        y,
    })
}

/// This takes the computed trace of a set of witnesses and creates a proof
/// for the provided `circuit` when given the public
/// parameters `params` and the proving key [`ProvingKey`] that was
/// generated previously for the same circuit. The provided `instances`
/// are zero-padded internally.
///
/// Benchmarks individual internal steps using the provided `group`.
pub(crate) fn finalise_proof<'a, F, CS: PolynomialCommitmentScheme<F>, T: Transcript>(
    params: &'a CS::Parameters,
    pk: &'a ProvingKey<F, CS>,
    // The prover needs to get all instances in non-committed form. However,
    // the first `nb_committed_instances` instance columns are dedicated for
    // instances that the verifier receives in committed form.
    #[cfg(feature = "committed-instances")] nb_committed_instances: usize,
    trace: ProverTrace<F>,
    transcript: &mut T,
    rng: &mut (impl RngCore + CryptoRng),
    group: &mut BenchmarkGroup<criterion::measurement::WallTime>,
) -> Result<(), Error>
where
    CS::Commitment: Hashable<T::Hash>,
    F: WithSmallOrderMulGroup<3>
        + Sampleable<T::Hash>
        + Hashable<T::Hash>
        + Hash
        + Ord
        + FromUniformBytes<64>,
{
    #[cfg(not(feature = "committed-instances"))]
    let nb_committed_instances: usize = 0;

    let domain = pk.get_vk().get_domain();

    let h_poly = {
        group.bench_function("Compute H poly", |b| {
            b.iter(|| {
                let _ = compute_nu_poly(pk, &trace);
            })
        });
        compute_nu_poly(pk, &trace)
    };

    let ProverTrace {
        advice_polys,
        instance_polys,
        lookups,
        trashcans,
        permutations,
        vanishing,
        ..
    } = trace;

    // Construct the vanishing argument's h(X) commitments
    let vanishing = {
        group.bench_function("Construct vanishing commitments", |b| {
            b.iter_batched(
                || (transcript.clone(), h_poly.clone(), vanishing.clone()),
                |(mut t, h, v)| {
                    let _ = v.construct::<CS, T>(params, domain, h, &mut t, rng);
                },
                criterion::BatchSize::PerIteration,
            )
        });
        vanishing.construct::<CS, T>(params, domain, h_poly, transcript, rng)?
    };

    let x: F = transcript.squeeze_challenge();

    group.bench_function("Write evals to transcript", |b| {
        b.iter_batched(
            || transcript.clone(),
            |mut t| {
                let _ = write_evals_to_transcript(
                    pk,
                    nb_committed_instances,
                    &instance_polys,
                    &advice_polys,
                    x,
                    &mut t,
                );
            },
            criterion::BatchSize::SmallInput,
        )
    });
    write_evals_to_transcript(
        pk,
        nb_committed_instances,
        &instance_polys,
        &advice_polys,
        x,
        transcript,
    )?;

    let vanishing = {
        group.bench_function("Evaluate vanishing", |b| {
            b.iter_batched(
                || (transcript.clone(), vanishing.clone()),
                |(mut t, v)| {
                    let _ = v.evaluate(x, domain, &mut t);
                },
                criterion::BatchSize::PerIteration,
            )
        });
        vanishing.evaluate(x, domain, transcript)?
    };

    // Evaluate common permutation data
    group.bench_function("Evaluate permutation data", |b| {
        b.iter_batched(
            || transcript.clone(),
            |mut t| {
                let _ = pk.permutation.evaluate(x, &mut t);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    pk.permutation.evaluate(x, transcript)?;

    // Evaluate the permutations, if any, at omega^i x.
    let permutations: Vec<permutation::prover::Evaluated<F>> = permutations
        .into_iter()
        .map(|permutation| -> Result<_, _> { permutation.evaluate(pk, x, transcript) })
        .collect::<Result<Vec<_>, _>>()?;

    // Evaluate the lookups, if any, at omega^i x.
    let lookups: Vec<Vec<lookup::prover::Evaluated<F>>> = lookups
        .into_iter()
        .map(|lookups| -> Result<Vec<_>, _> {
            lookups
                .into_iter()
                .map(|p| p.evaluate(pk, x, transcript))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Evaluate the trashcans, if any, at x.
    let trashcans: Vec<Vec<trash::prover::Evaluated<F>>> = trashcans
        .into_iter()
        .map(|trash| -> Result<Vec<_>, _> {
            trash
                .into_iter()
                .map(|p| p.evaluate(x, transcript))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let queries = {
        group.bench_function("Compute queries", |b| {
            b.iter(|| {
                let _ = compute_queries(
                    pk,
                    nb_committed_instances,
                    &instance_polys,
                    &advice_polys,
                    &permutations,
                    &lookups,
                    &trashcans,
                    &vanishing,
                    x,
                );
            })
        });
        compute_queries(
            pk,
            nb_committed_instances,
            &instance_polys,
            &advice_polys,
            &permutations,
            &lookups,
            &trashcans,
            &vanishing,
            x,
        )
    };

    group.bench_function("Multi open argument", |b| {
        b.iter_batched(
            || (transcript.clone(), queries.clone()),
            |(mut t, q)| {
                let _ = CS::multi_open(params, &q, &mut t);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    CS::multi_open(params, &queries, transcript).map_err(|_| Error::ConstraintSystemFailure)
}

/// Benchmarked version of proof creation that measures each internal step.
///
/// This function simply calls `compute_trace` and `finalise_proof` with the
/// provided benchmark group, which causes those functions to benchmark their
/// internal steps.
#[allow(clippy::too_many_arguments)]
pub fn benchmark_create_proof<
    F,
    CS: PolynomialCommitmentScheme<F>,
    T: Transcript,
    ConcreteCircuit: Circuit<F>,
>(
    params: &CS::Parameters,
    pk: &ProvingKey<F, CS>,
    circuits: &[ConcreteCircuit],
    #[cfg(feature = "committed-instances")] nb_committed_instances: usize,
    instances: &[&[&[F]]],
    transcript: &mut T,
    rng: &mut (impl RngCore + CryptoRng),
    group: &mut BenchmarkGroup<criterion::measurement::WallTime>,
) -> Result<(), Error>
where
    CS::Commitment: Hashable<T::Hash>,
    F: WithSmallOrderMulGroup<3>
        + Sampleable<T::Hash>
        + Hashable<T::Hash>
        + Hash
        + Ord
        + FromUniformBytes<64>,
{
    #[cfg(not(feature = "committed-instances"))]
    let nb_committed_instances: usize = 0;

    let trace = compute_trace(
        params,
        pk,
        circuits,
        #[cfg(feature = "committed-instances")]
        nb_committed_instances,
        instances,
        transcript,
        rng,
        group,
    )?;

    finalise_proof(
        params,
        pk,
        #[cfg(feature = "committed-instances")]
        nb_committed_instances,
        trace,
        transcript,
        rng,
        group,
    )
}
```