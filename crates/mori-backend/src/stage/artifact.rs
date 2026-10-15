//! `/4` staged artifact types and their strict decoder. A decoded artifact has
//! the exact frozen shape only. It establishes no proof, native binding, event
//! authentication or consumption: those gates stay unperformed.
use super::frame::{
    EVENT_ROLE, PENDING_ROLES, POST_CAPTURE_ROLE, POST_SITE_ROLE, PRE_CAPTURE_ROLE,
    PRE_SITE_ROLE,
};
use crate::{
    BalanceDescriptor, EffectDescriptor, Error, Gate, LiabilityDescriptor, PublicBinding, Target,
    UNSIGNED_MODE, authorization, error, signed_relation,
};
use mori_core::stage::{
    CaptureSlot, MAX_AWAITS_PER_ACTION, MAX_CAPTURE_SLOTS, MAX_EVENT_BINDERS, SEMANTICS_2,
    STAGE_LABELS, StageBinder,
};
use mori_core::{LOGICAL_COST_PROFILE_2, Type, UInt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const STAGED_ARTIFACT_FORMAT: &str = "moriarty-zkir-artifact/4";
pub const STAGED_ABI_VERSION: &str = "moriarty-public-abi/4";
pub const STAGED_RELATION_VERSION: &str = "moriarty-zkir-relation/4";
pub const STAGE_CONTEXT_FORMAT: &str = "moriarty-stage-context/1";
/// Logical work is static plan metadata: not circuit-enforced and not DUST.
pub const LOGICAL_ENFORCEMENT: &str = "not-circuit-enforced-not-dust";
pub const MAX_STAGED_CIRCUITS: usize = 999;
pub const STAGE_GATES: [&str; 4] = [
    "pending-frame-predecessor-binding",
    "stage-event-authentication",
    "stage-authority-and-consumption",
    "arbitrary-witness-stage-soundness",
];
/// The `/3` predicates, in `/3` order; `/4` lists them first.
const LEGACY_GATES: [&str; 6] = [
    "native-proof-generation",
    "circuit-cost-and-k-measurement",
    "fraud-proof-cost-and-k-measurement",
    "authenticated-ledger-frame-and-phase-correspondence",
    "signed-intent-and-predecessor-history-verification",
    "live-ledger-deployment",
];
const UNPERFORMED: &str = "unperformed";

/// Public positions of one circuit's own pending frame. Pre and post
/// captures have width L; events have the entry site's binder count.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingPositions {
    pub pre_site: u32,
    pub pre_captures: Vec<u32>,
    pub events: Vec<u32>,
    pub post_site: u32,
    pub post_captures: Vec<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedCircuit {
    pub action: String,
    pub segment: u16,
    pub entry_site: u16,
    pub exit_site: u16,
    pub zkir: String,
    pub context_hash: String,
    pub capture_width: u16,
    pub capture_in: Vec<CaptureSlot>,
    pub capture_out: Vec<CaptureSlot>,
    pub event: Vec<StageBinder>,
    pub public_inputs: Vec<PublicBinding>,
    /// Required key; `null` exactly for a zero-await action's single circuit.
    #[serde(deserialize_with = "Option::deserialize")]
    pub pending_positions: Option<PendingPositions>,
    pub instruction_count: u32,
    pub effect_frame: Vec<EffectDescriptor>,
    pub balance_frame: Vec<BalanceDescriptor>,
    pub liability_frame: Vec<LiabilityDescriptor>,
    pub replay_steps: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SegmentLogical {
    pub segment: u16,
    pub segment_logical: UInt,
    pub committed_before: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionLogical {
    pub action: String,
    pub plan_total: UInt,
    pub segments: Vec<SegmentLogical>,
}
/// Totals are sums over all circuits and `max_replay_steps` is the maximum.
/// Nothing is measured, so there are no k or cost-measured fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedResources {
    pub instructions: u32,
    pub zkir_bytes: u32,
    pub artifact_bytes: u32,
    pub public_values: u32,
    pub max_replay_steps: u32,
    #[serde(deserialize_with = "crate::decode_registry_entries")]
    pub liability_registry_entries: u32,
    #[serde(deserialize_with = "crate::decode_registry_ceiling")]
    pub max_liability_registry_entries: u32,
    pub logical_profile: String,
    pub logical_enforcement: String,
    pub logical: Vec<ActionLogical>,
}
/// Decoding validates the whole artifact; there is no From/Into with `/3`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "StagedArtifactFields")]
pub struct StagedArtifact {
    pub format: String,
    pub abi: String,
    pub relation: String,
    pub authorization_mode: String,
    pub program_hash: String,
    pub plan_hash: String,
    pub semantics: String,
    pub qualification: String,
    pub authentication: String,
    pub consumption: String,
    pub target: Target,
    pub circuits: Vec<StagedCircuit>,
    pub resources: StagedResources,
    pub gates: Vec<Gate>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StagedArtifactFields {
    format: String,
    abi: String,
    relation: String,
    authorization_mode: String,
    program_hash: String,
    plan_hash: String,
    semantics: String,
    qualification: String,
    authentication: String,
    consumption: String,
    #[serde(deserialize_with = "crate::decode_artifact_target")]
    target: Target,
    circuits: Vec<StagedCircuit>,
    resources: StagedResources,
    gates: Vec<Gate>,
}
impl TryFrom<StagedArtifactFields> for StagedArtifact {
    type Error = Error;
    fn try_from(f: StagedArtifactFields) -> Result<Self, Error> {
        let artifact = Self {
            format: f.format,
            abi: f.abi,
            relation: f.relation,
            authorization_mode: f.authorization_mode,
            program_hash: f.program_hash,
            plan_hash: f.plan_hash,
            semantics: f.semantics,
            qualification: f.qualification,
            authentication: f.authentication,
            consumption: f.consumption,
            target: f.target,
            circuits: f.circuits,
            resources: f.resources,
            gates: f.gates,
        };
        artifact.validate()?;
        Ok(artifact)
    }
}

fn refuse(message: impl Into<String>) -> Error {
    error("artifact", message)
}

impl StagedArtifact {
    /// The full `/4` decoder predicate, also applied to freshly compiled output.
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.format != STAGED_ARTIFACT_FORMAT
            || self.abi != STAGED_ABI_VERSION
            || self.relation != STAGED_RELATION_VERSION
        {
            return Err(refuse(format!(
                "Expected {STAGED_ARTIFACT_FORMAT}, {STAGED_ABI_VERSION} and {STAGED_RELATION_VERSION}; recompile for this backend revision."
            )));
        }
        if self.authorization_mode != UNSIGNED_MODE
            || self.target.authorization_mode() != UNSIGNED_MODE
        {
            return Err(refuse("Staged artifacts are unsigned only."));
        }
        if self.semantics != SEMANTICS_2 {
            return Err(refuse("Staged artifacts require moriarty-core/2 semantics."));
        }
        let labels = [
            ("qualification", &self.qualification),
            ("authentication", &self.authentication),
            ("consumption", &self.consumption),
        ];
        if labels
            .iter()
            .zip(STAGE_LABELS.iter())
            .any(|((key, value), (k, v))| key != k || value.as_str() != *v)
        {
            return Err(refuse("Staged qualification labels differ from the profile."));
        }
        if self.resources.logical_profile != LOGICAL_COST_PROFILE_2
            || self.resources.logical_enforcement != LOGICAL_ENFORCEMENT
        {
            return Err(refuse("Staged logical-work metadata differs from the profile."));
        }
        authorization::decode_digest(&self.program_hash)?;
        authorization::decode_digest(&self.plan_hash)?;
        for circuit in &self.circuits {
            authorization::decode_digest(&circuit.context_hash)?;
        }
        let expected_gates: Vec<_> = LEGACY_GATES.iter().chain(STAGE_GATES.iter()).collect();
        if self.gates.len() != expected_gates.len()
            || self
                .gates
                .iter()
                .zip(expected_gates)
                .any(|(gate, predicate)| gate.predicate != *predicate || gate.status != UNPERFORMED)
        {
            return Err(refuse("Staged gates differ from the ten unperformed predicates."));
        }
        if self.circuits.len() > MAX_STAGED_CIRCUITS {
            return Err(error("limit", "Program exceeds staged circuit limit."));
        }
        // Circuits are grouped by action in one contiguous run each.
        let mut runs: Vec<&[StagedCircuit]> = Vec::new();
        let mut start = 0;
        for i in 1..=self.circuits.len() {
            if i == self.circuits.len() || self.circuits[i].action != self.circuits[start].action {
                runs.push(&self.circuits[start..i]);
                start = i;
            }
        }
        let mut names = BTreeSet::new();
        if runs.iter().any(|run| !names.insert(run[0].action.as_str())) {
            return Err(refuse("Each action's circuits must be one contiguous run."));
        }
        if self.resources.logical.len() != runs.len() {
            return Err(refuse("Logical metadata differs from the action circuits."));
        }
        let registry = self.resources.liability_registry_entries as usize;
        for (run, logical) in runs.iter().zip(&self.resources.logical) {
            check_action(run, logical, registry)?;
        }
        self.check_totals()
    }

    fn check_totals(&self) -> Result<(), Error> {
        let overflow = || error("limit", "Staged resource total overflow.");
        let (mut instructions, mut zkir_bytes, mut public_values, mut replay) = (0u32, 0u32, 0u32, 0);
        for c in &self.circuits {
            instructions = instructions.checked_add(c.instruction_count).ok_or_else(overflow)?;
            zkir_bytes = u32::try_from(c.zkir.len())
                .ok()
                .and_then(|n| zkir_bytes.checked_add(n))
                .ok_or_else(overflow)?;
            public_values = u32::try_from(c.public_inputs.len())
                .ok()
                .and_then(|n| public_values.checked_add(n))
                .ok_or_else(overflow)?;
            replay = replay.max(c.replay_steps);
        }
        let r = &self.resources;
        if (r.instructions, r.zkir_bytes, r.public_values, r.max_replay_steps)
            != (instructions, zkir_bytes, public_values, replay)
        {
            return Err(refuse("Staged resource totals differ from the circuits."));
        }
        if instructions > self.target.max_instructions || zkir_bytes > self.target.max_artifact_bytes {
            return Err(error(
                "limit",
                "Program exceeds target instruction or ZKIR byte limit.",
            ));
        }
        // artifact_bytes is the fixed point of this exact pretty serialization.
        let bytes =
            serde_json::to_vec_pretty(self).map_err(|e| error("serialization", e.to_string()))?;
        let count = u32::try_from(bytes.len())
            .map_err(|_| error("limit", "Artifact envelope exceeds u32 byte count."))?;
        if count > self.target.max_artifact_bytes {
            return Err(error("limit", "Artifact envelope exceeds target byte limit."));
        }
        if count != r.artifact_bytes {
            return Err(refuse("Staged artifact byte count is not its envelope size."));
        }
        Ok(())
    }
}

fn is_pending(role: &str) -> bool {
    PENDING_ROLES.contains(&role)
}

/// One action: segments exactly 0..=N, sites, capture continuity, pending
/// frames and logical work.
fn check_action(run: &[StagedCircuit], logical: &ActionLogical, registry: usize) -> Result<(), Error> {
    let n = run.len() - 1;
    let action = &run[0].action;
    if n > MAX_AWAITS_PER_ACTION
        || logical.action != *action
        || logical.segments.len() != run.len()
    {
        return Err(refuse(format!("Segments of {action:?} are not exactly 0..=N.")));
    }
    // captures(N) is the entry layout of the last segment: the width-L layout.
    let layout: &[CaptureSlot] = if n == 0 { &[] } else { &run[n].capture_in };
    let width = layout.len();
    if width > MAX_CAPTURE_SLOTS {
        return Err(refuse("Capture layout exceeds the slot bound."));
    }
    let mut committed = UInt(0);
    for (j, c) in run.iter().enumerate() {
        let segment = u16::try_from(j).map_err(|_| refuse("Segment index overflow."))?;
        let entry = segment;
        let exit = if j == n { 0 } else { segment + 1 };
        if c.segment != segment || c.entry_site != entry || c.exit_site != exit {
            return Err(refuse(format!(
                "Segment {j} of {action:?} has inconsistent sites."
            )));
        }
        signed_relation::validate_shape(UNSIGNED_MODE, &c.public_inputs, &[], None)?;
        if j > 0 && c.public_inputs.iter().any(|b| b.role == "input") {
            return Err(refuse("Advance segments have no input positions."));
        }
        if c.liability_frame.len() != registry {
            return Err(refuse("Every segment carries the whole liability registry."));
        }
        if c.balance_frame.iter().map(|b| (&b.asset, &b.account)).ne(run[0]
            .balance_frame
            .iter()
            .map(|b| (&b.asset, &b.account)))
        {
            return Err(refuse("Every segment carries the whole-action balance frame."));
        }
        let s = &logical.segments[j];
        if s.segment != segment || s.committed_before != committed {
            return Err(refuse(format!(
                "Logical work of segment {j} of {action:?} is inconsistent."
            )));
        }
        committed = committed
            .0
            .checked_add(s.segment_logical.0)
            .map(UInt)
            .ok_or_else(|| refuse("Logical plan total overflow."))?;
        if n == 0 {
            if c.capture_width != 0
                || !c.capture_in.is_empty()
                || !c.capture_out.is_empty()
                || !c.event.is_empty()
                || c.pending_positions.is_some()
                || c.public_inputs.iter().any(|b| is_pending(&b.role))
            {
                return Err(refuse(format!(
                    "Zero-await action {action:?} has no pending frame."
                )));
            }
            continue;
        }
        if usize::from(c.capture_width) != width
            || c.capture_in.len() > width
            || c.capture_out.len() > width
            || c.capture_in[..] != layout[..c.capture_in.len()]
            || c.capture_out[..] != layout[..c.capture_out.len()]
            || (j == 0 && !c.capture_in.is_empty())
            || (j == n && !c.capture_out.is_empty())
            || (j > 0 && c.capture_in != run[j - 1].capture_out)
            || (j > 0 && j < n && c.capture_out.len() < c.capture_in.len())
        {
            return Err(refuse(format!(
                "Capture layout of segment {j} of {action:?} is not the width-L prefix."
            )));
        }
        if (j == 0 && !c.event.is_empty()) || c.event.len() > MAX_EVENT_BINDERS {
            return Err(refuse("Event layout differs from the entry site."));
        }
        let p = c
            .pending_positions
            .as_ref()
            .ok_or_else(|| refuse(format!("Segment {j} of {action:?} lacks pending positions.")))?;
        check_pending(c, p, layout)?;
    }
    if logical.plan_total != committed {
        return Err(refuse(format!("Plan total of {action:?} differs from its segments.")));
    }
    Ok(())
}

/// Roles, names, types and constants per the frame rules; positions are in
/// range, distinct, disjoint from every other declared position, contiguous at
/// their two insertion points, and no other binding carries a pending role.
fn check_pending(c: &StagedCircuit, p: &PendingPositions, layout: &[CaptureSlot]) -> Result<(), Error> {
    let width = layout.len();
    if p.pre_captures.len() != width
        || p.post_captures.len() != width
        || p.events.len() != c.event.len()
    {
        return Err(refuse("Pending positions differ from the capture and event widths."));
    }
    let padding = |i: usize, len: usize| (i >= len).then(|| "0".to_string());
    let entry = c.entry_site.to_string();
    let exit = c.exit_site.to_string();
    let mut pre: Vec<(u32, &str, &str, Type, Option<String>)> = Vec::new();
    pre.push((p.pre_site, PRE_SITE_ROLE, &c.action, Type::UInt, Some(entry)));
    for (i, slot) in layout.iter().enumerate() {
        let constant = padding(i, c.capture_in.len());
        pre.push((p.pre_captures[i], PRE_CAPTURE_ROLE, &slot.name, slot.ty, constant));
    }
    for (position, binder) in p.events.iter().zip(&c.event) {
        pre.push((*position, EVENT_ROLE, &binder.name, binder.ty, None));
    }
    let mut post: Vec<(u32, &str, &str, Type, Option<String>)> = Vec::new();
    post.push((p.post_site, POST_SITE_ROLE, &c.action, Type::UInt, Some(exit)));
    for (i, slot) in layout.iter().enumerate() {
        let constant = padding(i, c.capture_out.len());
        post.push((p.post_captures[i], POST_CAPTURE_ROLE, &slot.name, slot.ty, constant));
    }
    let mut seen = BTreeSet::new();
    for (position, role, name, ty, constant) in pre.iter().chain(&post) {
        let b = c
            .public_inputs
            .get(*position as usize)
            .ok_or_else(|| refuse("Pending position is out of range."))?;
        if !seen.insert(*position) {
            return Err(refuse("Pending positions are not distinct."));
        }
        if b.role != *role
            || b.name != *name
            || b.ty != *ty
            || b.constant != *constant
            || b.asset.is_some()
        {
            return Err(refuse("Pending binding differs from the frame rules."));
        }
    }
    let mut others = vec![0, 1];
    for e in &c.effect_frame {
        others.extend(&e.public_positions);
    }
    for b in &c.balance_frame {
        others.extend([b.pre_position, b.post_position]);
    }
    for l in &c.liability_frame {
        others.extend([
            l.pre_presence_position,
            l.pre_amount_position,
            l.post_presence_position,
            l.post_amount_position,
        ]);
    }
    if others.iter().any(|q| seen.contains(q)) {
        return Err(refuse("Pending positions overlap other declared positions."));
    }
    if c
        .public_inputs
        .iter()
        .enumerate()
        .any(|(i, b)| is_pending(&b.role) && !seen.contains(&(i as u32)))
    {
        return Err(refuse("A pending role appears outside the declared positions."));
    }
    // The pre block follows the last pre-liability position; the post block
    // follows the last post-liability position and ends the unsigned ABI.
    let after = |roles: &[&str]| {
        c.public_inputs
            .iter()
            .rposition(|b| roles.contains(&b.role.as_str()))
            .map_or(0, |i| i + 1)
    };
    let pre_start = after(&[
        "context",
        "pre_state",
        "input",
        "pre_balance",
        "pre_liability_presence",
        "pre_liability_amount",
    ]);
    let post_start = after(&[
        "post_state",
        "post_balance",
        "post_liability_presence",
        "post_liability_amount",
    ]);
    let contiguous = |block: &[(u32, &str, &str, Type, Option<String>)], start: usize| {
        block
            .iter()
            .enumerate()
            .all(|(k, entry)| entry.0 as usize == start + k)
    };
    if !contiguous(&pre, pre_start)
        || !contiguous(&post, post_start)
        || post_start + post.len() != c.public_inputs.len()
    {
        return Err(refuse("Pending blocks are not at their frame insertion points."));
    }
    Ok(())
}
