//! Ordinary data export for the existing complete-pair producer's public types.
//! No producer invocation, transition, admission, hashing, proof, or SDK operation.
//! The caller binds `context` to the SAME checked complete layout/request tuple.
//! Linking the backend crate has NOT been classified as SDK-free by this source.
use std::io::{self, Write};
use mori_backend::sherlock_machine::{
    literal_complete_bridge::LiteralCompleteExecutionRequest,
    literal_host::ImageValues,
};

fn numbers<W: Write>(out: &mut W, items: impl IntoIterator<Item = u128>) -> io::Result<()> {
    out.write_all(b"[")?;
    let mut separator = "";
    for item in items {
        write!(out, "{separator}{item}")?;
        separator = ",";
    }
    out.write_all(b"]")
}

fn image<W: Write>(out: &mut W, value: &ImageValues) -> io::Result<()> {
    out.write_all(b"{\"root\":")?;
    numbers(out, value.root.iter().copied())?;
    out.write_all(b",\"tables\":[")?;
    let mut table_separator = "";
    for table in &value.tables {
        write!(out, "{table_separator}[")?;
        let mut row_separator = "";
        for row in table {
            out.write_all(row_separator.as_bytes())?;
            numbers(out, row.iter().copied())?;
            row_separator = ",";
        }
        out.write_all(b"]")?;
        table_separator = ",";
    }
    out.write_all(b"]}")
}

/// Export only actual Program, Init, explicit capacities and context. Do not read
/// the producer's intermediate/done images or put them into the K input/oracle.
/// `context` is required caller data; use the actual complete layout's
/// `context_limbs()`, never an A102-only context or a made-up replacement.
/// `observation_budget` is not semantic fuel and has no default.
pub fn write_input<W: Write>(
    request: &LiteralCompleteExecutionRequest,
    context: [u128; 2],
    observation_budget: usize,
    out: &mut W,
) -> io::Result<()> {
    out.write_all(b"{\"format\":\"moriarty-k-literal-complete-images-input/1\",\"context\":")?;
    numbers(out, context)?;
    out.write_all(b",\"capacities\":{\"program\":")?;
    numbers(out, request.capacities.program.iter().map(|n| *n as u128))?;
    out.write_all(b",\"state\":")?;
    numbers(out, request.capacities.state.iter().map(|n| *n as u128))?;
    write!(out, ",\"stack_limit\":{}}},\"program\":", request.capacities.stack_limit)?;
    image(out, &request.values.program)?;
    out.write_all(b",\"init\":")?;
    image(out, &request.values.init)?;
    writeln!(out, ",\"observation_budget\":{observation_budget}}}")
}
