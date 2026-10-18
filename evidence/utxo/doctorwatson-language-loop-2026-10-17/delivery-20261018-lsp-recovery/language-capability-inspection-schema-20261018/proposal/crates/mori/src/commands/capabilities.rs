//! Public build capability catalogue; no source file or target is evaluated.
use std::process::ExitCode;

use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Emit the capability JSON response (the only output format).
    #[arg(long)]
    pub json: bool,
}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let mut report: Value = serde_json::from_str(include_str!("capabilities.catalogue.json"))
        .map_err(super::core_io::operation)?;
    let features = json!({
        "native": cfg!(feature = "native"),
        "source6-expansion": cfg!(feature = "source6-expansion"),
        "core2-disjoint-draft": cfg!(feature = "core2-disjoint-draft"),
    });
    report["producer"] = json!({
        "implementation": "rust-workspace-cli",
        "packageVersion": env!("CARGO_PKG_VERSION"),
        "binarySourceRevision": null,
        "buildFeatures": features.clone(),
        "responseKind": "command-self-report",
    });
    report["catalogue"]["note"] = json!(
        "Source catalogue with actual compiled feature flags; no target or document was evaluated."
    );
    let consumers = report["consumers"].as_array_mut().ok_or_else(|| {
        super::core_io::operation("Capability catalogue consumers must be an array.")
    })?;
    for consumer in consumers {
        let dependencies = consumer["requiresBuildFeatures"].as_array().ok_or_else(|| {
            super::core_io::operation("Capability build dependencies must be an array.")
        })?;
        let available = consumer["implementation"] == "source-present"
            && dependencies.iter().all(|feature| {
                feature.as_str().is_some_and(|name| features[name] == true)
            });
        consumer["availableInBuild"] = json!(available);
    }
    super::core_io::print(&report)?;
    Ok(ExitCode::SUCCESS)
}
