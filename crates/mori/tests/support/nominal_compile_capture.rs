//! Retain the complete artifact from the actual production selected handler.
//! Ordinary direct consumer, not the backend-linked CLI. No expectation input.
#[path = "../../src/cli_args/compile.rs"]
pub(crate) mod compile_args;
mod cli_args { pub(crate) use crate::compile_args as compile; }
#[path = "../../src/commands/source_io.rs"] mod source_io;
#[path = "../../src/commands/publish_io.rs"] mod publish_io;
#[path = "../../src/commands/compile_sherlock.rs"] mod selected_compile;
#[path = "../../src/display.rs"] mod display;
#[path = "../../src/styles.rs"] mod styles;
#[path = "../../src/status_output.rs"] mod status_output;
use clap::{Args as ClapArgs, FromArgMatches};
use std::{ffi::OsString,io::Write,path::PathBuf,process::ExitCode};
fn run()->Result<ExitCode,mori_diagnostics::Report> {
    // Explicit SOURCE CONFIG TARGET NEWDIR; no fixture lookup or golden reads.
    let input:Vec<OsString>=std::env::args_os().skip(1).collect();
    assert_eq!(input.len(),4);
    let out=PathBuf::from(&input[3]);
    let argv=vec![OsString::from("compile"),input[0].clone(),"--execution-config".into(),input[1].clone(),
        "--target".into(),input[2].clone(),"--out".into(),input[3].clone(),
        "--execution-profile".into(),"sherlock-helper-microstep/1".into(),
        "--source-profile".into(),"moriarty-sherlock-nominal/1".into(),"--entry".into()];
    let matches=compile_args::Args::augment_args(clap::Command::new("compile"))
        .try_get_matches_from(argv).expect("actual production Args");
    let args=compile_args::Args::from_arg_matches(&matches).expect("actual production Args decoding");
    let source=source_io::read_source(&args.file)?;
    let mut err=anstream::AutoStream::new(std::io::stderr(),anstream::ColorChoice::Never);
    let code=selected_compile::run(&args,&source,|verb,message|
        status_output::status(&mut err,status_output::Verbosity::Normal,verb,message))?;
    assert_eq!(code,ExitCode::SUCCESS);
    // Production already syncs complete publication bytes. Retain the file and
    // emit those SAME full bytes, without JSON projection or reserialization.
    let bytes=std::fs::read(out.join("sherlock-artifact.json")).map_err(source_io::operation)?;
    let mut stdout=std::io::stdout().lock();
    stdout.write_all(&bytes).and_then(|()|stdout.write_all(b"\n"))
        .and_then(|()|stdout.flush()).map_err(source_io::operation)?;
    Ok(code)
}
fn main()->ExitCode {match run(){Ok(code)=>code,Err(error)=>{eprintln!("{error}");ExitCode::FAILURE}}}
