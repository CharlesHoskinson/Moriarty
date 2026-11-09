//! Sherlock general-purpose recursion node: an IVC step that folds one leaf
//! (one proved machine step) onto the proof of all earlier steps.
//!
//! Node N_h proves: "there is a valid N_{h-1} proof (under this same key) for
//! state (prog_d, init_d, prev_d, h-1), or h-1 = 0 and prev_d = init_d; and
//! there is a valid leaf proof under the fixed leaf key with statement
//! [prog_d, prev_d, cur_d, rule, h-1]". Its public statement is
//! `[self_vk_repr, prog_d, init_d, cur_d, h, acc...]`.
//!
//! Self-verification follows the midnight-zk-stdlib IVC pattern: the node's
//! own key enters as `assign_vk_as_public_input` (its constraint system and
//! domain are fixed off-circuit; only `transcript_repr` is a public input).
//! The self part of the carried accumulator keeps its fixed-base scalars
//! unresolved; whoever pins the node key (the decider) resolves them. The leaf
//! key is a circuit constant and its fixed bases are resolved in circuit.
//! At genesis (h-1 = 0) the previous proof's accumulator is scaled by zero and collapsed
//! (it then equals `trivial_acc`), and the supplied carried accumulator is constrained to
//! the neutral encoding of `trivial_acc` (SRC-005 realization (a)), so a genesis node
//! carries no obligation that no proof authenticates.
//!
//! Heights (SRC-004): `prev_h < 2^64`, `1 <= steps <= S_MAX` and `h = prev_h + steps < 2^64`
//! are range-checked, so `h` is the integer sum and never wraps to 0 mid-chain; `h = 0`
//! (the genesis test) holds only before the first leaf.
//!
//! Scope: the node is program-independent; the leaf relation is a parameter.
//! The node grants no acceptance by itself: validity requires the final
//! accumulator's pairing (the decider) and the decider's pins of
//! `self_vk_repr`, `init_d` and `prog_d`.
use std::{collections::BTreeMap, io};

use ff::{Field, PrimeField};
use group::Group;
use midnight_circuits::{
    hash::poseidon::PoseidonState,
    instructions::{
        ArithInstructions, AssertionInstructions, AssignmentInstructions, BinaryInstructions,
        ControlFlowInstructions, PublicInputInstructions, RangeCheckInstructions, ZeroInstructions,
    },
    types::{AssignedBit, AssignedNative, Instantiable},
    verifier::{Accumulator, AssignedAccumulator, AssignedVk, Msm, fixed_base_names, fixed_bases},
};
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::{self, ConstraintSystem, Error},
    poly::{
        EvaluationDomain,
        kzg::{KZGCommitmentScheme, params::ParamsVerifierKZG},
    },
    transcript::{CircuitTranscript, Transcript},
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch};
use num_bigint::BigUint;

use crate::{
    bls_arch,
    inner::{C, S, prepare_offcircuit_with, verify_child_incircuit_with, vk_name},
    util::{F, read_bytes, read_u32, vk_bytes, vk_from_bytes, write_bytes},
};

const MAGIC: &[u8; 8] = b"SHKIVC06";
/// Name prefix of the node's own (self) fixed bases.
pub const SELF_NAME: &str = "shk_node";
/// State fields after the self key: prog_d, init_d, cur_d, h.
pub const IVC_STATE: usize = 4;
/// Leaf statement: prog_d, pre_d, post_d, rule, h_pre.
/// Leaf instance: [binding, prog_d, pre_d, post_d, rule, h_pre, steps]. The binding lane is
/// the leaf's own claim commitment (public input 0 of every ZKIR child); the node takes it as
/// a free witness and constrains the other lanes. `steps` (>= 1) is the number of machine
/// steps the leaf proves, so h counts machine steps: h = h_pre + steps.
pub const LEAF_LANES: usize = 7;
/// Most machine steps one leaf may prove (SRC-004; the G-A provisional S, task 2.3). A
/// relation constant: changing it changes the node key.
pub const S_MAX: u64 = 16;
/// Heights are integers below 2^H_BITS (SRC-004).
pub const H_BITS: usize = 64;

/// Domain of a machine-state digest. Every leaf's pre_d/post_d is
/// `Poseidon(dom("SHK_SD"), outcome_tag, outcome_value, lanes_d)`, with
/// `lanes_d` the Poseidon digest of all carrier lanes, outcome_tag
/// `OUTCOME_RUNNING` (value 0) or `OUTCOME_HALTED` (value = the u128 result).
pub const DOM_STATE_D: &str = "SHK_SD";
pub const OUTCOME_RUNNING: u64 = 0;
pub const OUTCOME_HALTED: u64 = 1;

/// (domain, halted tag) field constants of a halted state digest.
pub fn state_digest_halted() -> (F, F) {
    (crate::parent::dom(DOM_STATE_D), F::from(OUTCOME_HALTED))
}

/// Off-circuit state digest.
pub fn state_digest(tag: u64, value: u128, lanes_d: F) -> F {
    crate::parent::poseidon(&[
        crate::parent::dom(DOM_STATE_D),
        F::from(tag),
        F::from_u128(value),
        lanes_d,
    ])
}

/// The node relation. `self_k` fixes the node's own domain and constraint system.
#[derive(Clone, Debug)]
pub struct SherlockIvcRelation {
    pub nr_cols: u8,
    pub self_k: u32,
    pub leaf_vk: MidnightVK,
    self_domain: EvaluationDomain<F>,
    self_cs: ConstraintSystem<F>,
    /// MockProver negatives only: field values of (prev_h, steps) that replace the witness
    /// heights, which a `u64` cannot express (a height at or above 2^64). Witness values
    /// only; the constraint system and the key do not depend on it.
    raw_heights: Option<(F, F)>,
}

#[derive(Clone, Debug)]
pub struct IvcState {
    pub prog_d: F,
    pub init_d: F,
    pub cur_d: F,
    pub h: u64,
}

impl IvcState {
    pub fn fields(&self) -> [F; IVC_STATE] {
        [self.prog_d, self.init_d, self.cur_d, F::from(self.h)]
    }
}

#[derive(Clone, Debug)]
pub struct IvcWitness {
    pub self_repr: F,
    pub prev: IvcState,
    /// Empty at genesis.
    pub prev_proof: Vec<u8>,
    pub prev_acc: Accumulator<S>,
    pub cur_d: F,
    pub rule: F,
    /// Public input 0 of the leaf proof (its claim binding), unconstrained by the node.
    pub leaf_binding: F,
    /// Machine steps proved by the leaf (>= 1).
    pub leaf_steps: u64,
    pub leaf_proof: Vec<u8>,
}

/// The node's constraint system exactly as `MidnightCircuit` configures it.
pub fn node_cs(nr_cols: u8, k: u32) -> ConstraintSystem<F> {
    let mut cs = ConstraintSystem::default();
    let _ = ZkStdLib::configure(&mut cs, (bls_arch(nr_cols), (k - 1) as u8));
    cs
}

impl SherlockIvcRelation {
    pub fn new(nr_cols: u8, self_k: u32, leaf_vk: MidnightVK) -> io::Result<Self> {
        if !(1..=4).contains(&nr_cols) || !(10..=24).contains(&self_k) {
            return Err(io::Error::other("node nr_cols in 1..=4 and k in 10..=24"));
        }
        let self_cs = node_cs(nr_cols, self_k);
        let self_domain = EvaluationDomain::new(self_cs.degree() as u32, self_k);
        Ok(Self {
            nr_cols,
            self_k,
            leaf_vk,
            self_domain,
            self_cs,
            raw_heights: None,
        })
    }

    /// MockProver negatives only (see `raw_heights`): the circuit assigns these field values
    /// to `prev_h` and `steps` instead of the witness's `u64` heights.
    #[doc(hidden)]
    pub fn with_raw_heights(mut self, prev_h: F, steps: F) -> Self {
        self.raw_heights = Some((prev_h, steps));
        self
    }

    pub fn leaf_name(&self) -> String {
        vk_name("shk_leaf", &self.leaf_vk)
    }

    /// Fixed-base names of the self part of the carried accumulator.
    pub fn self_base_names(&self) -> Vec<String> {
        let selectors = vec![vec![false]; self.self_cs.num_selectors()];
        let (processed, _) = self
            .self_cs
            .clone()
            .directly_convert_selectors_to_fixed(selectors);
        fixed_base_names::<S>(
            SELF_NAME,
            processed.num_fixed_columns(),
            processed.permutation().columns.len(),
        )
    }

    /// The genesis (trivial) accumulator: one default base, scalar one, and every
    /// self fixed-base scalar zero.
    pub fn trivial_acc(&self) -> Accumulator<S> {
        let zeros: BTreeMap<String, F> = self
            .self_base_names()
            .into_iter()
            .map(|n| (n, F::ZERO))
            .collect();
        Accumulator::new(
            Msm::new(&[C::default()], &[F::ONE], &BTreeMap::new()),
            Msm::new(&[C::default()], &[F::ONE], &zeros),
        )
    }

    /// The public statement of a node proof.
    pub fn instance(self_repr: F, st: &IvcState, acc: &Accumulator<S>) -> Vec<F> {
        Self::instance_from_fields(self_repr, &st.fields(), acc)
    }

    /// The public statement from raw state fields `[prog_d, init_d, cur_d, h]`.
    pub fn instance_from_fields(
        self_repr: F,
        fields: &[F; IVC_STATE],
        acc: &Accumulator<S>,
    ) -> Vec<F> {
        let mut v = vec![self_repr];
        v.extend(fields);
        v.extend(<AssignedAccumulator<S> as Instantiable<F>>::as_public_input(acc));
        v
    }

    /// True iff `acc` has the public encoding of `trivial_acc`, the only carried
    /// accumulator a genesis node accepts (SRC-005 realization (a)).
    pub fn is_neutral(&self, acc: &Accumulator<S>) -> bool {
        <AssignedAccumulator<S> as Instantiable<F>>::as_public_input(acc)
            == <AssignedAccumulator<S> as Instantiable<F>>::as_public_input(&self.trivial_acc())
    }

    pub fn leaf_instance(st_prev: &IvcState, binding: F, cur_d: F, rule: F, steps: u64) -> Vec<F> {
        vec![
            binding,
            st_prev.prog_d,
            st_prev.cur_d,
            cur_d,
            rule,
            F::from(st_prev.h),
            F::from(steps),
        ]
    }

    /// Off-circuit: the accumulator node N_h exposes, given its witness and the
    /// real node key (None only at genesis, where the previous proof is absent).
    pub fn next_acc(
        &self,
        node_vk: Option<&MidnightVK>,
        w: &IvcWitness,
    ) -> Result<Accumulator<S>, String> {
        if w.prev.h == 0 && !self.is_neutral(&w.prev_acc) {
            return Err("genesis carried accumulator must be trivial_acc (SRC-005)".into());
        }
        let leaf_inst = Self::leaf_instance(&w.prev, w.leaf_binding, w.cur_d, w.rule, w.leaf_steps);
        self.next_acc_raw(
            node_vk,
            w.self_repr,
            &w.prev.fields(),
            &w.prev_proof,
            &w.prev_acc,
            &leaf_inst,
            &w.leaf_proof,
        )
    }

    /// `next_acc` over raw field values: the previous state fields `[prog_d, init_d, cur_d,
    /// h]` and the leaf instance. Genesis is `h == 0` in the field, exactly as in circuit.
    /// It checks nothing beyond the fold, so it also builds the statements of negative tests.
    #[allow(clippy::too_many_arguments)]
    pub fn next_acc_raw(
        &self,
        node_vk: Option<&MidnightVK>,
        self_repr: F,
        prev: &[F; IVC_STATE],
        prev_proof: &[u8],
        prev_acc: &Accumulator<S>,
        leaf_inst: &[F],
        leaf_proof: &[u8],
    ) -> Result<Accumulator<S>, String> {
        let genesis = prev[3] == F::ZERO;
        let proof_acc = if genesis {
            self.trivial_acc()
        } else {
            let vk = node_vk.ok_or("non-genesis step needs the node key")?;
            let prev_inst = Self::instance_from_fields(self_repr, prev, prev_acc);
            let mut tr = CircuitTranscript::<PoseidonState<F>>::init_from_bytes(prev_proof);
            let dual = plonk::prepare::<
                F,
                KZGCommitmentScheme<Bls12>,
                CircuitTranscript<PoseidonState<F>>,
            >(vk.vk(), &[&[C::identity()]], &[&[&prev_inst]], &mut tr)
            .map_err(|e| format!("prepare previous node proof: {e:?}"))?;
            let bases = fixed_bases::<S>(SELF_NAME, vk.vk());
            // Collapsed, mirroring the circuit: the fold challenge hashes every folded
            // accumulator, so the genesis one must be the canonical trivial accumulator.
            let mut a = Accumulator::<S>::from_dual_msm(dual, SELF_NAME, &bases);
            a.collapse();
            a
        };
        let leaf_acc = prepare_offcircuit_with(
            &self.leaf_vk,
            &self.leaf_name(),
            leaf_inst,
            leaf_proof,
            false,
            false,
        )?;
        let mut acc = Accumulator::accumulate(&[proof_acc, prev_acc.clone(), leaf_acc]);
        acc.collapse();
        Ok(acc)
    }

    /// Off-circuit final check of a node accumulator against the real node key.
    pub fn check_acc(
        params: &ParamsVerifierKZG<Bls12>,
        node_vk: &MidnightVK,
        acc: &Accumulator<S>,
    ) -> bool {
        acc.check(params, &fixed_bases::<S>(SELF_NAME, node_vk.vk()))
    }
}

/// SRC-004 height checks; returns `h = prev_h + steps`. Heights are integers below 2^64,
/// `1 <= steps <= S_MAX` (as `steps - 1` in `[0, S_MAX)`, one check for both bounds), and the
/// sum is checked again, so `h` is the integer `prev_h + steps < 2^64`, `h >= 1`, and `h`
/// cannot wrap to 0 (which would set `is_genesis` on a later step and drop the previous proof).
pub fn constrain_heights(
    std: &ZkStdLib,
    l: &mut impl Layouter<F>,
    prev_h: &AssignedNative<F>,
    steps: &AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    let h_bound = BigUint::from(1u8) << H_BITS;
    std.assert_lower_than_fixed(l, prev_h, &h_bound)?;
    let steps_m1 = std.add_constant(l, steps, -F::ONE)?;
    std.assert_lower_than_fixed(l, &steps_m1, &BigUint::from(S_MAX))?;
    let h = std.add(l, prev_h, steps)?;
    std.assert_lower_than_fixed(l, &h, &h_bound)?;
    Ok(h)
}

/// SRC-005 realization (a): at genesis no previous proof authenticates the carried
/// accumulator (its proof is scaled away), so its public encoding `lanes` must equal the
/// neutral encoding of `trivial_acc`: `is_genesis * (lane - neutral) == 0` for every lane.
/// `accumulate` hashes exactly these lanes, so at genesis the fold challenge and the folded
/// value depend on the leaf alone: no grinding and no cancellation through the carry.
pub fn constrain_genesis_neutral(
    std: &ZkStdLib,
    l: &mut impl Layouter<F>,
    is_genesis: &AssignedBit<F>,
    lanes: &[AssignedNative<F>],
    neutral: &[F],
) -> Result<(), Error> {
    if neutral.len() != lanes.len() {
        return Err(Error::Synthesis(
            "carried accumulator encoding length differs from trivial_acc".into(),
        ));
    }
    let g: AssignedNative<F> = is_genesis.clone().into();
    for (x, t) in lanes.iter().zip(neutral) {
        let d = std.add_and_mul(l, (F::ZERO, x), (-*t, &g), (F::ZERO, x), F::ZERO, F::ONE)?;
        std.assert_zero(l, &d)?;
    }
    Ok(())
}

/// Inverse of the public accumulator encoding of a node instance tail, given the
/// self fixed-base names; refuses any lanes that do not re-encode identically.
pub fn acc_from_lanes(lanes: &[F], names: &[String]) -> Option<Accumulator<S>> {
    let np = crate::inner::point_pi_len();
    if lanes.len() != 2 * (np + 1) + names.len() {
        return None;
    }
    let pt = |s: &[F]| <crate::inner::AP as Instantiable<F>>::from_public_input(s);
    let (lb, ls) = (pt(&lanes[..np])?, lanes[np]);
    let (rb, rs) = (pt(&lanes[np + 1..2 * np + 1])?, lanes[2 * np + 1]);
    let mut sorted = names.to_vec();
    sorted.sort();
    let fixed: BTreeMap<String, F> = sorted
        .into_iter()
        .zip(lanes[2 * np + 2..].iter().copied())
        .collect();
    let acc = Accumulator::new(
        Msm::new(&[lb], &[ls], &BTreeMap::new()),
        Msm::new(&[rb], &[rs], &fixed),
    );
    (<AssignedAccumulator<S> as Instantiable<F>>::as_public_input(&acc) == lanes).then_some(acc)
}

impl Relation for SherlockIvcRelation {
    type Instance = Vec<F>;
    type Witness = IvcWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        witness: Value<IvcWitness>,
    ) -> Result<(), Error> {
        let verifier = std.verifier();
        let bls = std.bls12_381();
        let scalar = bls.scalar_field_chip();

        // 1. Own key as public input (public lane 0).
        let self_vk: AssignedVk<S> = verifier.assign_vk_as_public_input(
            l,
            SELF_NAME,
            &self.self_domain,
            &self.self_cs,
            witness.as_ref().map(|w| w.self_repr),
        )?;

        // 2. Previous state, transition and new public state.
        let wf = |f: fn(&IvcWitness) -> F| witness.as_ref().map(f);
        let prog_d: AssignedNative<F> = std.assign(l, wf(|w| w.prev.prog_d))?;
        let init_d: AssignedNative<F> = std.assign(l, wf(|w| w.prev.init_d))?;
        let prev_d: AssignedNative<F> = std.assign(l, wf(|w| w.prev.cur_d))?;
        let raw = self.raw_heights;
        let prev_h: AssignedNative<F> = std.assign(
            l,
            witness
                .as_ref()
                .map(|w| raw.map_or(F::from(w.prev.h), |r| r.0)),
        )?;
        let cur_d: AssignedNative<F> = std.assign(l, wf(|w| w.cur_d))?;
        let rule: AssignedNative<F> = std.assign(l, wf(|w| w.rule))?;
        let binding: AssignedNative<F> = std.assign(l, wf(|w| w.leaf_binding))?;
        let steps: AssignedNative<F> = std.assign(
            l,
            witness
                .as_ref()
                .map(|w| raw.map_or(F::from(w.leaf_steps), |r| r.1)),
        )?;
        let h = constrain_heights(std, l, &prev_h, &steps)?;
        let is_genesis = std.is_zero(l, &prev_h)?;
        let not_genesis = std.not(l, &is_genesis)?;
        let chosen = std.select(l, &is_genesis, &init_d, &prev_d)?;
        std.assert_equal(l, &chosen, &prev_d)?;
        for x in [&prog_d, &init_d, &cur_d, &h] {
            std.constrain_as_public_input(l, x)?;
        }

        // 3. Previous node proof under the self key; scaled away at genesis.
        let prev_acc = AssignedAccumulator::<S>::assign(
            l,
            bls,
            scalar,
            1,
            1,
            &[],
            &self.self_base_names(),
            witness.as_ref().map(|w| w.prev_acc.clone()),
        )?;
        let prev_acc_pi = verifier.as_public_input(l, &prev_acc)?;
        let neutral =
            <AssignedAccumulator<S> as Instantiable<F>>::as_public_input(&self.trivial_acc());
        constrain_genesis_neutral(std, l, &is_genesis, &prev_acc_pi, &neutral)?;
        let mut prev_pi = verifier.as_public_input(l, &self_vk)?;
        prev_pi.extend([
            prog_d.clone(),
            init_d.clone(),
            prev_d.clone(),
            prev_h.clone(),
        ]);
        prev_pi.extend(prev_acc_pi);
        let id = bls.assign_fixed(l, C::identity())?;
        let mut proof_acc = verifier.prepare(
            l,
            &self_vk,
            &[id],
            &[prev_pi.as_slice()],
            witness.as_ref().map(|w| w.prev_proof.clone()),
        )?;
        AssignedAccumulator::scale_by_bit(l, scalar, &not_genesis, &mut proof_acc)?;
        // Collapsed before the fold. `accumulate` derives its challenge from a hash of every
        // folded accumulator's full encoding, bases included. Scaled by zero at genesis, the
        // uncollapsed accumulator still carries the bases parsed from the (absent) previous
        // proof, so the challenge would depend on them and could not match `next_acc`, which
        // uses `trivial_acc` there. Collapsed, it is (identity, 1, zero fixed scalars): exactly
        // `trivial_acc`. The leaf accumulator stays uncollapsed (its shape is fixed by the
        // leaf key and mirrored off circuit) and is collapsed once with the fold below.
        proof_acc.collapse(l, bls, scalar)?;

        // 4. The leaf under its fixed key, linked to (prog_d, prev_d, cur_d, h_pre).
        let leaf_inst = vec![binding, prog_d, prev_d, cur_d, rule, prev_h, steps];
        let leaf_acc = verify_child_incircuit_with(
            std,
            l,
            &self.leaf_vk,
            &self.leaf_name(),
            &leaf_inst,
            witness.as_ref().map(|w| w.leaf_proof.clone()),
            false,
            false,
        )?;

        // 5. Fold, collapse, publish.
        let mut next = verifier.accumulate(l, &[proof_acc, prev_acc, leaf_acc])?;
        next.collapse(l, bls, scalar)?;
        verifier.constrain_as_public_input(l, &next)
    }

    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(self.nr_cols)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(MAGIC)?;
        w.write_all(&[self.nr_cols])?;
        w.write_all(&self.self_k.to_le_bytes())?;
        write_bytes(w, &vk_bytes(&self.leaf_vk))
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::other("not a SHKIVC06 node relation"));
        }
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        let k = read_u32(r)?;
        let leaf = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        Self::new(b[0], k, leaf)
    }
}
