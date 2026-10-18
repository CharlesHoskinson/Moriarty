//! Detailed local reference diagnostics. Existing runtime replay is unchanged.
use crate::{Error, Execution, Program, Scenario, simulate};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReplayDifference {
    pub step: Option<u16>,
    pub field: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq, thiserror::Error)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum DetailedReplayError {
    #[error("{0}")]
    Execution(Error),
    #[error("Reference divergence at {}", .0.field)]
    Divergence(ReplayDifference),
}
fn child(path: &str, key: &str) -> String {
    format!("{path}/{}", key.replace('~', "~0").replace('/', "~1"))
}
fn first(actual: &Value, candidate: &Value, path: &str) -> Option<String> {
    match (actual, candidate) {
        (Value::Object(a), Value::Object(b)) => {
            for key in a.keys().chain(b.keys()).collect::<BTreeSet<_>>() {
                let next = child(path, key);
                match (a.get(key), b.get(key)) {
                    (Some(x), Some(y)) => {
                        if let Some(d) = first(x, y, &next) {
                            return Some(d);
                        }
                    }
                    _ => return Some(next),
                }
            }
            None
        }
        (Value::Array(a), Value::Array(b)) => {
            for (i, (x, y)) in a.iter().zip(b).enumerate() {
                if let Some(d) = first(x, y, &child(path, &i.to_string())) {
                    return Some(d);
                }
            }
            (a.len() != b.len()).then(|| path.into())
        }
        _ => (actual != candidate).then(|| path.into()),
    }
}
/// Compare full execution fields in semantic order, before derived hashes.
/// Step ordinals come from traversal, never from a candidate's index field.
pub fn compare_execution(
    actual: &Execution,
    candidate: &Execution,
) -> Result<(), ReplayDifference> {
    // Core's concrete serializers contain no fallible custom data.
    let a = serde_json::to_value(actual).expect("Core execution serialization");
    let b = serde_json::to_value(candidate).expect("Core execution serialization");
    let compare = |field: &str, step| match first(&a[field], &b[field], &format!("/{field}")) {
        Some(field) => Err(ReplayDifference { step, field }),
        None => Ok(()),
    };
    for field in ["semantics", "action", "pre"] {
        compare(field, None)?;
    }
    for i in 0..actual.steps.len().max(candidate.steps.len()) {
        let step = Some(i as u16);
        if actual.steps.get(i).is_none() || candidate.steps.get(i).is_none() {
            return Err(ReplayDifference {
                step,
                field: format!("/steps/{i}"),
            });
        }
        for key in ["index", "statement", "value", "snapshot", "effects"] {
            if let Some(field) = first(
                &a["steps"][i][key],
                &b["steps"][i][key],
                &format!("/steps/{i}/{key}"),
            ) {
                return Err(ReplayDifference { step, field });
            }
        }
    }
    for field in ["effects", "post"] {
        compare(field, None)?;
    }
    Ok(())
}
pub fn replay_detailed(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    candidate: &Execution,
) -> Result<(), DetailedReplayError> {
    let actual = simulate(program, action, scenario).map_err(DetailedReplayError::Execution)?;
    compare_execution(&actual, candidate).map_err(DetailedReplayError::Divergence)
}
