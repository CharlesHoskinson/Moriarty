# Midnight proofs 0.8.3 key generation
> Source: https://docs.rs/crate/midnight-proofs/0.8.3/source/src/plonk/keygen.rs
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
* [Source](/crate/midnight-proofs/0.8.3/source/src/plonk/keygen.rs)
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
```

```
#![allow(clippy::int_plus_one)]

use std::ops::Range;

use ff::{Field, FromUniformBytes, WithSmallOrderMulGroup};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::{
    circuit::{
        Advice, Any, Assignment, Circuit, Column, ConstraintSystem, Fixed, FloorPlanner, Instance,
        Selector,
    },
    evaluation::Evaluator,
    permutation, Challenge, Error, LagrangeCoeff, Polynomial, ProvingKey, VerifyingKey,
};
use crate::{
    circuit::Value,
    dev::cost_model::cost_model_options,
    poly::{
        batch_invert_rational,
        commitment::{Params, PolynomialCommitmentScheme},
        EvaluationDomain, ExtendedLagrangeCoeff,
    },
    utils::{arithmetic::parallelize, rational::Rational},
};

pub(crate) fn create_domain<F, ConcreteCircuit>(
    k: u32,
    #[cfg(feature = "circuit-params")] params: ConcreteCircuit::Params,
) -> (
    EvaluationDomain<F>,
    ConstraintSystem<F>,
    ConcreteCircuit::Config,
)
where
    F: WithSmallOrderMulGroup<3>,
    ConcreteCircuit: Circuit<F>,
{
    let mut cs = ConstraintSystem::default();
    #[cfg(feature = "circuit-params")]
    let config = ConcreteCircuit::configure_with_params(&mut cs, params);
    #[cfg(not(feature = "circuit-params"))]
    let config = ConcreteCircuit::configure(&mut cs);

    let degree = cs.degree();

    let domain = EvaluationDomain::new(degree as u32, k);

    (domain, cs, config)
}

/// Assembly to be used in circuit synthesis.
#[derive(Debug)]
struct Assembly<F: Field> {
    k: u32,
    fixed: Vec<Polynomial<Rational<F>, LagrangeCoeff>>,
    permutation: permutation::keygen::Assembly,
    selectors: Vec<Vec<bool>>,
    // A range of available rows for assignment and copies.
    usable_rows: Range<usize>,
    _marker: std::marker::PhantomData<F>,
}

impl<F: Field> Assignment<F> for Assembly<F> {
    fn enter_region<NR, N>(&mut self, _: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        // Do nothing; we don't care about regions in this context.
    }

    fn exit_region(&mut self) {
        // Do nothing; we don't care about regions in this context.
    }

    fn enable_selector<A, AR>(&mut self, _: A, selector: &Selector, row: usize) -> Result<(), Error>
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        if !self.usable_rows.contains(&row) {
            return Err(Error::not_enough_rows_available(self.k));
        }

        self.selectors[selector.0][row] = true;

        Ok(())
    }

    fn query_instance(&self, _: Column<Instance>, row: usize) -> Result<Value<F>, Error> {
        if !self.usable_rows.contains(&row) {
            return Err(Error::not_enough_rows_available(self.k));
        }

        // There is no instance in this context.
        Ok(Value::unknown())
    }

    fn assign_advice<V, VR, A, AR>(
        &mut self,
        _: A,
        _: Column<Advice>,
        _: usize,
        _: V,
    ) -> Result<(), Error>
    where
        V: FnOnce() -> Value<VR>,
        VR: Into<Rational<F>>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        // We only care about fixed columns here
        Ok(())
    }

    fn assign_fixed<V, VR, A, AR>(
        &mut self,
        _: A,
        column: Column<Fixed>,
        row: usize,
        to: V,
    ) -> Result<(), Error>
    where
        V: FnOnce() -> Value<VR>,
        VR: Into<Rational<F>>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        if !self.usable_rows.contains(&row) {
            return Err(Error::not_enough_rows_available(self.k));
        }

        *self
            .fixed
            .get_mut(column.index())
            .and_then(|v| v.get_mut(row))
            .ok_or(Error::BoundsFailure)? = to().into_field().assign()?;

        Ok(())
    }

    fn copy(
        &mut self,
        left_column: Column<Any>,
        left_row: usize,
        right_column: Column<Any>,
        right_row: usize,
    ) -> Result<(), Error> {
        if !self.usable_rows.contains(&left_row) || !self.usable_rows.contains(&right_row) {
            return Err(Error::not_enough_rows_available(self.k));
        }

        self.permutation.copy(left_column, left_row, right_column, right_row)
    }

    fn fill_from_row(
        &mut self,
        column: Column<Fixed>,
        from_row: usize,
        to: Value<Rational<F>>,
    ) -> Result<(), Error> {
        if !self.usable_rows.contains(&from_row) {
            return Err(Error::not_enough_rows_available(self.k));
        }

        let col = self.fixed.get_mut(column.index()).ok_or(Error::BoundsFailure)?;

        let filler = to.assign()?;
        for row in self.usable_rows.clone().skip(from_row) {
            col[row] = filler;
        }

        Ok(())
    }

    fn get_challenge(&self, _: Challenge) -> Value<F> {
        Value::unknown()
    }

    fn annotate_column<A, AR>(&mut self, _annotation: A, _column: Column<Any>)
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        // Do nothing
    }

    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        // Do nothing; we don't care about namespaces in this context.
    }

    fn pop_namespace(&mut self, _: Option<String>) {
        // Do nothing; we don't care about namespaces in this context.
    }
}

/// Compute the minimal `k` to compute a circuit.
pub fn k_from_circuit<F: Ord + Field + FromUniformBytes<64>, C: Circuit<F>>(circuit: &C) -> u32 {
    cost_model_options(circuit).min_k as u32
}

/// Generates a `VerifyingKey` from a `Circuit` instance.
///
/// Automatically determines the smallest `k` required for the given circuit
/// and adjusts the received parameters to match the circuit's size.
/// Use `keygen_vk_with_k` to specify a custom `k` value.
pub fn keygen_vk<F, CS, ConcreteCircuit>(
    params: &CS::Parameters,
    circuit: &ConcreteCircuit,
) -> Result<VerifyingKey<F, CS>, Error>
where
    F: WithSmallOrderMulGroup<3> + FromUniformBytes<64> + Ord,
    CS: PolynomialCommitmentScheme<F>,
    ConcreteCircuit: Circuit<F>,
{
    let k = k_from_circuit(circuit);

    if params.max_k() != k {
        return Err(Error::SrsError(params.max_k() as usize, k as usize));
    }

    keygen_vk_with_k(params, circuit, k)
}

/// Generate a `VerifyingKey` from an instance of `Circuit`.
pub fn keygen_vk_with_k<F, CS, ConcreteCircuit>(
    params: &CS::Parameters,
    circuit: &ConcreteCircuit,
    k: u32,
) -> Result<VerifyingKey<F, CS>, Error>
where
    F: WithSmallOrderMulGroup<3> + FromUniformBytes<64> + Ord,
    CS: PolynomialCommitmentScheme<F>,
    ConcreteCircuit: Circuit<F>,
{
    if params.max_k() < k {
        return Err(Error::NotEnoughRowsAvailable {
            current_k: params.max_k(),
        });
    }

    let (domain, cs, config) = create_domain::<F, ConcreteCircuit>(
        k,
        #[cfg(feature = "circuit-params")]
        circuit.params(),
    );

    if (domain.n as usize) < cs.minimum_rows() {
        return Err(Error::not_enough_rows_available(domain.k()));
    }

    let mut assembly: Assembly<F> = Assembly {
        k: domain.k(),
        fixed: vec![domain.empty_lagrange_rational(); cs.num_fixed_columns],
        permutation: permutation::keygen::Assembly::new(domain.n as usize, &cs.permutation),
        selectors: vec![vec![false; domain.n as usize]; cs.num_selectors],
        usable_rows: 0..domain.n as usize - (cs.blinding_factors() + 1),
        _marker: std::marker::PhantomData,
    };

    // Synthesize the circuit to obtain URS
    ConcreteCircuit::FloorPlanner::synthesize(
        &mut assembly,
        circuit,
        config,
        cs.constants.clone(),
    )?;

    let mut fixed = batch_invert_rational(assembly.fixed);
    // After this, the ConstraintSystem should not have any selectors: `verify` does
    // not need them, and `keygen_pk` regenerates `cs` from scratch anyways.
    let selectors = std::mem::take(&mut assembly.selectors);
    let (cs, selector_polys) = cs.directly_convert_selectors_to_fixed(selectors);
    fixed.extend(selector_polys.into_iter().map(|poly| domain.lagrange_from_vec(poly)));

    let permutation_vk = assembly.permutation.build_vk(params, &domain, &cs.permutation);

    let fixed_commitments = fixed.iter().map(|poly| CS::commit_lagrange(params, poly)).collect();

    Ok(VerifyingKey::from_parts(
        domain,
        fixed_commitments,
        permutation_vk,
        cs,
    ))
}

/// Generate a `ProvingKey` from a `VerifyingKey` and an instance of `Circuit`.
pub fn keygen_pk<F, CS, ConcreteCircuit>(
    vk: VerifyingKey<F, CS>,
    circuit: &ConcreteCircuit,
) -> Result<ProvingKey<F, CS>, Error>
where
    F: WithSmallOrderMulGroup<3>,
    CS: PolynomialCommitmentScheme<F>,
    ConcreteCircuit: Circuit<F>,
{
    let mut cs = ConstraintSystem::default();
    #[cfg(feature = "circuit-params")]
    let config = ConcreteCircuit::configure_with_params(&mut cs, circuit.params());
    #[cfg(not(feature = "circuit-params"))]
    let config = ConcreteCircuit::configure(&mut cs);

    let cs = cs;

    let n = vk.domain.n as usize;
    let mut assembly: Assembly<F> = Assembly {
        k: vk.domain.k(),
        fixed: vec![vk.domain.empty_lagrange_rational(); cs.num_fixed_columns],
        permutation: permutation::keygen::Assembly::new(n, &cs.permutation),
        selectors: vec![vec![false; n]; cs.num_selectors],
        usable_rows: 0..n - (cs.blinding_factors() + 1),
        _marker: std::marker::PhantomData,
    };

    // Synthesize the circuit to obtain URS
    ConcreteCircuit::FloorPlanner::synthesize(
        &mut assembly,
        circuit,
        config,
        cs.constants.clone(),
    )?;

    let mut fixed = batch_invert_rational(assembly.fixed);
    let (cs, selector_polys) = cs.directly_convert_selectors_to_fixed(assembly.selectors);
    fixed.extend(selector_polys.into_iter().map(|poly| vk.domain.lagrange_from_vec(poly)));

    let fixed_polys: Vec<_> =
        fixed.par_iter().map(|poly| vk.domain.lagrange_to_coeff(poly.clone())).collect();

    let fixed_cosets = fixed_polys
        .par_iter()
        .map(|poly| vk.domain.coeff_to_extended(poly.clone()))
        .collect();

    let permutation_pk = assembly.permutation.build_pk::<F>(&vk.domain, &cs.permutation);

    let [l0, l_last, l_active_row] = compute_lagrange_polys(&vk, &cs);
    // Compute the optimized evaluation data structure
    let ev = Evaluator::new(&vk.cs);
    Ok(ProvingKey {
        vk,
        l0,
        l_last,
        l_active_row,
        fixed_values: fixed,
        fixed_polys,
        fixed_cosets,
        permutation: permutation_pk,
        ev,
    })
}

pub(crate) fn compute_lagrange_polys<F, CS>(
    vk: &VerifyingKey<F, CS>,
    cs: &ConstraintSystem<F>,
) -> [Polynomial<F, ExtendedLagrangeCoeff>; 3]
where
    F: WithSmallOrderMulGroup<3>,
    CS: PolynomialCommitmentScheme<F>,
{
    // Compute l_0(X)
    // TODO: this can be done more efficiently
    let mut l0 = vk.domain.empty_lagrange();
    l0[0] = F::ONE;
    let l0 = vk.domain.lagrange_to_coeff(l0);
    let l0 = vk.domain.coeff_to_extended(l0);

    // Compute l_blind(X) which evaluates to 1 for each blinding factor row
    // and 0 otherwise over the domain.
    let mut l_blind = vk.domain.empty_lagrange();
    for evaluation in l_blind[..].iter_mut().rev().take(cs.blinding_factors()) {
        *evaluation = F::ONE;
    }
    let l_blind = vk.domain.lagrange_to_coeff(l_blind);
    let l_blind = vk.domain.coeff_to_extended(l_blind);

    // Compute l_last(X) which evaluates to 1 on the first inactive row (just
    // before the blinding factors) and 0 otherwise over the domain
    let mut l_last = vk.domain.empty_lagrange();
    let n = vk.domain.n as usize;
    l_last[n - cs.blinding_factors() - 1] = F::ONE;
    let l_last = vk.domain.lagrange_to_coeff(l_last);
    let l_last = vk.domain.coeff_to_extended(l_last);

    // Compute l_active_row(X)
    let one = F::ONE;
    let mut l_active_row = vk.domain.empty_extended();
    parallelize(&mut l_active_row, |values, start| {
        for (i, value) in values.iter_mut().enumerate() {
            let idx = i + start;
            *value = one - (l_last[idx] + l_blind[idx]);
        }
    });

    [l0, l_last, l_active_row]
}
```