Provide ONE whole independent ordinary SOURCE+RESULT opinion with both sourceAssessment and actualResultAssessment. Direct reviewToken mori-inspect-header-4c03df51-1c16daa2586f6b619ae6. approve, request_changes and abstain are available; no forced approval. Read complete FULL-INPUT once before judging and report actual read limits/dissent. Do not fabricate reads or quotes. No provider/product/Git execution, shared mutation, private captures, SDK/native/ledger/wallet action or security judgment. Preserve original148/292/20 and allU0-U7/M1-M17/MC/SP/G/full-language/UTXO/financial/composition/proof/target obligations.

Exact candidate 4c03df51f78337d1a9e17cfaaabbbea112fef0df/tree27b0211b04921485afae58603811549b7b93cb7d, parent615eceee9c6250d2f203cb5807d5c75482dbdb5d; precisely2full changed bodies in CANDIDATE and full before inspect. The first source-only inspect probe changes language().is_ok_and(...) into existing language().map_err(core_io::operation)? equality. args.target.is_none() remains first &&operand; native/target block and all downstreamhelpers unchanged. No prefix detection, supported-profile change or native/target execution. Source-only unknown/malformed headers now retain existing precise producer Error and standard miette CLI rendering rather than Core1fallback. UNI-018 supports explicit profile/consumer reporting but exact wording comes from existing producer/libraryheader test. Inspect wholecandidate and real producer/lexer/renderer/consumer units, not helper-only summaries.

AUTHORFORMATTINGQUALIFICATION is mandatory: original private author test Rustfmtcheckexit1; root changed ONLY one argumentarray linewrap before RED/GREEN. Full original AUTHOR-TEST-BEFORE-FORMAT, original FORMAT-ONLY-DELTA and exactdiff retained. Current actual fulltest body equals TEST-AFTER-FORMAT (duplicate physicalcopy not repeated inline). Complete rawexpectation literals unchanged. Source2path/hash/line/realquote coverage must assess actual rootformatted test, not relabel original author bytes as tested. Builder statically reconstructed exactreplacement and checked every rawliteral before/after. Parentiginalauthorfile/receipt remains unchanged.

Actual phase RED is ONE failed aggregate exit101 with THREE raw CLI processes captured before first Rust humanstderrassertion fails (unknowncase). Later Rust case assertions are unreached, despite allprocesscaptures existing. Original root FROZEN-OBSERVATION-COMPARISON and builder separately compare all6complete producerErrorDTOs/UTF8offsets16/9/25, exactinputsource,exit1,emptyentirestdout and fullstderr before/after. ThreeREDstderr streams differ from frozen repair; allthreeGREEN match. GREEN is ONE aggregatePASS covering all3headers, not3Rusttestcases. Read complete all4phase argv/START/CHILD/RESULT/stdout/stderr,fullrootdriver,CHECKPLAN/final/mutation/binding,all6rawcaptures(eachheader/process/producererror/fullstdout/fullstderr),allfrozenexpectations and fulltestassertions. Zero-byte stdout is a realcomplete input, not omission. Public human stream has NOoffsetfield: offsets belong to captured/tested producerJSON. Source-author expectedvalues were implementation-exposed beforeexecution, not an independentfinancial/work oracle. Keep original RED/failures/freeze unchanged; no generatednewgoldens.

Existingpositivecontrols eachPASS: capabilities reported_core_profiles_select_real_source_only_cli_consumers runs capabilitiesJSON and bothvalidCore1/Core2check/sourceinspect with statedfields/filepreservation; core_continuations check_and_source_inspect_route_core_two_without_legacy_consumers runs Reservecheck+inspect asserting normaloutputlabels,language/semantics andstagecosts11/15/10. Complete selectedtests/helpers/usedliteral program and normal_output_labels_from_opus values/provenance mandatory; fulloriginaltest/oraclebodies preservedphysicalcustody. Only unused reserve/escrow/compatibility oraclebranches are not mandatoryreading. NOusedsemanticfield or assertion omitted. No exactcompletepublicreportbytecontrol or native/target result is inferred beyond actualwrittenassertions. Existing external fixture paths remain original guidelines deviations.

Physicalcustody vs MODELREAD: original full903row before/after source manifests physicallypreserved,all903actualafterpins verified againstcurrentfiles,rootpostcommitbinding reports Git equality/clean2pathcommit. Read exact SOURCE-RESULT-CUSTODY and original POST-COMMIT-BINDING instead of hundreds of unrelatedunchangedhashrows. Do not claim fullmanifest modelread/fullrepoaudit. Every mandatory unit has originabsolutepath/fullsourcesha/exactlinerange/unitsha. Fullinspectcandidate includes unchangednativehelpers for wholebodycoverage; do not expand into unrelated native/backend closure or supply securityjudgments. Exactduplicate aftersource/test representations are physicalcustody but fullactualcandidate bodies mandatoryonce. All originalinputs are preserved. No fakecontextreduction or narrower candidate/review scope.

Use actual typed Draft2020-12 REVIEW-RESPONSE.schema.json (not an exemplar). Return one JSON whole opinion with directtoken,bothassessments,all2fullbody/hash/realquoteanchors,4phase/log/assertioncoverage,all6rawcapturecoverage,formattingdelta/REDassertionreachability/controlassertionscope dispositions,mandatoryinputcoverage,actualreadlimits,dissent,findings,limitations. Schema cannot prove genuine reads/quotes/substantive reasoning. If anyrequiredinput is unread, name exactpaths and actual limit, abstain for affected assessment and still return bothwholeassessmentobjects. No provider timeout/packet author claim supplies approval. No newexecution, nativeauthorization, admission or parentexitclosure.


BEGIN FILE FORMAT-EXACT.diff | SHA256 4df3dd203b07f90aa747d50637d7f7246cb7dbbc41e2d3972e8cffc5084d5c9a | BYTES 542
--- AUTHOR-TEST-BEFORE-FORMAT.rs
+++ TEST-AFTER-FORMAT.rs
@@ -91,12 +91,7 @@
         let dir = self.root.join(name);
         fs::create_dir(&dir).unwrap();
         fs::write(dir.join("header.mori"), source).unwrap();
-        let args = [
-            "--color",
-            "never",
-            "inspect",
-            "header.mori",
-        ];
+        let args = ["--color", "never", "inspect", "header.mori"];
         let captured = Command::new(env!("CARGO_BIN_EXE_mori"))
             .current_dir(&dir)
             .args(args)

END FILE FORMAT-EXACT.diff


BEGIN FILE source/crates/mori/src/commands/inspect.rs | SHA256 0016e768e086f0e6abdd616e4765838df8563392aaf9a7ea708e8e63bb9d8c78 | BYTES 21672
#[cfg(feature = "native")]
use std::collections::BTreeSet;
use std::{path::PathBuf, process::ExitCode};

#[cfg(feature = "native")]
use mori_backend::{Artifact, CircuitArtifact, PublicBinding};
#[cfg(feature = "native")]
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type};
use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
    /// Compile a public enforcement map against this exact target descriptor
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    #[cfg(not(feature = "native"))]
    if args.target.is_some() {
        return super::native_unavailable();
    }
    let source = super::core_io::read_source(&args.file)?;
    if args.target.is_none()
        && mori_core::language(&source).map_err(super::core_io::operation)? == "moriarty/2"
    {
        let staged = match mori_core::stage::load(&source) {
            Ok(staged) => staged,
            Err(error) => {
                super::simulate::print_stage_report(json!({"status":"rejected", "error":error}))?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let program = staged.program();
        super::simulate::print_stage_report(json!({
            "status": "accepted", "language": program.language, "semantics": program.semantics,
            "agreement": program.name, "states": program.states, "assets": program.assets,
            "actions": program.actions, "expressions": program.expressions,
            "stage_plan": staged.plan(), "enforcement_report": null
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    #[cfg(feature = "native")]
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        // The enforcement map reports the /3 relation. A Core2 --target refuses
        // with Core's typed profile error before reading the target, backend
        // layout or any output; source-only Core2 inspect is handled above.
        let staged = mori_core::stage::load(&source).map_err(super::core_io::operation)?;
        let refusal = match mori_core::stage::require_core1(staged.program()) {
            Err(error) => error,
            Ok(()) => core1_refusal(),
        };
        return Err(super::core_io::operation(refusal));
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let enforcement = match args.target {
        #[cfg(feature = "native")]
        Some(path) => {
            let target: mori_backend::Target = super::core_io::read_json(&path)?;
            // Use the same whole-program admission and explicit mode as compile.
            // No scenario, key, signature, proof or native execution is consumed.
            let artifact = if target.signed_policy.is_some() {
                mori_backend::compile_signed(&program, &target)
            } else {
                mori_backend::compile(&program, &target)
            }
            .map_err(super::core_io::operation)?;
            enforcement_report(&source, &program, &artifact)?
        }
        #[cfg(not(feature = "native"))]
        Some(_) => return super::native_unavailable(),
        None => Value::Null,
    };
    let qualification = if enforcement.is_null() {
        "source-only-unqualified"
    } else {
        "local-compiled-unqualified"
    };
    // Export validates depth/work/byte bounds and escapes public labels before
    // printing anything. A compiler or report-size refusal leaves stdout empty.
    super::core_io::print(&json!({
        "format": "moriarty-enforcement-report/1",
        "qualification": qualification,
        "language": program.language, "semantics": program.semantics,
        "agreement": program.name, "states": program.states,
        "assets": program.assets, "actions": program.actions,
        "expressions": program.expressions,
        "enforcement_report": enforcement,
        "open_premises": [
            "a supplied exact target is required for the compiled enforcement map",
            "ledger authentication and acceptance",
            "native proof soundness and compiler correspondence",
            "account and sponsor authority"
        ]
    }))?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "native")]
fn encoded(predicates: &[&str]) -> Value {
    json!({"status": "encoded-not-verified", "predicates": predicates})
}
#[cfg(feature = "native")]
fn reference(predicates: &[&str]) -> Value {
    json!({"status": "available-not-executed", "predicates": predicates})
}

#[cfg(feature = "native")]
fn enforcement_report(
    source: &str,
    program: &Program,
    artifact: &Artifact,
) -> super::Result<Value> {
    let actions = program
        .actions
        .iter()
        .zip(&artifact.circuits)
        .map(|(action, circuit)| action_report(program, action, circuit))
        .collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "scope": "source-derived-public-transition-relation",
        "identity": {
            "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
            "program_hash": artifact.program_hash, "semantics": artifact.semantics,
            "artifact_format": artifact.format, "abi": artifact.abi,
            "relation": artifact.relation, "authorization_mode": artifact.authorization_mode,
            "status": "host-computed-identities-not-certificates"
        },
        "target": artifact.target,
        "source_checks": {
            "status": "checked-locally",
            "predicates": [
                "exact language and semantic versions; finite acyclic typed expressions",
                "unique declared names, state initializer types and supported statements",
                "complete distinct asset network/issuer/reference/scale identities; scale 0..38",
                "every asset network equals the supplied target network",
                "exact supported ledger and ZKIR pins; bounded nonempty network and anchor",
                "positive target instruction and artifact limits; actual emitted counts fit",
                "source-wide unique duty identities and closed bounded liability registry"
            ],
            "initial_state": "declaration-only; not an authenticated predecessor or genesis",
            "signed_admission": if artifact.target.signed_policy.is_some() {
                "source-wide owner debits and debtor identity; signed label and asset limits checked"
            } else { "absent-unsigned" }
        },
        "numeric_profile": {
            "UInt": {"min": "0", "max": u128::MAX.to_string(), "representation": "canonical decimal strings; checked unsigned 128-bit integral atomic units"},
            "Bool": "true or false; boolean range constraints",
            "rounding": "exact integral operations only; division and directed rounding unsupported",
            "unit_dimensions": "asset identity and scale retained; generic UInt expressions have no asset dimension checker"
        },
        "resources": artifact.resources,
        "resource_scope": "emitted instruction, ZKIR and artifact-envelope counts; not transaction bytes, measured proving k, DUST or wall-clock replay cost",
        "gates": artifact.gates,
        "actions": actions,
        "unknown": unknown_premises()
    }))
}

#[cfg(feature = "native")]
fn action_report(
    program: &Program,
    action: &Action,
    circuit: &CircuitArtifact,
) -> super::Result<Value> {
    let fields: Vec<_> = circuit
        .public_inputs
        .iter()
        .enumerate()
        .map(|(position, binding)| public_field(position, binding))
        .collect();
    let authorization_fields: Vec<_> = circuit.authorization_bindings.iter().map(|binding| {
        let rule = authorization_rule(&binding.name);
        json!({
            "position": binding.position, "binding": binding,
            "source": "target-selected signed owner-intent profile",
            "reference": reference(&[rule]), "constraint": encoded(&[rule, "public transcript Impact binding"]),
            "state": {"status": "unauthenticated", "predicate": "supplied authority, time and history values require independent authenticated acceptance"},
            "unknown": ["owner-key-authority", "history-replay-revocation", "clock-and-external-inputs", "host-commitment-reconstruction", "native-proof-soundness", "compiler-correspondence"]
        })
    }).collect();
    let statements = action.statements.iter().enumerate().map(|(step, statement)| {
        let rule = statement_rule(statement)?;
        Ok(json!({"step": step, "source": statement, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "state": {"status": "computed-unqualified", "predicate": "ordered local transition on supplied pre-state; no ledger acceptance"},
            "unknown": ["ledger-authentication", "native-proof-soundness", "compiler-correspondence"]}))
    }).collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "action": circuit.action, "context_hash": circuit.context_hash,
        "instruction_count": circuit.instruction_count, "replay_steps": circuit.replay_steps,
        "fields": fields, "authorization_fields": authorization_fields,
        "private_input_descriptors": circuit.private_inputs,
        "authorization_constraints": if authorization_fields.is_empty() {
            json!({"status": "absent-unsigned", "predicates": []})
        } else {
            encoded(&[
                "source-fixed context and typed financial values bound into the owner-intent message",
                "nonidentity typed JubJub key and R; canonical response scalar; FENS signature equation",
                "supplied public key coordinates equal the private typed key coordinates",
                "supplied logical time lies in the signed validity interval; time is outside the signed message",
                "per-action complete owner transfer plus fee gross sum <= gross cap; refunds do not reduce gross",
                "per-action owner fees <= fee cap; beneficiary net credit minus owner fees >= net floor",
                "sum of all closing registered owner duties for each asset <= liability cap"
            ])
        },
        "statements": statements,
        "expression_constraints": expression_constraints(program, action)?,
        "effect_frame": circuit.effect_frame, "balance_frame": circuit.balance_frame,
        "liability_frame": circuit.liability_frame,
        "frame_scope": "all declared states, touched balances and the program-wide source duty registry; preservation of other ledger accounts/duties requires the authenticated adapter"
    }))
}

#[cfg(feature = "native")]
fn public_field(position: usize, binding: &PublicBinding) -> Value {
    let range = match binding.ty {
        Type::UInt => "unsigned 128-bit range",
        Type::Bool => "boolean range",
    };
    let rule = match binding.role.as_str() {
        "context" => "equality to source/target/action context hash limbs computed by the host",
        "input" => "supplied typed action input; no external observation authentication",
        "pre_state" => "supplied typed predecessor state; no equality to the declared initializer",
        "pre_balance" => "supplied opening balance in the touched account/asset frame",
        "pre_liability_presence" => {
            "opening duty presence bit; absent presence forces opening amount zero"
        }
        "pre_liability_amount" => "opening duty amount; absent presence forces opening amount zero",
        "post_state" => "equality to the sequentially computed state or unchanged opening state",
        "post_balance" => "equality to opening balance after ordered checked debits and credits",
        "post_liability_presence" => {
            "equality to computed duty presence; unchanged duties persist; absent amount is zero"
        }
        "post_liability_amount" => {
            "equality to checked duty evolution or unchanged opening amount; absent amount is zero"
        }
        "effect" => "equality to the source statement's computed ordered effect value",
        _ => unreachable!("fresh compiler artifacts use the fixed public ABI roles"),
    };
    let supplied = binding.role.starts_with("pre_") || binding.role == "input";
    let mut unknown = vec!["native-proof-soundness", "compiler-correspondence"];
    if binding.role == "context" {
        unknown.push("host-commitment-reconstruction");
    } else if binding.role == "input" {
        unknown.push("clock-and-external-inputs");
    } else {
        unknown.push("ledger-authentication");
    }
    json!({
        "position": position, "binding": binding,
        "source": "checked declarations, action body and source-fixed public ABI",
        "reference": reference(&[rule]),
        "constraint": encoded(&[range, "public transcript Impact binding", rule]),
        "state": {"status": if supplied { "unauthenticated" } else if binding.role == "context" { "host-reconstructed" } else { "computed-unqualified" },
            "predicate": rule},
        "unknown": unknown
    })
}

#[cfg(feature = "native")]
fn authorization_rule(name: &str) -> &'static str {
    match name {
        "payload.lo" | "payload.hi" => {
            "128-bit host payload hash limb included in the signed message; SHA computation is outside the circuit"
        }
        "public_key.x" | "public_key.y" => {
            "native field coordinate equality to the typed signature key; owner/key authentication is external"
        }
        "message" => {
            "native field equality to the computed owner-intent message and signature challenge"
        }
        "nonce" => {
            "128-bit nonce bound into the signed message; unique ledger consumption is external"
        }
        "not_before" | "not_after" => {
            "128-bit signed validity bound; interval checked against supplied at"
        }
        "predecessor.lo" | "predecessor.hi" => {
            "128-bit predecessor hash limb bound into the signed message; history freshness is external"
        }
        "at" => {
            "128-bit supplied logical time within signed interval; clock authentication is external"
        }
        _ if name.ends_with(".gross_cap") => {
            "128-bit signed gross cap bounds this action's owner transfers plus fees"
        }
        _ if name.ends_with(".fee_cap") => "128-bit signed fee cap bounds this action's owner fees",
        _ if name.ends_with(".net_floor") => {
            "128-bit signed floor bounds this action's beneficiary net credit minus owner fees"
        }
        _ if name.ends_with(".liability_cap") => {
            "128-bit signed cap bounds all closing registered owner duties for this asset"
        }
        _ => unreachable!("fresh signed compiler artifacts use the fixed authorization ABI"),
    }
}

/// Legacy enforcement-map helpers are `/1`-only. An Await reaching them gets
/// Core's typed `/1` profile refusal, never a panic or an omitted statement.
#[cfg(feature = "native")]
fn core1_refusal() -> mori_core::Error {
    mori_core::Error {
        code: "mori::core::profile".into(),
        message: "Await requires moriarty-core/2.".into(),
        offset: None,
        step: None,
    }
}

#[cfg(feature = "native")]
fn statement_rule(statement: &Statement) -> super::Result<&'static str> {
    Ok(match statement {
        Statement::Await { .. } => return Err(super::core_io::operation(core1_refusal())),
        Statement::Let { .. } => {
            "evaluate the expression and bind a typed local name for subsequent statements"
        }
        Statement::Requires { .. } | Statement::Ensures { .. } => {
            "assert the boolean predicate at this ordered statement, after any preceding updates"
        }
        Statement::Next { .. } => {
            "publish the exact typed before/after state values and use the new value in subsequent statements"
        }
        Statement::Emit { .. } => "publish the exact typed computed event value",
        Statement::Transfer { .. } | Statement::Fee { .. } => {
            "checked 128-bit source debit and recipient credit; publish exact amount with source-fixed asset and parties"
        }
        Statement::Liability { .. } => {
            "source-fixed duty identity; checked opening amount plus accrual; closing presence true including present-zero"
        }
        Statement::Discharge { .. } => {
            "require opening duty presence; checked opening amount minus discharge; retain presence even at zero"
        }
    })
}

#[cfg(feature = "native")]
fn expression_constraints(program: &Program, action: &Action) -> super::Result<Vec<Value>> {
    // Report only reachable expression nodes. Some nodes are shared by a
    // statement's DAG; iterating the sorted set is bounded and deterministic.
    let mut pending: Vec<ExprId> = action
        .statements
        .iter()
        .map(|s| match s {
            Statement::Await { .. } => Err(super::core_io::operation(core1_refusal())),
            Statement::Requires { predicate } | Statement::Ensures { predicate } => Ok(*predicate),
            Statement::Let { value, .. }
            | Statement::Next { value, .. }
            | Statement::Emit { value, .. } => Ok(*value),
            Statement::Transfer { amount, .. }
            | Statement::Fee { amount, .. }
            | Statement::Liability { amount, .. }
            | Statement::Discharge { amount, .. } => Ok(*amount),
        })
        .collect::<super::Result<Vec<_>>>()?;
    let mut reachable = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !reachable.insert(id) {
            continue;
        }
        match &program.expressions[id as usize] {
            Expr::Binary { lhs, rhs, .. } => pending.extend([*lhs, *rhs]),
            Expr::Not { value } => pending.push(*value),
            _ => {}
        }
    }
    Ok(reachable.into_iter().map(|id| {
        let node = &program.expressions[id as usize];
        let rule = match node {
            Expr::LiabilityPresent { .. } => "read current duty presence under its source-fixed identity",
            Expr::LiabilityAmount { .. } => "read current duty atomic amount under its source-fixed identity",
            Expr::Literal { .. } => "source-fixed typed literal immediate",
            Expr::Name { .. } => "use the typed binding at this ordered statement",
            Expr::Not { .. } => "boolean negation",
            Expr::Binary { op, .. } => match op {
                BinaryOp::Add => "integer addition with unsigned 128-bit result range; overflow refuses",
                BinaryOp::Sub => "integer subtraction with unsigned 128-bit result range; underflow refuses",
                BinaryOp::Mul => "64-bit limb multiplication with zero high product and unsigned 128-bit result range; overflow refuses",
                BinaryOp::Eq | BinaryOp::Ne => "typed equality or inequality with boolean result",
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => "unsigned 128-bit comparison with boolean result",
                BinaryOp::And | BinaryOp::Or => "boolean conjunction or disjunction with boolean result",
            }
        };
        json!({"expression": id, "source": node, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "unknown": ["native-proof-soundness", "compiler-correspondence"]})
    }).collect())
}

#[cfg(feature = "native")]
fn unknown_premises() -> Vec<Value> {
    [
        ("ledger-authentication", "opening state/balances/duty presence and full ledger frame, custody, funding and phase correspondence"),
        ("history-replay-revocation", "authenticated predecessor, unique nonce/head consumption, revocation and cumulative authority across calls; no recursive history proof"),
        ("clock-and-external-inputs", "supplied logical time and document/oracle/recipient observations are not authenticated by an input or interval check"),
        ("native-proof-soundness", "no proof generation or verification performed; native constraint soundness requires its own evidence"),
        ("compiler-correspondence", "source/reference/constraint and actual ledger correspondence are not established by compilation, hashes or this map"),
        ("fraud-proof-and-replay-cost", "no deployed fraud proof, proving k, transaction-byte or measured replay-time certificate"),
        ("owner-key-authority", "unsigned mode has no signature relation; signed mode encodes a signature equation but owner/account/key authority remains external"),
        ("sponsor-authority-and-settlement", "sponsor grant authentication, DUST reservation/spend and observed financial settlement are not encoded by these source statements"),
        ("live-deployment-and-finality", "no live network/anchor/root-of-trust compatibility, installed verifier/circuit coverage, submission or finality checked"),
        ("host-commitment-reconstruction", "source, target, asset, payload and predecessor SHA/JSON identities are host reconstructions; their hash computation is not proved in-circuit"),
        ("financial-unit-and-rounding-conformance", "asset dimensions, canonical price orientation and directed rounding beyond this exact integral Core profile remain open"),
        ("private-composition", "public transition fields and private signature descriptors do not establish confidentiality or private recursive handoff"),
    ].into_iter().map(|(id, predicate)| json!({"id": id, "status": "unproven", "predicate": predicate})).collect()
}

END FILE source/crates/mori/src/commands/inspect.rs


BEGIN FILE source/crates/mori/tests/inspect_header_dispatch.rs | SHA256 f925bcc0ccd78f5a9cc9512f03718830a39475ddd98385531eaff6e90a9b43c3 | BYTES 5351
//! Source-only inspect preserves the Core header producer's diagnostics.
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

struct HeaderCase {
    name: &'static str,
    source: &'static str,
    expected_error: &'static str,
    expected_stderr: &'static str,
}

const CASES: [HeaderCase; 3] = [
    HeaderCase {
        name: "unknown",
        source: r###"language "other";"###,
        expected_error: r###"{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 16,
  "step": null
}"###,
        expected_stderr: r###"  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".

"###,
    },
    HeaderCase {
        name: "missing",
        source: r###"language ;"###,
        expected_error: r###"{
  "code": "mori::core::syntax",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
  "offset": 9,
  "step": null
}"###,
        expected_stderr: r###"  × mori::command: mori::core::syntax: Expected language "moriarty/1" or
  │ "moriarty/2". Expected a quoted string.

"###,
    },
    HeaderCase {
        name: "commented",
        source: r###"// café
language "other";"###,
        expected_error: r###"{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 25,
  "step": null
}"###,
        expected_stderr: r###"  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".

"###,
    },
];

struct Project {
    root: PathBuf,
    temporary: bool,
}

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (root, temporary) = match std::env::var_os("MORI_INSPECT_HEADER_OBSERVATIONS") {
            Some(path) => {
                let path = PathBuf::from(path);
                assert!(path.is_absolute(), "use an absolute capture directory");
                (path, false)
            }
            None => (
                std::env::temp_dir().join(format!(
                    "mori-inspect-header-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                )),
                true,
            ),
        };
        fs::create_dir(&root).expect("use a fresh capture directory");
        Self { root, temporary }
    }

    fn run(&self, name: &str, source: &str) -> Output {
        let dir = self.root.join(name);
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("header.mori"), source).unwrap();
        let args = ["--color", "never", "inspect", "header.mori"];
        let captured = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&dir)
            .args(args)
            .output();
        if let Err(error) = &captured {
            fs::write(
                dir.join("process.json"),
                serde_json::to_vec_pretty(
                    &serde_json::json!({"args": args, "launchError": error.to_string()}),
                )
                .unwrap(),
            )
            .unwrap();
        }
        let output = captured.expect("run actual public source-only inspect");
        // Keep all process bytes/outcomes before any product assertion.
        fs::write(dir.join("stdout.bin"), &output.stdout).unwrap();
        fs::write(dir.join("stderr.txt"), &output.stderr).unwrap();
        fs::write(
            dir.join("process.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "executable": env!("CARGO_BIN_EXE_mori"),
                "args": args, "status": output.status.code(),
            }))
            .unwrap(),
        )
        .unwrap();
        output
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        if self.temporary && !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[test]
fn source_only_inspect_keeps_complete_header_errors() {
    let project = Project::new();
    // Capture each public process stream before assertions, including on failure.
    let captured = CASES
        .iter()
        .map(|case| {
            let error = mori_core::language(case.source)
                .err()
                .map(|error| serde_json::to_value(error).unwrap())
                .unwrap_or(Value::Null);
            let output = project.run(case.name, case.source);
            fs::write(
                project.root.join(case.name).join("producer-error.json"),
                serde_json::to_vec_pretty(&error).unwrap(),
            )
            .unwrap();
            (case, error, output)
        })
        .collect::<Vec<_>>();
    for (case, error, output) in captured {
        let expected: Value = serde_json::from_str(case.expected_error).unwrap();
        assert_eq!(error, expected, "{}: complete producer Error", case.name);
        assert_eq!(output.status.code(), Some(1), "{}", case.name);
        assert!(output.stdout.is_empty(), "{}: complete stdout", case.name);
        assert_eq!(
            output.stderr,
            case.expected_stderr.as_bytes(),
            "{}: complete human stream",
            case.name
        );
    }
}

END FILE source/crates/mori/tests/inspect_header_dispatch.rs


BEGIN FILE actual/AUTHOR-TEST-BEFORE-FORMAT.rs | SHA256 38d4b9fd8da39b6b3b2a03f78cd93f20abaa3fb77fb2f264aa5b68b25b969f9b | BYTES 5410
//! Source-only inspect preserves the Core header producer's diagnostics.
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

struct HeaderCase {
    name: &'static str,
    source: &'static str,
    expected_error: &'static str,
    expected_stderr: &'static str,
}

const CASES: [HeaderCase; 3] = [
    HeaderCase {
        name: "unknown",
        source: r###"language "other";"###,
        expected_error: r###"{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 16,
  "step": null
}"###,
        expected_stderr: r###"  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".

"###,
    },
    HeaderCase {
        name: "missing",
        source: r###"language ;"###,
        expected_error: r###"{
  "code": "mori::core::syntax",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
  "offset": 9,
  "step": null
}"###,
        expected_stderr: r###"  × mori::command: mori::core::syntax: Expected language "moriarty/1" or
  │ "moriarty/2". Expected a quoted string.

"###,
    },
    HeaderCase {
        name: "commented",
        source: r###"// café
language "other";"###,
        expected_error: r###"{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 25,
  "step": null
}"###,
        expected_stderr: r###"  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".

"###,
    },
];

struct Project {
    root: PathBuf,
    temporary: bool,
}

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (root, temporary) = match std::env::var_os("MORI_INSPECT_HEADER_OBSERVATIONS") {
            Some(path) => {
                let path = PathBuf::from(path);
                assert!(path.is_absolute(), "use an absolute capture directory");
                (path, false)
            }
            None => (
                std::env::temp_dir().join(format!(
                    "mori-inspect-header-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                )),
                true,
            ),
        };
        fs::create_dir(&root).expect("use a fresh capture directory");
        Self { root, temporary }
    }

    fn run(&self, name: &str, source: &str) -> Output {
        let dir = self.root.join(name);
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("header.mori"), source).unwrap();
        let args = [
            "--color",
            "never",
            "inspect",
            "header.mori",
        ];
        let captured = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&dir)
            .args(args)
            .output();
        if let Err(error) = &captured {
            fs::write(
                dir.join("process.json"),
                serde_json::to_vec_pretty(
                    &serde_json::json!({"args": args, "launchError": error.to_string()}),
                )
                .unwrap(),
            )
            .unwrap();
        }
        let output = captured.expect("run actual public source-only inspect");
        // Keep all process bytes/outcomes before any product assertion.
        fs::write(dir.join("stdout.bin"), &output.stdout).unwrap();
        fs::write(dir.join("stderr.txt"), &output.stderr).unwrap();
        fs::write(
            dir.join("process.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "executable": env!("CARGO_BIN_EXE_mori"),
                "args": args, "status": output.status.code(),
            }))
            .unwrap(),
        )
        .unwrap();
        output
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        if self.temporary && !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[test]
fn source_only_inspect_keeps_complete_header_errors() {
    let project = Project::new();
    // Capture each public process stream before assertions, including on failure.
    let captured = CASES
        .iter()
        .map(|case| {
            let error = mori_core::language(case.source)
                .err()
                .map(|error| serde_json::to_value(error).unwrap())
                .unwrap_or(Value::Null);
            let output = project.run(case.name, case.source);
            fs::write(
                project.root.join(case.name).join("producer-error.json"),
                serde_json::to_vec_pretty(&error).unwrap(),
            )
            .unwrap();
            (case, error, output)
        })
        .collect::<Vec<_>>();
    for (case, error, output) in captured {
        let expected: Value = serde_json::from_str(case.expected_error).unwrap();
        assert_eq!(error, expected, "{}: complete producer Error", case.name);
        assert_eq!(output.status.code(), Some(1), "{}", case.name);
        assert!(output.stdout.is_empty(), "{}: complete stdout", case.name);
        assert_eq!(
            output.stderr,
            case.expected_stderr.as_bytes(),
            "{}: complete human stream",
            case.name
        );
    }
}

END FILE actual/AUTHOR-TEST-BEFORE-FORMAT.rs


BEGIN FILE actual/CHECK-PLAN.json | SHA256 133d246b8e8156d1962309a64ec04ffd6a3d94488685bb89e355ca1fb84d2f09 | BYTES 3111
{
  "recordedAt": "2026-10-18T16:53:42.730981+00:00",
  "head": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "tree": "4a78114c06d5fbb858a4bf97b922c0446fabe978",
  "expectationFreezeSha256": "09b4fdf7c97803a01a1bdd82a08fbe4e2cc876554e230a799912b18d7450bcb6",
  "caseSpecificationSha256": "9774c3d75bf2efc1048318466a16a5f452242ad7a500af76acfdb4410d0027e6",
  "sourcePinsBeforeSha256": "0a9044cfb177588ac4213bb1b161a888e31c8b42998beb2edf2c2362a6bf8e80",
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-inspect-header-gi06yedp/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-inspect-header-gi06yedp/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-inspect-header-gi06yedp/tmp"
  },
  "ownedRoot": "/tmp/doctorwatson-inspect-header-gi06yedp",
  "absentAncestorCargoConfigs": [
    "/.cargo/config",
    "/.cargo/config.toml",
    "/home/.cargo/config",
    "/home/.cargo/config.toml",
    "/home/charl/.cargo/config",
    "/home/charl/.cargo/config.toml",
    "/home/charl/Moriarty/.cargo/config",
    "/home/charl/Moriarty/.cargo/config.toml",
    "/home/charl/Moriarty/.worktrees/.cargo/config",
    "/home/charl/Moriarty/.worktrees/.cargo/config.toml",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/.cargo/config",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/.cargo/config.toml",
    "/tmp/.cargo/config",
    "/tmp/.cargo/config.toml",
    "/tmp/doctorwatson-inspect-header-gi06yedp/.cargo/config",
    "/tmp/doctorwatson-inspect-header-gi06yedp/.cargo/config.toml",
    "/tmp/doctorwatson-inspect-header-gi06yedp/cwd/.cargo/config",
    "/tmp/doctorwatson-inspect-header-gi06yedp/cwd/.cargo/config.toml"
  ],
  "toolPins": [
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
      "sha256": "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
      "sha256": "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
      "sha256": "a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"
    },
    {
      "path": "/usr/bin/timeout",
      "sha256": "f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"
    }
  ],
  "scope": "Ordinary source-only inspect header diagnostics RED/GREEN and existing Core1/Core2 capability/stage-plan controls. No SDK/native/proof/key/wallet/ledger execution; no financial/work oracle replacement."
}

END FILE actual/CHECK-PLAN.json


BEGIN FILE actual/COMPLETE-ROOT-DRIVER.py | SHA256 7bfaee974d41167283a238fe2aff950b65be82f513f1d100abb9bef2b2c607fd | BYTES 8012
"""One finite ordinary RED/GREEN cycle; no scheduler or native SDK use."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import shutil
import subprocess
import tempfile
import time

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K = Path('/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018')
P = R / 'concurrency-increase-main238/inspect-header-current615'
C = R / 'inspect-header-dispatch-check-20261018'
HEAD = '615eceee9c6250d2f203cb5807d5c75482dbdb5d'
TREE = '4a78114c06d5fbb858a4bf97b922c0446fabe978'
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
TEST = 'crates/mori/tests/inspect_header_dispatch.rs'
PRODUCTION = 'crates/mori/src/commands/inspect.rs'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(name, value):
    target = C / name
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(value, indent=2) + '\n')


def git(*args):
    return subprocess.check_output(['git', *args], cwd=K).decode().strip()


assert C.is_dir()
assert git('rev-parse', 'HEAD') == HEAD
assert git('rev-parse', 'HEAD^{tree}') == TREE
assert git('status', '--porcelain=v1') == '?? ' + TEST
names = subprocess.check_output([
    'git', 'ls-tree', '-r', '--name-only', HEAD, 'crates',
    'examples/core/continuations', 'Cargo.toml', 'Cargo.lock',
    'rust-toolchain.toml', 'RUST_GUIDELINES.md', 'docs/NATIVE-CORE.md',
], cwd=K).decode().splitlines()
pins = []
for name in names:
    raw = subprocess.check_output(['git', 'show', HEAD + ':' + name], cwd=K)
    assert (K / name).read_bytes() == raw, name
    pins.append({'path': name, 'sha256': hashlib.sha256(raw).hexdigest()})
pins.append({'path': TEST, 'sha256': sha(K / TEST)})
write('SOURCE-PINS-BEFORE.json', {'head': HEAD, 'tree': TREE, 'files': pins})
for name in ['EXPECTATIONS.json', 'EXPECTATIONS.sha256']:
    (C / name).write_bytes((P / name).read_bytes())
assert sha(K / TEST) == sha(C / 'TEST-AFTER-FORMAT.rs')
assert sha(K / PRODUCTION) == sha(P / 'before' / PRODUCTION)

root = Path(tempfile.mkdtemp(prefix='doctorwatson-inspect-header-'))
for name in ['cwd', 'tmp', 'cargo-home']:
    (root / name).mkdir()
for name in ['registry', 'git']:
    (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
env = {
    'PATH': str(TC) + ':/usr/bin:/bin', 'RUSTC': str(TC / 'rustc'),
    'RUSTDOC': str(TC / 'rustdoc'), 'CARGO_HOME': str(root / 'cargo-home'),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8',
    'TMPDIR': str(root / 'tmp'),
}
configs = set()
for directory in [K, *K.parents, root / 'cwd', *(root / 'cwd').parents]:
    for name in ['config', 'config.toml']:
        path = directory / '.cargo' / name
        assert not path.exists(), str(path)
        configs.add(str(path))
write('CHECK-PLAN.json', {
    'recordedAt': now(), 'head': HEAD, 'tree': TREE,
    'expectationFreezeSha256': sha(P / 'EXPECTATIONS.sha256'),
    'caseSpecificationSha256': sha(P / 'EXPECTATIONS.json'),
    'sourcePinsBeforeSha256': sha(C / 'SOURCE-PINS-BEFORE.json'),
    'environment': env, 'ownedRoot': str(root),
    'absentAncestorCargoConfigs': sorted(configs),
    'toolPins': [{'path': str(path), 'sha256': sha(path)} for path in
                 [TC / 'cargo', TC / 'rustc', TC / 'rustdoc', Path('/usr/bin/timeout')]],
    'scope': 'Ordinary source-only inspect header diagnostics RED/GREEN and existing Core1/Core2 capability/stage-plan controls. No SDK/native/proof/key/wallet/ledger execution; no financial/work oracle replacement.',
})


def run(name, args, expected_exit, expected_summary, capture=False):
    output = C / name
    output.mkdir()
    argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s', *args]
    child_env = dict(env)
    if capture:
        child_env['MORI_INSPECT_HEADER_OBSERVATIONS'] = str(output / 'public-cli')
    write(name + '/START.json', {'startedAt': now(), 'argv': argv,
          'cwd': str(root / 'cwd'), 'head': HEAD, 'tree': TREE,
          'observations': child_env.get('MORI_INSPECT_HEADER_OBSERVATIONS')})
    begin = time.monotonic()
    with (output / 'STDOUT.log').open('xb') as stdout, (output / 'STDERR.log').open('xb') as stderr:
        child = subprocess.Popen(argv, cwd=root / 'cwd', env=child_env,
                                 stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        write(name + '/CHILD.json', {'pid': child.pid, 'session': child.pid})
        code = child.wait()
    remaining = [int(pid) for pid, sid in
                 (line.split() for line in subprocess.check_output(['ps', '-eo', 'pid=,sid=']).decode().splitlines())
                 if int(sid) == child.pid]
    result = {'name': name, 'exitCode': code, 'wallSeconds': time.monotonic() - begin,
              'finishedAt': now(), 'stdoutSha256': sha(output / 'STDOUT.log'),
              'stderrSha256': sha(output / 'STDERR.log'), 'ownedSessionRemaining': remaining}
    write(name + '/RESULT.json', result)
    results.append(result)
    print(json.dumps(result), flush=True)
    assert not remaining
    assert code == expected_exit
    assert expected_summary in (output / 'STDOUT.log').read_text()


def cargo(target, *selection):
    return [str(TC / 'cargo'), 'test', '--manifest-path', str(K / 'Cargo.toml'),
            '--frozen', '-j1', '-p', 'mori', '--no-default-features', '--test', target,
            *selection, '--', '--exact', '--test-threads=1']


results = []
try:
    selected = cargo('inspect_header_dispatch', 'source_only_inspect_keeps_complete_header_errors')
    run('red', selected, 101, '0 passed; 1 failed', capture=True)
    assert all(sha(K / pin['path']) == pin['sha256'] for pin in pins)
    before = (K / PRODUCTION).read_bytes()
    after = (P / 'candidate' / PRODUCTION).read_bytes()
    (C / 'INSPECT-BEFORE.rs').write_bytes(before)
    (C / 'INSPECT-AFTER.rs').write_bytes(after)
    (K / PRODUCTION).write_bytes(after)
    write('SOURCE-MUTATION.json', {'path': PRODUCTION, 'beforeSha256': hashlib.sha256(before).hexdigest(),
                                  'afterSha256': sha(K / PRODUCTION),
                                  'scope': 'Propagate existing language() Error; sole production mutation after RED.'})
    for pin in pins:
        if pin['path'] == PRODUCTION:
            pin['sha256'] = sha(K / PRODUCTION)
    write('SOURCE-PINS-AFTER.json', {'head': HEAD, 'tree': TREE, 'files': pins})
    run('green', selected, 0, '1 passed; 0 failed', capture=True)
    run('core1-positive', cargo('capabilities', 'reported_core_profiles_select_real_source_only_cli_consumers'), 0, '1 passed; 0 failed')
    run('core2-positive', cargo('core_continuations', 'check_and_source_inspect_route_core_two_without_legacy_consumers'), 0, '1 passed; 0 failed')
    assert git('rev-parse', 'HEAD') == HEAD
    assert all(sha(K / pin['path']) == pin['sha256'] for pin in pins)
    assert set(git('status', '--porcelain=v1').splitlines()) == {'M ' + PRODUCTION, '?? ' + TEST}
finally:
    shutil.rmtree(root)
    write('FINAL-RESULT.json', {'finishedAt': now(), 'baseHead': HEAD, 'baseTree': TREE,
          'results': results, 'allFourCommandsCompleted': len(results) == 4,
          'ownedRoot': str(root), 'existsAfter': root.exists(),
          'sourcePinsAfterSha256': sha(C / 'SOURCE-PINS-AFTER.json') if (C / 'SOURCE-PINS-AFTER.json').exists() else None,
          'scope': 'RED onefailedaggregate/3rawCLIcaptures; GREEN oneaggregatepasses3exactheaders; twoexistingcapability/Core2inspectpositivecontrolstests retained. No newindependentfinancial/workoracle or full-language/targetacceptance.'})

END FILE actual/COMPLETE-ROOT-DRIVER.py


BEGIN FILE actual/EXPECTATIONS.json | SHA256 9774c3d75bf2efc1048318466a16a5f452242ad7a500af76acfdb4410d0027e6 | BYTES 2446
{
  "status": "FROZEN_BEFORE_ANY_EXECUTION_UNRUN",
  "cases": [
    {
      "name": "unknown",
      "source": "language \"other\";",
      "error": {
        "code": "mori::core::profile",
        "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
        "offset": 16,
        "step": null
      },
      "stderr": "  × mori::command: mori::core::profile: Expected language \"moriarty/1\" or\n  │ \"moriarty/2\"; found \"other\".\n\n",
      "exitCode": 1,
      "stdout": ""
    },
    {
      "name": "missing",
      "source": "language ;",
      "error": {
        "code": "mori::core::syntax",
        "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
        "offset": 9,
        "step": null
      },
      "stderr": "  × mori::command: mori::core::syntax: Expected language \"moriarty/1\" or\n  │ \"moriarty/2\". Expected a quoted string.\n\n",
      "exitCode": 1,
      "stdout": ""
    },
    {
      "name": "commented",
      "source": "// café\nlanguage \"other\";",
      "error": {
        "code": "mori::core::profile",
        "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
        "offset": 25,
        "step": null
      },
      "stderr": "  × mori::command: mori::core::profile: Expected language \"moriarty/1\" or\n  │ \"moriarty/2\"; found \"other\".\n\n",
      "exitCode": 1,
      "stdout": ""
    }
  ],
  "provenance": "Ordinary source/diagnostic expectations. Author inspected current parser::language, Core1 fallback, operation/main/Shell renderer and existing simulate_header_dispatch literals. These are reused source-exposed literals, not independent security/financial/work or actual-output goldens. Full stderr assumes --color never and captured pipe width80, matching existing normal-simulate test and render::WIDTH. Do not regenerate literals from candidate output.",
  "positiveControls": [
    {
      "target": "capabilities",
      "selector": "reported_core_profiles_select_real_source_only_cli_consumers",
      "scope": "Existing valid Core1+Core2 public check/source-inspect reports and preservation, unchanged."
    },
    {
      "target": "core_continuations",
      "selector": "check_and_source_inspect_route_core_two_without_legacy_consumers",
      "scope": "Existing real Core2 Reserve source check+inspect/stage-plan assertions, existing oracle unchanged."
    }
  ]
}

END FILE actual/EXPECTATIONS.json


BEGIN FILE actual/EXPECTATIONS.sha256 | SHA256 09b4fdf7c97803a01a1bdd82a08fbe4e2cc876554e230a799912b18d7450bcb6 | BYTES 1298
9774c3d75bf2efc1048318466a16a5f452242ad7a500af76acfdb4410d0027e6  EXPECTATIONS.json
8b7d3911a422b3ad5c29d74e9d59e46d1313ed4e25effd54612cd4588618c190  expectations/commented/header.mori
aa985c7f05dfde7c25fe99337dfd1812d20dc63353910c1a576b86c9dac207a2  expectations/commented/producer-error.json
bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e  expectations/commented/stderr.txt
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  expectations/commented/stdout.bin
1f535fd2398b3d68b92477d7ad6749d6882da0a7c5f16b2c838fb8e7084b2525  expectations/missing/header.mori
747904e6e1acbd9feceae3708db14da48d8c8806eaefc582d4bb6af9faef97f3  expectations/missing/producer-error.json
ee5e996d685f0f7efdd99d69b52a63edb2503d8624504bc571c8014ccb1a1551  expectations/missing/stderr.txt
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  expectations/missing/stdout.bin
9d14df0aa37e44ff2c8f6e8ae15b5f9e691a1f30c95d42ab4bd28e978e48865b  expectations/unknown/header.mori
31be87d1ab331f2e6ee21e7305e68b50a2a3b5501e3af792dfc4a7fd967c668e  expectations/unknown/producer-error.json
bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e  expectations/unknown/stderr.txt
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  expectations/unknown/stdout.bin

END FILE actual/EXPECTATIONS.sha256


BEGIN FILE actual/FINAL-RESULT.json | SHA256 1dde76f673fb03c55ea0df1523dd9f4cd767bd9d452fcee8b7a0bbbe72e8ac4f | BYTES 2126
{
  "finishedAt": "2026-10-18T16:54:39.287991+00:00",
  "baseHead": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "baseTree": "4a78114c06d5fbb858a4bf97b922c0446fabe978",
  "results": [
    {
      "name": "red",
      "exitCode": 101,
      "wallSeconds": 53.62308415300504,
      "finishedAt": "2026-10-18T16:54:36.406136+00:00",
      "stdoutSha256": "45cc096d7cd37ab3d2975f8cebea192757479b31a7b60eaac48a78a74c0299f9",
      "stderrSha256": "2eb1606cae71067ffa6ce16a7d7f86d5e84cf5eff19cb18749808cc4a79bbd53",
      "ownedSessionRemaining": []
    },
    {
      "name": "green",
      "exitCode": 0,
      "wallSeconds": 1.965872084001603,
      "finishedAt": "2026-10-18T16:54:38.383440+00:00",
      "stdoutSha256": "e5e35dbb31146ffcbb3c7309078d57c7694bbad7fe619e1d6dd986e4b313e39d",
      "stderrSha256": "210ed52342588693838a27bccf642a86ccd80245c8216442b12f965717b2bd9b",
      "ownedSessionRemaining": []
    },
    {
      "name": "core1-positive",
      "exitCode": 0,
      "wallSeconds": 0.3787034790002508,
      "finishedAt": "2026-10-18T16:54:38.763247+00:00",
      "stdoutSha256": "51480449e1b23f6d63834ab4a06de3a6a817a45972ddc887b258f3ebfe54673b",
      "stderrSha256": "50242a9b5f426a7ea30adb772d83549334522b660d751a7f6eaae9df71364fbf",
      "ownedSessionRemaining": []
    },
    {
      "name": "core2-positive",
      "exitCode": 0,
      "wallSeconds": 0.41650749999826076,
      "finishedAt": "2026-10-18T16:54:39.180426+00:00",
      "stdoutSha256": "3bf7b12ef0b93ade3a3ac49db2b72c696f251163847cd46c2d072159025019b1",
      "stderrSha256": "4eb0419e4e6e5dc6ef975d5f12e87ab5eb8e56f12501953d88392744f9641da3",
      "ownedSessionRemaining": []
    }
  ],
  "allFourCommandsCompleted": true,
  "ownedRoot": "/tmp/doctorwatson-inspect-header-gi06yedp",
  "existsAfter": false,
  "sourcePinsAfterSha256": "7caefc46699293788561f653951b4a620526883ab21ab9387a58f502e7b2f990",
  "scope": "RED onefailedaggregate/3rawCLIcaptures; GREEN oneaggregatepasses3exactheaders; twoexistingcapability/Core2inspectpositivecontrolstests retained. No newindependentfinancial/workoracle or full-language/targetacceptance."
}

END FILE actual/FINAL-RESULT.json


BEGIN FILE actual/FORMAT-ONLY-DELTA.json | SHA256 552c542586ef71786a22dc3e0c0e6652e2fadf5fb803abed332cf712293dcfe7 | BYTES 319
{
  "beforeSha256": "38d4b9fd8da39b6b3b2a03f78cd93f20abaa3fb77fb2f264aa5b68b25b969f9b",
  "afterSha256": "f925bcc0ccd78f5a9cc9512f03718830a39475ddd98385531eaff6e90a9b43c3",
  "scope": "Only4argumentarraylinewrapping; allfrozenrawexpectationliteralsunchanged",
  "originalCandidatePreserved": true,
  "rustfmtExit": 0
}

END FILE actual/FORMAT-ONLY-DELTA.json


BEGIN FILE actual/FROZEN-OBSERVATION-COMPARISON.json | SHA256 f0f330a355c41de6cf012233db3111aac086ca751e3e0592c6f8ac9079445631 | BYTES 1817
{
  "rows": [
    {
      "phase": "red",
      "case": "unknown",
      "producerErrorExact": true,
      "exitCode": 1,
      "stdoutEmpty": true,
      "stderrMatchesFrozenRepair": false,
      "stderrSha256": "fa1bc7c97be79fd483f0af60ff91f89dcba675cb3854ea0acee897560157881f"
    },
    {
      "phase": "red",
      "case": "missing",
      "producerErrorExact": true,
      "exitCode": 1,
      "stdoutEmpty": true,
      "stderrMatchesFrozenRepair": false,
      "stderrSha256": "fb2f278f2014913605551ef83ac2f016a3460e13048c21c66f6fc02d14df5e94"
    },
    {
      "phase": "red",
      "case": "commented",
      "producerErrorExact": true,
      "exitCode": 1,
      "stdoutEmpty": true,
      "stderrMatchesFrozenRepair": false,
      "stderrSha256": "fa1bc7c97be79fd483f0af60ff91f89dcba675cb3854ea0acee897560157881f"
    },
    {
      "phase": "green",
      "case": "unknown",
      "producerErrorExact": true,
      "exitCode": 1,
      "stdoutEmpty": true,
      "stderrMatchesFrozenRepair": true,
      "stderrSha256": "bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e"
    },
    {
      "phase": "green",
      "case": "missing",
      "producerErrorExact": true,
      "exitCode": 1,
      "stdoutEmpty": true,
      "stderrMatchesFrozenRepair": true,
      "stderrSha256": "ee5e996d685f0f7efdd99d69b52a63edb2503d8624504bc571c8014ccb1a1551"
    },
    {
      "phase": "green",
      "case": "commented",
      "producerErrorExact": true,
      "exitCode": 1,
      "stdoutEmpty": true,
      "stderrMatchesFrozenRepair": true,
      "stderrSha256": "bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e"
    }
  ],
  "scope": "Source-exposedordinarydiagnostics; all3processescapturedbeforefirstRustassertionfailureinRED; no financial/work/security/target acceptance"
}

END FILE actual/FROZEN-OBSERVATION-COMPARISON.json


BEGIN FILE actual/INSPECT-BEFORE.rs | SHA256 e17139f43eaefc07530a2f9636e0b42fe2410ce8bcef1710f206ebc1bcc4e5c8 | BYTES 21667
#[cfg(feature = "native")]
use std::collections::BTreeSet;
use std::{path::PathBuf, process::ExitCode};

#[cfg(feature = "native")]
use mori_backend::{Artifact, CircuitArtifact, PublicBinding};
#[cfg(feature = "native")]
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type};
use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
    /// Compile a public enforcement map against this exact target descriptor
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    #[cfg(not(feature = "native"))]
    if args.target.is_some() {
        return super::native_unavailable();
    }
    let source = super::core_io::read_source(&args.file)?;
    if args.target.is_none()
        && mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")
    {
        let staged = match mori_core::stage::load(&source) {
            Ok(staged) => staged,
            Err(error) => {
                super::simulate::print_stage_report(json!({"status":"rejected", "error":error}))?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let program = staged.program();
        super::simulate::print_stage_report(json!({
            "status": "accepted", "language": program.language, "semantics": program.semantics,
            "agreement": program.name, "states": program.states, "assets": program.assets,
            "actions": program.actions, "expressions": program.expressions,
            "stage_plan": staged.plan(), "enforcement_report": null
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    #[cfg(feature = "native")]
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        // The enforcement map reports the /3 relation. A Core2 --target refuses
        // with Core's typed profile error before reading the target, backend
        // layout or any output; source-only Core2 inspect is handled above.
        let staged = mori_core::stage::load(&source).map_err(super::core_io::operation)?;
        let refusal = match mori_core::stage::require_core1(staged.program()) {
            Err(error) => error,
            Ok(()) => core1_refusal(),
        };
        return Err(super::core_io::operation(refusal));
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let enforcement = match args.target {
        #[cfg(feature = "native")]
        Some(path) => {
            let target: mori_backend::Target = super::core_io::read_json(&path)?;
            // Use the same whole-program admission and explicit mode as compile.
            // No scenario, key, signature, proof or native execution is consumed.
            let artifact = if target.signed_policy.is_some() {
                mori_backend::compile_signed(&program, &target)
            } else {
                mori_backend::compile(&program, &target)
            }
            .map_err(super::core_io::operation)?;
            enforcement_report(&source, &program, &artifact)?
        }
        #[cfg(not(feature = "native"))]
        Some(_) => return super::native_unavailable(),
        None => Value::Null,
    };
    let qualification = if enforcement.is_null() {
        "source-only-unqualified"
    } else {
        "local-compiled-unqualified"
    };
    // Export validates depth/work/byte bounds and escapes public labels before
    // printing anything. A compiler or report-size refusal leaves stdout empty.
    super::core_io::print(&json!({
        "format": "moriarty-enforcement-report/1",
        "qualification": qualification,
        "language": program.language, "semantics": program.semantics,
        "agreement": program.name, "states": program.states,
        "assets": program.assets, "actions": program.actions,
        "expressions": program.expressions,
        "enforcement_report": enforcement,
        "open_premises": [
            "a supplied exact target is required for the compiled enforcement map",
            "ledger authentication and acceptance",
            "native proof soundness and compiler correspondence",
            "account and sponsor authority"
        ]
    }))?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "native")]
fn encoded(predicates: &[&str]) -> Value {
    json!({"status": "encoded-not-verified", "predicates": predicates})
}
#[cfg(feature = "native")]
fn reference(predicates: &[&str]) -> Value {
    json!({"status": "available-not-executed", "predicates": predicates})
}

#[cfg(feature = "native")]
fn enforcement_report(
    source: &str,
    program: &Program,
    artifact: &Artifact,
) -> super::Result<Value> {
    let actions = program
        .actions
        .iter()
        .zip(&artifact.circuits)
        .map(|(action, circuit)| action_report(program, action, circuit))
        .collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "scope": "source-derived-public-transition-relation",
        "identity": {
            "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
            "program_hash": artifact.program_hash, "semantics": artifact.semantics,
            "artifact_format": artifact.format, "abi": artifact.abi,
            "relation": artifact.relation, "authorization_mode": artifact.authorization_mode,
            "status": "host-computed-identities-not-certificates"
        },
        "target": artifact.target,
        "source_checks": {
            "status": "checked-locally",
            "predicates": [
                "exact language and semantic versions; finite acyclic typed expressions",
                "unique declared names, state initializer types and supported statements",
                "complete distinct asset network/issuer/reference/scale identities; scale 0..38",
                "every asset network equals the supplied target network",
                "exact supported ledger and ZKIR pins; bounded nonempty network and anchor",
                "positive target instruction and artifact limits; actual emitted counts fit",
                "source-wide unique duty identities and closed bounded liability registry"
            ],
            "initial_state": "declaration-only; not an authenticated predecessor or genesis",
            "signed_admission": if artifact.target.signed_policy.is_some() {
                "source-wide owner debits and debtor identity; signed label and asset limits checked"
            } else { "absent-unsigned" }
        },
        "numeric_profile": {
            "UInt": {"min": "0", "max": u128::MAX.to_string(), "representation": "canonical decimal strings; checked unsigned 128-bit integral atomic units"},
            "Bool": "true or false; boolean range constraints",
            "rounding": "exact integral operations only; division and directed rounding unsupported",
            "unit_dimensions": "asset identity and scale retained; generic UInt expressions have no asset dimension checker"
        },
        "resources": artifact.resources,
        "resource_scope": "emitted instruction, ZKIR and artifact-envelope counts; not transaction bytes, measured proving k, DUST or wall-clock replay cost",
        "gates": artifact.gates,
        "actions": actions,
        "unknown": unknown_premises()
    }))
}

#[cfg(feature = "native")]
fn action_report(
    program: &Program,
    action: &Action,
    circuit: &CircuitArtifact,
) -> super::Result<Value> {
    let fields: Vec<_> = circuit
        .public_inputs
        .iter()
        .enumerate()
        .map(|(position, binding)| public_field(position, binding))
        .collect();
    let authorization_fields: Vec<_> = circuit.authorization_bindings.iter().map(|binding| {
        let rule = authorization_rule(&binding.name);
        json!({
            "position": binding.position, "binding": binding,
            "source": "target-selected signed owner-intent profile",
            "reference": reference(&[rule]), "constraint": encoded(&[rule, "public transcript Impact binding"]),
            "state": {"status": "unauthenticated", "predicate": "supplied authority, time and history values require independent authenticated acceptance"},
            "unknown": ["owner-key-authority", "history-replay-revocation", "clock-and-external-inputs", "host-commitment-reconstruction", "native-proof-soundness", "compiler-correspondence"]
        })
    }).collect();
    let statements = action.statements.iter().enumerate().map(|(step, statement)| {
        let rule = statement_rule(statement)?;
        Ok(json!({"step": step, "source": statement, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "state": {"status": "computed-unqualified", "predicate": "ordered local transition on supplied pre-state; no ledger acceptance"},
            "unknown": ["ledger-authentication", "native-proof-soundness", "compiler-correspondence"]}))
    }).collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "action": circuit.action, "context_hash": circuit.context_hash,
        "instruction_count": circuit.instruction_count, "replay_steps": circuit.replay_steps,
        "fields": fields, "authorization_fields": authorization_fields,
        "private_input_descriptors": circuit.private_inputs,
        "authorization_constraints": if authorization_fields.is_empty() {
            json!({"status": "absent-unsigned", "predicates": []})
        } else {
            encoded(&[
                "source-fixed context and typed financial values bound into the owner-intent message",
                "nonidentity typed JubJub key and R; canonical response scalar; FENS signature equation",
                "supplied public key coordinates equal the private typed key coordinates",
                "supplied logical time lies in the signed validity interval; time is outside the signed message",
                "per-action complete owner transfer plus fee gross sum <= gross cap; refunds do not reduce gross",
                "per-action owner fees <= fee cap; beneficiary net credit minus owner fees >= net floor",
                "sum of all closing registered owner duties for each asset <= liability cap"
            ])
        },
        "statements": statements,
        "expression_constraints": expression_constraints(program, action)?,
        "effect_frame": circuit.effect_frame, "balance_frame": circuit.balance_frame,
        "liability_frame": circuit.liability_frame,
        "frame_scope": "all declared states, touched balances and the program-wide source duty registry; preservation of other ledger accounts/duties requires the authenticated adapter"
    }))
}

#[cfg(feature = "native")]
fn public_field(position: usize, binding: &PublicBinding) -> Value {
    let range = match binding.ty {
        Type::UInt => "unsigned 128-bit range",
        Type::Bool => "boolean range",
    };
    let rule = match binding.role.as_str() {
        "context" => "equality to source/target/action context hash limbs computed by the host",
        "input" => "supplied typed action input; no external observation authentication",
        "pre_state" => "supplied typed predecessor state; no equality to the declared initializer",
        "pre_balance" => "supplied opening balance in the touched account/asset frame",
        "pre_liability_presence" => {
            "opening duty presence bit; absent presence forces opening amount zero"
        }
        "pre_liability_amount" => "opening duty amount; absent presence forces opening amount zero",
        "post_state" => "equality to the sequentially computed state or unchanged opening state",
        "post_balance" => "equality to opening balance after ordered checked debits and credits",
        "post_liability_presence" => {
            "equality to computed duty presence; unchanged duties persist; absent amount is zero"
        }
        "post_liability_amount" => {
            "equality to checked duty evolution or unchanged opening amount; absent amount is zero"
        }
        "effect" => "equality to the source statement's computed ordered effect value",
        _ => unreachable!("fresh compiler artifacts use the fixed public ABI roles"),
    };
    let supplied = binding.role.starts_with("pre_") || binding.role == "input";
    let mut unknown = vec!["native-proof-soundness", "compiler-correspondence"];
    if binding.role == "context" {
        unknown.push("host-commitment-reconstruction");
    } else if binding.role == "input" {
        unknown.push("clock-and-external-inputs");
    } else {
        unknown.push("ledger-authentication");
    }
    json!({
        "position": position, "binding": binding,
        "source": "checked declarations, action body and source-fixed public ABI",
        "reference": reference(&[rule]),
        "constraint": encoded(&[range, "public transcript Impact binding", rule]),
        "state": {"status": if supplied { "unauthenticated" } else if binding.role == "context" { "host-reconstructed" } else { "computed-unqualified" },
            "predicate": rule},
        "unknown": unknown
    })
}

#[cfg(feature = "native")]
fn authorization_rule(name: &str) -> &'static str {
    match name {
        "payload.lo" | "payload.hi" => {
            "128-bit host payload hash limb included in the signed message; SHA computation is outside the circuit"
        }
        "public_key.x" | "public_key.y" => {
            "native field coordinate equality to the typed signature key; owner/key authentication is external"
        }
        "message" => {
            "native field equality to the computed owner-intent message and signature challenge"
        }
        "nonce" => {
            "128-bit nonce bound into the signed message; unique ledger consumption is external"
        }
        "not_before" | "not_after" => {
            "128-bit signed validity bound; interval checked against supplied at"
        }
        "predecessor.lo" | "predecessor.hi" => {
            "128-bit predecessor hash limb bound into the signed message; history freshness is external"
        }
        "at" => {
            "128-bit supplied logical time within signed interval; clock authentication is external"
        }
        _ if name.ends_with(".gross_cap") => {
            "128-bit signed gross cap bounds this action's owner transfers plus fees"
        }
        _ if name.ends_with(".fee_cap") => "128-bit signed fee cap bounds this action's owner fees",
        _ if name.ends_with(".net_floor") => {
            "128-bit signed floor bounds this action's beneficiary net credit minus owner fees"
        }
        _ if name.ends_with(".liability_cap") => {
            "128-bit signed cap bounds all closing registered owner duties for this asset"
        }
        _ => unreachable!("fresh signed compiler artifacts use the fixed authorization ABI"),
    }
}

/// Legacy enforcement-map helpers are `/1`-only. An Await reaching them gets
/// Core's typed `/1` profile refusal, never a panic or an omitted statement.
#[cfg(feature = "native")]
fn core1_refusal() -> mori_core::Error {
    mori_core::Error {
        code: "mori::core::profile".into(),
        message: "Await requires moriarty-core/2.".into(),
        offset: None,
        step: None,
    }
}

#[cfg(feature = "native")]
fn statement_rule(statement: &Statement) -> super::Result<&'static str> {
    Ok(match statement {
        Statement::Await { .. } => return Err(super::core_io::operation(core1_refusal())),
        Statement::Let { .. } => {
            "evaluate the expression and bind a typed local name for subsequent statements"
        }
        Statement::Requires { .. } | Statement::Ensures { .. } => {
            "assert the boolean predicate at this ordered statement, after any preceding updates"
        }
        Statement::Next { .. } => {
            "publish the exact typed before/after state values and use the new value in subsequent statements"
        }
        Statement::Emit { .. } => "publish the exact typed computed event value",
        Statement::Transfer { .. } | Statement::Fee { .. } => {
            "checked 128-bit source debit and recipient credit; publish exact amount with source-fixed asset and parties"
        }
        Statement::Liability { .. } => {
            "source-fixed duty identity; checked opening amount plus accrual; closing presence true including present-zero"
        }
        Statement::Discharge { .. } => {
            "require opening duty presence; checked opening amount minus discharge; retain presence even at zero"
        }
    })
}

#[cfg(feature = "native")]
fn expression_constraints(program: &Program, action: &Action) -> super::Result<Vec<Value>> {
    // Report only reachable expression nodes. Some nodes are shared by a
    // statement's DAG; iterating the sorted set is bounded and deterministic.
    let mut pending: Vec<ExprId> = action
        .statements
        .iter()
        .map(|s| match s {
            Statement::Await { .. } => Err(super::core_io::operation(core1_refusal())),
            Statement::Requires { predicate } | Statement::Ensures { predicate } => Ok(*predicate),
            Statement::Let { value, .. }
            | Statement::Next { value, .. }
            | Statement::Emit { value, .. } => Ok(*value),
            Statement::Transfer { amount, .. }
            | Statement::Fee { amount, .. }
            | Statement::Liability { amount, .. }
            | Statement::Discharge { amount, .. } => Ok(*amount),
        })
        .collect::<super::Result<Vec<_>>>()?;
    let mut reachable = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !reachable.insert(id) {
            continue;
        }
        match &program.expressions[id as usize] {
            Expr::Binary { lhs, rhs, .. } => pending.extend([*lhs, *rhs]),
            Expr::Not { value } => pending.push(*value),
            _ => {}
        }
    }
    Ok(reachable.into_iter().map(|id| {
        let node = &program.expressions[id as usize];
        let rule = match node {
            Expr::LiabilityPresent { .. } => "read current duty presence under its source-fixed identity",
            Expr::LiabilityAmount { .. } => "read current duty atomic amount under its source-fixed identity",
            Expr::Literal { .. } => "source-fixed typed literal immediate",
            Expr::Name { .. } => "use the typed binding at this ordered statement",
            Expr::Not { .. } => "boolean negation",
            Expr::Binary { op, .. } => match op {
                BinaryOp::Add => "integer addition with unsigned 128-bit result range; overflow refuses",
                BinaryOp::Sub => "integer subtraction with unsigned 128-bit result range; underflow refuses",
                BinaryOp::Mul => "64-bit limb multiplication with zero high product and unsigned 128-bit result range; overflow refuses",
                BinaryOp::Eq | BinaryOp::Ne => "typed equality or inequality with boolean result",
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => "unsigned 128-bit comparison with boolean result",
                BinaryOp::And | BinaryOp::Or => "boolean conjunction or disjunction with boolean result",
            }
        };
        json!({"expression": id, "source": node, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "unknown": ["native-proof-soundness", "compiler-correspondence"]})
    }).collect())
}

#[cfg(feature = "native")]
fn unknown_premises() -> Vec<Value> {
    [
        ("ledger-authentication", "opening state/balances/duty presence and full ledger frame, custody, funding and phase correspondence"),
        ("history-replay-revocation", "authenticated predecessor, unique nonce/head consumption, revocation and cumulative authority across calls; no recursive history proof"),
        ("clock-and-external-inputs", "supplied logical time and document/oracle/recipient observations are not authenticated by an input or interval check"),
        ("native-proof-soundness", "no proof generation or verification performed; native constraint soundness requires its own evidence"),
        ("compiler-correspondence", "source/reference/constraint and actual ledger correspondence are not established by compilation, hashes or this map"),
        ("fraud-proof-and-replay-cost", "no deployed fraud proof, proving k, transaction-byte or measured replay-time certificate"),
        ("owner-key-authority", "unsigned mode has no signature relation; signed mode encodes a signature equation but owner/account/key authority remains external"),
        ("sponsor-authority-and-settlement", "sponsor grant authentication, DUST reservation/spend and observed financial settlement are not encoded by these source statements"),
        ("live-deployment-and-finality", "no live network/anchor/root-of-trust compatibility, installed verifier/circuit coverage, submission or finality checked"),
        ("host-commitment-reconstruction", "source, target, asset, payload and predecessor SHA/JSON identities are host reconstructions; their hash computation is not proved in-circuit"),
        ("financial-unit-and-rounding-conformance", "asset dimensions, canonical price orientation and directed rounding beyond this exact integral Core profile remain open"),
        ("private-composition", "public transition fields and private signature descriptors do not establish confidentiality or private recursive handoff"),
    ].into_iter().map(|(id, predicate)| json!({"id": id, "status": "unproven", "predicate": predicate})).collect()
}

END FILE actual/INSPECT-BEFORE.rs


BEGIN FILE actual/POST-COMMIT-BINDING.json | SHA256 5e15dd13bb11b09ebc156671fdcb93bff789790421f6d2571380a6100218dcfe | BYTES 992
{
  "head": "4c03df51f78337d1a9e17cfaaabbbea112fef0df",
  "tree": "27b0211b04921485afae58603811549b7b93cb7d",
  "parent": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "sourcePinsAfterSha256": "7caefc46699293788561f653951b4a620526883ab21ab9387a58f502e7b2f990",
  "allRecordedAfterPinsMatchGitAndWorktree": 903,
  "changedPaths": [
    "crates/mori/src/commands/inspect.rs",
    "crates/mori/tests/inspect_header_dispatch.rs"
  ],
  "workingTreeClean": true,
  "selectedGreenTestsPassed": 3,
  "newGreenHeaderPublicCalls": 3,
  "redCapturedHeaderPublicCalls": 3,
  "positiveControls": "Existing capabilities test selects validCore1+Core2 publiccheck/inspect; existingCore2Reservecheck+inspectstageplan test",
  "formatQualification": "OriginalprivatecandidateRustfmtcheckexit1, rootappliedonlyargumentarrayformatbeforeRED/GREEN; rawexpectationliteralsunchanged, before/aftertestandFORMAT-ONLY-DELTA retained",
  "independentSourceResultReview": "pending",
  "nativeProofLedgerTier": "unrun"
}

END FILE actual/POST-COMMIT-BINDING.json


BEGIN FILE actual/SOURCE-MUTATION.json | SHA256 dcf1e30a5074d13045ce83a451c912878b5feb1cf228a02e31620b64ad0aec5e | BYTES 310
{
  "path": "crates/mori/src/commands/inspect.rs",
  "beforeSha256": "e17139f43eaefc07530a2f9636e0b42fe2410ce8bcef1710f206ebc1bcc4e5c8",
  "afterSha256": "0016e768e086f0e6abdd616e4765838df8563392aaf9a7ea708e8e63bb9d8c78",
  "scope": "Propagate existing language() Error; sole production mutation after RED."
}

END FILE actual/SOURCE-MUTATION.json


BEGIN FILE actual/core1-positive/CHILD.json | SHA256 39802773498a6f66ac9853a9267345265dd7df24a6e6be81a6e89c52fd228689 | BYTES 41
{
  "pid": 958222,
  "session": 958222
}

END FILE actual/core1-positive/CHILD.json


BEGIN FILE actual/core1-positive/RESULT.json | SHA256 1eb24766b287774314ff92cf0cefb9e316de116435807cd73c5301f585a8c15a | BYTES 340
{
  "name": "core1-positive",
  "exitCode": 0,
  "wallSeconds": 0.3787034790002508,
  "finishedAt": "2026-10-18T16:54:38.763247+00:00",
  "stdoutSha256": "51480449e1b23f6d63834ab4a06de3a6a817a45972ddc887b258f3ebfe54673b",
  "stderrSha256": "50242a9b5f426a7ea30adb772d83549334522b660d751a7f6eaae9df71364fbf",
  "ownedSessionRemaining": []
}

END FILE actual/core1-positive/RESULT.json


BEGIN FILE actual/core1-positive/START.json | SHA256 544562521e225e198b4f18c718d7cd5059eea4c4831c5d106f6c976b0dce3e9f | BYTES 763
{
  "startedAt": "2026-10-18T16:54:38.384259+00:00",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "capabilities",
    "reported_core_profiles_select_real_source_only_cli_consumers",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-inspect-header-gi06yedp/cwd",
  "head": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "tree": "4a78114c06d5fbb858a4bf97b922c0446fabe978",
  "observations": null
}

END FILE actual/core1-positive/START.json


BEGIN FILE actual/core1-positive/STDERR.log | SHA256 50242a9b5f426a7ea30adb772d83549334522b660d751a7f6eaae9df71364fbf | BYTES 301
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running tests/capabilities.rs (/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/deps/capabilities-eaaf6a0f3fc9fecc)

END FILE actual/core1-positive/STDERR.log


BEGIN FILE actual/core1-positive/STDOUT.log | SHA256 51480449e1b23f6d63834ab4a06de3a6a817a45972ddc887b258f3ebfe54673b | BYTES 185

running 1 test
test reported_core_profiles_select_real_source_only_cli_consumers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s


END FILE actual/core1-positive/STDOUT.log


BEGIN FILE actual/core2-positive/CHILD.json | SHA256 97ef0008a84d0d84db658d74a9c46c890e357e0fac34d34ff3f409f879dd9d10 | BYTES 41
{
  "pid": 958254,
  "session": 958254
}

END FILE actual/core2-positive/CHILD.json


BEGIN FILE actual/core2-positive/RESULT.json | SHA256 edcedb4194520a15efc116a3fdf4ee2502d4142873cea1d5054e67c65b92733d | BYTES 341
{
  "name": "core2-positive",
  "exitCode": 0,
  "wallSeconds": 0.41650749999826076,
  "finishedAt": "2026-10-18T16:54:39.180426+00:00",
  "stdoutSha256": "3bf7b12ef0b93ade3a3ac49db2b72c696f251163847cd46c2d072159025019b1",
  "stderrSha256": "4eb0419e4e6e5dc6ef975d5f12e87ab5eb8e56f12501953d88392744f9641da3",
  "ownedSessionRemaining": []
}

END FILE actual/core2-positive/RESULT.json


BEGIN FILE actual/core2-positive/START.json | SHA256 3c79e03cbf55da05b07fc6d3840d891b6f686ad7280a01f53170ddab585d7493 | BYTES 773
{
  "startedAt": "2026-10-18T16:54:38.763852+00:00",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "core_continuations",
    "check_and_source_inspect_route_core_two_without_legacy_consumers",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-inspect-header-gi06yedp/cwd",
  "head": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "tree": "4a78114c06d5fbb858a4bf97b922c0446fabe978",
  "observations": null
}

END FILE actual/core2-positive/START.json


BEGIN FILE actual/core2-positive/STDERR.log | SHA256 4eb0419e4e6e5dc6ef975d5f12e87ab5eb8e56f12501953d88392744f9641da3 | BYTES 313
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running tests/core_continuations.rs (/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/deps/core_continuations-be03372060c9de90)

END FILE actual/core2-positive/STDERR.log


BEGIN FILE actual/core2-positive/STDOUT.log | SHA256 3bf7b12ef0b93ade3a3ac49db2b72c696f251163847cd46c2d072159025019b1 | BYTES 189

running 1 test
test check_and_source_inspect_route_core_two_without_legacy_consumers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.01s


END FILE actual/core2-positive/STDOUT.log


BEGIN FILE actual/green/CHILD.json | SHA256 bc30f06772b07e2db00a3b2d7957b6d24ba410146673d4ca775ab8483a778d84 | BYTES 41
{
  "pid": 958180,
  "session": 958180
}

END FILE actual/green/CHILD.json


BEGIN FILE actual/green/RESULT.json | SHA256 5c1e88b7f1b65723428b5de6a4222e3bdd91c570238161e483ea6319ce78a285 | BYTES 330
{
  "name": "green",
  "exitCode": 0,
  "wallSeconds": 1.965872084001603,
  "finishedAt": "2026-10-18T16:54:38.383440+00:00",
  "stdoutSha256": "e5e35dbb31146ffcbb3c7309078d57c7694bbad7fe619e1d6dd986e4b313e39d",
  "stderrSha256": "210ed52342588693838a27bccf642a86ccd80245c8216442b12f965717b2bd9b",
  "ownedSessionRemaining": []
}

END FILE actual/green/RESULT.json


BEGIN FILE actual/green/START.json | SHA256 8d7293312e3d1914f60b8ead85bbe5803bc85fae8bba6f71748b6793f253d6c2 | BYTES 864
{
  "startedAt": "2026-10-18T16:54:36.417456+00:00",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "inspect_header_dispatch",
    "source_only_inspect_keeps_complete_header_errors",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-inspect-header-gi06yedp/cwd",
  "head": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "tree": "4a78114c06d5fbb858a4bf97b922c0446fabe978",
  "observations": "/home/charl/moriarty-c2-opus55-restored-20261017/inspect-header-dispatch-check-20261018/green/public-cli"
}

END FILE actual/green/START.json


BEGIN FILE actual/green/STDERR.log | SHA256 210ed52342588693838a27bccf642a86ccd80245c8216442b12f965717b2bd9b | BYTES 323
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
     Running tests/inspect_header_dispatch.rs (/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/deps/inspect_header_dispatch-889c2d13c12c703d)

END FILE actual/green/STDERR.log


BEGIN FILE actual/green/STDOUT.log | SHA256 e5e35dbb31146ffcbb3c7309078d57c7694bbad7fe619e1d6dd986e4b313e39d | BYTES 173

running 1 test
test source_only_inspect_keeps_complete_header_errors ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


END FILE actual/green/STDOUT.log


BEGIN FILE actual/green/public-cli/commented/header.mori | SHA256 8b7d3911a422b3ad5c29d74e9d59e46d1313ed4e25effd54612cd4588618c190 | BYTES 26
// café
language "other";
END FILE actual/green/public-cli/commented/header.mori


BEGIN FILE actual/green/public-cli/commented/process.json | SHA256 f9475abd417861b33f3a6f88b3b618c12c8e93a13fb0b91611f2018eac779712 | BYTES 174
{
  "args": [
    "--color",
    "never",
    "inspect",
    "header.mori"
  ],
  "executable": "/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/mori",
  "status": 1
}
END FILE actual/green/public-cli/commented/process.json


BEGIN FILE actual/green/public-cli/commented/producer-error.json | SHA256 86eb77194ae9241df956aac201ce90446d8fc0898f88fa8cc19a8e594373953f | BYTES 152
{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 25,
  "step": null
}
END FILE actual/green/public-cli/commented/producer-error.json


BEGIN FILE actual/green/public-cli/commented/stderr.txt | SHA256 bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e | BYTES 111
  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".


END FILE actual/green/public-cli/commented/stderr.txt


BEGIN FILE actual/green/public-cli/commented/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/green/public-cli/commented/stdout.bin


BEGIN FILE actual/green/public-cli/missing/header.mori | SHA256 1f535fd2398b3d68b92477d7ad6749d6882da0a7c5f16b2c838fb8e7084b2525 | BYTES 10
language ;
END FILE actual/green/public-cli/missing/header.mori


BEGIN FILE actual/green/public-cli/missing/process.json | SHA256 f9475abd417861b33f3a6f88b3b618c12c8e93a13fb0b91611f2018eac779712 | BYTES 174
{
  "args": [
    "--color",
    "never",
    "inspect",
    "header.mori"
  ],
  "executable": "/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/mori",
  "status": 1
}
END FILE actual/green/public-cli/missing/process.json


BEGIN FILE actual/green/public-cli/missing/producer-error.json | SHA256 e0cc20f0a0de44057c525285796d4511ede821c115d6eb022bea56e228656ba1 | BYTES 159
{
  "code": "mori::core::syntax",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
  "offset": 9,
  "step": null
}
END FILE actual/green/public-cli/missing/producer-error.json


BEGIN FILE actual/green/public-cli/missing/stderr.txt | SHA256 ee5e996d685f0f7efdd99d69b52a63edb2503d8624504bc571c8014ccb1a1551 | BYTES 121
  × mori::command: mori::core::syntax: Expected language "moriarty/1" or
  │ "moriarty/2". Expected a quoted string.


END FILE actual/green/public-cli/missing/stderr.txt


BEGIN FILE actual/green/public-cli/missing/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/green/public-cli/missing/stdout.bin


BEGIN FILE actual/green/public-cli/unknown/header.mori | SHA256 9d14df0aa37e44ff2c8f6e8ae15b5f9e691a1f30c95d42ab4bd28e978e48865b | BYTES 17
language "other";
END FILE actual/green/public-cli/unknown/header.mori


BEGIN FILE actual/green/public-cli/unknown/process.json | SHA256 f9475abd417861b33f3a6f88b3b618c12c8e93a13fb0b91611f2018eac779712 | BYTES 174
{
  "args": [
    "--color",
    "never",
    "inspect",
    "header.mori"
  ],
  "executable": "/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/mori",
  "status": 1
}
END FILE actual/green/public-cli/unknown/process.json


BEGIN FILE actual/green/public-cli/unknown/producer-error.json | SHA256 4c0a6a11d2578e5f9a15c5cbb64f5f38943f4159ffbd2f7ff786767cfdabb9e1 | BYTES 152
{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 16,
  "step": null
}
END FILE actual/green/public-cli/unknown/producer-error.json


BEGIN FILE actual/green/public-cli/unknown/stderr.txt | SHA256 bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e | BYTES 111
  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".


END FILE actual/green/public-cli/unknown/stderr.txt


BEGIN FILE actual/green/public-cli/unknown/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/green/public-cli/unknown/stdout.bin


BEGIN FILE actual/red/CHILD.json | SHA256 56574f3b6587954aff566222f669ae05f4acad7785d89ea66c55734290e70045 | BYTES 41
{
  "pid": 955563,
  "session": 955563
}

END FILE actual/red/CHILD.json


BEGIN FILE actual/red/RESULT.json | SHA256 8172905be3ef9411402b9d5173ab474af962fa36da52d5d3893ce4fe7c304da9 | BYTES 330
{
  "name": "red",
  "exitCode": 101,
  "wallSeconds": 53.62308415300504,
  "finishedAt": "2026-10-18T16:54:36.406136+00:00",
  "stdoutSha256": "45cc096d7cd37ab3d2975f8cebea192757479b31a7b60eaac48a78a74c0299f9",
  "stderrSha256": "2eb1606cae71067ffa6ce16a7d7f86d5e84cf5eff19cb18749808cc4a79bbd53",
  "ownedSessionRemaining": []
}

END FILE actual/red/RESULT.json


BEGIN FILE actual/red/START.json | SHA256 577b0fb1501cb038ff37af9de9b089521b555d0c21ddcc09cae4b75496ca6321 | BYTES 862
{
  "startedAt": "2026-10-18T16:53:42.782963+00:00",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "inspect_header_dispatch",
    "source_only_inspect_keeps_complete_header_errors",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-inspect-header-gi06yedp/cwd",
  "head": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "tree": "4a78114c06d5fbb858a4bf97b922c0446fabe978",
  "observations": "/home/charl/moriarty-c2-opus55-restored-20261017/inspect-header-dispatch-check-20261018/red/public-cli"
}

END FILE actual/red/START.json


BEGIN FILE actual/red/STDERR.log | SHA256 2eb1606cae71067ffa6ce16a7d7f86d5e84cf5eff19cb18749808cc4a79bbd53 | BYTES 5134
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling stable_deref_trait v1.2.1
   Compiling typenum v1.20.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling writeable v0.6.4
   Compiling libc v0.2.190
   Compiling version_check v0.9.5
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling litemap v0.8.3
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling subtle v2.6.1
   Compiling linux-raw-sys v0.12.1
   Compiling const-oid v0.9.6
   Compiling icu_segmenter_data v2.3.0
   Compiling bitflags v2.13.2
   Compiling icu_provider v2.3.1
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling icu_locale_fallback v2.3.0
   Compiling smallvec v1.16.2
   Compiling icu_segmenter v2.3.0
   Compiling unicode-width v0.2.2
   Compiling is_ci v1.2.0
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.4
   Compiling terminal_size v0.4.4
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling getrandom v0.2.17
   Compiling supports-unicode v3.0.0
   Compiling supports-hyperlinks v3.2.0
   Compiling unicode-width v0.1.14
   Compiling owo-colors v4.4.0
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-span)
   Compiling der v0.7.10
   Compiling memchr v2.8.3
   Compiling crossbeam-utils v0.8.23
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling crossbeam-deque v0.8.8
   Compiling base16ct v0.2.0
   Compiling serde_core v1.0.229
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling utf8parse v0.2.2
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling serde_json v1.0.151
   Compiling rayon-core v1.13.0
   Compiling colorchoice v1.0.5
   Compiling serde v1.0.229
   Compiling anstyle-query v1.1.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling getrandom v0.4.3
   Compiling const-oid v0.10.2
   Compiling thiserror v2.0.21
   Compiling cpufeatures v0.2.17
   Compiling regex-syntax v0.8.11
   Compiling anstyle v1.0.14
   Compiling anstream v1.0.0
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling same-file v1.0.6
   Compiling strsim v0.11.1
   Compiling itoa v1.0.18
   Compiling similar v2.7.0
   Compiling log v0.4.34
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling cpufeatures v0.3.1
   Compiling sha2 v0.11.0
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling globset v0.4.20
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-parser)
   Compiling either v1.19.0
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling rayon v1.12.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 53.59s
     Running tests/inspect_header_dispatch.rs (/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/deps/inspect_header_dispatch-889c2d13c12c703d)
error: test failed, to rerun pass `-p mori --test inspect_header_dispatch`

END FILE actual/red/STDERR.log


BEGIN FILE actual/red/STDOUT.log | SHA256 45cc096d7cd37ab3d2975f8cebea192757479b31a7b60eaac48a78a74c0299f9 | BYTES 1564

running 1 test
test source_only_inspect_keeps_complete_header_errors ... FAILED

failures:

---- source_only_inspect_keeps_complete_header_errors stdout ----

thread 'source_only_inspect_keeps_complete_header_errors' (958175) panicked at crates/mori/tests/inspect_header_dispatch.rs:159:9:
assertion `left == right` failed: unknown: complete human stream
  left: [32, 32, 195, 151, 32, 109, 111, 114, 105, 58, 58, 99, 111, 109, 109, 97, 110, 100, 58, 32, 109, 111, 114, 105, 58, 58, 99, 111, 114, 101, 58, 58, 112, 114, 111, 102, 105, 108, 101, 58, 32, 69, 120, 112, 101, 99, 116, 101, 100, 32, 108, 97, 110, 103, 117, 97, 103, 101, 32, 34, 109, 111, 114, 105, 97, 114, 116, 121, 47, 49, 34, 59, 32, 102, 111, 117, 110, 100, 10, 32, 32, 226, 148, 130, 32, 34, 111, 116, 104, 101, 114, 34, 46, 10, 10]
 right: [32, 32, 195, 151, 32, 109, 111, 114, 105, 58, 58, 99, 111, 109, 109, 97, 110, 100, 58, 32, 109, 111, 114, 105, 58, 58, 99, 111, 114, 101, 58, 58, 112, 114, 111, 102, 105, 108, 101, 58, 32, 69, 120, 112, 101, 99, 116, 101, 100, 32, 108, 97, 110, 103, 117, 97, 103, 101, 32, 34, 109, 111, 114, 105, 97, 114, 116, 121, 47, 49, 34, 32, 111, 114, 10, 32, 32, 226, 148, 130, 32, 34, 109, 111, 114, 105, 97, 114, 116, 121, 47, 50, 34, 59, 32, 102, 111, 117, 110, 100, 32, 34, 111, 116, 104, 101, 114, 34, 46, 10, 10]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    source_only_inspect_keeps_complete_header_errors

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


END FILE actual/red/STDOUT.log


BEGIN FILE actual/red/public-cli/commented/header.mori | SHA256 8b7d3911a422b3ad5c29d74e9d59e46d1313ed4e25effd54612cd4588618c190 | BYTES 26
// café
language "other";
END FILE actual/red/public-cli/commented/header.mori


BEGIN FILE actual/red/public-cli/commented/process.json | SHA256 f9475abd417861b33f3a6f88b3b618c12c8e93a13fb0b91611f2018eac779712 | BYTES 174
{
  "args": [
    "--color",
    "never",
    "inspect",
    "header.mori"
  ],
  "executable": "/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/mori",
  "status": 1
}
END FILE actual/red/public-cli/commented/process.json


BEGIN FILE actual/red/public-cli/commented/producer-error.json | SHA256 86eb77194ae9241df956aac201ce90446d8fc0898f88fa8cc19a8e594373953f | BYTES 152
{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 25,
  "step": null
}
END FILE actual/red/public-cli/commented/producer-error.json


BEGIN FILE actual/red/public-cli/commented/stderr.txt | SHA256 fa1bc7c97be79fd483f0af60ff91f89dcba675cb3854ea0acee897560157881f | BYTES 95
  × mori::command: mori::core::profile: Expected language "moriarty/1"; found
  │ "other".


END FILE actual/red/public-cli/commented/stderr.txt


BEGIN FILE actual/red/public-cli/commented/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/red/public-cli/commented/stdout.bin


BEGIN FILE actual/red/public-cli/missing/header.mori | SHA256 1f535fd2398b3d68b92477d7ad6749d6882da0a7c5f16b2c838fb8e7084b2525 | BYTES 10
language ;
END FILE actual/red/public-cli/missing/header.mori


BEGIN FILE actual/red/public-cli/missing/process.json | SHA256 f9475abd417861b33f3a6f88b3b618c12c8e93a13fb0b91611f2018eac779712 | BYTES 174
{
  "args": [
    "--color",
    "never",
    "inspect",
    "header.mori"
  ],
  "executable": "/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/mori",
  "status": 1
}
END FILE actual/red/public-cli/missing/process.json


BEGIN FILE actual/red/public-cli/missing/producer-error.json | SHA256 e0cc20f0a0de44057c525285796d4511ede821c115d6eb022bea56e228656ba1 | BYTES 159
{
  "code": "mori::core::syntax",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
  "offset": 9,
  "step": null
}
END FILE actual/red/public-cli/missing/producer-error.json


BEGIN FILE actual/red/public-cli/missing/stderr.txt | SHA256 fb2f278f2014913605551ef83ac2f016a3460e13048c21c66f6fc02d14df5e94 | BYTES 67
  × mori::command: mori::core::syntax: Expected a quoted string.


END FILE actual/red/public-cli/missing/stderr.txt


BEGIN FILE actual/red/public-cli/missing/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/red/public-cli/missing/stdout.bin


BEGIN FILE actual/red/public-cli/unknown/header.mori | SHA256 9d14df0aa37e44ff2c8f6e8ae15b5f9e691a1f30c95d42ab4bd28e978e48865b | BYTES 17
language "other";
END FILE actual/red/public-cli/unknown/header.mori


BEGIN FILE actual/red/public-cli/unknown/process.json | SHA256 f9475abd417861b33f3a6f88b3b618c12c8e93a13fb0b91611f2018eac779712 | BYTES 174
{
  "args": [
    "--color",
    "never",
    "inspect",
    "header.mori"
  ],
  "executable": "/tmp/doctorwatson-inspect-header-gi06yedp/target/debug/mori",
  "status": 1
}
END FILE actual/red/public-cli/unknown/process.json


BEGIN FILE actual/red/public-cli/unknown/producer-error.json | SHA256 4c0a6a11d2578e5f9a15c5cbb64f5f38943f4159ffbd2f7ff786767cfdabb9e1 | BYTES 152
{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 16,
  "step": null
}
END FILE actual/red/public-cli/unknown/producer-error.json


BEGIN FILE actual/red/public-cli/unknown/stderr.txt | SHA256 fa1bc7c97be79fd483f0af60ff91f89dcba675cb3854ea0acee897560157881f | BYTES 95
  × mori::command: mori::core::profile: Expected language "moriarty/1"; found
  │ "other".


END FILE actual/red/public-cli/unknown/stderr.txt


BEGIN FILE actual/red/public-cli/unknown/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/red/public-cli/unknown/stdout.bin


BEGIN FILE original-author/README.md | SHA256 cbfc8b2c9c971ae1e1fed5eba79dbaeef945f013869353797c841d96d6e34687 | BYTES 1497
Source-only inspect header error propagation, base615

CANDIDATE.patch changes exactly one source-only expression in inspect.rs and adds one real public-process regression. Source-only inspect currently drops language() Err, falls into the /1 parser and replaces the established dual-profile diagnostic. Preserve the producer error through existing operation/main/Shell rendering. No new JSON shape or helper: inspect has no --json flag. Valid Core1/Core2 report paths are unchanged; full target/native branch and helpers stay byte-identical.

EXPECTATIONS.json and12physical source/error/full stdout/full stderr inputs are frozen before execution (EXPECTATIONS.sha256). Literals are explicitly source-exposed from parser and existing ordinary simulate regression, not independent financial/work expectations or actual inspect captures. New test captures all3public inspect processes before assertions, including on RED. Existing capabilities/Core2 continuation selectors give valid-profile controls; expectations are unchanged.

Full production before/after, full new test, exact source pins and direct-context files are physical. REGRESSION-PLAN lists3ordinary exact SDK-free test argv, all UNRUN by author. Root alone integrates, executes and reviews. No provider/Git/shared-worktree/product/native/SDK/proof/wallet operation occurred; no security judgment. All original grammar/family/financial/work and U/M/MC/SP/G exits retained. Do not regenerate frozen expectations from candidate output.

END FILE original-author/README.md


BEGIN FILE original-author/FINDING.json | SHA256 5470c0d2f9c5697b50464e902912d475fc8e743d83832a96a25a1bc3e8cd9552 | BYTES 1639
{
  "status": "REAL_ORDINARY_SOURCE_DEFECT_SOURCE_CONFIRMED_RUNTIME_UNRUN",
  "at": "inspect.rs28-30 source-only branch; Core1 fallback at59",
  "before": "language Err is converted tofalse by is_ok_and; then parse(/1) emits narrower Expected language moriarty/1 or underlying syntax message, losing existing language() dual-profile prefix.",
  "after": "Within args.target.is_none() short-circuit, preserve original language Error via existing core_io::operation and main/Shell failure renderer. /2 success still uses same stage loader/report; /1 success still same parse/report.",
  "argsAndJSON": "inspect Args has file and optional --target, no --json flag. Source-only accepted reports are JSON; a recognized /2 staged-load error keeps existing rejected JSON. Invalid header failure already used operation stderr; preserve that channel and empty stdout rather than invent a report DTO.",
  "unchangedTarget": "The entire native /2 --target branch and downstream target/report helpers remain byteidentical; no target/native profile expansion or decision. Non-native --target still returns native_unavailable before source I/O.",
  "knownLimits": "Native-target header is_ok_and remains present and outside this source-only repair; no judgment or execution on that path. No shared router/general profile semantics changes.",
  "acceptance": "UNAPPLIED/UNRUN; source proof only, frozen exposed ordinary expectations, independent source/result review still required.",
  "preserve": "All27grammar/148requirements/292scenarios/20families/fullU0-U7/M1-M17/MC/SP/G/finance/composition/native/target/Artifact/security ownership unchanged."
}

END FILE original-author/FINDING.json


BEGIN FILE original-author/REGRESSION-PLAN.json | SHA256 40a54e884b0ec7b9258e84af2e5bc1a8a73cd6a013d4b8fad99228aca08367dd | BYTES 2575
{
  "status": "SOURCE_ONLY_UNRUN",
  "commands": [
    {
      "argv": [
        "cargo",
        "test",
        "--frozen",
        "-j1",
        "-p",
        "mori",
        "--no-default-features",
        "--test",
        "inspect_header_dispatch",
        "source_only_inspect_keeps_complete_header_errors",
        "--",
        "--exact",
        "--test-threads=1"
      ],
      "status": "INSPECTED_UNRUN",
      "scope": "Source-only public process; no --target, native, trace, SDK/proof/key/wallet/ledger feature/action.",
      "expectedTestCount": 1
    },
    {
      "argv": [
        "cargo",
        "test",
        "--frozen",
        "-j1",
        "-p",
        "mori",
        "--no-default-features",
        "--test",
        "capabilities",
        "reported_core_profiles_select_real_source_only_cli_consumers",
        "--",
        "--exact",
        "--test-threads=1"
      ],
      "status": "INSPECTED_UNRUN",
      "scope": "Source-only public process; no --target, native, trace, SDK/proof/key/wallet/ledger feature/action.",
      "expectedTestCount": 1
    },
    {
      "argv": [
        "cargo",
        "test",
        "--frozen",
        "-j1",
        "-p",
        "mori",
        "--no-default-features",
        "--test",
        "core_continuations",
        "check_and_source_inspect_route_core_two_without_legacy_consumers",
        "--",
        "--exact",
        "--test-threads=1"
      ],
      "status": "INSPECTED_UNRUN",
      "scope": "Source-only public process; no --target, native, trace, SDK/proof/key/wallet/ledger feature/action.",
      "expectedTestCount": 1
    }
  ],
  "redGreen": "Root may apply only new regression on base615 for RED and then one-expression source fix for GREEN under its own existing execution plan; preserve all captures. No such execution by this author.",
  "captureEnv": "MORI_INSPECT_HEADER_OBSERVATIONS: optional root-owned absolute directory that must be absent before test; when set, process captures retained even on success. Without it, successful owned temp directory removed; panic leaves diagnostics. No global cleanup.",
  "assertionScope": "3complete producer errors+status1+empty entire stdout+literal entire stderr; existing separate valid Core1/Core2 controls unchanged. No new financial/work oracle.",
  "sideEffectsIfAuthorized": "Cargo ordinary build outputs; test fresh-owned temp or root capture directory with3source files, process/stdout/stderr/producer JSON; subprocesses are actual mori inspect --color never source-only. No external tools via product."
}

END FILE original-author/REGRESSION-PLAN.json


BEGIN FILE original-author/STATIC-VALIDATION.json | SHA256 c3eb4b416bd9cfb7d518064c8aeb2a0f3393ad9c082b38d74ee53284111edf55 | BYTES 413
{
  "status": "PASS_SOURCE_TRANSPORT_ONLY",
  "oneProductionExpressionChanged": true,
  "targetAndAllDownstreamHelpersVerbatim": true,
  "newRealPublicProcessTestCases": 3,
  "frozenStreams": {
    "stdout": "empty bytes all3",
    "stderr": "complete existing human diagnostic literals"
  },
  "existingBothProfileAndCore2Controls": true,
  "runtimeExecutions": 0,
  "providerCalls": 0,
  "GitSharedWrites": 0
}

END FILE original-author/STATIC-VALIDATION.json


BEGIN FILE original-author/expectations/commented/header.mori | SHA256 8b7d3911a422b3ad5c29d74e9d59e46d1313ed4e25effd54612cd4588618c190 | BYTES 26
// café
language "other";
END FILE original-author/expectations/commented/header.mori


BEGIN FILE original-author/expectations/commented/producer-error.json | SHA256 aa985c7f05dfde7c25fe99337dfd1812d20dc63353910c1a576b86c9dac207a2 | BYTES 153
{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 25,
  "step": null
}

END FILE original-author/expectations/commented/producer-error.json


BEGIN FILE original-author/expectations/commented/stderr.txt | SHA256 bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e | BYTES 111
  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".


END FILE original-author/expectations/commented/stderr.txt


BEGIN FILE original-author/expectations/commented/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE original-author/expectations/commented/stdout.bin


BEGIN FILE original-author/expectations/missing/header.mori | SHA256 1f535fd2398b3d68b92477d7ad6749d6882da0a7c5f16b2c838fb8e7084b2525 | BYTES 10
language ;
END FILE original-author/expectations/missing/header.mori


BEGIN FILE original-author/expectations/missing/producer-error.json | SHA256 747904e6e1acbd9feceae3708db14da48d8c8806eaefc582d4bb6af9faef97f3 | BYTES 160
{
  "code": "mori::core::syntax",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
  "offset": 9,
  "step": null
}

END FILE original-author/expectations/missing/producer-error.json


BEGIN FILE original-author/expectations/missing/stderr.txt | SHA256 ee5e996d685f0f7efdd99d69b52a63edb2503d8624504bc571c8014ccb1a1551 | BYTES 121
  × mori::command: mori::core::syntax: Expected language "moriarty/1" or
  │ "moriarty/2". Expected a quoted string.


END FILE original-author/expectations/missing/stderr.txt


BEGIN FILE original-author/expectations/missing/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE original-author/expectations/missing/stdout.bin


BEGIN FILE original-author/expectations/unknown/header.mori | SHA256 9d14df0aa37e44ff2c8f6e8ae15b5f9e691a1f30c95d42ab4bd28e978e48865b | BYTES 17
language "other";
END FILE original-author/expectations/unknown/header.mori


BEGIN FILE original-author/expectations/unknown/producer-error.json | SHA256 31be87d1ab331f2e6ee21e7305e68b50a2a3b5501e3af792dfc4a7fd967c668e | BYTES 153
{
  "code": "mori::core::profile",
  "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
  "offset": 16,
  "step": null
}

END FILE original-author/expectations/unknown/producer-error.json


BEGIN FILE original-author/expectations/unknown/stderr.txt | SHA256 bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e | BYTES 111
  × mori::command: mori::core::profile: Expected language "moriarty/1" or
  │ "moriarty/2"; found "other".


END FILE original-author/expectations/unknown/stderr.txt


BEGIN FILE original-author/expectations/unknown/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE original-author/expectations/unknown/stdout.bin


BEGIN FILE source/crates/mori/src/main.rs | SHA256 35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232 | BYTES 495
use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod commands;
mod display;
mod error;
mod json;
mod shell;
mod styles;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut shell = Shell::new(cli.global.verbosity(), cli.global.color.into());

    match commands::run(cli.command, &mut shell) {
        Ok(code) => code,
        Err(report) => {
            shell.report(&report);
            ExitCode::FAILURE
        }
    }
}

END FILE source/crates/mori/src/main.rs


BEGIN FILE source/crates/mori/src/commands/mod.rs | SHA256 4e39c2dd984c1dcd0687da30eddd4e630f3d9480aef0b5df25e48828001abc47 | BYTES 2838
//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
pub mod capabilities;
pub mod check;
mod check_source;
pub mod compile;
pub mod compile_call;
mod core_io;
pub mod expand;
pub mod format;
pub mod init;
pub mod inspect;
pub mod intent;
pub mod keygen_call;
pub mod keygen_signed_call;
pub mod lsp;
pub mod mcp;
pub mod prepare_call;
pub mod prepare_signed_call;
mod proof_io;
pub mod prove;
pub mod prove_call;
pub mod prove_signed_call;
pub mod replay;
mod signed_call_io;
pub mod simulate;
pub mod test;
pub mod verify;
pub mod verify_call;
pub mod verify_intent;
pub mod verify_signed_call;

pub type Result<T> = std::result::Result<T, Report>;

pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Capabilities(args) => capabilities::run(args, shell),
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
        Command::KeygenSignedCall(args) => keygen_signed_call::run(args, shell),
        Command::PrepareSignedCall(args) => prepare_signed_call::run(args, shell),
        Command::ProveSignedCall(args) => prove_signed_call::run(args, shell),
        Command::VerifySignedCall(args) => verify_signed_call::run(args, shell),
        Command::KeygenCall(args) => keygen_call::run(args, shell),
        Command::ProveCall(args) => prove_call::run(args, shell),
        Command::VerifyCall(args) => verify_call::run(args, shell),
        Command::CompileCall(args) => compile_call::run(args, shell),
        Command::PrepareCall(args) => prepare_call::run(args, shell),
        Command::Prove(args) => prove::run(args, shell),
        Command::Verify(args) => verify::run(args, shell),
        Command::Format(args) => format::run(args, shell),
        Command::Inspect(args) => inspect::run(args, shell),
        Command::Expand(args) => expand::run(args, shell),
        Command::Simulate(args) => simulate::run(args, shell),
        Command::Replay(args) => replay::run(args, shell),
        Command::Test(args) => test::run(args, shell),
        Command::Intent(args) => intent::run(args, shell),
        Command::VerifyIntent(args) => verify_intent::run(args, shell),
        Command::Lsp(args) => lsp::run(args, shell),
        Command::Mcp(args) => mcp::run(args, shell),
    }
}

#[cfg(not(feature = "native"))]
fn native_unavailable<T>() -> Result<T> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::native-unavailable".into(),
        message: "This mori build has no Midnight support. Rebuild with the native feature enabled."
            .into(),
    }
    .into())
}

END FILE source/crates/mori/src/commands/mod.rs


BEGIN FILE source/crates/mori/src/shell.rs | SHA256 b2116fcd9bb80abfb175f6dc2da6327b4d9787cfa3d56481fde705e62d94fff1 | BYTES 3802
//! Cargo-style status output on stderr.

use std::fmt::Display;
use std::io::{self, Stderr, Write};

use anstream::{AutoStream, ColorChoice};
use anstyle::Style;
use mori_diagnostics::Report;
use mori_diagnostics::render::{self, WIDTH};

use crate::styles;

/// Width of the right-aligned verb column, matching cargo.
const VERB_WIDTH: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// Writes status lines such as `    Checking invoice.mori` to stderr.
///
/// Command results go to stdout and never pass through the shell. Output is
/// best effort: if stderr cannot be written, there is nowhere to report it.
pub struct Shell {
    err: AutoStream<Stderr>,
    verbosity: Verbosity,
}

impl Shell {
    pub fn new(verbosity: Verbosity, color: ColorChoice) -> Self {
        Self {
            err: AutoStream::new(io::stderr(), color),
            verbosity,
        }
    }

    /// Prints a right-aligned green verb followed by a message.
    pub fn status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    /// Like [`Shell::status`], but only under `--verbose`.
    pub fn verbose_status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity == Verbosity::Verbose {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command prints notes")]
    pub fn note(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("note", styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command emits warnings")]
    pub fn warn(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("warning", styles::WARN, message);
        }
    }

    /// Prints an error line. Errors are shown even under `--quiet`.
    pub fn error(&mut self, message: impl Display) {
        self.prefixed("error", styles::ERROR, message);
    }

    /// Renders an error report, wrapped to the terminal width up to 80 columns.
    /// Errors are shown even under `--quiet`.
    pub fn report(&mut self, report: &Report) {
        let width = terminal_size::terminal_size_of(io::stderr())
            .map_or(WIDTH, |(width, _)| usize::from(width.0));
        let _ = writeln!(self.err, "{}", render::render_colored(report, width));
    }

    /// Prints `hunks` as `cargo fmt --check` does, indented to line up with a
    /// diagnostic's message.
    pub fn diff(&mut self, file: &str, hunks: &[mori_fmt::Hunk]) {
        const INDENT: &str = "    ";
        for hunk in hunks {
            let _ = writeln!(self.err, "{INDENT}Diff in {file}:{}:", hunk.line);
            for (change, line) in &hunk.lines {
                let (sign, style) = match change {
                    mori_fmt::Change::Context => (' ', Style::new()),
                    mori_fmt::Change::Removed => ('-', styles::REMOVED),
                    mori_fmt::Change::Added => ('+', styles::ADDED),
                };
                let _ = writeln!(self.err, "{INDENT}{style}{sign}{line}{style:#}");
            }
        }
        let _ = writeln!(self.err);
    }

    fn aligned(&mut self, verb: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{verb:>VERB_WIDTH$}{style:#} {message}");
    }

    fn prefixed(&mut self, label: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{label}:{style:#} {message}");
    }
}

END FILE source/crates/mori/src/shell.rs


BEGIN FILE source/crates/mori/src/error.rs | SHA256 9c1f5a1207cfaf2509e159cdeed344a60b172e196988d021acab6cc9a45b918c | BYTES 946
//! Command failures that do not point at source code.

use std::io;
use std::path::PathBuf;

use mori_diagnostics::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{code}: {message}")]
    Operation { code: String, message: String },
    #[error("could not read `{}`", crate::display::text(&path.display().to_string()))]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not write `{}`", crate::display::text(&path.display().to_string()))]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` does not exist", crate::display::text(&path.display().to_string()))]
    NotFound { path: PathBuf },
    #[error("could not list `{}`", crate::display::text(&path.display().to_string()))]
    Walk {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
}

impl Diagnostic for CliError {}

END FILE source/crates/mori/src/error.rs


BEGIN FILE source/crates/mori/src/display.rs | SHA256 81d499d32561dc576651e0b1df5a349b68112030f296d93cca95a488dc3b0deb | BYTES 408
/// Escape data before a terminal renderer adds its trusted styles and layout.
pub fn text(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for character in value.chars() {
        if character.is_ascii() && !character.is_control() {
            result.push(character);
        } else {
            result.extend(character.escape_default());
        }
    }
    result
}

END FILE source/crates/mori/src/display.rs


BEGIN FILE source/crates/mori/src/commands/check.rs | SHA256 2f08c04bdc1a07796c459385ec9271466a26572f2455ed7d63de906c1d5876bb | BYTES 3744
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

use super::check_source::{self, SourceCheckConsumer};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    let outcome = check_source::dispatch(
        &source,
        CliConsumer {
            name: &name,
            source: &source,
            json: args.json,
            shell,
        },
    )?;
    if outcome.beta {
        if let Some(report) = outcome.report {
            super::core_io::print(&report)?;
        } else if outcome.accepted {
            shell.status(
                "Finished",
                format!("check in {:.2}s", started.elapsed().as_secs_f64()),
            );
        }
    }
    Ok(if outcome.accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

struct CliOutcome {
    accepted: bool,
    beta: bool,
    report: Option<CheckReport>,
}

struct CliConsumer<'a> {
    name: &'a str,
    source: &'a str,
    json: bool,
    shell: &'a mut Shell,
}

impl SourceCheckConsumer for CliConsumer<'_> {
    type Output = super::Result<CliOutcome>;

    fn parsed(&mut self, tokens: usize, nodes: usize) {
        if !self.json {
            self.shell
                .verbose_status("Parsed", format!("{tokens} tokens, {nodes} nodes"));
        }
    }

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        let report = self.json.then(|| {
            check_source::beta_report(self.name.to_owned(), self.source, checked, &diagnostics)
        });
        let accepted = finish(self.shell, self.name, self.source, diagnostics, self.json);
        Ok(CliOutcome {
            accepted,
            beta: true,
            report,
        })
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        if self.json {
            let report = check_source::core_report(self.name, self.source, &result, staged);
            super::core_io::print(&report)?;
        } else {
            match &result {
                Ok(_) => self.shell.status("Finished", "bounded core check"),
                Err(error) => self.shell.error(error),
            }
        }
        Ok(CliOutcome {
            accepted: result.is_ok(),
            beta: false,
            report: None,
        })
    }
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}

END FILE source/crates/mori/src/commands/check.rs


BEGIN FILE source/crates/mori/src/commands/check_source.rs | SHA256 a53d5c3689f3c32ddc8c595e577a0ac13cfc4fe187b42a4f83c9caf86dc9ace3 | BYTES 3139
//! Shared source-profile dispatch and the existing check report formats.

use mori_diagnostics::MoriDiagnostic;

use crate::json::CheckReport;

/// Consume checked values while the Beta AST they borrow is still alive.
/// Reports remain optional: a human consumer can render diagnostics directly.
pub(super) trait SourceCheckConsumer {
    type Output;

    /// Called after Beta parsing, before its checker starts.
    fn parsed(&mut self, _tokens: usize, _nodes: usize) {}

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output;

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output;
}

/// Select exactly the same Beta, Core1 or staged Core2 path as `mori check`.
pub(super) fn dispatch<C: SourceCheckConsumer>(source: &str, mut consumer: C) -> C::Output {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let language = match mori_core::language(source) {
            Ok(language) => language,
            Err(error) => return consumer.core(Err(error), false),
        };
        let staged = language == "moriarty/2";
        let result = if staged {
            mori_core::stage::load(source).map(|program| program.program().clone())
        } else {
            mori_core::parse(source)
        };
        return consumer.core(result, staged);
    }
    match mori_parser::parse(source) {
        Err(error) => consumer.beta(None, vec![error]),
        Ok(ast) => {
            consumer.parsed(ast.tokens.len(), ast.nodes.len());
            let mori_checker::CheckResult {
                checked,
                diagnostics,
            } = mori_checker::check(&ast);
            consumer.beta(Some(&checked), diagnostics)
        }
    }
}

/// Preserve the Beta struct's serialization order and diagnostic/span mapping.
pub(super) fn beta_report(
    file: String,
    source: &str,
    checked: Option<&mori_checker::Checked<'_>>,
    diagnostics: &[MoriDiagnostic],
) -> CheckReport {
    CheckReport::new(file, source, checked, diagnostics)
}

/// Construct the existing Core JSON object only when a consumer requests it.
pub(super) fn core_report(
    file: &str,
    source: &str,
    result: &Result<mori_core::Program, mori_core::Error>,
    staged: bool,
) -> serde_json::Value {
    let mut report = match result {
        Ok(program) => {
            serde_json::json!({"status":"accepted", "file":file,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
        }
        Err(error) => {
            serde_json::json!({"status":"rejected","file":file,"diagnostics":[error]})
        }
    };
    if staged {
        let object = report
            .as_object_mut()
            .expect("stage reports are JSON objects");
        for (name, value) in mori_core::stage::STAGE_LABELS {
            object.insert(name.into(), serde_json::Value::String(value.into()));
        }
    }
    report
}

END FILE source/crates/mori/src/commands/check_source.rs


BEGIN FILE source/crates/mori-core/src/parser.rs | SHA256 66c839add400c62e70371a2cf1c412d391f3e0b626b860dc51a3e01039d46cc7 | BYTES 15611
use crate::*;
use mori_lexer::{TokenIdx, TokenKind};

/// Parses the explicitly versioned core profile; beta grammar is untouched.
pub fn parse(source: &str) -> Result<Program, Error> {
    parse_profile(source, false)
}

/// Syntax-only parser; only stage::load admits the resulting source.
pub(crate) fn parse_v2(source: &str) -> Result<Program, Error> {
    parse_profile(source, true)
}

fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let tokens = (0..lexed.len())
        .map(|i| {
            let idx = TokenIdx::new(i);
            let span = lexed.span(idx, source);
            let raw = &source[span.start as usize..span.end as usize];
            let text = if lexed.kind(idx) == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            };
            Tok {
                text,
                kind: lexed.kind(idx),
                start: span.start,
            }
        })
        .collect();
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: if staged {
                crate::stage::LANGUAGE_2
            } else {
                LANGUAGE
            }
            .into(),
            semantics: if staged {
                crate::stage::SEMANTICS_2
            } else {
                SEMANTICS
            }
            .into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    parser.expect("language")?;
    let profile = parser.string()?;
    let expected_language = if staged {
        crate::stage::LANGUAGE_2
    } else {
        LANGUAGE
    };
    if profile != expected_language {
        return Err(parser.error(
            "profile",
            format!("Expected language \"{expected_language}\"; found {profile:?}."),
        ));
    }
    parser.expect(";")?;
    parser.expect("agreement")?;
    parser.program.name = parser.word()?;
    parser.expect("{")?;
    while !parser.eat("}") {
        if parser.eat("state") {
            let name = parser.word()?;
            parser.expect(":")?;
            let ty = parser.ty()?;
            parser.expect("=")?;
            let initial = parser.literal()?;
            parser.expect(";")?;
            parser.program.states.push(StateDecl { name, ty, initial });
        } else if parser.eat("asset") {
            let name = parser.word()?;
            parser.expect("=")?;
            parser.expect("{")?;
            parser.expect("network")?;
            parser.expect(":")?;
            let network = parser.string()?;
            parser.expect(",")?;
            parser.expect("issuer")?;
            parser.expect(":")?;
            let issuer = parser.string()?;
            parser.expect(",")?;
            parser.expect("reference")?;
            parser.expect(":")?;
            let reference = parser.string()?;
            parser.expect(",")?;
            parser.expect("scale")?;
            parser.expect(":")?;
            let Value::UInt(UInt(scale)) = parser.literal()? else {
                return Err(parser.error("asset", "Asset scale must be an unsigned integer."));
            };
            let scale = u8::try_from(scale)
                .map_err(|_| parser.error("asset", "Asset scale must fit u8."))?;
            parser.eat(",");
            parser.expect("}")?;
            parser.expect(";")?;
            parser.program.assets.push(AssetDecl {
                name,
                identity: AssetId {
                    network,
                    issuer,
                    reference,
                    scale,
                },
            });
        } else if parser.eat("action") {
            let name = parser.word()?;
            parser.expect("(")?;
            let mut inputs = vec![];
            if !parser.eat(")") {
                loop {
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    inputs.push(InputDecl { name, ty });
                    if parser.eat(")") {
                        break;
                    }
                    parser.expect(",")?;
                }
            }
            parser.expect("{")?;
            let mut statements = vec![];
            while !parser.eat("}") {
                if parser.eat("input") {
                    if !statements.is_empty() {
                        return Err(
                            parser.error("input", "Declare action inputs before statements.")
                        );
                    }
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    parser.expect(";")?;
                    inputs.push(InputDecl { name, ty });
                    continue;
                }
                statements.push(parser.statement()?);
                if statements.len() > MAX_STATEMENTS {
                    return Err(parser.error("bound", "Action exceeds 1,024 statements."));
                }
            }
            parser.program.actions.push(Action {
                name,
                inputs,
                statements,
            });
        } else {
            return Err(parser.error("unsupported", "Expected state, asset or action declaration; loops and native calls are unsupported."));
        }
        if parser.program.states.len() + parser.program.assets.len() + parser.program.actions.len()
            > MAX_DECLARATIONS
        {
            return Err(parser.error("bound", "Program exceeds 256 declarations."));
        }
    }
    if parser.peek().kind != TokenKind::Eof {
        return Err(parser.error("syntax", "Unexpected content after agreement."));
    }
    if !staged {
        validate(&parser.program)?;
    }
    Ok(parser.program)
}
struct Tok {
    text: String,
    kind: TokenKind,
    start: u32,
}
struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    program: Program,
}
impl Parser {
    fn peek(&self) -> &Tok {
        &self.tokens[self.pos]
    }
    fn error(&self, code: &str, message: impl Into<String>) -> Error {
        let mut e = Error::new(code, message);
        e.offset = Some(self.peek().start);
        e
    }
    fn eat(&mut self, text: &str) -> bool {
        if self.peek().text == text && self.peek().kind != TokenKind::String {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, text: &str) -> Result<(), Error> {
        if self.eat(text) {
            Ok(())
        } else {
            Err(self.error(
                "syntax",
                format!("Expected {text:?}; found {:?}.", self.peek().text),
            ))
        }
    }
    fn word(&mut self) -> Result<String, Error> {
        if self.peek().kind.is_word() {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a name."))
        }
    }
    fn string(&mut self) -> Result<String, Error> {
        if self.peek().kind == TokenKind::String {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a quoted string."))
        }
    }
    fn ty(&mut self) -> Result<Type, Error> {
        match self.word()?.as_str() {
            "UInt" => Ok(Type::UInt),
            "Bool" => Ok(Type::Bool),
            _ => Err(self.error("type", "Supported types are UInt and Bool.")),
        }
    }
    fn literal(&mut self) -> Result<Value, Error> {
        if self.eat("true") {
            return Ok(Value::Bool(true));
        }
        if self.eat("false") {
            return Ok(Value::Bool(false));
        }
        if self.peek().kind != TokenKind::Number {
            return Err(self.error("syntax", "Expected an unsigned integer or boolean literal."));
        }
        let raw = self.peek().text.replace('_', "");
        let n = raw.parse::<u128>().map_err(|_| {
            self.error(
                "number",
                "UInt literal must fit u128; amounts use integral atomic units.",
            )
        })?;
        self.pos += 1;
        Ok(Value::UInt(UInt(n)))
    }
    fn push(&mut self, expr: Expr) -> Result<ExprId, Error> {
        if self.program.expressions.len() >= MAX_EXPRESSIONS {
            return Err(self.error("bound", "Program exceeds 8,192 expression nodes."));
        }
        let id = self.program.expressions.len() as u16;
        self.program.expressions.push(expr);
        Ok(id)
    }
    fn expr(&mut self, minimum: u8, depth: usize) -> Result<ExprId, Error> {
        if depth > 64 {
            return Err(self.error("bound", "Expression nesting exceeds 64."));
        }
        let mut lhs = if self.eat("(") {
            let e = self.expr(0, depth + 1)?;
            self.expect(")")?;
            e
        } else if self.eat("not") {
            let value = self.expr(6, depth + 1)?;
            self.push(Expr::Not { value })?
        } else if self.peek().kind == TokenKind::Number
            || self.peek().text == "true"
            || self.peek().text == "false"
        {
            let value = self.literal()?;
            self.push(Expr::Literal { value })?
        } else {
            let name = self.word()?;
            if matches!(name.as_str(), "liability_present" | "liability_amount") && self.eat("(") {
                let duty = self.string()?;
                self.expect(")")?;
                self.push(if name == "liability_present" {
                    Expr::LiabilityPresent { name: duty }
                } else {
                    Expr::LiabilityAmount { name: duty }
                })?
            } else {
                self.push(Expr::Name { name })?
            }
        };
        loop {
            // A decoded string spelling an operator is still a string token.
            if self.peek().kind == TokenKind::String {
                break;
            }
            let next = self
                .tokens
                .get(self.pos + 1)
                .filter(|t| t.kind != TokenKind::String)
                .map(|t| t.text.as_str());
            let (op, precedence, length) = match (self.peek().text.as_str(), next) {
                ("or", _) => (BinaryOp::Or, 1, 1),
                ("and", _) => (BinaryOp::And, 2, 1),
                ("=", Some("=")) => (BinaryOp::Eq, 3, 2),
                ("not", Some("=")) => (BinaryOp::Ne, 3, 2),
                ("<", Some("=")) => (BinaryOp::Le, 3, 2),
                (">", Some("=")) => (BinaryOp::Ge, 3, 2),
                ("<", _) => (BinaryOp::Lt, 3, 1),
                (">", _) => (BinaryOp::Gt, 3, 1),
                ("+", _) => (BinaryOp::Add, 4, 1),
                ("-", _) => (BinaryOp::Sub, 4, 1),
                ("*", _) => (BinaryOp::Mul, 5, 1),
                _ => break,
            };
            if precedence < minimum {
                break;
            }
            self.pos += length;
            let rhs = self.expr(precedence + 1, depth + 1)?;
            lhs = self.push(Expr::Binary { op, lhs, rhs })?;
        }
        Ok(lhs)
    }
    fn statement(&mut self) -> Result<Statement, Error> {
        let keyword = self.word()?;
        let s=match keyword.as_str() {
            "await" if self.program.language == crate::stage::LANGUAGE_2 => {
                let label = self.word()?;
                self.expect("(")?;
                let mut binders = Vec::new();
                if !self.eat(")") {
                    loop {
                        let name = self.word()?;
                        self.expect(":")?;
                        let ty = self.ty()?;
                        binders.push(InputDecl { name, ty });
                        if self.eat(")") { break; }
                        self.expect(",")?;
                    }
                }
                self.expect("when")?;
                Statement::Await { label, binders, guard: self.expr(0, 0)? }
            },
            "let"=>{let name=self.word()?;self.expect("=")?;Statement::Let{name,value:self.expr(0,0)?}},
            "next"=>{let state=self.word()?;self.expect("=")?;Statement::Next{state,value:self.expr(0,0)?}},
            "requires"=>Statement::Requires{predicate:self.expr(0,0)?},
            "ensures"=>Statement::Ensures{predicate:self.expr(0,0)?},
            "emit"=>{let name=self.string()?;self.expect("(")?;let value=self.expr(0,0)?;self.expect(")")?;Statement::Emit{name,value}},
            "transfer"|"fee"=>{self.expect("(")?;let asset=self.word()?;self.expect(",")?;let from=self.string()?;self.expect(",")?;let to=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;if keyword=="fee"{Statement::Fee{asset,from,to,amount}}else{Statement::Transfer{asset,from,to,amount}}},
            "liability"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let asset=self.word()?;self.expect(",")?;let debtor=self.string()?;self.expect(",")?;let creditor=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Liability{name,asset,debtor,creditor,amount}},
            "discharge"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Discharge{name,amount}},
            _=>return Err(self.error("unsupported",format!("Unsupported statement {keyword:?}; use let, requires, ensures, next, emit, transfer, fee, liability or discharge."))),
        };
        self.expect(";")?;
        Ok(s)
    }
}

/// Read the explicit source profile for CLI routing; this does not admit a program.
pub fn language(source: &str) -> Result<&'static str, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let mut tokens = Vec::new();
    for i in 0..lexed.len() {
        let idx = TokenIdx::new(i);
        let span = lexed.span(idx, source);
        let raw = &source[span.start as usize..span.end as usize];
        let kind = lexed.kind(idx);
        tokens.push(Tok {
            text: if kind == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            },
            kind,
            start: span.start,
        });
    }
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: LANGUAGE.into(),
            semantics: SEMANTICS.into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    let expected = format!(
        "language \"{LANGUAGE}\" or \"{}\"",
        crate::stage::LANGUAGE_2
    );
    parser.expect("language").map_err(|mut error| {
        error.message = format!("Expected {expected}. {}", error.message);
        error
    })?;
    let profile = parser.string().map_err(|mut error| {
        error.message = format!("Expected {expected}. {}", error.message);
        error
    })?;
    match profile.as_str() {
        LANGUAGE => Ok(LANGUAGE),
        crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
        _ => Err(parser.error(
            "profile",
            format!("Expected {expected}; found {profile:?}."),
        )),
    }
}

END FILE source/crates/mori-core/src/parser.rs


BEGIN FILE source/crates/mori-core/src/json.rs | SHA256 faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183 | BYTES 5391
//! Bounded JSON parsing that rejects duplicate keys before schema decoding.
use crate::Error;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::{cell::Cell, fmt};
const MAX_BYTES: usize = 1_048_576;
struct Seed<'a> {
    depth: usize,
    nodes: &'a Cell<usize>,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        use serde::de::Error as _;
        self.nodes.set(self.nodes.get() + 1);
        if self.depth > 32 || self.nodes.get() > 65_536 {
            return Err(D::Error::custom("JSON work/depth limit"));
        }
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite JSON number"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Seed {
            depth: self.depth + 1,
            nodes: self.nodes,
        })? {
            out.push(v)
        }
        Ok(Value::Array(out))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        use serde::de::Error as _;
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(A::Error::custom("duplicate decoded key"));
            }
            let v = a.next_value_seed(Seed {
                depth: self.depth + 1,
                nodes: self.nodes,
            })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
/// Strict recursive JSON value decoding for versioned DTO dynamic fields.
/// Duplicate decoded keys and the existing depth/node bounds apply even when
/// a consumer invokes serde directly instead of the bounded text entry point.
pub fn deserialize_strict<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Value, D::Error> {
    let nodes = Cell::new(0);
    Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(decoder)
}
pub fn parse(text: &str) -> Result<Value, Error> {
    parse_inner(text, MAX_BYTES, "JSON exceeds 1 MiB")
}
/// The same strict parser under a caller-selected byte ceiling. Duplicate-key,
/// depth (32) and node (65,536) limits are unchanged; `/2` stage files use it.
pub fn parse_limited(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    parse_inner(text, max, too_large)
}
fn parse_inner(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    use serde::de::DeserializeSeed;
    if text.len() > max {
        return Err(Error::new("json", too_large));
    }
    let nodes = Cell::new(0);
    let mut decoder = serde_json::Deserializer::from_str(text);
    let value = Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(&mut decoder)
    .map_err(|e| Error::new("json", e.to_string()))?;
    decoder
        .end()
        .map_err(|e| Error::new("json", e.to_string()))?;
    Ok(value)
}

/// ASCII-safe pretty JSON that fits the same public reader profile, including
/// the final newline added by CLI printing. Parsed values and signing bytes
/// are preserved; terminal-controlled Unicode remains visibly escaped.
pub fn export(value: &impl serde::Serialize) -> Result<String, Error> {
    let json =
        serde_json::to_string_pretty(value).map_err(|e| Error::new("json", e.to_string()))?;
    parse(&json)?;
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
        if display.len() >= MAX_BYTES {
            return Err(Error::new(
                "json",
                "Escaped JSON and final newline exceed 1 MiB",
            ));
        }
    }
    parse(&display)?;
    Ok(display)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_decoded_keys_and_trailing_data_reject() {
        assert!(parse(r#"{"x":1,"\u0078":2}"#).is_err());
        assert!(parse("{} []").is_err());
        assert!(parse(r#"{"x":"1"}"#).is_ok());
    }
}

END FILE source/crates/mori-core/src/json.rs


BEGIN FILE source/crates/mori-lexer/src/lib.rs | SHA256 5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7 | BYTES 8944
//! Lexer for `moriarty-beta/1` source.
//!
//! Turns source text into [`Tokens`] in a single pass.
//!
//! Lexing stops at the first error. Whitespace is skipped; comments are kept
//! separately for the formatter and do not count toward [`MAX_TOKENS`].

mod diagnostics;
mod number;
mod scan;
mod string;
mod token;

#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;
use soa_rs::Soa;

use crate::number::NumberError;
use crate::string::StringErrorKind;

pub use crate::token::{Comment, Token, TokenIdx, TokenKind, Tokens};

/// Largest accepted source file, in bytes.
pub const MAX_SOURCE_BYTES: usize = 65_536;
/// Most tokens in one file, including the end-of-file token.
pub const MAX_TOKENS: usize = 8_192;
/// Longest identifier or keyword, in bytes.
pub const MAX_IDENT_BYTES: usize = 64;
/// Longest decoded string, in UTF-8 bytes.
pub const MAX_STRING_BYTES: usize = 1_024;

/// The text of a string token, with escapes decoded.
///
/// `raw` must be the full text of a [`TokenKind::String`] token, quotes
/// included, as produced by [`lex`].
pub fn decode_string(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    string::decode_with(raw, |c| text.push(c)).expect("lexed strings are valid");
    text
}

/// Splits `source` into tokens and comments, or reports the first error.
pub fn lex(source: &str) -> Result<Tokens, MoriDiagnostic> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::source_too_large(source.len()));
    }
    Lexer::new(source).run(false)
}

/// Identify a leading profile token without requiring the remaining source to
/// lex successfully. Inspect only a bounded prefix; full parsing retains all
/// source and token limits.
pub fn first_token_text(source: &str) -> Result<&str, MoriDiagnostic> {
    let end = source.floor_char_boundary(source.len().min(MAX_SOURCE_BYTES));
    let prefix = &source[..end];
    let tokens = Lexer::new(prefix).run(true)?;
    Ok(tokens.span(TokenIdx::new(0), prefix).source_text(prefix))
}

struct Lexer<'src> {
    source: &'src str,
    bytes: &'src [u8],
    pos: usize,
    tokens: Soa<Token>,
    comments: Soa<Comment>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
            // Roughly one token per eight bytes, as in Zig.
            tokens: Soa::with_capacity(source.len() / 8 + 1),
            comments: Soa::new(),
        }
    }

    fn run(mut self, first_only: bool) -> Result<Tokens, MoriDiagnostic> {
        while let Some(&b) = self.bytes.get(self.pos) {
            let start = self.pos;
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => self.pos += 1,
                b'/' if self.peek(1) == Some(b'/') => self.line_comment(start),
                b'/' if self.peek(1) == Some(b'*') => self.block_comment(start)?,
                b'"' => self.string(start)?,
                b'0'..=b'9' => self.number(start)?,
                b'A'..=b'Z' | b'a'..=b'z' => self.word(start)?,
                _ => match TokenKind::punctuation(b) {
                    Some(kind) => self.push(kind, start, start + 1)?,
                    None => return Err(self.unexpected_character(start)),
                },
            }
            if first_only && !self.tokens.is_empty() {
                break;
            }
        }
        // The end-of-file token is always pushed: the bound reserves room for it.
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            start: offset(self.pos),
        });
        Ok(Tokens {
            tokens: self.tokens,
            comments: self.comments,
        })
    }

    fn peek(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos + ahead).copied()
    }

    /// Records a token ending at `end` and moves past it.
    fn push(&mut self, kind: TokenKind, start: usize, end: usize) -> Result<(), MoriDiagnostic> {
        if self.tokens.len() >= MAX_TOKENS - 1 {
            return Err(diagnostics::too_many_tokens(span(start, end)));
        }
        self.tokens.push(Token {
            kind,
            start: offset(start),
        });
        self.pos = end;
        Ok(())
    }

    fn push_comment(&mut self, start: usize, end: usize) {
        self.comments.push(Comment {
            start: offset(start),
            end: offset(end),
        });
        self.pos = end;
    }

    /// `//` up to, but not including, the next line break.
    fn line_comment(&mut self, start: usize) {
        let end = self.bytes[start..]
            .iter()
            .position(|&b| b == b'\r' || b == b'\n')
            .map_or(self.bytes.len(), |len| start + len);
        self.push_comment(start, end);
    }

    /// `/*` through the first `*/`. Block comments do not nest.
    fn block_comment(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let body = start + 2;
        let Some(len) = self.source[body..].find("*/") else {
            return Err(diagnostics::unterminated_block_comment(span(start, body)));
        };
        self.push_comment(start, body + len + 2);
        Ok(())
    }

    fn string(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let Some(end) = scan::string_end(self.bytes, start) else {
            return Err(diagnostics::unterminated_string(span(start, start + 1)));
        };
        let raw = &self.source[start..end];
        let mut decoded_len = 0;
        if let Err(error) = string::decode_with(raw, |c| decoded_len += c.len_utf8()) {
            let at = span(start + error.start, start + error.end);
            return Err(match error.kind {
                StringErrorKind::ControlCharacter('\r' | '\n') => {
                    diagnostics::string_line_break(at)
                }
                StringErrorKind::ControlCharacter(c) => {
                    diagnostics::string_control_character(at, c)
                }
                StringErrorKind::InvalidEscape => {
                    diagnostics::invalid_escape(at, at.source_text(self.source))
                }
                StringErrorKind::InvalidUnicodeEscape => diagnostics::invalid_unicode_escape(at),
                StringErrorKind::LoneSurrogate => diagnostics::lone_surrogate(at),
            });
        }
        if decoded_len > MAX_STRING_BYTES {
            return Err(diagnostics::string_too_long(span(start, end), decoded_len));
        }
        self.push(TokenKind::String, start, end)
    }

    fn number(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::number_end(self.bytes, start);
        let raw = &self.source[start..end];
        if let Err(error) = number::validate(raw) {
            return Err(match error {
                NumberError::LeadingZero => diagnostics::number_leading_zero(
                    span(start, end),
                    &number::without_leading_zeros(raw),
                ),
                NumberError::MisplacedUnderscore { at } => {
                    diagnostics::number_misplaced_underscore(span(start + at, start + at + 1))
                }
                NumberError::MissingFraction => {
                    diagnostics::number_missing_fraction(span(end - 1, end))
                }
                NumberError::ExtraDot { at } => {
                    diagnostics::number_extra_dot(span(start + at, start + at + 1))
                }
            });
        }
        self.push(TokenKind::Number, start, end)
    }

    /// An identifier, or a keyword when the text matches one.
    fn word(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::word_end(self.bytes, start);
        if end - start > MAX_IDENT_BYTES {
            return Err(diagnostics::identifier_too_long(span(start, end)));
        }
        let kind = TokenKind::keyword(&self.source[start..end]).unwrap_or(TokenKind::Ident);
        self.push(kind, start, end)
    }

    fn unexpected_character(&self, start: usize) -> MoriDiagnostic {
        let c = self.source[start..]
            .chars()
            .next()
            .expect("a character remains at the current position");
        let at = span(start, start + c.len_utf8());
        match c {
            '/' => diagnostics::division(at),
            '“' | '”' | '‘' | '’' => diagnostics::curly_quote(at),
            '\'' => diagnostics::single_quote(at),
            '_' => diagnostics::leading_underscore(at),
            c if !c.is_ascii() => diagnostics::non_ascii(at, c),
            c => diagnostics::unexpected_character(at, c),
        }
    }
}

/// Converts a byte index to an offset. Sources fit in `u32` after the bound check.
fn offset(index: usize) -> u32 {
    u32::try_from(index).expect("source offsets fit in u32")
}

fn span(start: usize, end: usize) -> Span {
    Span::new(offset(start), offset(end))
}

END FILE source/crates/mori-lexer/src/lib.rs


BEGIN FILE source/crates/mori-lexer/src/token.rs | SHA256 e7860a893c0827bbf362165438c67d621f229f0eacecb9175d959788687bc3db | BYTES 5490
use mori_span::Span;
use soa_rs::{Soa, Soars};

use crate::scan;

/// The kind of a token. Keywords get their own kinds so the parser compares
/// tags instead of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    /// A name that is not a keyword: `[A-Za-z][A-Za-z0-9_]*`, at most 64 bytes.
    Ident,
    /// A canonical decimal number, such as `1_000` or `10.50`.
    Number,
    /// A JSON string literal, such as `"Midnight"`.
    String,

    // Keywords stay contiguous from `KwProfile` to `KwShareClass`; see `is_keyword`.
    KwProfile,
    KwAgreement,
    KwAction,
    KwUses,
    KwTrue,
    KwFalse,
    KwNone,
    KwSuccessOnly,

    // Declaration kinds stay contiguous from `KwDomain` to `KwShareClass`; see
    // `is_declaration_kind`.
    KwDomain,
    KwAccount,
    KwAsset,
    KwConst,
    KwIntent,
    KwObligation,
    KwPool,
    KwInstrument,
    KwObservation,
    KwPolicy,
    KwGrant,
    KwStage,
    KwEpisode,
    KwParty,
    KwShareClass,

    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    Semicolon,
    Comma,
    Eq,
    Lt,
    Gt,
    Dot,
    Plus,
    Minus,
    Star,

    Eof,
}

impl TokenKind {
    /// The keyword spelled `text`, if any.
    pub fn keyword(text: &str) -> Option<Self> {
        let kind = match text {
            "profile" => Self::KwProfile,
            "agreement" => Self::KwAgreement,
            "action" => Self::KwAction,
            "uses" => Self::KwUses,
            "true" => Self::KwTrue,
            "false" => Self::KwFalse,
            "None" => Self::KwNone,
            "SuccessOnly" => Self::KwSuccessOnly,
            "domain" => Self::KwDomain,
            "account" => Self::KwAccount,
            "asset" => Self::KwAsset,
            "const" => Self::KwConst,
            "intent" => Self::KwIntent,
            "obligation" => Self::KwObligation,
            "pool" => Self::KwPool,
            "instrument" => Self::KwInstrument,
            "observation" => Self::KwObservation,
            "policy" => Self::KwPolicy,
            "grant" => Self::KwGrant,
            "stage" => Self::KwStage,
            "episode" => Self::KwEpisode,
            "party" => Self::KwParty,
            "share_class" => Self::KwShareClass,
            _ => return None,
        };
        Some(kind)
    }

    /// The punctuation token for byte `b`, if any.
    pub fn punctuation(b: u8) -> Option<Self> {
        let kind = match b {
            b'{' => Self::LBrace,
            b'}' => Self::RBrace,
            b'[' => Self::LBracket,
            b']' => Self::RBracket,
            b'(' => Self::LParen,
            b')' => Self::RParen,
            b':' => Self::Colon,
            b';' => Self::Semicolon,
            b',' => Self::Comma,
            b'=' => Self::Eq,
            b'<' => Self::Lt,
            b'>' => Self::Gt,
            b'.' => Self::Dot,
            b'+' => Self::Plus,
            b'-' => Self::Minus,
            b'*' => Self::Star,
            _ => return None,
        };
        Some(kind)
    }

    pub fn is_keyword(self) -> bool {
        (Self::KwProfile as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Keywords that start a declaration, such as `const` or `asset`.
    pub fn is_declaration_kind(self) -> bool {
        (Self::KwDomain as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Identifiers and keywords. Record fields and call names accept either.
    pub fn is_word(self) -> bool {
        self == Self::Ident || self.is_keyword()
    }
}

/// A token's kind and start offset. Its end is recovered by re-scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: u32,
}

/// A `//` or `/* */` comment, kept for the formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Comment {
    pub start: u32,
    pub end: u32,
}

/// Index of a token in [`Tokens`]. Sources hold at most 8,192 tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenIdx(u16);

impl TokenIdx {
    pub fn new(index: usize) -> Self {
        Self(u16::try_from(index).expect("token index exceeds u16"))
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

/// The output of [`lex`](crate::lex): every token, ending with
/// [`TokenKind::Eof`], and every comment, both in source order.
#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub tokens: Soa<Token>,
    pub comments: Soa<Comment>,
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Always false: a token list ends with [`TokenKind::Eof`].
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn kind(&self, idx: TokenIdx) -> TokenKind {
        self.tokens.kind()[idx.index()]
    }

    pub fn start(&self, idx: TokenIdx) -> u32 {
        self.tokens.start()[idx.index()]
    }

    /// The token's span, recovered by re-scanning `source` from its start.
    pub fn span(&self, idx: TokenIdx, source: &str) -> Span {
        let start = self.start(idx);
        Span::new(
            start,
            scan::token_end(self.kind(idx), source.as_bytes(), start),
        )
    }

    pub fn comment_span(&self, index: usize) -> Span {
        Span::new(self.comments.start()[index], self.comments.end()[index])
    }
}

END FILE source/crates/mori-lexer/src/token.rs


BEGIN FILE source/crates/mori-lexer/src/string.rs | SHA256 b94878f41914f56dfd4945ebb0b44690db541e77848817bc6e265dc422f88aaa | BYTES 4862
//! JSON string literals.
//!
//! Strings use JSON syntax: no raw control characters, and only the escapes
//! `\" \\ \/ \b \f \n \r \t \uXXXX`. A `\u` escape that encodes half of a
//! surrogate pair without its other half is rejected, but only after the
//! whole literal is valid JSON, matching the reference implementation.

use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringErrorKind {
    /// A raw character below U+0020, such as a line break.
    ControlCharacter(char),
    /// A backslash followed by a character that is not a JSON escape.
    InvalidEscape,
    /// `\u` not followed by four hexadecimal digits.
    InvalidUnicodeEscape,
    /// A `\u` escape for half of a surrogate pair, without its other half.
    LoneSurrogate,
}

/// An error at byte range `start..end` of the literal, quotes included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringError {
    pub kind: StringErrorKind,
    pub start: usize,
    pub end: usize,
}

/// Decodes the literal `raw`, quotes included, passing each character to
/// `emit`. `raw` must start and end with an unescaped `"`.
pub fn decode_with(raw: &str, mut emit: impl FnMut(char)) -> Result<(), StringError> {
    let body = &raw[1..raw.len() - 1];
    let mut chars = body.char_indices().peekable();
    let mut pending_high: Option<(u32, usize)> = None;
    let mut first_lone: Option<(usize, usize)> = None;

    let error = |kind, start: usize, end: usize| StringError {
        kind,
        start: start + 1,
        end: end + 1,
    };

    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            let Some((_, escape)) = chars.next() else {
                return Err(error(StringErrorKind::InvalidEscape, i, i + 1));
            };
            if escape == 'u' {
                let unit = hex4(&mut chars).ok_or_else(|| {
                    let end = chars.peek().map_or(body.len(), |&(j, _)| j);
                    error(StringErrorKind::InvalidUnicodeEscape, i, end)
                })?;
                let end = i + 6;
                match (pending_high.take(), unit) {
                    (Some((high, _)), 0xDC00..=0xDFFF) => {
                        let scalar = 0x10000 + ((high - 0xD800) << 10) + (unit - 0xDC00);
                        emit(char::from_u32(scalar).expect("surrogate pair is a scalar"));
                    }
                    (pending, _) => {
                        if let Some((_, start)) = pending {
                            first_lone.get_or_insert((start, start + 6));
                            emit(char::REPLACEMENT_CHARACTER);
                        }
                        match unit {
                            0xD800..=0xDBFF => pending_high = Some((unit, i)),
                            0xDC00..=0xDFFF => {
                                first_lone.get_or_insert((i, end));
                                emit(char::REPLACEMENT_CHARACTER);
                            }
                            _ => emit(char::from_u32(unit).expect("non-surrogate is a scalar")),
                        }
                    }
                }
                continue;
            }

            let decoded = match escape {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{8}',
                'f' => '\u{c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => {
                    let end = i + 1 + escape.len_utf8();
                    return Err(error(StringErrorKind::InvalidEscape, i, end));
                }
            };
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(decoded);
        } else if c < ' ' {
            return Err(error(StringErrorKind::ControlCharacter(c), i, i + 1));
        } else {
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(c);
        }
    }
    flush_high(&mut pending_high, &mut first_lone, &mut emit);

    match first_lone {
        Some((start, end)) => Err(error(StringErrorKind::LoneSurrogate, start, end)),
        None => Ok(()),
    }
}

/// A high surrogate not followed by a low surrogate escape is lone.
fn flush_high(
    pending_high: &mut Option<(u32, usize)>,
    first_lone: &mut Option<(usize, usize)>,
    emit: &mut impl FnMut(char),
) {
    if let Some((_, start)) = pending_high.take() {
        first_lone.get_or_insert((start, start + 6));
        emit(char::REPLACEMENT_CHARACTER);
    }
}

/// Reads exactly four hexadecimal digits.
fn hex4(chars: &mut Peekable<CharIndices<'_>>) -> Option<u32> {
    let mut value = 0;
    for _ in 0..4 {
        let digit = chars.peek()?.1.to_digit(16)?;
        chars.next();
        value = value * 16 + digit;
    }
    Some(value)
}

END FILE source/crates/mori-lexer/src/string.rs


BEGIN FILE source/crates/mori-lexer/src/scan.rs | SHA256 4119a98f6ade2d96cb1231b0531cdbb63d01663d24c7e2615ef5b74931db5637 | BYTES 1822
//! Token extent rules, shared by the lexer and by token-end recovery.

use crate::TokenKind;

pub fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// End of the identifier or keyword starting at `start`.
pub fn word_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| is_word_byte(b))
            .count()
}

/// End of the number starting at `start`. Numbers extend over digits, `_`
/// and `.`; spelling is validated separately.
pub fn number_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| b.is_ascii_digit() || b == b'_' || b == b'.')
            .count()
}

/// End of the string starting with `"` at `start`, just past the closing
/// quote, or `None` if it is unterminated. A backslash escapes the next byte.
pub fn string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut escaped = false;
    for (offset, &b) in bytes[start + 1..].iter().enumerate() {
        if b == b'"' && !escaped {
            return Some(start + 1 + offset + 1);
        }
        escaped = b == b'\\' && !escaped;
    }
    None
}

/// Recovers the end offset of a token from its kind and start.
pub fn token_end(kind: TokenKind, bytes: &[u8], start: u32) -> u32 {
    let start_index = start as usize;
    let end = match kind {
        TokenKind::Ident => word_end(bytes, start_index),
        _ if kind.is_keyword() => word_end(bytes, start_index),
        TokenKind::Number => number_end(bytes, start_index),
        TokenKind::String => string_end(bytes, start_index).expect("lexed strings are terminated"),
        TokenKind::Eof => start_index,
        _ => start_index + 1,
    };
    end as u32
}

END FILE source/crates/mori-lexer/src/scan.rs


BEGIN FILE source/crates/mori/Cargo.toml | SHA256 2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b | BYTES 2883
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
core2-disjoint-draft = ["mori-core/core2-disjoint-draft"]
default = ["native"]
source6-expansion = ["dep:mori-source6", "mori-source6/source6-transfer-slice"]
native = [
    "dep:mori-backend",
    "dep:mori-ledger",
    "dep:futures",
    "dep:rand",
    "dep:midnight-ledger",
    "dep:midnight-base-crypto",
    "dep:midnight-coin-structure",
    "dep:midnight-onchain-state",
    "dep:midnight-onchain-runtime",
    "dep:midnight-storage",
    "dep:midnight-transient-crypto",
    "dep:midnight-serialize",
]

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend = { workspace = true, optional = true }
mori-ledger = { workspace = true, optional = true }
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-source6 = { workspace = true, optional = true }
mori-span.workspace = true
rayon.workspace = true
futures = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

midnight-ledger = { workspace = true, optional = true }
midnight-base-crypto = { workspace = true, optional = true }
midnight-coin-structure = { workspace = true, optional = true }
midnight-onchain-state = { workspace = true, optional = true }
midnight-onchain-runtime = { workspace = true, optional = true }
midnight-storage = { workspace = true, optional = true }
midnight-transient-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

[[test]]
name = "check_compile_io"
required-features = ["native"]

[[test]]
name = "check_static_compile_io"
required-features = ["native"]

[[test]]
name = "prepare_call"
required-features = ["native"]

[[test]]
name = "installed_call"
required-features = ["native"]

[[test]]
name = "signed_installed_call"
required-features = ["native"]

[[test]]
name = "native_proofs"
required-features = ["native"]

[[test]]
name = "signed_intents"
required-features = ["native"]

[[test]]
name = "replay"
required-features = ["native"]

[[test]]
name = "replay_challenge"
required-features = ["native"]

[[test]]
name = "enforcement_report"
required-features = ["native"]

[[test]]
name = "mapped_role_static_io"
required-features = ["native"]

[[test]]
name = "source6_expand"
required-features = ["source6-expansion"]

END FILE source/crates/mori/Cargo.toml


BEGIN FILE source/crates/mori-core/Cargo.toml | SHA256 93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c | BYTES 881
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[features]
core2-disjoint-draft = []
# Opt-in and non-default: enables only the Midnight-native signing-frame
# compatibility test. Ordinary mori-core consumers and SDK-free test
# selection never pull these dependencies.
midnight-compat-tests = ["dep:midnight-base-crypto", "dep:midnight-serialize"]

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true
# Optional and off by default; pulled in only by the midnight-compat-tests
# feature for the native compatibility test.
midnight-base-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
proptest.workspace = true

END FILE source/crates/mori-core/Cargo.toml


BEGIN FILE source/rust-toolchain.toml | SHA256 72f7dd8c9fdf9c1fcd8eb8bf9326d4bc66833e4ebf4bc2932636e2b85efcb7af | BYTES 86
[toolchain]
channel = "1.99.0"
components = ["rustfmt", "clippy"]
profile = "minimal"

END FILE source/rust-toolchain.toml


BEGIN FILE units/core_io-read_source.rs | SHA256 e8f1a0d5cd1c2eeee11708b9ceaab206b8f08435fd6e072259edc1932203ce33 | BYTES 530
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

END FILE units/core_io-read_source.rs


BEGIN FILE units/core_io-operation.rs | SHA256 51d63b55691c08ef387fc01056e995f95500a286660ef79ec6fea42edb8d9f6c | BYTES 221
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}

END FILE units/core_io-operation.rs


BEGIN FILE units/core_io-print.rs | SHA256 7f1351583d72965b909c84ede5884a48fba83dc868dde6f57ebcd4f165a868cd | BYTES 171
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}

END FILE units/core_io-print.rs


BEGIN FILE units/simulate-print-stage-report.rs | SHA256 a10447885a0129d50bb969d563e3e0daf84734db2f669d4da9b10d58e055db85 | BYTES 365
pub(super) fn print_stage_report(mut report: serde_json::Value) -> super::Result<()> {
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in mori_core::stage::STAGE_LABELS {
        object.insert(name.into(), serde_json::Value::String(value.into()));
    }
    super::core_io::print(&report)
}

END FILE units/simulate-print-stage-report.rs


BEGIN FILE units/profile-constants.rs | SHA256 2ad2722658f82d62e485b670182ecfc124c615114a2d1464e9a23e7d70d237fb | BYTES 323
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;
pub const MAX_TRACE_CELLS: usize = 65_536;

END FILE units/profile-constants.rs


BEGIN FILE units/core-Error-dto.rs | SHA256 27c7090a3c80b1316a670a7660ff7bdd2f4b289ba017379ac6c9616180404613 | BYTES 489
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub offset: Option<u32>,
    pub step: Option<u16>,
}
impl Error {
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: format!("mori::core::{code}"),
            message: message.into(),
            offset: None,
            step: None,
        }
    }
}

END FILE units/core-Error-dto.rs


BEGIN FILE units/existing-library-header-test.rs | SHA256 50711fa2ad2f50dc82327e1c6b078b8c65b9880833640873e3d073784a8481c9 | BYTES 1359
#[test]
fn language_header_diagnostics_name_both_supported_core_profiles() {
    for (source, expected) in [
        (
            r#"language "moriarty/1"; agreement Example {}"#,
            "moriarty/1",
        ),
        (
            r#"language "moriarty/2"; agreement Example {}"#,
            "moriarty/2",
        ),
    ] {
        assert_eq!(language(source).unwrap(), expected);
    }

    let expected = r#"Expected language "moriarty/1" or "moriarty/2""#;
    for (source, code, offset, suffix) in [
        (
            r#"language "other";"#,
            "mori::core::profile",
            16,
            r#"; found "other"."#,
        ),
        (
            "",
            "mori::core::syntax",
            0,
            r#". Expected "language"; found ""."#,
        ),
        (
            "agreement Empty {}",
            "mori::core::syntax",
            0,
            r#". Expected "language"; found "agreement"."#,
        ),
        (
            "language ;",
            "mori::core::syntax",
            9,
            ". Expected a quoted string.",
        ),
    ] {
        let error = language(source).unwrap_err();
        assert_eq!(error.code, code, "{source:?}");
        assert_eq!(error.offset, Some(offset), "{source:?}");
        assert_eq!(error.message, format!("{expected}{suffix}"), "{source:?}");
    }
}

END FILE units/existing-library-header-test.rs


BEGIN FILE units/stage-profile-labels.rs | SHA256 27c6e67cdd971f72eebe9dc29c7ae8eeef3c60552d8796dfc86ccd8d9b6eaa57 | BYTES 725
pub const LANGUAGE_2: &str = "moriarty/2";
pub const SEMANTICS_2: &str = "moriarty-core/2";
pub const STAGE_PLAN_FORMAT: &str = "moriarty-stage-plan/1";
pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1";
pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1";
pub const MAX_AWAITS_PER_ACTION: usize = 16;
pub const MAX_EVENT_BINDERS: usize = 16;
pub const MAX_CAPTURE_SLOTS: usize = 256;
pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576;
pub const MAX_EVENT_BYTES: usize = 65_536;
/// Inserted verbatim into every `/2` output object.
pub const STAGE_LABELS: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];

END FILE units/stage-profile-labels.rs


BEGIN FILE units/staged-program-accessors.rs | SHA256 090e597b90cc121029a5b61844b8a93be11ab264bf44840a11277357c0569de0 | BYTES 879
/// A `/2` program that passed profile, validation, plan invariants and static
/// costing, with its canonical digests. Only `load` constructs one; there is
/// no public constructor, `Default` or `Deserialize`.
#[derive(Debug, Clone)]
pub struct StagedProgram {
    program: Program,
    plan: StagePlan,
    program_hash: [u8; 32],
    plan_hash: [u8; 32],
}
impl StagedProgram {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn plan(&self) -> &StagePlan {
        &self.plan
    }
    pub fn program_hash(&self) -> [u8; 32] {
        self.program_hash
    }
    pub fn plan_hash(&self) -> [u8; 32] {
        self.plan_hash
    }
    /// Declaration index of an action; plan and program actions share it.
    pub(crate) fn action_index(&self, name: &str) -> Option<usize> {
        self.program.actions.iter().position(|a| a.name == name)
    }
}

END FILE units/staged-program-accessors.rs


BEGIN FILE units/stage-load.rs | SHA256 e958f90290985ad748278626e1cc591d3885f4e174a0bdf3c52c5a5c88c53e25 | BYTES 767
/// Parses, admits and plans a `moriarty/2` source. Legacy `parse`/`validate`
/// remain `/1`-only, so a `/2` source fails every legacy consumer.
pub fn load(src: &str) -> Result<StagedProgram, Error> {
    let program = crate::parser::parse_v2(src)?;
    profile::require_core2(&program)?;
    validate::validate_v2(&program)?;
    let mut plan = plan::build(&program)?;
    validate::plan_invariants(&program, &plan)?;
    cost::fill(&program, &mut plan)?;
    validate::cost_invariants(&program, &plan)?;
    let program_hash = digest::program_hash(&program)?;
    plan.program_hash = digest::hex(&program_hash);
    let plan_hash = digest::plan_hash(&plan)?;
    Ok(StagedProgram {
        program,
        plan,
        program_hash,
        plan_hash,
    })
}

END FILE units/stage-load.rs


BEGIN FILE units/cli-global-options.rs | SHA256 d39eb77f73e940188e0746b96231f679ee88c9bad30c4072a49a7e6761137fd2 | BYTES 684
#[derive(Debug, Args)]
#[command(next_help_heading = "Global Options")]
pub struct GlobalArgs {
    /// Print no status output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Print extra status output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// When to use color
    #[arg(long, global = true, value_name = "WHEN", default_value = "auto")]
    pub color: Color,
}

impl GlobalArgs {
    pub fn verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else if self.verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        }
    }
}

END FILE units/cli-global-options.rs


BEGIN FILE units/cli-color-conversion.rs | SHA256 b3040b38b5cfa64baa23e2c50da404635b86adc6c7ca5b9d61bc049827df1fc5 | BYTES 338
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl From<Color> for anstream::ColorChoice {
    fn from(color: Color) -> Self {
        match color {
            Color::Auto => Self::Auto,
            Color::Always => Self::Always,
            Color::Never => Self::Never,
        }
    }
}

END FILE units/cli-color-conversion.rs


BEGIN FILE units/mori-diagnostics-render-production.rs | SHA256 064524e1e7dc533a5e1fc7136aeac478cd2d39e2e20f5b202f455a62ccb45929 | BYTES 1664
//! Graphical rendering of diagnostics against their source.

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, NamedSource, Report};

use crate::MoriDiagnostic;

/// Widest rendering, in columns. Snapshots always use this width.
pub const WIDTH: usize = 80;

/// Renders a report in color for a terminal `width` columns wide, capped at
/// [`WIDTH`]. Write it through a stream that strips color when it is off.
pub fn render_colored(report: &Report, width: usize) -> String {
    trim_lines(&render(
        GraphicalTheme::unicode(),
        width.min(WIDTH),
        report.as_ref(),
    ))
}

/// Renders a diagnostic against its source with no color and a fixed width.
///
/// The output is what a user sees, minus color and trailing spaces, so it is
/// stable across terminals and editors and suitable for snapshots.
pub fn render_plain(diagnostic: &MoriDiagnostic, name: &str, source: &str) -> String {
    let report = diagnostic
        .clone()
        .with_source_code(NamedSource::new(name, source.to_owned()));
    trim_lines(&render(
        GraphicalTheme::unicode_nocolor(),
        WIDTH,
        report.as_ref(),
    ))
}

/// Removes trailing spaces from every line.
fn trim_lines(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end().to_owned() + "\n")
        .collect()
}

fn render(theme: GraphicalTheme, width: usize, diagnostic: &dyn Diagnostic) -> String {
    let mut out = String::new();
    GraphicalReportHandler::new_themed(theme)
        .with_width(width)
        .with_links(false)
        .render_report(&mut out, diagnostic)
        .expect("writing to a String cannot fail");
    out
}


END FILE units/mori-diagnostics-render-production.rs


BEGIN FILE units/check-json-production.rs | SHA256 58e0745bc8829225c06f355a254f8539f416619f9fd6253c565f7d27ee5c4dbb | BYTES 6016
//! The `--json` report of `mori check`.
//!
//! Spans are UTF-8 byte ranges into the checked file. This format is our own;
//! it is not the TypeScript beta's.

use mori_ast::{Ast, NodeIdx};
use mori_checker::{Checked, DeclIdx, Support};
use mori_diagnostics::{Diagnostic, MoriDiagnostic, Severity};
use mori_lexer::TokenIdx;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub struct CheckReport {
    /// `accepted` when there are no diagnostics, otherwise `rejected`.
    pub status: &'static str,
    pub file: String,
    /// SHA-256 of the file's bytes, in lowercase hex.
    pub source_hash: String,
    /// Absent when the file did not parse.
    pub agreement: Option<Named>,
    pub declarations: Vec<DeclarationReport>,
    pub actions: Vec<ActionReport>,
    pub diagnostics: Vec<DiagnosticReport>,
}

#[derive(Serialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl From<mori_span::Span> for Span {
    fn from(span: mori_span::Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Serialize)]
pub struct Named {
    pub name: String,
    pub span: Span,
}

#[derive(Serialize)]
pub struct DeclarationReport {
    /// The declaring keyword, such as `asset` or `intent`.
    pub kind: String,
    pub name: String,
    pub span: Span,
    pub name_span: Span,
}

#[derive(Serialize)]
pub struct ActionReport {
    pub name: String,
    pub intent: String,
    /// `local_s0` when it runs locally, `specified_only` when it is only
    /// checked; absent when its intent did not check.
    pub support: Option<&'static str>,
    pub span: Span,
    pub name_span: Span,
}

#[derive(Serialize)]
pub struct DiagnosticReport {
    pub code: Option<String>,
    pub severity: &'static str,
    pub message: String,
    pub help: Option<String>,
    pub labels: Vec<LabelReport>,
}

#[derive(Serialize)]
pub struct LabelReport {
    pub span: Span,
    pub text: Option<String>,
    pub primary: bool,
}

impl CheckReport {
    pub fn new(
        file: String,
        source: &str,
        checked: Option<&Checked<'_>>,
        diagnostics: &[MoriDiagnostic],
    ) -> Self {
        let source_hash = Sha256::digest(source.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let mut report = Self {
            status: if diagnostics.is_empty() {
                "accepted"
            } else {
                "rejected"
            },
            file,
            source_hash,
            agreement: None,
            declarations: Vec::new(),
            actions: Vec::new(),
            diagnostics: diagnostics.iter().map(DiagnosticReport::new).collect(),
        };
        if let Some(checked) = checked {
            report.fill(checked);
        }
        report
    }

    fn fill(&mut self, checked: &Checked<'_>) {
        let ast = checked.ast;
        let name_token = ast.main_token(NodeIdx::ROOT);
        self.agreement = Some(Named {
            name: ast.token_text(name_token).to_owned(),
            span: token_span(ast, name_token).into(),
        });
        for index in 0..checked.declarations.len() {
            let decl = DeclIdx::new(index);
            let node = checked.declarations.node()[index];
            let keyword = ast.main_token(node);
            self.declarations.push(DeclarationReport {
                kind: ast.token_text(keyword).to_owned(),
                name: checked.name(decl).to_owned(),
                span: ast.span(node).into(),
                name_span: token_span(ast, ast.token_after(keyword, 1)).into(),
            });
        }
        for (&node, &support) in checked.actions.node().iter().zip(checked.actions.support()) {
            let keyword = ast.main_token(node);
            self.actions.push(ActionReport {
                name: ast.token_text(ast.token_after(keyword, 1)).to_owned(),
                intent: ast.token_text(ast.token_after(keyword, 3)).to_owned(),
                support: support.map(|support| match support {
                    Support::LocalS0 => "local_s0",
                    Support::SpecifiedOnly => "specified_only",
                }),
                span: ast.span(node).into(),
                name_span: token_span(ast, ast.token_after(keyword, 1)).into(),
            });
        }
    }
}

impl DiagnosticReport {
    fn new(diagnostic: &MoriDiagnostic) -> Self {
        let severity = match diagnostic.severity() {
            Some(Severity::Warning) => "warning",
            Some(Severity::Advice) => "advice",
            _ => "error",
        };
        let mut labels: Vec<LabelReport> = diagnostic
            .labels()
            .map(|labels| {
                labels
                    .map(|label| LabelReport {
                        span: Span {
                            start: to_u32(label.offset()),
                            end: to_u32(label.offset() + label.len()),
                        },
                        text: label.label().map(str::to_owned),
                        primary: label.primary(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        // Every diagnostic reports one primary location: the marked label, or
        // else the first.
        if !labels.iter().any(|label| label.primary)
            && let Some(first) = labels.first_mut()
        {
            first.primary = true;
        }
        Self {
            code: diagnostic.code().map(|code| code.to_string()),
            severity,
            message: diagnostic.to_string(),
            help: diagnostic.help().map(|help| help.to_string()),
            labels,
        }
    }
}

fn token_span(ast: &Ast<'_>, token: TokenIdx) -> mori_span::Span {
    ast.tokens.span(token, ast.source)
}

/// Sources are at most 65,536 bytes, so offsets fit in `u32`.
fn to_u32(offset: usize) -> u32 {
    u32::try_from(offset).expect("source offsets fit in u32")
}


END FILE units/check-json-production.rs


BEGIN FILE units/capabilities-positive-helpers.rs | SHA256 723bd317d6dfe1a2c9063030127230b2ff85d8eb10e4370ad8e161731f78b1ad | BYTES 1876
//! Public capability consumers only; no compiler, target, SDK or native operation.
use serde_json::Value;
use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "mori-capabilities-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        Self(dir)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .env("PATH", "")
            .stdin(Stdio::null())
            .args(["--color", "never"])
            .args(args)
            .output()
            .unwrap()
    }
    fn snapshot(&self) -> Vec<(OsString, Vec<u8>)> {
        let mut files: Vec<_> = fs::read_dir(&self.0)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                (entry.file_name(), fs::read(entry.path()).unwrap())
            })
            .collect();
        files.sort();
        files
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "exit {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn row<'a>(report: &'a Value, section: &str, id: &str) -> &'a Value {
    report[section]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == id)
        .unwrap()
}


END FILE units/capabilities-positive-helpers.rs


BEGIN FILE units/capabilities-positive-test.rs | SHA256 d85df856dfe8c3ff666b2bc16158045e2e0341716d423002257d3ab06554d496 | BYTES 2231
fn reported_core_profiles_select_real_source_only_cli_consumers() {
    let f = Files::new();
    let report = json(&f.run(&["capabilities", "--json"]));
    for (id, language, semantics, body) in [
        (
            "rust-core1",
            "moriarty/1",
            "moriarty-core/1",
            "next count = count + 1;",
        ),
        (
            "rust-core2",
            "moriarty/2",
            "moriarty-core/2",
            "await review(ok: Bool) when ok; next count = count + 1;",
        ),
    ] {
        let profile = row(&report, "profiles", id);
        assert_eq!(profile["sourceProfile"], language);
        assert_eq!(profile["semanticsProfile"], semantics);
        let source = format!(
            "{}\nagreement CapabilityProbe {{ state count: UInt = 0; action run() {{ {body} }} }}\n",
            profile["sourceHeader"].as_str().unwrap()
        );
        fs::write(f.0.join("source.mori"), source).unwrap();
        let before = f.snapshot();
        let checked = json(&f.run(&["check", "source.mori", "--json"]));
        assert_eq!(checked["status"], "accepted");
        assert_eq!(checked["language"], language);
        let inspected = json(&f.run(&["inspect", "source.mori"]));
        assert_eq!(inspected["language"], language);
        assert_eq!(inspected["semantics"], semantics);
        assert_eq!(inspected.get("enforcement_report"), Some(&Value::Null));
        if language == "moriarty/2" {
            assert_eq!(inspected["status"], "accepted");
            assert_eq!(inspected["qualification"], "local-reference-stage");
            assert_eq!(inspected["authentication"], "none");
            assert_eq!(inspected["consumption"], "untracked");
            assert!(row(&report, "consumers", "core2-inspect")["outputProfile"].is_null());
            assert!(inspected.get("format").is_none());
            assert_eq!(inspected["stage_plan"]["format"], "moriarty-stage-plan/1");
        } else {
            assert!(inspected.get("status").is_none());
            assert_eq!(inspected["qualification"], "source-only-unqualified");
            assert_eq!(inspected["format"], "moriarty-enforcement-report/1");
        }
        assert_eq!(before, f.snapshot());
    }
}

END FILE units/capabilities-positive-test.rs


BEGIN FILE units/core2-positive-helpers.rs | SHA256 134f6432e09ace6e416e977c365ed90342385b7ef63d0b0aca64fed8156dbe8f | BYTES 3563
//! Public reference-stage consumers against literals frozen before implementation.
//! No test in this file reaches native, proof, replay, or ledger execution; the
//! H16/H19 `--trace` and `compile-call` requests are refused at the profile first.
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const DELIVERY: &str = include_str!("../../../examples/core/continuations/delivery.mori");
const RESERVE: &str = include_str!("../../../examples/core/continuations/reserve.mori");
const LEGACY: &str = include_str!("../../../examples/core/continuations/atomic-compatibility.mori");
fn oracle() -> Value {
    serde_json::from_str(include_str!(
        "../../../examples/core/continuations/normal-oracle.json"
    ))
    .unwrap()
}
struct Files(PathBuf);
impl Files {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-core-continuations-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        let f = Self(path);
        f.write("source.mori", source.as_bytes());
        f
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
        std::fs::write(self.path(name), bytes).unwrap();
    }
    fn json(&self, name: &str, value: &Value) {
        self.write(name, serde_json::to_vec(value).unwrap());
    }
    fn command(&self, command: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", command])
            .arg(self.path("source.mori"));
        c
    }
    fn start(&self, action: &str, opening: &Value, cap: Option<&str>) -> Output {
        self.json("scenario.json", opening);
        let mut c = self.command("simulate");
        c.args(["--action", action, "--scenario"])
            .arg(self.path("scenario.json"));
        if let Some(cap) = cap {
            c.args(["--logical-cap", cap]);
        }
        c.output().unwrap()
    }
    fn resume(&self, continuation: &Value, event: &Value, action: Option<&str>) -> Output {
        self.json("continuation.json", continuation);
        self.json("event.json", event);
        let mut c = self.command("simulate");
        c.arg("--resume")
            .arg(self.path("continuation.json"))
            .arg("--event")
            .arg(self.path("event.json"));
        if let Some(action) = action {
            c.args(["--action", action]);
        }
        c.output().unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "{e}: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}
fn accepted(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let r = report(output);
    for (key, expected) in oracle()["normal_output_labels_from_opus"]
        .as_object()
        .unwrap()
    {
        assert_eq!(&r[key], expected, "{key}");
    }
    r
}

END FILE units/core2-positive-helpers.rs


BEGIN FILE units/core2-positive-test.rs | SHA256 fd01a5831321b25da9443400fa5f9400539a8295d33ed51afd456fe1a1736fa4 | BYTES 514
fn check_and_source_inspect_route_core_two_without_legacy_consumers() {
    let f = Files::new(RESERVE);
    let output = f.command("check").arg("--json").output().unwrap();
    let checked = accepted(&output);
    assert_eq!(checked["language"], "moriarty/2");
    let inspected = accepted(&f.command("inspect").output().unwrap());
    assert_eq!(inspected["semantics"], "moriarty-core/2");
    assert_eq!(
        inspected["stage_plan"]["actions"][0]["segment_cost"],
        json!(["11", "15", "10"])
    );
}

END FILE units/core2-positive-test.rs


BEGIN FILE source/examples/core/continuations/reserve.mori | SHA256 260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530 | BYTES 758
language "moriarty/2";
agreement ReserveContinuationOracle {
  asset CREDIT = { network: "local", issuer: "continuation-reserve-oracle", reference: "UNIT", scale: 0 };
  state basis: UInt = 40;
  state allocated: UInt = 0;
  state marker: UInt = 77;
  action allocate(extra: UInt) {
    let held = basis + extra;
    next basis = basis - 5;
    liability("allocation", CREDIT, "reserve", "target", held);
    await approval(quote: UInt) when quote >= held;
    next allocated = held;
    transfer(CREDIT, "reserve", "target", held);
    discharge("allocation", held);
    next basis = basis + quote;
    await settlement(ready: Bool) when ready;
    requires quote <= 50;
    fee(CREDIT, "reserve", "treasury", extra);
    emit "settled_quote"(quote);
  }
}

END FILE source/examples/core/continuations/reserve.mori


BEGIN FILE positive-oracle-used-values-and-provenance.json | SHA256 c6d775daaa2308c52b39b479bf819b093e2ac66ca096a1aebec3bd0a976e6a78 | BYTES 5453
{
  "baseline": "fe01bc8b7d889427192c304b0e75d0ec719f63db",
  "checkout": "/home/charl/Moriarty/.worktrees/core-continuations-20261015",
  "cost_contract_sources": [
    {
      "bytes": 33669,
      "path": "/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/src/runtime.rs",
      "sha256": "96e736529a3635d2892bfdc2ae7df72bbc724557d8966ddb0ac7cf51ff9892d2"
    },
    {
      "bytes": 2605,
      "path": "/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/src/logical_cost.rs",
      "sha256": "9c3631eb760ed673d1f21178be07ec3cced7e7ab445e823374bed3a46bf87338"
    },
    {
      "bytes": 6982,
      "path": "/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/src/ir.rs",
      "sha256": "9bc79d6446fcdbeb10f4cefbdf581ad95bd1460e7a2dd5af6081fd5e95197549"
    }
  ],
  "design_sources": {
    "interpretation_scope": "Opus C2/C3 normal outcome/cost changes supplied by root are used as proposed constraints; no independent security interpretation or hostile tests supplied.",
    "opus_constraints": {
      "bytes": 22758,
      "path": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/DESIGN-OPUS.md",
      "sha256": "f611ee6e3e8809d6c4be990c9d782ce5be352efe8d0dc82a0cfc4c7587b9cb00"
    },
    "sol_proposal": {
      "bytes": 12010,
      "path": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/DESIGN-SOL.md",
      "sha256": "a68ca134d0dc6e1c5b22c53920fdd793231943a4e756b0cf0c4e0b7096f360db"
    }
  },
  "freeze_finalized_at_utc": "2026-10-15T04:06:03.116559+00:00",
  "frozen_at_utc": "2026-10-15T04:05:39.905666+00:00",
  "hypothesis_history": [
    {
      "claims": {
        "escrow_start_false_true_cumulative": 13,
        "reserve_start_false_first_true_false_second_true_cumulative": 42
      },
      "meaning": "Original Sol proposal charged false waits into the carried cumulative counter and proposed N false-wait circuits. These numeric hypotheses were communicated to root before C2/C3 steering, not implementation observations.",
      "order": 1,
      "source": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/DESIGN-SOL.md",
      "source_file_mtime_utc": "2026-10-15T03:56:39.004850+00:00",
      "status": "superseded unadopted Sol-only proposal; retained before any product execution"
    },
    {
      "changes": "False observations do not commit, leave continuation identical and have no circuit. Committed cost is successful segment plan constant. Escrow committed11/attempted13 after false, reserve committed36/attempted42 after two false observations.",
      "constraint_ids": [
        "C2",
        "C3"
      ],
      "order": 2,
      "source": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/DESIGN-OPUS.md",
      "source_file_mtime_utc": "2026-10-15T04:00:38.327091+00:00",
      "status": "active conditional oracle, still unadopted"
    }
  ],
  "normal_output_labels_from_opus": {
    "authentication": "none",
    "consumption": "untracked",
    "qualification": "local-reference-stage"
  },
  "oracle_method": "Manually chosen integers, literal state/effect frames and syntax-node cost counts from source contract. No compiler/evaluator/model imported or run; Python only serialized literal arithmetic observations and hashes.",
  "schema": "moriarty-independent-continuation-normal-oracle/1",
  "source_artifacts": [
    {
      "bytes": 452,
      "path": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/oracle-escrow-v2.mori",
      "sha256": "2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a"
    },
    {
      "bytes": 758,
      "path": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/oracle-reserve-v2.mori",
      "sha256": "260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530"
    },
    {
      "bytes": 448,
      "path": "/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/oracle-compat-v1.mori",
      "sha256": "4ad04eed61ba6349a395c9b60984193e0a8c178151d2d23896d7f95d118d6b85"
    }
  ],
  "status": "Specified-only; /2 semantics still unadopted pending revised Sol vote and Muse audit. No code or product execution.",
  "uncertainties": [
    "No adopted implementation currently accepts the proposed /2 files; they are concrete proposed source specimens, not parse/run evidence.",
    "Exact program_hash/plan_hash require the future canonical parsed Program/StagePlan; no guessed hashes or compiler-derived expectations are supplied.",
    "Continuation/pending-frame wire encoding is not re-created here. pending_frame rows are conceptual fixed ABI values separate from declared Snapshot.state; exact output integration follows adopted schema.",
    "Source_step rows name statements and values, but do not invent future Await ExprId allocation or serialized AST fields.",
    "Residual rows are positive liability financial identities; exact Residual Rust/JSON field names remain implementation/adoption details.",
    "No costs are assigned to serialization, capture encoding, circuit operations, DUST, service fees, proof or ledger work.",
    "Security/native/target/once-only/authority claims are outside this oracle and remain Opus-owned.",
    "No cap-exhaustion/hostile/altered-continuation test is specified here. The quote51 case is a normal business predicate refusal from unmodified source."
  ]
}

END FILE positive-oracle-used-values-and-provenance.json


BEGIN FILE source/crates/mori/src/commands/capabilities.rs | SHA256 800fe8007fe12c719128ee51b40273ea89dbc2457eebf8d28c4d9903d9a99380 | BYTES 9928
//! Curated public source capabilities, with this binary's actual Cargo features.
//! This query does not inspect files, targets, installations or runtime evidence.
use std::process::ExitCode;

use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Print JSON (the only output format)
    #[arg(long)]
    pub json: bool,
}

// Inspection basis for the curated consumer notes, not a binary/build identity.
// Update this catalogue when public routing or editor implementations change.
const SOURCE_REVISION: &str = "e4d7a2b5709caa18a94f5256d03a0302364b8235";

fn profile(
    id: &str,
    source: &str,
    semantics: Option<&str>,
    parser: &str,
    header_keyword: &str,
    types: &[&str],
) -> Value {
    json!({
        "id": id, "sourceProfile": source,
        "sourceHeader": format!("{header_keyword} \"{source}\";"),
        "semanticsProfile": semantics, "parser": parser,
        "implementation": "source-present",
        "grammar": {
            "coverage": "profile-selection-summary-not-grammar-completion",
            "types": types, "declarations": [], "statements": [],
            "expressions": [], "unsupported": [], "bounds": {},
            "notes": ["Grammar is not enumerated here; use the actual parser/checker for admission."]
        },
        "qualification": "source-description-only",
        "limitations": ["Parser/checker support does not imply compiler or target qualification."]
    })
}

/// Metadata only. The profile tuple is (input, semantics, output); an optional
/// build requirement carries its actual cfg value, not a runtime probe.
fn consumer(
    id: &str,
    entrypoint: &str,
    profiles: (Option<&str>, Option<&str>, Option<&str>),
    build: Option<(&str, bool)>,
    status: (&str, &str),
    limitations: &[&str],
    source_refs: &[&str],
) -> Value {
    let requirements: Vec<_> = build.iter().map(|(name, _)| *name).collect();
    json!({
        "id": id, "entrypoint": entrypoint,
        "inputProfile": profiles.0, "semanticsProfile": profiles.1,
        "outputProfile": profiles.2, "requiresBuildFeatures": requirements,
        "implementation": status.0,
        "availableInBuild": status.0 == "source-present" && build.is_none_or(|(_, enabled)| enabled),
        "qualification": status.1, "limitations": limitations, "sourceRefs": source_refs
    })
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    use mori_core::stage::{LANGUAGE_2, SEMANTICS_2};
    use mori_core::{LANGUAGE, SEMANTICS};

    // JSON is the sole format; the flag is an explicit client spelling.
    let _ = args.json;
    let native = cfg!(feature = "native");
    let source6 = cfg!(feature = "source6-expansion");
    let disjoint = cfg!(feature = "core2-disjoint-draft");
    let report = json!({
        "format": "moriarty-cli-capabilities/1",
        "producer": {
            "implementation": "rust-workspace-cli", "packageVersion": env!("CARGO_PKG_VERSION"),
            "binarySourceRevision": null, "responseKind": "command-self-report",
            "buildFeatures": {"native": native, "source6-expansion": source6, "core2-disjoint-draft": disjoint}
        },
        "catalogue": {
            "sourceRevision": SOURCE_REVISION, "coverage": "selected-public-profile-consumers",
            "basis": "source-inspection", "runtimeEvidence": "not-included",
            "note": "Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update."
        },
        "profiles": [
            profile("rust-core1", LANGUAGE, Some(SEMANTICS), "mori_core::parse", "language", &["UInt", "Bool"]),
            profile("rust-core2", LANGUAGE_2, Some(SEMANTICS_2), "mori_core::stage::load", "language", &["UInt", "Bool"]),
            profile("rust-beta-frontend", mori_parser::PROFILE, None, "mori_parser::parse then mori_checker::check", "profile", &[])
        ],
        "consumers": [
            consumer("core1-check", "check FILE --json", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "source-check"), &["No compilation or target qualification."], &["crates/mori/src/commands/check.rs"]),
            consumer("core2-check", "check FILE --json", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "source-check"), &["Uses staged admission and planning."], &["crates/mori/src/commands/check.rs"]),
            consumer("beta-check", "check FILE --json", (Some(mori_parser::PROFILE), None, None), None,
                ("source-present", "frontend-check-only"), &["Named-action admission is determined by the checker; no Core semantics are inferred."], &["crates/mori/src/commands/check.rs"]),
            consumer("core1-reference", "simulate FILE --action NAME --scenario FILE", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "local-reference-execution"), &["--trace is a separate native-feature consumer."], &["crates/mori/src/commands/simulate.rs"]),
            consumer("core2-reference", "simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.", "Authentication is none; consumption is untracked."], &["crates/mori/src/commands/simulate.rs", "crates/mori-core/src/stage/mod.rs"]),
            consumer("core1-inspect", "inspect FILE", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-enforcement-report/1")), None,
                ("source-present", "source-only-unqualified"), &["No target was supplied or compiled."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-inspect", "inspect FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-target-inspect", "inspect FILE --target FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), Some(("native", native)),
                ("unsupported", "profile-refused"), &["Core/2 target inspection refuses even though Core/2 has a separate compiler."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-disjoint-draft", "test DIRECTORY", (Some("moriarty-tests/disjoint-draft/1"), Some(SEMANTICS_2), None), Some(("core2-disjoint-draft", disjoint)),
                ("source-present", "unadopted-local-disjoint-product"), &["Opt-in draft; not required for base Core/2 check, start or resume."], &["crates/mori/src/commands/test.rs"]),
            consumer("beta-to-source6-transfer", "expand FILE --emit-source6-transfer --action NAME --scenario FILE", (Some(mori_parser::PROFILE), None, Some("Source/6")), Some(("source6-expansion", source6)),
                ("source-present", "local-stipulation-only"), &["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.", "Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."], &["crates/mori/src/commands/expand.rs", "crates/mori/src/commands/expand/source6.rs"]),
            consumer("core1-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-zkir-artifact/3")), Some(("native", native)),
                ("source-present", "compiler-source-present"), &["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."], &["crates/mori/src/commands/compile.rs"]),
            consumer("core2-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE_2), Some(SEMANTICS_2), Some("moriarty-zkir-artifact/4")), Some(("native", native)),
                ("source-present", "staged-compiler-source-present"), &["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.", "Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."], &["crates/mori/src/commands/compile.rs", "crates/mori-backend/src/stage/mod.rs"]),
            consumer("rust-lsp", "lsp", (None, None, None), None,
                ("stub", "not-implemented"), &["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."], &["crates/mori/src/commands/lsp.rs"]),
            consumer("rust-mcp", "mcp", (None, None, None), None,
                ("stub", "not-implemented"), &["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."], &["crates/mori/src/commands/mcp.rs"])
        ],
        "targets": [{
            "id": "midnight-ledger9-zkirv3", "consumerIds": ["core1-compile", "core2-compile"],
            "descriptorRequired": true, "configuredTarget": null, "qualification": "not-checked",
            "notes": ["No target, network, anchor, parameters, prover, verifier or deployment was inspected."]
        }],
        "otherExecutables": [{
            "id": "node-beta-cli", "package": "@moriarty-lang/beta", "binaryName": "mori",
            "relationship": "separate executable; no Rust capability inheritance", "availability": "not-inspected"
        }],
        "limitations": [
            "availableInBuild means the source path is compiled in, not that a request passed admission or execution.",
            "This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.",
            "No runtime, proof, installation, deployment or ledger qualification is established."
        ]
    });
    super::core_io::print(&report)?;
    Ok(ExitCode::SUCCESS)
}

END FILE source/crates/mori/src/commands/capabilities.rs


BEGIN FILE POSITIVE-CONTROL-SCOPE.json | SHA256 9a09c7b253a12842d48a651725b737524a0fcc1314931859948db50d53e6b8b9 | BYTES 1324
{
  "core1Control": "reported_core_profiles_select_real_source_only_cli_consumers performs capabilitiesJSON + bothCore1/Core2 checkJSON/sourceinspect; all literal expectation fields, headers, generated program body and file-preservation comparisons included in complete selected test and helpers. No full report byte oracle or native target qualification.",
  "core2Control": "check_and_source_inspect_route_core_two_without_legacy_consumers uses RESERVE source, accepted helper and normal_output_labels_from_opus only, plus literal stage_plan.actions[0].segment_cost [11,15,10]. Complete selected helper/test/source/used JSON values mandatory.",
  "fullOriginalOracle": "positive-custody/examples/core/continuations/normal-oracle.json",
  "fullOriginalOracleSha256": "76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640",
  "mandatoryUsedValues": "positive-oracle-used-values-and-provenance.json",
  "unusedCustodyOnlyOracleTopLevelKeys": [
    "reserve",
    "compatibility_v1",
    "escrow"
  ],
  "scopeQualification": "No value read by either executed selected test is omitted; complete unused oracle branches/full unrelated test bodies preserved physically but not mandatory model reading. Existing external fixture paths remain original guideline deviations. No new independent financial/work oracle."
}

END FILE POSITIVE-CONTROL-SCOPE.json


BEGIN FILE units/miette-graphical-handler-types.rs | SHA256 0fe9c622f285b27c1619555a2736c07cea545f4944d1fe54b602af3ff0487d77 | BYTES 1717
use std::fmt::{self, Write};

use owo_colors::{OwoColorize, Style, StyledList};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::diagnostic_chain::{DiagnosticChain, ErrorKind};
use crate::handlers::theme::*;
use crate::highlighters::{Highlighter, MietteHighlighter};
use crate::protocol::{Diagnostic, Severity};
use crate::{LabeledSpan, ReportHandler, SourceCode, SourceSpan, SpanContents};

/**
A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`] of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.
*/
#[derive(Debug, Clone)]
pub struct GraphicalReportHandler {
    pub(crate) links: LinkStyle,
    pub(crate) termwidth: usize,
    pub(crate) theme: GraphicalTheme,
    pub(crate) footer: Option<String>,
    pub(crate) context_lines: usize,
    pub(crate) tab_width: usize,
    pub(crate) with_cause_chain: bool,
    pub(crate) wrap_lines: bool,
    pub(crate) break_words: bool,
    pub(crate) with_primary_span_start: bool,
    pub(crate) word_separator: Option<textwrap::WordSeparator>,
    pub(crate) word_splitter: Option<textwrap::WordSplitter>,
    pub(crate) highlighter: MietteHighlighter,
    pub(crate) link_display_text: Option<String>,
    pub(crate) show_related_as_nested: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinkStyle {
    None,
    Link,
    Text,
}

END FILE units/miette-graphical-handler-types.rs


BEGIN FILE units/miette-new-themed.rs | SHA256 a0418146dbab6e4d245629dc94b1f8590b1c919c826af972dcd91876778eba7c | BYTES 593
    pub fn new_themed(theme: GraphicalTheme) -> Self {
        Self {
            links: LinkStyle::Link,
            termwidth: 200,
            theme,
            footer: None,
            context_lines: 1,
            tab_width: 4,
            wrap_lines: true,
            with_cause_chain: true,
            with_primary_span_start: true,
            break_words: true,
            word_separator: None,
            word_splitter: None,
            highlighter: MietteHighlighter::default(),
            link_display_text: None,
            show_related_as_nested: false,
        }
    }

END FILE units/miette-new-themed.rs


BEGIN FILE units/miette-with-links.rs | SHA256 f39509e7c0bf3d3cdb14e960d53ab8924f767f48f831257c1a95cb0a4f9b63d9 | BYTES 190
    pub fn with_links(mut self, links: bool) -> Self {
        self.links = if links {
            LinkStyle::Link
        } else {
            LinkStyle::Text
        };
        self
    }

END FILE units/miette-with-links.rs


BEGIN FILE units/miette-with-width.rs | SHA256 d1c7ef18f0768038cf068be5ddd7f06ab0388e247ad568c6a3b474eb861b89c9 | BYTES 107
    pub fn with_width(mut self, width: usize) -> Self {
        self.termwidth = width;
        self
    }

END FILE units/miette-with-width.rs


BEGIN FILE units/miette-render-report.rs | SHA256 b85413d207d58b0620fb5572526025dfc899320970872a8e3cf2802fb21a2fda | BYTES 216
    pub fn render_report(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
    ) -> fmt::Result {
        self.render_report_inner(f, diagnostic, diagnostic.source_code())
    }

END FILE units/miette-render-report.rs


BEGIN FILE units/miette-render-report-inner.rs | SHA256 1e5f6beb356eb45119adaaefbc7f523970ca492077a38150030ac24c3b2ed7a7 | BYTES 1169
    fn render_report_inner(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        parent_src: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let src = diagnostic.source_code().or(parent_src);
        self.render_header(f, diagnostic, false)?;
        self.render_causes(f, diagnostic, src)?;
        self.render_snippets(f, diagnostic, src)?;
        self.render_footer(f, diagnostic)?;
        self.render_related(f, diagnostic, src)?;
        if let Some(footer) = &self.footer {
            writeln!(f)?;
            let width = self.termwidth.saturating_sub(2);
            let mut opts = textwrap::Options::new(width)
                .initial_indent("  ")
                .subsequent_indent("  ")
                .break_words(self.break_words);
            if let Some(word_separator) = self.word_separator {
                opts = opts.word_separator(word_separator);
            }
            if let Some(word_splitter) = self.word_splitter.clone() {
                opts = opts.word_splitter(word_splitter);
            }

            writeln!(f, "{}", self.wrap(footer, opts))?;
        }
        Ok(())
    }

END FILE units/miette-render-report-inner.rs


BEGIN FILE units/miette-render-header.rs | SHA256 a51c2ea70dd106980305f46faa6ece57bbb686aef70ad5ce18f0230b89a9054b | BYTES 1752
    fn render_header(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        is_nested: bool,
    ) -> fmt::Result {
        let severity_style = match diagnostic.severity() {
            Some(Severity::Error) | None => self.theme.styles.error,
            Some(Severity::Warning) => self.theme.styles.warning,
            Some(Severity::Advice) => self.theme.styles.advice,
        };
        let mut header = String::new();
        let mut need_newline = is_nested;
        if self.links == LinkStyle::Link && diagnostic.url().is_some() {
            let url = diagnostic.url().unwrap(); // safe
            let code = if let Some(code) = diagnostic.code() {
                format!("{} ", code)
            } else {
                "".to_string()
            };
            let display_text = self.link_display_text.as_deref().unwrap_or("(link)");
            let link = format!(
                "\u{1b}]8;;{}\u{1b}\\{}{}\u{1b}]8;;\u{1b}\\",
                url,
                code.style(severity_style),
                display_text.style(self.theme.styles.link)
            );
            write!(header, "{}", link)?;
            writeln!(f, "{}", header)?;
            need_newline = true;
        } else if let Some(code) = diagnostic.code() {
            write!(header, "{}", code.style(severity_style),)?;
            if self.links == LinkStyle::Text && diagnostic.url().is_some() {
                let url = diagnostic.url().unwrap(); // safe
                write!(header, " ({})", url.style(self.theme.styles.link))?;
            }
            writeln!(f, "{}", header)?;
            need_newline = true;
        }
        if need_newline {
            writeln!(f)?;
        }
        Ok(())
    }

END FILE units/miette-render-header.rs


BEGIN FILE units/miette-render-causes.rs | SHA256 12a7a08435245f872af1288ba569df21f1708a843311e21b300f1d186bbf934c | BYTES 4291
    fn render_causes(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        parent_src: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let src = diagnostic.source_code().or(parent_src);

        let (severity_style, severity_icon) = match diagnostic.severity() {
            Some(Severity::Error) | None => (self.theme.styles.error, &self.theme.characters.error),
            Some(Severity::Warning) => (self.theme.styles.warning, &self.theme.characters.warning),
            Some(Severity::Advice) => (self.theme.styles.advice, &self.theme.characters.advice),
        };

        let initial_indent = format!("  {} ", severity_icon.style(severity_style));
        let rest_indent = format!("  {} ", self.theme.characters.vbar.style(severity_style));
        let width = self.termwidth.saturating_sub(2);
        let mut opts = textwrap::Options::new(width)
            .initial_indent(&initial_indent)
            .subsequent_indent(&rest_indent)
            .break_words(self.break_words);
        if let Some(word_separator) = self.word_separator {
            opts = opts.word_separator(word_separator);
        }
        if let Some(word_splitter) = self.word_splitter.clone() {
            opts = opts.word_splitter(word_splitter);
        }

        writeln!(f, "{}", self.wrap(&diagnostic.to_string(), opts))?;

        if !self.with_cause_chain {
            return Ok(());
        }

        if let Some(mut cause_iter) = diagnostic
            .diagnostic_source()
            .map(DiagnosticChain::from_diagnostic)
            .or_else(|| diagnostic.source().map(DiagnosticChain::from_stderror))
            .map(|it| it.peekable())
        {
            while let Some(error) = cause_iter.next() {
                let is_last = cause_iter.peek().is_none();
                let char = if !is_last {
                    self.theme.characters.lcross
                } else {
                    self.theme.characters.lbot
                };
                let initial_indent = format!(
                    "  {}{}{} ",
                    char, self.theme.characters.hbar, self.theme.characters.rarrow
                )
                .style(severity_style)
                .to_string();
                let rest_indent = format!(
                    "  {}   ",
                    if is_last {
                        ' '
                    } else {
                        self.theme.characters.vbar
                    }
                )
                .style(severity_style)
                .to_string();
                let mut opts = textwrap::Options::new(width)
                    .initial_indent(&initial_indent)
                    .subsequent_indent(&rest_indent)
                    .break_words(self.break_words);
                if let Some(word_separator) = self.word_separator {
                    opts = opts.word_separator(word_separator);
                }
                if let Some(word_splitter) = self.word_splitter.clone() {
                    opts = opts.word_splitter(word_splitter);
                }

                match error {
                    ErrorKind::Diagnostic(diag) => {
                        let mut inner = String::new();

                        let mut inner_renderer = self.clone();
                        // Don't print footer for inner errors
                        inner_renderer.footer = None;
                        // Cause chains are already flattened, so don't double-print the nested error
                        inner_renderer.with_cause_chain = false;
                        // Since everything from here on is indented, shrink the virtual terminal
                        inner_renderer.termwidth -= rest_indent.width();
                        inner_renderer.render_report_inner(&mut inner, diag, src)?;

                        // If there was no header, remove the leading newline
                        let inner = inner.trim_start_matches('\n');
                        writeln!(f, "{}", self.wrap(inner, opts))?;
                    }
                    ErrorKind::StdError(err) => {
                        writeln!(f, "{}", self.wrap(&err.to_string(), opts))?;
                    }
                }
            }
        }

        Ok(())
    }

END FILE units/miette-render-causes.rs


BEGIN FILE units/miette-render-footer.rs | SHA256 d6f42b639ccfc1fe64393362931f6e3b7c0b2e9dbf0801c1abe1c0679a5d24b5 | BYTES 875
    fn render_footer(&self, f: &mut impl fmt::Write, diagnostic: &(dyn Diagnostic)) -> fmt::Result {
        if let Some(help) = diagnostic.help() {
            let width = self.termwidth.saturating_sub(2);
            let initial_indent = "  help: ".style(self.theme.styles.help).to_string();
            let mut opts = textwrap::Options::new(width)
                .initial_indent(&initial_indent)
                .subsequent_indent("        ")
                .break_words(self.break_words);
            if let Some(word_separator) = self.word_separator {
                opts = opts.word_separator(word_separator);
            }
            if let Some(word_splitter) = self.word_splitter.clone() {
                opts = opts.word_splitter(word_splitter);
            }

            writeln!(f, "{}", self.wrap(&help.to_string(), opts))?;
        }
        Ok(())
    }

END FILE units/miette-render-footer.rs


BEGIN FILE units/miette-render-related.rs | SHA256 03fc636df776fe74f10ecd5d703fc63d3d9f101501f01584fa715f416d39adef | BYTES 3862
    fn render_related(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        parent_src: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let src = diagnostic.source_code().or(parent_src);

        if let Some(related) = diagnostic.related() {
            let severity_style = match diagnostic.severity() {
                Some(Severity::Error) | None => self.theme.styles.error,
                Some(Severity::Warning) => self.theme.styles.warning,
                Some(Severity::Advice) => self.theme.styles.advice,
            };

            let mut inner_renderer = self.clone();
            // Re-enable the printing of nested cause chains for related errors
            inner_renderer.with_cause_chain = true;
            if self.show_related_as_nested {
                let width = self.termwidth.saturating_sub(2);
                let mut related = related.peekable();
                while let Some(rel) = related.next() {
                    let is_last = related.peek().is_none();
                    let char = if !is_last {
                        self.theme.characters.lcross
                    } else {
                        self.theme.characters.lbot
                    };
                    let initial_indent = format!(
                        "  {}{}{} ",
                        char, self.theme.characters.hbar, self.theme.characters.rarrow
                    )
                    .style(severity_style)
                    .to_string();
                    let rest_indent = format!(
                        "  {}   ",
                        if is_last {
                            ' '
                        } else {
                            self.theme.characters.vbar
                        }
                    )
                    .style(severity_style)
                    .to_string();

                    let mut opts = textwrap::Options::new(width)
                        .initial_indent(&initial_indent)
                        .subsequent_indent(&rest_indent)
                        .break_words(self.break_words);
                    if let Some(word_separator) = self.word_separator {
                        opts = opts.word_separator(word_separator);
                    }
                    if let Some(word_splitter) = self.word_splitter.clone() {
                        opts = opts.word_splitter(word_splitter);
                    }

                    let mut inner = String::new();

                    let mut inner_renderer = self.clone();
                    inner_renderer.footer = None;
                    inner_renderer.with_cause_chain = false;
                    inner_renderer.termwidth -= rest_indent.width();
                    inner_renderer.render_report_inner(&mut inner, rel, src)?;

                    // If there was no header, remove the leading newline
                    let inner = inner.trim_matches('\n');
                    writeln!(f, "{}", self.wrap(inner, opts))?;
                }
            } else {
                for rel in related {
                    writeln!(f)?;
                    match rel.severity() {
                        Some(Severity::Error) | None => write!(f, "Error: ")?,
                        Some(Severity::Warning) => write!(f, "Warning: ")?,
                        Some(Severity::Advice) => write!(f, "Advice: ")?,
                    };
                    inner_renderer.render_header(f, rel, true)?;
                    let src = rel.source_code().or(parent_src);
                    inner_renderer.render_causes(f, rel, src)?;
                    inner_renderer.render_snippets(f, rel, src)?;
                    inner_renderer.render_footer(f, rel)?;
                    inner_renderer.render_related(f, rel, src)?;
                }
            }
        }
        Ok(())
    }

END FILE units/miette-render-related.rs


BEGIN FILE units/miette-render-snippets.rs | SHA256 500e333da03b577f69e9bdde09429eb8e7321dd8befa38fcffc6aaac7deb9a8e | BYTES 2963
    fn render_snippets(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        opt_source: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let source = match opt_source {
            Some(source) => source,
            None => return Ok(()),
        };
        let labels = match diagnostic.labels() {
            Some(labels) => labels,
            None => return Ok(()),
        };

        let mut labels = labels.collect::<Vec<_>>();
        labels.sort_unstable_by_key(|l| l.inner().offset());

        let mut contexts = Vec::with_capacity(labels.len());
        for right in labels.iter().cloned() {
            let right_conts =
                match source.read_span(right.inner(), self.context_lines, self.context_lines) {
                    Ok(cont) => cont,
                    Err(err) => {
                        writeln!(
                            f,
                            "  [{} `{}` (offset: {}, length: {}): {:?}]",
                            "Failed to read contents for label".style(self.theme.styles.error),
                            right
                                .label()
                                .unwrap_or("<none>")
                                .style(self.theme.styles.link),
                            right.offset().style(self.theme.styles.link),
                            right.len().style(self.theme.styles.link),
                            err.style(self.theme.styles.warning)
                        )?;
                        return Ok(());
                    }
                };

            if contexts.is_empty() {
                contexts.push((right, right_conts));
                continue;
            }

            let (left, left_conts) = contexts.last().unwrap();
            if left_conts.line() + left_conts.line_count() >= right_conts.line() {
                // The snippets will overlap, so we create one Big Chunky Boi
                let left_end = left.offset() + left.len();
                let right_end = right.offset() + right.len();
                let new_end = std::cmp::max(left_end, right_end);

                let new_span = LabeledSpan::new(
                    left.label().map(String::from),
                    left.offset(),
                    new_end - left.offset(),
                );
                // Check that the two contexts can be combined
                if let Ok(new_conts) =
                    source.read_span(new_span.inner(), self.context_lines, self.context_lines)
                {
                    contexts.pop();
                    // We'll throw the contents away later
                    contexts.push((new_span, new_conts));
                    continue;
                }
            }

            contexts.push((right, right_conts));
        }
        for (ctx, _) in contexts {
            self.render_context(f, source, &ctx, &labels[..])?;
        }

        Ok(())
    }

END FILE units/miette-render-snippets.rs


BEGIN FILE units/miette-wrap.rs | SHA256 baf49f23157793c716b7e853bb13ec35249f63f100a2a5d088bf19ba419e27b4 | BYTES 1300
    fn wrap(&self, text: &str, opts: textwrap::Options<'_>) -> String {
        if self.wrap_lines {
            textwrap::fill(text, opts)
        } else {
            // Format without wrapping, but retain the indentation options
            // Implementation based on `textwrap::indent`
            let mut result = String::with_capacity(2 * text.len());
            let trimmed_indent = opts.subsequent_indent.trim_end();
            for (idx, line) in text.split_terminator('\n').enumerate() {
                if idx > 0 {
                    result.push('\n');
                }
                if idx == 0 {
                    if line.trim().is_empty() {
                        result.push_str(opts.initial_indent.trim_end());
                    } else {
                        result.push_str(opts.initial_indent);
                    }
                } else if line.trim().is_empty() {
                    result.push_str(trimmed_indent);
                } else {
                    result.push_str(opts.subsequent_indent);
                }
                result.push_str(line);
            }
            if text.ends_with('\n') {
                // split_terminator will have eaten the final '\n'.
                result.push('\n');
            }
            result
        }
    }

END FILE units/miette-wrap.rs


BEGIN FILE renderer/miette-7.6.0/src/handlers/theme.rs | SHA256 befe7bbd48805e2fd1e792c46926478e11eb6b3f829ea47d335f9b2d4ad57907 | BYTES 7978
use std::io::IsTerminal;

use owo_colors::Style;

/**
Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler) to
render fancy [`Diagnostic`](crate::Diagnostic) reports.

A theme consists of two things: the set of characters to be used for drawing,
and the
[`owo_colors::Style`](https://docs.rs/owo-colors/latest/owo_colors/struct.Style.html)s to be used to paint various items.

You can create your own custom graphical theme using this type, or you can use
one of the predefined ones using the methods below.
*/
#[derive(Debug, Clone)]
pub struct GraphicalTheme {
    /// Characters to be used for drawing.
    pub characters: ThemeCharacters,
    /// Styles to be used for painting.
    pub styles: ThemeStyles,
}

impl GraphicalTheme {
    /// ASCII-art-based graphical drawing, with ANSI styling.
    pub fn ascii() -> Self {
        Self {
            characters: ThemeCharacters::ascii(),
            styles: ThemeStyles::ansi(),
        }
    }

    /// Graphical theme that draws using both ansi colors and unicode
    /// characters.
    ///
    /// Note that full rgb colors aren't enabled by default because they're
    /// an accessibility hazard, especially in the context of terminal themes
    /// that can change the background color and make hardcoded colors illegible.
    /// Such themes typically remap ansi codes properly, treating them more
    /// like CSS classes than specific colors.
    pub fn unicode() -> Self {
        Self {
            characters: ThemeCharacters::unicode(),
            styles: ThemeStyles::ansi(),
        }
    }

    /// Graphical theme that draws in monochrome, while still using unicode
    /// characters.
    pub fn unicode_nocolor() -> Self {
        Self {
            characters: ThemeCharacters::unicode(),
            styles: ThemeStyles::none(),
        }
    }

    /// A "basic" graphical theme that skips colors and unicode characters and
    /// just does monochrome ascii art. If you want a completely non-graphical
    /// rendering of your [`Diagnostic`](crate::Diagnostic)s, check out
    /// [`NarratableReportHandler`](crate::NarratableReportHandler), or write
    /// your own [`ReportHandler`](crate::ReportHandler)
    pub fn none() -> Self {
        Self {
            characters: ThemeCharacters::ascii(),
            styles: ThemeStyles::none(),
        }
    }
}

impl Default for GraphicalTheme {
    fn default() -> Self {
        match std::env::var("NO_COLOR") {
            _ if !std::io::stdout().is_terminal() || !std::io::stderr().is_terminal() => {
                Self::none()
            }
            Ok(string) if string != "0" => Self::unicode_nocolor(),
            _ => Self::unicode(),
        }
    }
}

/**
Styles for various parts of graphical rendering for the
[`GraphicalReportHandler`](crate::GraphicalReportHandler).
*/
#[derive(Debug, Clone)]
pub struct ThemeStyles {
    /// Style to apply to things highlighted as "error".
    pub error: Style,
    /// Style to apply to things highlighted as "warning".
    pub warning: Style,
    /// Style to apply to things highlighted as "advice".
    pub advice: Style,
    /// Style to apply to the help text.
    pub help: Style,
    /// Style to apply to filenames/links/URLs.
    pub link: Style,
    /// Style to apply to line numbers.
    pub linum: Style,
    /// Styles to cycle through (using `.iter().cycle()`), to render the lines
    /// and text for diagnostic highlights.
    pub highlights: Vec<Style>,
}

fn style() -> Style {
    Style::new()
}

impl ThemeStyles {
    /// Nice RGB colors.
    /// [Credit](http://terminal.sexy/#FRUV0NDQFRUVrEFCkKlZ9L91ap-1qnWfdbWq0NDQUFBQrEFCkKlZ9L91ap-1qnWfdbWq9fX1).
    pub fn rgb() -> Self {
        Self {
            error: style().fg_rgb::<255, 30, 30>(),
            warning: style().fg_rgb::<244, 191, 117>(),
            advice: style().fg_rgb::<106, 159, 181>(),
            help: style().fg_rgb::<106, 159, 181>(),
            link: style().fg_rgb::<92, 157, 255>().underline().bold(),
            linum: style().dimmed(),
            highlights: vec![
                style().fg_rgb::<246, 87, 248>(),
                style().fg_rgb::<30, 201, 212>(),
                style().fg_rgb::<145, 246, 111>(),
            ],
        }
    }

    /// ANSI color-based styles.
    pub fn ansi() -> Self {
        Self {
            error: style().red(),
            warning: style().yellow(),
            advice: style().cyan(),
            help: style().cyan(),
            link: style().cyan().underline().bold(),
            linum: style().dimmed(),
            highlights: vec![
                style().magenta().bold(),
                style().yellow().bold(),
                style().green().bold(),
            ],
        }
    }

    /// No styling. Just regular ol' monochrome.
    pub fn none() -> Self {
        Self {
            error: style(),
            warning: style(),
            advice: style(),
            help: style(),
            link: style(),
            linum: style(),
            highlights: vec![style()],
        }
    }
}

// ----------------------------------------
// Most of these characters were taken from
// https://github.com/zesterer/ariadne/blob/e3cb394cb56ecda116a0a1caecd385a49e7f6662/src/draw.rs

/// Characters to be used when drawing when using
/// [`GraphicalReportHandler`](crate::GraphicalReportHandler).
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThemeCharacters {
    pub hbar: char,
    pub vbar: char,
    pub xbar: char,
    pub vbar_break: char,

    pub uarrow: char,
    pub rarrow: char,

    pub ltop: char,
    pub mtop: char,
    pub rtop: char,
    pub lbot: char,
    pub rbot: char,
    pub mbot: char,

    pub lbox: char,
    pub rbox: char,

    pub lcross: char,
    pub rcross: char,

    pub underbar: char,
    pub underline: char,

    pub error: String,
    pub warning: String,
    pub advice: String,
}

impl ThemeCharacters {
    /// Fancy unicode-based graphical elements.
    pub fn unicode() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            xbar: '┼',
            vbar_break: '·',
            uarrow: '▲',
            rarrow: '▶',
            ltop: '╭',
            mtop: '┬',
            rtop: '╮',
            lbot: '╰',
            mbot: '┴',
            rbot: '╯',
            lbox: '[',
            rbox: ']',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
            error: "×".into(),
            warning: "⚠".into(),
            advice: "☞".into(),
        }
    }

    /// Emoji-heavy unicode characters.
    pub fn emoji() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            xbar: '┼',
            vbar_break: '·',
            uarrow: '▲',
            rarrow: '▶',
            ltop: '╭',
            mtop: '┬',
            rtop: '╮',
            lbot: '╰',
            mbot: '┴',
            rbot: '╯',
            lbox: '[',
            rbox: ']',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
            error: "💥".into(),
            warning: "⚠️".into(),
            advice: "💡".into(),
        }
    }
    /// ASCII-art-based graphical elements. Works well on older terminals.
    pub fn ascii() -> Self {
        Self {
            hbar: '-',
            vbar: '|',
            xbar: '+',
            vbar_break: ':',
            uarrow: '^',
            rarrow: '>',
            ltop: ',',
            mtop: 'v',
            rtop: '.',
            lbot: '`',
            mbot: '^',
            rbot: '\'',
            lbox: '[',
            rbox: ']',
            lcross: '|',
            rcross: '|',
            underbar: '|',
            underline: '^',
            error: "x".into(),
            warning: "!".into(),
            advice: ">".into(),
        }
    }
}

END FILE renderer/miette-7.6.0/src/handlers/theme.rs


BEGIN FILE renderer/textwrap-0.16.4/src/options.rs | SHA256 086bad3aabfaec2cf654d60688e265146e2113fa6c218e4c428b2d629c7818ad | BYTES 10885
//! Options for wrapping text.

use crate::{LineEnding, WordSeparator, WordSplitter, WrapAlgorithm};

/// Holds configuration options for wrapping and filling text.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Options<'a> {
    /// The width in columns at which the text will be wrapped.
    pub width: usize,
    /// Line ending used for breaking lines.
    pub line_ending: LineEnding,
    /// Indentation used for the first line of output. See the
    /// [`Options::initial_indent`] method.
    pub initial_indent: &'a str,
    /// Indentation used for subsequent lines of output. See the
    /// [`Options::subsequent_indent`] method.
    pub subsequent_indent: &'a str,
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to `false`, some lines may be longer than
    /// `self.width`. See the [`Options::break_words`] method.
    pub break_words: bool,
    /// Wrapping algorithm to use, see the [`WrapAlgorithm`] enum for
    /// details.
    pub wrap_algorithm: WrapAlgorithm,
    /// The line breaking algorithm to use, see the [`WordSeparator`]
    /// enum for an overview and possible implementations.
    pub word_separator: WordSeparator,
    /// The method for splitting words. This can be used to prohibit
    /// splitting words on hyphens, or it can be used to implement
    /// language-aware machine hyphenation.
    pub word_splitter: WordSplitter,
    /// Allow trailing spaces to be preserved at the end of the line.
    pub preserve_trailing_space: bool,
}

impl<'a> From<&'a Options<'a>> for Options<'a> {
    fn from(options: &'a Options<'a>) -> Self {
        Self {
            width: options.width,
            line_ending: options.line_ending,
            initial_indent: options.initial_indent,
            subsequent_indent: options.subsequent_indent,
            break_words: options.break_words,
            word_separator: options.word_separator,
            wrap_algorithm: options.wrap_algorithm,
            word_splitter: options.word_splitter.clone(),
            preserve_trailing_space: options.preserve_trailing_space,
        }
    }
}

impl From<usize> for Options<'_> {
    fn from(width: usize) -> Self {
        Options::new(width)
    }
}

impl<'a> Options<'a> {
    /// Creates a new [`Options`] with the specified width.
    ///
    /// The other fields are given default values as follows:
    ///
    /// ```
    /// # use textwrap::{LineEnding, Options, WordSplitter, WordSeparator, WrapAlgorithm};
    /// # let width = 80;
    /// let options = Options::new(width);
    /// assert_eq!(options.line_ending, LineEnding::LF);
    /// assert_eq!(options.initial_indent, "");
    /// assert_eq!(options.subsequent_indent, "");
    /// assert_eq!(options.break_words, true);
    /// assert_eq!(options.preserve_trailing_space, false);
    ///
    /// #[cfg(feature = "unicode-linebreak")]
    /// assert_eq!(options.word_separator, WordSeparator::UnicodeBreakProperties);
    /// #[cfg(not(feature = "unicode-linebreak"))]
    /// assert_eq!(options.word_separator, WordSeparator::AsciiSpace);
    ///
    /// #[cfg(feature = "smawk")]
    /// assert_eq!(options.wrap_algorithm, WrapAlgorithm::new_optimal_fit());
    /// #[cfg(not(feature = "smawk"))]
    /// assert_eq!(options.wrap_algorithm, WrapAlgorithm::FirstFit);
    ///
    /// assert_eq!(options.word_splitter, WordSplitter::HyphenSplitter);
    /// ```
    ///
    /// Note that the default word separator and wrap algorithms
    /// change based on the available Cargo features. The best
    /// available algorithms are used by default.
    pub const fn new(width: usize) -> Self {
        Options {
            width,
            line_ending: LineEnding::LF,
            initial_indent: "",
            subsequent_indent: "",
            break_words: true,
            word_separator: WordSeparator::new(),
            wrap_algorithm: WrapAlgorithm::new(),
            word_splitter: WordSplitter::HyphenSplitter,
            preserve_trailing_space: false,
        }
    }

    /// Change [`self.line_ending`]. This specifies which of the
    /// supported line endings should be used to break the lines of the
    /// input text.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{refill, LineEnding, Options};
    ///
    /// let options = Options::new(15).line_ending(LineEnding::CRLF);
    /// assert_eq!(refill("This is a little example.", options),
    ///            "This is a\r\nlittle example.");
    /// ```
    ///
    /// [`self.line_ending`]: #structfield.line_ending
    pub fn line_ending(self, line_ending: LineEnding) -> Self {
        Options {
            line_ending,
            ..self
        }
    }

    /// Set [`self.width`] to the given value.
    ///
    /// [`self.width`]: #structfield.width
    pub fn width(self, width: usize) -> Self {
        Options { width, ..self }
    }

    /// Change [`self.initial_indent`]. The initial indentation is
    /// used on the very first line of output.
    ///
    /// # Examples
    ///
    /// Classic paragraph indentation can be achieved by specifying an
    /// initial indentation and wrapping each paragraph by itself:
    ///
    /// ```
    /// use textwrap::{wrap, Options};
    ///
    /// let options = Options::new(16).initial_indent("    ");
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["    This is a",
    ///                 "little example."]);
    /// ```
    ///
    /// [`self.initial_indent`]: #structfield.initial_indent
    pub fn initial_indent(self, initial_indent: &'a str) -> Self {
        Options {
            initial_indent,
            ..self
        }
    }

    /// Change [`self.subsequent_indent`]. The subsequent indentation
    /// is used on lines following the first line of output.
    ///
    /// # Examples
    ///
    /// Combining initial and subsequent indentation lets you format a
    /// single paragraph as a bullet list:
    ///
    /// ```
    /// use textwrap::{wrap, Options};
    ///
    /// let options = Options::new(12)
    ///     .initial_indent("* ")
    ///     .subsequent_indent("  ");
    /// #[cfg(feature = "smawk")]
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["* This is",
    ///                 "  a little",
    ///                 "  example."]);
    ///
    /// // Without the `smawk` feature, the wrapping is a little different:
    /// #[cfg(not(feature = "smawk"))]
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["* This is a",
    ///                 "  little",
    ///                 "  example."]);
    /// ```
    ///
    /// [`self.subsequent_indent`]: #structfield.subsequent_indent
    pub fn subsequent_indent(self, subsequent_indent: &'a str) -> Self {
        Options {
            subsequent_indent,
            ..self
        }
    }

    /// Change [`self.break_words`]. This controls if words longer
    /// than `self.width` can be broken, or if they will be left
    /// protruding into the right margin.
    ///
    /// See [`Options::word_splitter`] instead if you want to control
    /// hyphenation.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{wrap, Options};
    ///
    /// let options = Options::new(4).break_words(true);
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["This",
    ///                 "is a",
    ///                 "litt",
    ///                 "le",
    ///                 "exam",
    ///                 "ple."]);
    /// ```
    ///
    /// [`self.break_words`]: #structfield.break_words
    pub fn break_words(self, break_words: bool) -> Self {
        Options {
            break_words,
            ..self
        }
    }

    /// Change [`self.word_separator`].
    ///
    /// See the [`WordSeparator`] enum for details on the choices.
    ///
    /// [`self.word_separator`]: #structfield.word_separator
    pub fn word_separator(self, word_separator: WordSeparator) -> Options<'a> {
        Options {
            word_separator,
            ..self
        }
    }

    /// Change [`self.wrap_algorithm`].
    ///
    /// See the [`WrapAlgorithm`] enum for details on the choices.
    ///
    /// [`self.wrap_algorithm`]: #structfield.wrap_algorithm
    pub fn wrap_algorithm(self, wrap_algorithm: WrapAlgorithm) -> Options<'a> {
        Options {
            wrap_algorithm,
            ..self
        }
    }

    /// Change [`self.word_splitter`]. The [`WordSplitter`] is used to
    /// fit part of a word into the current line when wrapping text.
    ///
    /// See [`Options::break_words`] instead if you want to control the
    /// handling of words longer than the line width.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{wrap, Options, WordSplitter};
    ///
    /// // The default is WordSplitter::HyphenSplitter.
    /// let options = Options::new(5);
    /// assert_eq!(wrap("foo-bar-baz", &options),
    ///            vec!["foo-", "bar-", "baz"]);
    ///
    /// // The word is now so long that break_words kicks in:
    /// let options = Options::new(5)
    ///     .word_splitter(WordSplitter::NoHyphenation);
    /// assert_eq!(wrap("foo-bar-baz", &options),
    ///            vec!["foo-b", "ar-ba", "z"]);
    ///
    /// // If you want no breaks at all, disable both:
    /// let options = Options::new(5)
    ///     .break_words(false)
    ///     .word_splitter(WordSplitter::NoHyphenation);
    /// assert_eq!(wrap("foo-bar-baz", &options),
    ///            vec!["foo-bar-baz"]);
    /// ```
    ///
    /// [`self.word_splitter`]: #structfield.word_splitter
    pub fn word_splitter(self, word_splitter: WordSplitter) -> Options<'a> {
        Options {
            word_splitter,
            ..self
        }
    }

    /// Change [`self.preserve_trailing_space`]. This controls if the
    /// trailing spaces at the end of a line are preserved or trimmed.
    ///
    /// [`self.preserve_trailing_space`]: #structfield.preserve_trailing_space
    pub fn preserve_trailing_space(self, preserve_trailing_space: bool) -> Options<'a> {
        Options {
            preserve_trailing_space,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_agree_with_usize() {
        let opt_usize = Options::from(42_usize);
        let opt_options = Options::new(42);

        assert_eq!(opt_usize.width, opt_options.width);
        assert_eq!(opt_usize.initial_indent, opt_options.initial_indent);
        assert_eq!(opt_usize.subsequent_indent, opt_options.subsequent_indent);
        assert_eq!(opt_usize.break_words, opt_options.break_words);
        assert_eq!(
            opt_usize.word_splitter.split_points("hello-world"),
            opt_options.word_splitter.split_points("hello-world")
        );
    }
}

END FILE renderer/textwrap-0.16.4/src/options.rs


BEGIN FILE renderer/textwrap-0.16.4/src/word_separators.rs | SHA256 e2f18b45c44150d257c40c3abe8dfef8d24db2da85c42b6371b31ec0959e2ce8 | BYTES 17408
//! Functionality for finding words.
//!
//! In order to wrap text, we need to know where the legal break
//! points are, i.e., where the words of the text are. This means that
//! we need to define what a "word" is.
//!
//! A simple approach is to simply split the text on whitespace, but
//! this does not work for East Asian languages such as Chinese or
//! Japanese where there are no spaces between words. Breaking a long
//! sequence of emojis is another example where line breaks might be
//! wanted even if there is no whitespace to be found.
//!
//! The [`WordSeparator`] enum is responsible for determining where
//! the words are in a line of text. Please refer to the enum and
//! its variants for more information.

use alloc::boxed::Box;
#[cfg(any(feature = "unicode-linebreak", test))]
use alloc::vec::Vec;

use crate::core::Word;
#[cfg(feature = "unicode-linebreak")]
use crate::core::skip_ansi_escape_sequence;

#[cfg(feature = "unicode-linebreak")]
thread_local! {
    static LINE_SEGMENTER: icu_segmenter::LineSegmenterBorrowed<'static> = {
        use icu_segmenter::options::LineBreakOptions;
        use icu_segmenter::LineSegmenter;

        let options = LineBreakOptions::default();
        #[cfg(feature = "unicode-linebreak-complex")]
        {
            LineSegmenter::new_auto(options)
        }
        #[cfg(not(feature = "unicode-linebreak-complex"))]
        {
            LineSegmenter::new_for_non_complex_scripts(options)
        }
    };
}

/// Describes where words occur in a line of text.
///
/// The simplest approach is to say that words are separated by one or
/// more ASCII spaces (`' '`). This works for Western languages
/// without emojis. A more complex approach is to use the Unicode line
/// breaking algorithm, which finds break points in non-ASCII text.
///
/// The line breaks occur between words, please see
/// [`WordSplitter`](crate::WordSplitter) for options of how to handle
/// hyphenation of individual words.
///
/// # Examples
///
/// ```
/// use textwrap::core::Word;
/// use textwrap::WordSeparator::AsciiSpace;
///
/// let words = AsciiSpace.find_words("Hello World!").collect::<Vec<_>>();
/// assert_eq!(words, vec![Word::from("Hello "), Word::from("World!")]);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum WordSeparator {
    /// Find words by splitting on runs of `' '` characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::core::Word;
    /// use textwrap::WordSeparator::AsciiSpace;
    ///
    /// let words = AsciiSpace.find_words("Hello   World!").collect::<Vec<_>>();
    /// assert_eq!(words, vec![Word::from("Hello   "),
    ///                        Word::from("World!")]);
    /// ```
    AsciiSpace,

    /// Split `line` into words using Unicode break properties.
    ///
    /// This word separator uses the Unicode line breaking algorithm
    /// described in [Unicode Standard Annex
    /// #14](https://www.unicode.org/reports/tr14/) to find legal places
    /// to break lines. There is a small difference in that the U+002D
    /// (Hyphen-Minus) and U+00AD (Soft Hyphen) don’t create a line break:
    /// to allow a line break at a hyphen, use
    /// [`WordSplitter::HyphenSplitter`](crate::WordSplitter::HyphenSplitter).
    /// Soft hyphens are not currently supported.
    ///
    /// # Examples
    ///
    /// Unlike [`WordSeparator::AsciiSpace`], the Unicode line
    /// breaking algorithm will find line break opportunities between
    /// some characters with no intervening whitespace:
    ///
    /// ```
    /// #[cfg(feature = "unicode-linebreak")] {
    /// use textwrap::core::Word;
    /// use textwrap::WordSeparator::UnicodeBreakProperties;
    ///
    /// assert_eq!(UnicodeBreakProperties.find_words("Emojis: 😂😍").collect::<Vec<_>>(),
    ///            vec![Word::from("Emojis: "),
    ///                 Word::from("😂"),
    ///                 Word::from("😍")]);
    ///
    /// assert_eq!(UnicodeBreakProperties.find_words("CJK: 你好").collect::<Vec<_>>(),
    ///            vec![Word::from("CJK: "),
    ///                 Word::from("你"),
    ///                 Word::from("好")]);
    /// }
    /// ```
    ///
    /// A U+2060 (Word Joiner) character can be inserted if you want to
    /// manually override the defaults and keep the characters together:
    ///
    /// ```
    /// #[cfg(feature = "unicode-linebreak")] {
    /// use textwrap::core::Word;
    /// use textwrap::WordSeparator::UnicodeBreakProperties;
    ///
    /// assert_eq!(UnicodeBreakProperties.find_words("Emojis: 😂\u{2060}😍").collect::<Vec<_>>(),
    ///            vec![Word::from("Emojis: "),
    ///                 Word::from("😂\u{2060}😍")]);
    /// }
    /// ```
    ///
    /// The Unicode line breaking algorithm will also automatically
    /// suppress line breaks around certain punctuation characters:
    ///
    /// ```
    /// #[cfg(feature = "unicode-linebreak")] {
    /// use textwrap::core::Word;
    /// use textwrap::WordSeparator::UnicodeBreakProperties;
    ///
    /// assert_eq!(UnicodeBreakProperties.find_words("[ foo ] bar !").collect::<Vec<_>>(),
    ///            vec![Word::from("[ foo ] "),
    ///                 Word::from("bar !")]);
    /// }
    /// ```
    #[cfg(feature = "unicode-linebreak")]
    UnicodeBreakProperties,

    /// Find words using a custom word separator.
    Custom(fn(line: &str) -> Box<dyn Iterator<Item = Word<'_>> + '_>),
}

impl PartialEq for WordSeparator {
    /// Compare two word separators.
    ///
    /// ```
    /// use textwrap::WordSeparator;
    ///
    /// assert_eq!(WordSeparator::AsciiSpace, WordSeparator::AsciiSpace);
    /// #[cfg(feature = "unicode-linebreak")] {
    ///     assert_eq!(WordSeparator::UnicodeBreakProperties,
    ///                WordSeparator::UnicodeBreakProperties);
    /// }
    /// ```
    ///
    /// Note that `WordSeparator::Custom` values never compare equal:
    ///
    /// ```
    /// use textwrap::WordSeparator;
    /// use textwrap::core::Word;
    /// fn word_separator(line: &str) -> Box<dyn Iterator<Item = Word<'_>> + '_> {
    ///     Box::new(line.split_inclusive(' ').map(Word::from))
    /// }
    /// assert_ne!(WordSeparator::Custom(word_separator),
    ///            WordSeparator::Custom(word_separator));
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WordSeparator::AsciiSpace, WordSeparator::AsciiSpace) => true,
            #[cfg(feature = "unicode-linebreak")]
            (WordSeparator::UnicodeBreakProperties, WordSeparator::UnicodeBreakProperties) => true,
            (_, _) => false,
        }
    }
}

impl WordSeparator {
    /// Create a new word separator.
    ///
    /// The best available algorithm is used by default, i.e.,
    /// [`WordSeparator::UnicodeBreakProperties`] if available,
    /// otherwise [`WordSeparator::AsciiSpace`].
    pub const fn new() -> Self {
        #[cfg(feature = "unicode-linebreak")]
        {
            WordSeparator::UnicodeBreakProperties
        }

        #[cfg(not(feature = "unicode-linebreak"))]
        {
            WordSeparator::AsciiSpace
        }
    }

    // This function should really return impl Iterator<Item = Word>, but
    // this isn't possible until Rust supports higher-kinded types:
    // https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md
    /// Find all words in `line`.
    pub fn find_words<'a>(&self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>> + 'a> {
        match self {
            WordSeparator::AsciiSpace => find_words_ascii_space(line),
            #[cfg(feature = "unicode-linebreak")]
            WordSeparator::UnicodeBreakProperties => find_words_unicode_break_properties(line),
            WordSeparator::Custom(func) => func(line),
        }
    }
}

fn find_words_ascii_space<'a>(line: &'a str) -> Box<dyn Iterator<Item = Word<'a>> + 'a> {
    let mut start = 0;
    let mut in_whitespace = false;
    let mut char_indices = line.char_indices();

    Box::new(core::iter::from_fn(move || {
        for (idx, ch) in char_indices.by_ref() {
            if in_whitespace && ch != ' ' {
                let word = Word::from(&line[start..idx]);
                start = idx;
                in_whitespace = ch == ' ';
                return Some(word);
            }

            in_whitespace = ch == ' ';
        }

        if start < line.len() {
            let word = Word::from(&line[start..]);
            start = line.len();
            return Some(word);
        }

        None
    }))
}

// Strip all ANSI escape sequences from `text`.
#[cfg(feature = "unicode-linebreak")]
fn strip_ansi_escape_sequences(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if skip_ansi_escape_sequence(ch, &mut chars) {
            continue;
        }
        result.push(ch);
    }

    result
}

/// Soft hyphen, also knows as a “shy hyphen”. Should show up as ‘-’
/// if a line is broken at this point, and otherwise be invisible.
/// Textwrap does not currently support breaking words at soft
/// hyphens.
#[cfg(feature = "unicode-linebreak")]
const SHY: char = '\u{00ad}';

/// Find words in line. ANSI escape sequences are ignored in `line`.
#[cfg(feature = "unicode-linebreak")]
fn find_words_unicode_break_properties<'a>(
    line: &'a str,
) -> Box<dyn Iterator<Item = Word<'a>> + 'a> {
    // Construct an iterator over (original index, stripped index)
    // tuples. We find the Unicode linebreaks on a stripped string,
    // but we need the original indices so we can form words based on
    // the original string.
    let mut last_stripped_idx = 0;
    let mut char_indices = line.char_indices();
    let mut idx_map = core::iter::from_fn(move || match char_indices.next() {
        Some((orig_idx, ch)) => {
            let stripped_idx = last_stripped_idx;
            if !skip_ansi_escape_sequence(ch, &mut char_indices.by_ref().map(|(_, ch)| ch)) {
                last_stripped_idx += ch.len_utf8();
            }
            Some((orig_idx, stripped_idx))
        }
        None => None,
    });

    let stripped = strip_ansi_escape_sequences(line);
    let opportunities_vec = LINE_SEGMENTER.with(|segmenter| {
        segmenter
            .segment_str(&stripped)
            .filter(|&idx| idx > 0)
            .filter(|&idx| {
                #[allow(clippy::match_like_matches_macro)]
                match &stripped[..idx].chars().next_back() {
                    // We suppress breaks at ‘-’ since we want to control
                    // this via the WordSplitter.
                    Some('-') => false,
                    // Soft hyphens are currently not supported since we
                    // require all `Word` fragments to be continuous in
                    // the input string.
                    Some(SHY) => false,
                    // Other breaks should be fine!
                    _ => true,
                }
            })
            .collect::<Vec<_>>()
    });
    let mut opportunities = opportunities_vec.into_iter();

    // Remove final break opportunity, we will add it below using
    // &line[start..]; This ensures that we correctly include a
    // trailing ANSI escape sequence.
    opportunities.next_back();

    let mut start = 0;
    Box::new(core::iter::from_fn(move || {
        for idx in opportunities.by_ref() {
            if let Some((orig_idx, _)) = idx_map.find(|&(_, stripped_idx)| stripped_idx == idx) {
                let word = Word::from(&line[start..orig_idx]);
                start = orig_idx;
                return Some(word);
            }
        }

        if start < line.len() {
            let word = Word::from(&line[start..]);
            start = line.len();
            return Some(word);
        }

        None
    }))
}

#[cfg(test)]
mod tests {
    use super::WordSeparator::*;
    use super::*;
    use alloc::vec;

    // Like assert_eq!, but the left expression is an iterator.
    macro_rules! assert_iter_eq {
        ($left:expr, $right:expr) => {
            assert_eq!($left.collect::<Vec<_>>(), $right);
        };
    }

    fn to_words(words: Vec<&str>) -> Vec<Word<'_>> {
        words.into_iter().map(Word::from).collect()
    }

    macro_rules! test_find_words {
        ($ascii_name:ident,
         $unicode_name:ident,
         $([ $line:expr, $ascii_words:expr, $unicode_words:expr ]),+) => {
            #[test]
            fn $ascii_name() {
                $(
                    let expected_words = to_words($ascii_words.to_vec());
                    let actual_words = WordSeparator::AsciiSpace
                        .find_words($line)
                        .collect::<Vec<_>>();
                    assert_eq!(actual_words, expected_words, "Line: {:?}", $line);
                )+
            }

            #[test]
            #[cfg(feature = "unicode-linebreak")]
            fn $unicode_name() {
                $(
                    let expected_words = to_words($unicode_words.to_vec());
                    let actual_words = WordSeparator::UnicodeBreakProperties
                        .find_words($line)
                        .collect::<Vec<_>>();
                    assert_eq!(actual_words, expected_words, "Line: {:?}", $line);
                )+
            }
        };
    }

    test_find_words!(ascii_space_empty, unicode_empty, ["", [], []]);

    test_find_words!(
        ascii_single_word,
        unicode_single_word,
        ["foo", ["foo"], ["foo"]]
    );

    test_find_words!(
        ascii_two_words,
        unicode_two_words,
        ["foo bar", ["foo ", "bar"], ["foo ", "bar"]]
    );

    test_find_words!(
        ascii_multiple_words,
        unicode_multiple_words,
        ["foo bar", ["foo ", "bar"], ["foo ", "bar"]],
        ["x y z", ["x ", "y ", "z"], ["x ", "y ", "z"]]
    );

    test_find_words!(
        ascii_only_whitespace,
        unicode_only_whitespace,
        [" ", [" "], [" "]],
        ["    ", ["    "], ["    "]]
    );

    test_find_words!(
        ascii_inter_word_whitespace,
        unicode_inter_word_whitespace,
        ["foo   bar", ["foo   ", "bar"], ["foo   ", "bar"]]
    );

    test_find_words!(
        ascii_trailing_whitespace,
        unicode_trailing_whitespace,
        ["foo   ", ["foo   "], ["foo   "]]
    );

    test_find_words!(
        ascii_leading_whitespace,
        unicode_leading_whitespace,
        ["   foo", ["   ", "foo"], ["   ", "foo"]]
    );

    test_find_words!(
        ascii_multi_column_char,
        unicode_multi_column_char,
        ["\u{1f920}", ["\u{1f920}"], ["\u{1f920}"]] // cowboy emoji 🤠
    );

    test_find_words!(
        ascii_hyphens,
        unicode_hyphens,
        ["foo-bar", ["foo-bar"], ["foo-bar"]],
        ["foo- bar", ["foo- ", "bar"], ["foo- ", "bar"]],
        ["foo - bar", ["foo ", "- ", "bar"], ["foo ", "- ", "bar"]],
        ["foo -bar", ["foo ", "-bar"], ["foo ", "-bar"]]
    );

    test_find_words!(
        ascii_newline,
        unicode_newline,
        ["foo\nbar", ["foo\nbar"], ["foo\n", "bar"]]
    );

    test_find_words!(
        ascii_tab,
        unicode_tab,
        ["foo\tbar", ["foo\tbar"], ["foo\t", "bar"]]
    );

    test_find_words!(
        ascii_non_breaking_space,
        unicode_non_breaking_space,
        ["foo\u{00A0}bar", ["foo\u{00A0}bar"], ["foo\u{00A0}bar"]]
    );

    #[test]
    #[cfg(all(unix, feature = "std"))]
    fn find_words_colored_text() {
        use termion::color::{Blue, Fg, Green, Reset};

        let green_hello = format!("{}Hello{} ", Fg(Green), Fg(Reset));
        let blue_world = format!("{}World!{}", Fg(Blue), Fg(Reset));
        assert_iter_eq!(
            AsciiSpace.find_words(&format!("{}{}", green_hello, blue_world)),
            vec![Word::from(&green_hello), Word::from(&blue_world)]
        );

        #[cfg(feature = "unicode-linebreak")]
        assert_iter_eq!(
            UnicodeBreakProperties.find_words(&format!("{}{}", green_hello, blue_world)),
            vec![Word::from(&green_hello), Word::from(&blue_world)]
        );
    }

    #[test]
    fn find_words_color_inside_word() {
        let text = "foo\u{1b}[0m\u{1b}[32mbar\u{1b}[0mbaz";
        assert_iter_eq!(AsciiSpace.find_words(text), vec![Word::from(text)]);

        #[cfg(feature = "unicode-linebreak")]
        assert_iter_eq!(
            UnicodeBreakProperties.find_words(text),
            vec![Word::from(text)]
        );
    }

    #[test]
    #[cfg(feature = "unicode-linebreak")]
    fn find_words_complex_scripts() {
        let text = "ภาษาไทย"; // English: "Thai language"

        #[cfg(feature = "unicode-linebreak-complex")]
        assert_iter_eq!(
            UnicodeBreakProperties.find_words(text),
            vec![Word::from("ภาษา"), Word::from("ไทย")]
        );

        #[cfg(not(feature = "unicode-linebreak-complex"))]
        assert_iter_eq!(
            UnicodeBreakProperties.find_words(text),
            vec![Word::from(text)]
        );
    }

    #[test]
    fn word_separator_new() {
        #[cfg(feature = "unicode-linebreak")]
        assert!(matches!(WordSeparator::new(), UnicodeBreakProperties));

        #[cfg(not(feature = "unicode-linebreak"))]
        assert!(matches!(WordSeparator::new(), AsciiSpace));
    }
}

END FILE renderer/textwrap-0.16.4/src/word_separators.rs


BEGIN FILE renderer/textwrap-0.16.4/src/wrap_algorithms.rs | SHA256 800a5a31efe5a0d026331a3a8e8dc5b74f1e8a4c85b09495addd2ca38519d48b | BYTES 14585
//! Word wrapping algorithms.
//!
//! After a text has been broken into words (or [`Fragment`]s), one
//! now has to decide how to break the fragments into lines. The
//! simplest algorithm for this is implemented by
//! [`wrap_first_fit()`]: it uses no look-ahead and simply adds
//! fragments to the line as long as they fit. However, this can lead
//! to poor line breaks if a large fragment almost-but-not-quite fits
//! on a line. When that happens, the fragment is moved to the next
//! line and it will leave behind a large gap.
//!
//! A more advanced algorithm, implemented by [`wrap_optimal_fit()`],
//! will take this into account. The optimal-fit algorithm considers
//! all possible line breaks and will attempt to minimize the gaps
//! left behind by overly short lines.
//!
//! While both algorithms run in linear time, the first-fit algorithm
//! is about 4 times faster than the optimal-fit algorithm.

#[cfg(feature = "smawk")]
mod optimal_fit;
#[cfg(feature = "smawk")]
pub use optimal_fit::{OverflowError, Penalties, wrap_optimal_fit};

use crate::core::{Fragment, Word};
use alloc::vec::Vec;

/// Describes how to wrap words into lines.
///
/// The simplest approach is to wrap words one word at a time and
/// accept the first way of wrapping that fits
/// ([`WrapAlgorithm::FirstFit`]). If the `smawk` Cargo feature is
/// enabled, a more complex algorithm is available that will look at
/// an entire paragraph at a time in order to find optimal line breaks
/// ([`WrapAlgorithm::OptimalFit`]).
#[derive(Clone, Copy, Debug)]
pub enum WrapAlgorithm {
    /// Wrap words using a fast and simple algorithm.
    ///
    /// This algorithm uses no look-ahead when finding line breaks.
    /// Implemented by [`wrap_first_fit()`], please see that function
    /// for details and examples.
    FirstFit,

    /// Wrap words using an advanced algorithm with look-ahead.
    ///
    /// This wrapping algorithm considers the entire paragraph to find
    /// optimal line breaks. When wrapping text, "penalties" are
    /// assigned to line breaks based on the gaps left at the end of
    /// lines. See [`Penalties`] for details.
    ///
    /// The underlying wrapping algorithm is implemented by
    /// [`wrap_optimal_fit()`], please see that function for examples.
    ///
    /// **Note:** Only available when the `smawk` Cargo feature is
    /// enabled.
    #[cfg(feature = "smawk")]
    OptimalFit(Penalties),

    /// Custom wrapping function.
    ///
    /// Use this if you want to implement your own wrapping algorithm.
    /// The function can freely decide how to turn a slice of
    /// [`Word`]s into lines.
    ///
    /// # Example
    ///
    /// ```
    /// use textwrap::core::Word;
    /// use textwrap::{wrap, Options, WrapAlgorithm};
    ///
    /// fn stair<'a, 'b>(words: &'b [Word<'a>], _: &'b [usize]) -> Vec<&'b [Word<'a>]> {
    ///     let mut lines = Vec::new();
    ///     let mut step = 1;
    ///     let mut start_idx = 0;
    ///     while start_idx + step <= words.len() {
    ///       lines.push(&words[start_idx .. start_idx+step]);
    ///       start_idx += step;
    ///       step += 1;
    ///     }
    ///     lines
    /// }
    ///
    /// let options = Options::new(10).wrap_algorithm(WrapAlgorithm::Custom(stair));
    /// assert_eq!(wrap("First, second, third, fourth, fifth, sixth", options),
    ///            vec!["First,",
    ///                 "second, third,",
    ///                 "fourth, fifth, sixth"]);
    /// ```
    Custom(for<'a, 'b> fn(words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]>),
}

impl PartialEq for WrapAlgorithm {
    /// Compare two wrap algorithms.
    ///
    /// ```
    /// use textwrap::WrapAlgorithm;
    ///
    /// assert_eq!(WrapAlgorithm::FirstFit, WrapAlgorithm::FirstFit);
    /// #[cfg(feature = "smawk")] {
    ///     assert_eq!(WrapAlgorithm::new_optimal_fit(), WrapAlgorithm::new_optimal_fit());
    /// }
    /// ```
    ///
    /// Note that `WrapAlgorithm::Custom` values never compare equal:
    ///
    /// ```
    /// use textwrap::WrapAlgorithm;
    ///
    /// assert_ne!(WrapAlgorithm::Custom(|words, line_widths| vec![words]),
    ///            WrapAlgorithm::Custom(|words, line_widths| vec![words]));
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WrapAlgorithm::FirstFit, WrapAlgorithm::FirstFit) => true,
            #[cfg(feature = "smawk")]
            (WrapAlgorithm::OptimalFit(a), WrapAlgorithm::OptimalFit(b)) => a == b,
            (_, _) => false,
        }
    }
}

impl WrapAlgorithm {
    /// Create new wrap algorithm.
    ///
    /// The best wrapping algorithm is used by default, i.e.,
    /// [`WrapAlgorithm::OptimalFit`] if available, otherwise
    /// [`WrapAlgorithm::FirstFit`].
    pub const fn new() -> Self {
        #[cfg(not(feature = "smawk"))]
        {
            WrapAlgorithm::FirstFit
        }

        #[cfg(feature = "smawk")]
        {
            WrapAlgorithm::new_optimal_fit()
        }
    }

    /// New [`WrapAlgorithm::OptimalFit`] with default penalties. This
    /// works well for monospace text.
    ///
    /// **Note:** Only available when the `smawk` Cargo feature is
    /// enabled.
    #[cfg(feature = "smawk")]
    pub const fn new_optimal_fit() -> Self {
        WrapAlgorithm::OptimalFit(Penalties::new())
    }

    /// Wrap words according to line widths.
    ///
    /// The `line_widths` slice gives the target line width for each
    /// line (the last slice element is repeated as necessary). This
    /// can be used to implement hanging indentation.
    #[inline]
    pub fn wrap<'a, 'b>(
        &self,
        words: &'b [Word<'a>],
        line_widths: &'b [usize],
    ) -> Vec<&'b [Word<'a>]> {
        // Every integer up to 2u64.pow(f64::MANTISSA_DIGITS) = 2**53
        // = 9_007_199_254_740_992 can be represented without loss by
        // a f64. Larger line widths will be rounded to the nearest
        // representable number.
        let f64_line_widths = line_widths.iter().map(|w| *w as f64).collect::<Vec<_>>();

        match self {
            WrapAlgorithm::FirstFit => wrap_first_fit(words, &f64_line_widths),

            #[cfg(feature = "smawk")]
            WrapAlgorithm::OptimalFit(penalties) => {
                // The computation cannot overflow when the line
                // widths are restricted to usize.
                wrap_optimal_fit(words, &f64_line_widths, penalties).unwrap()
            }

            WrapAlgorithm::Custom(func) => func(words, line_widths),
        }
    }
}

impl Default for WrapAlgorithm {
    fn default() -> Self {
        WrapAlgorithm::new()
    }
}

/// Wrap abstract fragments into lines with a first-fit algorithm.
///
/// The `line_widths` slice gives the target line width for each line
/// (the last slice element is repeated as necessary). This can be
/// used to implement hanging indentation.
///
/// The fragments must already have been split into the desired
/// widths. This function will not (and cannot) attempt to split them
/// further when arranging them into lines.
///
/// # First-Fit Algorithm
///
/// This implements a simple “greedy” algorithm: accumulate fragments
/// one by one and when a fragment no longer fits, start a new line.
/// There is no look-ahead, we simply take the first fit of the
/// fragments we find.
///
/// While fast and predictable, this algorithm can produce poor line
/// breaks when a long fragment is moved to a new line, leaving behind
/// a large gap:
///
/// ```
/// use textwrap::core::Word;
/// use textwrap::wrap_algorithms::wrap_first_fit;
/// use textwrap::WordSeparator;
///
/// // Helper to convert wrapped lines to a Vec<String>.
/// fn lines_to_strings(lines: Vec<&[Word<'_>]>) -> Vec<String> {
///     lines.iter().map(|line| {
///         line.iter().map(|word| &**word).collect::<Vec<_>>().join(" ")
///     }).collect::<Vec<_>>()
/// }
///
/// let text = "These few words will unfortunately not wrap nicely.";
/// let words = WordSeparator::AsciiSpace.find_words(text).collect::<Vec<_>>();
/// assert_eq!(lines_to_strings(wrap_first_fit(&words, &[15.0])),
///            vec!["These few words",
///                 "will",  // <-- short line
///                 "unfortunately",
///                 "not wrap",
///                 "nicely."]);
///
/// // We can avoid the short line if we look ahead:
/// #[cfg(feature = "smawk")]
/// use textwrap::wrap_algorithms::{wrap_optimal_fit, Penalties};
/// #[cfg(feature = "smawk")]
/// assert_eq!(lines_to_strings(wrap_optimal_fit(&words, &[15.0], &Penalties::new()).unwrap()),
///            vec!["These few",
///                 "words will",
///                 "unfortunately",
///                 "not wrap",
///                 "nicely."]);
/// ```
///
/// The [`wrap_optimal_fit()`] function was used above to get better
/// line breaks. It uses an advanced algorithm which tries to avoid
/// short lines. This function is about 4 times faster than
/// [`wrap_optimal_fit()`].
///
/// # Examples
///
/// Imagine you're building a house site and you have a number of
/// tasks you need to execute. Things like pouring the foundation,
/// framing, plumbing, electrical cabling, and installing insulation.
///
/// The construction workers can only work during daytime, so they
/// need to pack up everything at night. Because they need to secure
/// their tools and move machines back to the garage, this process
/// takes much more time than the time it would take them to simply
/// switch to another task.
///
/// You would like to make a list of tasks to execute every day based
/// on your estimates. You can model this with a program like this:
///
/// ```
/// use textwrap::core::{Fragment, Word};
/// use textwrap::wrap_algorithms::wrap_first_fit;
///
/// #[derive(Debug)]
/// struct Task<'a> {
///     name: &'a str,
///     hours: f64,   // Time needed to complete task.
///     sweep: f64,   // Time needed for a quick sweep after task during the day.
///     cleanup: f64, // Time needed for full cleanup if day ends with this task.
/// }
///
/// impl Fragment for Task<'_> {
///     fn width(&self) -> f64 { self.hours }
///     fn whitespace_width(&self) -> f64 { self.sweep }
///     fn penalty_width(&self) -> f64 { self.cleanup }
/// }
///
/// // The morning tasks
/// let tasks = vec![
///     Task { name: "Foundation",  hours: 4.0, sweep: 2.0, cleanup: 3.0 },
///     Task { name: "Framing",     hours: 3.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Plumbing",    hours: 2.0, sweep: 2.0, cleanup: 2.0 },
///     Task { name: "Electrical",  hours: 2.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Insulation",  hours: 2.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Drywall",     hours: 3.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Floors",      hours: 3.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Countertops", hours: 1.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Bathrooms",   hours: 2.0, sweep: 1.0, cleanup: 2.0 },
/// ];
///
/// // Fill tasks into days, taking `day_length` into account. The
/// // output shows the hours worked per day along with the names of
/// // the tasks for that day.
/// fn assign_days<'a>(tasks: &[Task<'a>], day_length: f64) -> Vec<(f64, Vec<&'a str>)> {
///     let mut days = Vec::new();
///     // Assign tasks to days. The assignment is a vector of slices,
///     // with a slice per day.
///     let assigned_days: Vec<&[Task<'a>]> = wrap_first_fit(&tasks, &[day_length]);
///     for day in assigned_days.iter() {
///         let last = day.last().unwrap();
///         let work_hours: f64 = day.iter().map(|t| t.hours + t.sweep).sum();
///         let names = day.iter().map(|t| t.name).collect::<Vec<_>>();
///         days.push((work_hours - last.sweep + last.cleanup, names));
///     }
///     days
/// }
///
/// // With a single crew working 8 hours a day:
/// assert_eq!(
///     assign_days(&tasks, 8.0),
///     [
///         (7.0, vec!["Foundation"]),
///         (8.0, vec!["Framing", "Plumbing"]),
///         (7.0, vec!["Electrical", "Insulation"]),
///         (5.0, vec!["Drywall"]),
///         (7.0, vec!["Floors", "Countertops"]),
///         (4.0, vec!["Bathrooms"]),
///     ]
/// );
///
/// // With two crews working in shifts, 16 hours a day:
/// assert_eq!(
///     assign_days(&tasks, 16.0),
///     [
///         (14.0, vec!["Foundation", "Framing", "Plumbing"]),
///         (15.0, vec!["Electrical", "Insulation", "Drywall", "Floors"]),
///         (6.0, vec!["Countertops", "Bathrooms"]),
///     ]
/// );
/// ```
///
/// Apologies to anyone who actually knows how to build a house and
/// knows how long each step takes :-)
pub fn wrap_first_fit<'a, T: Fragment>(fragments: &'a [T], line_widths: &[f64]) -> Vec<&'a [T]> {
    // The final line width is used for all remaining lines.
    let default_line_width = line_widths.last().copied().unwrap_or(0.0);
    let mut lines = Vec::new();
    let mut start = 0;
    let mut width = 0.0;

    for (idx, fragment) in fragments.iter().enumerate() {
        let line_width = line_widths
            .get(lines.len())
            .copied()
            .unwrap_or(default_line_width);
        if width + fragment.width() + fragment.penalty_width() > line_width && idx > start {
            lines.push(&fragments[start..idx]);
            start = idx;
            width = 0.0;
        }
        width += fragment.width() + fragment.whitespace_width();
    }
    lines.push(&fragments[start..]);
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[derive(Debug, PartialEq)]
    struct Word(f64);

    #[rustfmt::skip]
    impl Fragment for Word {
        fn width(&self) -> f64 { self.0 }
        fn whitespace_width(&self) -> f64 { 1.0 }
        fn penalty_width(&self) -> f64 { 0.0 }
    }

    #[test]
    fn wrap_string_longer_than_f64() {
        let words = vec![
            Word(1e307),
            Word(2e307),
            Word(3e307),
            Word(4e307),
            Word(5e307),
            Word(6e307),
        ];
        // Wrap at just under f64::MAX (~19e307). The tiny
        // whitespace_widths disappear because of loss of precision.
        assert_eq!(
            wrap_first_fit(&words, &[15e307]),
            &[
                vec![
                    Word(1e307),
                    Word(2e307),
                    Word(3e307),
                    Word(4e307),
                    Word(5e307)
                ],
                vec![Word(6e307)]
            ]
        );
    }
}

END FILE renderer/textwrap-0.16.4/src/wrap_algorithms.rs


BEGIN FILE units/UNI-018.md | SHA256 97a9c37cc07986731b005ea281e07e4449e90617f8a77c2bc57a43acaf1cf1d5 | BYTES 814
### Requirement: UNI-018 Profile and consumer admission
When a tool reports checking, compilation or simulation, it SHALL identify the source/Core profile, resolved executable, consumer and admitted subset, and SHALL report unsupported consumer constructs explicitly.

#### Scenario: Supported Rust Core program
- **WHEN** A `moriarty/1` program is checked and compiled by its supported Rust consumers.
- **THEN** The record names `moriarty-core/1`, both consumers and the construct coverage actually exercised.

#### Scenario: Different authoring profile
- **WHEN** A beta or Source/6 program passes its local checker but is outside a Rust compiler's admitted input.
- **THEN** The record retains the local checking result and reports the compiler boundary as unsupported; compilation receives no inferred pass.


END FILE units/UNI-018.md


BEGIN FILE units/RUST-diagnostics-testing-cli.md | SHA256 a7571b648e1a7f8840f1959043a8ff97c17c9c50fdace1afb40bf62a45fd0fe2 | BYTES 7423
## Diagnostics

- Follow [oxc](https://github.com/oxc-project/oxc): one open `MoriDiagnostic`
  struct (message, labels, help, severity, code) in a `mori-diagnostics` crate,
  implementing miette's `Diagnostic` trait. Do not create a struct or enum
  variant per error.
- Each compiler crate has a `diagnostics.rs` of builder functions:

  ```rust
  pub fn quantity_separator(span: Span, source: &str) -> MoriDiagnostic {
      MoriDiagnostic::error("A quantity needs a space between the number and its asset.")
          .with_code("mori::syntax::quantity_without_space")
          .with_label(span.label("no space here"))
          .with_help_code("Put a space between them:", corrected_line(source, span, ...))
  }
  ```

- Every phase builds a `MoriDiagnostic` where the error is found, through a
  builder in its `diagnostics.rs`. Errors are rare and never on a hot path.
- Messages must be Elm-quality: say what was being parsed, what was expected,
  show a correct example and give a specific hint. Errors should teach the
  language.
- To support that, the parser keeps a context stack (agreement body,
  declaration, record fields, call arguments, type annotation) and records the
  set of expected token tags at each failure.
- Common mistakes get their own codes and teaching messages, for example:
  `10USD` (missing space before the asset), a decimal without an asset, a
  forward reference, an unknown call (suggest the closest of the known calls),
  a reserved word as a name, `/` (no division), a dotted name that is not a
  call, and adding quantities of different assets.
- Codes follow miette's style, one per kind of error:
  `mori::<phase>::<kind>`, such as `mori::lex::leading_zero` or
  `mori::syntax::missing_comma`.
- No title banners. A help that suggests a fix shows the corrected code on its
  own indented lines, not inside a sentence:

  ```
    help: Fields in a record are separated by commas:

              { domain: Preview, id: "A" }
  ```
- Spans are UTF-8 byte ranges, matching miette's `SourceSpan` and beta. Editors
  and a future language server need UTF-16 line/character positions, converted
  from the same compact records.
- The lexer and parser stop at the first error, like the TypeScript frontend.
  Recovery at declaration boundaries may come later.
- `mori-span` converts `Span` into miette's span types, as `oxc_span` does.
- The `fancy` feature of `mori-diagnostics` enables miette's
  `fancy-no-backtrace`. The `mori` CLI turns it on; library crates turn it on
  only in `[dev-dependencies]`.
- Use `thiserror` for errors that never point at source, such as I/O. No
  `anyhow` in library crates.

## Testing

- Snapshot tests use [`insta`](https://insta.rs) and `cargo-insta`.
- The finite backend `financial_observation_relation` target uses frozen
  complete Program/Execution literals and positive SDK relation checks instead
  of insta snapshots. The exact sources and independent expectations remain in
  `crates/fixtures/financial-report-contract/`; expected values must not come
  from the evaluator, compiler output or generated baselines. This exception
  adds no authority to execute SDK checks or claim native/proof acceptance.
- The finite Core `financial_program_wire_contract` target uses independently
  frozen complete financial Program JSON and the public Program/v1 decoder
  instead of generated snapshots. It selects no Midnight compatibility feature
  and makes no compiler-Artifact, SDK, native-proof or ledger acceptance claim.
- The `guarded_quantity_fee_positive`, `program_plan_roundtrip`, and
  `core_reserve_spend_refund` regressions compare frozen financial or continuation
  values directly instead of generating insta snapshots. Their sources and
  expected values stay in `crates/fixtures/`; they use the ordinary Core library
  and, for the reserve case, its public CLI. Source exposure and the exact
  assertion boundaries are recorded with the test evidence. These tests do not
  establish SDK, native-proof, compiler-Artifact or ledger correspondence.
- Test programs shared by several crates live in `crates/fixtures/`, which is
  excluded from the workspace. Nothing under `crates/` reads files outside it;
  copy anything needed in.
- **Any test that consumes `.mori` source must put the source, not the Rust
  expression, in the snapshot.** Use a macro that takes an `indoc!` literal and
  sets `description` and `omit_expression`:

  ```rust
  insta::with_settings!({
      description => format!("Code:\n\n{}", input),
      omit_expression => true,
  }, {
      insta::assert_snapshot!(render(&output));
  });
  ```

- Use separate macros for success and error cases. Error snapshots render the
  diagnostic through miette's `GraphicalReportHandler` with
  `GraphicalTheme::unicode_nocolor()` and a fixed width, so the snapshot is
  exactly what a user sees.
- Snapshot the AST through a tree printer, not `Debug` of the struct-of-arrays.
- Every phase that consumes source has a `proptest` property that it never
  panics, fed random strings, sequences of tricky fragments and, from the
  parser on, real programs with random damage.
- Formatter tests also assert that formatting the output again changes nothing.
- **Test semantics in the library crates; keep the CLI a thin layer over them.**
  The CLI feature-boundary work has a narrow exception for real binary tests
  of public dispatch, feature availability, frozen complete output, process
  exits and I/O. This exception does not make a shared evaluator an independent
  oracle. Never snapshot `--help`; use clap schema assertions for that contract.
- Existing external fixture paths and test-support mounts are retained during
  the feature-boundary cut; they are not precedents for new paths or mounts.
  `core_continuations` includes the three `examples/core/continuations/*.mori`
  sources and `normal-oracle.json`; `stage_test_public` reads
  `models/core-stage-tests`; `fee_pool_component` and one `core_test_report`
  case read `examples/core/fee-pool`. Native targets also retain their existing
  example/deliverable inputs. Their provenance and the rule against fixtures
  outside `crates/` need separate reconciliation before claiming full guideline
  conformance. Preserve originals and evidence identities.
- Existing `#[path = "../../mori-ledger/tests/support/mod.rs"]` mounts in
  `prepare_call`, `installed_call` and `signed_installed_call` remain unchanged
  in the native-only targets. They are existing deviations from the no-remount
  rule above; this cut adds none and does not claim to resolve them.

## CLI

- [`clap`](https://crates.io/crates/clap) with derive, styled with cargo's colors
  through `clap::builder::styling::Styles`.
- Cargo-style status lines on stderr: a right-aligned, bold, colored verb, then
  details (`    Checking invoice.mori`, `    Finished check in 3.2ms`). Use
  `anstream` and `anstyle`.
- Results go to stdout. `--json` turns status lines off. JSON reports are our
  own format, not the TypeScript beta's; spans are UTF-8 byte ranges and every
  diagnostic has exactly one primary label.
- Global `-q`/`--quiet`, `-v`/`--verbose` and `--color` (`auto` respects
  `NO_COLOR` and non-terminal output). Verbose adds stage timings and
  token/node counts.
- Final verbs say exactly what happened, for example `Prepared (unqualified)`.
  Never imply proof, signature authority or ledger acceptance that did not occur.


END FILE units/RUST-diagnostics-testing-cli.md


BEGIN FILE units/permissionless-product-boundary.md | SHA256 dc2de73287cd8cc172a409e10cfbe404badea933842d292452cceb146464b4f3 | BYTES 2230
# Moriarty: permissionless language of provable intention

Status: target architecture, specified-only where not linked to implementation. Authority: user clarification, 2026-09-19.

Moriarty is a programming language for all DeFi developers on Midnight. Any developer may author, compile, prove, publish and deploy programs in the supported language without approval from Moriarty maintainers, an agent council, Foreman, a hosted solver or a program registry. Objective Midnight validity rules, resource costs and asset-owner authorization still apply.

This document supersedes conflicting product-scope and administrative-deployment requirements in earlier designs. Historical receipts remain evidence of their original scope. It does not declare missing proof or ledger capabilities implemented.

## Consolidated design and delivery

The [consolidated vision and design](MORIARTY-CONSOLIDATED-DESIGN.md), [single U0–U7 roadmap](../ROADMAP.md), and [next ZKIR/recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) integrate the research agenda. P/C/K identifiers remain requirement lineage; they are not independent delivery queues. The user supplied a planning assumption on 2026-09-19 that comprehensive Midnight recursion arrives in about six months. Plan full native recursive compliance and private multi-parent composition accordingly; current interface limitations do not remove that scope. The timing is not a verified release claim.

## Product boundary

The product is the language, semantics, compiler, proof interfaces, verifier integration and developer tooling. ACTUS, loans, swaps, vaults and other DeFi applications test expressivity and conformance; they are libraries and examples, not an exhaustive list of programs developers may deploy. Supported constructs compose into new programs. New language primitives require sound semantics and implementation; they do not justify approval of individual programs built from existing constructs.

A managed multichain router, identity provider, adapter catalog or pricing service can be an optional application. None is required to use Moriarty. Application-specific access control remains expressible without becoming a language-wide deployment restriction.

END FILE units/permissionless-product-boundary.md


BEGIN FILE SOURCE-RESULT-CUSTODY.json | SHA256 e8f7c2fbb534b58a596d79ca20fef2cf6b05f69291116bb87633dd3ba4625df2 | BYTES 4915
{
  "format": "inspect-header-exact-source-result-custody/1",
  "checkedAt": "2026-10-18T17:07:38.802273+00:00",
  "reportedCommit": "4c03df51f78337d1a9e17cfaaabbbea112fef0df",
  "reportedTree": "27b0211b04921485afae58603811549b7b93cb7d",
  "reportedParent": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "rootPostCommitBinding": "actual/POST-COMMIT-BINDING.json",
  "rootVsBuilder": "Root reports Git exact2changedpaths,cleanHEAD and903afterpins match Git+worktree; builder performs noGit and compares all903 current afterpin filebytes itself.",
  "all903AfterPinsMatchWorktree": true,
  "productionExactFirstProbeReplacementOnly": true,
  "nativeTargetBlockAndAllDownstreamHelpersVerbatim": true,
  "beforeAfterPinsCount": 903,
  "soleBetweenPhaseProductionMutation": [
    {
      "path": "crates/mori/src/commands/inspect.rs",
      "beforeSha256": "e17139f43eaefc07530a2f9636e0b42fe2410ce8bcef1710f206ebc1bcc4e5c8",
      "afterSha256": "0016e768e086f0e6abdd616e4765838df8563392aaf9a7ea708e8e63bb9d8c78"
    }
  ],
  "changedPathHashes": [
    {
      "path": "crates/mori/src/commands/inspect.rs",
      "sha256": "0016e768e086f0e6abdd616e4765838df8563392aaf9a7ea708e8e63bb9d8c78"
    },
    {
      "path": "crates/mori/tests/inspect_header_dispatch.rs",
      "sha256": "f925bcc0ccd78f5a9cc9512f03718830a39475ddd98385531eaff6e90a9b43c3"
    }
  ],
  "formattingQualification": {
    "originalAuthorTestSha256": "38d4b9fd8da39b6b3b2a03f78cd93f20abaa3fb77fb2f264aa5b68b25b969f9b",
    "actualTestSha256": "f925bcc0ccd78f5a9cc9512f03718830a39475ddd98385531eaff6e90a9b43c3",
    "actualTestEqualsCommittedAndBothPhasePins": true,
    "exactReconstruction": "Replace precisely one five-line four-argument array layout with same one-line array; complete after bytes equal that replacement. All other bytes unchanged.",
    "allRawExpectationLiteralsByteIdentical": true,
    "formattingReceipt": "actual/FORMAT-ONLY-DELTA.json",
    "formatExactDiff": "FORMAT-EXACT.diff",
    "originalPrivateCandidateRustfmtExit": 1,
    "rootFormattedRustfmtExit": 0
  },
  "originalAuthorManifestEntries": 41,
  "originalAuthorPhysicalFiles": 43,
  "allAuthorManifestAndFreezeDigestsMatch": true,
  "allFourPhaseLogDigestPairsMatch": true,
  "completeRawCaptureComparisons": [
    {
      "phase": "red",
      "case": "unknown",
      "inputSourceExact": true,
      "producerErrorCompleteExact": true,
      "offset": 16,
      "exit": 1,
      "stdoutCompleteExactEmpty": true,
      "stderrMatchesFrozenRepair": false,
      "stderrSha256": "fa1bc7c97be79fd483f0af60ff91f89dcba675cb3854ea0acee897560157881f"
    },
    {
      "phase": "red",
      "case": "missing",
      "inputSourceExact": true,
      "producerErrorCompleteExact": true,
      "offset": 9,
      "exit": 1,
      "stdoutCompleteExactEmpty": true,
      "stderrMatchesFrozenRepair": false,
      "stderrSha256": "fb2f278f2014913605551ef83ac2f016a3460e13048c21c66f6fc02d14df5e94"
    },
    {
      "phase": "red",
      "case": "commented",
      "inputSourceExact": true,
      "producerErrorCompleteExact": true,
      "offset": 25,
      "exit": 1,
      "stdoutCompleteExactEmpty": true,
      "stderrMatchesFrozenRepair": false,
      "stderrSha256": "fa1bc7c97be79fd483f0af60ff91f89dcba675cb3854ea0acee897560157881f"
    },
    {
      "phase": "green",
      "case": "unknown",
      "inputSourceExact": true,
      "producerErrorCompleteExact": true,
      "offset": 16,
      "exit": 1,
      "stdoutCompleteExactEmpty": true,
      "stderrMatchesFrozenRepair": true,
      "stderrSha256": "bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e"
    },
    {
      "phase": "green",
      "case": "missing",
      "inputSourceExact": true,
      "producerErrorCompleteExact": true,
      "offset": 9,
      "exit": 1,
      "stdoutCompleteExactEmpty": true,
      "stderrMatchesFrozenRepair": true,
      "stderrSha256": "ee5e996d685f0f7efdd99d69b52a63edb2503d8624504bc571c8014ccb1a1551"
    },
    {
      "phase": "green",
      "case": "commented",
      "inputSourceExact": true,
      "producerErrorCompleteExact": true,
      "offset": 25,
      "exit": 1,
      "stdoutCompleteExactEmpty": true,
      "stderrMatchesFrozenRepair": true,
      "stderrSha256": "bde52d25b439cafd2526c6df31c3060a763e44079990dd4c6267dda196fb241e"
    }
  ],
  "sourceManifestSha256": "7caefc46699293788561f653951b4a620526883ab21ab9387a58f502e7b2f990",
  "physicalVsMandatoryModelScope": "Full903row before/after source manifests and all original author files preserved custody. Exact summaries,both fullchangedbodies,fullbefore,originalformatbefore,formatdiff/receipt,allrawcaptures+semantics+receipts,driver and selected control source/oraclevalues mandatory. Do not claim whole-repository or full-manifest model reading. Exact duplicate representations are read once with physical hash equality; no candidate or used semantic field omitted."
}

END FILE SOURCE-RESULT-CUSTODY.json


BEGIN FILE SOURCE-UNIT-MAP.json | SHA256 aba184aea5024f89202ae1b822c0926d41a13b35a57279d461930d3ce0f40427 | BYTES 16383
[
  {
    "packetPath": "units/core_io-read_source.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/commands/core_io.rs",
    "originSha256": "6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594",
    "startLine": 5,
    "endLine": 22,
    "unitSha256": "e8f1a0d5cd1c2eeee11708b9ceaab206b8f08435fd6e072259edc1932203ce33",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/core_io-operation.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/commands/core_io.rs",
    "originSha256": "6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594",
    "startLine": 158,
    "endLine": 164,
    "unitSha256": "51d63b55691c08ef387fc01056e995f95500a286660ef79ec6fea42edb8d9f6c",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/core_io-print.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/commands/core_io.rs",
    "originSha256": "6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594",
    "startLine": 247,
    "endLine": 251,
    "unitSha256": "7f1351583d72965b909c84ede5884a48fba83dc868dde6f57ebcd4f165a868cd",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/simulate-print-stage-report.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/commands/simulate.rs",
    "originSha256": "d1f700c8b31112a682325a32e3336e9c9168a4b7556c9184504012c15037cd08",
    "startLine": 152,
    "endLine": 160,
    "unitSha256": "a10447885a0129d50bb969d563e3e0daf84734db2f669d4da9b10d58e055db85",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/profile-constants.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core/src/ir.rs",
    "originSha256": "0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d",
    "startLine": 1,
    "endLine": 9,
    "unitSha256": "2ad2722658f82d62e485b670182ecfc124c615114a2d1464e9a23e7d70d237fb",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/core-Error-dto.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core/src/ir.rs",
    "originSha256": "0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d",
    "startLine": 466,
    "endLine": 483,
    "unitSha256": "27c7090a3c80b1316a670a7660ff7bdd2f4b289ba017379ac6c9616180404613",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/existing-library-header-test.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core/tests/language.rs",
    "originSha256": "c9520a4e5ab35f77b9da24030f43b997b3135fba865fde4dd949cd35b7d5421c",
    "startLine": 428,
    "endLine": 475,
    "unitSha256": "50711fa2ad2f50dc82327e1c6b078b8c65b9880833640873e3d073784a8481c9",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/stage-profile-labels.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core/src/stage/mod.rs",
    "originSha256": "e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23",
    "startLine": 27,
    "endLine": 42,
    "unitSha256": "27c6e67cdd971f72eebe9dc29c7ae8eeef3c60552d8796dfc86ccd8d9b6eaa57",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/staged-program-accessors.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core/src/stage/mod.rs",
    "originSha256": "e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23",
    "startLine": 44,
    "endLine": 71,
    "unitSha256": "090e597b90cc121029a5b61844b8a93be11ab264bf44840a11277357c0569de0",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/stage-load.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core/src/stage/mod.rs",
    "originSha256": "e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23",
    "startLine": 73,
    "endLine": 92,
    "unitSha256": "e958f90290985ad748278626e1cc591d3885f4e174a0bdf3c52c5a5c88c53e25",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/cli-global-options.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/cli.rs",
    "originSha256": "a1046abb0191c29bbc5b10e169ba3ce81a4b71324a48246d0abde34eb81c3988",
    "startLine": 37,
    "endLine": 63,
    "unitSha256": "d39eb77f73e940188e0746b96231f679ee88c9bad30c4072a49a7e6761137fd2",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/cli-color-conversion.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/cli.rs",
    "originSha256": "a1046abb0191c29bbc5b10e169ba3ce81a4b71324a48246d0abde34eb81c3988",
    "startLine": 99,
    "endLine": 114,
    "unitSha256": "b3040b38b5cfa64baa23e2c50da404635b86adc6c7ca5b9d61bc049827df1fc5",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/mori-diagnostics-render-production.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-diagnostics/src/render.rs",
    "originSha256": "b28d889ac235f07a7513995ef5bbffd42b1999ce95d8cc2056bd2774f5559bdc",
    "startLine": 1,
    "endLine": 51,
    "unitSha256": "064524e1e7dc533a5e1fc7136aeac478cd2d39e2e20f5b202f455a62ccb45929",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/check-json-production.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/src/json.rs",
    "originSha256": "6f68940fe32eced929b8af229336eb07e9af745a78b32f45966858c7c382a3aa",
    "startLine": 1,
    "endLine": 195,
    "unitSha256": "58e0745bc8829225c06f355a254f8539f416619f9fd6253c565f7d27ee5c4dbb",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/capabilities-positive-helpers.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/tests/capabilities.rs",
    "originSha256": "7743a40ccf758ac71af613f2957a1cfcee6ea05473578478a130a3d050b388a3",
    "startLine": 1,
    "endLine": 69,
    "unitSha256": "723bd317d6dfe1a2c9063030127230b2ff85d8eb10e4370ad8e161731f78b1ad",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/capabilities-positive-test.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/tests/capabilities.rs",
    "originSha256": "7743a40ccf758ac71af613f2957a1cfcee6ea05473578478a130a3d050b388a3",
    "startLine": 186,
    "endLine": 234,
    "unitSha256": "d85df856dfe8c3ff666b2bc16158045e2e0341716d423002257d3ab06554d496",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/core2-positive-helpers.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/tests/core_continuations.rs",
    "originSha256": "051a19463f2ffbfb632ec63a6a17a0aba654e5fe477b39c870531efe046d9563",
    "startLine": 1,
    "endLine": 106,
    "unitSha256": "134f6432e09ace6e416e977c365ed90342385b7ef63d0b0aca64fed8156dbe8f",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/core2-positive-test.rs",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori/tests/core_continuations.rs",
    "originSha256": "051a19463f2ffbfb632ec63a6a17a0aba654e5fe477b39c870531efe046d9563",
    "startLine": 237,
    "endLine": 248,
    "unitSha256": "fd01a5831321b25da9443400fa5f9400539a8295d33ed51afd456fe1a1736fa4",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-graphical-handler-types.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 1,
    "endLine": 49,
    "unitSha256": "0fe9c622f285b27c1619555a2736c07cea545f4944d1fe54b602af3ff0487d77",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-new-themed.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 75,
    "endLine": 93,
    "unitSha256": "a0418146dbab6e4d245629dc94b1f8590b1c919c826af972dcd91876778eba7c",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-with-links.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 102,
    "endLine": 109,
    "unitSha256": "f39509e7c0bf3d3cdb14e960d53ab8924f767f48f831257c1a95cb0a4f9b63d9",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-with-width.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 159,
    "endLine": 162,
    "unitSha256": "d1c7ef18f0768038cf068be5ddd7f06ab0388e247ad568c6a3b474eb861b89c9",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-report.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 242,
    "endLine": 248,
    "unitSha256": "b85413d207d58b0620fb5572526025dfc899320970872a8e3cf2802fb21a2fda",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-report-inner.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 250,
    "endLine": 279,
    "unitSha256": "1e5f6beb356eb45119adaaefbc7f523970ca492077a38150030ac24c3b2ed7a7",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-header.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 281,
    "endLine": 324,
    "unitSha256": "a51c2ea70dd106980305f46faa6ece57bbb686aef70ad5ce18f0230b89a9054b",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-causes.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 326,
    "endLine": 425,
    "unitSha256": "12a7a08435245f872af1288ba569df21f1708a843311e21b300f1d186bbf934c",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-footer.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 427,
    "endLine": 445,
    "unitSha256": "d6f42b639ccfc1fe64393362931f6e3b7c0b2e9dbf0801c1abe1c0679a5d24b5",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-related.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 447,
    "endLine": 533,
    "unitSha256": "03fc636df776fe74f10ecd5d703fc63d3d9f101501f01584fa715f416d39adef",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-render-snippets.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 535,
    "endLine": 610,
    "unitSha256": "500e333da03b577f69e9bdde09429eb8e7321dd8befa38fcffc6aaac7deb9a8e",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/miette-wrap.rs",
    "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/grammar-current5f7/renderer-source/miette-7.6.0/src/handlers/graphical.rs",
    "originSha256": "3c9569415d286bfbf09ef750d8fc5b72e61f3a984499bb0aaf321c1615737414",
    "startLine": 997,
    "endLine": 1028,
    "unitSha256": "baf49f23157793c716b7e853bb13ec35249f63f100a2a5d088bf19ba419e27b4",
    "scope": "Complete named unit(s); not a full transitive unchanged module closure"
  },
  {
    "packetPath": "units/UNI-018.md",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
    "originSha256": "2da7f711ab50794cd7a94756c4e46ca450e65e2127610aa723805caaf7a8de76",
    "startLine": 190,
    "endLine": 200,
    "unitSha256": "97a9c37cc07986731b005ea281e07e4449e90617f8a77c2bc57a43acaf1cf1d5",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/RUST-diagnostics-testing-cli.md",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/RUST_GUIDELINES.md",
    "originSha256": "4bd3702dd3d488446f58246e2b1cb86525e2b6bef0a2e3cf5589e398c0642ad2",
    "startLine": 190,
    "endLine": 322,
    "unitSha256": "a7571b648e1a7f8840f1959043a8ff97c17c9c50fdace1afb40bf62a45fd0fe2",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  },
  {
    "packetPath": "units/permissionless-product-boundary.md",
    "origin": "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/docs/MORIARTY-PRODUCT-CONTRACT.md",
    "originSha256": "d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396",
    "startLine": 1,
    "endLine": 17,
    "unitSha256": "dc2de73287cd8cc172a409e10cfbe404badea933842d292452cceb146464b4f3",
    "scope": "Complete named unit(s); no expanded transitive unchanged module closure"
  }
]

END FILE SOURCE-UNIT-MAP.json


BEGIN FILE CANDIDATE.json | SHA256 1e4d9a21e0a1086a9ffd082ea4ef767a010050a75f715d331a17df1c716a35e9 | BYTES 811
{
  "reviewToken": "mori-inspect-header-4c03df51-1c16daa2586f6b619ae6",
  "head": "4c03df51f78337d1a9e17cfaaabbbea112fef0df",
  "tree": "27b0211b04921485afae58603811549b7b93cb7d",
  "parent": "615eceee9c6250d2f203cb5807d5c75482dbdb5d",
  "changedPaths": [
    {
      "path": "crates/mori/src/commands/inspect.rs",
      "sha256": "0016e768e086f0e6abdd616e4765838df8563392aaf9a7ea708e8e63bb9d8c78"
    },
    {
      "path": "crates/mori/tests/inspect_header_dispatch.rs",
      "sha256": "f925bcc0ccd78f5a9cc9512f03718830a39475ddd98385531eaff6e90a9b43c3"
    }
  ],
  "fullBefore": "actual/INSPECT-BEFORE.rs",
  "newTestBefore": "absentinparent;rootformattedfrozenauthorcandidatebeforeRED;sametestbothphases",
  "noNativeTargetBlockChange": "Fullinspectnativehelperbytesretained;noexecutionorqualification."
}

END FILE CANDIDATE.json
