use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature JSON produced by the external signer (the exact signed
    /// wrapper under --profile moriarty-jubjub-owner-intent/1)
    #[arg(long, value_name = "FILE")]
    pub signature: PathBuf,
    /// Expected account authority and domain binding JSON
    #[arg(long)]
    pub authority: PathBuf,
    /// Current logical time used to check the signed validity interval
    #[arg(long)]
    pub at: u128,
    /// Signed native owner-intent profile; requires --target
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,

    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with_all = ["profile", "target"])]
    pub logical_cap: Option<mori_core::UInt>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    match (args.profile, &args.target) {
        (Some(_), Some(target)) => return signed(&args, target),
        (Some(_), None) => {
            return Err(super::core_io::operation(
                "--profile moriarty-jubjub-owner-intent/1 requires --target",
            ));
        }
        (None, Some(_)) => {
            return Err(super::core_io::operation(
                "--target belongs to --profile moriarty-jubjub-owner-intent/1; legacy intents never accept a Target",
            ));
        }
        (None, None) => {}
    }
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority = super::core_io::read_json(&args.authority)?;
    let signed = super::core_io::read_json(&args.signature)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::intent::verify_and_simulate_metered(
            &program,
            &mori_core::intent::hash_bytes(source.as_bytes()),
            &args.input.action,
            &scenario,
            &authority,
            &signed,
            mori_core::UInt(args.at),
            cap,
        ) {
            Ok(metered) => {
                if args.output.review {
                    println!(
                        "Signature and local authority constraints verified at logical time {}.",
                        args.at
                    );
                    println!(
                        "Complete local reference effects follow; ledger acceptance requires its own verification."
                    );
                }
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "signature_valid": true,
                    "qualification": "local-signature-and-reference-execution",
                    "execution": metered.execution, "profile": metered.cost.profile,
                    "logical": {"cap": metered.cost.cap, "used": metered.cost.used}
                }))?;
                Ok(ExitCode::SUCCESS)
            }
            Err(refused) => {
                super::core_io::print(&serde_json::json!({
                    "status": "rejected", "error": refused.error,
                    "profile": refused.cost.profile,
                    "logical": {"cap": refused.cost.cap, "used": refused.cost.used}
                }))?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    match mori_core::intent::verify_and_simulate(
        &program,
        &mori_core::intent::hash_bytes(source.as_bytes()),
        &args.input.action,
        &scenario,
        &authority,
        &signed,
        mori_core::UInt(args.at),
    ) {
        Ok(execution) => {
            if args.output.review {
                println!(
                    "Signature and local authority constraints verified at logical time {}.",
                    args.at
                );
                println!(
                    "Complete local reference effects follow; ledger acceptance requires its own verification."
                );
            }
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "signature_valid":true, "qualification":"local-signature-and-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Strict signed acceptance: trusted reconstruction, exact wrapper equality,
/// host FENS under the reconstructed message and the ordinary compiled
/// relation check. No unsigned or legacy fallback.
fn signed(args: &Args, target: &std::path::Path) -> super::Result<ExitCode> {
    let source = super::core_io::read_source(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let target = super::core_io::signed_target(target)?;
    let authority = super::core_io::authority(&args.authority)?;
    let signed = super::core_io::signed_intent(&args.signature)?;
    match mori_backend::signed_relation::accept_intent(
        &source,
        &args.input.action,
        &scenario,
        &target,
        &authority,
        mori_core::UInt(args.at),
        &signed,
    ) {
        Ok(accepted) => {
            if args.output.review {
                println!(
                    "Exact signed payload, strict FENS attachment and compiled relation accepted at logical time {}.",
                    args.at
                );
                println!(
                    "Host reference check only: not a native proof, key-to-account authority or ledger acceptance."
                );
            }
            super::core_io::print(&serde_json::json!({
                "status": "accepted",
                "signature_valid": true,
                "profile": signed.payload.profile,
                "qualification": "local-signed-owner-intent-host-and-relation-check",
                "payload_hash": accepted.payload_hash,
                "context_hash": accepted.context_hash,
                "message_hex": accepted.message_hex,
                "at": accepted.at,
                "execution": accepted.execution,
            }))?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}
