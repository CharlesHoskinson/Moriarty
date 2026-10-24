#[cfg(feature = "native")]
use super::core_io;
use crate::cli::ActionInput;
#[cfg(feature = "native")]
use crate::error::CliError;
use std::path::PathBuf;
#[cfg(feature = "native")]
use std::{io::ErrorKind, path::Path};

#[derive(Debug, clap::Args)]
pub struct Input {
    #[command(flatten)]
    pub action: ActionInput,
    /// Exact target profile JSON
    #[arg(long)]
    pub target: PathBuf,
    /// Explicit directory containing checksum-pinned offline SRS files
    #[arg(long)]
    pub params_dir: PathBuf,
    /// Maximum permitted native circuit degree (1 through 19)
    #[arg(long, default_value_t = 9)]
    pub max_k: u8,
    /// Exact signed owner-intent wrapper (required with a signed Target)
    #[arg(long, value_name = "FILE")]
    pub signature: Option<PathBuf>,
    /// Expected owner authority JSON (required with a signed Target)
    #[arg(long, value_name = "FILE")]
    pub authority: Option<PathBuf>,
    /// Logical time checked against the signed interval (required with a signed Target)
    #[arg(long)]
    pub at: Option<u128>,
}
/// The complete signed group accompanying a trusted signed Target.
#[cfg(feature = "native")]
pub struct Signed {
    pub authority: mori_backend::authorization::ExpectedAuthority,
    pub at: mori_core::UInt,
    pub intent: mori_backend::authorization::SignedIntent,
}
#[cfg(feature = "native")]
pub type Inputs = (
    String,
    mori_core::Scenario,
    mori_backend::Target,
    Option<Signed>,
);
#[cfg(feature = "native")]
impl Input {
    /// The trusted Target selects the mode. A signed Target requires the whole
    /// signed group; an unsigned Target refuses any part of it.
    pub fn read(&self) -> super::Result<Inputs> {
        let target: mori_backend::Target = core_io::read_json(&self.target)?;
        let signed = match (
            target.signed_policy.is_some(),
            &self.signature,
            &self.authority,
            self.at,
        ) {
            (true, Some(signature), Some(authority), Some(at)) => Some(Signed {
                authority: core_io::authority(authority)?,
                at: mori_core::UInt(at),
                intent: core_io::signed_intent(signature)?,
            }),
            (true, ..) => {
                return Err(core_io::operation(
                    "a signed Target requires --signature, --authority and --at together; it never falls back to unsigned proofs",
                ));
            }
            (false, None, None, None) => None,
            (false, ..) => {
                return Err(core_io::operation(
                    "--signature, --authority and --at require a signed Target; unsigned proofs never accept signed inputs",
                ));
            }
        };
        Ok((
            core_io::read_source(&self.action.file)?,
            core_io::read_json(&self.action.scenario)?,
            target,
            signed,
        ))
    }
    pub fn options(&self) -> mori_backend::proving::ProofOptions {
        mori_backend::proving::ProofOptions {
            params_dir: self.params_dir.clone(),
            max_k: self.max_k,
        }
    }
}
#[cfg(feature = "native")]
pub fn require_new(path: &Path) -> super::Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Err(CliError::Write {
            path: path.into(),
            source: std::io::Error::new(ErrorKind::AlreadyExists, "destination already exists"),
        }
        .into()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        Err(source) => Err(CliError::Write {
            path: path.into(),
            source,
        }
        .into()),
    }
}
/// Publish only complete bytes through the shared no-replacement publisher.
#[cfg(feature = "native")]
pub fn publish_new(path: &Path, bytes: &[u8]) -> super::Result<()> {
    super::publish_io::publish_new(path, bytes).map_err(|error| {
        CliError::Write {
            path: error.path,
            source: error.source,
        }
        .into()
    })
}

#[cfg(all(test, feature = "native"))]
mod tests {
    use super::super::publish_io::NEXT;
    use super::*;
    use std::sync::atomic::Ordering;
    #[test]
    fn concurrent_publication_keeps_one_complete_winner() {
        let dir = std::env::temp_dir().join(format!(
            "mori-proof-race-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("proof.json");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let handles: Vec<_> = [
            b"complete first proof".as_slice(),
            b"complete second proof".as_slice(),
        ]
        .into_iter()
        .map(|bytes| {
            let path = path.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                (bytes, publish_new(&path, bytes).is_ok())
            })
        })
        .collect();
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.iter().filter(|(_, ok)| *ok).count(), 1);
        assert_eq!(
            std::fs::read(&path).unwrap(),
            results.iter().find(|(_, ok)| *ok).unwrap().0
        );
        assert_eq!(
            std::fs::read_dir(&dir).unwrap().count(),
            1,
            "no staging file survives normal refusal"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }
}
