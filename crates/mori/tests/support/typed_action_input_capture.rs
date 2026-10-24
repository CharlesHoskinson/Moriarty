//! Ordinary real selected-handler positive capture. Source-only/UNRUN.
//! No expected file, semantic assertions, alternate evaluator or owner policy.
#[path = "../../src/cli_args/compile.rs"] pub(crate) mod compile_args;
mod cli_args { pub(crate) use crate::compile_args as compile; }
#[path = "../../src/commands/source_io.rs"] mod source_io;
#[path = "../../src/commands/publish_io.rs"] mod publish_io;
#[path = "../../src/commands/compile_sherlock.rs"] mod selected_compile;
#[path = "../../src/display.rs"] mod display;
use clap::{Args as ClapArgs, FromArgMatches};
use std::{ffi::OsString, path::PathBuf, process::ExitCode};
fn run() -> Result<ExitCode,mori_diagnostics::Report> {
    // SOURCE ACTION INPUTS CONFIG TARGET OUT; Root selects only frozen cases.
    let a:Vec<OsString>=std::env::args_os().skip(1).collect();assert_eq!(a.len(),6);
    let out=PathBuf::from(&a[5]);assert!(!out.exists());
    let argv=vec![OsString::from("compile"),a[0].clone(),"--action".into(),a[1].clone(),
        "--execution-inputs".into(),a[2].clone(),"--execution-config".into(),a[3].clone(),
        "--target".into(),a[4].clone(),"--out".into(),a[5].clone(),"--execution-profile".into(),
        "sherlock-helper-microstep/1".into(),"--source-profile".into(),"moriarty/1".into(),"--observe".into()];
    let matches=compile_args::Args::augment_args(clap::Command::new("compile")).try_get_matches_from(argv).expect("production Args");
    let args=compile_args::Args::from_arg_matches(&matches).expect("production Args decode");
    let source=source_io::read_source(&args.file)?;
    let code=selected_compile::run(&args,&source,|verb,msg|eprintln!("{verb}: {msg}"))?;
    // Production already published the complete report. Flush it before returning;
    // Root retains/raw-freezes both reports before independent comparison.
    std::fs::File::open(out.join("sherlock-artifact.json")).map_err(source_io::operation)?.sync_all().map_err(source_io::operation)?;
    std::fs::File::open(&out).map_err(source_io::operation)?.sync_all().map_err(source_io::operation)?;
    println!("published complete HostOnly report: {}",out.join("sherlock-artifact.json").display());
    Ok(code)
}
fn main()->ExitCode { match run(){Ok(code)=>code,Err(e)=>{eprintln!("{e}");ExitCode::FAILURE}} }
