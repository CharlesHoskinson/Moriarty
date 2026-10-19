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
