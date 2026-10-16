//! Runtime registration for the fixed private codec observation writer.
mod reference_types;
mod reference;
pub mod reference_display;
pub mod compare;
mod normal_oracle_inputs;
pub use reference::Machine;
pub use reference_types::*;
mod layout_a;
pub(crate) fn capture() -> Result<(), layout_a::layout_observations::CaptureFailure> {
    use std::io::{self, BufWriter, Write};
    let mut out = BufWriter::new(io::stdout().lock());
    layout_a::layout_observations::write_n01_n08(&mut out)?;
    out.flush().map_err(layout_a::layout_observations::CaptureFailure::Io)
}
