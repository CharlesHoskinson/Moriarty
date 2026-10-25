//! SOURCE_ONLY / UNRUN. Command literals precede the route implementation.
//! Pure parsing/selection checks: no files, subject execution or native work.
use crate::cli::{Cli, Command};
use clap::Parser;

#[test]
fn explicit_draft_selection_and_legacy_shapes_match_frozen_literals() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/sherlock-simulate/CLI-EXPECTATIONS.json"
    )))
    .unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut argv = vec!["mori", "simulate", "missing-source.mori"];
        argv.extend(
            case["flags"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap()),
        );
        let result = Cli::try_parse_from(argv);
        assert_eq!(result.is_ok(), case["parse"].as_bool().unwrap(), "{name}");
        let Ok(cli) = result else {
            continue;
        };
        let Command::Simulate(args) = cli.command else {
            panic!("{name}: wrong command");
        };
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

#[test]
fn technical_chunk_flags_parse_and_conflict() {
    let base = [
        "mori",
        "simulate",
        "p.mori",
        "--execution-profile",
        "sherlock-helper-microstep/1",
        "--entry",
        "--execution-config",
        "config.json",
        "--target",
        "target.json",
    ];
    let parse = |extra: &[&str]| {
        let mut argv = base.to_vec();
        argv.extend_from_slice(extra);
        Cli::try_parse_from(argv)
    };
    let Command::Simulate(args) = parse(&["--chunk", "3", "--chunk-events"]).unwrap().command
    else {
        panic!("wrong command");
    };
    assert_eq!(args.chunk, Some(3));
    assert!(args.chunk_events);
    let Command::Simulate(args) = parse(&["--chunk-at", "1,4,9"]).unwrap().command else {
        panic!("wrong command");
    };
    assert_eq!(args.chunk_at, Some(vec![1, 4, 9]));
    assert!(parse(&["--chunk", "2", "--chunk-at", "1"]).is_err());
    let Command::Simulate(args) = parse(&["--reference-slices", "4"]).unwrap().command else {
        panic!("wrong command");
    };
    assert_eq!(args.reference_slices, Some(4));
    let Command::Simulate(args) = parse(&["--chunk", "1", "--chunk-represent"])
        .unwrap()
        .command
    else {
        panic!("wrong command");
    };
    assert!(args.chunk_represent);
    let Command::Simulate(args) = parse(&["--chunk", "1", "--chunk-witness"]).unwrap().command
    else {
        panic!("wrong command");
    };
    assert!(args.chunk_witness);
    assert!(parse(&["--chunk", "-1"]).is_err());
    // Chunking needs the explicit Sherlock route.
    assert!(Cli::try_parse_from(["mori", "simulate", "p.mori", "--chunk", "2"]).is_err());
}
