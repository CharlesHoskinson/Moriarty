//! Explicit Core scalar value projection through the existing B host owner.
//!
//! SOURCE_ONLY / UNRUN. The caller supplies a real parsed Core program, named
//! scalar inputs and every operational bound/profile value. This module does
//! not parse, evaluate Core, invoke Machine A, or implement any B transition.
//! It leaves the default Core, Core/2 and financial consumers unchanged.
//!
//! The supported source shape is exactly `prepare_scalar_action`'s current
//! stateless Core/1 scalar projection. A final emit's value becomes the return
//! value; its ordered Core effect is not executed here. Its label, statement
//! position and role remain unchanged in the returned preparation report.
//!
//! Host preprocessing and error policy are supplied by the separate
//! security-owned module. This in-process diagnostic API is not a remote or
//! untrusted-input sandbox. B work is the actual owner's helper schedule;
//! neither A work nor the Core evaluator's work is substituted for it.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::sherlock::{Expr, ExprRef, Program as ScalarProgram, Value as ScalarValue};
use crate::sherlock_source::{prepare_scalar_action, ScalarActionReport};

use super::core_action_consumer_security::{
    check_combined_generated_ast_nodes, check_generated, preflight_source, EventForwarding,
};
use super::shared::owner_host::{host_owner_engine, observe_bound_image, HostObservation};
use super::shared::{Admitted, FinanceInput, HostViolation, MachineImage, OwnerEngine, StepEventC};
use super::source_input_bridge;

pub use super::core_action_consumer_security::{
    preflight_action_inputs, preflight_source_global, validate_config, CompletedRun,
    ConsumerConfig, ConsumerError, ConsumerProfile, HostGuardError, HostMeasure, SourceHostBounds,
};

/// The unchanged source preparation report and the actual completed B run.
///
/// The report's function-body paths describe source projection occurrences.
/// B event expression IDs and occurrence locators come from the owner image;
/// they are not relabelled as Core expression IDs by this consumer.
pub struct CoreActionRun {
    pub preparation: ScalarActionReport,
    pub completed: CompletedRun,
}

impl CoreActionRun {
    /// Read the actual owner-bound admission/image pair without copying it.
    pub fn observe(&self) -> Result<HostObservation<'_>, ConsumerError> {
        self.completed.observe()
    }

    /// Project actual owner counters and storage lengths. Construction and
    /// serialization costs are explicitly unmeasured, outside run counters.
    /// The existing observation refusal is propagated without a new error.
    pub fn resource_report(&self)
        -> Result<super::resource_report::ResourceReport, ConsumerError>
    {
        let observation = self.observe()?;
        Ok(super::resource_report::ResourceReport::from_owner(
            &observation, &self.completed.summary))
    }
}

/// Actual owner-bound admitted pair, prepared without executing a B step.
/// The same immutable source/guard/projection/admission sequence is shared with run.
pub struct CoreActionPrepared {
    pub preparation: ScalarActionReport,
    pub admitted: Admitted,
    pub image: MachineImage,
}

impl CoreActionPrepared {
    /// Read actual admission/image fields; no fabricated completed RunSummary.
    pub fn observe(&self) -> Result<HostObservation<'_>, HostViolation> {
        observe_bound_image(&self.admitted, &self.image)
    }
}

/// Prepare and admit the existing scalar projection without running it.
/// Existing preflight, generated-AST checks and admission order are retained.
pub fn prepare_scalar_action_host(
    source: &crate::Program,
    action_index: usize,
    inputs: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
) -> Result<CoreActionPrepared, ConsumerError> {
    let preflight = preflight_source(source, action_index, inputs, config)?;
    let mapped = prepare_scalar_action(source, action_index)
        .map_err(ConsumerError::SourceProjection)?;
    let (body_nodes, _) = check_generated(&mapped, &preflight, &config.host)?;

    let arguments: Result<Vec<ExprRef>, ConsumerError> = mapped.report.inputs.iter().map(|site| {
        // Preflight and check_generated already bind these same immutable names
        // to the declared scalar inputs. Keep the typed missing-input error on
        // this lookup too; do not fabricate a literal if the binding is absent.
        let value = inputs.get(&site.name).ok_or_else(|| {
            ConsumerError::HostGuard(HostGuardError::MissingInput { name: site.name.clone() })
        })?;
        let value = match value {
            crate::Value::UInt(value) => ScalarValue::UInt(value.0),
            crate::Value::Bool(value) => ScalarValue::Bool(*value),
        };
        Ok(Arc::new(Expr::Literal(value)))
    }).collect();
    let entry = Arc::new(Expr::Call(mapped.function.name.clone(), arguments?));
    check_combined_generated_ast_nodes(body_nodes, &entry, &config.host)?;

    let preparation = mapped.report;
    let canonical = ScalarProgram { functions: vec![mapped.function], entry };
    let owner_source = source_input_bridge::from_program(&canonical);
    let engine = host_owner_engine();
    let admitted = engine.admit(&owner_source, &config.bounds, &config.layout)
        .map_err(ConsumerError::Admit)?;
    let image = engine.admit_run(&admitted, &config.fuel, &config.stack, &FinanceInput::Absent)
        .map_err(ConsumerError::AdmitRun)?;

    Ok(CoreActionPrepared { preparation, admitted, image })
}

/// Project and run one currently supported scalar action through the fixed B owner.
///
/// `inputs` is the same map type as `Scenario.inputs`. Only that explicit map
/// is accepted: state, balances, liabilities and source effects are not a
/// financial input to this pure projection. Names are resolved in the real
/// adapter report's declaration order, independently of BTreeMap key order.
///
/// Preparation/guard failures and the owner's Tooling/HostViolation values
/// retain the staged security-owned error contract. If `run` fails, events
/// already forwarded to `sink` form a partial prefix and no completed result
/// is returned. The sink receives each real full StepEventC exactly once;
/// this consumer stores no event vector. Any sink retention is caller-owned.
/// Execute the existing fixed owner on the pair produced by shared preparation.
pub fn run_scalar_action(
    source: &crate::Program,
    action_index: usize,
    inputs: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<CoreActionRun, ConsumerError> {
    let CoreActionPrepared { preparation, admitted, mut image } =
        prepare_scalar_action_host(source, action_index, inputs, config)?;
    let engine = host_owner_engine();
    let mut events = EventForwarding::default();
    let result = {
        let mut forwarding_sink = |event: &StepEventC| {
            events.note();
            sink(event);
        };
        engine.run(&admitted, &mut image, &mut forwarding_sink)
    };
    let summary = result.map_err(|violation| ConsumerError::Run {
        violation,
        events_forwarded: events.forwarded,
    })?;
    Ok(CoreActionRun {
        preparation,
        completed: CompletedRun {
            admitted,
            image,
            summary,
            events_forwarded: events.forwarded,
        },
    })
}
