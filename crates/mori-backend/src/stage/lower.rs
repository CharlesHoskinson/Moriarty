//! Continuation compilation through the ordinary scalar/financial lowerer.
use super::artifact::{
    ActionLogical, LOGICAL_ENFORCEMENT, MAX_STAGED_CIRCUITS, STAGE_GATES,
    STAGED_ABI_VERSION, STAGED_ARTIFACT_FORMAT, STAGED_RELATION_VERSION,
    SegmentLogical, StagedArtifact, StagedCircuit, StagedResources,
};
use super::frame;
use crate::{Error, Gate, MAX_LIABILITY_REGISTRY_ENTRIES, Target, error, liabilities,
    lower_action_segment, proving};
use mori_core::{LOGICAL_COST_PROFILE_2, stage::{STAGE_LABELS, StagedProgram}};
use std::path::Path;

/// Compile every action into its successful continuation segments.
///
/// `out` is the caller's publication destination. Compilation remains pure and
/// the backend performs no filesystem effect; the CLI reserves that directory
/// and publishes the manifest last through its existing publication helpers
/// after this function returns successfully. Fresh output must pass the full
/// `/4` decoder predicate before it is returned. That predicate establishes
/// shape validity only: it does not recompute the program, plan or context
/// digests, so neither a decoded nor a fresh artifact is provenance-bound.
pub fn compile_staged(src: &str, target: &Target, _out: &Path) -> Result<StagedArtifact, Error> {
    if target.signed_policy.is_some() {
        return Err(error(
            "unsupported",
            "Staged moriarty/2 relations are unsigned; a signed Target is refused.",
        ));
    }
    let sp = mori_core::stage::load(src).map_err(|e| error("program", e.to_string()))?;
    target.validate()?;
    let program = sp.program();
    if program.assets.iter().any(|asset| asset.identity.network != target.network) {
        return Err(error("target", "Asset network differs from the target domain; cross-domain assets are unsupported."));
    }
    let registry = liabilities::registry(program)?;
    let layouts = frame::layouts(&sp)?;
    if layouts.len() > MAX_STAGED_CIRCUITS {
        return Err(error("limit", "Program exceeds staged circuit limit."));
    }
    let mut resources = StagedResources {
        instructions: 0,
        zkir_bytes: 0,
        artifact_bytes: 0,
        public_values: 0,
        max_replay_steps: 0,
        liability_registry_entries: registry.len() as u32,
        max_liability_registry_entries: MAX_LIABILITY_REGISTRY_ENTRIES as u32,
        logical_profile: LOGICAL_COST_PROFILE_2.into(),
        logical_enforcement: LOGICAL_ENFORCEMENT.into(),
        logical: logical_resources(&sp),
    };
    let mut circuits = Vec::with_capacity(layouts.len());
    for layout in &layouts {
        let action = &program.actions[layout.action];
        let (circuit, pending_positions) = lower_action_segment(
            program, action, target, &registry, None, Some((&sp, layout)),
        )?;
        resources.instructions = resources.instructions.checked_add(circuit.instruction_count)
            .ok_or_else(|| error("limit", "Instruction count overflow."))?;
        resources.zkir_bytes = resources.zkir_bytes.checked_add(
            u32::try_from(circuit.zkir.len()).map_err(|_| error("limit", "Artifact too large."))?
        ).ok_or_else(|| error("limit", "Artifact size overflow."))?;
        resources.public_values = resources.public_values.checked_add(
            u32::try_from(circuit.public_inputs.len()).map_err(|_| error("limit", "Public ABI size overflow."))?
        ).ok_or_else(|| error("limit", "Public ABI size overflow."))?;
        resources.max_replay_steps = resources.max_replay_steps.max(circuit.replay_steps);
        if resources.instructions > target.max_instructions
            || resources.zkir_bytes > target.max_artifact_bytes {
            return Err(error("limit", "Program exceeds target instruction or ZKIR byte limit."));
        }
        circuits.push(StagedCircuit {
            action: circuit.action,
            segment: layout.segment,
            entry_site: layout.entry_site,
            exit_site: layout.exit_site,
            zkir: circuit.zkir,
            context_hash: circuit.context_hash,
            capture_width: layout.slots.len() as u16,
            capture_in: layout.slots[..layout.len_in as usize].to_vec(),
            capture_out: layout.slots[..layout.len_out as usize].to_vec(),
            event: layout.event.clone(),
            public_inputs: circuit.public_inputs,
            pending_positions,
            instruction_count: circuit.instruction_count,
            effect_frame: circuit.effect_frame,
            balance_frame: circuit.balance_frame,
            liability_frame: circuit.liability_frame,
            replay_steps: circuit.replay_steps,
        });
    }
    let mut artifact = StagedArtifact {
        format: STAGED_ARTIFACT_FORMAT.into(),
        abi: STAGED_ABI_VERSION.into(),
        relation: STAGED_RELATION_VERSION.into(),
        authorization_mode: target.authorization_mode().into(),
        program_hash: proving::hex(&sp.program_hash()),
        plan_hash: proving::hex(&sp.plan_hash()),
        semantics: program.semantics.clone(),
        qualification: STAGE_LABELS[0].1.into(),
        authentication: STAGE_LABELS[1].1.into(),
        consumption: STAGE_LABELS[2].1.into(),
        target: target.clone(),
        circuits,
        resources,
        gates: [
            "native-proof-generation",
            "circuit-cost-and-k-measurement",
            "fraud-proof-cost-and-k-measurement",
            "authenticated-ledger-frame-and-phase-correspondence",
            "signed-intent-and-predecessor-history-verification",
            "live-ledger-deployment",
        ].into_iter().chain(STAGE_GATES).map(|p| Gate {
            predicate: p.into(), status: "unperformed".into(),
        }).collect(),
    };
    // Identical fixed-point procedure and pretty serialization to /3.
    for _ in 0..12 {
        let bytes = serde_json::to_vec_pretty(&artifact)
            .map_err(|e| error("serialization", e.to_string()))?;
        let count = u32::try_from(bytes.len())
            .map_err(|_| error("limit", "Artifact envelope exceeds u32 byte count."))?;
        if count > target.max_artifact_bytes {
            return Err(error("limit", "Artifact envelope exceeds target byte limit."));
        }
        if count == artifact.resources.artifact_bytes {
            // Nothing the decoder would refuse is ever returned for publication.
            artifact.validate()?;
            return Ok(artifact);
        }
        artifact.resources.artifact_bytes = count;
    }
    Err(error("serialization", "Artifact size count did not stabilize."))
}

fn logical_resources(sp: &StagedProgram) -> Vec<ActionLogical> {
    sp.plan().actions.iter().map(|action| ActionLogical {
        action: action.action.clone(),
        plan_total: action.plan_total,
        segments: action.segment_cost.iter().zip(&action.committed_before).enumerate()
            .map(|(segment, (&segment_logical, &committed_before))| SegmentLogical {
                segment: segment as u16,
                segment_logical,
                committed_before,
            }).collect(),
    }).collect()
}
