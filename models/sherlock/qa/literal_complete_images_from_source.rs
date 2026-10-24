//! Proposed actual-producer input exporter; SOURCE ONLY / UNCOMPILED / UNRUN.
//! Backend linkage is SDK-PENDING; this is not labelled an SDK-free consumer.
//! Exports only safe input, never the producer's intermediate/done/statement.
#[path = "literal_complete_images_export.rs"]
mod export;
use mori_backend::sherlock_machine::{
    literal_complete_bridge::produce_checked_literal_complete_execution_request,
    literal_host::{LiteralCapacities, LiteralSourceIdentity},
};
use mori_core::sherlock_compile_artifact::draft_reference::compile_draft_with_reference;
use sha2::{Digest, Sha256};
use std::{fs, io::{self, Write}, path::PathBuf, sync::Arc};

fn sha(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Explicit paths, including identity-bearing source bytes; no environment/default IDs.
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    let [source_path, config_path, target_path, k_path, reference_path, output_path] = args.as_slice()
        else { return Err(io::Error::new(io::ErrorKind::InvalidInput,
            "expected SOURCE CONFIG TARGET K_MAIN REFERENCE FRESH_OUTPUT").into()); };
    let source = fs::read_to_string(source_path)?;
    let config = fs::read_to_string(config_path)?;
    let target = fs::read_to_string(target_path)?;
    let prepared = compile_draft_with_reference(&source, &config, &target)
        .map_err(|error| io::Error::other(format!(
            "public source preparation refused: stage={} actual_error={} evaluation_performed={} events_forwarded={} completed={}",
            error.stage, error.actual_error, error.evaluation_performed,
            error.events_forwarded, error.completed,
        )))?;
    let identity = LiteralSourceIdentity {
        k_source_sha256: sha(&fs::read(k_path)?),
        reference_source_sha256: sha(&fs::read(reference_path)?),
        program_source_sha256: sha(source.as_bytes()),
    };
    // The finite existing L4 scalar-literal profile, not a general-program capacity default.
    let capacities = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1], stack_limit: 0,
    };
    // Genuine same-Arc producer and complete-layout API; no manual post-state encoding.
    // This invocation is only proposed source. Root must classify its actual backend closure.
    let (_produced, layout, request, _statement) = produce_checked_literal_complete_execution_request(
        Arc::clone(prepared.reference_program()), &capacities, 1, 0, &identity, 335,
    ).map_err(|error| io::Error::other(format!("actual complete-pair producer refused: {error}")))?;
    let mut output = fs::OpenOptions::new().write(true).create_new(true).open(output_path)?;
    export::write_input(&request, layout.context_limbs(), 2, &mut output)?;
    output.flush()?;
    Ok(())
}
