//! External SDK-free normal producer client. Source only until separately admitted.
//! Uses the real host producer and mori_core types; contains no evaluator or oracle.
use literal_host::literal_witness::build_checked_literal_transition_request;
use literal_host::{
    literal_schema, ImageValues, LiteralCapacities, LiteralLane, LiteralSourceIdentity,
    LiteralTable, BINDING_VERSION, LEDGER_REVISION, OPEN_OBLIGATIONS, PROFILE, SCOPE,
    STATEMENT_FORMAT, ZKIR_VERSION,
};
use mori_core::sherlock::{Expr, Program, Snapshot, Value};
use serde_json::{json, Value as Json};
use std::{io::{self, Write}, sync::Arc};

fn supplied_digest(text: &str) -> Result<String, String> {
    literal_host::decode_digest(text).map_err(|e| e.to_string())?;
    if text.bytes().all(|b| b == b'0') {
        return Err("Supply an actual externally computed digest, not a zero placeholder".into());
    }
    Ok(text.to_owned())
}

fn lane_schema(lane: &LiteralLane) -> Json {
    json!({
        "name": lane.name,
        "bits": lane.bits.to_string(),
        "boolean": lane.boolean,
        "maximum": lane.maximum.map(|n| n.to_string()),
    })
}

fn table_schema(table: &LiteralTable) -> Json {
    json!({
        "name": table.name,
        "capacity": table.capacity.to_string(),
        "lanes": table.lanes.iter().map(lane_schema).collect::<Vec<_>>(),
    })
}

/// Serialize all returned cells directly; empty tables and rows stay present.
fn image_values(image: &ImageValues) -> Json {
    let tables: Vec<Vec<Vec<String>>> = image.tables.iter()
        .map(|rows| rows.iter()
            .map(|row| row.iter().map(u128::to_string).collect())
            .collect())
        .collect();
    json!({
        "root": image.root.iter().map(u128::to_string).collect::<Vec<_>>(),
        "tables": tables,
    })
}

/// Metadata comes from the returned StepRecord, never a reconstructed state.
fn step_snapshot_metadata(snapshot: &Snapshot) -> Json {
    json!({
        "remaining_fuel": snapshot.remaining_fuel.to_string(),
        "initial_fuel": snapshot.initial_fuel.to_string(),
        "stack_limit": snapshot.stack_limit.to_string(),
        "control_debug": format!("{:?}", snapshot.control),
        "environment_debug": format!("{:?}", snapshot.environment),
        "frames_debug": format!("{:?}", snapshot.frames),
        "outcome_debug": format!("{:?}", snapshot.outcome),
        "rendering": "Debug text is supplemental host diagnostic metadata, not a persistence ABI",
    })
}

/// Capture the single normal case specified in PUBLIC-INPUT.json.
/// Identity values are caller-bound diagnostics, not authenticated source claims.
pub fn capture_literal(
    identity: LiteralSourceIdentity,
    max_statement_lanes: usize,
) -> Result<Json, String> {
    // Validate the public CLI contract for direct callers as well. No fallback IDs.
    supplied_digest(&identity.k_source_sha256)?;
    supplied_digest(&identity.reference_source_sha256)?;
    supplied_digest(&identity.program_source_sha256)?;
    let capacities = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 16,
    };
    let program = Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    // The ONLY producer/evaluator entry call made by this client.
    let (produced, statement) = build_checked_literal_transition_request(
        program, &capacities, 1, 16, &identity, max_statement_lanes,
    ).map_err(|e| e.to_string())?;
    let request = &produced.request;
    let schema = literal_schema(&request.capacities)
        .map_err(|e| format!("public schema construction: {e:?}"))?;
    let values = &request.values;
    let record = &produced.step_record;
    Ok(json!({
        "schema": "moriarty.sherlock-literal-producer-normal-capture/1",
        "caseid": "first-literal-minimal-step-normal",
        "qualification": "SDK-free caller-bound host diagnostic capture; no source frontend, native ABI, proof, ledger or settlement claim",
        "identity_qualification": "Three externally computed caller-supplied source byte hashes; syntax checked, source contents not opened or authenticated by this client",
        "format": STATEMENT_FORMAT,
        "binding_version": BINDING_VERSION,
        "profile": PROFILE,
        "scope": SCOPE,
        "ledger_revision_label": LEDGER_REVISION,
        "zkir_version_label": ZKIR_VERSION,
        "input": {
            "program": {"functions": [], "entry": {"tag": "Literal", "value": {"tag": "UInt", "n": "7"}}},
            "initial_fuel": "1",
            "stack_limit": "16",
            "max_statement_lanes": max_statement_lanes.to_string(),
        },
        "request": {
            "capacities": {
                "program": request.capacities.program.iter().map(usize::to_string).collect::<Vec<_>>(),
                "state": request.capacities.state.iter().map(usize::to_string).collect::<Vec<_>>(),
                "stack_limit": request.capacities.stack_limit.to_string(),
            },
            "identity": {
                "k_source_sha256": request.identity.k_source_sha256,
                "reference_source_sha256": request.identity.reference_source_sha256,
                "program_source_sha256": request.identity.program_source_sha256,
            },
            "values": {
                "program": image_values(&values.program),
                "pre": image_values(&values.pre),
                "post": image_values(&values.post),
                "rule_tag": values.rule_tag.to_string(),
                "charge": values.charge.to_string(),
            },
        },
        "schema_metadata": {
            "image_order": ["program", "pre", "post"],
            "program_header": schema.program_header.iter().map(lane_schema).collect::<Vec<_>>(),
            "program_tables": schema.program_tables.iter().map(table_schema).collect::<Vec<_>>(),
            "state_root": schema.state_root.iter().map(lane_schema).collect::<Vec<_>>(),
            "state_tables": schema.state_tables.iter().map(table_schema).collect::<Vec<_>>(),
            "ordering": "Roots follow lane arrays; tables follow schema arrays; each table is row-major then lane-major; zero-capacity tables remain empty arrays",
        },
        "envelope": {"rule_tag": values.rule_tag.to_string(), "charge": values.charge.to_string()},
        "ordered_statement": statement.iter().map(u128::to_string).collect::<Vec<_>>(),
        "ordered_statement_length": statement.len().to_string(),
        "statement_order": "context limb.0, context limb.1, complete program root/tables, complete pre root/tables, complete post root/tables, envelope rule_tag, envelope charge; returned by producer without recomputation",
        "selected": {
            "expression_id": produced.selected_expr_id.to_string(),
            "constant_ref": {
                "space": produced.selected_constant_ref.space,
                "index": produced.selected_constant_ref.index.to_string(),
            },
        },
        "step_metadata": {
            "rule_start_line": record.rule_start_line.to_string(),
            "charge": record.charge.to_string(),
            "before": step_snapshot_metadata(&record.before),
            "after": step_snapshot_metadata(&record.after),
        },
        "open_obligations": OPEN_OBLIGATIONS,
        "acceptance": "none",
    }))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("Usage: capture_literal K_SOURCE_SHA256 REFERENCE_SOURCE_SHA256 PROGRAM_SOURCE_SHA256 MAX_STATEMENT_LANES (all four explicit; no defaults)".into());
    }
    let identity = LiteralSourceIdentity {
        k_source_sha256: supplied_digest(&args[0])?,
        reference_source_sha256: supplied_digest(&args[1])?,
        program_source_sha256: supplied_digest(&args[2])?,
    };
    let bound: usize = args[3].parse()
        .map_err(|_| "MAX_STATEMENT_LANES must be an explicit usize decimal".to_string())?;
    let capture = capture_literal(identity, bound)?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, &capture).map_err(|e| e.to_string())?;
    out.write_all(b"\n").map_err(|e| e.to_string())
}
