use crate::error::CliError;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub fn read_source(path: &Path) -> super::Result<String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut source = String::new();
    file.take(1_048_577)
        .read_to_string(&mut source)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if source.len() > 1_048_576 {
        return Err(operation("input file exceeds 1 MiB"));
    }
    Ok(source)
}
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}
pub fn read_json<T: DeserializeOwned>(path: &Path) -> super::Result<T> {
    let source = read_source(path)?;
    let value = mori_core::json::parse(&source).map_err(operation)?;
    serde_json::from_value(value).map_err(operation)
}
pub fn program(path: &Path) -> super::Result<(String, mori_core::Program)> {
    let source = read_source(path)?;
    let program = mori_core::parse(&source).map_err(operation)?;
    Ok((source, program))
}
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    // JSON permits literal Unicode controls. Use JSON Unicode escapes in the
    // display while preserving parsed values and canonical signing bytes.
    let json = serde_json::to_string_pretty(value).map_err(operation)?;
    let mut display = String::with_capacity(json.len());
    for character in json.chars() {
        if character.is_ascii() && (!character.is_control() || character == '\n') {
            display.push(character);
        } else {
            for unit in character.encode_utf16(&mut [0; 2]) {
                use std::fmt::Write;
                write!(&mut display, "\\u{unit:04x}").expect("writing to String cannot fail");
            }
        }
    }
    println!("{display}");
    Ok(())
}
