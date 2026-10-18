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

