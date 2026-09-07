# Exact source API and gadget supplement

Source-only inspection at pinned midnight-zk `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`. No Rust/Cargo/native execution. Every block gives the complete source-file SHA256 and exact line interval; source-identity.json verifies all 421 backend blobs against that Git commit.


## MockProver two-argument API and RowSizer

`proofs/src/dev/mod.rs:595-633`; SHA256 `e1104ad7d727da562fbffcab3ce700150674c144b3ef5e3a72d3c3a6a3600746`.

```rust
impl<F: FromUniformBytes<64> + Ord> RowSizer<F> {
    /// Synthesizes `circuit` and returns `(k, n)` — the minimum circuit size
    /// parameters such that `n = 2^k` is large enough to hold all assigned
    /// rows.
    pub fn min_k<ConcreteCircuit: Circuit<F>>(
        circuit: &ConcreteCircuit,
        instance: Vec<Vec<F>>,
    ) -> Result<(u32, usize), Error> {
        let mut cs = ConstraintSystem::default();
        #[cfg(feature = "circuit-params")]
        let config = ConcreteCircuit::configure_with_params(&mut cs, circuit.params());
        #[cfg(not(feature = "circuit-params"))]
        let config = ConcreteCircuit::configure(&mut cs);

        let mut sizer = RowSizer::new(instance);
        let constants = cs.constants.clone();
        ConcreteCircuit::FloorPlanner::synthesize(&mut sizer, circuit, config, constants)?;

        let blinding_factors = cs.blinding_factors();
        // Find the maximum positive rotation used by any gate query. Rows near the end
        // of usable_rows that are accessed with a positive rotation could
        // otherwise land in blinding rows, causing Value::Poison to appear
        // during constraint verification.
        let max_positive_rotation = cs
            .advice_queries()
            .iter()
            .map(|(_, r)| r.0)
            .chain(cs.fixed_queries().iter().map(|(_, r)| r.0))
            .chain(cs.instance_queries().iter().map(|(_, r)| r.0))
            .filter(|&r| r > 0)
            .max()
            .unwrap_or(0) as usize;
        let required_n = (*sizer.max_row.borrow() + blinding_factors + 2 + max_positive_rotation)
            .max(cs.minimum_rows());
        let n = required_n.next_power_of_two();
        let k = n.trailing_zeros();
        Ok((k, n))
    }
}
```


## MockProver run

`proofs/src/dev/mod.rs:744-775`; SHA256 `e1104ad7d727da562fbffcab3ce700150674c144b3ef5e3a72d3c3a6a3600746`.

```rust

impl<F: FromUniformBytes<64> + Ord> MockProver<F> {
    /// Runs a synthetic keygen-and-prove operation on the given circuit,
    /// automatically determining the minimum required `k` (circuit size
    /// parameter).
    ///
    /// Uses [`RowSizer`] for a lightweight dry run to find the minimum `k`,
    /// then performs full synthesis with that `k`.
    pub fn run<ConcreteCircuit: Circuit<F>>(
        circuit: &ConcreteCircuit,
        instance: Vec<Vec<F>>,
    ) -> Result<Self, Error> {
        let mut cs = ConstraintSystem::default();
        #[cfg(feature = "circuit-params")]
        let config = ConcreteCircuit::configure_with_params(&mut cs, circuit.params());
        #[cfg(not(feature = "circuit-params"))]
        let config = ConcreteCircuit::configure(&mut cs);

        assert_eq!(instance.len(), cs.num_instance_columns);

        // Dry run via RowSizer to find the minimum n = 2^k.
        let (k, n) = RowSizer::min_k(circuit, instance.clone())?;
        let constants = cs.constants.clone();

        let instance = instance
            .into_iter()
            .map(|instance| {
                assert!(
                    instance.len() <= n - (cs.blinding_factors() + 1),
                    "instance.len={}, n={}, cs.blinding_factors={}",
                    instance.len(),
                    n,
```


## PCS trait method

`proofs/src/poly/commitment.rs:113-125`; SHA256 `9a63fce3fccbdf2f61ff41f64bff0965135498ba9b49ec0375f7b351d0959782`.

```rust
    /// Multiplicative blow-up factor by which `params.g_monomial_size()` must
    /// exceed `2^k` (the circuit's Lagrange-domain size) for this PCS to
    /// commit every polynomial it produces at the requested circuit size.
    /// Returns `1` when no extension is needed.
    ///
    /// `cs_degree` is the constraint system's `cs.degree()`. Schemes that
    /// commit to a single combined polynomial (e.g. `single-h-commitment`,
    /// fflonk's bundles) factor that into their requested blow-up.
    fn srs_monomial_blowup(cs_degree: usize) -> usize {
        let _ = cs_degree; // Just to avoid a clippy warning.
        1
    }

```


## KZG implementation

`proofs/src/poly/kzg/mod.rs:113-126`; SHA256 `1f56dd2a2c1bb3243f3ee797406f59c395ed904e82af687bc2fbbdd4446fa0b7`.

```rust

    /// With `single-h-commitment` the quotient is committed as one polynomial
    /// of degree up to `(cs_degree - 1) * n`, so the monomial basis must be
    /// blown up to the next power of two of `cs_degree - 1`. Otherwise KZG
    /// commits each polynomial at the Lagrange size and needs no extension.
    fn srs_monomial_blowup(cs_degree: usize) -> usize {
        if cfg!(feature = "single-h-commitment") {
            (cs_degree - 1).next_power_of_two()
        } else {
            1
        }
    }

    fn read_commitment<T: Transcript>(
```


## Params trait

`proofs/src/poly/commitment.rs:174-202`; SHA256 `9a63fce3fccbdf2f61ff41f64bff0965135498ba9b49ec0375f7b351d0959782`.

```rust
    {
        assert_eq!(guards.len(), params.len());
        guards
            .into_iter()
            .zip(params)
            .try_for_each(|(guard, params)| guard.verify(params))
    }
}

/// Interface for PCS params
pub trait Params: Send + Sync {
    /// Returns the size of the Lagrange basis, expressed as the exponent `k`
    /// such that the Lagrange domain has `2^k` elements. This equals the
    /// circuit domain size and is used by keygen to validate the SRS.
    fn max_k(&self) -> u32;

    /// Returns the number of monomial-basis elements `[s^i]G₁` available in
    /// the SRS. For a standard SRS this equals `1 << max_k()`. When the
    /// `single-h-commitment` feature is enabled the monomial basis may be
    /// larger than the Lagrange basis (which covers only the circuit
    /// domain), so this method returns the true capacity for
    /// coefficient-form commitments.
    fn g_monomial_size(&self) -> usize {
        1 << self.max_k()
    }

    /// Downsize the params to work with a circuit of size `new_k`
    fn downsize(&mut self, new_k: u32);

```


## ParamsKZG methods

`proofs/src/poly/kzg/params.rs:58-76`; SHA256 `3dffddf75e91d5b2555056756f99fb81b587f7ef0b38756a23d3960e5b52a24f`.

```rust

impl<E: Engine> Params for ParamsKZG<E>
where
    E::G1Affine: CurveAffine,
{
    fn max_k(&self) -> u32 {
        #[cfg(not(feature = "single-h-commitment"))]
        assert_eq!(self.g.len(), self.g_lagrange.len());
        self.g_lagrange.len().ilog2()
    }

    fn g_monomial_size(&self) -> usize {
        self.g.len()
    }

    fn downsize(&mut self, new_k: u32) {
        ParamsKZG::<E>::downsize(self, new_k)
    }
}
```


## ParamsKZG read_custom

`proofs/src/poly/kzg/params.rs:281-345`; SHA256 `3dffddf75e91d5b2555056756f99fb81b587f7ef0b38756a23d3960e5b52a24f`.

```rust
    /// Reads params from a buffer.
    pub fn read_custom<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self>
    where
        E::G1Affine: SerdeObject,
        E::G2: ProcessedSerdeObject,
    {
        let mut k = [0u8; 4];
        reader.read_exact(&mut k[..])?;
        let k = u32::from_le_bytes(k);
        let n = 1 << k;

        let (g, g_lagrange) = match format {
            SerdeFormat::Processed => {
                let load_points_from_file_parallelly =
                    |reader: &mut R| -> io::Result<Vec<E::G1Affine>> {
                        let mut points_compressed =
                            vec![<E::G1Affine as GroupEncoding>::Repr::default(); n];
                        for points_compressed in points_compressed.iter_mut() {
                            reader.read_exact((*points_compressed).as_mut())?;
                        }
                        let mut points = vec![Option::<E::G1Affine>::None; n];
                        parallelize(&mut points, |points, chunks| {
                            for (i, point) in points.iter_mut().enumerate() {
                                *point = Option::from(E::G1Affine::from_bytes(
                                    &points_compressed[chunks + i],
                                ));
                            }
                        });
                        points
                            .into_iter()
                            .map(|p| p.ok_or_else(|| io::Error::other("invalid point encoding")))
                            .collect()
                    };

                let g = load_points_from_file_parallelly(reader)?;
                let g_lagrange = load_points_from_file_parallelly(reader)?;
                (g, g_lagrange)
            }
            SerdeFormat::RawBytes => {
                let g =
                    (0..n).map(|_| E::G1Affine::read_raw(reader)).collect::<Result<Vec<_>, _>>()?;
                let g_lagrange =
                    (0..n).map(|_| E::G1Affine::read_raw(reader)).collect::<Result<Vec<_>, _>>()?;
                (g, g_lagrange)
            }
            SerdeFormat::RawBytesUnchecked => {
                let g = (0..n).map(|_| E::G1Affine::read_raw_unchecked(reader)).collect::<Vec<_>>();
                let g_lagrange =
                    (0..n).map(|_| E::G1Affine::read_raw_unchecked(reader)).collect::<Vec<_>>();
                (g, g_lagrange)
            }
        };

        let g2 = E::G2::read(reader, format)?;
        let s_g2 = E::G2::read(reader, format)?;

        let g_lagrange_delta = suffix_sum(&g_lagrange);
        let g_lagrange_double_delta = suffix_sum(&g_lagrange_delta);
        Ok(Self {
            g,
            g_lagrange,
            g_lagrange_delta,
            g_lagrange_double_delta,
            g2,
            s_g2,
```


## Verifier public reexports

`circuits/src/verifier/mod.rs:32-54`; SHA256 `3a22403386081cd96a2411588760d21cb9443497e5675bdae946e73e3dea1c83`.

```rust
mod argument;
mod expressions;
mod kzg;
mod lookup;
mod msm;
pub(crate) mod pcs;
mod permutation;
mod traces;
mod transcript_gadget;
mod types;
mod utils;
mod verifier_gadget;

pub use accumulator::{Accumulator, AssignedAccumulator};
pub use kzg::{AssignedKZGCommitment, AssignedKZGMultiCommitment, InCircuitKZG};
pub use msm::{AssignedMsm, AssignedPoint, Msm, Point};
pub use pcs::{InCircuitHomomorphicCommitment, InCircuitPCS};
#[cfg(feature = "dev-curves")]
pub use types::BnEmulation;
pub use types::{BlstrsEmulation, SelfEmulation};
pub use verifier_gadget::VerifierGadget;

type VerifyingKey<S> =
```


## IVC setup

`aggregation/src/ivc/setup.rs:24-67`; SHA256 `8edf56b4b1e64ee754d484bac12207b7633f26073800efc1e368d4eb30d6614e`.

```rust
pub fn setup<T: Ivc>(
    params: ParamsKZG<E>,
    k: u32,
    ctx: T::Context,
) -> (IvcProver<T>, IvcVerifier<T>) {
    let mut cs = ConstraintSystem::default();
    ZkStdLib::configure(&mut cs, (IvcCircuit::<T>::arch(), (k - 1) as u8));
    let domain = EvaluationDomain::new(cs.degree() as u32, k);
    let relation = IvcCircuit::<T>::new(domain, cs, ctx.clone());

    // Uncomment for visualizing the size of this IVC circuit.
    // dbg!(midnight_zk_stdlib::cost_model(&relation, Some(k)));

    let vk = midnight_zk_stdlib::setup_vk(&params, &relation);
    let pk = midnight_zk_stdlib::setup_pk(&relation, &vk);

    let fixed_base_labels: Vec<PolynomialLabel> =
        fixed_bases::<S>(vk.vk()).keys().cloned().collect();

    let verifier = IvcVerifier {
        ctx: ctx.clone(),
        vk,
        params_verifier: params.verifier_params(),
    };

    let prover = IvcProver {
        params,
        relation,
        pk,
        state: T::genesis(&ctx),
        proof: vec![],
        acc: Accumulator::<S>::trivial(&fixed_base_labels),
    };

    (prover, verifier)
}
```


## IVC error variants

`aggregation/src/ivc/error.rs:1-26`; SHA256 `4e201e1b19a55827dc59aa5fb2954dc98359823f979f5162d84796db2b947dd3`.

```rust
//! Custom error type for the IVC module.

use std::fmt;

use midnight_proofs::plonk;

/// Error type for IVC operations.
#[derive(Debug)]
pub enum IvcError {
    /// A proof generation failed.
    ProofGeneration(plonk::Error),
    /// The provided instance is malformed.
    InvalidInstance,
    /// The provided witness is invalid.
    InvalidWitness(String),
    /// The instance's VK representation does not match the verifier's key.
    VkMismatch,
    /// The proof is invalid (accumulator pairing check failed).
    InvalidProof,
    /// The proof transcript contains trailing data.
    TranscriptNotEmpty,
    /// The application-level decider check failed.
    DeciderFailed,
}

impl From<plonk::Error> for IvcError {
```


## Stdlib instruction delegation

`zk_stdlib/src/instructions.rs:30-50`; SHA256 `1c076443c162853572f3a8b8390bb8065357d47233b5bcc787eea4e7c87d0941`.

```rust
impl<T> AssignmentInstructions<F, T> for ZkStdLib
where
    T: InnerValue,
    T::Element: Clone,
    NG: AssignmentInstructions<F, T>,
{
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<T::Element>,
    ) -> Result<T, Error> {
        self.native_gadget.assign(layouter, value)
    }

    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        constant: T::Element,
    ) -> Result<T, Error> {
        self.native_gadget.assign_fixed(layouter, constant)
    }
```


## Stdlib select

`zk_stdlib/src/instructions.rs:304-320`; SHA256 `1c076443c162853572f3a8b8390bb8065357d47233b5bcc787eea4e7c87d0941`.

```rust
impl<Assigned> ControlFlowInstructions<F, Assigned> for ZkStdLib
where
    Assigned: InnerValue,
    NG: ControlFlowInstructions<F, Assigned>,
{
    fn select(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        x: &Assigned,
        y: &Assigned,
    ) -> Result<Assigned, Error> {
        self.native_gadget.select(layouter, cond, x, y)
    }

    fn cond_swap(
        &self,
```


## Stdlib range check

`zk_stdlib/src/instructions.rs:336-355`; SHA256 `1c076443c162853572f3a8b8390bb8065357d47233b5bcc787eea4e7c87d0941`.

```rust
impl RangeCheckInstructions<F, AssignedNative<F>> for ZkStdLib {
    fn assign_lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<F>,
        bound: &BigUint,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.assign_lower_than_fixed(layouter, value, bound)
    }

    fn assert_lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        bound: &BigUint,
    ) -> Result<(), Error> {
        self.native_gadget.assert_lower_than_fixed(layouter, x, bound)
    }
}

```


## Native strict bound implementation

`circuits/src/field/native/native_gadget.rs:274-306`; SHA256 `770717b4198d80615235d53141859f9b7ddb88c2f52e3ebb7af7d3cdd0c35c04`.

```rust
    fn assert_lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        bound: &BigUint,
    ) -> Result<(), Error> {
        if let Some(current_bound) = self.constrained_cells.borrow().get(x)
            && current_bound <= bound
        {
            return Ok(());
        }
        self.update_bound(x, bound.clone());

        // compute largest k such that 2^k <= bound
        let k = (bound.bits() - 1) as usize;
        let two_pow_k = BigUint::from(1u8) << k; // 2^k

        // if the bound is a power of 2, the check is easier
        if two_pow_k == *bound {
            return self.core_decomposition_chip.assert_less_than_pow2(layouter, x, k);
        }

        // b := x in [0, 2^k)
        let b_value = x.value().map(|x| x.to_biguint() < two_pow_k);
        let b: AssignedBit<F> = self.assign(layouter, b_value)?;

        let diff: F = big_to_fe(bound - two_pow_k);

        // x in [0, bound) <=> x in [0, 2^k) or (x - diff) in [0, 2^k)
        let shifted_x = self.add_constant(layouter, x, -diff)?;
        let y = self.select(layouter, &b, x, &shifted_x)?;
        self.core_decomposition_chip.assert_less_than_pow2(layouter, &y, k)
    }
```


## Native select meaning

`circuits/src/field/native/native_chip.rs:1523-1550`; SHA256 `f2da7a69c32dbaf46467cff26eaf699814bb5b954409e34a762c39728f818695`.

```rust
impl<F> ControlFlowInstructions<F, AssignedNative<F>> for NativeChip<F>
where
    F: CircuitField + From<u64> + Neg<Output = F>,
{
    fn select(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        x: &AssignedNative<F>,
        y: &AssignedNative<F>,
    ) -> Result<AssignedNative<F>, Error> {
        // Return bit * x + (1 - bit) * y.

        // 0*bit + 0*x + 1*y + 0 + bit*x - bit*y
        self.add_and_double_mul(
            layouter,
            (F::ZERO, &cond.0),
            (F::ZERO, x),
            (F::ONE, y),
            F::ZERO,
            (F::ONE, -F::ONE),
        )
    }

    fn cond_swap(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
```


## Native equality bit constraints

`circuits/src/field/native/native_chip.rs:1377-1421`; SHA256 `f2da7a69c32dbaf46467cff26eaf699814bb5b954409e34a762c39728f818695`.

```rust
    fn is_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        c: F,
    ) -> Result<AssignedBit<F>, Error> {
        // We enforce (i) (x - c) * aux = 1 - res
        // and       (ii) (x - c) * res = 0.
        //  * If  x = c, (i)  implies res = 1; (ii) becomes trivial.
        //  * If x != c, (ii) implies res = 0; (i) can be relaxed with aux = (x - c)^-1.

        let value_cols = &self.config.value_cols;
        let res_val = x.value().map(|x| F::from((*x == c) as u64));
        let aux_val = x.value().map(|x| (*x - c).invert().unwrap_or(F::ONE));

        // (i) enforced as res - c * aux + aux * x - 1 = 0.
        let res = layouter.assign_region(
            || "is_equal (i)",
            |mut region| {
                region.assign_advice(|| "aux", value_cols[0], 0, || aux_val)?;
                self.copy_in_row(&mut region, x, &value_cols[1], 0)?;
                let res = region.assign_advice(|| "res", value_cols[4], 0, || res_val)?;
                let mut coeffs = vec![F::ZERO; self.config.coeff_cols.len()];
                coeffs[0] = -c; // coeff of aux
                coeffs[4] = F::ONE; // coeff of res
                self.custom(&mut region, &coeffs, F::ZERO, (F::ONE, F::ZERO), -F::ONE, 0)?;
                Ok(res)
            },
        )?;

        // (ii) enforced as -c * res + res * x = 0.
        let must_be_zero = self.add_and_mul(
            layouter,
            (-c, &res),
            (F::ZERO, x),
            (F::ZERO, x),
            F::ZERO,
            F::ONE,
        )?;
        self.assert_zero(layouter, &must_be_zero)?;

        // The two equations we have enforced guarantee the bit-ness of `res`.
        Ok(AssignedBit(res))
    }

```


## Fixed constant assignment

`circuits/src/field/native/native_chip.rs:578-605`; SHA256 `f2da7a69c32dbaf46467cff26eaf699814bb5b954409e34a762c39728f818695`.

```rust
    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        constant: F,
    ) -> Result<AssignedNative<F>, Error> {
        let constant_big = constant.to_biguint();
        if let Some(assigned) = self.cached_fixed.borrow().get(&constant_big) {
            return Ok(assigned.clone());
        };

        let x = layouter.assign_region(
            || "Assign fixed",
            |mut region| {
                let mut x = region.assign_fixed(
                    || "fixed",
                    self.config.fixed_values_col,
                    0,
                    || Value::known(constant),
                )?;
                // This is hacky but necessary because we are treating a fixed cell as an
                // assigned one. Fixed cells do not get properly filled until the end.
                x.update_value(constant)?;
                Ok(x)
            },
        )?;

        // Save the assigned constant in the cache.
        self.cached_fixed.borrow_mut().insert(constant_big.clone(), x.clone());
```


## Decomposition copy equality

`circuits/src/field/decomposition/instructions.rs:48-68`; SHA256 `21a89d39c6a4f845f12e2f0d52b7eee7d5443172c3e96775b561bd9ffaf28595`.

```rust
    fn assign_less_than_pow2(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<F>,
        bit_length: usize,
    ) -> Result<AssignedNative<F>, Error>;

    /// Function that guarantees that x < 2^{bit_length}
    fn assert_less_than_pow2(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        bit_length: usize,
    ) -> Result<(), Error> {
        let y = self.assign_less_than_pow2(layouter, x.value().copied(), bit_length)?;
        layouter.assign_region(
            || "copy",
            |mut region| region.constrain_equal(x.cell(), y.cell()),
        )
    }

```


## Decomposition power-of-two assignment

`circuits/src/field/decomposition/chip.rs:405-429`; SHA256 `c4238414c3583ca67a9a8f615758175fe76aa39f4bfc86dafe48bb3e073fdd97`.

```rust
    fn assign_less_than_pow2(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<F>,
        bit_length: usize,
    ) -> Result<AssignedNative<F>, Error> {
        #[cfg(not(test))]
        assert!((bit_length as u32) < F::NUM_BITS);

        // 1. get the limb sizes that minimize the number of parallel lookup rangechecks
        // should never panic since the HashMap contains all possible solutions
        let mut optimal_limb_sizes = self.opt_limbs.get(&(bit_length as i32)).unwrap().clone();

        // 2. process them by adding 0 terms in non-full rows
        optimal_limb_sizes
            .iter_mut()
            .for_each(|row| process_limb_sizes(self.pow2range_chip.config().val_cols.len(), row));
        let limb_sizes = optimal_limb_sizes.concat();

        // 3. use decompose_core to compute the result
        let (y, _) = self.decompose_core(layouter, value, &limb_sizes)?;

        Ok(y)
    }

```


## Decomposition limb assignment

`circuits/src/field/decomposition/chip.rs:248-283`; SHA256 `c4238414c3583ca67a9a8f615758175fe76aa39f4bfc86dafe48bb3e073fdd97`.

```rust
            || "decompose core",
            |mut region| {
                let mut offset = 0;

                // compute the range_check tags for each column
                let tags = limb_sizes.chunks(nr_pow2range_cols).map(|x| x[0]).collect::<Vec<_>>();

                // compute the linear combination terms, i.e. (coef, limb) pairs
                // by convention the coefficient of a zero sized limb is 0 so no constraint
                // needs to be imposed in the corresponding limb
                let coefficients = variable_limbsize_coefficients::<F>(limb_sizes);
                let limbs = x
                    .map(|x_value| decompose_in_variable_limbsizes(&x_value, limb_sizes))
                    .transpose_vec(limb_sizes.len());

                // we create the terms for the linear combination.
                let terms = coefficients.into_iter().zip(limbs.iter().copied()).collect::<Vec<_>>();

                // assign terms for linear combination
                let native_chip = self.native_chip();
                let (assigned_limbs, assigned_result) = native_chip.assign_linear_combination_aux(
                    &mut region,
                    terms.as_slice(),
                    F::ZERO,
                    &x,
                    nr_pow2range_cols,
                    &mut offset,
                )?;
                offset += 1;

                // enable the appropriate copy constraints in the rows where we assigned the
                // linear combination terms
                let pow2range_chip = self.pow2range_chip();

                // we reverse since we do the range-checks from higher end to start
                for (i, tag) in tags.into_iter().rev().enumerate() {
```


## Known witness decomposition boundary

`circuits/src/field/decomposition/cpu_utils.rs:30-71`; SHA256 `3473a154838bfde80aa1f6c0f030129d96315561f0c8fa96bf063fdbbf179860`.

```rust
/// # Panics
///
/// If the field element cannot be represented with limb_sizes
pub(crate) fn decompose_in_variable_limbsizes<InF: CircuitField, OutF: CircuitField>(
    x: &InF,
    limb_sizes: &[usize],
) -> Vec<OutF> {
    // convert the given number to bigint for efficient bitwise operations
    let x: BigInt = x.to_biguint().into();

    // vector to keep the result
    let mut limbs: Vec<OutF> = Vec::with_capacity(limb_sizes.len());

    // each time we shift the mask to "extract" the correct bits. This variable
    // holds the next shift
    let mut shift = 0;

    for limb_size in limb_sizes {
        // compute the mask vector i.e. 111..1 (limb_size ones)
        // NOTE: when the limb_size is 0 he mask will always be 0
        let mask_bits: u64 = (1 << limb_size) - 1;
        let mask = BigInt::from(mask_bits);

        // right shift the number and perform an and operation to take the limb
        let limb_int = (x.clone() >> shift) & mask;
        let limb = bigint_to_fe(&limb_int);
        limbs.push(limb);

        // update shift to get the next limb
        shift += limb_size;
    }

    // sanity check. Panics if the limbs are not enough to represent the number
    #[cfg(not(test))]
    debug_assert_eq!(
        x.clone() >> shift,
        0.into(),
        "Decomposition Chip: the integer cannot be represented with the given limb_sizes"
    );

    limbs
}
```


## Linear combination source

`circuits/src/field/native/native_chip.rs:425-478`; SHA256 `f2da7a69c32dbaf46467cff26eaf699814bb5b954409e34a762c39728f818695`.

```rust
    pub(crate) fn assign_linear_combination_aux(
        &self,
        region: &mut Region<'_, F>,
        terms: &[(F, Value<F>)],
        constant: F,
        result: &Value<F>,
        cols_used: usize,
        offset: &mut usize,
    ) -> Result<(Vec<AssignedNative<F>>, AssignedNative<F>), Error> {
        assert!(cols_used < self.config.value_cols.len());

        // If |terms| <= cols_used, we assert the relation in one row.
        // Otherwise we consume up to `cols_used` terms to reduce to a linear
        // combination of smaller size.
        let chunk_len = min(terms.len(), cols_used);

        // Initialize the coefficients vector
        let mut coeffs = vec![F::ZERO; self.config.coeff_cols.len()];

        // assign the lc result in the first advice column
        let assigned_result = region.assign_advice(
            || "assign linear combination term",
            self.config.value_cols[0],
            *offset,
            || *result,
        )?;
        coeffs[0] = -F::ONE;

        // assign the first `chunk_len` terms values in the current row
        let mut assigned_limbs = terms[0..chunk_len]
            .iter()
            .enumerate()
            .map(|(i, term)| {
                coeffs[i + 1] = term.0;
                region.assign_advice(
                    || "assign linear combination term",
                    self.config.value_cols[i + 1],
                    *offset,
                    || term.1,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        // If everything fits in this row, we add the constraint in the current row and
        // finish.
        if terms.len() <= cols_used {
            self.custom(
                region,
                &coeffs,
                F::ZERO,
                (F::ZERO, F::ZERO),
                constant,
                *offset,
            )?;
```


## Queried-tag lookup loading

`circuits/src/field/decomposition/pow2range.rs:205-229`; SHA256 `9a114628d51d90ce1952feb453f122cffa381babba3459fe0840e68081d17044`.

```rust
    /// Load the pow2range lookup table (to be used in synthesis).
    pub fn load_table(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_table(
            || "pow2range table",
            |mut table| {
                let mut offset = 0;
                for bit_len in 0..=self.max_bit_len {
                    // The lookup is disabled with tag 0, which we always include.
                    if bit_len > 0 && !self.queried_tags.borrow().contains(&bit_len) {
                        continue;
                    }
                    let tag = Value::known(F::from(bit_len as u64));
                    for value in 0..(1 << bit_len) {
                        let val = Value::known(F::from(value));
                        table.assign_cell(|| "t_tag", self.config.t_tag, offset, || tag)?;
                        table.assign_cell(|| "t_val", self.config.t_val, offset, || val)?;
                        offset += 1;
                    }
                }
                Ok(())
            },
        )
    }
}

```


## Circuit k and unknown witnesses

`zk_stdlib/src/interface.rs:55-80`; SHA256 `f679793a7ab046a01bca3612247bb7b14c8c56273e3056f84baa51b8980d89e5`.

```rust
}

impl<'a, R: Relation> MidnightCircuit<'a, R> {
    /// A MidnightCircuit with unknown instance-witness for the given relation.
    /// `k` is the log2 of the circuit size (i.e. the circuit has `2^k` rows).
    /// If `k` is `None`, the optimal value is computed automatically.
    pub fn from_relation(relation: &'a R, k: Option<u32>) -> Self {
        MidnightCircuit::new(relation, Value::unknown(), Value::unknown(), k)
    }

    /// Creates a new MidnightCircuit for the given relation.
    /// `k` is the log2 of the circuit size (i.e. the circuit has `2^k` rows).
    /// If `k` is `None`, the optimal value is computed automatically.
    pub fn new(
        relation: &'a R,
        instance: Value<R::Instance>,
        witness: Value<R::Witness>,
        k: Option<u32>,
    ) -> Self {
        let k = k.unwrap_or_else(|| optimal_k(relation));
        MidnightCircuit {
            relation,
            k,
            instance,
            witness,
            nb_public_inputs: Rc::new(RefCell::new(None)),
```


## Circuit params

`zk_stdlib/src/interface.rs:396-414`; SHA256 `f679793a7ab046a01bca3612247bb7b14c8c56273e3056f84baa51b8980d89e5`.

```rust
    type Params = (ZkStdLibArch, u8);

    fn without_witnesses(&self) -> Self {
        unreachable!()
    }

    fn params(&self) -> Self::Params {
        (self.relation.used_chips(), (self.k - 1) as u8)
    }

    fn configure_with_params(
        meta: &mut ConstraintSystem<F>,
        params: (ZkStdLibArch, u8),
    ) -> Self::Config {
        ZkStdLib::configure(meta, params)
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        ZkStdLib::configure(meta, (ZkStdLibArch::default(), 8))
```


## VK and PK keygen unknown witnesses

`zk_stdlib/src/interface.rs:494-526`; SHA256 `f679793a7ab046a01bca3612247bb7b14c8c56273e3056f84baa51b8980d89e5`.

```rust
pub fn setup_vk<R: Relation>(
    params: &ParamsKZG<midnight_curves::Bls12>,
    relation: &R,
) -> MidnightVK {
    let k = params.max_k();
    let circuit = MidnightCircuit::from_relation(relation, Some(k));
    let vk = keygen_vk_with_k(params, &circuit, k).expect("keygen_vk should not fail");

    // During the call to [setup_vk] the circuit RefCell on public inputs has been
    // mutated with the correct value. The following [unwrap] is safe here.
    let nb_public_inputs = circuit.nb_public_inputs.clone().borrow().unwrap();

    MidnightVK {
        architecture: relation.used_chips(),
        k: circuit.k as u8,
        nb_public_inputs,
        vk,
    }
}

/// Generates a proving key for a `MidnightCircuit<R>` circuit.
pub fn setup_pk<R: Relation>(relation: &R, vk: &MidnightVK) -> MidnightPK<R> {
    let circuit = MidnightCircuit::new(
        relation,
        Value::unknown(),
        Value::unknown(),
        Some(vk.k() as u32),
    );
    let pk = BlstPLONK::<MidnightCircuit<R>>::setup_pk(&circuit, &vk.vk);
    MidnightPK {
        k: vk.k(),
        relation: relation.clone(),
        pk,
```

## IVC module exports

`aggregation/src/ivc/mod.rs:19-48`; SHA256 `ee417bc43b5d4febdb93fb347acb2ddc888d0b4546b82f3caa1867f3bd0c4017`.

```rust
//! the chain length is relevant, it can be tracked by including a counter in
//! the state that the transition function increments at each step.

pub use circuit::{IvcCircuit, IvcInstance, IvcWitness};
pub use error::IvcError;
use midnight_circuits::{
    instructions::{BinaryInstructions, EqualityInstructions},
    types::{AssignedBit, AssignedNative},
    verifier::{BlstrsEmulation, SelfEmulation},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{ZkStdLib, ZkStdLibArch};
pub use prover::IvcProver;
pub use setup::setup;
pub use verifier::IvcVerifier;

pub(crate) type S = BlstrsEmulation;
pub(crate) type F = <S as SelfEmulation>::F;
pub(crate) type C = <S as SelfEmulation>::C;
pub(crate) type E = <S as SelfEmulation>::Engine;

pub mod circuit;
pub mod error;
pub mod prover;
pub mod setup;
pub mod verifier;

```
