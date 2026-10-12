//! Source-derived local native call preparation and explicit proving APIs.
//! Preparation stays unsigned/unproven; native APIs require separately admitted
//! resources. No funding, signatures, ledger apply or network I/O.
#[cfg(test)]
extern crate self as mori_ledger;

pub mod codec;
pub mod custody;
pub mod layout;
pub mod ops;
pub mod prepare;
pub mod profile;
pub mod proving;
pub mod relation;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("mori::ledger::{layer}: {code}")]
pub struct Error {
    pub layer: &'static str,
    pub code: &'static str,
}
impl Error {
    pub fn new(layer: &'static str, code: &'static str) -> Self {
        Self { layer, code }
    }
    pub fn input(code: &'static str) -> Self {
        Self::new("input", code)
    }
}
pub type Result<T> = std::result::Result<T, Error>;
