//! Curated public source capabilities, with this binary's actual Cargo features.
//! This query does not inspect files, targets, installations or runtime evidence.
use std::process::ExitCode;

use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Print JSON (the only output format)
    #[arg(long)]
    pub json: bool,
}

// Inspection basis for the curated consumer notes, not a binary/build identity.
// Update this catalogue when public routing or editor implementations change.
const SOURCE_REVISION: &str = "214b377daf372f7df1506c49d3e15148f9931012";

fn profile(
    id: &str,
    source: &str,
    semantics: Option<&str>,
    parser: &str,
    header_keyword: &str,
    types: &[&str],
) -> Value {
    json!({
        "id": id, "sourceProfile": source,
        "sourceHeader": format!("{header_keyword} \"{source}\";"),
        "semanticsProfile": semantics, "parser": parser,
        "implementation": "source-present",
        "grammar": {
            "coverage": "profile-selection-summary-not-grammar-completion",
            "types": types, "declarations": [], "statements": [],
            "expressions": [], "unsupported": [], "bounds": {},
            "notes": ["Grammar is not enumerated here; use the actual parser/checker for admission."]
        },
        "qualification": "source-description-only",
        "limitations": ["Parser/checker support does not imply compiler or target qualification."]
    })
}

/// Metadata only. The profile tuple is (input, semantics, output); an optional
/// build requirement carries its actual cfg value, not a runtime probe.
fn consumer(
    id: &str,
    entrypoint: &str,
    profiles: (Option<&str>, Option<&str>, Option<&str>),
    build: Option<(&str, bool)>,
    status: (&str, &str),
    limitations: &[&str],
    source_refs: &[&str],
) -> Value {
    let requirements: Vec<_> = build.iter().map(|(name, _)| *name).collect();
    json!({
        "id": id, "entrypoint": entrypoint,
        "inputProfile": profiles.0, "semanticsProfile": profiles.1,
        "outputProfile": profiles.2, "requiresBuildFeatures": requirements,
        "implementation": status.0,
        "availableInBuild": status.0 == "source-present" && build.is_none_or(|(_, enabled)| enabled),
        "qualification": status.1, "limitations": limitations, "sourceRefs": source_refs
    })
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    use mori_core::stage::{LANGUAGE_2, LANGUAGE_3, SEMANTICS_2, SEMANTICS_3};
    use mori_core::{LANGUAGE, SEMANTICS};

    // JSON is the sole format; the flag is an explicit client spelling.
    let _ = args.json;
    let native = cfg!(feature = "native");
    let source6 = cfg!(feature = "source6-expansion");
    let disjoint = cfg!(feature = "core2-disjoint-draft");
    let report = json!({
        "format": "moriarty-cli-capabilities/1",
        "producer": {
            "implementation": "rust-workspace-cli", "packageVersion": env!("CARGO_PKG_VERSION"),
            "binarySourceRevision": null, "responseKind": "command-self-report",
            "buildFeatures": {"native": native, "source6-expansion": source6, "core2-disjoint-draft": disjoint}
        },
        "catalogue": {
            "sourceRevision": SOURCE_REVISION, "coverage": "selected-public-profile-consumers",
            "basis": "source-inspection", "runtimeEvidence": "not-included",
            "note": "sourceRevision is a historical inspection basis, not the exact source commit for every row or a binary revision. Public Core/3 rows reflect the later Core/3 consumer candidate. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update."
        },
        "profiles": [
            profile("rust-core1", LANGUAGE, Some(SEMANTICS), "mori_core::parse", "language", &["UInt", "Bool"]),
            profile("rust-core2", LANGUAGE_2, Some(SEMANTICS_2), "mori_core::stage::load", "language", &["UInt", "Bool"]),
            profile("rust-core3", LANGUAGE_3, Some(SEMANTICS_3), "mori_core::stage::load_v3", "language", &["UInt", "Bool"]),
            profile("rust-beta-frontend", mori_parser::PROFILE, None, "mori_parser::parse then mori_checker::check", "profile", &[])
        ],
        "consumers": [
            consumer("core1-check", "check FILE --json", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "source-check"), &["No compilation or target qualification."], &["crates/mori/src/commands/check.rs"]),
            consumer("core2-check", "check FILE --json", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "source-check"), &["Uses staged admission and planning."], &["crates/mori/src/commands/check.rs"]),
            consumer("core3-check", "check FILE --json", (Some(LANGUAGE_3), Some(SEMANTICS_3), None), None,
                ("source-present", "source-check"), &["Uses load_v3 admission and planning; no compilation or target qualification."], &["crates/mori/src/commands/check_source.rs"]),
            consumer("core3-reference", "simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE", (Some(LANGUAGE_3), Some(SEMANTICS_3), None), None,
                ("source-present", "local-reference-stage"), &["Complete staged outcomes; authentication none and consumption untracked.", "No native trace, proof or ledger result is supplied."], &["crates/mori/src/commands/simulate.rs"]),
            consumer("core3-inspect", "inspect FILE", (Some(LANGUAGE_3), Some(SEMANTICS_3), None), None,
                ("source-present", "local-reference-stage"), &["Full source declarations including duties and stage plan; no compiled enforcement map."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core3-test", "test DIRECTORY", (Some("moriarty-tests/3"), Some(SEMANTICS_3), None), None,
                ("source-present", "local-reference-stage"), &["Closed manifest; whole supplied expected reports compare against actual start/resume results."], &["crates/mori/src/commands/test/stage_test.rs"]),
            consumer("core3-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE_3), Some(SEMANTICS_3), None), Some(("native", native)),
                ("unsupported", "profile-refused"), &["No Core/3 staged artifact/context/registry compiler contract is implemented."], &["crates/mori/src/commands/compile.rs"]),
            consumer("core3-target-inspect", "inspect FILE --target FILE", (Some(LANGUAGE_3), Some(SEMANTICS_3), None), Some(("native", native)),
                ("unsupported", "profile-refused"), &["No Core/3 compiler enforcement map is implemented."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core3-native-trace", "simulate FILE --trace --target FILE", (Some(LANGUAGE_3), Some(SEMANTICS_3), None), Some(("native", native)),
                ("unsupported", "profile-refused"), &["Native replay/proof/ledger consumers are separate and retain their existing profile boundaries."], &["crates/mori/src/commands/simulate.rs"]),
            consumer("beta-check", "check FILE --json", (Some(mori_parser::PROFILE), None, None), None,
                ("source-present", "frontend-check-only"), &["Named-action admission is determined by the checker; no Core semantics are inferred."], &["crates/mori/src/commands/check.rs"]),
            consumer("core1-reference", "simulate FILE --action NAME --scenario FILE", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "local-reference-execution"), &["--trace is a separate native-feature consumer."], &["crates/mori/src/commands/simulate.rs"]),
            consumer("core2-reference", "simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.", "Authentication is none; consumption is untracked."], &["crates/mori/src/commands/simulate.rs", "crates/mori-core/src/stage/mod.rs"]),
            consumer("core1-inspect", "inspect FILE", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-enforcement-report/1")), None,
                ("source-present", "source-only-unqualified"), &["No target was supplied or compiled."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-inspect", "inspect FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-target-inspect", "inspect FILE --target FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), Some(("native", native)),
                ("unsupported", "profile-refused"), &["Core/2 target inspection refuses even though Core/2 has a separate compiler."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-disjoint-draft", "test DIRECTORY", (Some("moriarty-tests/disjoint-draft/1"), Some(SEMANTICS_2), None), Some(("core2-disjoint-draft", disjoint)),
                ("source-present", "unadopted-local-disjoint-product"), &["Opt-in draft; not required for base Core/2 check, start or resume."], &["crates/mori/src/commands/test.rs"]),
            consumer("beta-to-source6-transfer", "expand FILE --emit-source6-transfer --action NAME --scenario FILE", (Some(mori_parser::PROFILE), None, Some("Source/6")), Some(("source6-expansion", source6)),
                ("source-present", "local-stipulation-only"), &["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.", "Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."], &["crates/mori/src/commands/expand.rs", "crates/mori/src/commands/expand/source6.rs"]),
            consumer("core1-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-zkir-artifact/3")), Some(("native", native)),
                ("source-present", "compiler-source-present"), &["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."], &["crates/mori/src/commands/compile.rs"]),
            consumer("core2-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE_2), Some(SEMANTICS_2), Some("moriarty-zkir-artifact/4")), Some(("native", native)),
                ("source-present", "staged-compiler-source-present"), &["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.", "Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."], &["crates/mori/src/commands/compile.rs", "crates/mori-backend/src/stage/mod.rs"]),
            consumer("rust-lsp", "lsp", (None, None, None), None,
                ("stub", "not-implemented"), &["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."], &["crates/mori/src/commands/lsp.rs"]),
            consumer("rust-mcp", "mcp", (None, None, None), None,
                ("stub", "not-implemented"), &["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."], &["crates/mori/src/commands/mcp.rs"])
        ],
        "targets": [{
            "id": "midnight-ledger9-zkirv3", "consumerIds": ["core1-compile", "core2-compile"],
            "descriptorRequired": true, "configuredTarget": null, "qualification": "not-checked",
            "notes": ["No target, network, anchor, parameters, prover, verifier or deployment was inspected."]
        }],
        "otherExecutables": [{
            "id": "node-beta-cli", "package": "@moriarty-lang/beta", "binaryName": "mori",
            "relationship": "separate executable; no Rust capability inheritance", "availability": "not-inspected"
        }],
        "limitations": [
            "availableInBuild means the source path is compiled in, not that a request passed admission or execution.",
            "This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.",
            "No runtime, proof, installation, deployment or ledger qualification is established."
        ]
    });
    super::core_io::print(&report)?;
    Ok(ExitCode::SUCCESS)
}
