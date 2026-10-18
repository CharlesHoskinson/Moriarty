//! Canonical program and plan digests. Both hash `intent::canonical`, the
//! same encoding the backend uses for its `/3` program hash.
use super::StagePlan;
use crate::{Error, Program};
use sha2::{Digest, Sha256};

fn digest<T: serde::Serialize>(value: &T) -> Result<[u8; 32], Error> {
    Ok(Sha256::digest(crate::intent::canonical(value)?).into())
}

pub(crate) fn hex(hash: &[u8; 32]) -> String {
    crate::intent::hex(hash)
}

/// Exact comparison against the canonical 64-character lowercase encoding.
pub(crate) fn matches(text: &str, hash: &[u8; 32]) -> bool {
    text == hex(hash)
}

pub(crate) fn program_hash(program: &Program) -> Result<[u8; 32], Error> {
    digest(program)
}

/// Hashes the costed plan, which already names its program hash and format.
pub(crate) fn plan_hash(plan: &StagePlan) -> Result<[u8; 32], Error> {
    let filled = plan.program_hash.len() == 64
        && plan
            .program_hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if !filled {
        return Err(Error::new(
            "program",
            "Stage plan invariant violated: program hash is not filled.",
        ));
    }
    digest(plan)
}
