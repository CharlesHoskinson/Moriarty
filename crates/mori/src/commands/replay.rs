//! Actual CLI replay consumer; all roots and signature attachment are separate inputs.
use crate::{cli::ActionInput, shell::Shell};
#[cfg(feature = "native")]
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    replay::{ReplayAuthorization, ReplayError, ReplayRecord},
};
#[cfg(feature = "native")]
use mori_core::UInt;
use std::{path::PathBuf, process::ExitCode};

pub use crate::cli_args::replay::AuthorizationArgs;

#[cfg(feature = "native")]
pub enum LoadedAuthorization {
    Unsigned,
    Signed {
        authority: Box<ExpectedAuthority>,
        at: UInt,
        signature: Box<SignedIntent>,
    },
}
impl AuthorizationArgs {
    #[cfg(feature = "native")]
    pub fn load(&self, target: &Target) -> super::Result<LoadedAuthorization> {
        if target.signed_policy.is_none() {
            if self.present() {
                return Err(super::core_io::operation(
                    "Signed replay arguments require a trusted Target with signed_policy",
                ));
            }
            return Ok(LoadedAuthorization::Unsigned);
        }
        let (Some(_), Some(authority), Some(at), Some(signature)) =
            (&self.profile, &self.authority, self.at, &self.signature)
        else {
            return Err(super::core_io::operation(
                "Signed Target requires --profile moriarty-jubjub-owner-intent/1, --authority, --at and --signature together",
            ));
        };
        Ok(LoadedAuthorization::Signed {
            authority: Box::new(super::core_io::authority(authority)?),
            at: UInt(at),
            signature: Box::new(super::core_io::signed_intent(signature)?),
        })
    }
}
#[cfg(feature = "native")]
impl LoadedAuthorization {
    pub fn as_input(&self) -> ReplayAuthorization<'_> {
        match self {
            Self::Unsigned => ReplayAuthorization::Unsigned,
            Self::Signed {
                authority,
                at,
                signature,
            } => ReplayAuthorization::Signed {
                authority,
                at: *at,
                signature,
            },
        }
    }
}
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,
    /// Separately trusted Target JSON
    #[arg(long, value_name = "FILE")]
    pub target: PathBuf,
    /// Public sidecar to compare; never supplies trusted roots
    #[arg(long, value_name = "FILE")]
    pub record: PathBuf,
    /// Export a bounded local first-divergence diagnostic to a new file
    #[arg(long, value_name = "FILE")]
    pub challenge_out: Option<PathBuf>,
    #[command(flatten)]
    pub authorization: AuthorizationArgs,
}
#[cfg(feature = "native")]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    // File/schema/group errors have the same input classification as failed
    // reconstruction. Neither case creates or publishes a challenge.
    let inputs = (|| {
        let target = super::core_io::read_json(&args.target)?;
        let authorization = args.authorization.load(&target)?;
        let candidate = ReplayRecord::from_json(&super::core_io::read_source(&args.record)?)
            .map_err(super::core_io::operation)?;
        let source = super::core_io::read_source(&args.input.file)?;
        let scenario = super::core_io::read_json(&args.input.scenario)?;
        Ok::<_, mori_diagnostics::Report>((target, authorization, candidate, source, scenario))
    })();
    let (target, authorization, candidate, source, scenario) = match inputs {
        Ok(inputs) => inputs,
        Err(error) => {
            super::core_io::print(
                &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"input","detail":error.to_string()}}),
            )?;
            return Ok(ExitCode::FAILURE);
        }
    };
    let result = mori_backend::replay::challenge(
        &source,
        &args.input.action,
        &scenario,
        &target,
        authorization.as_input(),
        &candidate,
    );
    let (report, status) = match result {
        // Successful full equality includes independently reconstructed record_hash.
        Ok(None) => (
            serde_json::json!({"status":"accepted","qualification":"local-reference-replay","record_hash":candidate.record_hash}),
            ExitCode::SUCCESS,
        ),
        Ok(Some(challenge)) => {
            let d = &challenge.divergence;
            if let Some(path) = &args.challenge_out {
                let published = challenge
                    .to_json()
                    .map_err(super::core_io::operation)
                    .and_then(|text| {
                        super::proof_io::publish_new(path, format!("{text}\n").as_bytes())
                    });
                if let Err(error) = published {
                    super::core_io::print(
                        &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"output","detail":error.to_string()},"divergence":d}),
                    )?;
                    return Ok(ExitCode::FAILURE);
                }
            }
            (
                serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
                ExitCode::FAILURE,
            )
        }
        Err(ReplayError::Input(error)) => (
            serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"input","detail":error}}),
            ExitCode::FAILURE,
        ),
        // challenge converts semantic divergence to a diagnostic. Preserve an
        // explicit refusal if that API ever gains another divergence branch.
        Err(ReplayError::Divergence(d)) => (
            serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
            ExitCode::FAILURE,
        ),
    };
    super::core_io::print(&report)?;
    Ok(status)
}

#[cfg(not(feature = "native"))]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let _ = (
        &args.input,
        &args.target,
        &args.record,
        &args.challenge_out,
        &args.authorization,
    );
    super::native_unavailable()
}
