//! SDK-free diagnostic of the real A102 -> A80 host producer, not the checked
//! composed bridge, a 335-lane statement, a circuit execution or an acceptance result.
use literal_host::literal_witness::validate_literal_schema;
use literal_host::return_publication_witness::produce_a102_literal_return_publication;
use literal_host::{ImageValues, LiteralCapacities, LiteralLane, LiteralSourceIdentity, LiteralTable};
use mori_core::sherlock::layout_a::StateResult;
use mori_core::sherlock::{Expr, Program, Value};
use serde_json::{json, Value as Json};
use std::{io::{self, Write}, sync::Arc};

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

/// The same public minimal UInt(7), fuel=1, stack=16 specimen as the earlier
/// external literal client. These capacities are specimen inputs, not defaults
/// or resource admission. Caller identity is retained data, not authenticated.
/// Producer failures remain typed StateFailure values at this API boundary.
pub fn capture_composed(identity: LiteralSourceIdentity) -> StateResult<Json> {
    let capacities = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 16,
    };
    let program = Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    // Exactly one end-to-end producer invocation. The production producer runs
    // one Machine through A102 and then A80; this client never steps a machine.
    let produced = produce_a102_literal_return_publication(
        program, &capacities, 1, 16, &identity,
    )?;
    let request = &produced.literal_transition.request;
    let schema = validate_literal_schema(&request.capacities)?;
    let literal = &request.values;
    let returned = &produced.values;

    // LiteralCompleteValues field/serialization order, without its SDK-dependent
    // request adapter or checked layout. Only returned ImageValues are flattened.
    let mut ordered = Vec::<String>::new();
    for image in [&literal.program, &literal.pre, &literal.post, &returned.post] {
        ordered.extend(image.root.iter()
            .chain(image.tables.iter().flatten().flatten())
            .map(u128::to_string));
    }
    ordered.extend([
        literal.rule_tag, literal.charge, returned.rule_tag, returned.charge,
    ].iter().map(u128::to_string));
    let ordered_length = ordered.len().to_string();
    let first_record = &produced.literal_transition.step_record;
    let second_record = &produced.step_record;

    Ok(json!({
        "schema": "moriarty.sherlock-composed-real-host-capture/1",
        "payload": {
            "program": image_values(&literal.program),
            "init": image_values(&literal.pre),
            "intermediate": image_values(&literal.post),
            "done": image_values(&returned.post),
            "literal_envelope": {
                "rule_tag": literal.rule_tag.to_string(),
                "charge": literal.charge.to_string(),
            },
            "return_envelope": {
                "rule_tag": returned.rule_tag.to_string(),
                "charge": returned.charge.to_string(),
            },
        },
        "ordered_non_context_payload": ordered,
        "metadata": {
            "caseid": "first-literal-minimal-step-normal",
            "input": {"literal_uint": "7", "initial_fuel": "1", "stack_limit": "16"},
            "capacities": {
                "program": request.capacities.program.iter().map(usize::to_string).collect::<Vec<_>>(),
                "state": request.capacities.state.iter().map(usize::to_string).collect::<Vec<_>>(),
                "stack_limit": request.capacities.stack_limit.to_string(),
            },
            "source_identity": {
                "k_source_sha256": request.identity.k_source_sha256,
                "reference_source_sha256": request.identity.reference_source_sha256,
                "program_source_sha256": request.identity.program_source_sha256,
            },
            "source_identity_qualification": "Caller-supplied retained data; this client opens or authenticates no source bytes",
            "schema_metadata": {
                "program_header": schema.program_header.iter().map(lane_schema).collect::<Vec<_>>(),
                "program_tables": schema.program_tables.iter().map(table_schema).collect::<Vec<_>>(),
                "state_root": schema.state_root.iter().map(lane_schema).collect::<Vec<_>>(),
                "state_tables": schema.state_tables.iter().map(table_schema).collect::<Vec<_>>(),
            },
            "ordering": "program, init, intermediate, done: each root then all tables in schema order, row-major/lane-major; then literal rule_tag/charge, return rule_tag/charge",
            "ordered_non_context_payload_length": ordered_length,
            "steps": [
                {"rule_start_line": first_record.rule_start_line.to_string(), "charge": first_record.charge.to_string()},
                {"rule_start_line": second_record.rule_start_line.to_string(), "charge": second_record.charge.to_string()},
            ],
            "composed_context": "unbound; two context limbs omitted without placeholders",
            "checked_composed_bridge": "not called; no checked request or 335-lane statement tested",
            "qualification": "H1 trusted-host producer diagnostic; Step-mode Constant-space intermediate/done; no canonical projection, circuit, proof, native, ledger or settlement evidence",
            "acceptance": "none",
        },
    }))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("Usage: capture_composed K_SOURCE_SHA256 REFERENCE_SOURCE_SHA256 PROGRAM_SOURCE_SHA256 (all explicit; no defaults)".into());
    }
    // Reuse the existing public digest-syntax API; add no identity policy.
    for digest in &args {
        literal_host::decode_digest(digest).map_err(|error| error.to_string())?;
    }
    let identity = LiteralSourceIdentity {
        k_source_sha256: args[0].clone(),
        reference_source_sha256: args[1].clone(),
        program_source_sha256: args[2].clone(),
    };
    let capture = capture_composed(identity).map_err(|failure| format!("{failure:?}"))?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, &capture).map_err(|error| error.to_string())?;
    out.write_all(b"\n").map_err(|error| error.to_string())
}
