//! Usage: cargo run -p mori-backend --example c6_witness_stream -- PROGRAM CONFIG
//! Public JSON lines go to stdout; failures (including a partial run) exit nonzero.
use std::{io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: c6_witness_stream PROGRAM.mori HOST-CONFIG.json".into());
    }
    let source = std::fs::read_to_string(PathBuf::from(&args[0]))?;
    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());
    mori_backend::sherlock_c6_witness_stream::write_witness_stream(
        &source,
        &PathBuf::from(&args[1]),
        &mut writer,
    )?;
    writer.flush()?;
    Ok(())
}
