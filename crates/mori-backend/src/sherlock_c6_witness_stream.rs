//! Public owner-run carriers for C6 leaf producers. This is a host witness
//! stream, not a proof or a claim that every fired rule has a circuit fragment.
use std::{io::Write, path::Path};

use midnight_transient_crypto::curve::Fr;
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits,
};
use mori_core::sherlock::helper_machine::complete_observation::{
    ConstantSnapshot, DataSnapshot, HostSnapshot, capture,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{
    OutcomeC, OwnerEngine, RefC, RuleIdC, StepResult,
};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use serde_json::json;

use crate::sherlock_c6_child::{
    DigestSections, StateOutcome, hash_tree_offcircuit, rule_lane, state_digest_offcircuit,
};
use crate::sherlock_child::field_hex;
use crate::sherlock_machine::c6_push_step::C6_PADDED_CARRIER_VERSION;
use crate::sherlock_machine::c6_step_harness::{C6StepWitness, step_witness_from_host_pair};
use crate::{Error, error};

pub const C6_WITNESS_STREAM_FORMAT: &str = "moriarty-sherlock-c6-witness-stream/1";

/// One actual owner event, including the publication step that sets Halted.
#[derive(Clone, Debug)]
pub struct C6WitnessStep {
    pub witness: C6StepWitness,
    pub rule: RuleIdC,
    pub h_pre: u64,
    pub prog_d: Fr,
    pub init_d: Fr,
    pub pre_d: Fr,
    pub post_d: Fr,
    pub pre_lanes_d: Fr,
    pub post_lanes_d: Fr,
    pub pre_outcome: StateOutcome,
    pub post_outcome: StateOutcome,
}

fn prefix(w: &C6StepWitness) -> Vec<u128> {
    [
        u128::from(w.enable),
        C6_HOST_SCHEMA,
        C6_PADDED_CARRIER_VERSION,
    ]
    .into_iter()
    .chain(w.host_prefix.meta.iter().copied())
    .chain(w.host_prefix.prog.iter().copied())
    .chain(w.host_prefix.ctx)
    .collect()
}

impl C6WitnessStep {
    /// Adapter order: one shared prefix, then pre IMAGE/DATA/HELPER, then post
    /// IMAGE/DATA/HELPER. The original harness statement repeats META/PROG/CTX
    /// on the post side; it remains available through `witness.to_statement_frs()`.
    pub fn digest_statement(&self) -> (Vec<u128>, DigestSections) {
        let w = &self.witness;
        let mut lanes = prefix(w);
        let sections = DigestSections {
            prog_lanes: lanes.len(),
            pre_lanes: w.pre_image.len() + w.pre_data.len() + w.pre_helper.len(),
            post_lanes: w.post_image.len() + w.post_data.len() + w.post_helper.len(),
        };
        lanes.extend(
            w.pre_image
                .iter()
                .chain(&w.pre_data)
                .chain(&w.pre_helper)
                .chain(&w.post_image)
                .chain(&w.post_data)
                .chain(&w.post_helper),
        );
        (lanes, sections)
    }

    /// Emit one JSON line. All integer lanes (including u128 values and h_pre)
    /// are decimal strings; digests and rule_lane are 32-byte little-endian hex.
    /// Only the public source's owner carriers are exported: no source paths,
    /// environment, keys, credentials or prover-private randomness is read.
    pub fn write_json_line(&self, writer: &mut impl Write) -> Result<(), Error> {
        let w = &self.witness;
        let decimal =
            |lanes: &[u128]| -> Vec<String> { lanes.iter().map(u128::to_string).collect() };
        let outcome =
            |o: StateOutcome| json!({"tag": o.tag.to_string(), "value": o.value.to_string()});
        let rule = format!("{:?}", self.rule);
        let record = json!({
            "format": C6_WITNESS_STREAM_FORMAT,
            "rule": rule,
            "rule_lane": field_hex(&rule_lane(&rule)),
            "h_pre": self.h_pre.to_string(),
            "prog_d": field_hex(&self.prog_d),
            "init_d": field_hex(&self.init_d),
            "pre_d": field_hex(&self.pre_d),
            "post_d": field_hex(&self.post_d),
            "pre_lanes_d": field_hex(&self.pre_lanes_d),
            "post_lanes_d": field_hex(&self.post_lanes_d),
            "pre_outcome": outcome(self.pre_outcome),
            "post_outcome": outcome(self.post_outcome),
            "witness": {
                "enable": w.enable,
                "host_schema": C6_HOST_SCHEMA.to_string(),
                "carrier_version": C6_PADDED_CARRIER_VERSION.to_string(),
                "meta": decimal(&w.host_prefix.meta),
                "prog": decimal(&w.host_prefix.prog),
                "ctx": decimal(&w.host_prefix.ctx),
                "pre": {"image": decimal(&w.pre_image), "data": decimal(&w.pre_data), "helper": decimal(&w.pre_helper)},
                "post": {"image": decimal(&w.post_image), "data": decimal(&w.post_data), "helper": decimal(&w.post_helper)},
            },
        });
        serde_json::to_writer(&mut *writer, &record).map_err(failure)?;
        writer.write_all(b"\n").map_err(failure)
    }
}

/// Final opening for the recursion decider; returned only after a UInt halt.
#[derive(Clone, Debug)]
pub struct C6WitnessRun {
    pub prog_d: Fr,
    pub init_d: Fr,
    pub fin_d: Fr,
    pub lanes_d: Fr,
    pub n_steps: u64,
    pub result: u128,
}

fn failure(e: impl std::fmt::Display) -> Error {
    error("sherlock-c6-witness-stream", e.to_string())
}

fn outcome(host: &HostSnapshot) -> Result<StateOutcome, Error> {
    let result = match host.image.outcome {
        OutcomeC::Running => return Ok(StateOutcome::RUNNING),
        OutcomeC::Halted {
            value: RefC::Const(i),
        } => match host.admission.view.constants.get(i) {
            Some(ConstantSnapshot::U128 {
                marker_free: true,
                n,
            }) => *n,
            _ => return Err(failure("Halted result is not a UInt constant.")),
        },
        OutcomeC::Halted {
            value: RefC::Data(i),
        } => match host.image.data.get(i) {
            Some(DataSnapshot::U128 { n }) => *n,
            _ => return Err(failure("Halted result is not a UInt data value.")),
        },
        other => return Err(failure(format!("Run did not halt with a UInt: {other:?}."))),
    };
    Ok(StateOutcome::halted(result))
}

/// Run committed public draft source under the caller's host config. Drive the
/// same owner as `mori simulate`, capture both sides of every Event, and encode
/// them with the C6 harness. Terminal is not an extra step. Codec budgets and
/// limits match the dispatch tests. No fragment filtering or proving occurs.
///
/// The sink receives one step at a time; no trace is retained by this function.
/// A later owner/encoding/sink failure can leave an emitted prefix. Consumers
/// must require successful completion and the final halted opening.
pub fn for_each_witness(
    source: &str,
    config_path: &Path,
    mut sink: impl FnMut(C6WitnessStep) -> Result<(), Error>,
) -> Result<C6WitnessRun, Error> {
    let config_source = std::fs::read_to_string(config_path).map_err(failure)?;
    let genesis = draft_genesis(source, &config_source).map_err(|e| {
        failure(serde_json::to_string(&e).unwrap_or_else(|_| "genesis failed".into()))
    })?;
    let cfg = C6HostConfig::from_retained_layout(
        &genesis.layout,
        C6CodecBudgets {
            type_expr_nodes: 128,
            template_bytes: 4096,
        },
    );
    let limits = C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    };
    let engine = host_owner_engine();
    let mut image = genesis.image;
    let mut pre =
        capture(&genesis.admitted, &image).map_err(|e| failure(format!("capture: {e:?}")))?;
    let mut previous: Option<C6WitnessRun> = None;
    let mut h_pre = 0u64;
    loop {
        let step = engine
            .step(&genesis.admitted, &mut image)
            .map_err(|e| failure(format!("owner step {h_pre}: {e:?}")))?;
        let StepResult::Event { event } = step else {
            let terminal = outcome(&pre)?;
            if terminal.tag != 1 {
                return Err(failure("Owner terminated without a halted UInt."));
            }
            return previous.ok_or_else(|| failure("Owner emitted no steps."));
        };
        let post =
            capture(&genesis.admitted, &image).map_err(|e| failure(format!("capture: {e:?}")))?;
        let witness =
            step_witness_from_host_pair(true, &pre, &post, &cfg, &limits).map_err(failure)?;
        let pre_outcome = outcome(&pre)?;
        let post_outcome = outcome(&post)?;
        let prefix: Vec<Fr> = prefix(&witness).into_iter().map(Fr::from).collect();
        let carrier_digest = |image: &[u128], data: &[u128], helper: &[u128]| {
            let mut carrier = prefix.clone();
            carrier.extend(
                image
                    .iter()
                    .chain(data)
                    .chain(helper)
                    .copied()
                    .map(Fr::from),
            );
            hash_tree_offcircuit(&carrier)
        };
        let (prog_d, init_d, pre_d, pre_lanes_d) = if let Some(prev) = &previous {
            (prev.prog_d, prev.init_d, prev.fin_d, prev.lanes_d)
        } else {
            let lanes_d =
                carrier_digest(&witness.pre_image, &witness.pre_data, &witness.pre_helper);
            let init_d = state_digest_offcircuit(pre_outcome, lanes_d);
            (hash_tree_offcircuit(&prefix), init_d, init_d, lanes_d)
        };
        let post_lanes_d = carrier_digest(
            &witness.post_image,
            &witness.post_data,
            &witness.post_helper,
        );
        let post_d = state_digest_offcircuit(post_outcome, post_lanes_d);
        sink(C6WitnessStep {
            witness,
            rule: event.rule_id,
            h_pre,
            prog_d,
            init_d,
            pre_d,
            post_d,
            pre_lanes_d,
            post_lanes_d,
            pre_outcome,
            post_outcome,
        })?;
        h_pre = h_pre
            .checked_add(1)
            .ok_or_else(|| failure("Step height overflow."))?;
        previous = Some(C6WitnessRun {
            prog_d,
            init_d,
            fin_d: post_d,
            lanes_d: post_lanes_d,
            n_steps: h_pre,
            result: post_outcome.value,
        });
        pre = post;
    }
}

/// Write the public JSON-lines stream to a caller-owned sink. See
/// `C6WitnessStep::write_json_line` for encoding and `for_each_witness` for
/// partial-stream failure semantics. The caller owns flushing the writer.
pub fn write_witness_stream(
    source: &str,
    config_path: &Path,
    writer: &mut impl Write,
) -> Result<C6WitnessRun, Error> {
    for_each_witness(source, config_path, |step| step.write_json_line(writer))
}
