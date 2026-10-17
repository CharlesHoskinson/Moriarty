# Main8174 Sherlock entrypoints and callers

Unreviewed source-only notes at `8174e52975d8db9bf59636fea0fa338c0f47c855`; native authority0, runtimeReady=false and full goal open. All20 family and full financial/ACTUS/DeFi/recursion/private-handoff/multiparent/Preprod obligations remain open. No project code was compiled,imported or run.

## S01 SOURCE_DECLARATION

Core lib publicly declares sherlock,sherlock_source and sherlock_bounded_source; the draft source/typechecker/tests/image bridge modules are separate private cfg(test) declarations.

`CORE-LIB.rs:8–31`, SHA-256 `df45168fa300562f532577739731c257d016a387fc943b25975ae46e3968a2fa`:

```text
pub mod sherlock;

// Unregistered draft source capability: Core-only ordinary host test proposal.
// These lexer-dependent modules stay outside Sherlock's standalone Std graph.
#[cfg(test)]
#[path = "sherlock/draft_source.rs"]
mod sherlock_draft;
#[cfg(test)]
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[cfg(test)]
#[path = "sherlock/draft_source_tests.rs"]
mod sherlock_draft_tests;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge.rs"]
mod sherlock_draft_image;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge_tests.rs"]
mod sherlock_draft_image_tests;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
```

## S02 SOURCE_DECLARATION

Sherlock mod publicly reexports Machine,VisitResult/VisitStop and reference_types; image/program/helper/layout/arithmetic modules displayed here are cfg(test). Implementation bodies of Machine/reference streams are not inspected in this packet.

`SHERLOCK-MOD.rs:8–25`, SHA-256 `562f563d6f679d188bdb283d1dc191b34f19459a98d9c8a260911f16e7e367de`:

```text
pub mod compare;
mod reference;
mod reference_stream;
pub mod reference_display;
mod reference_types;

pub use reference::Machine;
pub use reference_stream::{VisitResult, VisitStop};
pub use reference_types::*;

#[cfg(test)]
mod reference_tests;

#[cfg(test)]
mod reference_bounded_tests;

#[cfg(test)]
mod reference_bounded_extra_tests;
```

`SHERLOCK-MOD.rs:39–79`, SHA-256 `562f563d6f679d188bdb283d1dc191b34f19459a98d9c8a260911f16e7e367de`:

```text
// Host-only image proposal: exercised here without exporting a public runtime API.
#[cfg(test)]
pub(crate) mod program_image;

// Private flat-table consumer proposal, normal-oracle comparisons only.
#[cfg(test)]
pub(crate) mod image_reference;

// Host-only event grouping; every internal reference record stays observable.
#[cfg(test)]
mod reference_slice;

// Unadopted host snapshot rows; normal materialization checks only.
#[cfg(test)]
mod machine_image;

// Unadopted finite call-helper proposal; clock and handle adapters stay separate.
#[cfg(test)]
mod helper_schedule;

// Host projection of helper completions to the unchanged A reference boundary.
#[cfg(test)]
mod helper_call_adapter;

// Causal occurrence metadata on the private image execution path.
#[cfg(test)]
mod source_origin_normal_tests;

// Internal L4 host preparation source proposal; H1, no network/untrusted API.
// Compilation and all normal tests remain UNRUN pending separate admission.
#[cfg(test)]
mod layout_a;

#[cfg(test)]
mod reference_stream_tests;

// Private host arithmetic/fuel materialization; no constraint or wire ABI.
#[cfg(test)]
mod arithmetic_witness;
#[cfg(test)]
mod arithmetic_witness_normal_tests;
```

## S06 CALL_EDGE

Public source_projection_report(&CoreProgram)->SourceProjectionReport calls crate::validate once,iterates actions,attempts prepare_validated_action on validation success,and converts each local Result into Prepared/Unsupported report entries. It returns report metadata rather than Result<Artifact,...>.

`SOURCE-ADAPTER.rs:116–144`, SHA-256 `d7fba670a7903ff751d895806375c652ecaa21a9f71689446b3cabf9f960e065`:

```text
/// Report every action without changing the real compiler's supported subset.
pub fn source_projection_report(program: &Program) -> SourceProjectionReport {
    let validation = crate::validate(program);
    let mut remaining = MAX_PROJECTION_OCCURRENCES;
    let actions = program.actions.iter().enumerate().map(|(index, action)| {
        let result = match &validation {
            Ok(()) => prepare_validated_action(program, index, &mut remaining),
            Err(error) => Err(diagnostic(index, None, None, "core-validation", &error.code)),
        };
        match result {
            Ok(map) => ActionProjection::Prepared { report: map.report },
            Err(diagnostic) => ActionProjection::Unsupported {
                action_index: index, action: action.name.clone(), diagnostic,
            },
        }
    }).collect();
    SourceProjectionReport {
        format: FORMAT.into(), source_language: program.language.clone(),
        source_semantics: program.semantics.clone(), agreement: program.name.clone(),
        reference_profile: PROFILE.into(), qualification: "source-static-scalar-projection-only".into(),
        source_function_support: false, source_branch_support: false,
        source_recursion_support: false, source_data_support: false,
        evaluation_performed: false, source_expression_count: program.expressions.len(),
        source_action_count: program.actions.len(), source_state_count: program.states.len(),
        source_asset_count: program.assets.len(),
        coverage: "direct scalar nodes; stateless actions; ordered lets and one final emit value; first unsupported site per action; no effects execution or unvisited-site mapping".into(),
        actions,
    }
}
```

## S08 CALL_EDGE

Public prepare_scalar_action(&CoreProgram,action_index)->Result<ScalarActionMap,ProjectionDiagnostic> calls Core validate,starts a local occurrence counter,and calls prepare_validated_action.

`SOURCE-ADAPTER.rs:146–155`, SHA-256 `d7fba670a7903ff751d895806375c652ecaa21a9f71689446b3cabf9f960e065`:

```text
/// Compile a typed scalar body with dynamic ordered parameters. The generated
/// function is a representation wrapper; it is not a source function feature.
pub fn prepare_scalar_action(program: &Program, action_index: usize)
    -> Result<ScalarActionMap, ProjectionDiagnostic>
{
    crate::validate(program).map_err(|error|
        diagnostic(action_index, None, None, "core-validation", &error.code))?;
    let mut remaining = MAX_PROJECTION_OCCURRENCES;
    prepare_validated_action(program, action_index, &mut remaining)
}
```

## B02 CALL_EDGE

Public observe_scalar_action(&CoreProgram,action_index,&[SherlockValue],initial_fuel,bounds)->Result<BoundedSourceObservation,HostObservationRefusal> checks source shape/arguments/action,then calls Core validate before its count pass and local bounds.

`BOUNDED-SOURCE.rs:126–171`, SHA-256 `cd7ad770ef310486cf0e6ff38beb6aa5ae33075d96fb56df97f1752f3bd18e68`:

```text
pub fn observe_scalar_action(program: &crate::Program, action_index: usize, arguments: &[Value],
    initial_fuel: u128, bounds: HostObservationBounds)
    -> Result<BoundedSourceObservation, HostObservationRefusal>
{
    within(HostMeasure::SourceActions, Some(count(program.actions.len())),
        count(bounds.max_source_actions))?;
    within(HostMeasure::SourceExpressions, Some(count(program.expressions.len())),
        count(bounds.max_source_expressions))?;
    let statements = program.actions.iter()
        .try_fold(0u128, |sum, action| sum.checked_add(count(action.statements.len())));
    within(HostMeasure::SourceStatements, statements, count(bounds.max_source_statements))?;
    within(HostMeasure::ScalarArguments, Some(count(arguments.len())),
        count(bounds.max_scalar_arguments))?;
    if let Some(index) = arguments.iter()
        .position(|value| !matches!(value, Value::UInt(_) | Value::Bool(_)))
    {
        return Err(HostObservationRefusal::NonScalarArgument { index });
    }
    let action = program.actions.get(action_index).ok_or(HostObservationRefusal::NoSuchAction {
        action_index, action_count: program.actions.len(),
    })?;
    crate::validate(program).map_err(HostObservationRefusal::CoreValidation)?;

    // Validation orders every child before its parent, so one forward pass gives
    // unshared tree sizes and node depths without recursion. Saturated sizes
    // exceed every usize bound and are refused below.
    let mut sizes: Vec<u128> = Vec::with_capacity(program.expressions.len());
    let mut depths: Vec<u128> = Vec::with_capacity(program.expressions.len());
    for expression in &program.expressions {
        let (children, fanout): ([crate::ExprId; 2], usize) = match expression {
            crate::Expr::Binary { lhs, rhs, .. } => ([*lhs, *rhs], 2),
            crate::Expr::Not { value } => ([*value, 0], 1),
            _ => ([0, 0], 0),
        };
        let (mut size, mut depth) = (1u128, 0u128);
        for child in &children[..fanout] {
            let index = usize::from(*child);
            let (Some(child_size), Some(child_depth)) = (sizes.get(index), depths.get(index)) else {
                return Err(contract("expression-order"));
            };
            size = size.saturating_add(*child_size);
            depth = depth.max(*child_depth);
        }
        sizes.push(size);
        depths.push(depth + 1);
    }
```

## B04 CALL_EDGE

After preparation,observe_scalar_action checks generated signature/body extents,constructs one Expr::Call containing literal scalar arguments,constructs Sherlock Program,then calls Machine::new,snapshot,run_steps,snapshot and step. Pending returns UnpublishedOutcome; otherwise it returns BoundedSourceObservation with QUALIFICATION.

`BOUNDED-SOURCE.rs:203–244`, SHA-256 `cd7ad770ef310486cf0e6ff38beb6aa5ae33075d96fb56df97f1752f3bd18e68`:

```text
    let mapped = prepare_scalar_action(program, action_index)
        .map_err(HostObservationRefusal::Projection)?;
    if count(mapped.report.occurrences.len()) > occurrences {
        return Err(contract("occurrence-precount"));
    }
    let function = mapped.function;
    if function.parameters.len() != action.inputs.len()
        || !function.parameters.iter().all(|parameter| scalar(parameter.ty))
        || !scalar(function.return_type)
    {
        return Err(contract("generated-signature"));
    }
    // Each occurrence is exactly one fresh generated body node.
    let (nodes, observed_ast_nesting) = admitted_body(&function.body, mapped.report.occurrences.len())
        .map_err(contract)?;
    if nodes != mapped.report.occurrences.len() || count(observed_ast_nesting) > ast_nesting_bound {
        return Err(contract("generated-body-extent"));
    }
    let argument_types_match_inputs = arguments.len() == action.inputs.len()
        && arguments.iter().zip(&action.inputs).all(|(value, input)| matches!((value, input.ty),
            (Value::UInt(_), crate::Type::UInt) | (Value::Bool(_), crate::Type::Bool)));

    // The fixed diagnostic invocation of the returned real function.
    let entry = Arc::new(Expr::Call(function.name.clone(), arguments.iter()
        .map(|value| Arc::new(Expr::Literal(value.clone()))).collect()));
    let generated = Arc::new(Program { functions: vec![function], entry });
    let mut machine = Machine::new(generated.clone(), initial_fuel, bounds.stack_limit);
    let initial = machine.snapshot();
    let records = machine.run_steps(events);
    let terminal = machine.snapshot();
    if matches!(terminal.outcome, Outcome::Pending) {
        return Err(HostObservationRefusal::UnpublishedOutcome);
    }
    let terminal_has_next = machine.step().is_some();
    Ok(BoundedSourceObservation {
        report: mapped.report, program: generated, initial, records, terminal, terminal_has_next,
        extent: HostExtent { bounds, projected_occurrences: occurrences, ast_nesting_bound,
            observed_ast_nesting, step_record_bound, frame_bound, environment_entry_bound,
            peak_slot_bound, argument_types_match_inputs },
        qualification: QUALIFICATION,
    })
}
```

`BOUNDED-SOURCE.rs:278–305`, SHA-256 `cd7ad770ef310486cf0e6ff38beb6aa5ae33075d96fb56df97f1752f3bd18e68`:

```text
/// Explicit-stack walk admitting only the forms the scalar adapter emits.
/// Returns the node count and nesting, stopping once `node_limit` is passed.
fn admitted_body(body: &ExprRef, node_limit: usize) -> Result<(usize, usize), &'static str> {
    let mut pending = vec![(body, 1usize)];
    let (mut nodes, mut nesting) = (0usize, 0usize);
    while let Some((expr, depth)) = pending.pop() {
        nodes += 1;
        if nodes > node_limit {
            return Err("generated-node-count");
        }
        nesting = nesting.max(depth);
        match &**expr {
            Expr::Literal(Value::UInt(_) | Value::Bool(_)) | Expr::Var(_) => {}
            Expr::Let(_, value, body) => {
                pending.push((value, depth + 1));
                pending.push((body, depth + 1));
            }
            Expr::Binary(BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Eq
                | BinaryOp::Lt, left, right) => {
                pending.push((left, depth + 1));
                pending.push((right, depth + 1));
            }
            Expr::Unary(UnaryOp::Not, value) => pending.push((value, depth + 1)),
            _ => return Err("generated-node-form"),
        }
    }
    Ok((nodes, nesting))
}
```

## C02 CALL_EDGE

compile_reported first selects compile_signed or compile according to target.signed_policy and propagates any error. Only after that Artifact return does it call source_projection_report and bound_paths,then return ReportedCompilation{artifact,source_projection}.

`BACKEND-PROJECTION.rs:17–47`, SHA-256 `e346ca735246d4339c5d32ce684bff7ff0dc790a412ef59b59c06b31313ed6d5`:

```text
#[derive(Serialize)]
pub struct CompilationSourceProjectionReport {
    pub compiled_program_hash: String,
    pub artifact_format: String,
    pub abi: String,
    pub relation: String,
    pub projection: SourceProjectionReport,
}

pub struct ReportedCompilation {
    pub artifact: Artifact,
    pub source_projection: CompilationSourceProjectionReport,
}

pub fn compile_reported(program: &Program, target: &Target) -> Result<ReportedCompilation, Error> {
    // Preserve the same signed/unsigned selection as the existing CLI.
    let artifact = if target.signed_policy.is_some() {
        crate::compile_signed(program, target)
    } else {
        crate::compile(program, target)
    }?;
    // An unsupported projection remains metadata after successful compilation.
    let mut projection = source_projection_report(program);
    bound_paths(&mut projection);
    let source_projection = CompilationSourceProjectionReport {
        compiled_program_hash: artifact.program_hash.clone(),
        artifact_format: artifact.format.clone(), abi: artifact.abi.clone(),
        relation: artifact.relation.clone(), projection,
    };
    Ok(ReportedCompilation { artifact, source_projection })
}
```

## C03 SOURCE_DECLARATION

CompilationSourceProjectionReport copies Artifact program_hash/format/abi/relation plus projection. ReportedCompilation holds Artifact and separate CompilationSourceProjectionReport; bound_paths can replace a Prepared report entry with Unsupported metadata using checked path-element cost. The inspected wrapper passes no projected Function into the earlier compile call.

`BACKEND-PROJECTION.rs:14–46`, SHA-256 `e346ca735246d4339c5d32ce684bff7ff0dc790a412ef59b59c06b31313ed6d5`:

```text
pub const MAX_PROJECTION_PATH_ELEMENTS: usize =
    mori_core::sherlock_source::MAX_PROJECTION_OCCURRENCES * 66;

#[derive(Serialize)]
pub struct CompilationSourceProjectionReport {
    pub compiled_program_hash: String,
    pub artifact_format: String,
    pub abi: String,
    pub relation: String,
    pub projection: SourceProjectionReport,
}

pub struct ReportedCompilation {
    pub artifact: Artifact,
    pub source_projection: CompilationSourceProjectionReport,
}

pub fn compile_reported(program: &Program, target: &Target) -> Result<ReportedCompilation, Error> {
    // Preserve the same signed/unsigned selection as the existing CLI.
    let artifact = if target.signed_policy.is_some() {
        crate::compile_signed(program, target)
    } else {
        crate::compile(program, target)
    }?;
    // An unsupported projection remains metadata after successful compilation.
    let mut projection = source_projection_report(program);
    bound_paths(&mut projection);
    let source_projection = CompilationSourceProjectionReport {
        compiled_program_hash: artifact.program_hash.clone(),
        artifact_format: artifact.format.clone(), abi: artifact.abi.clone(),
        relation: artifact.relation.clone(), projection,
    };
    Ok(ReportedCompilation { artifact, source_projection })
```

`BACKEND-PROJECTION.rs:52–73`, SHA-256 `e346ca735246d4339c5d32ce684bff7ff0dc790a412ef59b59c06b31313ed6d5`:

```text
fn bound_paths(projection: &mut SourceProjectionReport) {
    let mut remaining = MAX_PROJECTION_PATH_ELEMENTS;
    for entry in &mut projection.actions {
        let withheld = match entry {
            ActionProjection::Prepared { report } => {
                let cost = report.occurrences.iter().try_fold(0usize, |sum, occurrence|
                    sum.checked_add(occurrence.body_path.len())?.checked_add(1));
                match cost.and_then(|cost| remaining.checked_sub(cost)) {
                    Some(left) => { remaining = left; None }
                    None => Some((report.action_index, report.action.clone())),
                }
            }
            ActionProjection::Unsupported { .. } => None,
        };
        if let Some((action_index, action)) = withheld {
            *entry = ActionProjection::Unsupported { action_index, action,
                diagnostic: ProjectionDiagnostic { action_index, statement_index: None,
                    expression: None, construct: "projection-path-budget".into(),
                    code: "projection-path-budget-exhausted".into() } };
        }
    }
}
```

## C05 CALL_EDGE

CLI commands::run routes Command::Compile to compile::run. compile::run checks a literal moriarty/2 route first and calls backend stage::compile_staged there; its other branch parses Core source then calls backend source_projection::compile_reported. These are inspected non-test command source calls.

`CLI-COMMANDS.rs:10–14`, SHA-256 `34ceaabb846836fad4b0df7ddc788dff37c786bfbed9397b8358330a87e3c081`:

```text
mod call_io;
pub mod check;
pub mod compile;
pub mod compile_call;
mod core_io;
```

`CLI-COMMANDS.rs:41–45`, SHA-256 `34ceaabb846836fad4b0df7ddc788dff37c786bfbed9397b8358330a87e3c081`:

```text
pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
```

`CLI-COMPILE.rs:18–41`, SHA-256 `cadcdcfd1951219af01e2677df1b7b56494238243be3a7c41e3bd4f86cbf0867`:

```text
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let source = core_io::read_source(&args.file)?;
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        let target: mori_backend::Target = core_io::read_json(&args.target)?;
        let artifact = mori_backend::stage::compile_staged(&source, &target, &args.out)
            .map_err(core_io::operation)?;
        let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
        std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
            path: args.out.clone(),
            source,
        })?;
        publish_staged_artifact(&artifact, &manifest, &args.out)?;
        shell.status(
            "Compiled",
            format!("{} circuits into {}", artifact.circuits.len(), args.out.display()),
        );
        return Ok(ExitCode::SUCCESS);
    }
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
```

## C06 SOURCE_DECLARATION

In the nonstaged CLI branch,reported.artifact is taken as artifact; reported.source_projection is serialized with source_hash and published separately as source-projection.json. publish_artifact writes circuit.zkir values and artifact.json from Artifact. This is source consumer placement,not evidence that files were generated.

`CLI-COMPILE.rs:36–80`, SHA-256 `cadcdcfd1951219af01e2677df1b7b56494238243be3a7c41e3bd4f86cbf0867`:

```text
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;
    let projection = serde_json::to_string_pretty(&serde_json::json!({
        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
        "compilation": reported.source_projection,
    }))
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    super::proof_io::publish_new(&args.out.join("source-projection.json"), projection.as_bytes())?;
    publish_artifact(&artifact, &manifest, &args.out)?;
    shell.status(
        "Compiled",
        format!(
            "{} circuits into {}",
            artifact.circuits.len(),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}

// The caller has reserved a new directory after all input/compiler preflight.
fn publish_artifact(
    artifact: &mori_backend::Artifact,
    manifest: &str,
    out: &Path,
) -> super::Result<()> {
    for (index, circuit) in artifact.circuits.iter().enumerate() {
        let path = out.join(format!("{index:03}-{}.zkir", circuit.action));
        super::proof_io::publish_new(&path, circuit.zkir.as_bytes())?;
    }
    let path = out.join("artifact.json");
    // The manifest is the completion marker; publish it after every circuit.
    super::proof_io::publish_new(&path, manifest.as_bytes())?;
    Ok(())
```

## C07 CALL_EDGE

A separate models/sherlock/reference source producer imports observe_scalar_action,parses fixed included Core specimens in prepare,and calls observe_scalar_action(source,0,arguments,fuel,BOUNDS) from its observe function. It has a standalone main that loops fixed inputs/fuels. This is a non-test diagnostic source caller,not an observed installed/compiler/ledger entry.

`MODEL-SOURCE-OBSERVATIONS.rs:12–25`, SHA-256 `66bde18f5505eef625025f030107b3a54acd6b015e8bd80bef956f48125e8445`:

```text
use mori_core::sherlock::reference_display::{KDisplay, environment_to_k};
use mori_core::sherlock::{Snapshot, StepRecord, Value, K_SOURCE_SHA256, PROFILE};
use mori_core::sherlock_bounded_source::{BoundedSourceObservation, HostObservationBounds,
                                        observe_scalar_action};
use mori_core::sherlock_source::ScalarActionReport;
use std::io::{self, BufWriter, Write};

struct Input {
    case_id: &'static str,
    source_path: &'static str,
    source: &'static str,
    arguments: &'static [u128],
    max_fuel: u128,
}
```

`MODEL-SOURCE-OBSERVATIONS.rs:49–82`, SHA-256 `66bde18f5505eef625025f030107b3a54acd6b015e8bd80bef956f48125e8445`:

```text
const BOUNDS: HostObservationBounds = HostObservationBounds {
    max_source_actions: 8,
    max_source_expressions: 64,
    max_source_statements: 16,
    max_scalar_arguments: 4,
    max_projection_occurrences: 64,
    max_ast_nesting: 16,
    stack_limit: 8,
    max_step_records: 64,
    max_retained_slots: 1 << 20,
};

struct Prepared {
    source: mori_core::Program,
    arguments: Vec<Value>,
    zero_fuel: BoundedSourceObservation,
}

fn prepare(input: &Input) -> io::Result<Prepared> {
    let source = mori_core::parse(input.source)
        .map_err(|error| io::Error::other(format!("{}: parse: {error:?}", input.case_id)))?;
    let arguments: Vec<_> = input.arguments.iter().copied().map(Value::UInt).collect();
    // The first actual bounded result supplies the program/preparation record
    // and is reused as the fuel-zero run. No hand-built Sherlock AST is used.
    let zero_fuel = observe(input.case_id, &source, &arguments, 0)?;
    Ok(Prepared { source, arguments, zero_fuel })
}

fn observe(case_id: &str, source: &mori_core::Program, arguments: &[Value], fuel: u128)
    -> io::Result<BoundedSourceObservation>
{
    observe_scalar_action(source, 0, arguments, fuel, BOUNDS)
        .map_err(|error| io::Error::other(format!("{case_id}: bounded observation: {error:?}")))
}
```

`MODEL-SOURCE-OBSERVATIONS.rs:207–221`, SHA-256 `66bde18f5505eef625025f030107b3a54acd6b015e8bd80bef956f48125e8445`:

```text
fn main() -> io::Result<()> {
    // Parse both real sources and retain their actual bounded fuel-zero results
    // before emitting any observation document. There is no arbitrary input.
    let prepared = INPUTS.iter().map(prepare).collect::<io::Result<Vec<_>>>()?;
    let mut out = BufWriter::new(io::stdout().lock());
    write!(out, "{{")?;
    string_field(&mut out, "producer", "sherlock-core-source-reference")?;
    write!(out, ",")?;
    string_field(&mut out, "profile", PROFILE)?;
    write!(out, ",")?;
    string_field(&mut out, "source_sha256", K_SOURCE_SHA256)?;
    write!(out, ",")?;
    string_field(&mut out, "boundary", "post-startup")?;
    write!(out, ",\"programs\":[")?;
    for (index, (input, actual)) in INPUTS.iter().zip(&prepared).enumerate() {
```

`MODEL-SOURCE-OBSERVATIONS.rs:241–262`, SHA-256 `66bde18f5505eef625025f030107b3a54acd6b015e8bd80bef956f48125e8445`:

```text
    write!(out, "],\"runs\":[")?;
    let mut first = true;
    for (input, actual) in INPUTS.iter().zip(&prepared) {
        for fuel in 0..=input.max_fuel {
            let next_observation;
            let observed = if fuel == 0 {
                &actual.zero_fuel
            } else {
                next_observation = observe(input.case_id, &actual.source, &actual.arguments, fuel)?;
                &next_observation
            };
            if !first { write!(out, ",")?; }
            first = false;
            // Retain the unchanged common diagnostic shape. Extent and
            // qualification are typed API metadata, not part of this codec.
            observation(&mut out, input.case_id, &observed.initial, &observed.records,
                        &observed.terminal, observed.terminal_has_next)?;
        }
    }
    writeln!(out, "]}}")?;
    out.flush()
}
```

## C08 INFERENCE

The inspected non-test command→compile_reported→source_projection_report edge defeats a blanket claim that these public adapters have no production source caller. Its returned projection is separate metadata after Artifact construction; this does not establish Sherlock-to-Cardano compilation,semantic correspondence or runtime behavior.

`CLI-COMMANDS.rs:41–45`, SHA-256 `34ceaabb846836fad4b0df7ddc788dff37c786bfbed9397b8358330a87e3c081`:

```text
pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
```

`CLI-COMPILE.rs:36–55`, SHA-256 `cadcdcfd1951219af01e2677df1b7b56494238243be3a7c41e3bd4f86cbf0867`:

```text
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;
    let projection = serde_json::to_string_pretty(&serde_json::json!({
        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
        "compilation": reported.source_projection,
    }))
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    super::proof_io::publish_new(&args.out.join("source-projection.json"), projection.as_bytes())?;
    publish_artifact(&artifact, &manifest, &args.out)?;
```

`BACKEND-PROJECTION.rs:31–47`, SHA-256 `e346ca735246d4339c5d32ce684bff7ff0dc790a412ef59b59c06b31313ed6d5`:

```text
pub fn compile_reported(program: &Program, target: &Target) -> Result<ReportedCompilation, Error> {
    // Preserve the same signed/unsigned selection as the existing CLI.
    let artifact = if target.signed_policy.is_some() {
        crate::compile_signed(program, target)
    } else {
        crate::compile(program, target)
    }?;
    // An unsupported projection remains metadata after successful compilation.
    let mut projection = source_projection_report(program);
    bound_paths(&mut projection);
    let source_projection = CompilationSourceProjectionReport {
        compiled_program_hash: artifact.program_hash.clone(),
        artifact_format: artifact.format.clone(), abi: artifact.abi.clone(),
        relation: artifact.relation.clone(), projection,
    };
    Ok(ReportedCompilation { artifact, source_projection })
}
```

## Search and read limits

SEARCH-RECEIPTS.json preserves six exact fixed-tree query outputs. Actual non-test source callers were inspected; tests/audit hits are query matches only. Empty main8174 k2 pathspecs provide no evidence of absence in the separately pinned00ffd Cardano driver. Core1 CLI projection metadata,main staged compiler calls,public typed host adapters and private cfg(test) drafts remain distinct. No grammar,compiler/proof correspondence,capacity adequacy or Cardano compilation follows from the declarations. Machine/staged compiler/helper bodies listed under U01–U04 remain uninspected.
