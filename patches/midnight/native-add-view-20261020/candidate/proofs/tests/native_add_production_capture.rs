//! Bounded production-path capture regression for `NativeChip`.
//!
//! This test holds recorder guards across the real
//! `keygen_vk_with_k` and `create_proof` calls. It checks only the
//! structural capture relation implemented by
//! `join_linear_combination_view`.
//!
//! It does not check gate satisfaction, copy satisfaction, verifier
//! acceptance, same-key authentication, circuit authentication, the
//! actual add coefficient pattern, or any canonical field map.
//!
//! `ParamsKZG::unsafe_setup` is used only because this is a bounded
//! test. Its documentation states that it MUST NOT be used in
//! production parameter generation.

use blake2b_simd::State;
use ff::Field;
use midnight_circuits::{
    field::{
        decomposition::{
            chip::P2RDecompositionConfig, pow2range::Pow2RangeChip,
        },
        native::{NB_ARITH_COLS, NB_ARITH_FIXED_COLS},
        NativeChip,
    },
    instructions::*,
    types::ComposableChip,
};
use midnight_curves::{Bls12, Fq as Scalar};
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        create_proof, keygen_pk, keygen_vk_with_k, native_add_view::{
            begin_recording, join_linear_combination_view,
            take_last_keygen_recording, take_last_proving_recording,
            CapturePhase,
        }, Circuit, ConstraintSystem, Error,
    },
    poly::kzg::{params::ParamsKZG, KZGCommitmentScheme},
    transcript::{CircuitTranscript, Transcript},
};
use rand_core::OsRng;

const K: u32 = 6;
const NUM_ROWS: usize = 1usize << K;

#[derive(Clone)]
struct NativeAddCircuit {
    x: Scalar,
    y: Scalar,
}

impl Circuit<Scalar> for NativeAddCircuit {
    type Config = P2RDecompositionConfig;
    type FloorPlanner = SimpleFloorPlanner;

    #[cfg(feature = "circuit-params")]
    type Params = ();

    fn without_witnesses(&self) -> Self {
        Self {
            x: Scalar::ZERO,
            y: Scalar::ZERO,
        }
    }

    fn configure(meta: &mut ConstraintSystem<Scalar>) -> Self::Config {
        let advice_columns: [_; NB_ARITH_COLS] =
            core::array::from_fn(|_| meta.advice_column());

        let fixed_columns: [_; NB_ARITH_FIXED_COLS] =
            core::array::from_fn(|_| meta.fixed_column());

        let committed_instance_column = meta.instance_column();
        let instance_column = meta.instance_column();

        let native_config = NativeChip::configure(
            meta,
            &(
                advice_columns,
                fixed_columns,
                [committed_instance_column, instance_column],
            ),
        );

        // This config container is used only to obtain the recorded
        // `NativeConfig`. The decomposition chip is not synthesized,
        // so it creates no additional synthesis regions in this test.
        let pow2range_config =
            Pow2RangeChip::configure(meta, &advice_columns[1..=4]);

        P2RDecompositionConfig::new(&native_config, &pow2range_config)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Scalar>,
    ) -> Result<(), Error> {
        let chip = NativeChip::new(config.native_config(), &());

        let x = chip.assign(&mut layouter, Value::known(self.x))?;
        let y = chip.assign(&mut layouter, Value::known(self.y))?;

        // Two non-filtered terms enter the ordinary multi-term
        // `linear_combination` path. No fixed-result equality is
        // asserted here: this test records structural synthesis
        // data and does not create a desired-sum certificate.
        let _result = chip.add(&mut layouter, &x, &y)?;

        Ok(())
    }
}

#[test]
fn native_add_capture_across_real_keygen_and_proving() {
    let circuit = NativeAddCircuit {
        x: Scalar::from(2u64),
        y: Scalar::from(3u64),
    };

    // Bounded test parameters only. This is not a production setup.
    let params = ParamsKZG::<Bls12>::unsafe_setup(K, OsRng);

    // Real keygen synthesis through:
    //
    //   Circuit::configure
    //     -> NativeChip::configure recording hook
    //   SimpleFloorPlanner
    //     -> SingleChipLayouter recording hook
    //   keygen Assembly
    //     -> fixed, selector, dimension, and permutation hooks
    let vk = {
        let _guard = begin_recording(CapturePhase::Keygen, 1);

        let vk = keygen_vk_with_k::<
            Scalar,
            KZGCommitmentScheme<Bls12>,
            NativeAddCircuit,
        >(&params, &circuit, K)
        .expect("keygen_vk_with_k should succeed");

        drop(_guard);
        vk
    };

    let keygen_session = take_last_keygen_recording()
        .expect("real keygen synthesis should publish a recording");

    assert_eq!(keygen_session.declared_phase, CapturePhase::Keygen);
    assert_eq!(keygen_session.declared_circuit_count, 1);
    assert_eq!(keygen_session.k, Some(K));
    assert_eq!(keygen_session.num_rows, Some(NUM_ROWS));
    assert_eq!(keygen_session.config_bindings.len(), 1);
    assert!(!keygen_session.fixed_values.is_empty());
    assert!(!keygen_session.selectors.is_empty());
    assert!(!keygen_session.converted_selector_fixed.is_empty());
    assert!(!keygen_session.permutation_columns.is_empty());
    assert!(!keygen_session.copy_pairs.is_empty());

    let pk = keygen_pk(vk, &circuit).expect("keygen_pk should succeed");

    let empty_instance_column: &[Scalar] = &[];
    let instances: &[&[Scalar]] =
        &[empty_instance_column, empty_instance_column];

    let mut transcript = CircuitTranscript::<State>::init();

    // Real proving synthesis through:
    //
    //   Circuit::configure
    //     -> NativeChip::configure recording hook
    //   SimpleFloorPlanner
    //     -> SingleChipLayouter recording hook
    //   parse_advices / WitnessCollection
    //     -> dimension and advice recording hooks
    {
        let _guard = begin_recording(CapturePhase::Proving, 1);

        create_proof::<Scalar, KZGCommitmentScheme<Bls12>, _, _>(
            &params,
            &pk,
            &[circuit],
            #[cfg(feature = "committed-instances")]
            0,
            &[instances],
            &mut transcript,
            OsRng,
        )
        .expect("create_proof should succeed");

        drop(_guard);
    }

    let proof = transcript.finalize();
    assert!(!proof.is_empty());

    let proving_session = take_last_proving_recording()
        .expect("real proving synthesis should publish a recording");

    assert_eq!(proving_session.declared_phase, CapturePhase::Proving);
    assert_eq!(proving_session.declared_circuit_count, 1);
    assert_eq!(proving_session.k, Some(K));
    assert_eq!(proving_session.num_rows, Some(NUM_ROWS));
    assert_eq!(proving_session.config_bindings.len(), 1);
    assert!(!proving_session.advice_values.is_empty());

    let view = join_linear_combination_view(keygen_session, proving_session)
        .expect("real keygen and proving captures should join structurally");

    assert_eq!(view.k, K);
    assert_eq!(view.num_rows, NUM_ROWS);
    assert_eq!(view.add_row, view.linear_combination_region_start);
    assert_eq!(view.value_cells.len(), NB_ARITH_COLS);
    assert_eq!(view.coeff_cells.len(), NB_ARITH_COLS);

    // Five coefficients, constant, q_next, mul_ab, mul_ac, and the
    // converted q_arith cell.
    assert_eq!(view.gate_fixed_encodings.len(), 10);
    assert!(view.q_arith_enabled);
    assert!(view.zero_cell_encoding.iter().all(|byte| *byte == 0));
}
