# Stock foreign curve MSM implementation excerpts
> Source: file:///home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs
> Collected: 2026-10-26
> Published: Unknown

Lines 797-880:
```rust
    fn msm(
        &self,
        layouter: &mut impl Layouter<F>,
        scalars: &[Self::Scalar],
        bases: &[Self::Point],
    ) -> Result<Self::Point, Error> {
        let scalars = scalars
            .iter()
            .map(|s| (s.clone(), C::ScalarField::NUM_BITS as usize))
            .collect::<Vec<_>>();
        self.msm_by_bounded_scalars(layouter, &scalars, bases)
    }

    fn msm_by_bounded_scalars(
        &self,
        layouter: &mut impl Layouter<F>,
        scalars: &[(S::Scalar, usize)],
        bases: &[AssignedForeignPoint<F, C, B>],
    ) -> Result<AssignedForeignPoint<F, C, B>, Error> {
        assert_eq!(scalars.len(), bases.len(), "`|scalars| != |bases|`");

        const WS: usize = 4;
        let scalar_chip = self.scalar_field_chip();

        let (scalars, bases, bases_with_1bit_scalar) =
            msm_preprocess(self, scalar_chip, layouter, scalars, bases)?;

        // In order to support the identity point for some bases, we select in-circuit
        // based on the value of is_id and put a 0 scalar and an arbitrary non-id point
        // (e.g. the generator) for the base when is_id equals 1.
        let mut non_id_bases = vec![];
        let mut scalars_of_non_id_bases = vec![];
        let zero: S::Scalar = scalar_chip.assign_fixed(layouter, C::ScalarField::ZERO)?;
        let g = self.assign_fixed(layouter, C::CryptographicGroup::generator())?;
        for (s, b) in scalars.iter().zip(bases.iter()) {
            let new_b = self.select(layouter, &b.is_id, &g, b)?;
            let new_s = scalar_chip.select(layouter, &b.is_id, &zero, &s.0)?;
            non_id_bases.push(new_b);
            scalars_of_non_id_bases.push((new_s, s.1));
        }

        // Scalars with a "bad" bound will be split with GLV into 2 scalars with a
        // half-size bound.
        // (The GLV scalars are guaranteed to have half-size.)
        let nb_bits_per_glv_scalar = C::ScalarField::NUM_BITS.div_ceil(2) as usize;
        let mut non_glv_scalars = vec![];
        let mut non_glv_bases = vec![];
        let mut glv_scalars = vec![];
        let mut glv_bases = vec![];
        for (s, b) in scalars_of_non_id_bases.iter().zip(non_id_bases.iter()) {
            // We heuristically say a bound is "bad" if it far from NUM_BITS / 2 in the
            // following sense. Note that, ATM, in windowed_msm all sequences
            // are padded with zeros to meet the longest one.
            if C::has_cubic_endomorphism() && s.1 > nb_bits_per_glv_scalar + WS {
                let ((s1, s2), (b1, b2)) = self.glv_split(layouter, &s.0, b)?;
                glv_scalars.push((s1, nb_bits_per_glv_scalar));
                glv_scalars.push((s2, nb_bits_per_glv_scalar));
                glv_bases.push(b1);
                glv_bases.push(b2);
            } else {
                non_glv_scalars.push(s.clone());
                non_glv_bases.push(b.clone());
            }
        }

        let scalars = [glv_scalars, non_glv_scalars].concat();
        let bases = [glv_bases, non_glv_bases].concat();

        let mut decomposed_scalars = vec![];
        for (s, nb_bits_s) in scalars.iter() {
            let s_bits = self.scalar_field_chip().assigned_to_le_chunks(
                layouter,
                s,
                WS,
                Some(nb_bits_s.div_ceil(WS)),
            )?;
            decomposed_scalars.push(s_bits)
        }
        let res = self.windowed_msm::<WS>(layouter, &decomposed_scalars, &bases)?;

        add_1bit_scalar_bases(layouter, self, scalar_chip, &bases_with_1bit_scalar, res)
    }

    fn mul_by_constant(
```

Lines 1726-1750:
```rust
    fn msm_randomness<const WS: usize>(
        &self,
        layouter: &mut impl Layouter<F>,
    ) -> Result<MsmRandomness<F, C, B>, Error> {
        if let Some(cached) = self.msm_randomness.borrow().get(&WS) {
            return Ok(cached.clone());
        }

        // Reuse `r` from any cached window size to avoid unnecessary point assignments.
        let r = match self.msm_randomness.borrow().values().next().map(|c| c.r.clone()) {
            Some(r) => r,
            None => {
                self.assign_without_subgroup_check(layouter, Value::known(self.random_point))?
            }
        };
        self.assert_non_zero(layouter, &r)?;

        let alpha = self.mul_by_u128(layouter, (1u128 << WS) - 1, &r)?;
        let neg_alpha = self.negate(layouter, &alpha)?;

        let windowed_randomness = MsmRandomness { r, neg_alpha };
        self.msm_randomness.borrow_mut().insert(WS, windowed_randomness.clone());
        Ok(windowed_randomness)
    }

```

Lines 1771-1910:
```rust
    fn windowed_msm<const WS: usize>(
        &self,
        layouter: &mut impl Layouter<F>,
        scalars: &[Vec<AssignedNative<F>>],
        bases: &[AssignedForeignPoint<F, C, B>],
    ) -> Result<AssignedForeignPoint<F, C, B>, Error> {
        assert_eq!(scalars.len(), bases.len(), "`|scalars| != |bases|`");

        if scalars.is_empty() {
            return self.assign_fixed(layouter, C::CryptographicGroup::identity());
        }

        // Assert that none of the bases is the identity point
        for p in bases.iter() {
            self.assert_non_zero(layouter, p)?;
        }

        // Pad all the sequences of chunks to have the same length.
        // TODO: This is not strictly necessary, we could be more efficient if some
        // sequences are shorter. For now we do not do this, as it is highly involved,
        // and not very compatible with our random accumulation trick below.
        let zero: AssignedNative<F> = self.native_gadget.assign_fixed(layouter, F::ZERO)?;
        let max_len = scalars.iter().fold(0, |m, chunks| max(m, chunks.len()));
        let mut padded_scalars = vec![];
        for s_bits in scalars.iter() {
            // Pad with zeros up to length `padded_len`.
            let mut s_bits = s_bits.to_vec();
            s_bits.resize(max_len, zero.clone());
            // Reverse to get big-endian order, for the double-and-add.
            let rev_s_bits = s_bits.into_iter().rev().collect::<Vec<_>>();
            padded_scalars.push(rev_s_bits)
        }

        // Sample `r`, a random point that allows us to use incomplete addition in the
        // double-and-add loop.
        // The value of `r` can be chosen by the prover (who will choose it uniformly
        // at random for statistical completeness) and this is not a problem for
        // soundness.
        //
        // The randomness `r` is shared across all MSM calls on this chip, so that
        // precomputed tables (which depend on α = (2^WS - 1) * r) can be cached
        // and reused for the same base points.
        //
        // Let l := bases.len(), the initial double-and-add accumulator will be
        // acc := l * r. On every double-and-add iteration i, we will double WS times
        // and then, for every j in [l], conditionally add (kj_i * base_j - α)
        // or (-α), depending on the relevant segment, kj_i, of scalars_j at
        // that iteration, where α := (2^WS - 1) r. After this, the total randomness in
        // the accumulator becomes:
        //   2^WS (l * r) - l * α = 2^WS (l * r) - l * (2^WS - 1) r = l * r.
        // This makes the total randomness invariant (l * r) at the beginning of every
        // iteration.
        //
        // To finish the computation we will thus need to subtract l * r, which will
        // be done in-circuit.
        //
        // TODO: Maybe we should check that the sampled r will not have a completeness
        // problem. The probability should be overwhelming, but if the bad event
        // happened, the proof would fail. We could sample another r here instead.
        let MsmRandomness { r, neg_alpha } = self.msm_randomness::<WS>(layouter)?;

        let l_times_r = self.mul_by_u128(layouter, bases.len() as u128, &r)?;

        // Get the global tag counter and increase it with |bases|
        let tag_cnt = *self.tag_cnt.clone().borrow();
        self.tag_cnt.replace(tag_cnt + bases.len() as u64);

        // Compute table, [-α, p-α, 2p-α, ..., (2^WS-1)p-α] for every p in bases.
        let mut tables = vec![];
        for (i, p) in bases.iter().enumerate() {
            // Assert that α.x ≠ p.x (note that α and -α have the same x-coordinate).
            self.incomplete_assert_different_x(layouter, &neg_alpha, p)?;
            let mut acc = neg_alpha.clone();
            let mut p_table = vec![acc.clone()];
            for _ in 1..(1usize << WS) {
                // In order to safetly use [incomplete_add] here, we need to ensure that:
                //   (1) acc != id
                //   (2) p != id
                //   (3) acc != p
                //
                // (1) holds because acc is initially -α, which cannot be the identity,
                //     because r is not and we have asserted 2^WS < Scalar::ORDER. Then,
                //     acc is always the result of incomplete_add, which is guaranteed to
                //     not produce the identity.
                // (2) holds because all the bases were asserted to not be the identity.
                // (3) holds because acc is initially -α, which has been asserted to not
                //     share the x coordinate with p, so the first iteration is fine.
                //     In other iterations, acc will be of the form kp-α for some
                //     k = 1,...,(2^WS-2). Note that (k-1)p-α cannot be the identity as it is
                //     the result of a previous call to [incomplete_add], thus kp-α != p, so
                //     the third precondition of [incomplete_add] is met.
                Self::completeness_error_if(&acc.value().zip(p.value()), |(av, pv)| {
                    av == pv || *av == -(*pv)
                })?;

                acc = self.incomplete_add(layouter, &acc, p)?;

                assert!(acc.x.is_well_formed() && acc.y.is_well_formed());
                p_table.push(acc.clone())
            }
            self.load_multi_select_table(layouter, &p_table, F::from(tag_cnt + i as u64))?;
            tables.push(p_table)
        }

        let nb_iterations = max_len;
        let mut acc = l_times_r.clone();

        #[allow(clippy::needless_range_loop)]
        for i in 0..nb_iterations {
            for _ in 0..WS {
                acc = self.double(layouter, &acc)?;
            }
            for j in 0..bases.len() {
                let window = &padded_scalars[j][i];
                let addend =
                    self.multi_select(layouter, window, &tables[j], F::from(tag_cnt + j as u64))?;
                // In order to safetly use [incomplete_add] here, we need to ensure that:
                //   (1) acc != id
                //   (2) addend != id
                //   (3) acc != addend
                //
                // (1) holds because acc is the result of doubling a non-identity point
                //     (this is guaranteed because in the first iteration acc != id, and in
                //     any other iteration acc is the result of incomplete_add, which is
                //     guaranteed to not produce the identity.)
                // (2) holds because all the points in the tables are different from the
                //     identity, as asserted above (in the construction of the tables).
                // (3) is asserted here, this assertion will not hinder completeness except
                //     with negligible probability (over the choice of α).
                Self::completeness_error_if(&acc.value().zip(addend.value()), |(av, addv)| {
                    av == addv || *av == -(*addv)
                })?;

                self.incomplete_assert_different_x(layouter, &acc, &addend)?;
                acc = self.incomplete_add(layouter, &acc, &addend)?;
            }
        }

        let r_correction = self.negate(layouter, &l_times_r)?;
        self.add(layouter, &acc, &r_correction)
```
