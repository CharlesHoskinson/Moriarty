    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;
    let projection = serde_json::to_string_pretty(&serde_json::json!({
        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
        "compilation": reported.source_projection,
    }))
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    publish_artifact(&artifact, &manifest, Some(&projection), &args.out)?;
    shell.status(
        "Compiled",
        format!(
            "{} circuits into {}",
            artifact.circuits.len(),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}

// The caller has reserved a new directory after all input/compiler preflight.
#[cfg(feature = "native")]
fn publish_artifact(
    artifact: &mori_backend::Artifact,
    manifest: &str,
    projection: Option<&str>,
    out: &Path,
) -> super::Result<()> {
    for (index, circuit) in artifact.circuits.iter().enumerate() {
        let path = out.join(format!("{index:03}-{}.zkir", circuit.action));
        super::proof_io::publish_new(&path, circuit.zkir.as_bytes())?;
    }
    if let Some(projection) = projection {
        let path = out.join("source-projection.json");
        super::proof_io::publish_new(&path, projection.as_bytes())?;
    }
    let path = out.join("artifact.json");
    // The manifest is the completion marker; publish it after every circuit.
    super::proof_io::publish_new(&path, manifest.as_bytes())?;
    Ok(())
}
