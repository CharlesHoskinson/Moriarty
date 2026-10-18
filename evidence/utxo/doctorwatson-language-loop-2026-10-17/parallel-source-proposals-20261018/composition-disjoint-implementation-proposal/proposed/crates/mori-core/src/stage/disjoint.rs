//! Local product of two staged actions with disjoint state and asset domains.
use super::{PendingFrame, StageEvent, StageOutcome, StageRejection, StagedProgram};
use crate::{AssetId, Effect, Scenario, Snapshot, Statement, UInt};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub const MAX_INVOCATIONS: usize = 256;

pub struct Member {
    pub program: StagedProgram,
    pub action: String,
    pub scenario: Scenario,
    pub cap: UInt,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("local disjoint product: {0}")]
    Shape(String),
    #[error("member {member}: {rejection}")]
    Branch {
        member: usize,
        rejection: StageRejection,
        /// Start may have attempted an earlier member before this refusal.
        attempted_before: Vec<UInt>,
    },
}

#[derive(Clone)]
struct Observation {
    member: usize,
    outcome: StageOutcome,
}

/// Opaque local state: member boundaries come only from actual stage outputs.
/// It has no decoder, authentication or consumption tracking.
pub struct Pair {
    programs: [StagedProgram; 2],
    caps: [UInt; 2],
    cap_total: UInt,
    attempted_total: UInt,
    opening: Snapshot,
    latest: [StageOutcome; 2],
    history: Vec<Observation>,
}

fn shape(message: &str) -> Error {
    Error::Shape(message.into())
}

fn sum(left: UInt, right: UInt) -> Result<UInt, Error> {
    left.0.checked_add(right.0).map(UInt).ok_or_else(|| shape("logical sum exceeds UInt"))
}

fn declared_domains(program: &StagedProgram) -> (BTreeSet<&str>, BTreeSet<&AssetId>, BTreeSet<&str>) {
    let program = program.program();
    let states = program.states.iter().map(|state| state.name.as_str()).collect();
    let assets = program.assets.iter().map(|asset| &asset.identity).collect();
    let duties = program.actions.iter().flat_map(|action| &action.statements).filter_map(|stmt| {
        match stmt {
            Statement::Liability { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }).collect();
    (states, assets, duties)
}

fn disjoint(left: &StagedProgram, right: &StagedProgram) -> Result<(), Error> {
    let (left_state, left_assets, left_duties) = declared_domains(left);
    let (right_state, right_assets, right_duties) = declared_domains(right);
    if !left_state.is_disjoint(&right_state)
        || !left_assets.is_disjoint(&right_assets)
        || !left_duties.is_disjoint(&right_duties)
    {
        return Err(shape("requires disjoint declared state names, exact assets and duty names"));
    }
    Ok(())
}

fn merge(left: &Snapshot, right: &Snapshot) -> Result<Snapshot, Error> {
    let mut state = left.state.clone();
    for (name, value) in &right.state {
        if state.insert(name.clone(), value.clone()).is_some() {
            return Err(shape("state domains overlap"));
        }
    }
    let mut balances = left.balances.clone();
    balances.extend(right.balances.iter().cloned());
    balances.sort_by(|a, b| (&a.asset, &a.account).cmp(&(&b.asset, &b.account)));
    if balances.windows(2).any(|rows| rows[0].asset == rows[1].asset && rows[0].account == rows[1].account) {
        return Err(shape("balance domains overlap"));
    }
    let mut liabilities = left.liabilities.clone();
    liabilities.extend(right.liabilities.iter().cloned());
    liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    if liabilities.windows(2).any(|rows| rows[0].name == rows[1].name) {
        return Err(shape("duty domains overlap"));
    }
    Ok(Snapshot { state, balances, liabilities })
}

fn boundary(outcome: &StageOutcome) -> &Snapshot {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => &continuation.boundary,
        StageOutcome::Complete { segment, .. } => &segment.execution.post,
    }
}

fn start_pre(outcome: &StageOutcome) -> Result<&Snapshot, Error> {
    match outcome {
        StageOutcome::Waiting { segment: Some(segment), .. }
        | StageOutcome::Complete { segment, .. } => Ok(&segment.execution.pre),
        _ => Err(shape("start did not supply a committed segment")),
    }
}

fn attempted(outcome: &StageOutcome) -> UInt {
    match outcome {
        StageOutcome::Waiting { attempted, .. } | StageOutcome::Complete { attempted, .. } => *attempted,
    }
}

fn committed(outcome: &StageOutcome) -> UInt {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => continuation.logical.committed,
        StageOutcome::Complete { committed, .. } => *committed,
    }
}

fn frame(outcome: &StageOutcome) -> PendingFrame {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => PendingFrame {
            site: continuation.site,
            captures: continuation.captures.clone(),
        },
        StageOutcome::Complete { segment, .. } => segment.frame_post.clone(),
    }
}

fn effects(outcome: &StageOutcome) -> &[Effect] {
    match outcome {
        StageOutcome::Waiting { segment: Some(segment), .. }
        | StageOutcome::Complete { segment, .. } => &segment.execution.effects,
        StageOutcome::Waiting { segment: None, .. } => &[],
    }
}

impl Pair {
    /// Start uses a declared left-then-right reference linearization. Neither
    /// member receives the other's boundary, effects, captures or inputs.
    pub fn start([left, right]: [Member; 2]) -> Result<Self, Error> {
        disjoint(&left.program, &right.program)?;
        let caps = [left.cap, right.cap];
        let cap_total = sum(caps[0], caps[1])?;
        let programs = [left.program, right.program];
        let first = super::start(&programs[0], &left.action, left.scenario, Some(caps[0]))
            .map_err(|rejection| Error::Branch { member: 0, rejection, attempted_before: vec![] })?;
        let second = super::start(&programs[1], &right.action, right.scenario, Some(caps[1]))
            .map_err(|rejection| Error::Branch {
                member: 1, rejection, attempted_before: vec![attempted(&first)],
            })?;
        let attempted_total = sum(attempted(&first), attempted(&second))?;
        let opening = merge(start_pre(&first)?, start_pre(&second)?)?;
        merge(boundary(&first), boundary(&second))?;
        let history = vec![
            Observation { member: 0, outcome: first.clone() },
            Observation { member: 1, outcome: second.clone() },
        ];
        Ok(Self { programs, caps, cap_total, attempted_total, opening, latest: [first, second], history })
    }

    /// Resume one member from its own actual continuation. Publication replaces
    /// only that member after its stage call and complete frame merge succeed.
    pub fn resume(&mut self, member: usize, event: StageEvent) -> Result<(), Error> {
        if self.history.len() >= MAX_INVOCATIONS {
            return Err(shape("invocation limit reached"));
        }
        let Some(StageOutcome::Waiting { continuation, .. }) = self.latest.get(member) else {
            return Err(shape("member must identify a pending branch"));
        };
        let outcome = super::resume(
            &self.programs[member], continuation.clone(), event, Some(&continuation.action), None,
        ).map_err(|rejection| Error::Branch { member, rejection, attempted_before: vec![] })?;
        let mut latest = self.latest.clone();
        latest[member] = outcome.clone();
        merge(boundary(&latest[0]), boundary(&latest[1]))?;
        let attempted_total = sum(self.attempted_total, attempted(&outcome))?;
        self.latest = latest;
        self.attempted_total = attempted_total;
        self.history.push(Observation { member, outcome });
        Ok(())
    }

    /// The report contains the complete product frame and all original member
    /// envelopes. Effects retain invocation order, then source statement order.
    pub fn report(&self) -> Result<Value, Error> {
        let post = merge(boundary(&self.latest[0]), boundary(&self.latest[1]))?;
        let committed = [committed(&self.latest[0]), committed(&self.latest[1])];
        let committed_total = sum(committed[0], committed[1])?;

        let ordered_effects: Vec<_> = self.history.iter().flat_map(|item| effects(&item.outcome)).collect();
        let history: Vec<_> = self.history.iter().map(|item| {
            json!({"member":item.member,"outcome":super::outcome_report(Ok(item.outcome.clone()))})
        }).collect();
        let members: Vec<_> = self.latest.iter().map(|outcome| {
            super::outcome_report(Ok(outcome.clone()))
        }).collect();
        let frames = self.latest.each_ref().map(frame);
        let waiting = self.latest.iter().any(|outcome| matches!(outcome, StageOutcome::Waiting { .. }));
        Ok(json!({
            "format":"moriarty-local-disjoint-draft/1",
            "status":if waiting { "waiting" } else { "complete" },
            "qualification":"local-reference-stage",
            "composition_qualification":"unadopted-local-disjoint-product",
            "authentication":"none", "consumption":"untracked",
            "ordering":{"start":[0,1],"effects":"invocation-then-source-order"},
            "pre":self.opening, "post":post, "frames":frames,
            "members":members, "history":history, "effects":ordered_effects,
            "logical":{"caps":self.caps,"cap_total":self.cap_total,
                "committed":committed,"committed_total":committed_total,
                "attempted_total":self.attempted_total},
            "residual":super::residual(&post),
        }))
    }
}
