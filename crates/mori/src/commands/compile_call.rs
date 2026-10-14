use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: Option<String>,
    #[arg(long)]
    pub deployment: Option<PathBuf>,
    /// New output directory, containing tagged IR and static manifest
    #[arg(long)]
    pub out: Option<PathBuf>,
    #[arg(long)]
    pub all_actions: bool,
    #[arg(long)]
    pub static_bindings: Option<PathBuf>,
    #[arg(long)]
    pub authorization: Option<String>,
    #[arg(long)]
    pub signer_policy: Option<PathBuf>,
    #[arg(long)]
    pub work_limits: Option<PathBuf>,
    #[arg(long)]
    pub inspect_static: Option<PathBuf>,
}
pub(crate) fn artifact_bytes(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = vec![];
    file.take((mori_ledger::codec::OUTPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::OUTPUT_BYTES {
        return Err(core_io::operation("relation artifact exceeds output bound"));
    }
    Ok(bytes)
}
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    validate_shape(&args)?;
    if args.all_actions {
        return run_static(&args, shell);
    }
    let action = args.action.as_ref().expect("validated legacy action");
    let deployment_path = args
        .deployment
        .as_ref()
        .expect("validated legacy deployment");
    let out = args.out.as_ref().expect("validated legacy output");
    let (source, p) = core_io::program(&args.file)?;
    let deployment =
        mori_ledger::profile::Deployment::from_json(&core_io::read_source(deployment_path)?)
            .map_err(core_io::operation)?;
    let artifact = mori_ledger::relation::compile(&source, &p, &deployment, action)
        .map_err(core_io::operation)?;
    let ir = artifact.ir_bytes().map_err(core_io::operation)?;
    let manifest = artifact.manifest_bytes().map_err(core_io::operation)?;
    std::fs::create_dir(out).map_err(|source| CliError::Write {
        path: out.clone(),
        source,
    })?;
    publish(out, &ir, &manifest)?;
    shell.status(
        "Compiled",
        format!(
            "ordinary native call relation {} into {}",
            core_io::review_identity(action),
            core_io::review_identity(&out.to_string_lossy())
        ),
    );
    Ok(ExitCode::SUCCESS)
}
// Shape validation precedes all source, limits and bundle reads.
fn validate_shape(args: &Args) -> super::Result<()> {
    let bad = |message| core_io::operation(message);
    if args.all_actions {
        if args.action.is_some()
            || args.deployment.is_some()
            || args.static_bindings.is_none()
            || args.work_limits.is_none()
            || args.out.is_some() == args.inspect_static.is_some()
        {
            return Err(bad(
                "--all-actions requires --static-bindings, --authorization, --work-limits and exactly one of --out/--inspect-static; --action/--deployment are forbidden",
            ));
        }
        match (args.authorization.as_deref(), args.signer_policy.is_some()) {
            (Some("unsigned" | "unsigned-mapped-roles"), false) | (Some("signed-v2"), true) => {}
            _ => {
                return Err(bad(
                    "authorization must be unsigned or unsigned-mapped-roles without --signer-policy, or signed-v2 with --signer-policy; no default or downgrade",
                ));
            }
        }
    } else if args.action.is_none()
        || args.deployment.is_none()
        || args.out.is_none()
        || args.static_bindings.is_some()
        || args.authorization.is_some()
        || args.signer_policy.is_some()
        || args.work_limits.is_some()
        || args.inspect_static.is_some()
    {
        return Err(bad(
            "legacy compile-call requires --action, --deployment and --out and forbids all static flags",
        ));
    }
    Ok(())
}
fn static_text(path: &Path, cap: usize) -> super::Result<String> {
    let bytes =
        mori_ledger::static_compile::read_static_input(path, cap).map_err(core_io::operation)?;
    String::from_utf8(bytes).map_err(|_| core_io::operation("static input must be UTF-8"))
}
fn run_static(args: &Args, shell: &mut Shell) -> super::Result<ExitCode> {
    use mori_ledger::{
        codec,
        static_compile::{self, StaticAuthorization},
    };
    let limits_text = static_text(
        args.work_limits.as_deref().expect("validated work limits"),
        codec::INPUT_BYTES,
    )?;
    let limits = static_compile::parse_work_limits(&limits_text).map_err(core_io::operation)?;
    let input_cap = usize::try_from(limits.max_input_bytes)
        .map_err(|_| core_io::operation("input limit overflow"))?
        .min(codec::INPUT_BYTES);
    let aggregate_cap = usize::try_from(limits.max_artifact_bytes)
        .map_err(|_| core_io::operation("aggregate limit overflow"))?
        .min(codec::OUTPUT_BYTES);
    let mut raw_total = limits_text.len();
    if raw_total > aggregate_cap {
        return Err(core_io::operation("static input aggregate limit"));
    }
    let mut read = |path: &Path, cap: usize| -> super::Result<String> {
        let remaining = aggregate_cap
            .checked_sub(raw_total)
            .ok_or_else(|| core_io::operation("static input aggregate limit"))?;
        let text = static_text(path, cap.min(remaining))?;
        raw_total = raw_total
            .checked_add(text.len())
            .filter(|n| *n <= aggregate_cap)
            .ok_or_else(|| core_io::operation("static input aggregate limit"))?;
        Ok(text)
    };
    let source = read(&args.file, input_cap)?;
    let bindings_text = read(
        args.static_bindings
            .as_deref()
            .expect("validated static bindings"),
        input_cap,
    )?;
    let bindings =
        static_compile::parse_static_bindings(&bindings_text).map_err(core_io::operation)?;
    let (authorization, raw_auth) = if let Some(policy_path) = &args.signer_policy {
        let policy_text = read(
            policy_path,
            input_cap.min(mori_ledger::signed::SIGNED_JSON_BYTES),
        )?;
        let policy =
            static_compile::parse_signer_policy(&policy_text).map_err(core_io::operation)?;
        (
            StaticAuthorization::SignedV2 {
                signer_policy: policy,
            },
            policy_text
                .len()
                .checked_add("signed-v2".len())
                .ok_or_else(|| core_io::operation("authorization size"))?,
        )
    } else {
        match args.authorization.as_deref() {
            Some("unsigned") => (StaticAuthorization::Unsigned, "unsigned".len()),
            Some("unsigned-mapped-roles") => (
                StaticAuthorization::UnsignedMappedRoles,
                "unsigned-mapped-roles".len(),
            ),
            _ => return Err(core_io::operation("explicit static authorization required")),
        }
    };
    static_compile::check_input_budget(
        source.len(),
        &bindings,
        &authorization,
        &limits,
        bindings_text.len(),
        raw_auth,
        limits_text.len(),
    )
    .map_err(core_io::operation)?;
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    if let Some(directory) = &args.inspect_static {
        let admitted = static_compile::reopen(
            &source,
            &program,
            &bindings,
            &authorization,
            directory,
            &limits,
        )
        .map_err(core_io::operation)?;
        shell.status(
            "Inspected",
            format!(
                "source-static-recompile-admitted {} actions; identity {}{}",
                admitted.action_names().len(),
                admitted.identity(),
                if matches!(authorization, StaticAuthorization::UnsignedMappedRoles) {
                    format!("; qualification {}", authorization.qualification())
                } else {
                    String::new()
                }
            ),
        );
    } else {
        let bundle = static_compile::compile(&source, &program, &bindings, &authorization, &limits)
            .map_err(core_io::operation)?;
        let out = args.out.as_deref().expect("validated export output");
        proof_io::require_new(out)?;
        std::fs::create_dir(out).map_err(|source| CliError::Write {
            path: out.into(),
            source,
        })?;
        publish_static(out, bundle.leaves())?;
        shell.status(
            "Compiled",
            format!(
                "{} {} actions into {}; identity {}",
                authorization.qualification(),
                bundle.action_names().len(),
                core_io::review_identity(&out.to_string_lossy()),
                bundle.identity()
            ),
        );
    }
    Ok(ExitCode::SUCCESS)
}
fn publish_static(
    out: &Path,
    output: &mori_ledger::call_artifacts::ArtifactLeaves,
) -> super::Result<()> {
    publish_static_with(out, output, proof_io::publish_new)
}
fn publish_static_with(
    out: &Path,
    output: &mori_ledger::call_artifacts::ArtifactLeaves,
    mut writer: impl FnMut(&Path, &[u8]) -> super::Result<()>,
) -> super::Result<()> {
    if let Some(source) = output.leaves.get("source.mori") {
        writer(&out.join("source.mori"), source)?;
    }
    for (name, bytes) in &output.leaves {
        if name != "source.mori" {
            writer(&out.join(name), bytes)?;
        }
    }
    // Successful marker publication follows every complete source/action leaf.
    writer(&out.join(output.marker_name), &output.marker)
}
fn publish(out: &Path, ir: &[u8], manifest: &[u8]) -> super::Result<()> {
    proof_io::publish_new(&out.join("relation.zkir.bin"), ir)?;
    // Completion marker follows the complete source-static IR leaf.
    proof_io::publish_new(&out.join("manifest.json"), manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collision_does_not_publish_completed_manifest() {
        let out = std::env::temp_dir().join(format!(
            "mori-call-artifact-collision-{}",
            std::process::id()
        ));
        std::fs::create_dir(&out).unwrap();
        std::fs::write(out.join("relation.zkir.bin"), b"keep").unwrap();
        assert!(publish(&out, b"new", b"marker").is_err());
        assert_eq!(
            std::fs::read(out.join("relation.zkir.bin")).unwrap(),
            b"keep"
        );
        assert!(!out.join("manifest.json").exists());
        std::fs::remove_dir_all(out).unwrap();
    }
    fn static_output() -> mori_ledger::call_artifacts::ArtifactLeaves {
        mori_ledger::call_artifacts::ArtifactLeaves {
            leaves: std::collections::BTreeMap::from([
                ("source.mori".into(), b"source".to_vec()),
                ("action-000000.ir.bin".into(), b"first".to_vec()),
                ("action-000001.ir.bin".into(), b"last".to_vec()),
            ]),
            marker_name: "static-compile.json",
            marker: b"complete".to_vec(),
        }
    }
    static STATIC_TEST_NEXT: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn static_test_dir() -> PathBuf {
        let out = std::env::temp_dir().join(format!(
            "mori-static-publish-{}-{}",
            std::process::id(),
            STATIC_TEST_NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir(&out).unwrap();
        out
    }
    #[test]
    fn static_leaf_and_late_marker_collisions_preserve_foreign_bytes() {
        for name in [
            "source.mori",
            "action-000000.ir.bin",
            "action-000001.ir.bin",
            "static-compile.json",
        ] {
            let dir = static_test_dir();
            std::fs::write(dir.join(name), b"foreign").unwrap();
            assert!(publish_static(&dir, &static_output()).is_err());
            assert_eq!(std::fs::read(dir.join(name)).unwrap(), b"foreign");
            if name != "static-compile.json" {
                assert!(!dir.join("static-compile.json").exists());
            }
            assert!(!std::fs::read_dir(&dir).unwrap().any(|e| {
                e.unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".mori-proof")
            }));
            std::fs::remove_dir_all(dir).unwrap();
        }
    }
    #[test]
    fn late_static_writer_failure_leaves_partial_bytes_and_no_own_marker() {
        let dir = static_test_dir();
        let mut writes = 0;
        let result = publish_static_with(&dir, &static_output(), |path, bytes| {
            writes += 1;
            if writes == 3 {
                return Err(core_io::operation("injected write failure"));
            }
            proof_io::publish_new(path, bytes)
        });
        assert!(result.is_err());
        assert_eq!(writes, 3);
        assert_eq!(std::fs::read(dir.join("source.mori")).unwrap(), b"source");
        assert_eq!(
            std::fs::read(dir.join("action-000000.ir.bin")).unwrap(),
            b"first"
        );
        assert!(!dir.join("static-compile.json").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn publication_refuses_read_only_handle_and_directory_leaf() {
        let dir = static_test_dir();
        std::fs::create_dir(dir.join("action-000001.ir.bin")).unwrap();
        assert!(publish_static(&dir, &static_output()).is_err());
        assert!(!dir.join("static-compile.json").exists());
        std::fs::remove_dir_all(dir).unwrap();
        let dir = static_test_dir();
        let file = dir.join("readonly");
        std::fs::write(&file, b"keep").unwrap();
        let mut handle = std::fs::File::open(&file).unwrap();
        use std::io::Write;
        assert!(
            publish_static_with(&dir, &static_output(), |_, bytes| handle
                .write_all(bytes)
                .map_err(core_io::operation))
            .is_err()
        );
        assert!(!dir.join("static-compile.json").exists());
        assert_eq!(std::fs::read(file).unwrap(), b"keep");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
