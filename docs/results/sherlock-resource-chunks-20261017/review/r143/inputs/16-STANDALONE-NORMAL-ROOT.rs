//! Standalone fixed-input observation producer for the existing host Machine.
//! No Cargo, SDK, model engine, source parser or public input endpoint is used.
//! Diagnostic K strings below are not a canonical state codec or public ABI.

#[path = "../../../crates/mori-core/src/sherlock/reference_types.rs"]
mod reference_types;
#[path = "../../../crates/mori-core/src/sherlock/reference.rs"]
mod reference;
#[path = "../../../crates/mori-core/src/sherlock/reference_display.rs"]
mod reference_display;
#[path = "../../../crates/mori-core/src/sherlock/compare.rs"]
mod compare;
#[path = "../../../crates/mori-core/src/sherlock/normal_oracle_inputs.rs"]
mod normal_oracle_inputs;

#[cfg(test)]
#[path = "../../../crates/mori-core/src/sherlock/reference_bounded_tests.rs"]
mod reference_bounded_tests;

#[cfg(test)]
#[path = "../../../crates/mori-core/src/sherlock/reference_bounded_extra_tests.rs"]
mod reference_bounded_extra_tests;

use compare::{TraceIdentity, TraceView};
use normal_oracle_inputs::normal_program;
use reference::Machine;
use reference_display::{KDisplay, environment_to_k};
use reference_types::*;
use std::io::{self, BufWriter, Write};

// Immutable input selection from the committed independent program documents.
// These are budgets/stack limits, not calculated expected work or output values.
const INPUTS: [(&str, u128, usize); 8] = [
    ("N01", 1, 8), ("N02", 7, 8), ("N03", 9, 8), ("N04", 17, 8),
    ("N05", 27, 8), ("N06", 19, 8), ("N07", 38, 8), ("N08", 82, 8),
];

fn json_string(out: &mut impl Write, value: &str) -> io::Result<()> {
    write!(out, "\"")?;
