//! Node row count with a per-gadget breakdown (research `prover.md` P1(a)). Modeling only:
//! `cost_model` over a copy of `SherlockIvcRelation::circuit` (`crates/mori-recursion/src/ivc.rs`),
//! no SRS, no keygen, no proof.
//!
//! The copy places the `cost_model` measurement markers around one section at a time; the
//! reported rows of a marked run are the row span of the regions assigned in that section.
//! Unmarked, the copy must give exactly the node's own row count (checked by
//! [`node_row_breakdown`]), so a drift of the node circuit shows up as an error, never as a
//! silently wrong breakdown. Rows the standard library assigns outside every section (shared
//! tables, deferred range checks) are reported as `other_rows`.
use std::io;

use ff::Field;
use group::Group;
use midnight_circuits::{
    instructions::{
        ArithInstructions, AssertionInstructions, AssignmentInstructions, BinaryInstructions,
        ControlFlowInstructions, PublicInputInstructions, ZeroInstructions,
    },
    types::AssignedNative,
    verifier::{AssignedAccumulator, AssignedVk},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::cost_model::{COST_MEASURE_END, COST_MEASURE_START},
    plonk::{ConstraintSystem, Error},
    poly::EvaluationDomain,
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch, cost_model};
use mori_recursion::{
    decider::bls_arch,
    inner::{C, S, verify_child_incircuit_with},
    ivc::{SELF_NAME, SherlockIvcRelation, node_cs},
    util::F,
};

/// The node's gadgets in circuit order (`ivc.rs` steps 1-5).
pub const SECTIONS: [&str; 5] = [
    "own_key_and_state",
    "prepare_self",
    "prepare_leaf",
    "accumulate",
    "collapse_and_publish",
];

#[derive(Clone, Debug)]
struct NodeRowProbe {
    nr_cols: u8,
    leaf_vk: MidnightVK,
    leaf_name: String,
    self_base_names: Vec<String>,
    self_domain: EvaluationDomain<F>,
    self_cs: ConstraintSystem<F>,
    measured: Option<usize>,
}

impl NodeRowProbe {
    fn mark(&self, l: &mut impl Layouter<F>, section: usize, start: bool) {
        if self.measured == Some(section) {
            let _ = l.namespace(|| {
                if start {
                    COST_MEASURE_START
                } else {
                    COST_MEASURE_END
                }
            });
        }
    }
}

impl Relation for NodeRowProbe {
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
        let verifier = std.verifier();
        let bls = std.bls12_381();
        let scalar = bls.scalar_field_chip();
        let u = || Value::<F>::unknown();

        self.mark(l, 0, true);
        let self_vk: AssignedVk<S> = verifier.assign_vk_as_public_input(
            l,
            SELF_NAME,
            &self.self_domain,
            &self.self_cs,
            u(),
        )?;
        let prog_d: AssignedNative<F> = std.assign(l, u())?;
        let init_d: AssignedNative<F> = std.assign(l, u())?;
        let prev_d: AssignedNative<F> = std.assign(l, u())?;
        let prev_h: AssignedNative<F> = std.assign(l, u())?;
        let cur_d: AssignedNative<F> = std.assign(l, u())?;
        let rule: AssignedNative<F> = std.assign(l, u())?;
        let binding: AssignedNative<F> = std.assign(l, u())?;
        let steps: AssignedNative<F> = std.assign(l, u())?;
        std.assert_not_equal_to_fixed(l, &steps, F::ZERO)?;
        let h = std.add(l, &prev_h, &steps)?;
        let is_genesis = std.is_zero(l, &prev_h)?;
        let not_genesis = std.not(l, &is_genesis)?;
        let chosen = std.select(l, &is_genesis, &init_d, &prev_d)?;
        std.assert_equal(l, &chosen, &prev_d)?;
        for x in [&prog_d, &init_d, &cur_d, &h] {
            std.constrain_as_public_input(l, x)?;
        }
        self.mark(l, 0, false);

        self.mark(l, 1, true);
        let prev_acc = AssignedAccumulator::<S>::assign(
            l,
            bls,
            scalar,
            1,
            1,
            &[],
            &self.self_base_names,
            Value::unknown(),
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
        let mut proof_acc =
            verifier.prepare(l, &self_vk, &[id], &[prev_pi.as_slice()], Value::unknown())?;
        AssignedAccumulator::scale_by_bit(l, scalar, &not_genesis, &mut proof_acc)?;
        self.mark(l, 1, false);

        self.mark(l, 2, true);
        let leaf_inst = vec![binding, prog_d, prev_d, cur_d, rule, prev_h, steps];
        let leaf_acc = verify_child_incircuit_with(
            std,
            l,
            &self.leaf_vk,
            &self.leaf_name,
            &leaf_inst,
            Value::unknown(),
            false,
            false,
        )?;
        self.mark(l, 2, false);

        self.mark(l, 3, true);
        let mut next = verifier.accumulate(l, &[proof_acc, prev_acc, leaf_acc])?;
        self.mark(l, 3, false);
        self.mark(l, 4, true);
        next.collapse(l, bls, scalar)?;
        verifier.constrain_as_public_input(l, &next)?;
        self.mark(l, 4, false);
        Ok(())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(self.nr_cols)
    }

    fn write_relation<W: io::Write>(&self, _: &mut W) -> io::Result<()> {
        Err(io::Error::other("modeling-only probe"))
    }

    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("modeling-only probe"))
    }
}

/// Rows of one node at `k`: the total from the node relation itself and the row span of each
/// section in [`SECTIONS`].
#[derive(Clone, Debug)]
pub struct NodeRows {
    pub k: u32,
    pub model_k: u32,
    pub rows: usize,
    pub table_rows: usize,
    pub unusable_rows: usize,
    pub sections: Vec<(&'static str, usize)>,
    /// `rows` minus the sum of the section spans; negative when spans interleave.
    pub other_rows: i64,
}

impl NodeRows {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "k": self.k, "model_k": self.model_k, "rows": self.rows, "table_rows": self.table_rows,
            "unusable_rows": self.unusable_rows, "rows_limit": 1u64 << self.k,
            "sections": self.sections.iter().map(|(n, r)| serde_json::json!({"gadget": n, "rows": r}))
                .collect::<Vec<_>>(),
            "other_rows": self.other_rows,
            "method": "cost_model row span per gadget (COST_MEASURE markers) over a copy of the node circuit, checked equal to the node's own row count unmarked",
        })
    }
}

/// Count the rows of `relation` and break them down by gadget. Fails when the copy drifts
/// from the node circuit (the unmarked probe and the node disagree on rows).
pub fn node_row_breakdown(relation: &SherlockIvcRelation) -> io::Result<NodeRows> {
    let k = relation.self_k;
    let model = cost_model(relation, Some(k));
    let self_cs = node_cs(relation.nr_cols, k);
    let mut probe = NodeRowProbe {
        nr_cols: relation.nr_cols,
        leaf_vk: relation.leaf_vk.clone(),
        leaf_name: relation.leaf_name(),
        self_base_names: relation.self_base_names(),
        self_domain: EvaluationDomain::new(self_cs.degree() as u32, k),
        self_cs,
        measured: None,
    };
    let copy = cost_model(&probe, Some(k));
    if copy.rows != model.rows {
        return Err(io::Error::other(format!(
            "node row probe drifted from ivc.rs: probe {} rows, node {} rows",
            copy.rows, model.rows
        )));
    }
    let mut sections = Vec::new();
    for (index, name) in SECTIONS.iter().enumerate() {
        probe.measured = Some(index);
        sections.push((*name, cost_model(&probe, Some(k)).rows));
    }
    let spans: usize = sections.iter().map(|(_, r)| r).sum();
    Ok(NodeRows {
        k,
        model_k: model.k,
        rows: model.rows,
        table_rows: model.table_rows,
        unusable_rows: model.nb_unusable_rows,
        sections,
        other_rows: model.rows as i64 - spans as i64,
    })
}
