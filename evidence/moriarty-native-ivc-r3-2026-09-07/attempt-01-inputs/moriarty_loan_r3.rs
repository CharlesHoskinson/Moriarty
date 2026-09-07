//! Fixed-instance R2 first-period loan IVC, not full Moriarty HistoryCompliance.
//! The circuit specializes to exactly two transitions and three complete public
//! states. SHA256 preimages/effects are pinned by the independently reviewed
//! fixture; SHA256 and dynamic authority are NOT recomputed in this circuit.

use ff::{Field, PrimeField};
use midnight_aggregation::ivc::{self, IvcCircuit, IvcContext, IvcIO, IvcState, IvcTransition};
use midnight_circuits::{
    instructions::*,
    types::{AssignedBit, AssignedNative},
    verifier::{Accumulator, BlstrsEmulation, SelfEmulation},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::MockProver,
    plonk::Error,
    poly::{
        commitment::{Params, PolynomialCommitmentScheme},
        kzg::{KZGCommitmentScheme, params::ParamsKZG},
    },
    utils::SerdeFormat,
};
use midnight_zk_stdlib::{MidnightCircuit, Relation, ZkStdLib, ZkStdLibArch};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{self, Cursor, Read, Write},
    path::Path,
    time::Instant,
};

#[path = "moriarty_r3/episode.rs"]
mod episode;

type S = BlstrsEmulation;
type F = <S as SelfEmulation>::F;
type E = <S as SelfEmulation>::Engine;
const K: u32 = 17;
const LIMBS: usize = episode::NUM_FIELDS * 2 + episode::NUM_DIGESTS * 4;
const PREDICATE: &str = "R2-fixed-first-period-loan-two-transition-IVC-with-fixed-local-authority";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    financial: [u128; episode::NUM_FIELDS],
    digests: [[u64; 4]; episode::NUM_DIGESTS],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    immutable: [[u64; 4]; 4],
}
#[derive(Clone, Debug)]
pub struct AssignedState(Vec<AssignedNative<F>>);
#[derive(Clone, Debug)]
pub struct Loan {
    std_lib: ZkStdLib,
    ctx: Context,
}

fn fixed_context() -> Context {
    Context {
        immutable: episode::DIGESTS[0][..4].try_into().unwrap(),
    }
}

// Separate checked arithmetic, with values read from the R2 package's fixed
// terms, not the exported expected financial array. Every intermediate is u128.
fn native_financial() -> [[u128; episode::NUM_FIELDS]; 3] {
    let genesis = [0, 2, 5_000_000_000, 0, 0, 0, 0, 20_000_000_000, 0, 0, 0];
    let mut accrued = genesis;
    let principal = 500_000_000u128;
    let interest = genesis[2]
        .checked_mul(8)
        .unwrap()
        .checked_mul(31)
        .unwrap()
        .checked_div(100u128.checked_mul(365).unwrap())
        .unwrap();
    assert_eq!(interest, 33_972_602);
    accrued[0] = genesis[0].checked_add(1).unwrap();
    accrued[1] = genesis[1].checked_sub(1).unwrap();
    accrued[2] = genesis[2].checked_sub(principal).unwrap();
    accrued[3] = principal;
    accrued[4] = interest;
    accrued[9] = 1;
    let mut settled = accrued;
    let total = accrued[3].checked_add(accrued[4]).unwrap();
    assert_eq!(total, 533_972_602);
    // Fixed cumulative gross authority cap; no refunds/intermediate recipients.
    assert!(total <= 533_972_602);
    settled[0] = accrued[0].checked_add(1).unwrap();
    settled[1] = accrued[1].checked_sub(1).unwrap();
    settled[5] = accrued[5].checked_add(accrued[3]).unwrap();
    settled[6] = accrued[6].checked_add(accrued[4]).unwrap();
    settled[7] = accrued[7].checked_sub(total).unwrap();
    settled[8] = accrued[8].checked_add(total).unwrap();
    settled[3] = 0;
    settled[4] = 0;
    settled[9] = 2;
    settled[10] = 1;
    [genesis, accrued, settled]
}
fn fixed_state(index: usize) -> State {
    State {
        financial: native_financial()[index],
        digests: episode::DIGESTS[index],
    }
}
fn state_index(ctx: &Context, state: &State) -> Option<usize> {
    if ctx != &fixed_context() {
        return None;
    }
    (0..3).find(|&i| *state == fixed_state(i))
}
fn checked_transition(ctx: &Context, state: &State) -> Result<State, &'static str> {
    match state_index(ctx, state) {
        Some(i @ 0..=1) => Ok(fixed_state(i + 1)),
        Some(_) => Err("closed episode has no ordinary successor"),
        None => Err("state or context outside fixed relation"),
    }
}
fn wire(state: &State) -> Vec<u64> {
    state
        .financial
        .iter()
        .flat_map(|&v| [v as u64, (v >> 64) as u64])
        .chain(state.digests.iter().flat_map(|d| d.iter().copied()))
        .collect()
}

impl IvcContext for Loan {
    type Context = Context;
    fn new(std_lib: ZkStdLib, ctx: &Context) -> Self {
        assert_eq!(
            ctx,
            &fixed_context(),
            "unsupported fixed specialization context"
        );
        Self {
            std_lib,
            ctx: ctx.clone(),
        }
    }
    fn write_context<W: Write>(ctx: &Context, writer: &mut W) -> io::Result<()> {
        if ctx != &fixed_context() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "wrong context"));
        }
        for limb in ctx.immutable.iter().flatten() {
            writer.write_all(&limb.to_le_bytes())?;
        }
        Ok(())
    }
    fn read_context<R: Read>(reader: &mut R) -> io::Result<Context> {
        let mut ctx = Context {
            immutable: [[0; 4]; 4],
        };
        for limb in ctx.immutable.iter_mut().flatten() {
            let mut bytes = [0; 8];
            reader.read_exact(&mut bytes)?;
            *limb = u64::from_le_bytes(bytes);
        }
        if ctx != fixed_context() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "wrong context"));
        }
        Ok(ctx)
    }
}
impl IvcState for Loan {
    type State = State;
    type AssignedState = AssignedState;
    fn genesis(ctx: &Context) -> State {
        assert_eq!(ctx, &fixed_context());
        fixed_state(0)
    }
    fn decider(ctx: &Context, state: &State) -> bool {
        // State has no hidden full-data fields: every field is in the PI.
        // All fixed financial/digest values must match the reviewed specialization.
        state_index(ctx, state).is_some()
    }
}
impl Loan {
    fn assign_limbs(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<Vec<F>>,
    ) -> Result<AssignedState, Error> {
        let mut cells = Vec::with_capacity(LIMBS);
        for i in 0..LIMBS {
            let cell: AssignedNative<F> = self
                .std_lib
                .assign(layouter, value.as_ref().map(|v| v[i]))?;
            self.std_lib
                .assert_lower_than_fixed(layouter, &cell, &(1u128 << 64).into())?;
            cells.push(cell);
        }
        Ok(AssignedState(cells))
    }
}
impl IvcIO for Loan {
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<State>,
    ) -> Result<AssignedState, Error> {
        self.assign_limbs(layouter, value.as_ref().map(Self::format_public_input))
    }
    fn constrain_as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        state: &AssignedState,
    ) -> Result<(), Error> {
        for cell in &state.0 {
            self.std_lib.constrain_as_public_input(layouter, cell)?;
        }
        Ok(())
    }
    fn as_public_input(
        &self,
        _: &mut impl Layouter<F>,
        state: &AssignedState,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        Ok(state.0.clone())
    }
    fn format_public_input(state: &State) -> Vec<F> {
        wire(state).into_iter().map(F::from).collect()
    }
}
impl IvcTransition for Loan {
    type Witness = ();
    fn arch() -> ZkStdLibArch {
        ZkStdLibArch {
            nb_arith_cols: 9,
            nr_pow2range_cols: 8,
            ..ZkStdLibArch::default()
        }
    }
    fn transition(ctx: &Context, state: &State, _: ()) -> State {
        checked_transition(ctx, state).expect("invalid ordinary transition")
    }
    fn circuit_transition(
        &self,
        layouter: &mut impl Layouter<F>,
        state: &AssignedState,
        _: Value<()>,
    ) -> Result<AssignedState, Error> {
        assert_eq!(self.ctx, fixed_context());
        // Boolean selector is revision==0. Every before limb must equal the
        // corresponding fixed genesis/accrued limb; a closed state cannot fit.
        let genesis: AssignedBit<F> =
            self.std_lib
                .is_equal_to_fixed(layouter, &state.0[0], F::ZERO)?;
        let before0 = Self::format_public_input(&fixed_state(0));
        let before1 = Self::format_public_input(&fixed_state(1));
        let after1 = Self::format_public_input(&fixed_state(2));
        let mut next = Vec::with_capacity(LIMBS);
        for i in 0..LIMBS {
            let a: AssignedNative<F> = self.std_lib.assign_fixed(layouter, before0[i])?;
            let b: AssignedNative<F> = self.std_lib.assign_fixed(layouter, before1[i])?;
            let c: AssignedNative<F> = self.std_lib.assign_fixed(layouter, after1[i])?;
            let expected = self.std_lib.select(layouter, &genesis, &a, &b)?;
            self.std_lib
                .assert_equal(layouter, &state.0[i], &expected)?;
            let output = self.std_lib.select(layouter, &genesis, &b, &c)?;
            self.std_lib
                .assert_lower_than_fixed(layouter, &output, &(1u128 << 64).into())?;
            next.push(output);
        }
        Ok(AssignedState(next))
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
fn event(name: &str, stage: &str, outcome: &str) {
    println!(
        "{{\"event\":\"control\",\"name\":\"{name}\",\"stage\":\"{stage}\",\"outcome\":\"{outcome}\"}}"
    );
}
fn preflight_native() {
    assert!(
        F::NUM_BITS > 64,
        "field cannot injectively encode u64 limbs"
    );
    assert_eq!(
        native_financial(),
        episode::FINANCIAL,
        "independent calculation differs from R2 evaluator"
    );
    assert_eq!(episode::FIELD_NAMES.len(), 11);
    assert_eq!(episode::DIGEST_NAMES.len(), 8);
    for i in 0..3 {
        assert_eq!(&episode::DIGESTS[i][..4], &episode::DIGESTS[0][..4]);
        let s = fixed_state(i);
        assert!(Loan::decider(&fixed_context(), &s));
        assert_eq!(wire(&s).len(), LIMBS);
        for j in 0..episode::NUM_FIELDS {
            for bit in [0, 64] {
                let mut bad = s.clone();
                bad.financial[j] ^= 1u128 << bit;
                assert!(!Loan::decider(&fixed_context(), &bad));
                assert!(checked_transition(&fixed_context(), &bad).is_err());
                assert_ne!(
                    Loan::format_public_input(&s),
                    Loan::format_public_input(&bad)
                );
            }
        }
        for j in 0..episode::NUM_DIGESTS {
            for limb in 0..4 {
                let mut bad = s.clone();
                bad.digests[j][limb] ^= 1;
                assert!(!Loan::decider(&fixed_context(), &bad));
                assert!(checked_transition(&fixed_context(), &bad).is_err());
            }
        }
    }
    for n in [0, u64::MAX as u128, 1u128 << 64, u128::MAX] {
        let mut s = fixed_state(0);
        s.financial[0] = n;
        let encoded = wire(&s);
        assert_eq!(encoded[0] as u128 | ((encoded[1] as u128) << 64), n);
    }
    let mut bytes = Vec::new();
    Loan::write_context(&fixed_context(), &mut bytes).unwrap();
    assert_eq!(
        Loan::read_context(&mut Cursor::new(&bytes)).unwrap(),
        fixed_context()
    );
    bytes[0] ^= 1;
    assert!(Loan::read_context(&mut Cursor::new(&bytes)).is_err());
    for i in 0..2 {
        assert_eq!(
            checked_transition(&fixed_context(), &fixed_state(i)).unwrap(),
            fixed_state(i + 1)
        );
    }
    assert!(checked_transition(&fixed_context(), &fixed_state(2)).is_err());
    event(
        "independent_arithmetic_all_state_limb_mutations_context_and_closed",
        "native_host",
        "pass",
    );
}

// Four bounded application-only MockProver runs, not recursive proofs.
// These check both valid transitions, closed input, and the raw 2^64 limb bound.
#[derive(Clone)]
struct LocalRelation {
    before: Vec<F>,
    after: State,
    range_only: bool,
}
impl Relation for LocalRelation {
    type Instance = ();
    type Witness = ();
    type Error = Error;
    fn format_instance(_: &()) -> Result<Vec<F>, Error> {
        Ok(vec![])
    }
    fn used_chips(&self) -> ZkStdLibArch {
        Loan::arch()
    }
    fn circuit(
        &self,
        lib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        _: Value<()>,
        _: Value<()>,
    ) -> Result<(), Error> {
        let loan = Loan::new(lib.clone(), &fixed_context());
        let before = loan.assign_limbs(layouter, Value::known(self.before.clone()))?;
        if !self.range_only {
            let after = loan.circuit_transition(layouter, &before, Value::known(()))?;
            for (cell, constant) in after.0.iter().zip(Loan::format_public_input(&self.after)) {
                lib.assert_equal_to_fixed(layouter, cell, constant)?;
            }
        }
        Ok(())
    }
    fn write_relation<W: Write>(&self, _: &mut W) -> io::Result<()> {
        Err(io::Error::other("local test relation is not serialized"))
    }
    fn read_relation<R: Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("local test relation is not serialized"))
    }
}
fn preflight_circuit() {
    for case in 0..4 {
        let index = if case < 3 { case } else { 0 };
        let mut before = Loan::format_public_input(&fixed_state(index));
        if case == 3 {
            before[0] = F::from(u64::MAX) + F::ONE;
        }
        let relation = LocalRelation {
            before,
            after: fixed_state(if case == 0 { 1 } else { 2 }),
            range_only: case == 3,
        };
        let circuit = MidnightCircuit::new(&relation, Value::known(()), Value::known(()), Some(K));
        let prover =
            MockProver::run(&circuit, vec![vec![], vec![]]).expect("local synthesis failed");
        let accepted = prover.verify().is_ok();
        assert_eq!(
            accepted,
            case < 2,
            "unexpected local circuit outcome in case {case}"
        );
        event(
            ["accrue", "settle", "closed_continuation", "raw_limb_2pow64"][case],
            "local_circuit",
            if accepted { "accepted" } else { "rejected" },
        );
    }
}

fn main() {
    assert!(
        cfg!(feature = "truncated-challenges"),
        "enable truncated-challenges"
    );
    let output = std::env::var("MORIARTY_R3_OUTPUT").expect("MORIARTY_R3_OUTPUT required");
    let srs_path = std::env::var("MORIARTY_R3_SRS").expect("MORIARTY_R3_SRS required");
    fs::create_dir_all(&output).unwrap();
    let output = Path::new(&output);
    println!(
        "{{\"event\":\"start\",\"predicate\":\"{PREDICATE}\",\"k\":{K},\"steps\":2,\"episode_sha256\":\"{}\",\"source_commit\":\"{}\"}}",
        episode::EPISODE_SHA256,
        episode::INPUT_SOURCE_COMMIT
    );
    preflight_native();
    preflight_circuit();
    let degree = IvcCircuit::<Loan>::cs_degree();
    let blowup =
        <KZGCommitmentScheme<E> as PolynomialCommitmentScheme<F>>::srs_monomial_blowup(degree);
    assert_eq!(blowup, 1, "only unextended K17 SRS authorized");
    assert!(
        fs::metadata(&srs_path).unwrap().len() <= 64 * 1024 * 1024,
        "SRS exceeds bounded local input size"
    );
    let bytes = fs::read(&srs_path).expect("local SRS unavailable");
    assert!(bytes.len() >= 4);
    assert_eq!(
        u32::from_le_bytes(bytes[..4].try_into().unwrap()),
        K,
        "SRS header must be K17 before decoding"
    );
    let srs_hash = sha(&bytes);
    let mut cursor = Cursor::new(&bytes);
    // Match the pinned Midnight trusted-setup file encoding. Trust is in the
    // parent's pinned hash receipt; RawBytesUnchecked is not a ceremony audit.
    let srs = ParamsKZG::<E>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked)
        .expect("SRS decoding failed");
    assert_eq!(
        cursor.position() as usize,
        bytes.len(),
        "trailing SRS bytes"
    );
    assert_eq!(srs.max_k(), K);
    assert_eq!(srs.g_monomial_size(), 1usize << K);
    println!(
        "{{\"event\":\"srs_loaded\",\"sha256\":\"{srs_hash}\",\"bytes\":{},\"cs_degree\":{degree},\"monomial_blowup\":{blowup},\"format\":\"RawBytesUnchecked\"}}",
        bytes.len()
    );
    drop(cursor);
    drop(bytes);
    let start = Instant::now();
    println!("{{\"event\":\"setup_started\"}}");
    let (mut prover, verifier) = ivc::setup::<Loan>(srs, K, fixed_context());
    println!(
        "{{\"event\":\"setup_complete\",\"milliseconds\":{}}}",
        start.elapsed().as_millis()
    );
    let mut proofs = Vec::new();
    let mut proof_hashes = Vec::new();
    let mut sizes = Vec::new();
    for step in 1..=2 {
        let start = Instant::now();
        let proof = prover.prove_step(()).expect("positive prove_step failed");
        let instance = prover.instance();
        assert_eq!(instance.state(), &fixed_state(step));
        assert!(
            proof.len() <= 32 * 1024 * 1024,
            "proof exceeds artifact budget"
        );
        fs::write(output.join(format!("step-{step}.proof")), &proof).unwrap();
        let pi = IvcCircuit::<Loan>::format_instance(&instance).unwrap();
        let pi_bytes: Vec<u8> = pi
            .iter()
            .flat_map(|f| f.to_repr().as_ref().to_vec())
            .collect();
        assert!(
            pi_bytes.len() <= 1024 * 1024,
            "public inputs exceed artifact budget"
        );
        fs::write(
            output.join(format!("step-{step}.public-inputs.le.bin")),
            &pi_bytes,
        )
        .unwrap();
        fs::write(
            output.join(format!("step-{step}.state-limbs.le.bin")),
            wire(instance.state())
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        // Preserve bytes/PI before verification, including a failed result.
        // Native verifier discharges BOTH the proof and carried accumulator.
        verifier
            .verify(&instance, &proof)
            .expect("positive final-accumulator verification failed");
        sizes.push(proof.len());
        proof_hashes.push(sha(&proof));
        println!(
            "{{\"event\":\"step_verified\",\"step\":{step},\"proof_bytes\":{},\"proof_sha256\":\"{}\",\"public_input_elements\":{},\"public_inputs_sha256\":\"{}\",\"milliseconds\":{}}}",
            proof.len(),
            sha(&proof),
            pi.len(),
            sha(&pi_bytes),
            start.elapsed().as_millis()
        );
        proofs.push(proof);
    }
    let final_instance = prover.instance();
    let final_proof = &proofs[1];
    let mut altered = final_proof.clone();
    let midpoint = altered.len() / 2;
    altered[midpoint] ^= 1;
    let mut appended = final_proof.clone();
    appended.push(0);
    for (name, proof) in [
        ("absent_proof", &[][..]),
        ("truncated_proof", &final_proof[..final_proof.len() / 2]),
        ("altered_proof", &altered),
        ("appended_proof", &appended),
        ("step1_proof_for_step2_statement", &proofs[0]),
    ] {
        assert!(
            verifier.verify(&final_instance, proof).is_err(),
            "unexpected acceptance: {name}"
        );
        event(name, "native_verifier", "rejected");
    }
    // resume_from is the only public state mutation interface. These are
    // explicitly DECIDER controls: the replacement accumulator is irrelevant
    // because verifier::verify checks the decider before touching it.
    for (name, financial, digest) in [
        ("altered_cash", Some(7), None),
        ("altered_due", Some(4), None),
        ("wrong_domain", None, Some(0)),
        ("wrong_program", None, Some(1)),
        ("wrong_intent", None, Some(3)),
        ("mismatched_predecessor", None, Some(4)),
        ("excessive_authority_commitment", None, Some(7)),
        ("forged_genesis", Some(0), None),
    ] {
        let mut bad = if name == "forged_genesis" {
            fixed_state(0)
        } else {
            fixed_state(2)
        };
        if let Some(i) = financial {
            bad.financial[i] ^= 1;
        }
        if let Some(i) = digest {
            bad.digests[i][0] ^= 1;
        }
        prover.resume_from(bad, Vec::new(), Accumulator::<S>::trivial(&[]));
        let rejection = verifier.verify(&prover.instance(), final_proof);
        assert!(
            matches!(rejection, Err(ivc::IvcError::DeciderFailed)),
            "expected decider rejection: {name}"
        );
        event(name, "native_verifier_application_decider", "rejected");
    }
    event(
        "different_verifying_key",
        "not_exercised",
        "gap_private_vk_no_second_setup",
    );
    event(
        "unsatisfied_recursive_accumulator",
        "not_exercised",
        "gap_private_accumulator_no_injected_dependency",
    );
    event(
        "verifier_serialization",
        "not_supported",
        "gap_no_public_serialization_api",
    );
    let receipt = format!(
        "{{\"status\":\"completed_with_explicit_gaps\",\"predicate\":\"{PREDICATE}\",\"fixed_instance_specialization\":true,\"steps_proved_and_verified\":2,\"final_accumulator_discharged\":true,\"proof_sizes\":[{},{}],\"proof_sha256\":[\"{}\",\"{}\"],\"episode_sha256\":\"{}\",\"srs_sha256\":\"{srs_hash}\",\"gaps\":[\"different_vk_control\",\"unsatisfied_recursive_accumulator_control\",\"verifier_serialization\",\"dynamic_authorization\",\"general_refinement\",\"ledger_acceptance\"]}}\n",
        sizes[0],
        sizes[1],
        proof_hashes[0],
        proof_hashes[1],
        episode::EPISODE_SHA256
    );
    fs::write(output.join("native-receipt.json"), &receipt).unwrap();
    print!("{receipt}");
}
