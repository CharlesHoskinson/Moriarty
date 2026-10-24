//! Source-first direct consumer of the production simulate argument definitions.
//! SOURCE_ONLY / UNRUN. No handler, file input, owner runtime or SDK import.
#[path = "../src/cli_args.rs"]
mod cli_args;

use clap::{Args, FromArgMatches};

#[test]
fn shared_production_simulate_arguments_and_selection_match_original_24_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/sherlock-simulate/CLI-EXPECTATIONS.json"
    ))
    .expect("original frozen command literals");
    assert_eq!(cases.as_array().unwrap().len(), 24);
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        // This is clap's actual Args augmentation for the production type,
        // not a second Cli or a copy of its fields/attributes. The enclosing
        // public Cli consumes this same Args implementation for Simulate.
        let command = cli_args::simulate::Args::augment_args(clap::Command::new("simulate"));
        let mut argv = vec!["simulate", "missing-source.mori"];
        argv.extend(
            case["flags"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap()),
        );
        let result = command.try_get_matches_from(argv);
        assert_eq!(result.is_ok(), case["parse"].as_bool().unwrap(), "{name}");
        let Ok(matches) = result else {
            continue;
        };
        let args = cli_args::simulate::Args::from_arg_matches(&matches)
            .unwrap_or_else(|error| panic!("{name}: actual Args conversion failed: {error}"));
        if case["route"] == "legacy" {
            assert!(args.execution_profile.is_none(), "{name}");
            assert!(args.source_profile.is_none() && !args.entry, "{name}");
            continue;
        }
        match cli_args::simulate::selection(&args) {
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
