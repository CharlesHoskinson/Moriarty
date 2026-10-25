//! WitnessPlan materialization of an owner boundary under four schedules.
//!
//! The complete `HostSnapshot` of the boundary is captured once. One plan is
//! constructed and then evaluated by the uncached baseline, the last-use
//! schedule, a memoized schedule with a fresh (cold) cache, and the same
//! memoized schedule again on that cache (warm). The decoded snapshots must be
//! identical; the projection published is the uncached baseline's. Logical
//! host diagnostics (evaluated nodes, copies, live slots, cache hits) stay
//! separate per schedule and are not bytes, time or semantic work.
//!
//! The projection is derived from the decoded DAG output only; no expectation
//! is produced here. Callers compare it with independently authored values.
use serde_json::{Value, json};

use crate::sherlock::helper_machine::complete_observation::capture;
use crate::sherlock::helper_machine::shared::{Admitted, MachineImage};
use crate::sherlock::helper_machine::witness_plan::{
    DecodedValue, HostDiagnostics, SlotStorage, SlotValue, StrategyDiagnostics, ValueNode,
    WitnessCache, construct_plan, evaluate_last_use, evaluate_memoized, evaluate_uncached,
};

pub const FORMAT: &str = "moriarty-sherlock-witness-materialization/1";

/// Structural value: UInt as decimal string, Bool, lists as arrays, trees and
/// constructors as tagged objects. The arena lists children before parents.
fn value(v: &DecodedValue) -> Value {
    let mut built: Vec<Value> = Vec::with_capacity(v.nodes.len());
    for node in &v.nodes {
        let at = |i: &usize| built.get(*i).cloned().unwrap_or(Value::Null);
        let item = match node {
            ValueNode::UInt(n) => json!(n.to_string()),
            ValueNode::Bool(b) => json!(b),
            ValueNode::Nil => json!([]),
            ValueNode::U128OutOfRange => json!({"U128OutOfRange": true}),
            ValueNode::Cons { head, tail } => {
                let mut list = vec![at(head)];
                match at(tail) {
                    Value::Array(rest) => list.extend(rest),
                    other => list.push(json!({"improper_tail": other})),
                }
                Value::Array(list)
            }
            ValueNode::Leaf { value } => json!({"leaf": at(value)}),
            ValueNode::Branch { left, right } => json!({"branch": [at(left), at(right)]}),
            ValueNode::Constructor {
                type_id,
                ctor_id,
                fields,
            } => json!({
                "type_id": format!("{type_id:?}"),
                "ctor_id": format!("{ctor_id:?}"),
                "fields": fields.iter().map(at).collect::<Vec<_>>(),
            }),
        };
        built.push(item);
    }
    built.get(v.root).cloned().unwrap_or(Value::Null)
}

fn slot(s: &SlotValue) -> Value {
    match s {
        SlotValue::Source(raw) => json!({"source": format!("{raw:?}")}),
        SlotValue::Value(v) => json!({"value": value(v)}),
        SlotValue::Environment(entries) => json!({
            "environment": entries.iter().map(|e| json!({
                "binder": e.binder.to_string(),
                "name": e.name,
                "value": value(&e.value),
            })).collect::<Vec<_>>(),
        }),
        SlotValue::Payload(p) => json!({
            "payload": format!("{:?}", p.kind),
            "raw": format!("{:?}", p.raw),
            "fields": p.fields.iter().map(|f| json!({"field": f.field, "value": slot(&f.value)})).collect::<Vec<_>>(),
        }),
        SlotValue::Sequence { kind, items } => json!({
            "sequence": format!("{kind:?}"),
            "items": items.iter().map(slot).collect::<Vec<_>>(),
        }),
        SlotValue::Snapshot(snapshot) => json!({
            "snapshot": snapshot.fields.iter().map(|f| json!({"field": f.field, "value": slot(&f.value)})).collect::<Vec<_>>(),
        }),
    }
}

fn host(d: &HostDiagnostics) -> Value {
    json!({
        "evaluated_nodes": d.evaluated_nodes.to_string(),
        "input_reads": d.input_reads.to_string(),
        "value_constructions": d.value_constructions.to_string(),
        "environment_constructions": d.environment_constructions.to_string(),
        "payload_constructions": d.payload_constructions.to_string(),
        "copied_value_nodes": d.copied_value_nodes.to_string(),
        "copied_environment_entries": d.copied_environment_entries.to_string(),
        "retained_slots": d.retained_slots.to_string(),
        "measured_host_bytes": d.measured_host_bytes.map(|b| b.to_string()),
        "measured_host_time_ns": d.measured_host_time_ns.map(|t| t.to_string()),
    })
}

fn scheduling(d: &StrategyDiagnostics) -> Value {
    json!({
        "scheduled_nodes": d.scheduled_nodes.to_string(),
        "cache_hits": d.cache_hits.to_string(),
        "cache_misses": d.cache_misses.to_string(),
        "scope_reset": d.scope_reset,
        "copied_cached_slot_values": d.copied_cached_slot_values.to_string(),
        "reclaimed_slots": d.reclaimed_slots.to_string(),
        "peak_live_slots": d.peak_live_slots.to_string(),
        "retained_slots_at_publication": d.retained_slots_at_publication.to_string(),
        "cached_value_slots": d.cached_value_slots.to_string(),
        "cached_argument_slots": d.cached_argument_slots.to_string(),
    })
}

/// Materialize the boundary of the bound pair under all four schedules.
pub fn materialize_boundary(admitted: &Admitted, image: &MachineImage) -> Value {
    let refused = |stage: &str, detail: String| json!({"format": FORMAT, "status": "refused", "stage": stage, "detail": detail});
    let snapshot = match capture(admitted, image) {
        Ok(s) => s,
        Err(v) => return refused("capture", format!("{v:?}")),
    };
    let plan = match construct_plan(&snapshot) {
        Ok(p) => p,
        Err(e) => return refused("plan", format!("{e:?}")),
    };
    let uncached = match evaluate_uncached(&plan, &snapshot) {
        Ok(e) => e,
        Err(e) => return refused("uncached", format!("{e:?}")),
    };
    let last_use = match evaluate_last_use(&plan, &snapshot) {
        Ok(e) => e,
        Err(e) => return refused("last-use", format!("{e:?}")),
    };
    let mut cache = WitnessCache::default();
    let cold = match evaluate_memoized(&plan, &snapshot, &mut cache, SlotStorage::RetainAll) {
        Ok(e) => e,
        Err(e) => return refused("cold-cache", format!("{e:?}")),
    };
    let warm = match evaluate_memoized(&plan, &snapshot, &mut cache, SlotStorage::RetainAll) {
        Ok(e) => e,
        Err(e) => return refused("warm-cache", format!("{e:?}")),
    };
    let baseline = &uncached.decoded;
    let same = [&last_use, &cold, &warm]
        .iter()
        .all(|s| &s.witness.decoded == baseline);
    let order: Vec<String> = plan
        .stable_topological_order
        .iter()
        .map(|n| n.0.to_string())
        .collect();
    let decoded: serde_json::Map<String, Value> = baseline
        .fields
        .iter()
        .map(|f| (f.field.to_string(), slot(&f.value)))
        .collect();
    json!({
        "format": FORMAT,
        "status": "materialized",
        "plan": {
            "format": plan.format,
            "lifetime": format!("{:?}", plan.lifetime),
            "inputs": plan.inputs.len().to_string(),
            "nodes": plan.nodes.len().to_string(),
            "stable_order_is_identity": order.iter().enumerate().all(|(i, n)| *n == i.to_string()),
            "live_outs": plan.live_outs.len().to_string(),
        },
        "schedules_agree": same,
        "decoded": {
            "consumed_work": baseline.metadata.consumed_work.to_string(),
            "remaining_work": baseline.metadata.remaining_work.to_string(),
            "helper_work": baseline.metadata.helper_work.to_string(),
            "helper_visits": baseline.metadata.helper_visits.to_string(),
            "reference_cursor": baseline.metadata.reference_cursor.to_string(),
            "fields": decoded,
        },
        "cache_key_scope": ["format", "lifetime", "admission (program, profile, layout, bounds)", "nodes", "binding paths and types", "immutable argument values"],
        "diagnostics": {
            "uncached": host(&uncached.diagnostics),
            "last_use": {"host": host(&last_use.witness.diagnostics), "scheduling": scheduling(&last_use.scheduling)},
            "cold_cache": {"host": host(&cold.witness.diagnostics), "scheduling": scheduling(&cold.scheduling)},
            "warm_cache": {"host": host(&warm.witness.diagnostics), "scheduling": scheduling(&warm.scheduling)},
            "qualification": "logical host diagnostics only; not bytes, time, prover memory or semantic work",
        },
    })
}
