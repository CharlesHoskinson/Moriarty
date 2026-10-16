// Runtime-only lexical composition for an ordinary stdlib compile, not installation.
#[path = "types.proposed.rs"]
mod raw_types;

mod row_projections {
    use super::raw_types::{ConstRow, DataRow, LogicalKind};
    include!("functions.proposed.rs");
}
