//! Fresh Groth16 setup for exactly one compiled SettlementHashV2 relation.
//!
//! Trust profile `single-party-test-setup/1`. The randomness source is the caller's RNG; the
//! CLI passes only `rand_core::OsRng`. ark-groth16 samples alpha, beta, gamma, delta, the
//! evaluation point and the generators inside `generate_random_parameters_with_reduction`
//! (`generator.rs:19-44`) and drops them on return without zeroizing. This module never sees,
//! stores or exports them. Never call this with a seeded or test RNG outside a test that
//! discards the keys: a known seed is known toxic waste.
//!
//! K4H successor: the circuit handed to ark-groth16 is wrapped in [`DigestBound`]. The
//! generator synthesizes it into its own constraint system (`generator.rs:63-73`); the wrapper
//! finalizes that system, hashes the exact matrices the QAP reduction will read and refuses
//! unless they have the descriptor's `r1csSha256`. The setup process therefore observes the
//! VK-to-matrix link itself. No consumer can check that link from a VK; the setup record still
//! only declares it.
//!
//! Setup does not prove or verify anything. Prove and verify are separate entry points.
use crate::encoding;
use crate::error::DriverError;
use crate::export::{self, CURVE, ENCODING, SETUP_FORMAT, SETUP_NOTICE, SNARK, SetupRecord, TRUST_PROFILE};
use crate::hexutil;
use crate::relation::{self, SelectedRelation};
use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use mori_r1cs::{Tamper, TransitionCircuit};
use rand_core::{CryptoRng, RngCore};
use std::cell::RefCell;

/// Public record bytes and custody-only proving-key bytes, kept apart.
#[derive(Debug, Clone)]
pub struct SetupOutput {
    pub record: SetupRecord,
    /// Exact bytes to write as the public setup record.
    pub record_bytes: Vec<u8>,
    /// Exact bytes to write as the proving key (custody material, not a public export).
    pub proving_key_bytes: Vec<u8>,
    /// `mori-r1cs-matrices/1` digest of the matrices this setup reduced (equal to the
    /// descriptor's `r1csSha256`, or setup refuses).
    pub observed_r1cs_sha256: String,
}

/// Setup-mode circuit that binds the matrices ark-groth16 reduces to an expected digest.
///
/// `generate_constraints` synthesizes the relation's own circuit into the generator's
/// constraint system, finalizes it, digests `to_matrices()` and records the digest in
/// `observed`; a different digest is `SynthesisError::Unsatisfiable`, so no key is produced.
/// The generator finalizes again afterwards; this driver's constraints carry only concrete
/// variables, so the second inlining leaves the matrices unchanged (UNVERIFIED-LIB reading of
/// ark-relations 0.4, observed by `hostile_tier` h11 without keys).
pub struct DigestBound<'r, 'o> {
    inner: TransitionCircuit<'r, Fr>,
    expected: String,
    observed: &'o RefCell<Option<String>>,
}

impl<'r, 'o> DigestBound<'r, 'o> {
    pub fn new(inner: TransitionCircuit<'r, Fr>, expected: impl Into<String>, observed: &'o RefCell<Option<String>>) -> Self {
        Self { inner, expected: expected.into(), observed }
    }
}

impl ConstraintSynthesizer<Fr> for DigestBound<'_, '_> {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        self.inner.synthesize_into(cs.clone())?;
        cs.finalize();
        let matrices = cs.to_matrices().ok_or(SynthesisError::Unsatisfiable)?;
        let digest = relation::matrix_digest(&matrices).hex();
        drop(matrices);
        let same = digest == self.expected;
        *self.observed.borrow_mut() = Some(digest);
        if same { Ok(()) } else { Err(SynthesisError::Unsatisfiable) }
    }
}

/// Runs one setup over the relation's own setup-mode circuit (`circuit_v2(None, None, _)`).
pub fn setup<R: RngCore + CryptoRng>(selected: &SelectedRelation, rng: &mut R) -> Result<SetupOutput, DriverError> {
    let relation = selected.relation();
    let probe = relation.probe();
    let expected = selected.descriptor().r1cs_sha256.clone();
    let observed = RefCell::new(None);
    let circuit = DigestBound::new(
        relation.circuit_v2::<Fr>(None, None, Tamper::none()).map_err(DriverError::Compile)?,
        expected.clone(),
        &observed,
    );
    let pk = Groth16::<Bls12_381>::generate_random_parameters_with_reduction(circuit, rng).map_err(|e| {
        let seen = observed.borrow().clone().unwrap_or_else(|| "none".into());
        DriverError::Backend(format!("Groth16 setup: {e}; matrices observed in setup: {seen}; descriptor: {expected}"))
    })?;
    let observed_r1cs_sha256 = observed.into_inner().ok_or_else(|| DriverError::Backend("setup reduced no observed matrices".into()))?;
    if observed_r1cs_sha256 != expected {
        return Err(DriverError::mismatch("r1cs matrices reduced by setup", expected, observed_r1cs_sha256));
    }

    // Structural agreement between the parameters and the compiler's probe of the same
    // circuit (ark-groth16 `generator.rs:96-125`, `r1cs_to_qap.rs:122-126`).
    let instance = probe.instance_variables;
    let witness = probe.witness_variables;
    let domain = probe.constraints.checked_add(instance).ok_or_else(|| DriverError::Backend("domain size overflow".into()))?;
    let checks = [
        ("vk IC points", pk.vk.gamma_abc_g1.len(), instance),
        ("a_query length", pk.a_query.len(), instance + witness),
        ("b_g1_query length", pk.b_g1_query.len(), instance + witness),
        ("b_g2_query length", pk.b_g2_query.len(), instance + witness),
        ("l_query length", pk.l_query.len(), witness),
    ];
    for (what, got, want) in checks {
        if got != want {
            return Err(DriverError::mismatch(format!("setup {what}"), want.to_string(), got.to_string()));
        }
    }
    let h = pk.h_query.len() + 1;
    if !h.is_power_of_two() || h < domain {
        return Err(DriverError::Backend(format!("setup h_query length {} does not fit the domain {domain}", h - 1)));
    }
    encoding::vk_profile_ok(&pk.vk)?;

    let vk_bytes = encoding::encode_vk(&pk.vk)?;
    let proving_key_bytes = encoding::encode_proving_key(&pk)?;
    drop(pk);
    let record = SetupRecord {
        format: SETUP_FORMAT.into(),
        trust_profile: TRUST_PROFILE.into(),
        notice: SETUP_NOTICE.into(),
        relation: selected.descriptor().clone(),
        curve: CURVE.into(),
        snark: SNARK.into(),
        encoding: ENCODING.into(),
        verifying_key_hex: hexutil::encode(&vk_bytes),
        vk_digest_blake2b256: hexutil::encode(&mori_r1cs::blake2b::blake2b256(&vk_bytes)),
        proving_key_sha256: mori_r1cs::identity::digest_bytes(&proving_key_bytes).hex(),
        proving_key_bytes: proving_key_bytes.len() as u64,
        driver: crate::DRIVER_ID.into(),
    };
    let record_bytes = export::document_bytes(&record)?;
    // The record must pass the same strict parser a verifier uses.
    let parsed = export::parse_setup_record(&record_bytes)?;
    if parsed.record != record {
        return Err(DriverError::Backend("setup record does not round-trip through its parser".into()));
    }
    Ok(SetupOutput { record, record_bytes, proving_key_bytes, observed_r1cs_sha256 })
}
