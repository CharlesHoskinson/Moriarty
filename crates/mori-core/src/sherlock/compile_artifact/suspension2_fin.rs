//! Financial route of `sherlock-helper-microstep/2` (`--source-profile
//! moriarty/2 --action A --scenario S`): reports and checkpoints over the
//! `crate::stage::sherlock2` XFin engine. The checkpoint body is the /2 header
//! lanes followed by the canonical JSON of the complete `FinImage2`.
use serde_json::{Value, json};

use super::{
    CompileFailure, Envelope, Header, ROUTE_FIN, Rd, Report, admit_observation, exhaust_name,
    failure, hex, observation_digest, open, parse_draft2_config, refused, report, seal, sort_name,
};
use crate::sherlock::Type;
use crate::sherlock::helper_machine::shared::owner_suspend::ScalarC;
use crate::stage::sherlock2::{self as fin2, FinImage2, FinOutcome2, FinResume};
use crate::stage::{NamedValue, StagedProgram};
use crate::{Scenario, UInt};

fn sherlock_sort(t: crate::Type) -> Type {
    match t {
        crate::Type::UInt => Type::UInt,
        crate::Type::Bool => Type::Bool,
    }
}

fn fuel_of(config_source: &str) -> Result<u128, CompileFailure> {
    parse_draft2_config(config_source)?;
    let value = crate::json::parse(config_source).map_err(|e| CompileFailure::core("config", e))?;
    value["fuel"]["value"]
        .as_str()
        .and_then(|s| s.parse::<u128>().ok())
        .ok_or_else(|| CompileFailure::input("config", "config fuel is not a Nat"))
}

fn load(source: &str) -> Result<StagedProgram, CompileFailure> {
    crate::stage::load(source).map_err(|e| CompileFailure::core("parse", e))
}

fn admission_failure(stage: &'static str, error: crate::Error) -> Report {
    failure(CompileFailure {
        stage,
        actual_error: json!(error),
        evaluation_performed: false,
        events_forwarded: "0".into(),
        completed: false,
    })
}

fn fin_block(image: &FinImage2) -> Result<Value, CompileFailure> {
    let (gross, fees, receipt) = fin2::accounting(&image.committed_effects).ok_or_else(|| {
        CompileFailure::input("accounting", "committed accounting exceeds UInt128")
    })?;
    Ok(json!({
        "committed": {
            "snapshot": image.committed,
            "effects": image.committed_effects,
            "logical_work": image.committed_logical.0.to_string(),
        },
        "residual_duties": fin2::residual_duties(image)
            .iter()
            .map(|l| json!({"name": l.name, "amount": l.amount.0.to_string(), "debtor": l.debtor, "creditor": l.creditor}))
            .collect::<Vec<_>>(),
        "accounting": {
            "gross_debit": gross.to_string(),
            "fees": fees.to_string(),
            "recipient_receipt": receipt.to_string(),
        },
        "discarded_tentative": {
            "effects": image.discarded_effects,
            "segment_index": image.segment,
        },
        "retained_by_policy": {
            "policy": image.policy,
            "effects": image.retained_by_policy,
        },
        "phase_policy": image.policy,
        "ledger_rollback": "none-inferred",
    }))
}

fn body(header: &Header, image: &FinImage2) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    header.put(&mut out);
    out.extend_from_slice(&crate::intent::canonical(image).map_err(|e| e.message)?);
    Ok(out)
}

fn publish(sp: &StagedProgram, image: FinImage2, mut header: Header, attempted: UInt) -> Report {
    header.wait_site = match image.outcome {
        FinOutcome2::Waiting { site } => site,
        _ => 0,
    };
    header.capture.clear();
    let bytes = match body(&header, &image) {
        Ok(b) => b,
        Err(e) => {
            return failure(CompileFailure::input("checkpoint", e));
        }
    };
    let fin = match fin_block(&image) {
        Ok(v) => v,
        Err(f) => return failure(f),
    };
    let (bytes, ck) = seal(ROUTE_FIN, bytes);
    let committed = image.consumed_work.0.to_string();
    let attempted = attempted.0.to_string();
    let (value, exit) = match &image.outcome {
        FinOutcome2::Waiting { .. } => {
            let (site, label, binders) = fin2::waiting_site(sp, &image).expect("waiting site");
            let binders: Vec<(String, Type)> = binders
                .iter()
                .map(|(n, t)| (n.clone(), sherlock_sort(*t)))
                .collect();
            (
                json!({
                    "status": "waiting",
                    "site": site,
                    "label": label,
                    "action": image.action,
                    "checkpoint": hex(&ck),
                    "binders": binders.iter().map(|(n, t)| json!({"name": n, "type": sort_name(t)})).collect::<Vec<_>>(),
                    "committed_work": committed,
                    "attempted_work": attempted,
                    "resume_epoch": header.resume_epoch.to_string(),
                    "frames": [],
                    "fin": fin,
                }),
                0,
            )
        }
        FinOutcome2::Halted => (
            json!({"status": "halted", "value": {"type": "Unit", "value": "unit"},
                   "committed_work": committed, "checkpoint": hex(&ck),
                   "resume_epoch": header.resume_epoch.to_string(), "fin": fin}),
            0,
        ),
        FinOutcome2::Rejected { code, message } => (
            json!({"status": "rejected", "code": code, "error": message,
                   "committed_work": committed, "attempted_work": attempted,
                   "checkpoint": hex(&ck), "fin": fin}),
            1,
        ),
        FinOutcome2::Exhausted { reason } => (
            json!({"status": "exhausted", "reason": reason, "committed_work": committed,
                   "checkpoint": hex(&ck), "fin": fin}),
            1,
        ),
        FinOutcome2::Running => {
            return failure(CompileFailure::input(
                "run",
                "the run stopped while Running",
            ));
        }
    };
    let _ = exhaust_name;
    report(value, exit, Some(bytes))
}

/// Financial start under profile /2.
pub fn start_fin(
    source: &str,
    config_source: &str,
    _target_source: &str,
    policy: Option<&str>,
    action: &str,
    scenario_source: &str,
) -> Report {
    let fuel = match fuel_of(config_source) {
        Ok(f) => f,
        Err(f) => return failure(f),
    };
    let policy = match fin2::policy_of(policy) {
        Ok(p) => p,
        Err(e) => return admission_failure("config", e),
    };
    let sp = match load(source) {
        Ok(sp) => sp,
        Err(f) => return failure(f),
    };
    let scenario: Scenario = match crate::json::parse(scenario_source)
        .map_err(|e| CompileFailure::core("scenario", e))
        .and_then(|v| {
            serde_json::from_value(v).map_err(|e| CompileFailure::input("scenario", e.to_string()))
        }) {
        Ok(s) => s,
        Err(f) => return failure(f),
    };
    match fin2::start(&sp, action, scenario, policy, fuel) {
        Ok(start) => publish(&sp, start.image, Header::default(), start.attempted),
        Err(e) => admission_failure("admit", e.error),
    }
}

fn decode_body(env: &Envelope) -> Result<(Header, FinImage2), String> {
    let mut r = Rd::new(&env.body);
    let header = Header::get(&mut r)?;
    let json = r.rest();
    let image: FinImage2 =
        serde_json::from_slice(json).map_err(|e| format!("financial image: {e}"))?;
    // The body must be exactly the canonical encoding (one checkpoint identity).
    if crate::intent::canonical(&image).map_err(|e| e.message)? != json {
        return Err("financial image is not canonically encoded".into());
    }
    Ok((header, image))
}

/// Financial resume: R0-R7, then `XFinResumeAccept` or `not_ready`.
pub fn resume_fin(
    source: &str,
    config_source: &str,
    _target_source: &str,
    policy: Option<&str>,
    checkpoint: &[u8],
    observation: &str,
) -> Report {
    if let Err(f) = fuel_of(config_source) {
        return failure(f);
    }
    let env = match open(checkpoint) {
        Ok(env) => env,
        Err(e) => return refused("R0", e),
    };
    if env.route != ROUTE_FIN {
        return refused("R2", "checkpoint belongs to the pure route");
    }
    let (header, image) = match decode_body(&env) {
        Ok(x) => x,
        Err(e) => return refused("R0", e),
    };
    if header.resume_epoch == u64::MAX {
        return refused("R0", "resume_epoch is at its maximum");
    }
    // R1
    let FinOutcome2::Waiting { site } = image.outcome else {
        return refused("R1", "checkpoint is not Waiting at an await site");
    };
    if header.wait_site != site || !header.capture.is_empty() {
        return refused(
            "R0",
            "WAIT_SITE or capture section disagrees with the image",
        );
    }
    // R2
    let sp = match load(source) {
        Ok(sp) => sp,
        Err(f) => {
            return refused(
                "R2",
                format!("supplied source is not admitted: {}", f.actual_error),
            );
        }
    };
    let policy = match fin2::policy_of(policy) {
        Ok(p) => p,
        Err(e) => return refused("R2", e.message),
    };
    if !fin2::same_admission(&sp, &image, policy) {
        return refused(
            "R2",
            "checkpoint does not belong to the supplied program, plan and phase policy",
        );
    }
    let Some((_, label, core_layout)) = fin2::waiting_site(&sp, &image) else {
        return refused(
            "R2",
            "waiting site is not an await site of the supplied program",
        );
    };
    let layout: Vec<(String, Type)> = core_layout
        .iter()
        .map(|(n, t)| (n.clone(), sherlock_sort(*t)))
        .collect();
    // R3-R5
    let values = match admit_observation(observation, &env.ck, site, &label, &layout) {
        Ok(v) => v,
        Err(refusal) => return refusal,
    };
    let named: Vec<NamedValue> = values
        .iter()
        .zip(&layout)
        .map(|(v, (name, _))| NamedValue {
            name: name.clone(),
            value: match v {
                ScalarC::UInt(n) => crate::Value::UInt(UInt(*n)),
                ScalarC::Bool(b) => crate::Value::Bool(*b),
            },
        })
        .collect();
    match fin2::resume(&sp, &image, named) {
        Err((stage, e)) => refused(stage, e.message),
        Ok(FinResume::NotReady {
            attempted,
            guard_error,
        }) => {
            let fin = match fin_block(&image) {
                Ok(v) => v,
                Err(f) => return failure(f),
            };
            let mut v = json!({
                "status": "not_ready",
                "checkpoint": hex(&env.ck),
                "site": site,
                "attempted_work": attempted.0.to_string(),
                "committed_work": image.consumed_work.0.to_string(),
                "fin": fin,
            });
            match guard_error {
                Some(code) => {
                    v["reason"] = json!("guard_error");
                    v["code"] = json!(code);
                }
                None => v["reason"] = json!("guard_false"),
            }
            report(v, 3, Some(checkpoint.to_vec()))
        }
        Ok(FinResume::Advanced {
            image: next,
            attempted,
        }) => {
            let Some(resume_epoch) = header.resume_epoch.checked_add(1) else {
                return refused("R0", "resume_epoch is at its maximum");
            };
            let header = Header {
                resume_epoch,
                last_obs_digest: observation_digest(&env.ck, site, &values),
                wait_site: 0,
                capture: Vec::new(),
            };
            publish(&sp, next, header, attempted)
        }
    }
}
