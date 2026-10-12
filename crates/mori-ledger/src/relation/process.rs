//! Mirrors the pinned SDK's one-activity-record-per-Op Noop expansion.
use super::{Artifact, refusal};
use crate::{Result, codec, ops::NativeOp};
use midnight_base_crypto::cost_model::CostDuration;
use midnight_onchain_runtime::cost_model::CostModel;
use midnight_onchain_vm::ops::Op;
use midnight_transient_crypto::{curve::Fr, repr::FieldRepr};

pub struct ProcessedOps {
    pub program: Vec<NativeOp>,
    pub operation_fields: Vec<Fr>,
    pub noop_cost: CostDuration,
    pub noop_count: usize,
    pub inactive_fields: usize,
}
pub fn process(
    a: &Artifact,
    raw: &[NativeOp],
    activity: &[Option<usize>],
    cost: &CostModel,
) -> Result<ProcessedOps> {
    if activity.len() != a.manifest.groups.len() {
        return Err(refusal("activity-count"));
    }
    let mut program: Vec<NativeOp> = vec![];
    let mut operation_fields = vec![];
    let mut cursor = 0usize;
    let mut inactive_fields = 0usize;
    for (skip, g) in activity.iter().zip(&a.manifest.groups) {
        if g.field_count == 0
            || operation_fields
                .len()
                .checked_add(g.field_count)
                .is_none_or(|n| n > codec::OUTPUT_BYTES / 32)
        {
            return Err(refusal("statement-field-bound"));
        }
        let op = if let Some(n) = skip {
            if *n != g.field_count {
                return Err(refusal("skip-width"));
            }
            inactive_fields = inactive_fields
                .checked_add(*n)
                .ok_or(refusal("skip-overflow"))?;
            operation_fields.extend(std::iter::repeat_n(Fr::from(0), *n));
            Op::Noop {
                n: u32::try_from(*n).map_err(|_| refusal("skip-overflow"))?,
            }
        } else {
            let op = raw.get(cursor).ok_or(refusal("missing-active-op"))?;
            cursor += 1;
            if op.field_size() != g.field_count {
                return Err(refusal("active-op-width"));
            }
            operation_fields.extend(op.field_vec());
            op.clone()
        };
        match (program.last_mut(), &op) {
            (Some(Op::Noop { n: prior }), Op::Noop { n }) => {
                *prior = prior.checked_add(*n).ok_or(refusal("skip-overflow"))?
            }
            _ => program.push(op),
        }
    }
    if cursor != raw.len() {
        return Err(refusal("extra-active-op"));
    }
    let mut noop_cost = CostDuration::ZERO;
    let mut noop_count = 0;
    for op in &program {
        if let Op::Noop { n } = op {
            noop_count += 1;
            noop_cost += cost.noop_constant + cost.noop_coeff_arg * u64::from(*n);
        }
    }
    if program
        .iter()
        .flat_map(FieldRepr::field_vec)
        .collect::<Vec<_>>()
        != operation_fields
    {
        return Err(refusal("noop-projection"));
    }
    Ok(ProcessedOps {
        program,
        operation_fields,
        noop_cost,
        noop_count,
        inactive_fields,
    })
}
