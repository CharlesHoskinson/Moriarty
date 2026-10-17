//! `mori-k2`: thin command line over the driver library. UNADOPTED, never built or run.
//!
//! Effects per subcommand (no network code exists in this crate):
//! - `compile`: reads source and selector; checks the descriptor's fixed profile
//!   (`check_profile`); prints the relation descriptor; writes nothing.
//! - `inspect`: reads source and request and takes a declared VK digest; prints the ordinary
//!   statement receipt (decoded statement values and the fixed setup-mode matrices' decision);
//!   writes nothing and draws no randomness.
//! - `setup`: reads source and selector; draws OsRng; writes the public setup record and the
//!   proving key (mode 0600). Both outputs are created with `create_new`.
//! - `prove`: reads source, request, setup record and proving key; draws OsRng; writes the
//!   proof export with `create_new`.
//! - `verify`: reads the setup record, the proof export and optionally the source; prints a
//!   report; writes nothing and draws no randomness.
//!
//! Exit status: 0 success, 1 refusal, 2 usage error. No retry, fallback or chaining.
use mori_k2_driver::{DriverError, compile_selected, export, hexutil, input, inspect, prove, relation, setup, verify};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "\
mori-k2 compile --source FILE --selector FILE
mori-k2 inspect --source FILE --request FILE --vk-digest HEX
mori-k2 setup   --source FILE --selector FILE --out-record FILE --out-proving-key FILE
mori-k2 prove   --source FILE --request FILE --setup-record FILE --proving-key FILE --out-proof FILE
mori-k2 verify  --setup-record FILE --proof FILE [--source FILE]";

enum CliError {
    Usage(String),
    Refused(DriverError),
}

impl From<DriverError> for CliError {
    fn from(e: DriverError) -> Self {
        Self::Refused(e)
    }
}

fn main() -> ExitCode {
    let mut args = Vec::new();
    for a in std::env::args_os().skip(1) {
        match a.into_string() {
            Ok(s) => args.push(s),
            Err(_) => {
                eprintln!("usage: arguments must be UTF-8\n{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError::Usage(m)) => {
            eprintln!("usage: {m}\n{USAGE}");
            ExitCode::from(2)
        }
        Err(CliError::Refused(e)) => {
            eprintln!("refused: {e}");
            ExitCode::from(1)
        }
    }
}

/// Parses `--flag value` pairs. Every flag must be allowed, appear at most once and have a
/// value; every required flag must be present. No positional arguments.
fn flags(args: &[String], required: &[&str], optional: &[&str]) -> Result<BTreeMap<String, PathBuf>, CliError> {
    let mut out = BTreeMap::new();
    let mut i = 0;
    while i < args.len() {
        let name = args[i].strip_prefix("--").ok_or_else(|| CliError::Usage(format!("unexpected argument {:?}", args[i])))?;
        if !required.contains(&name) && !optional.contains(&name) {
            return Err(CliError::Usage(format!("unknown flag --{name}")));
        }
        let value = args.get(i + 1).ok_or_else(|| CliError::Usage(format!("--{name} needs a value")))?;
        if value.is_empty() || value.starts_with("--") {
            return Err(CliError::Usage(format!("--{name} needs a path value")));
        }
        if out.insert(name.to_string(), PathBuf::from(value)).is_some() {
            return Err(CliError::Usage(format!("--{name} given twice")));
        }
        i += 2;
    }
    for r in required {
        if !out.contains_key(*r) {
            return Err(CliError::Usage(format!("missing --{r}")));
        }
    }
    Ok(out)
}

fn path<'a>(f: &'a BTreeMap<String, PathBuf>, name: &str) -> Result<&'a Path, CliError> {
    f.get(name).map(PathBuf::as_path).ok_or_else(|| CliError::Usage(format!("missing --{name}")))
}

/// A flag whose value is text rather than a path (`--vk-digest`).
fn text<'a>(f: &'a BTreeMap<String, PathBuf>, name: &str) -> Result<&'a str, CliError> {
    path(f, name)?.to_str().ok_or_else(|| CliError::Usage(format!("--{name} must be UTF-8")))
}

fn json_bytes(p: &Path, what: &str) -> Result<Vec<u8>, DriverError> {
    input::read_bounded(p, input::MAX_JSON_BYTES as u64, what)
}

/// Creates `p` exclusively and writes `bytes`. An existing path is refused, never replaced.
fn write_new(p: &Path, bytes: &[u8], private: bool) -> Result<(), DriverError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(if private { 0o600 } else { 0o644 });
    }
    #[cfg(not(unix))]
    let _ = private;
    let mut file = options.open(p).map_err(|e| DriverError::Io(format!("cannot create {}: {e}", p.display())))?;
    file.write_all(bytes).map_err(|e| DriverError::Io(format!("cannot write {}: {e}", p.display())))?;
    file.sync_all().map_err(|e| DriverError::Io(format!("cannot sync {}: {e}", p.display())))?;
    Ok(())
}

fn refuse_existing(paths: &[&Path]) -> Result<(), DriverError> {
    for p in paths {
        if p.exists() {
            return Err(DriverError::Io(format!("{} already exists; outputs are never overwritten", p.display())));
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), DriverError> {
    let bytes = export::document_bytes(value)?;
    std::io::stdout().write_all(&bytes).map_err(|e| DriverError::Io(format!("stdout: {e}")))
}

fn run(args: &[String]) -> Result<(), CliError> {
    let (command, rest) = args.split_first().ok_or_else(|| CliError::Usage("missing subcommand".into()))?;
    match command.as_str() {
        "compile" => {
            let f = flags(rest, &["source", "selector"], &[])?;
            let source = input::read_source(path(&f, "source")?)?;
            let selector = input::parse_selector(&json_bytes(path(&f, "selector")?, "selector")?)?;
            let selected = compile_selected(&source, &selector)?;
            selected.descriptor().check_profile()?;
            print_json(selected.descriptor())?;
            Ok(())
        }
        "inspect" => {
            let f = flags(rest, &["source", "request", "vk-digest"], &[])?;
            let vk_digest = hexutil::decode_exact::<32>(text(&f, "vk-digest")?, "--vk-digest")?;
            let source = input::read_source(path(&f, "source")?)?;
            let request = input::parse_request(&json_bytes(path(&f, "request")?, "request")?)?;
            let selected = compile_selected(&source, &request.selector)?;
            selected.descriptor().check_profile()?;
            let receipt = inspect::inspect(&selected, &request, vk_digest)?;
            print_json(&receipt)?;
            Ok(())
        }
        "setup" => {
            let f = flags(rest, &["source", "selector", "out-record", "out-proving-key"], &[])?;
            let (out_record, out_pk) = (path(&f, "out-record")?, path(&f, "out-proving-key")?);
            refuse_existing(&[out_record, out_pk])?;
            let source = input::read_source(path(&f, "source")?)?;
            let selector = input::parse_selector(&json_bytes(path(&f, "selector")?, "selector")?)?;
            let selected = compile_selected(&source, &selector)?;
            let mut rng = rand_core::OsRng;
            let out = setup::setup(&selected, &mut rng)?;
            write_new(out_pk, &out.proving_key_bytes, true)?;
            write_new(out_record, &out.record_bytes, false)?;
            print_json(&serde_json::json!({
                "setupRecordSha256": mori_r1cs::identity::digest_bytes(&out.record_bytes).hex(),
                "predicateDigest": out.record.relation.predicate_digest,
                "observedR1csSha256": out.observed_r1cs_sha256,
                "vkDigestBlake2b256": out.record.vk_digest_blake2b256,
                "provingKeySha256": out.record.proving_key_sha256,
                "trustProfile": out.record.trust_profile,
            }))?;
            Ok(())
        }
        "prove" => {
            let f = flags(rest, &["source", "request", "setup-record", "proving-key", "out-proof"], &[])?;
            let out_proof = path(&f, "out-proof")?;
            refuse_existing(&[out_proof])?;
            let source = input::read_source(path(&f, "source")?)?;
            let request = input::parse_request(&json_bytes(path(&f, "request")?, "request")?)?;
            let selected = compile_selected(&source, &request.selector)?;
            let setup = export::parse_setup_record(&json_bytes(path(&f, "setup-record")?, "setup record")?)?;
            relation::require_same(&setup.record.relation, selected.descriptor())?;
            let pk_bytes = input::read_bounded(path(&f, "proving-key")?, input::MAX_PROVING_KEY_BYTES, "proving key")?;
            let pk = prove::load_proving_key(&pk_bytes, &setup)?;
            drop(pk_bytes);
            let mut rng = rand_core::OsRng;
            let out = prove::prove(&selected, &setup, &pk, &request, &mut rng)?;
            write_new(out_proof, &out.export_bytes, false)?;
            print_json(&serde_json::json!({
                "proofExportSha256": mori_r1cs::identity::digest_bytes(&out.export_bytes).hex(),
                "statementDigestBlake2b256": out.export.statement_digest_blake2b256,
                "x1": out.export.public_inputs.x1,
                "x2": out.export.public_inputs.x2,
            }))?;
            Ok(())
        }
        "verify" => {
            let f = flags(rest, &["setup-record", "proof"], &["source"])?;
            let record_bytes = json_bytes(path(&f, "setup-record")?, "setup record")?;
            let export_bytes = json_bytes(path(&f, "proof")?, "proof export")?;
            let recompiled = match f.get("source") {
                Some(p) => {
                    let setup = export::parse_setup_record(&record_bytes)?;
                    let source = input::read_source(p)?;
                    Some(compile_selected(&source, &relation::selector_for(&setup.record.relation))?)
                }
                None => None,
            };
            let report = verify::verify_export(&record_bytes, &export_bytes, recompiled.as_ref())?;
            print_json(&report)?;
            Ok(())
        }
        other => Err(CliError::Usage(format!("unknown subcommand {other:?}"))),
    }
}
