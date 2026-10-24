//! Designated-owner security-negative tests for nominal external inputs.
//! Status: SOURCE_ONLY_UNRUN. Author-derived from the current public contract;
//! not an independent semantic oracle and not a claimed test pass.
//! Tests 13 and 15 are SHARED_PRODUCER_EQUIVALENCE_ONLY (current handler
//! versus the current public command/direct API), not historical byte
//! equality and not an independent runtime oracle. Genuinely unreachable
//! synthesized-entry body-error, partial-prefix HostViolation, and
//! serializer-failure cases remain UNEXERCISED and are not fabricated here.

#[path = "../src/cli_args/compile.rs"]
pub(crate) mod compile_args;
mod cli_args {
    pub(crate) use crate::compile_args as compile;
}
#[path = "../src/commands/source_io.rs"]
mod source_io;
#[path = "../src/commands/publish_io.rs"]
mod publish_io;
#[path = "../src/commands/compile_sherlock.rs"]
mod selected_compile;
#[path = "../src/display.rs"]
mod display;

use clap::{Args as ClapArgs, FromArgMatches};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::ExtProgram;
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtExpr, ExtFunction, ExtParamSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::{
    BoundsC, ExtCapacitiesC, LayoutCapacitiesC, LayoutSchemaC, RunIntC,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::ExtDomain;
use mori_core::sherlock::nominal_external_inputs::{
    decode_and_bind_nominal_inputs, NominalInputError, NominalInputLimits,
};
use mori_core::sherlock_compile_artifact::ext_compile::ExtCompileConfig;
use mori_core::sherlock_compile_artifact::nominal_body_compile::{
    compile_nominal_source_host_bounded_with_nominal_inputs, NominalBindingRequest,
    NominalBodyCompileError,
};
use mori_core::sherlock_compile_artifact as host;
use mori_core::sherlock_nominal_source::{DraftParseBudget, SourceOwner};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const CONFIG: &str = include_str!("fixtures/nominal-compile-handler/config.json");
const TARGET: &str = include_str!("fixtures/nominal-compile-handler/target.json");
const BODY_PROFILE: &str = "moriarty-sherlock-nominal-body-typed/1";

// Hand-counted for SOURCE below: function-body Var (1) + original-entry Var (1)
// = 2 existing AST occurrences, 0 patterns. Synthesized entry for `pick`:
// 1 Call + 3 Literals = 4 expr nodes; values Link+head+End+n+flag = 5 nodes;
// constructor types Chain<UInt> twice = 4 type nodes; total synthesized 13;
// combined 15; synthesized structural depth = max value depth 2 + 2 = 4.
const SOURCE: &str = r#"language "moriarty-sherlock-nominal/1";
program BoundCheck {
  type Chain<T> { End; Link(head: T, tail: Chain<T>); }
  fn pick(chain: Chain<UInt>, n: UInt, flag: Bool): UInt { n }
  entry missing_sentinel;
}
"#;

const AMBIGUOUS_SOURCE: &str = r#"language "moriarty-sherlock-nominal/1";
program AmbiguousCheck {
  type Chain<T> { End; Link(head: T, tail: Chain<T>); }
  fn pick(chain: Chain<UInt>, n: UInt, flag: Bool): UInt { n }
  fn pick(chain: Chain<UInt>, n: UInt, flag: Bool): UInt { n }
  entry missing_sentinel;
}
"#;

const ILLTYPED_SOURCE: &str = r#"language "moriarty-sherlock-nominal/1";
program IllCheck {
  type Chain<T> { End; Link(head: T, tail: Chain<T>); }
  fn pick(chain: Chain<UInt>, n: UInt, flag: Bool): UInt { n }
  fn unused(n: UInt): Bool { n }
  entry missing_sentinel;
}
"#;

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mori-nominal-handler-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn parsed(argv: Vec<String>) -> Result<compile_args::Args, clap::Error> {
    let m = compile_args::Args::augment_args(clap::Command::new("compile"))
        .try_get_matches_from(argv)?;
    compile_args::Args::from_arg_matches(&m)
}

fn handler_failure_text(failure: &host::CompileFailure) -> String {
    format!(
        "mori::command: {}",
        display::text(&mori_core::json::export(failure).unwrap())
    )
}

fn handler_text_of(failure: &host::CompileFailure) -> String {
    handler_failure_text(&host::CompileFailure {
        stage: failure.stage,
        actual_error: failure.actual_error.clone(),
        evaluation_performed: failure.evaluation_performed,
        events_forwarded: failure.events_forwarded.clone(),
        completed: failure.completed,
    })
}

fn handler_refusal(actual: &str, stage: &'static str, error: Value) {
    let expected = host::CompileFailure {
        stage,
        actual_error: error,
        evaluation_performed: false,
        events_forwarded: "0".into(),
        completed: false,
    };
    assert_eq!(actual, handler_failure_text(&expected));
}

struct Invocation {
    temp: Temp,
    args: compile_args::Args,
}

impl Invocation {
    fn run(&self) -> Result<String, String> {
        let source = source_io::read_source(&self.args.file).unwrap();
        self.run_with_source(&source)
    }
    fn run_with_source(&self, source: &str) -> Result<String, String> {
        let mut status: Vec<(String, String)> = Vec::new();
        let result = selected_compile::run(&self.args, source, |verb, message| {
            status.push((verb.to_owned(), message))
        });
        match result {
            Ok(code) => {
                assert_eq!(code, std::process::ExitCode::SUCCESS);
                assert_eq!(status.len(), 1);
                assert_eq!(status[0].0, "Prepared");
                Ok(std::fs::read_to_string(
                    self.args.out.join("sherlock-artifact.json"),
                )
                .unwrap())
            }
            Err(e) => {
                assert!(status.is_empty());
                Err(e.to_string())
            }
        }
    }
}

fn make_invocation(
    source: &str,
    nominal: Option<(&str, &str)>,
    observe: bool,
    config: &str,
) -> Invocation {
    let temp = Temp::new();
    std::fs::write(temp.0.join("source.mori"), source).unwrap();
    std::fs::write(temp.0.join("config.json"), config).unwrap();
    std::fs::write(temp.0.join("target.json"), TARGET).unwrap();
    let mut args = compile_args::Args {
        file: temp.0.join("source.mori"),
        target: temp.0.join("target.json"),
        out: temp.0.join("out"),
        execution_profile: Some(host::EXECUTION_PROFILE.into()),
        source_profile: Some(BODY_PROFILE.into()),
        entry: true,
        action: None,
        execution_config: Some(temp.0.join("config.json")),
        execution_inputs: None,
        nominal_inputs: None,
        nominal_function: None,
        observe,
    };
    if let Some((inputs, function)) = nominal {
        std::fs::write(temp.0.join("nominal.json"), inputs).unwrap();
        args.nominal_inputs = Some(temp.0.join("nominal.json"));
        args.nominal_function = Some(function.into());
    }
    Invocation { temp, args }
}

fn manual_args(temp: &Temp) -> compile_args::Args {
    compile_args::Args {
        file: temp.0.join("missing-source.mori"),
        target: temp.0.join("missing-target.json"),
        out: temp.0.join("out"),
        execution_profile: Some(host::EXECUTION_PROFILE.into()),
        source_profile: Some(BODY_PROFILE.into()),
        entry: true,
        action: None,
        execution_config: Some(temp.0.join("missing-config.json")),
        execution_inputs: None,
        nominal_inputs: None,
        nominal_function: None,
        observe: false,
    }
}

fn run_manual(args: &compile_args::Args) -> Result<String, String> {
    let inv_source = "unused";
    let mut status: Vec<String> = Vec::new();
    match selected_compile::run(args, inv_source, |v, _| status.push(v.to_owned())) {
        Ok(code) => {
            assert_eq!(code, std::process::ExitCode::SUCCESS);
            Ok(std::fs::read_to_string(args.out.join("sherlock-artifact.json")).unwrap())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn good_value() -> Value {
    json!({
        "format": "sherlock-nominal-external-inputs/1",
        "arguments": [
            {"name": "chain", "value": {"kind": "Constructor",
                "type": {"kind": "Apply", "name": "Chain", "args": [{"kind": "UInt"}]},
                "ctor": "Link",
                "fields": [
                    {"name": "head", "value": {"kind": "UInt", "value": "7"}},
                    {"name": "tail", "value": {"kind": "Constructor",
                        "type": {"kind": "Apply", "name": "Chain", "args": [{"kind": "UInt"}]},
                        "ctor": "End", "fields": []}}
                ]}},
            {"name": "n", "value": {"kind": "UInt", "value": "42"}},
            {"name": "flag", "value": {"kind": "Bool", "value": true}}
        ]
    })
}

fn good_inputs() -> String {
    good_value().to_string()
}

fn nominal_failure(source: &str, inputs: &str, function: &str) -> host::CompileFailure {
    host::nominal_body_command::compile_with_nominal_inputs(
        source, CONFIG, TARGET, inputs, function,
    )
    .err()
    .expect("nominal refusal")
}

fn assert_nominal_variant(failure: &host::CompileFailure, variant: &str) -> Value {
    assert_eq!(failure.stage, "nominal-input");
    assert!(!failure.evaluation_performed);
    assert_eq!(failure.events_forwarded, "0");
    assert!(!failure.completed);
    let v = serde_json::to_value(failure).unwrap();
    assert_eq!(v["actual_error"]["source_profile"], BODY_PROFILE);
    assert_eq!(
        v["actual_error"]["body_typechecking"]["status"], "not-performed"
    );
    assert_eq!(v["actual_error"]["span"], Value::Null);
    let inner = v["actual_error"]["error"]["NominalInput"].clone();
    assert_eq!(inner["variant"], variant);
    inner
}

fn typed_budget() -> DraftParseBudget {
    DraftParseBudget {
        max_source_bytes: 32768,
        max_tokens: 8192,
        max_functions: 8,
        max_parameters_per_function: 16,
        max_total_parameters: 16,
        max_ast_occurrences: 512,
        max_parser_recursion_depth: 128,
        max_structural_ast_depth: 64,
    }
}

fn typed_config() -> ExtCompileConfig {
    let bounds = BoundsC {
        code_rows: 256, function_rows: 8, code_child_slots: 256,
        parameter_slots: 16, pattern_rows: 128, match_plan_rows: 64,
        match_arm_rows: 32, type_rows: 64, constructor_rows: 64,
        constructor_field_slots: 128, max_arity: 4, constant_nodes: 128,
        constant_field_slots: 128, data_nodes: 64, data_field_slots: 128,
        environment_cells: 64, frames: 64, helper_tasks: 64,
        serialization_bytes: 1048576, max_reference_work: 100000,
        max_helper_work: 100000, step_cap: 1,
    };
    ExtCompileConfig {
        bounds,
        layout: LayoutSchemaC {
            layout_version: ("independent-public-generic-runtime".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds,
            capacities: LayoutCapacitiesC {
                name_rows: 64, name_bytes: 1024,
                occurrence_path_slots: 1024, value_slots: 4096,
            },
            ext_capacities: Some(ExtCapacitiesC {
                plan_case_slots: 256, field_path_slots: 512,
                pattern_child_slots: 256, ..ExtCapacitiesC::all_zero()
            }),
            boundary_capacities_present: false,
        },
        domain: ExtDomain::GENERIC_ABSENT,
        fuel: mori_core::sherlock::helper_machine::shared::RunIntC::Nat(4096),
        stack: mori_core::sherlock::helper_machine::shared::RunIntC::Nat(64),
    }
}

fn typed_bound(
    source: &str,
    config: ExtCompileConfig,
) -> Result<
    mori_core::sherlock_compile_artifact::nominal_body_compile::NominalBodyCompileArtifact,
    mori_core::sherlock_compile_artifact::nominal_body_compile::NominalBodyCompileFailure,
> {
    compile_nominal_source_host_bounded_with_nominal_inputs(
        source,
        typed_budget(),
        config,
        None,
        NominalBindingRequest {
            function_name: "pick".into(),
            input: good_value(),
        },
    )
}

fn idx(v: &Value) -> usize {
    v.as_str().expect("decimal-string index").parse().unwrap()
}

fn slice_of(v: &Value) -> (usize, usize) {
    (idx(&v["start"]), idx(&v["len"]))
}

#[test]
fn nominal_pair_clap_requires_and_conflicts() {
    let base: Vec<String> = [
        "compile", "file.mori", "--target", "t.json", "--out", "out",
        "--execution-profile", host::EXECUTION_PROFILE,
        "--source-profile", BODY_PROFILE, "--entry",
        "--execution-config", "c.json",
    ]
    .iter().map(|s| s.to_string()).collect();
    let with = |extra: &[&str]| -> Vec<String> {
        base.iter().cloned()
            .chain(extra.iter().map(|s| s.to_string()))
            .collect()
    };
    // Half pair: each flag requires the other through clap.
    assert!(parsed(with(&["--nominal-inputs", "n.json"])).is_err());
    assert!(parsed(with(&["--nominal-function", "pick"])).is_err());
    // Pair conflicts with action / execution-inputs at the clap layer.
    assert!(parsed(with(&[
        "--nominal-inputs", "n.json", "--nominal-function", "pick",
        "--action", "check",
    ])).is_err());
    assert!(parsed(with(&[
        "--nominal-inputs", "n.json", "--nominal-function", "pick",
        "--execution-inputs", "e.json",
    ])).is_err());
    // Complete pair parses and retains both values.
    let ok = parsed(with(&[
        "--nominal-inputs", "n.json", "--nominal-function", "pick",
    ]))
    .unwrap();
    assert_eq!(ok.nominal_inputs.as_deref(), Some(std::path::Path::new("n.json")));
    assert_eq!(ok.nominal_function.as_deref(), Some("pick"));
    assert!(ok.entry);
    assert!(ok.action.is_none());
}

#[test]
fn manual_nominal_pair_scope_refusal_precedes_io() {
    // Half pair, all IO paths missing: exact select payload, no out dir.
    let temp = Temp::new();
    let mut args = manual_args(&temp);
    args.nominal_inputs = Some(temp.0.join("missing-nominal.json"));
    let err = run_manual(&args).unwrap_err();
    handler_refusal(&err, "select",
        json!({"message": "--nominal-inputs and --nominal-function must be supplied together"}));
    assert!(!args.out.exists());

    // Only the function name present is the same half-pair refusal.
    let temp = Temp::new();
    let mut args = manual_args(&temp);
    args.nominal_function = Some("pick".into());
    let err = run_manual(&args).unwrap_err();
    handler_refusal(&err, "select",
        json!({"message": "--nominal-inputs and --nominal-function must be supplied together"}));
    assert!(!args.out.exists());

    // Complete pair on the structural nominal profile: scope refusal first.
    let temp = Temp::new();
    let mut args = manual_args(&temp);
    args.source_profile = Some("moriarty-sherlock-nominal/1".into());
    args.nominal_inputs = Some(temp.0.join("missing-nominal.json"));
    args.nominal_function = Some("pick".into());
    let err = run_manual(&args).unwrap_err();
    handler_refusal(&err, "select",
        json!({"message": "--nominal-inputs is supported only for nominal-body-typed/1 entry compilation"}));
    assert!(!args.out.exists());

    // Complete pair on a Core action selection: same scope refusal.
    let temp = Temp::new();
    let mut args = manual_args(&temp);
    args.source_profile = None;
    args.entry = false;
    args.action = Some("check".into());
    args.nominal_inputs = Some(temp.0.join("missing-nominal.json"));
    args.nominal_function = Some("pick".into());
    let err = run_manual(&args).unwrap_err();
    handler_refusal(&err, "select",
        json!({"message": "--nominal-inputs is supported only for nominal-body-typed/1 entry compilation"}));
    assert!(!args.out.exists());

    // Old Core-only guard keeps priority over the nominal guard.
    let temp = Temp::new();
    let mut args = manual_args(&temp);
    args.execution_inputs = Some(temp.0.join("missing-inputs.json"));
    args.nominal_inputs = Some(temp.0.join("missing-nominal.json"));
    args.nominal_function = Some("pick".into());
    let err = run_manual(&args).unwrap_err();
    handler_refusal(&err, "select",
        json!({"message": "--execution-inputs is supported only for Core/1 action compilation"}));
    assert!(!args.out.exists());
}

#[test]
fn nominal_json_strict_failure_retains_stage() {
    // SHARED_DEPENDENCY_PROPAGATION: expected payload is the existing strict
    // parser's own Error value, obtained from mori_core::json::parse itself.
    let deep = format!("{}{}", "[".repeat(40), "]".repeat(40));
    let many_nodes = format!("[{}0]", "0,".repeat(65_536));
    for bad in [
        "{",
        r#"{"format":"a","format":"b"}"#,
        "{} true",
        deep.as_str(),
        many_nodes.as_str(),
    ] {
        let parser_error = mori_core::json::parse(bad).err()
            .expect("existing strict parser refusal");
        let failure = nominal_failure(SOURCE, bad, "pick");
        assert_eq!(failure.stage, "nominal-input");
        assert!(!failure.evaluation_performed);
        assert_eq!(failure.events_forwarded, "0");
        assert!(!failure.completed);
        assert_eq!(
            failure.actual_error,
            serde_json::to_value(&parser_error).unwrap()
        );
        assert_eq!(failure.actual_error["code"], "mori::core::json");
    }
    // Through the real handler: same failure text, nothing published.
    let inv = make_invocation(SOURCE, Some(("{", "pick")), false, CONFIG);
    let direct = nominal_failure(SOURCE, "{", "pick");
    let err = inv.run().unwrap_err();
    assert_eq!(err, handler_text_of(&direct));
    assert!(!inv.args.out.exists());
}

#[test]
fn unknown_and_ambiguous_nominal_function_payloads() {
    let f = nominal_failure(SOURCE, &good_inputs(), "missing");
    let inner = assert_nominal_variant(&f, "UnknownFunction");
    assert_eq!(inner["name"], "missing");

    let f = nominal_failure(AMBIGUOUS_SOURCE, &good_inputs(), "pick");
    let inner = assert_nominal_variant(&f, "AmbiguousFunction");
    assert_eq!(inner["name"], "pick");
    assert_eq!(inner["count"], 2);
}

#[test]
fn nominal_arguments_missing_extra_swapped_duplicate_payloads() {
    let mut missing = good_value();
    missing["arguments"].as_array_mut().unwrap().pop();
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &missing.to_string(), "pick"), "MissingArgument");
    assert_eq!(inner["index"], 2);
    assert_eq!(inner["name"], "flag");

    let mut extra = good_value();
    extra["arguments"].as_array_mut().unwrap().push(
        json!({"name": "surplus", "value": {"kind": "UInt", "value": "1"}}));
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &extra.to_string(), "pick"), "ExtraArgument");
    assert_eq!(inner["index"], 3);
    assert_eq!(inner["name"], "surplus");

    // True unique two-name swap: names stay unique, order is refused.
    let mut swapped = good_value();
    swapped["arguments"][0]["name"] = json!("n");
    swapped["arguments"][1]["name"] = json!("chain");
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &swapped.to_string(), "pick"), "ArgumentNameMismatch");
    assert_eq!(inner["index"], 0);
    assert_eq!(inner["expected"], "chain");
    assert_eq!(inner["actual"], "n");

    // Duplicate is reported before any name-mismatch at the same position.
    let mut duplicate = good_value();
    duplicate["arguments"][1]["name"] = json!("chain");
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &duplicate.to_string(), "pick"), "DuplicateArgument");
    assert_eq!(inner["index"], 1);
    assert_eq!(inner["name"], "chain");
}

#[test]
fn nominal_constructor_type_and_field_payloads() {
    let mut wrong_ty = good_value();
    wrong_ty["arguments"][0]["value"]["type"]["args"][0] = json!({"kind": "Bool"});
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &wrong_ty.to_string(), "pick"), "TypeMismatch");
    assert_eq!(inner["argument_index"], 0);
    assert_eq!(inner["expected"], "Chain<UInt>");
    assert_eq!(inner["actual"], "Chain<Bool>");

    let mut unknown_ctor = good_value();
    unknown_ctor["arguments"][0]["value"]["ctor"] = json!("Nope");
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &unknown_ctor.to_string(), "pick"), "UnknownConstructor");
    assert_eq!(inner["type_name"], "Chain");
    assert_eq!(inner["ctor"], "Nope");

    let mut missing_field = good_value();
    missing_field["arguments"][0]["value"]["fields"].as_array_mut().unwrap().pop();
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &missing_field.to_string(), "pick"), "MissingField");
    assert_eq!(inner["type_name"], "Chain");
    assert_eq!(inner["ctor"], "Link");
    assert_eq!(inner["field_index"], 1);
    assert_eq!(inner["field_name"], "tail");

    let mut extra_field = good_value();
    extra_field["arguments"][0]["value"]["fields"].as_array_mut().unwrap()
        .push(json!({"name": "extra", "value": {"kind": "UInt", "value": "1"}}));
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &extra_field.to_string(), "pick"), "ExtraField");
    assert_eq!(inner["field_index"], 2);
    assert_eq!(inner["actual_name"], "extra");

    let mut renamed = good_value();
    renamed["arguments"][0]["value"]["fields"][1]["name"] = json!("wrong");
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &renamed.to_string(), "pick"), "FieldNameMismatch");
    assert_eq!(inner["field_index"], 1);
    assert_eq!(inner["expected"], "tail");
    assert_eq!(inner["actual"], "wrong");

    // Reordered fields refuse on the first position, in declaration order.
    let mut reordered = good_value();
    reordered["arguments"][0]["value"]["fields"][0]["name"] = json!("tail");
    reordered["arguments"][0]["value"]["fields"][1]["name"] = json!("head");
    let inner = assert_nominal_variant(
        &nominal_failure(SOURCE, &reordered.to_string(), "pick"), "FieldNameMismatch");
    assert_eq!(inner["field_index"], 0);
    assert_eq!(inner["expected"], "head");
    assert_eq!(inner["actual"], "tail");
}

#[test]
fn nominal_scalar_wire_is_exact() {
    // Positive canonical boundaries through the public command API.
    for n in ["0", "340282366920938463463374607431768211455"] {
        let mut v = good_value();
        v["arguments"][1]["value"]["value"] = json!(n);
        v["arguments"][2]["value"]["value"] = json!(false);
        host::nominal_body_command::compile_with_nominal_inputs(
            SOURCE, CONFIG, TARGET, &v.to_string(), "pick",
        )
        .unwrap_or_else(|failure| panic!("canonical UInt boundary and native Bool compile: {}", serde_json::to_string(&failure).unwrap()));
    }
    for (text, variant) in [
        ("042", "InvalidUInt"),
        ("-1", "InvalidUInt"),
        ("340282366920938463463374607431768211456", "UIntOutOfRange"),
    ] {
        let mut v = good_value();
        v["arguments"][1]["value"]["value"] = json!(text);
        let inner = assert_nominal_variant(
            &nominal_failure(SOURCE, &v.to_string(), "pick"), variant);
        assert_eq!(inner["argument_index"], 1);
        assert_eq!(inner["text"], text);
    }
    // Bool must be a native JSON bool, and UInt a canonical decimal string.
    let mut string_bool = good_value();
    string_bool["arguments"][2]["value"]["value"] = json!("true");
    assert_nominal_variant(
        &nominal_failure(SOURCE, &string_bool.to_string(), "pick"), "JsonTypeMismatch");
    let mut numeric_uint = good_value();
    numeric_uint["arguments"][1]["value"]["value"] = json!(42);
    assert_nominal_variant(
        &nominal_failure(SOURCE, &numeric_uint.to_string(), "pick"), "JsonTypeMismatch");
}

#[test]
fn nominal_existing_budget_boundary_is_exact() {
    fn chain_ty(arg: TypeExprSrc) -> TypeExprSrc {
        TypeExprSrc::Apply { name: "Chain".into(), args: vec![arg] }
    }
    let program = ExtProgram {
        types: vec![TypeDeclSrc {
            name: "Chain".into(),
            parameters: vec!["T".into()],
            ctors: vec![
                CtorDeclSrc { name: "End".into(), fields: vec![] },
                CtorDeclSrc {
                    name: "Link".into(),
                    fields: vec![
                        CtorFieldSrc {
                            name: "head".into(),
                            ty: TypeExprSrc::Param { name: "T".into() },
                        },
                        CtorFieldSrc {
                            name: "tail".into(),
                            ty: chain_ty(TypeExprSrc::Param { name: "T".into() }),
                        },
                    ],
                },
            ],
        }],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![ExtFunction {
            name: "pick".into(),
            parameters: vec![
                mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::ExtParamSrc {
                    name: "chain".into(), ty: chain_ty(TypeExprSrc::UInt),
                },
                ExtParamSrc { name: "n".into(), ty: TypeExprSrc::UInt },
                ExtParamSrc { name: "flag".into(), ty: TypeExprSrc::Bool },
            ],
            return_type: TypeExprSrc::UInt,
            body: ExtExpr::Var { name: "n".into() },
        }],
        entry: ExtExpr::Var { name: "unused".into() },
    };
    let limits = |ast: usize, depth: usize| NominalInputLimits {
        max_ast_occurrences: ast,
        max_structural_ast_depth: depth,
    };
    // Hand count (see SOURCE comment): synthesized 13, depth 4, existing 0.
    let bound = decode_and_bind_nominal_inputs(
        &program, "pick", &good_value(), limits(13, 4), 0,
    )
    .expect("equal-limit decode");
    assert_eq!(bound.binding.synthesized_expr_nodes, 4);
    assert_eq!(bound.binding.synthesized_value_nodes, 5);
    assert_eq!(bound.binding.synthesized_type_nodes, 4);
    assert_eq!(bound.binding.synthesized_structural_depth, 4);
    assert_eq!(bound.binding.combined_ast_occurrences, 13);
    assert!(matches!(
        decode_and_bind_nominal_inputs(&program, "pick", &good_value(), limits(12, 4), 0),
        Err(NominalInputError::CombinedAstBudgetExceeded {
            existing: 0, synthesized: 13, limit: 12
        })
    ));
    assert!(matches!(
        decode_and_bind_nominal_inputs(&program, "pick", &good_value(), limits(13, 3), 0),
        Err(NominalInputError::StructuralDepthExceeded { depth: 4, limit: 3 })
    ));
    assert!(matches!(
        decode_and_bind_nominal_inputs(&program, "pick", &good_value(), limits(0, 4), 0),
        Err(NominalInputError::LimitsInvalid { field: "max_ast_occurrences" })
    ));
    assert!(matches!(
        decode_and_bind_nominal_inputs(&program, "pick", &good_value(), limits(13, 0), 0),
        Err(NominalInputError::LimitsInvalid { field: "max_structural_ast_depth" })
    ));

    // Same hand count through the selected full handler entry (existing 2,
    // combined 15): equal limit compiles, one below refuses at nominal-input.
    let cfg_equal = CONFIG.replace(
        "\"max_ast_nodes\": \"256\"", "\"max_ast_nodes\": \"15\"");
    assert!(cfg_equal.contains("\"max_ast_nodes\": \"15\""));
    make_invocation(SOURCE, Some((&good_inputs(), "pick")), false, &cfg_equal)
        .run()
        .expect("equal combined budget through handler");
    let cfg_below = CONFIG.replace(
        "\"max_ast_nodes\": \"256\"", "\"max_ast_nodes\": \"14\"");
    let inv = make_invocation(SOURCE, Some((&good_inputs(), "pick")), false, &cfg_below);
    let err = inv.run().unwrap_err();
    let direct = host::nominal_body_command::compile_with_nominal_inputs(
        SOURCE, &cfg_below, TARGET, &good_inputs(), "pick",
    )
    .err()
    .unwrap();
    assert_eq!(assert_nominal_variant(&direct, "CombinedAstBudgetExceeded")["limit"], 14);
    assert_eq!(err, handler_text_of(&direct));
    assert!(!inv.args.out.exists());

    let cfg_depth = CONFIG.replace(
        "\"max_structural_ast_depth\": \"32\"", "\"max_structural_ast_depth\": \"3\"");
    let direct = host::nominal_body_command::compile_with_nominal_inputs(
        SOURCE, &cfg_depth, TARGET, &good_inputs(), "pick",
    )
    .err()
    .unwrap();
    let inner = assert_nominal_variant(&direct, "StructuralDepthExceeded");
    assert_eq!(inner["depth"], 4);
    assert_eq!(inner["limit"], 3);
}

#[test]
fn bound_entry_replaces_original_before_body_check() {
    // Absent mode: the original entry body error is retained.
    let absent = host::nominal_body_command::compile(SOURCE, CONFIG, TARGET)
        .err()
        .expect("original entry refusal");
    assert_eq!(absent.stage, "body-type");
    let v = serde_json::to_value(&absent).unwrap();
    assert_eq!(v["actual_error"]["body_typechecking"]["status"], "failed");
    assert_eq!(
        v["actual_error"]["error"]["kind"], json!({"UnknownVariable": "missing_sentinel"}));
    assert_eq!(
        v["actual_error"]["error"]["source"]["function_index"], Value::Null);

    // Bound mode: the synthesized entry replaces it before body checking.
    let bound = host::nominal_body_command::compile_with_nominal_inputs(
        SOURCE, CONFIG, TARGET, &good_inputs(), "pick",
    )
    .unwrap_or_else(|failure| panic!("bound entry compiles: {}", serde_json::to_string(&failure).unwrap()));
    assert_eq!(bound["evaluation_performed"], false);
    assert_eq!(bound["body_typechecking"]["status"], "completed");
    assert_eq!(bound["nominal_binding"]["function_name"], "pick");

    // Declarations and functions are unchanged in the typed artifact.
    let artifact = typed_bound(SOURCE, typed_config()).expect("typed bound artifact");
    assert_eq!(artifact.name(), "BoundCheck");
    let provenance = artifact.provenance();
    assert_eq!(provenance.declarations.len(), 1);
    assert_eq!(provenance.declarations[0].name.name, "Chain");
    assert_eq!(provenance.declarations[0].constructors.len(), 2);
    assert_eq!(provenance.functions.len(), 1);
    assert_eq!(provenance.functions[0].name.name, "pick");
    let binding = artifact.binding().expect("binding retained");
    assert_eq!(binding.function_index, 0);
    assert_eq!(binding.function_name, "pick");
    assert_eq!(binding.combined_ast_occurrences, 15);
}

#[test]
fn bound_provenance_marks_original_entry_stale() {
    let bound = host::nominal_body_command::compile_with_nominal_inputs(
        SOURCE, CONFIG, TARGET, &good_inputs(), "pick",
    )
    .unwrap_or_else(|failure| panic!("bound entry compiles: {}", serde_json::to_string(&failure).unwrap()));
    assert_eq!(
        bound["nominal_binding"]["entry_provenance"],
        json!({
            "synthesized_entry_span": null,
            "retained_entry_provenance": "stale-original-entry-source-only",
            "source_spans_repurposed": false
        })
    );

    // Typed artifact retains the actual original Entry sites, unchanged.
    let artifact = typed_bound(SOURCE, typed_config()).expect("typed bound artifact");
    let entry_sites: Vec<_> = artifact
        .provenance()
        .expressions
        .iter()
        .filter(|site| site.owner == SourceOwner::Entry)
        .collect();
    assert_eq!(entry_sites.len(), 1);
    let span = entry_sites[0].span;
    assert_eq!(
        &SOURCE[span.start as usize..span.end as usize], "missing_sentinel");
    // No synthesized-entry body error is fabricated: UNEXERCISED by design.
}

#[test]
fn function_body_error_keeps_actual_source_span() {
    let failure = typed_bound(ILLTYPED_SOURCE, typed_config())
        .err()
        .expect("unused-function body refusal");
    assert!(!failure.body_check_completed());
    let span = failure.span.expect("actual function span");
    assert_eq!(&ILLTYPED_SOURCE[span.start as usize..span.end as usize], "n");
    match &failure.error {
        NominalBodyCompileError::BodyType(error) => {
            assert_eq!(error.source.function_index, Some(1));
            assert_eq!(error.source.child_path, Vec::<usize>::new());
        }
        other => panic!("wrong stage: {other:?}"),
    }

    // Handler projection keeps the same stage, payload, and span.
    let direct = host::nominal_body_command::compile_with_nominal_inputs(
        ILLTYPED_SOURCE, CONFIG, TARGET, &good_inputs(), "pick",
    )
    .err()
    .unwrap();
    assert_eq!(direct.stage, "body-type");
    let v = serde_json::to_value(&direct).unwrap();
    assert_eq!(v["actual_error"]["body_typechecking"]["status"], "failed");
    assert_eq!(
        v["actual_error"]["error"]["kind"],
        json!({"Expected": {"expected": "Bool", "actual": "UInt"}}));
    assert_eq!(
        v["actual_error"]["error"]["source"]["function_index"], json!("1"));
    assert_eq!(v["actual_error"]["span"]["start"], span.start);
    assert_eq!(v["actual_error"]["span"]["end"], span.end);
}

#[test]
fn nominal_owner_and_genesis_refusals_preserve_payload() {
    // JSON maps one bounds object into both copies: StepCap 0 via JSON.
    let cfg_step = CONFIG.replace("\"step_cap\": \"1\"", "\"step_cap\": \"0\"");
    assert!(cfg_step.contains("\"step_cap\": \"0\""));
    let f = host::nominal_body_command::compile_with_nominal_inputs(
        SOURCE, &cfg_step, TARGET, &good_inputs(), "pick",
    )
    .err()
    .expect("StepCap refusal");
    assert_eq!(f.stage, "owner-admission");
    let v = serde_json::to_value(&f).unwrap();
    assert_eq!(v["actual_error"]["body_typechecking"]["status"], "completed");
    assert_eq!(
        v["actual_error"]["error"],
        json!({"CoreTooling": {"CapacityNotAdmitted": {
            "kind": "StepCap", "needed": {"Exact": "1"},
            "limit": "0", "locator": null}}}));

    // Genesis fuel above the existing reference-work bound, via JSON.
    let cfg_fuel = CONFIG.replace("\"value\": \"4096\"", "\"value\": \"100001\"");
    assert!(cfg_fuel.contains("\"value\": \"100001\""));
    let f = host::nominal_body_command::compile_with_nominal_inputs(
        SOURCE, &cfg_fuel, TARGET, &good_inputs(), "pick",
    )
    .err()
    .expect("genesis fuel refusal");
    assert_eq!(f.stage, "genesis");
    let v = serde_json::to_value(&f).unwrap();
    assert_eq!(v["actual_error"]["body_typechecking"]["status"], "completed");
    assert_eq!(
        v["actual_error"]["error"],
        json!({"CapacityNotAdmitted": {
            "kind": "MaxReferenceWork", "needed": {"Exact": "100001"},
            "limit": "100000", "locator": null}}));

    // Layout mismatch cannot be expressed in JSON; use the public typed API.
    let mut cfg = typed_config();
    cfg.layout.bounds.step_cap = 2;
    let failure = typed_bound(SOURCE, cfg).err().expect("layout refusal");
    assert!(failure.body_check_completed());
    assert!(failure.span.is_none());
    assert!(matches!(
        &failure.error,
        NominalBodyCompileError::OwnerAdmission(
            mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::ExtTypeBuildError::CoreTooling(
                mori_core::sherlock::helper_machine::shared::Tooling::AdmissionRejected {
                    reason: mori_core::sherlock::helper_machine::shared::AdmissionReason::LayoutBoundsMismatch,
                    detail, ..
                })) if detail == "layout.bounds != bounds"
    ));
}

#[test]
fn nominal_absent_pair_preserves_artifact_and_errors() {
    // SHARED_PRODUCER_EQUIVALENCE_ONLY: current handler versus the current
    // public command it delegates to, both without a nominal pair.
    let direct = host::nominal_body_command::compile(SOURCE, CONFIG, TARGET)
        .err()
        .expect("original entry still refuses in absent mode");
    // Use a well-typed absent-pair source for the artifact equivalence.
    let ok_source = SOURCE.replace("entry missing_sentinel;", "entry pick(\n    construct Chain<UInt>::Link(7, construct Chain<UInt>::End()), 42, true);");
    let direct = host::nominal_body_command::compile(&ok_source, CONFIG, TARGET)
        .unwrap_or_else(|failure| panic!("absent-pair public command: {}", serde_json::to_string(&failure).unwrap()));
    assert!(direct.get("nominal_binding").is_none());
    let text = make_invocation(&ok_source, None, false, CONFIG)
        .run()
        .expect("absent-pair handler");
    assert_eq!(text, serde_json::to_string_pretty(&direct).unwrap());
    let parsed: Value = serde_json::from_str(&text).unwrap();
    assert!(parsed.get("nominal_binding").is_none());

    // Error equivalence on the same absent-pair refusal.
    let direct_err = host::nominal_body_command::compile(SOURCE, CONFIG, TARGET)
        .err()
        .unwrap();
    assert_eq!(direct_err.stage, "body-type");
    let inv = make_invocation(SOURCE, None, false, CONFIG);
    let err = inv.run().unwrap_err();
    assert_eq!(err, handler_text_of(&direct_err));
    assert!(!inv.args.out.exists());
    let _ = direct;
}

#[test]
fn nominal_compile_only_retains_full_bound_program() {
    let text = make_invocation(SOURCE, Some((&good_inputs(), "pick")), false, CONFIG)
        .run()
        .expect("bound compile-only handler");
    let a: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(a["format"], "sherlock-nominal-body-typed-host-artifact/1");
    assert_eq!(a["evaluation_performed"], false);
    assert_eq!(a["body_typechecking"]["status"], "completed");
    assert_eq!(a["source"]["name"], "BoundCheck");
    assert_eq!(
        a["source"]["source_digest"],
        mori_core::intent::hash_bytes(SOURCE.as_bytes()));
    assert_eq!(
        a["config"]["file_digest"],
        mori_core::intent::hash_bytes(CONFIG.as_bytes()));
    assert_eq!(
        a["target"]["file_digest"],
        mori_core::intent::hash_bytes(TARGET.as_bytes()));
    assert_eq!(
        a["target"]["descriptor"], serde_json::from_str::<Value>(TARGET).unwrap());

    let b = &a["nominal_binding"];
    assert_eq!(b["format"], "sherlock-nominal-external-inputs/1");
    assert_eq!(b["function_index"], 0);
    assert_eq!(b["function_name"], "pick");
    assert_eq!(
        b["input_digest"],
        mori_core::intent::hash_bytes(good_inputs().as_bytes()));
    let args = b["arguments"].as_array().unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0]["name"], "chain");
    assert_eq!(args[0]["declared_type"], "Chain<UInt>");
    assert_eq!(args[0]["bound_value"], "Link(UInt(7),End())");
    assert_eq!(args[1]["name"], "n");
    assert_eq!(args[1]["declared_type"], "UInt");
    assert_eq!(args[1]["bound_value"], "UInt(42)");
    assert_eq!(args[2]["name"], "flag");
    assert_eq!(args[2]["declared_type"], "Bool");
    assert_eq!(args[2]["bound_value"], "Bool(true)");
    assert_eq!(b["synthesized_expr_nodes"], 4);
    assert_eq!(b["synthesized_value_nodes"], 5);
    assert_eq!(b["synthesized_type_nodes"], 4);
    assert_eq!(b["synthesized_structural_depth"], 4);
    assert_eq!(b["combined_ast_occurrences"], 15);

    // Full 23-field ProgramView, resolved from declared rows (no assumed IDs).
    let view = &a["admission"]["program_view"];
    assert_eq!(view.as_object().unwrap().len(), 23);
    let names: Vec<&str> = view["names"].as_array().unwrap()
        .iter().map(|n| n.as_str().unwrap()).collect();
    assert_eq!(
        view["decl_templates"],
        json!([{
            "name": "Chain", "parameters": ["T"],
            "ctors": [
                {"name": "End", "fields": []},
                {"name": "Link", "fields": [
                    {"name": "head", "ty": {"Param": {"name": "T"}}},
                    {"name": "tail", "ty": {"Apply": {
                        "name": "Chain",
                        "args": [{"Param": {"name": "T"}}]}}}
                ]}
            ]
        }])
    );
    // Resolve Chain<UInt> from its type row: name plus type-argument row.
    let (chain_pos, _) = view["types"].as_array().unwrap().iter().enumerate()
        .find(|(_, row)| {
            names[idx(&row["name_id"])] == "Chain" && {
                let (start, len) = slice_of(&row["args"]);
                len == 1 && view["type_args"][start] == json!("UInt")
            }
        })
        .expect("declared Chain<UInt> type row");
    let chain_type_id = json!({"TypeId": chain_pos.to_string()});

    let function = &view["functions"][0];
    assert_eq!(names[idx(&function["function_name"])], "pick");
    assert_eq!(function["return_type"], json!("UInt"));
    let (pstart, plen) = slice_of(&function["params"]);
    assert_eq!(plen, 3);
    let param_names: Vec<&str> = (0..plen)
        .map(|i| names[idx(&view["parameters"][pstart + i]["binder"])])
        .collect();
    assert_eq!(param_names, ["chain", "n", "flag"]);
    assert_eq!(view["parameters"][pstart]["sort"], chain_type_id);
    assert_eq!(view["parameters"][pstart + 1]["sort"], json!("UInt"));
    assert_eq!(view["parameters"][pstart + 2]["sort"], json!("Bool"));

    // Resolve Link under Chain<UInt> from constructor/field rows.
    let (link_pos, link_row) = view["constructors"].as_array().unwrap()
        .iter().enumerate()
        .find(|(_, row)| {
            names[idx(&row["name_id"])] == "Link"
                && row["owning_type"] == chain_type_id
        })
        .expect("declared Link constructor row");
    let (fstart, flen) = (idx(&link_row["field_start"]), idx(&link_row["field_len"]));
    assert_eq!(flen, 2);
    assert_eq!(names[idx(&view["ctor_fields"][fstart]["name_id"])], "head");
    assert_eq!(view["ctor_fields"][fstart]["type_id"], json!("UInt"));
    assert_eq!(names[idx(&view["ctor_fields"][fstart + 1]["name_id"])], "tail");
    assert_eq!(view["ctor_fields"][fstart + 1]["type_id"], chain_type_id);

    // Entry is the synthesized Call; its literal constants agree with inputs.
    let entry = &view["expressions"][idx(&view["entry"])]["Call"];
    assert_eq!(names[idx(&entry["function_name"])], "pick");
    let (astart, alen) = slice_of(&entry["args"]);
    assert_eq!(alen, 3);
    let arg_constant = |i: usize| -> Value {
        let expr_index = idx(&view["children"][astart + i]);
        let constant_index = idx(&view["expressions"][expr_index]["Literal"]["constant"]);
        view["constants"][constant_index].clone()
    };
    let chain_constant = arg_constant(0);
    assert_eq!(chain_constant["Constructor"]["type_id"], chain_type_id);
    assert_eq!(
        chain_constant["Constructor"]["ctor_id"], json!(link_pos.to_string()));
    assert_eq!(arg_constant(1), json!({"U128": {"marker_free": true, "n": "42"}}));
    assert_eq!(arg_constant(1)["U128"]["n"], "42");
    assert_eq!(arg_constant(2)["Bool"]["b"], true);
}

#[test]
fn nominal_observe_uses_bound_actual_pair() {
    // SHARED_PRODUCER_EQUIVALENCE_ONLY on this module's own synthetic input:
    // current handler envelope versus the current direct public API.
    // No private-cohort values and no independent runtime-result claim.
    let direct = host::nominal_body_command::observe_with_nominal_inputs(
        SOURCE, CONFIG, TARGET, &good_inputs(), "pick",
    )
    .unwrap_or_else(|failure| panic!("direct bound observation: {}", serde_json::to_string(&failure).unwrap()));
    let text = make_invocation(SOURCE, Some((&good_inputs(), "pick")), true, CONFIG)
        .run()
        .expect("handler bound observation");
    assert_eq!(text, serde_json::to_string_pretty(&direct).unwrap());

    let a: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(a["format"], "sherlock-nominal-host-observation/1");
    assert_eq!(a["preparation"]["nominal_binding"]["function_name"], "pick");
    assert_eq!(
        a["preparation"]["nominal_binding"]["arguments"][1]["bound_value"],
        "UInt(42)");
    assert_eq!(a["preparation"]["evaluation_performed"], false);
    assert_eq!(a["execution"]["evaluation_performed"], true);
    assert_eq!(a["execution"]["completed"], true);
    assert_eq!(a["execution"]["event_capture"], "not-retained");
}
