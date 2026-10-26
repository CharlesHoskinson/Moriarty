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
//! At genesis (h-1 = 0) the previous proof's accumulator is scaled by zero.
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
        ControlFlowInstructions, PublicInputInstructions, ZeroInstructions,
    },
    types::{AssignedNative, Instantiable},
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

use crate::{
    bls_arch,
    inner::{C, S, prepare_offcircuit_with, verify_child_incircuit_with, vk_name},
    util::{F, read_bytes, read_u32, vk_bytes, vk_from_bytes, write_bytes},
};

const MAGIC: &[u8; 8] = b"SHKIVC04";
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
        })
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
        let mut v = vec![self_repr];
        v.extend(st.fields());
        v.extend(<AssignedAccumulator<S> as Instantiable<F>>::as_public_input(acc));
        v
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
        let genesis = w.prev.h == 0;
        let proof_acc = if genesis {
            self.trivial_acc()
        } else {
            let vk = node_vk.ok_or("non-genesis step needs the node key")?;
            let prev_inst = Self::instance(w.self_repr, &w.prev, &w.prev_acc);
            let mut tr = CircuitTranscript::<PoseidonState<F>>::init_from_bytes(&w.prev_proof);
            let dual = plonk::prepare::<
                F,
                KZGCommitmentScheme<Bls12>,
                CircuitTranscript<PoseidonState<F>>,
            >(vk.vk(), &[&[C::identity()]], &[&[&prev_inst]], &mut tr)
            .map_err(|e| format!("prepare previous node proof: {e:?}"))?;
            let bases = fixed_bases::<S>(SELF_NAME, vk.vk());
            // Left uncollapsed, mirroring the circuit (one collapse after folding).
            Accumulator::<S>::from_dual_msm(dual, SELF_NAME, &bases)
        };
        let leaf_inst = Self::leaf_instance(&w.prev, w.leaf_binding, w.cur_d, w.rule, w.leaf_steps);
        let leaf_acc = prepare_offcircuit_with(
            &self.leaf_vk,
            &self.leaf_name(),
            &leaf_inst,
            &w.leaf_proof,
            false,
            false,
        )?;
        let mut acc = Accumulator::accumulate(&[proof_acc, w.prev_acc.clone(), leaf_acc]);
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
        let prev_h: AssignedNative<F> = std.assign(l, wf(|w| F::from(w.prev.h)))?;
        let cur_d: AssignedNative<F> = std.assign(l, wf(|w| w.cur_d))?;
        let rule: AssignedNative<F> = std.assign(l, wf(|w| w.rule))?;
        let binding: AssignedNative<F> = std.assign(l, wf(|w| w.leaf_binding))?;
        let steps: AssignedNative<F> = std.assign(l, wf(|w| F::from(w.leaf_steps)))?;
        std.assert_not_equal_to_fixed(l, &steps, F::ZERO)?;
        let h = std.add(l, &prev_h, &steps)?;
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
        let mut prev_pi = verifier.as_public_input(l, &self_vk)?;
        prev_pi.extend([
            prog_d.clone(),
            init_d.clone(),
            prev_d.clone(),
            prev_h.clone(),
        ]);
        prev_pi.extend(verifier.as_public_input(l, &prev_acc)?);
        let id = bls.assign_fixed(l, C::identity())?;
        let mut proof_acc = verifier.prepare(
            l,
            &self_vk,
            &[id],
            &[prev_pi.as_slice()],
            witness.as_ref().map(|w| w.prev_proof.clone()),
        )?;
        AssignedAccumulator::scale_by_bit(l, scalar, &not_genesis, &mut proof_acc)?;
        // Not collapsed here: the proof, carried and leaf accumulators are folded first and
        // collapsed once below (one shared MSM keeps the node at k=19).

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
            return Err(io::Error::other("not a SHKIVC04 node relation"));
        }
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        let k = read_u32(r)?;
        let leaf = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        Self::new(b[0], k, leaf)
    }
}
