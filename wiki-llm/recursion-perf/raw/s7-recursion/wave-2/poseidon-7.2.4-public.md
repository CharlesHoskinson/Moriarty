# Pinned Midnight Poseidon chip 7.2.4 source
> Source: https://docs.rs/midnight-circuits/7.2.4/src/midnight_circuits/hash/poseidon/poseidon_chip.rs.html
> Collected: 2026-10-26
> Published: Unknown

poseidon\_chip.rs - source

[Docs.rs](/)

* [midnight-circuits-7.2.4](# "Circuit and gadget implementations for Midnight zero-knowledge proofs")

  + midnight-circuits 7.2.4
  + [Docs.rs crate page](/crate/midnight-circuits/7.2.4 "See midnight-circuits in docs.rs")
  + 05 August 2026

  + Links
  + [crates.io](https://crates.io/crates/midnight-circuits "See midnight-circuits in crates.io")
  + [Source](/crate/midnight-circuits/7.2.4/source/ "Browse source of midnight-circuits-7.2.4")

  + Owners
  + [MidnightCI](https://crates.io/users/MidnightCI)

  + Dependencies
  + - [base64 ^0.13.1
      *normal*](/base64/^0.13.1/)
    - [ff ^0.13
      *normal*](/ff/^0.13/)
    - [goldenfile =1.8.0
      *normal*
      *optional*](/goldenfile/=1.8.0/)
    - [group ^0.13
      *normal*](/group/^0.13/)
    - [lazy\_static ^1.5.0
      *normal*](/lazy_static/^1.5.0/)
    - [midnight-curves ^0.3.1
      *normal*](/midnight-curves/^0.3.1/)
    - [midnight-proofs ^0.8.2
      *normal*](/midnight-proofs/^0.8.2/)
    - [num-bigint ^0.4
      *normal*](/num-bigint/^0.4/)
    - [num-integer ^0.1
      *normal*](/num-integer/^0.1/)
    - [num-traits ^0.2
      *normal*](/num-traits/^0.2/)
    - [rand ^0.8
      *normal*](/rand/^0.8/)
    - [rand\_chacha ^0.3
      *normal*
      *optional*](/rand_chacha/^0.3/)
    - [ripemd ^0.2.0
      *normal*](/ripemd/^0.2.0/)
    - [rustc-hash ^2
      *normal*](/rustc-hash/^2/)
    - [serde ^1.0
      *normal*
      *optional*](/serde/^1.0/)
    - [serde\_json ^1
      *normal*
      *optional*](/serde_json/^1/)
    - [sha2 ^0.10.9
      *normal*](/sha2/^0.10.9/)
    - [subtle ^2.6
      *normal*](/subtle/^2.6/)
    - [criterion ^0.7
      *dev*](/criterion/^0.7/)
    - [dhat ^0.3
      *dev*](/dhat/^0.3/)
    - [goldenfile =1.8.0
      *dev*](/goldenfile/=1.8.0/)
    - [itertools ^0.14
      *dev*](/itertools/^0.14/)
    - [num-bigint ^0.4
      *dev*](/num-bigint/^0.4/)
    - [rand\_chacha ^0.3.1
      *dev*](/rand_chacha/^0.3.1/)
    - [serde\_json ^1
      *dev*](/serde_json/^1/)
    - [midnight-curves ^0.3.0
      *normal*](/midnight-curves/^0.3.0/)

  + Versions

  + [**100%**
    of the crate is documented](/crate/midnight-circuits/7.2.4)
* [Platform](#)
  + [x86\_64-unknown-linux-gnu](/crate/midnight-circuits/7.2.4/target-redirect/src/midnight_circuits/hash/poseidon/poseidon_chip.rs.html)
* [Feature flags](/crate/midnight-circuits/7.2.4/features "Browse available feature flags of midnight-circuits-7.2.4")

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

[Skip to main content](#main-content)

Files
-----

midnight\_circuits/hash/poseidon/ poseidon\_chip.rs
===================================================

```
1// This file is part of MIDNIGHT-ZK.
2// Copyright (C) Midnight Foundation
3// SPDX-License-Identifier: Apache-2.0
4// Licensed under the Apache License, Version 2.0 (the "License");
5// You may not use this file except in compliance with the License.
6// You may obtain a copy of the License at
7// http://www.apache.org/licenses/LICENSE-2.0
8// Unless required by applicable law or agreed to in writing, software
9// distributed under the License is distributed on an "AS IS" BASIS,
10// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
11// See the License for the specific language governing permissions and
12// limitations under the License.
13
14use core::array::from_fn;
15use std::iter::once;
16
17use ff::Field;
18use midnight_proofs::{
19    circuit::{Chip, Layouter, Region, Value},
20    plonk::{Advice, Column, ConstraintSystem, Constraints, Error, Expression, Fixed, Selector},
21    poly::Rotation,
22};
23#[cfg(any(test, feature = "testing"))]
24use {crate::testing_utils::FromScratch, midnight_proofs::plonk::Instance};
25
26use super::{
27    constants::{PoseidonField, NB_FULL_ROUNDS, NB_PARTIAL_ROUNDS, RATE, WIDTH},
28    full_round_cpu, partial_round_cpu_for_circuits,
29    round_skips::PreComputedRoundCircuit,
30    NB_POSEIDON_ADVICE_COLS, NB_POSEIDON_FIXED_COLS,
31};
32#[cfg(any(test, feature = "testing"))]
33use crate::field::{
34    native::{NB_ARITH_COLS, NB_ARITH_FIXED_COLS},
35    NativeConfig,
36};
37use crate::{
38    field::NativeChip,
39    instructions::{ArithInstructions, AssignmentInstructions, SpongeInstructions},
40    types::AssignedNative,
41    utils::ComposableChip,
42    CircuitField,
43};
44
45/// Number of times the linear part of the partial rounds is skipped in the
46/// Poseidon chip (0 is the default implementation without skips at all).
47///
48/// Note: The chip configuration will panic if `NB_PARTIAL_ROUNDS` is not
49/// dividable by `1 + NB_SKIPS_CIRCUIT`.
50pub(crate) const NB_SKIPS_CIRCUIT: usize = 5;
51
52// A recurring type representing a set of assigned registers, representing the
53// internal state of Poseidon's computation. Does not account for the additional
54// registers needed in skipped rounds.
55pub(super) type AssignedRegister<F> = [AssignedNative<F>; WIDTH];
56
57/// In-circuit Poseidon state.
58#[derive(Clone, Debug)]
59pub struct AssignedPoseidonState<F: CircuitField> {
60    pub(super) register: AssignedRegister<F>,
61    pub(super) queue: Vec<AssignedNative<F>>,
62    pub(super) squeeze_position: usize,
63    input_len: Option<usize>,
64}
65
66#[derive(Clone, Debug)]
67/// Poseidon configuration setting.
68pub struct PoseidonConfig<F: PoseidonField> {
69    /// Selector for full rounds.
70    q_full_round: Selector,
71
72    /// Selector for optimized partial rounds skipping `1+NB_SKIPS_CIRCUIT`
73    /// rounds.
74    q_partial_round: Selector,
75
76    /// Advice columns, including those potentially needed for optimised
77    /// skipping rounds. The Poseidon circuit (`PoseidonChip::permutation`)
78    /// assumes that the first `WIDTH` columns of `register_cols` are the (first
79    /// `WIDTH`) columns where `native_chip::add_constants_in_region` assigns
80    /// its result. An assertion is checking this assumption in
81    /// `PoseidonChip::permutation` in debug mode.
82    register_cols: [Column<Advice>; NB_POSEIDON_ADVICE_COLS],
83
84    /// Fixed columns, one for each register column (their content will be
85    /// loaded from the precomputed array stored at the field
86    /// `round_constant_opt`).
87    constant_cols: [Column<Fixed>; NB_POSEIDON_FIXED_COLS],
88
89    /// Precomputed data for partial rounds (identities and round constants).
90    pre_computed: PreComputedRoundCircuit<F>,
91}
92
93/// Chip for Poseidon operations.
94#[derive(Clone, Debug)]
95pub struct PoseidonChip<F: PoseidonField> {
96    config: PoseidonConfig<F>,
97    native_chip: NativeChip<F>,
98}
99
100impl<F: PoseidonField> Chip<F> for PoseidonChip<F> {
101    type Config = PoseidonConfig<F>;
102    type Loaded = ();
103
104    fn config(&self) -> &Self::Config {
105        &self.config
106    }
107
108    fn loaded(&self) -> &Self::Loaded {
109        &()
110    }
111}
112
113impl<F: PoseidonField> PoseidonChip<F> {
114    /// Size of Poseidon's register.
115    pub const fn register_size() -> usize {
116        WIDTH
117    }
118
119    /// Hash rate provided by this implementation of Poseidon.
120    pub const fn rate() -> usize {
121        RATE
122    }
123
124    /// Number of full rounds of the Poseidon permutation.
125    pub const fn nb_full_rounds() -> usize {
126        NB_FULL_ROUNDS
127    }
128
129    /// Number of partial rounds of the Poseidon permutation.
130    pub const fn nb_partial_rounds() -> usize {
131        NB_PARTIAL_ROUNDS
132    }
133}
134
135/// Computes the identity of the linear layer of Poseidon's rounds
136/// (multiplication by the MDS matrix and addition of round constants). To save
137/// up the cost of addition, the identity is computed by mutating a variable
138/// initialised as the round-constant argument.
139fn linear_layer<F: PoseidonField>(
140    inputs: [Expression<F>; WIDTH],
141    outputs: [Expression<F>; WIDTH],
142    constants: [Expression<F>; WIDTH],
143) -> [Expression<F>; WIDTH] {
144    let mut ids = constants;
145    for (i, output) in outputs.into_iter().enumerate() {
146        ids[i] = &ids[i] - output;
147        #[allow(clippy::needless_range_loop)]
148        for j in 0..WIDTH {
149            ids[i] = &ids[i] + Expression::Constant(F::MDS[i][j]) * &inputs[j];
150        }
151    }
152    ids
153}
154
155/// Performs an S-box computation.
156pub(crate) fn sbox<F: Field>(x: Expression<F>) -> Expression<F> {
157    x.clone() * x.square().square()
158}
159
160impl<F: PoseidonField> ComposableChip<F> for PoseidonChip<F> {
161    type SharedResources = (
162        [Column<Advice>; NB_POSEIDON_ADVICE_COLS],
163        [Column<Fixed>; NB_POSEIDON_FIXED_COLS],
164    );
165
166    type InstructionDeps = NativeChip<F>;
167
168    fn new(config: &PoseidonConfig<F>, native_chip: &Self::InstructionDeps) -> Self {
169        Self {
170            config: config.clone(),
171            native_chip: native_chip.clone(),
172        }
173    }
174
175    fn configure(
176        meta: &mut ConstraintSystem<F>,
177        shared_res: &Self::SharedResources,
178    ) -> PoseidonConfig<F> {
179        let register_cols = shared_res.0;
180        let constant_cols = shared_res.1;
181
182        let q_full_round = meta.selector();
183        let q_partial_round = meta.complex_selector();
184
185        register_cols[..WIDTH].iter().for_each(|col| meta.enable_equality(*col));
186
187        // Custom full round gate. It focuses on only the first `WIDTH` advice/fixed
188        // columns, and compute the corresponding identity (application of the power-5
189        // S-box, multiplication by the MDS matrix, and addition of the round constants
190        // assigned to the fixed columns).
191        //
192        // Note: These are shifted rounds, i.e., the S-box is first applied, followed
193        // by the linear layer (`linear_layer`). Therefore,
194        // `F::ROUND_CONSTANTS[0]` will be added the the initial state before
195        // applying a permutation, and the last full round will use `[F::ZERO;
196        // WIDTH]` as round constants.
197        meta.create_gate("full_round_gate", |meta| {
198            // We provide hints for computing the S-box on the inputs.
199            // Concretely, for every input, we hint its cube.
200            let inputs_and_hints: [(Expression<F>, Expression<F>); WIDTH] = from_fn(|i| {
201                (
202                    meta.query_advice(register_cols[i], Rotation::cur()),
203                    meta.query_advice(register_cols[i + WIDTH], Rotation::cur()),
204                )
205            });
206            let outputs = from_fn(|i| meta.query_advice(register_cols[i], Rotation::next()));
207            let constants = from_fn(|i| meta.query_fixed(constant_cols[i], Rotation::cur()));
208
209            let sboxed_inputs = inputs_and_hints.clone().map(|(x, x3)| x.square() * x3);
210
211            Constraints::with_selector(
212                q_full_round,
213                [
214                    inputs_and_hints.map(|(x, x3)| x.clone() * x.square() - x3),
215                    linear_layer(sboxed_inputs, outputs, constants),
216                ]
217                .concat(),
218            )
219        });
220
221        // Generation of the optimised round identities, representing a batch of
222        // `1+NB_SKIPS` partial rounds.
223        let pre_computed = PreComputedRoundCircuit::<F>::init();
224        let ids = pre_computed.partial_round_id;
225
226        // A batch of `1+NB_SKIPS` partial gates. Most of the work has been done in the
227        // previous line, with `ids` now storing the expressions that will be used to
228        // represent the core of the gates' polynomial identities.
229        meta.create_gate("partial_round_gate", |meta| {
230            let inputs = register_cols
231                .iter()
232                .map(|col| meta.query_advice(*col, Rotation::cur()))
233                .collect::<Vec<_>>();
234            let round_constants = constant_cols
235                .iter()
236                .map(|col| meta.query_fixed(*col, Rotation::cur()))
237                .collect::<Vec<_>>();
238            let outputs = register_cols[0..WIDTH]
239                .iter()
240                .map(|col| meta.query_advice(*col, Rotation::next()))
241                .collect::<Vec<_>>();
242
243            let constraints = ids.to_expression(&inputs);
244
245            let output_lin_constraints =
246                (0..WIDTH - 1).map(|i| &round_constants[i] - &outputs[i] + &constraints[i]);
247            let input_pow_constraints = (WIDTH - 1..WIDTH + NB_SKIPS_CIRCUIT - 1)
248                .map(|i| &round_constants[i] - &inputs[i + 1] + &constraints[i]);
249            let output_pow_constraint: Expression<F> =
250                &round_constants[WIDTH + NB_SKIPS_CIRCUIT - 1] - &outputs[WIDTH - 1]
251                    + &constraints[WIDTH + NB_SKIPS_CIRCUIT - 1];
252
253            let constraints = output_lin_constraints
254                .chain(input_pow_constraints)
255                .chain(once(output_pow_constraint))
256                .collect::<Vec<_>>();
257            Constraints::with_additive_selector(q_partial_round, constraints)
258        });
259
260        PoseidonConfig {
261            q_full_round,
262            q_partial_round,
263            register_cols,
264            constant_cols,
265            pre_computed,
266        }
267    }
268
269    fn load(&self, _layouter: &mut impl Layouter<F>) -> Result<(), Error> {
270        Ok(())
271    }
272}
273
274impl<F: PoseidonField> PoseidonChip<F> {
275    /// Assign constants in the circuit for a full round.
276    fn assign_constants_full(
277        &self,
278        region: &mut Region<'_, F>,
279        round_index: usize,
280        offset: usize,
281    ) -> Result<(), Error> {
282        let round_constants = if round_index == NB_FULL_ROUNDS + NB_PARTIAL_ROUNDS - 1 {
283            [F::ZERO; WIDTH]
284        } else {
285            F::ROUND_CONSTANTS[round_index + 1]
286        };
287        for (col, constant) in self.config.constant_cols[0..WIDTH].iter().zip(round_constants) {
288            region.assign_fixed(
289                || "load constant for a full round",
290                *col,
291                offset,
292                || Value::known(constant),
293            )?;
294        }
295        Ok(())
296    }
297
298    /// Assign constants in the circuit for a partial round.
299    fn assign_constants_partial(
300        &self,
301        region: &mut Region<'_, F>,
302        round_batch_index: usize,
303        offset: usize,
304    ) -> Result<(), Error> {
305        for (col, constant) in self
306            .config
307            .constant_cols
308            .iter()
309            .zip(&self.config.pre_computed.round_constants[round_batch_index])
310        {
311            region.assign_fixed(
312                || format!("load constant for partial a round with {NB_SKIPS_CIRCUIT} skips"),
313                *col,
314                offset,
315                || Value::known(*constant),
316            )?;
317        }
318        Ok(())
319    }
320
321    /// Operates a full round in-circuit. The variable assignment in the circuit
322    /// is done by calling `full_round_cpu`.
323    ///
324    /// Note: This function does not copy the inputs in the current offset, but
325    /// assumes they were copied there earlier.
326    fn full_round(
327        &self,
328        region: &mut Region<'_, F>,
329        inputs: &mut [AssignedNative<F>; WIDTH],
330        round_index: usize,
331        offset: &mut usize,
332    ) -> Result<(), Error> {
333        self.config.q_full_round.enable(region, *offset)?;
334        self.assign_constants_full(region, round_index, *offset)?;
335
336        // Assign the hints (inputs cubed).
337        for (x, col) in inputs.iter().zip(self.config.register_cols[WIDTH..(2 * WIDTH)].iter()) {
338            region.assign_advice(
339                || "full round hint",
340                *col,
341                *offset,
342                || x.value().map(|x| *x * x.square()),
343            )?;
344        }
345
346        *offset += 1;
347
348        let outputs = Value::from_iter(inputs.iter().map(|x| x.value().copied()))
349            .map(|inputs: Vec<F>| {
350                let mut inputs = inputs;
351                full_round_cpu(round_index, &mut inputs);
352                inputs
353            })
354            .transpose_vec(WIDTH);
355
356        outputs
357            .iter()
358            .zip(self.config.register_cols[0..WIDTH].iter())
359            .zip(inputs)
360            .try_for_each(|((output, column), input)| {
361                region
362                    .assign_advice(|| "full round output", *column, *offset, || *output)
363                    .map(|assigned| *input = assigned)
364            })
365    }
366
367    /// Analogue for optimised partial rounds.
368    ///
369    /// Note: Initially, only the first `WIDTH` inputs are assigned, but not the
370    /// `NB_SKIPS_CIRCUIT` auxiliary advice columns at the current offset. This
371    /// function assigns them as well as the first `WIDTH` columns of the
372    /// resulting output.
373    fn partial_round(
374        &self,
375        region: &mut Region<'_, F>,
376        inputs: &mut [AssignedNative<F>], // Length `WIDTH`.
377        round_batch_index: usize,
378        offset: &mut usize,
379    ) -> Result<(), Error> {
380        self.config.q_partial_round.enable(region, *offset)?;
381        self.assign_constants_partial(region, round_batch_index, *offset)?;
382
383        let outputs = Value::from_iter(inputs.iter().map(|x| x.value().copied()))
384            .map(|inputs: Vec<F>| {
385                let mut state = inputs;
386                let skip_advice_vals = partial_round_cpu_for_circuits(
387                    &self.config.pre_computed,
388                    round_batch_index,
389                    &mut state,
390                );
391                for (col, skip_advice) in self.config.register_cols[WIDTH..WIDTH + NB_SKIPS_CIRCUIT]
392                    .iter()
393                    .zip(skip_advice_vals)
394                {
395                    let _ = region.assign_advice(
396                        || "partial round intermediary inputs",
397                        *col,
398                        *offset,
399                        || Value::known(skip_advice),
400                    );
401                }
402                state
403            })
404            .transpose_vec(WIDTH);
405
406        *offset += 1;
407        outputs
408            .iter()
409            .zip(self.config.register_cols[0..WIDTH].iter())
410            .zip(inputs)
411            .try_for_each(|((output, column), input)| {
412                region
413                    .assign_advice(|| "partial round outputs", *column, *offset, || *output)
414                    .map(|assigned| *input = assigned)
415            })
416    }
417
418    /// A combination of the different circuit gates to produce the full
419    /// Poseidon permutation (`NB_FULL_ROUNDS` full rounds, separated in the
420    /// middle by `NB_PARTIAL_ROUNDS` partial rounds, possibly with
421    /// optimized skips).
422    pub(super) fn permutation(
423        &self,
424        layouter: &mut impl Layouter<F>,
425        inputs: &AssignedRegister<F>,
426    ) -> Result<AssignedRegister<F>, Error> {
427        layouter.assign_region(
428            || "permutation layout",
429            |mut region| {
430                let mut offset: usize = 0;
431
432                let mut state: AssignedRegister<F> = self
433                    .native_chip
434                    .add_constants_in_region(
435                        &mut region,
436                        inputs,
437                        &F::ROUND_CONSTANTS[0],
438                        &mut offset,
439                    )?
440                    .try_into()
441                    .unwrap();
442
443                // The first full round assumes that `add_constants_in_region` above assigns
444                // `state` in the same columns as Poseidon in the region. This is checked by the
445                // below assertion.
446                assert!(
447                    state.iter().zip(self.config.register_cols).all(|(acell, col)| {
448                        let col1: Column<Advice> = acell.cell().column.try_into().unwrap();
449                        col1 == col
450                    })
451                );
452
453                for round_index in 0..NB_FULL_ROUNDS / 2 {
454                    self.full_round(&mut region, &mut state, round_index, &mut offset)?;
455                }
456                for round_batch_index in 0..NB_PARTIAL_ROUNDS / (1 + NB_SKIPS_CIRCUIT) {
457                    self.partial_round(&mut region, &mut state, round_batch_index, &mut offset)?;
458                }
459                for round_index in
460                    (NB_FULL_ROUNDS / 2 + NB_PARTIAL_ROUNDS..).take(NB_FULL_ROUNDS / 2)
461                {
462                    self.full_round(&mut region, &mut state, round_index, &mut offset)?;
463                }
464                Ok(state)
465            },
466        )
467    }
468}
469
470impl<F: PoseidonField> SpongeInstructions<F, AssignedNative<F>, AssignedNative<F>>
471    for PoseidonChip<F>
472{
473    type State = AssignedPoseidonState<F>;
474
475    fn init(
476        &self,
477        layouter: &mut impl Layouter<F>,
478        input_len: Option<usize>,
479    ) -> Result<Self::State, Error> {
480        let zero = self.native_chip.assign_fixed(layouter, F::ZERO)?;
481        let mut register: AssignedRegister<F> = vec![zero; WIDTH].try_into().unwrap();
482        register[RATE] = self.native_chip.assign_fixed(
483            layouter,
484            F::from_u128(input_len.map(|len| len as u128).unwrap_or(1 << 64)),
485        )?;
486        Ok(AssignedPoseidonState {
487            register,
488            queue: Vec::new(),
489            squeeze_position: 0,
490            input_len,
491        })
492    }
493
494    fn absorb(
495        &self,
496        _layouter: &mut impl Layouter<F>,
497        state: &mut Self::State,
498        inputs: &[AssignedNative<F>],
499    ) -> Result<(), Error> {
500        state.queue.extend(inputs.to_vec());
501        state.squeeze_position = 0;
502        Ok(())
503    }
504
505    fn squeeze(
506        &self,
507        layouter: &mut impl Layouter<F>,
508        state: &mut Self::State,
509    ) -> Result<AssignedNative<F>, Error> {
510        if state.squeeze_position > 0 {
511            // If `input_len` was specified, we only allow 1 squeeze.
512            if state.input_len.is_some() {
513                panic!(
514                    "Attempting to squeeze multiple times a fixed-size Poseidon sponge (Circuit)."
515                )
516            };
517            debug_assert!(state.queue.is_empty());
518            let output = state.register[state.squeeze_position % RATE].clone();
519            state.squeeze_position = (state.squeeze_position + 1) % RATE;
520            return Ok(output);
521        }
522
523        match state.input_len {
524            None => {
525                let padding =
526                    self.native_chip.assign_fixed(layouter, F::from(state.queue.len() as u64))?;
527                state.queue.push(padding);
528            }
529            Some(len) => {
530                if state.queue.len() != len {
531                    panic!("Inconsistent lengths in fixed-size Poseidon sponge (Circuit). Expected: {}, found: {}.", len, state.queue.len())
532                };
533            }
534        }
535
536        for chunk in state.queue.chunks(RATE) {
537            for (entry, value) in state.register.iter_mut().zip(chunk.iter()) {
538                *entry = self.native_chip.add(layouter, entry, value)?;
539            }
540            state.register = self.permutation(layouter, &state.register)?;
541        }
542
543        state.queue = Vec::new();
544        state.squeeze_position = 1 % RATE;
545        Ok(state.register[0].clone())
546    }
547}
548
549#[cfg(any(test, feature = "testing"))]
550impl<F: PoseidonField> FromScratch<F> for PoseidonChip<F> {
551    type Config = (NativeConfig, PoseidonConfig<F>);
552
553    fn new_from_scratch(config: &Self::Config) -> Self {
554        let native_chip = NativeChip::new(&config.0, &());
555        PoseidonChip::new(&config.1, &native_chip)
556    }
557
558    fn configure_from_scratch(
559        meta: &mut ConstraintSystem<F>,
560        advice_columns: &mut Vec<Column<Advice>>,
561        fixed_columns: &mut Vec<Column<Fixed>>,
562        instance_columns: &[Column<Instance>; 2],
563    ) -> Self::Config {
564        let nb_advice_needed = std::cmp::max(NB_POSEIDON_ADVICE_COLS, NB_ARITH_COLS);
565        let nb_fixed_needed = std::cmp::max(NB_POSEIDON_FIXED_COLS, NB_ARITH_FIXED_COLS);
566
567        while advice_columns.len() < nb_advice_needed {
568            advice_columns.push(meta.advice_column());
569        }
570        while fixed_columns.len() < nb_fixed_needed {
571            fixed_columns.push(meta.fixed_column());
572        }
573
574        let native_config = NativeChip::configure(
575            meta,
576            &(
577                advice_columns[..NB_ARITH_COLS].try_into().unwrap(),
578                fixed_columns[..NB_ARITH_FIXED_COLS].try_into().unwrap(),
579                *instance_columns,
580            ),
581        );
582        let poseidon_config = PoseidonChip::configure(
583            meta,
584            &(
585                advice_columns[..NB_POSEIDON_ADVICE_COLS].try_into().unwrap(),
586                fixed_columns[..NB_POSEIDON_FIXED_COLS].try_into().unwrap(),
587            ),
588        );
589
590        (native_config, poseidon_config)
591    }
592
593    fn load_from_scratch(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
594        self.native_chip.load_from_scratch(layouter)
595    }
596}
597
598#[cfg(test)]
599mod tests {
600    use midnight_proofs::{circuit::SimpleFloorPlanner, dev::MockProver, plonk::Circuit};
601
602    use super::*;
603    use crate::{
604        field::NativeGadget,
605        hash::poseidon::{permutation_cpu, round_skips::PreComputedRoundCPU},
606        instructions::{sponge::tests::test_sponge, AssertionInstructions},
607        utils::circuit_modeling::{circuit_to_json, cost_measure_end, cost_measure_start},
608    };
609
610    #[derive(Clone, Debug, Default)]
611    struct PermCircuit<F> {
612        inputs: [Value<F>; WIDTH],
613        expected: [F; WIDTH],
614    }
615
616    // Implements one instance of a Poseidon circuit (i.e., where the prover
617    // justifies they know the preimage of a given value). In combination with the
618    // golden files, it can be used to estimate precisely the resources consumed by
619    // Poseidon's hash: subtract the resources for two hashes (uncomment the line
620    // at the end of this `impl` block to trigger a second dummy hash) by the
621    // resources for one hash.
622    impl<F: PoseidonField> Circuit<F> for PermCircuit<F> {
623        type Config = <PoseidonChip<F> as FromScratch<F>>::Config;
624
625        type FloorPlanner = SimpleFloorPlanner;
626
627        type Params = ();
628
629        fn without_witnesses(&self) -> Self {
630            unreachable!()
631        }
632
633        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
634            let committed_instance_column = meta.instance_column();
635            let instance_column = meta.instance_column();
636            PoseidonChip::configure_from_scratch(
637                meta,
638                &mut vec![],
639                &mut vec![],
640                &[committed_instance_column, instance_column],
641            )
642        }
643
644        fn synthesize(
645            &self,
646            config: Self::Config,
647            mut layouter: impl Layouter<F>,
648        ) -> Result<(), Error> {
649            let poseidon_chip = PoseidonChip::new_from_scratch(&config);
650
651            let inputs: AssignedRegister<F> = poseidon_chip
652                .native_chip
653                .assign_many(&mut layouter, &self.inputs)?
654                .try_into()
655                .unwrap();
656            cost_measure_start(&mut layouter);
657            let outputs = poseidon_chip.permutation(&mut layouter, &inputs)?;
658            cost_measure_end(&mut layouter);
659
660            for (out, expected) in outputs.iter().zip(self.expected.iter()) {
661                poseidon_chip.native_chip.assert_equal_to_fixed(&mut layouter, out, *expected)?;
662            }
663
664            poseidon_chip.load_from_scratch(&mut layouter)
665        }
666    }
667
668    pub fn run_permutation_test<F>(inputs: [F; WIDTH], cost_model: bool)
669    where
670        F: PoseidonField + ff::FromUniformBytes<64> + Ord,
671    {
672        let pre_computed = PreComputedRoundCPU::init();
673        let mut expected = inputs;
674        permutation_cpu(&pre_computed, &mut expected);
675
676        let circuit = PermCircuit {
677            inputs: inputs.map(Value::known),
678            expected,
679        };
680
681        MockProver::run(&circuit, vec![vec![], vec![]]).unwrap().assert_satisfied();
682
683        if cost_model {
684            circuit_to_json("Poseidon", "one_permutation", circuit);
685        }
686    }
687
688    fn run_sponge_test<F>(field: &str, cost_model: bool)
689    where
690        F: PoseidonField + ff::FromUniformBytes<64> + Ord,
691    {
692        println!(
693            ">> Testing Poseidon Sponge (field {field}, {} partial-round skips)",
694            NB_SKIPS_CIRCUIT
695        );
696        test_sponge::<
697            F,
698            AssignedNative<F>,
699            AssignedNative<F>,
700            PoseidonChip<F>,
701            NativeGadget<F, _, _>,
702        >(cost_model, "Poseidon");
703        println!("=> Done.\n")
704    }
705
706    #[test]
707    fn permutation_test() {
708        let inputs = [midnight_curves::Fq::from(0); WIDTH];
709        // Set the second argument to true to experiment on the permutation cost.
710        run_permutation_test(inputs, true);
711    }
712
713    #[test]
714    fn sponge_test() {
715        // Consistency tests between the CPU and circuit implementations of the
716        // permutation.
717        run_sponge_test::<midnight_curves::Fq>("blstrs", true);
718    }
719}
```