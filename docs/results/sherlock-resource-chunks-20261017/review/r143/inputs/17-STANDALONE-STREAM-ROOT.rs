//! Fixed trusted normal producer using the actual borrowed visit_steps API.
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

#[path = "../../../crates/mori-core/src/sherlock/reference_stream.rs"]
mod reference_stream;

#[cfg(test)]
#[path = "../../../crates/mori-core/src/sherlock/reference_stream_tests.rs"]
mod reference_stream_tests;

use compare::{TraceIdentity, TraceView};
use normal_oracle_inputs::normal_program;
use reference::Machine;
use reference_display::{KDisplay, environment_to_k};
use reference_types::*;
use std::io::{self, BufWriter, Write};
use std::ops::ControlFlow;
use reference_stream::VisitStop;

const WIDTHS: [usize; 4] = [1, 2, 4, 8];

// Immutable input selection from the committed independent program documents.
// These are budgets/stack limits, not calculated expected work or output values.
const INPUTS: [(&str, u128, usize); 8] = [
    ("N01", 1, 8), ("N02", 7, 8), ("N03", 9, 8), ("N04", 17, 8),
    ("N05", 27, 8), ("N06", 19, 8), ("N07", 38, 8), ("N08", 82, 8),
];
