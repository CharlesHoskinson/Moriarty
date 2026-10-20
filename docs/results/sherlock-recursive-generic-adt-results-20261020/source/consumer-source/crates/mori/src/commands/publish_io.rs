//! Existing complete-byte, no-replacement publisher shared by CLI consumers.
//! This is the existing staging/sync/hard-link/cleanup algorithm, not a new policy.
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

// Same display and io::Error source as CliError::Write. Existing proof_io callers
// convert this carrier back into that original diagnostic before returning.
#[derive(Debug, thiserror::Error)]
#[error("could not write `{}`", crate::display::text(&path.display().to_string()))]
pub struct WriteError {
    pub path: PathBuf,
    #[source]
    pub source: std::io::Error,
}

struct Staging(PathBuf);
impl Drop for Staging {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
pub(super) static NEXT: AtomicU64 = AtomicU64::new(0);
/// Publish only complete bytes. A same-directory hard link atomically refuses
/// any destination created since the early check, including dangling symlinks.
pub fn publish_new(path: &Path, bytes: &[u8]) -> Result<(), WriteError> {
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
        .map_err(|source| WriteError {
            path: stage.clone(),
            source,
        })?;
    let staging = Staging(stage);
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|source| WriteError {
            path: staging.0.clone(),
            source,
        })?;
    drop(file);
    std::fs::hard_link(&staging.0, path).map_err(|source| WriteError {
        path: path.into(),
        source,
    })?;
    Ok(())
}
