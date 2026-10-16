//! Root-owned lexical composition proposal over the existing production rlib.
//! This root exports no public replacement Core implementation.
pub(crate) use mori_core::*;
#[path = "financial_projection.rs"]
mod sherlock_financial_projection;
#[cfg(test)]
#[path = "financial_core2_consumer.rs"]
mod financial_core2_consumer;
