//! Row-count probe of the tree (slot) node shapes, `tree.md` T0 (task 2.13). Modeling only:
//! `cost_model`, no SRS, no keygen of any node. The leaf is the stand-in leaf keyed on
//! throwaway parameters, exactly as `ivc_cost.rs`.
//!
//! The crate has no tree node yet (`ivc.rs` folds one self proof and one leaf). `TreeNodeProbe`
//! is the closest stand-in built from the same gadgets as `SherlockIvcRelation::circuit`, per the
//! segment statement of `tree.md` F8:
//! - own key as public input (`assign_vk_as_public_input`, node domain and cs at `SELF_K`);
//! - per self child: slot bit, child instance `[self_repr, prog_d, pre_d, post_d, h_lo, h_hi,
//!   acc...]`, carried accumulator (`AssignedAccumulator::assign` over the self fixed-base
//!   names), `verifier.prepare` under the self key, both accumulators `scale_by_bit` by the slot
//!   bit;
//! - per leaf: the seven-lane leaf instance under the fixed leaf key (`verifier.prepare` with
//!   the leaf key and its fixed bases assigned once and shared by all leaf slots), the leaf
//!   accumulator scaled by its slot bit;
//! - linkage with selects on the slot bits (child post = sibling pre, additive heights), the
//!   no-empty-node check, `1 <= steps`, and `assert_lower_than_fixed(2^64)` on every height
//!   and on `steps`;
//! - one `accumulate` of all accumulators, one `collapse`, the result as public input.
//!
//! Slot order: L, leaf_1, ..., R. Omitted: `steps <= S` (one more range check per leaf).
#![cfg(feature = "sherlock-recursion")]

use std::{collections::BTreeMap, io};

use ff::Field;
use group::Group;
use midnight_circuits::{
    instructions::{
        ArithInstructions, AssertionInstructions, AssignmentInstructions, BinaryInstructions,
        ControlFlowInstructions, PublicInputInstructions, RangeCheckInstructions,
    },
    types::AssignedNative,
    verifier::{AssignedAccumulator, AssignedVk, fixed_bases},
};
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::{ConstraintSystem, Error},
    poly::{EvaluationDomain, kzg::params::ParamsKZG},
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch, cost_model, setup_vk};
use mori_recursion::{
    inner::{C, S, vk_name},
    ivc::{SELF_NAME, SherlockIvcRelation, node_cs},
    standin::StandInLeaf,
};
use num_bigint::BigUint;
use rand::rngs::OsRng;

use mori_recursion::util::F;

/// The tree node keeps k=20 (`tree.md` F4); its own domain is fixed at this k.
const SELF_K: u32 = 20;
const NR_COLS: u8 = 4;
const KILL_ROWS: u64 = 996_147; // floor(0.95 * 2^20)

#[derive(Clone, Debug)]
struct TreeNodeProbe {
    n_self: usize,
    n_leaf: usize,
    leaf_vk: MidnightVK,
    self_domain: EvaluationDomain<F>,
    self_cs: ConstraintSystem<F>,
}

impl TreeNodeProbe {
    fn new(n_self: usize, n_leaf: usize, leaf_vk: MidnightVK) -> Self {
        let self_cs = node_cs(NR_COLS, SELF_K);
        let self_domain = EvaluationDomain::new(self_cs.degree() as u32, SELF_K);
        Self {
            n_self,
            n_leaf,
            leaf_vk,
            self_domain,
            self_cs,
        }
    }

    fn self_base_names(&self) -> Vec<String> {
        let selectors = vec![vec![false]; self.self_cs.num_selectors()];
        let (processed, _) = self
            .self_cs
            .clone()
            .directly_convert_selectors_to_fixed(selectors);
        midnight_circuits::verifier::fixed_base_names::<S>(
            SELF_NAME,
            processed.num_fixed_columns(),
            processed.permutation().columns.len(),
        )
    }
}

fn arch() -> ZkStdLibArch {
    ZkStdLibArch {
        bls12_381: true,
        poseidon: true,
        nr_pow2range_cols: NR_COLS,
        ..ZkStdLibArch::default()
    }
}

impl Relation for TreeNodeProbe {
    type Instance = Vec<F>;
    type Witness = ();
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        _witness: Value<()>,
    ) -> Result<(), Error> {
        type A = AssignedNative<F>;
        let verifier = std.verifier();
        let bls = std.bls12_381();
        let scalar = bls.scalar_field_chip();
        let u = || Value::<F>::unknown();
        let bound = BigUint::from(1u8) << 64usize;

        // Own key as public input (lane 0).
        let self_vk: AssignedVk<S> = verifier.assign_vk_as_public_input(
            l,
            SELF_NAME,
            &self.self_domain,
            &self.self_cs,
            u(),
        )?;

        // Node statement.
        let prog_d: A = std.assign(l, u())?;
        let pre_d: A = std.assign(l, u())?;
        let post_d: A = std.assign(l, u())?;
        let h_lo: A = std.assign(l, u())?;
        let h_hi: A = std.assign(l, u())?;
        for x in [&h_lo, &h_hi] {
            std.assert_lower_than_fixed(l, x, &bound)?;
        }
        for x in [&prog_d, &pre_d, &post_d, &h_lo, &h_hi] {
            std.constrain_as_public_input(l, x)?;
        }

        let names = self.self_base_names();
        let id = bls.assign_fixed(l, C::identity())?;
        let mut accs: Vec<AssignedAccumulator<S>> = Vec::new();
        let mut bits = Vec::new();

        // Running boundary (digest, height) from pre_d/h_lo through the slots.
        let mut cur_d = pre_d.clone();
        let mut cur_h = h_lo.clone();

        // One self child: slot bit, instance, carried acc, prepare, scale both, link.
        let self_slot = |l: &mut _,
                         cur_d: &mut A,
                         cur_h: &mut A,
                         accs: &mut Vec<AssignedAccumulator<S>>,
                         bits: &mut Vec<_>|
         -> Result<(), Error> {
            let has = std.assign(l, Value::<bool>::unknown())?;
            let c_pre: A = std.assign(l, u())?;
            let c_post: A = std.assign(l, u())?;
            let c_lo: A = std.assign(l, u())?;
            let c_hi: A = std.assign(l, u())?;
            std.assert_lower_than_fixed(l, &c_hi, &bound)?;
            // has ? (c_pre = cur_d, c_lo = cur_h) : no constraint; then advance.
            let want_pre = std.select(l, &has, &c_pre, cur_d)?;
            std.assert_equal(l, &want_pre, cur_d)?;
            let want_lo = std.select(l, &has, &c_lo, cur_h)?;
            std.assert_equal(l, &want_lo, cur_h)?;
            *cur_d = std.select(l, &has, &c_post, cur_d)?;
            *cur_h = std.select(l, &has, &c_hi, cur_h)?;

            let carried = AssignedAccumulator::<S>::assign(
                l,
                bls,
                scalar,
                1,
                1,
                &[],
                &names,
                Value::unknown(),
            )?;
            let mut pi = verifier.as_public_input(l, &self_vk)?;
            pi.extend([prog_d.clone(), c_pre, c_post, c_lo, c_hi]);
            pi.extend(verifier.as_public_input(l, &carried)?);
            let mut proof_acc = verifier.prepare(
                l,
                &self_vk,
                std::slice::from_ref(&id),
                &[pi.as_slice()],
                Value::unknown(),
            )?;
            let mut carried = carried;
            AssignedAccumulator::scale_by_bit(l, scalar, &has, &mut proof_acc)?;
            AssignedAccumulator::scale_by_bit(l, scalar, &has, &mut carried)?;
            accs.push(proof_acc);
            accs.push(carried);
            bits.push(has);
            Ok(())
        };

        // Left child.
        if self.n_self >= 1 {
            self_slot(l, &mut cur_d, &mut cur_h, &mut accs, &mut bits)?;
        }

        // Leaf slots under the fixed leaf key; key and fixed bases assigned once.
        if self.n_leaf > 0 {
            let pvk = self.leaf_vk.vk();
            let name = vk_name("shk_leaf", &self.leaf_vk);
            let leaf_vk = verifier.assign_fixed_vk(
                l,
                &name,
                pvk.get_domain(),
                pvk.cs(),
                pvk.transcript_repr(),
            )?;
            let mut bases = BTreeMap::new();
            for (n, base) in fixed_bases::<S>(&name, pvk) {
                bases.insert(n, bls.assign_fixed(l, base)?);
            }
            let mut leaf_accs = Vec::new();
            for _ in 0..self.n_leaf {
                let has = std.assign(l, Value::<bool>::unknown())?;
                let binding: A = std.assign(l, u())?;
                let post: A = std.assign(l, u())?;
                let rule: A = std.assign(l, u())?;
                let steps: A = std.assign(l, u())?;
                std.assert_not_equal_to_fixed(l, &steps, F::ZERO)?;
                std.assert_lower_than_fixed(l, &steps, &bound)?;
                let h2 = std.add(l, &cur_h, &steps)?;
                std.assert_lower_than_fixed(l, &h2, &bound)?;
                let inst = vec![
                    binding,
                    prog_d.clone(),
                    cur_d.clone(),
                    post.clone(),
                    rule,
                    cur_h.clone(),
                    steps,
                ];
                let mut acc = verifier.prepare(
                    l,
                    &leaf_vk,
                    std::slice::from_ref(&id),
                    &[inst.as_slice()],
                    Value::unknown(),
                )?;
                AssignedAccumulator::scale_by_bit(l, scalar, &has, &mut acc)?;
                cur_d = std.select(l, &has, &post, &cur_d)?;
                cur_h = std.select(l, &has, &h2, &cur_h)?;
                leaf_accs.push(acc);
                bits.push(has);
            }
            let mut leaf_acc = verifier.accumulate(l, &leaf_accs)?;
            leaf_acc.resolve_fixed_bases(&bases);
            accs.push(leaf_acc);
        }

        // Right child, then the node's post boundary.
        for _ in 1..self.n_self {
            self_slot(l, &mut cur_d, &mut cur_h, &mut accs, &mut bits)?;
        }
        std.assert_equal(l, &cur_d, &post_d)?;
        std.assert_equal(l, &cur_h, &h_hi)?;

        // No empty node: at least one slot bit set.
        let mut any = bits[0].clone();
        for b in &bits[1..] {
            any = std.or(l, &[any, b.clone()])?;
        }
        std.assert_equal_to_fixed(l, &any, true)?;

        // Fold, collapse, publish.
        let mut next = verifier.accumulate(l, &accs)?;
        next.collapse(l, bls, scalar)?;
        verifier.constrain_as_public_input(l, &next)
    }

    fn used_chips(&self) -> ZkStdLibArch {
        arch()
    }

    fn write_relation<W: io::Write>(&self, _w: &mut W) -> io::Result<()> {
        Err(io::Error::other("probe only"))
    }

    fn read_relation<R: io::Read>(_r: &mut R) -> io::Result<Self> {
        Err(io::Error::other("probe only"))
    }
}

#[test]
#[ignore = "row-count probe, ~145 s in debug; light lane: --features sherlock-recursion -- --ignored"]
fn tree_node_rows() {
    let leaf = StandInLeaf;
    let lk = cost_model(&leaf, None).k;
    let p: ParamsKZG<Bls12> = ParamsKZG::unsafe_setup(lk, OsRng);
    let vk = setup_vk(&p, &leaf);

    // Calibration: the existing linear node {self, leaf} and the probe of the same shape.
    for k in [19u32, 20] {
        let rel = SherlockIvcRelation::new(NR_COLS, k, vk.clone()).unwrap();
        let m = cost_model(&rel, Some(k));
        println!(
            "TREEROWS shape=ivc_linear(self+carried+leaf) self_k={k} rows={} k={}",
            m.rows, m.k
        );
    }
    let base = cost_model(&TreeNodeProbe::new(1, 1, vk.clone()), Some(SELF_K));
    println!(
        "TREEROWS shape=probe{{self,leaf}} rows={} k={}",
        base.rows, base.k
    );

    let mut killed = false;
    for (label, n_self, n_leaf) in [
        ("{self,self,leaf}", 2, 1),
        ("{self,self}", 2, 0),
        ("{self,self,leaf,leaf}", 2, 2),
    ] {
        let m = cost_model(
            &TreeNodeProbe::new(n_self, n_leaf, vk.clone()),
            Some(SELF_K),
        );
        let over = m.rows as u64 > KILL_ROWS;
        killed |= over && n_leaf == 1;
        println!(
            "TREEROWS shape=probe{label} rows={} k={} margin_to_2^20={:.1}% over_0.95x2^20={over}",
            m.rows,
            m.k,
            100.0 * (1.0 - m.rows as f64 / f64::from(1u32 << 20))
        );
    }
    println!(
        "TREEROWS verdict={}",
        if killed { "KILL" } else { "tree not killed" }
    );
}
