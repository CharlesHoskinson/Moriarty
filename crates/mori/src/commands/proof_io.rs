use super::core_io;
use crate::{cli::ActionInput, error::CliError};
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

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
}
impl Input {
    pub fn read(&self) -> super::Result<(String, mori_core::Scenario, mori_backend::Target)> {
        Ok((
            core_io::read_source(&self.action.file)?,
            core_io::read_json(&self.action.scenario)?,
            core_io::read_json(&self.target)?,
        ))
    }
    pub fn options(&self) -> mori_backend::proving::ProofOptions {
        mori_backend::proving::ProofOptions {
            params_dir: self.params_dir.clone(),
            max_k: self.max_k,
        }
    }
}
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
struct Staging(PathBuf);
impl Drop for Staging {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
static NEXT: AtomicU64 = AtomicU64::new(0);
/// Publish only complete bytes. A same-directory hard link atomically refuses
/// any destination created since the early check, including dangling symlinks.
pub fn publish_new(path: &Path, bytes: &[u8]) -> super::Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let stage = parent.join(format!(
        ".mori-proof-{}-{}.tmp",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file: File = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stage)
        .map_err(|source| CliError::Write {
            path: stage.clone(),
            source,
        })?;
    let staging = Staging(stage);
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|source| CliError::Write {
            path: staging.0.clone(),
            source,
        })?;
    drop(file);
    std::fs::hard_link(&staging.0, path).map_err(|source| CliError::Write {
        path: path.into(),
        source,
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
