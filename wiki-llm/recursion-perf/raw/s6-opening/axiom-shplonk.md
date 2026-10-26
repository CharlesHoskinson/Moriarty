# Axiom SHPLONK prover
> Source: https://raw.githubusercontent.com/axiom-crypto/halo2-gpu/b2d4bcfd4261cc506b5412fb04fe631f028849d7/halo2_proofs/src/poly/kzg/multiopen/shplonk/prover.rs
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Fetched with scrapling; raw response retrieved again to preserve source-code punctuation.

```
use std::fmt::Debug;
use std::hash::Hash;

use super::{
    construct_intermediate_sets, ChallengeU, ChallengeV, ChallengeY, Commitment, RotationSet,
};
use crate::arithmetic::{
    eval_polynomial, evaluate_vanishing_polynomial, lagrange_interpolate, powers, CurveAffine,
};
use crate::cuda::funcs::{
    kate_division_device, kate_division_device_padded_with_d_root,
    kate_division_device_with_d_root, poly_multiply_add_device,
    poly_multiply_add_device_at_lut_offset, poly_scale_device_with_d_s_minus_one,
    poly_sub_scalar_at_zero_device, poly_sub_short_out_of_place_device, GPU_MSM_THRESHOLD,
};
use crate::cuda::utils::HALO2_GPU_CTX;
use crate::poly::commitment::{Blind, ParamsProver, Prover};
use crate::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use crate::poly::query::{PolynomialPointer, ProverQuery};
use crate::poly::{Coeff, Device, DevicePolyExt, PolyRef, Polynomial};
use crate::transcript::{EncodedChallenge, TranscriptWrite};
use crate::SerdeCurveAffine;
use openvm_cuda_common::copy::{MemCopyD2H, MemCopyH2D};
use openvm_cuda_common::d_buffer::DeviceBuffer;

#[cfg(feature = "profile")]
use ark_std::{end_timer, start_timer};
use ff::Field;
use group::Curve;

use pairing::Engine;
use rand_core::RngCore;
use rayon::prelude::*;

use std::io;

struct CommitmentExtension<'a, C: CurveAffine> {
    commitment: Commitment<C::Scalar, PolynomialPointer<'a, C>>,
    low_degree_equivalent: Polynomial<C::Scalar, Coeff>,
}

impl<'a, C: CurveAffine> Commitment<C::Scalar, PolynomialPointer<'a, C>> {
    fn extend(&self, points: &[C::Scalar]) -> CommitmentExtension<'a, C> {
        let poly = lagrange_interpolate(points, &self.evals()[..]);

        let low_degree_equivalent = Polynomial::new(poly);

        CommitmentExtension {
            commitment: self.clone(),
            low_degree_equivalent,
        }
    }
}

struct RotationSetExtension<'a, C: CurveAffine> {
    commitments: Vec<CommitmentExtension<'a, C>>,
    points: Vec<C::Scalar>,
}

impl<'a, C: CurveAffine> RotationSet<C::Scalar, PolynomialPointer<'a, C>> {
    fn extend(self, commitments: Vec<CommitmentExtension<'a, C>>) -> RotationSetExtension<'a, C> {
        RotationSetExtension {
            commitments,
            points: self.points,
        }
    }
}

/// Concrete KZG prover with SHPLONK variant
#[derive(Debug)]
pub struct ProverSHPLONK<'a, E: Engine> {
    params: &'a ParamsKZG<E>,
}

impl<'a, E: Engine> ProverSHPLONK<'a, E> {
    /// Given parameters creates new prover instance
    pub fn new(params: &'a ParamsKZG<E>) -> Self {
        Self { params }
    }
}

/// Create a multi-opening proof
impl<'params, E: Engine + Debug> Prover<'params, KZGCommitmentScheme<E>>
    for ProverSHPLONK<'params, E>
where
    E::G1Affine: SerdeCurveAffine<ScalarExt = E::Fr, CurveExt = E::G1>,
    E::G2Affine: SerdeCurveAffine,
    E::Fr: Hash,
{
    const QUERY_INSTANCE: bool = false;

    fn new(params: &'params ParamsKZG<E>) -> Self {
        Self { params }
    }

    /// Create a multi-opening proof
    fn create_proof<
        'com,
        Ch: EncodedChallenge<E::G1Affine>,
        T: TranscriptWrite<E::G1Affine, Ch>,
        R,
        I,
    >(
        &self,
        _: R,
        transcript: &mut T,
        queries: I,
    ) -> io::Result<()>
    where
        I: IntoIterator<Item = ProverQuery<'com, E::G1Affine>> + Clone,
        R: RngCore,
    {
        crate::perf_section!("shplonk");
        let y: ChallengeY<_> = transcript.squeeze_challenge_scalar();

        let intermediate_sets = {
            crate::perf_section!("construct_intermediate_sets");
            construct_intermediate_sets(queries)
        };
        let (rotation_sets, super_point_set) = (
            intermediate_sets.rotation_sets,
            intermediate_sets.super_point_set,
        );

        #[cfg(feature = "profile")]
        for (i, rotate_set) in rotation_sets.iter().enumerate() {
            log::debug!(
                "rotation set {}: points.len = {}, polys.len = {}",
                i,
                rotate_set.points.len(),
                rotate_set.commitments.len()
            );
        }

        #[cfg(feature = "profile")]
        let get_r_x_time = start_timer!(|| "get r(X)");
        let rotation_sets: Vec<RotationSetExtension<E::G1Affine>> = rotation_sets
            .into_par_iter()
            .map(|rotation_set| {
                let commitments: Vec<CommitmentExtension<E::G1Affine>> = rotation_set
                    .commitments
                    .as_slice()
                    .into_par_iter()
                    .map(|commitment_data| commitment_data.extend(&rotation_set.points))
                    .collect();
                rotation_set.extend(commitments)
            })
            .collect();
        #[cfg(feature = "profile")]
        end_timer!(get_r_x_time);

        let v: ChallengeV<_> = transcript.squeeze_challenge_scalar();

        let n = self.params.n as usize;

        // Hoist a y-powers LUT: max k = (max commitments across rotation sets),
        // uploaded ONCE per shplonk call. Both `quotient_contribution` and
        // `linearisation_contribution`'s `r_i` host fold index into it (the
        // host fold reads it via the existing `powers(*y)` iterator —
        // identical values, just precomputed for the device side).
        let max_commitments_per_rs = rotation_sets
            .iter()
            .map(|rs| rs.commitments.len())
            .max()
            .unwrap_or(1)
            .max(1);
        let y_powers_host: Vec<E::Fr> = powers(*y).take(max_commitments_per_rs).collect();
        let d_y_powers: DeviceBuffer<E::Fr> = y_powers_host
            .as_slice()
            .to_device_on(&HALO2_GPU_CTX)
            .expect("H2D of y_powers LUT failed");

        #[allow(clippy::type_complexity)]
        let quotient_contribution = |rotation_set: &RotationSetExtension<E::G1Affine>| -> (
            Polynomial<E::Fr, Coeff, Device>,
            Polynomial<E::Fr, Coeff, Device>,
        ) {
            crate::perf_section!("quotient_contribution.rayon_worker");

            // device-resident p_x = sum_i y^i * commitments[i].poly.
            // Zero-init via cudaMemsetAsync; Fr::ZERO is all-zero bytes.
            let mut d_p_x: DeviceBuffer<E::Fr> =
                DeviceBuffer::<E::Fr>::with_capacity_on(n, &HALO2_GPU_CTX);
            d_p_x
                .fill_zero_on(&HALO2_GPU_CTX)
                .expect("fill_zero on p_x accumulator failed");

            for (i, commitment) in rotation_set.commitments.iter().enumerate() {
                match commitment.commitment.get().poly {
                    PolyRef::Device(d_poly) => {
                        poly_multiply_add_device_at_lut_offset(
                            &mut d_p_x,
                            d_poly.device_buf(),
                            &d_y_powers,
                            i,
                        )
                        .expect("p_x device FMA failed");
                    }
                    PolyRef::Host(h_poly) => {
                        let d_tmp = h_poly
                            .values()
                            .to_device_on(&HALO2_GPU_CTX)
                            .expect("H2D of Host poly for p_x FMA failed");
                        poly_multiply_add_device_at_lut_offset(&mut d_p_x, &d_tmp, &d_y_powers, i)
                            .expect("p_x mixed FMA failed");
                    }
                }
            }

            // small host fold r_x over low_degree_equivalent (<=5 elts).
            let r_x = rotation_set
                .commitments
                .iter()
                .zip(powers(*y))
                .map(|(commitment, power_of_y)| {
                    commitment.low_degree_equivalent.clone() * power_of_y
                })
                .reduce(|acc, r_x| acc + &r_x)
                .unwrap();

            // out-of-place `n_x = p_x - r_x` (short-prefix subtract)
            // into a fresh buffer. No D2D clone of p_x — d_p_x is preserved
            // for the linearisation closure which consumes it as
            // `l_x_short = p_x - r_i` after the quotient pass completes.
            let d_r_x_short: DeviceBuffer<E::Fr> = r_x
                .values()
                .to_device_on(&HALO2_GPU_CTX)
                .expect("H2D r_x_short failed");
            let mut d_n_x: DeviceBuffer<E::Fr> =
                DeviceBuffer::<E::Fr>::with_capacity_on(n, &HALO2_GPU_CTX);
            poly_sub_short_out_of_place_device(&mut d_n_x, &d_p_x, &d_r_x_short)
                .expect("poly_sub_short_out_of_place_device on d_n_x failed");

            // chained kate_division by each root in rs.points.
            // The final kate dispatches the padded variant, which writes
            // its length-(n - n_points) quotient and the trailing zeros
            // directly into a length-n buffer in a single kernel launch.
            // If `n_points == 0`, d_n_x is already length n and is
            // returned verbatim.
            let n_points = rotation_set.points.len();
            let d_q_padded: DeviceBuffer<E::Fr> = if n_points == 0 {
                d_n_x
            } else {
                let mut d_q = d_n_x;
                for (i, point) in rotation_set.points.iter().enumerate() {
                    let d_root: DeviceBuffer<E::Fr> = std::slice::from_ref(point)
                        .to_device_on(&HALO2_GPU_CTX)
                        .expect("H2D kate root failed");
                    if i + 1 == n_points {
                        d_q = kate_division_device_padded_with_d_root(&d_q, &d_root, n)
                            .expect("kate_division_device_padded final failed");
                    } else {
                        d_q = kate_division_device_with_d_root(&d_q, &d_root)
                            .expect("kate_division_device_with_d_root failed");
                    }
                }
                d_q
            };

            let p_x_dev = Polynomial::<E::Fr, Coeff, Device>::from_device(d_p_x);
            (Polynomial::from_device(d_q_padded), p_x_dev)
        };

        #[cfg(feature = "profile")]
        let quotients_time = start_timer!(|| "get quotient 1");
        // (P_i(X) - R_i(X)) / (Z_S_i(X))
        // Rayon worker threads enqueue on the shared CUDA stream; the prover's
        // tracing span must propagate so emitted metrics carry the parent group.
        let parent_span = tracing::Span::current();
        #[allow(clippy::type_complexity)]
        let (quotient_polynomials, p_xs): (
            Vec<Polynomial<E::Fr, Coeff, Device>>,
            Vec<Polynomial<E::Fr, Coeff, Device>>,
        ) = rotation_sets
            .par_iter()
            .map(|rs| parent_span.in_scope(|| quotient_contribution(rs)))
            .unzip();
        #[cfg(feature = "profile")]
        end_timer!(quotients_time);

        // device h_x = sum_j v^j * quotient_polynomials[j].
        // Hoist a v-powers LUT for both this fold and the linearisation l_x fold.
        let v_powers_host: Vec<E::Fr> =
            powers(*v).take(quotient_polynomials.len().max(1)).collect();
        let d_v_powers: DeviceBuffer<E::Fr> = v_powers_host
            .as_slice()
            .to_device_on(&HALO2_GPU_CTX)
            .expect("H2D of v_powers LUT failed");

        let h_x_device = {
            crate::perf_section!("h_x_device_reduce");
            let mut d_h_x: DeviceBuffer<E::Fr> =
                DeviceBuffer::<E::Fr>::with_capacity_on(n, &HALO2_GPU_CTX);
            d_h_x
                .fill_zero_on(&HALO2_GPU_CTX)
                .expect("fill_zero on h_x accumulator failed");
            for (j, poly) in quotient_polynomials.into_iter().enumerate() {
                poly_multiply_add_device_at_lut_offset(
                    &mut d_h_x,
                    poly.device_buf(),
                    &d_v_powers,
                    j,
                )
                .expect("h_x device FMA failed");
            }
            Polynomial::<E::Fr, Coeff, Device>::from_device(d_h_x)
        };

        #[cfg(feature = "profile")]
        let get_h_commit = start_timer!(|| "get H(X) commit");
        let h = {
            crate::perf_section!("h_commit");
            commit_device_with_host_fallback(self.params, &h_x_device).to_affine()
        };
        transcript.write_point(h)?;
        let u: ChallengeU<_> = transcript.squeeze_challenge_scalar();
        #[cfg(feature = "profile")]
        end_timer!(get_h_commit);

        let linearisation_contribution = |(rotation_set, p_x_dev): (
            RotationSetExtension<E::G1Affine>,
            Polynomial<E::Fr, Coeff, Device>,
        )|
         -> (Polynomial<E::Fr, Coeff, Device>, E::Fr) {
            crate::perf_section!("shplonk.linearisation_contribution.rayon_worker");

            // diffs = super_point_set \ rs.points.
            let mut diffs = super_point_set.clone();
            for point in rotation_set.points.iter() {
                diffs.remove(point);
            }
            let diffs = diffs.into_iter().collect::<Vec<_>>();

            // z_i = vanishing(diffs, u). Tiny host fold.
            let z_i = evaluate_vanishing_polynomial(&diffs[..], *u);

            // r_i_j = eval(low_degree_equivalent_i, u). Host fold (<=5 elts each).
            let r_i_j = rotation_set
                .commitments
                .iter()
                .map(|commitment| eval_polynomial(&commitment.low_degree_equivalent, *u))
                .collect::<Vec<_>>();

            // r_i = sum_i y^i * r_i_j. Tiny host fold.
            let r_i: E::Fr = r_i_j
                .into_iter()
                .zip(powers(*y))
                .map(|(r, p)| r * p)
                .reduce(|acc, val| acc + val)
                .unwrap();

            // consume p_x_dev as l_x_short; l_x_short[0] -= r_i.
            let mut d_l_x: DeviceBuffer<E::Fr> = p_x_dev.into_device_buf();
            let d_r_i: DeviceBuffer<E::Fr> = std::slice::from_ref(&r_i)
                .to_device_on(&HALO2_GPU_CTX)
                .expect("H2D r_i failed");
            poly_sub_scalar_at_zero_device(&mut d_l_x, &d_r_i)
                .expect("poly_sub_scalar_at_zero_device(l_x_short, r_i): device subtract of scalar r_i from coefficient 0 failed");

            // l_x_part = l_x_short * z_i (in-place).
            let z_i_minus_one = z_i - E::Fr::ONE;
            let d_z_i_minus_one: DeviceBuffer<E::Fr> = std::slice::from_ref(&z_i_minus_one)
                .to_device_on(&HALO2_GPU_CTX)
                .expect("H2D z_i - 1 failed");
            poly_scale_device_with_d_s_minus_one(&mut d_l_x, &d_z_i_minus_one)
                .expect("device l_x_part *= z_i failed");

            (Polynomial::from_device(d_l_x), z_i)
        };

        #[cfg(feature = "profile")]
        let get_l_x_part = start_timer!(|| "get l(X) parts");
        let parent_span_lin = tracing::Span::current();
        #[allow(clippy::type_complexity)]
        let (linearisation_contributions, z_diffs): (
            Vec<Polynomial<E::Fr, Coeff, Device>>,
            Vec<E::Fr>,
        ) = rotation_sets
            .into_par_iter()
            .zip(p_xs.into_par_iter())
            .map(|t| parent_span_lin.in_scope(|| linearisation_contribution(t)))
            .unzip();
        #[cfg(feature = "profile")]
        end_timer!(get_l_x_part);

        // device l_x_total = sum_j v^j * linearisation_contributions[j].
        let mut d_l_x: DeviceBuffer<E::Fr> = {
            crate::perf_section!("shplonk.l_x_device_reduce");
            let mut acc: DeviceBuffer<E::Fr> =
                DeviceBuffer::<E::Fr>::with_capacity_on(n, &HALO2_GPU_CTX);
            acc.fill_zero_on(&HALO2_GPU_CTX)
                .expect("fill_zero on l_x accumulator failed");
            for (j, poly) in linearisation_contributions.into_iter().enumerate() {
                poly_multiply_add_device_at_lut_offset(&mut acc, poly.device_buf(), &d_v_powers, j)
                    .expect("l_x device FMA failed");
            }
            acc
        };

        // zt_eval = vanishing(super_point_set, u). Tiny host fold.
        let super_point_set_vec = super_point_set.into_iter().collect::<Vec<_>>();
        let zt_eval = evaluate_vanishing_polynomial(&super_point_set_vec[..], *u);

        // l_x = l_x_total - h_x * zt_eval => FMA with -zt_eval.
        let neg_zt = -zt_eval;
        poly_multiply_add_device(&mut d_l_x, h_x_device.device_buf(), neg_zt)
            .expect("l_x FMA with -zt_eval failed");

        // debug-only sanity check that eval(l_x, u) == 0.
        #[cfg(debug_assertions)]
        {
            let got = crate::cuda::funcs::eval_polynomial_device(&d_l_x, *u)
                .expect("eval_polynomial_device for l_x at u failed");
            assert_eq!(got, E::Fr::ZERO);
        }

        // h_x_final = kate_division(l_x, u). Length n-1.
        let mut d_h_x_final = {
            crate::perf_section!("shplonk.final_l_x_kate_div");
            kate_division_device(&d_l_x, *u).expect("final kate_division_device failed")
        };

        // normalize coefficients by z_0_diff_inv via in-place scale.
        let z_0_diff_inv = z_diffs[0].invert().unwrap();
        let z_0_diff_inv_minus_one = z_0_diff_inv - E::Fr::ONE;
        let d_z0_minus_one: DeviceBuffer<E::Fr> = std::slice::from_ref(&z_0_diff_inv_minus_one)
            .to_device_on(&HALO2_GPU_CTX)
            .expect("H2D z_0_diff_inv - 1 failed");
        poly_scale_device_with_d_s_minus_one(&mut d_h_x_final, &d_z0_minus_one)
            .expect("device h_x_final scale failed");

        // commit h_x_final via commit_device (Host fallback below threshold).
        let h_x_final_dev = Polynomial::<E::Fr, Coeff, Device>::from_device(d_h_x_final);
        let h_final = {
            crate::perf_section!("shplonk.h_final_commit");
            commit_device_with_host_fallback(self.params, &h_x_final_dev).to_affine()
        };
        transcript.write_point(h_final)?;

        Ok(())
    }
}

/// Dispatch `ParamsKZG::commit_device` for circuits at or above
/// `GPU_MSM_THRESHOLD`; fall back to `commit` (Host arm) for smaller
/// circuits whose MSM length the device MSM kernel refuses.
fn commit_device_with_host_fallback<E>(
    params: &ParamsKZG<E>,
    poly: &Polynomial<E::Fr, Coeff, Device>,
) -> E::G1
where
    E: Engine + Debug,
    E::G1Affine: SerdeCurveAffine<ScalarExt = E::Fr, CurveExt = E::G1>,
    E::G2Affine: SerdeCurveAffine,
{
    if poly.len() >= GPU_MSM_THRESHOLD {
        params.commit_device(poly, Blind::default())
    } else {
        let host_vec: Vec<E::Fr> = poly
            .device_buf()
            .to_host_on(&HALO2_GPU_CTX)
            .expect("D2H of small-circuit poly for commit fallback failed");
        let host_poly: Polynomial<E::Fr, Coeff> = Polynomial::new(host_vec);
        params.commit(&host_poly, Blind::default())
    }
}

```
