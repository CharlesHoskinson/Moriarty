//! Fixed-instance R2 first-period loan IVC, not full Moriarty HistoryCompliance.
//! The circuit specializes to exactly two transitions and three complete private
//! rows with a canonical public phase. All 54 row limbs enter synthesized constraints. SHA256 preimages/effects are pinned by the independently reviewed
//! fixture; SHA256 and dynamic authority are NOT recomputed in this circuit.

use ff::{Field, PrimeField};
use group::Group;
use midnight_aggregation::ivc::{self, IvcCircuit, IvcContext, IvcIO, IvcState, IvcTransition};
use midnight_circuits::{
    instructions::*,
    types::{AssignedBit, AssignedNative},
    verifier::{Accumulator, BlstrsEmulation, Msm, Point, SelfEmulation},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::MockProver,
    plonk::{ConstraintSystem, Error},
    poly::{
        PolynomialLabel,
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
    phase: u64,
    financial: [u128; episode::NUM_FIELDS],
    digests: [[u64; 4]; episode::NUM_DIGESTS],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    immutable: [[u64; 4]; 4],
}
#[derive(Clone, Debug)]
pub struct AssignedState {
    phase: AssignedNative<F>,
    limbs: [AssignedNative<F>; LIMBS],
}
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
    let genesis: [u128; episode::NUM_FIELDS] =
        [0, 2, 5_000_000_000, 0, 0, 0, 0, 20_000_000_000, 0, 0, 0];
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
        phase: index as u64,
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
        // All private full-data fields must match the unique phase-indexed row.
        // This decider supplements, and never replaces, in-circuit equalities.
        state_index(ctx, state).is_some()
    }
}
// Circuit constants come directly from the unchanged generated episode table.
// Host arithmetic checks are separate controls and are never the VK binding.
fn table_limbs(index: usize) -> Vec<F> {
    wire(&State {
        phase: index as u64,
        financial: episode::FINANCIAL[index],
        digests: episode::DIGESTS[index],
    })
    .into_iter()
    .map(F::from)
    .collect()
}
impl Loan {
    fn assign_full(
        &self,
        layouter: &mut impl Layouter<F>,
        phase: Value<F>,
        value: Value<Vec<F>>,
    ) -> Result<AssignedState, Error> {
        let phase: AssignedNative<F> = self.std_lib.assign(layouter, phase)?;
        // Canonical field value 0, 1 or 2; never low-bit truncation.
        self.std_lib
            .assert_lower_than_fixed(layouter, &phase, &3u128.into())?;
        let mut cells = Vec::with_capacity(LIMBS);
        for i in 0..LIMBS {
            let cell: AssignedNative<F> = self
                .std_lib
                .assign(layouter, value.as_ref().map(|v| v[i]))?;
            self.std_lib
                .assert_lower_than_fixed(layouter, &cell, &(1u128 << 64).into())?;
            cells.push(cell);
        }
        Ok(AssignedState {
            phase,
            limbs: cells.try_into().expect("exactly 54 limbs"),
        })
    }
}
impl IvcIO for Loan {
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<State>,
    ) -> Result<AssignedState, Error> {
        self.assign_full(
            layouter,
            value.as_ref().map(|state| F::from(state.phase)),
            value
                .as_ref()
                .map(|state| wire(state).into_iter().map(F::from).collect()),
        )
    }
    fn constrain_as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        state: &AssignedState,
    ) -> Result<(), Error> {
        // Only called on the complete constant-selected successor in IvcCircuit.
        self.std_lib
            .constrain_as_public_input(layouter, &state.phase)
    }
    fn as_public_input(
        &self,
        _: &mut impl Layouter<F>,
        state: &AssignedState,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        // IvcCircuit has already constrained all 54 predecessor limbs through
        // circuit_transition before this feeds the prior-proof/genesis checks.
        Ok(vec![state.phase.clone()])
    }
    fn format_public_input(state: &State) -> Vec<F> {
        // Partial encoding: reject noncanonical full State values. On the three
        // admitted rows phase is injective. This host guard is NOT the circuit
        // binding; circuit_transition emits that binding for every private limb.
        assert!(
            state_index(&fixed_context(), state).is_some(),
            "noncanonical full row"
        );
        vec![F::from(state.phase)]
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
        // Ordinary predecessors are exactly phase 0 or 1. A phase-2 row must
        // never bypass the genesis/prior-proof boundary as a fresh input.
        self.std_lib
            .assert_lower_than_fixed(layouter, &state.phase, &2u128.into())?;
        let genesis: AssignedBit<F> =
            self.std_lib
                .is_equal_to_fixed(layouter, &state.phase, F::ZERO)?;
        let before0 = table_limbs(0);
        let before1 = table_limbs(1);
        let after1 = table_limbs(2);
        let mut next = Vec::with_capacity(LIMBS);
        for i in 0..LIMBS {
            let a: AssignedNative<F> = self.std_lib.assign_fixed(layouter, before0[i])?;
            let b: AssignedNative<F> = self.std_lib.assign_fixed(layouter, before1[i])?;
            let c: AssignedNative<F> = self.std_lib.assign_fixed(layouter, after1[i])?;
            let expected = self.std_lib.select(layouter, &genesis, &a, &b)?;
            self.std_lib
                .assert_equal(layouter, &state.limbs[i], &expected)?;
            let output = self.std_lib.select(layouter, &genesis, &b, &c)?;
            self.std_lib
                .assert_lower_than_fixed(layouter, &output, &(1u128 << 64).into())?;
            next.push(output);
        }
        let next_phase = self.std_lib.add_constant(layouter, &state.phase, F::ONE)?;
        self.std_lib
            .assert_lower_than_fixed(layouter, &next_phase, &3u128.into())?;
        Ok(AssignedState {
            phase: next_phase,
            limbs: next.try_into().expect("exactly 54 limbs"),
        })
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
        assert_eq!(LIMBS, 54);
        assert_eq!(Loan::format_public_input(&s), vec![F::from(i as u64)]);
        for phase in [0, 1, 2, 3, u64::MAX] {
            let mut wrong_phase = s.clone();
            wrong_phase.phase = phase;
            assert_eq!(
                Loan::decider(&fixed_context(), &wrong_phase),
                phase == i as u64
            );
        }
        for j in 0..episode::NUM_FIELDS {
            for bit in [0, 64] {
                let mut bad = s.clone();
                bad.financial[j] ^= 1u128 << bit;
                assert!(!Loan::decider(&fixed_context(), &bad));
                assert!(checked_transition(&fixed_context(), &bad).is_err());
                // Same phase is intentionally insufficient for an arbitrary
                // host row; format_public_input rejects this invalid domain.
                assert_eq!(s.phase, bad.phase);
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

// Exhaustive fixed-size application controls. They are executable preparation,
// not executed evidence. All cases must finish inside the single runner budget.
#[derive(Clone)]
struct LocalRelation {
    phase: F,
    before: Vec<F>,
    after_phase: F,
    after: Vec<F>,
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
        let before = loan.assign_full(
            layouter,
            Value::known(self.phase),
            Value::known(self.before.clone()),
        )?;
        if !self.range_only {
            let after = loan.circuit_transition(layouter, &before, Value::known(()))?;
            lib.assert_equal_to_fixed(layouter, &after.phase, self.after_phase)?;
            for (cell, constant) in after.limbs.iter().zip(&self.after) {
                lib.assert_equal_to_fixed(layouter, cell, *constant)?;
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
fn local_case(name: &str, relation: LocalRelation, expected: bool) {
    let circuit = MidnightCircuit::new(&relation, Value::known(()), Value::known(()), Some(K));
    // Synthesis errors are not counted as successful rejection controls.
    let prover = MockProver::run(&circuit, vec![vec![], vec![]]).expect("local synthesis failed");
    let accepted = prover.verify().is_ok();
    assert_eq!(accepted, expected, "unexpected local outcome: {name}");
    event(
        name,
        "local_circuit",
        if accepted { "accepted" } else { "rejected" },
    );
}
fn base_case(phase: usize) -> LocalRelation {
    LocalRelation {
        phase: F::from(phase as u64),
        before: table_limbs(phase),
        after_phase: F::from((phase + 1) as u64),
        after: table_limbs(phase + 1),
        range_only: false,
    }
}
fn preflight_circuit() {
    for phase in 0..2 {
        let base = base_case(phase);
        local_case(&format!("valid_phase_{phase}"), base.clone(), true);
        for limb in 0..LIMBS {
            let mut before = base.clone();
            before.before[limb] += F::ONE;
            local_case(&format!("before_phase_{phase}_limb_{limb}"), before, false);
            let mut after = base.clone();
            after.after[limb] += F::ONE;
            local_case(&format!("after_phase_{phase}_limb_{limb}"), after, false);
        }
        for row in 0..3 {
            let mut wrong_before = base.clone();
            wrong_before.before = table_limbs(row);
            local_case(
                &format!("phase_{phase}_before_row_{row}"),
                wrong_before,
                row == phase,
            );
            let mut wrong_after = base.clone();
            wrong_after.after = table_limbs(row);
            local_case(
                &format!("phase_{phase}_after_row_{row}"),
                wrong_after,
                row == phase + 1,
            );
        }
        for after_phase in [0u64, 1, 2, 3, u64::MAX] {
            let mut wrong_edge = base.clone();
            wrong_edge.after_phase = F::from(after_phase);
            local_case(
                &format!("edge_{phase}_to_{after_phase}"),
                wrong_edge,
                after_phase == phase as u64 + 1,
            );
        }
    }
    for (name, phase) in [
        ("closed_phase", F::from(2)),
        ("phase_three", F::from(3)),
        ("phase_u64max", F::from(u64::MAX)),
        ("phase_modular_minus_one", -F::ONE),
    ] {
        let mut bad = base_case(0);
        bad.phase = phase;
        if name == "closed_phase" {
            bad.before = table_limbs(2);
        }
        local_case(name, bad, false);
    }
    // Raw-field tests bypass native u64 construction and exercise every limb.
    for limb in 0..LIMBS {
        let mut bad = base_case(0);
        bad.range_only = true;
        bad.before[limb] = F::from(u64::MAX) + F::ONE;
        local_case(&format!("raw_limb_{limb}_2pow64"), bad, false);
    }
    let mut boundary = base_case(0);
    boundary.range_only = true;
    boundary.before = vec![F::from(u64::MAX); LIMBS];
    local_case("all_u64_max_range_boundary", boundary, true);
}

fn main() {
    assert!(
        cfg!(feature = "truncated-challenges"),
        "enable truncated-challenges"
    );
    let candidate_sha =
        std::env::var("MORIARTY_R3_CANDIDATE_SHA256").expect("reviewed candidate hash required");
    assert!(
        candidate_sha.len() == 64
            && candidate_sha
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
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
    assert_eq!(
        srs_hash, "4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74",
        "unreviewed SRS bytes"
    );
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
    let srs_verifier_params = srs.verifier_params();
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
        assert_eq!(
            pi[1],
            F::from(step as u64),
            "application PI is canonical phase"
        );
        if step == 1 {
            fs::write(
                output.join("canonical-vk-repr.le.bin"),
                pi[0].to_repr().as_ref(),
            )
            .unwrap();
        } else {
            assert_eq!(
                fs::read(output.join("canonical-vk-repr.le.bin")).unwrap(),
                pi[0].to_repr().as_ref()
            );
        }
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
        ("wrong_specification", None, Some(2)),
        ("wrong_intent", None, Some(3)),
        ("wrong_output", None, Some(5)),
        ("wrong_effects", None, Some(6)),
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
    // An invalid carried accumulator with the same public shape, built through
    // public APIs. Direct pairing failure and final-verifier rejection are distinct
    // controls; this does not claim a newly proved invalid-accumulator chain.
    let mut cs = ConstraintSystem::default();
    ZkStdLib::configure(&mut cs, (IvcCircuit::<Loan>::arch(), (K - 1) as u8));
    let labels = midnight_circuits::verifier::fixed_base_labels::<S>(
        cs.num_fixed_columns() + cs.num_selectors(),
        cs.permutation().columns.len(),
    );
    let trivial = Accumulator::<S>::trivial(&labels);
    let invalid_lhs = Msm::<S>::new(
        &[Point::Variable(<S as SelfEmulation>::C::generator())],
        &[F::ONE],
        &[PolynomialLabel::NoLabel],
    );
    let invalid_acc = Accumulator::<S>::new(invalid_lhs, trivial.rhs());
    // All fixed-base coefficients in this RHS are zero; identity bases suffice
    // for this direct invariant test and are not a substitute VK for verification.
    let zero_rhs_bases = labels
        .iter()
        .cloned()
        .map(|label| (label, <S as SelfEmulation>::C::identity()))
        .collect();
    assert!(
        !invalid_acc.check(&srs_verifier_params, &zero_rhs_bases),
        "invalid accumulator passed pairing"
    );
    prover.resume_from(fixed_state(2), final_proof.clone(), invalid_acc);
    assert!(
        matches!(
            verifier.verify(&prover.instance(), final_proof),
            Err(ivc::IvcError::InvalidProof)
        ),
        "invalid carried accumulator was not rejected after a valid application decider"
    );
    event(
        "invalid_accumulator_pairing_and_final_statement",
        "native_verifier",
        "rejected",
    );
    event(
        "verifier_serialization",
        "not_supported",
        "gap_no_public_serialization_api",
    );
    let receipt = format!(
        "{{\"status\":\"completed_with_explicit_gaps\",\"candidate_sha256\":\"{candidate_sha}\",\"predicate\":\"{PREDICATE}\",\"fixed_instance_specialization\":true,\"steps_proved_and_verified\":2,\"final_accumulator_discharged\":true,\"application_public_elements\":1,\"private_application_elements\":55,\"proof_sizes\":[{},{}],\"proof_sha256\":[\"{}\",\"{}\"],\"episode_sha256\":\"{}\",\"srs_sha256\":\"{srs_hash}\",\"gaps\":[\"different_vk_control\",\"verifier_serialization\",\"dynamic_authorization\",\"general_refinement\",\"ledger_acceptance\"]}}\n",
        sizes[0],
        sizes[1],
        proof_hashes[0],
        proof_hashes[1],
        episode::EPISODE_SHA256
    );
    fs::write(output.join("native-receipt.json"), &receipt).unwrap();
    print!("{receipt}");
}
