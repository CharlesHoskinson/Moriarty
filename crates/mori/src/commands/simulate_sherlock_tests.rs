//! SOURCE_ONLY / UNRUN. Command literals precede the route implementation.
//! Pure parsing/selection checks: no files, subject execution or native work.
use clap::Parser;
use crate::cli::{Cli, Command};

#[test]
fn explicit_draft_selection_and_legacy_shapes_match_frozen_literals() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sherlock-simulate/CLI-EXPECTATIONS.json"
    ))).unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut argv = vec!["mori", "simulate", "missing-source.mori"];
        argv.extend(case["flags"].as_array().unwrap().iter().map(|s| s.as_str().unwrap()));
        let result = Cli::try_parse_from(argv);
        assert_eq!(result.is_ok(), case["parse"].as_bool().unwrap(), "{name}");
        let Ok(cli) = result else { continue; };
        let Command::Simulate(args) = cli.command else { panic!("{name}: wrong command"); };
        if case["route"] == "legacy" {
            assert!(args.execution_profile.is_none(), "{name}");
            assert!(args.source_profile.is_none() && !args.entry, "{name}");
            continue;
        }
        match super::sherlock::selection(&args) {
            Ok((config, target)) => {
                assert_eq!(case["route"], "draft", "{name}");
                assert_eq!(config, std::path::Path::new("config.json"), "{name}");
                assert_eq!(target, std::path::Path::new("target.json"), "{name}");
                assert!(args.action.is_none() && args.scenario.is_none(), "{name}");
            }
            Err(failure) => assert_eq!(failure.stage, case["route"].as_str().unwrap(), "{name}"),
        }
    }
}
