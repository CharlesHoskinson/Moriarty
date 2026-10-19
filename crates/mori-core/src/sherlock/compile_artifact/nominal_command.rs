//! Explicit nominal command wire inputs and the existing admitted owner consumer.
//! No new bounds, guards, body typechecker, evaluator or native exporter.
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde_json::{json, Value};
use super::{CompileFailure, config::{Bounds, Finance, Layout, Profile, RunInput, U128Decimal, UsizeDecimal}, wire::{self, OwnerWire}};
use super::ext_compile::{compile_nominal_host, ExtCompileConfig, ExtCompileFailure, NominalCompileFailure};
use crate::sherlock_nominal_source::{DraftParseBudget, NOMINAL_LANGUAGE};
use crate::sherlock::helper_machine::shared::{ExtCapacitiesC, owner_ext_source_program::ExtDomain};

pub const CONFIG_FORMAT: &str = "sherlock-nominal-host-config/1";
pub const ARTIFACT_FORMAT: &str = "sherlock-nominal-host-artifact/1";
macro_rules! tag {
    ($ty:ident, $text:literal) => {
        #[derive(Clone, Copy)] pub struct $ty;
        impl Serialize for $ty { fn serialize<S:Serializer>(&self,s:S)->Result<S::Ok,S::Error>{s.serialize_str($text)} }
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D:Deserializer<'de>>(d:D)->Result<Self,D::Error>{
                let text=String::deserialize(d)?;
                if text==$text {Ok(Self)} else {Err(serde::de::Error::custom(concat!("expected ",$text)))}
            }
        }
    };
}
tag!(NominalFormat, "sherlock-nominal-host-config/1");
tag!(NominalGrammar, "GenericNominalExtensionReserved");
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NominalPreprocessing {
    pub max_source_bytes: UsizeDecimal,
    pub max_tokens: UsizeDecimal,
    pub max_functions: UsizeDecimal,
    pub max_parameters_per_function: UsizeDecimal,
    pub max_total_parameters: UsizeDecimal,
    pub max_ast_nodes: UsizeDecimal,
    pub max_parse_depth: UsizeDecimal,
    pub max_structural_ast_depth: UsizeDecimal,
}
impl NominalPreprocessing {
    fn parser_budget(self)->DraftParseBudget {
        DraftParseBudget { max_source_bytes:self.max_source_bytes.0,max_tokens:self.max_tokens.0,
            max_functions:self.max_functions.0,max_parameters_per_function:self.max_parameters_per_function.0,
            max_total_parameters:self.max_total_parameters.0,max_ast_occurrences:self.max_ast_nodes.0,
            max_parser_recursion_depth:self.max_parse_depth.0,max_structural_ast_depth:self.max_structural_ast_depth.0 }
    }
}
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NominalExtensionCapacities {
    pub plan_case_slots: UsizeDecimal,
    pub field_path_slots: UsizeDecimal,
    pub pattern_child_slots: UsizeDecimal,
    pub input_rows: UsizeDecimal,
    pub asset_rows: UsizeDecimal,
    pub state_rows: UsizeDecimal,
    pub account_rows: UsizeDecimal,
    pub liability_rows: UsizeDecimal,
    pub await_site_rows: UsizeDecimal,
    pub await_binder_slots: UsizeDecimal,
    pub balance_cells: UsizeDecimal,
    pub journal_rows: UsizeDecimal,
    pub pending_op_rows: UsizeDecimal,
    pub observations_per_invocation: UsizeDecimal,
    pub max_lifetime_work: U128Decimal,
    pub resume_rows: UsizeDecimal,
}
impl NominalExtensionCapacities {
    fn owner(self)->ExtCapacitiesC {
        ExtCapacitiesC {
            plan_case_slots:self.plan_case_slots.0,
            field_path_slots:self.field_path_slots.0,
            pattern_child_slots:self.pattern_child_slots.0,
            input_rows:self.input_rows.0,
            asset_rows:self.asset_rows.0,
            state_rows:self.state_rows.0,
            account_rows:self.account_rows.0,
            liability_rows:self.liability_rows.0,
            await_site_rows:self.await_site_rows.0,
            await_binder_slots:self.await_binder_slots.0,
            balance_cells:self.balance_cells.0,
            journal_rows:self.journal_rows.0,
            pending_op_rows:self.pending_op_rows.0,
            observations_per_invocation:self.observations_per_invocation.0,
            max_lifetime_work:self.max_lifetime_work.0,
            resume_rows:self.resume_rows.0,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NominalConfig {
    pub format:NominalFormat,pub profile:Profile,pub grammar:NominalGrammar,pub finance:Finance,
    pub bounds:Bounds,pub layout:Layout,pub extension_capacities:NominalExtensionCapacities,
    pub fuel:RunInput,pub stack:RunInput,pub preprocessing:NominalPreprocessing,
}
impl NominalConfig {
    fn owner(&self)->ExtCompileConfig {
        let bounds=self.bounds.owner();
        let mut layout=self.layout.owner(bounds);
        layout.ext_capacities=Some(self.extension_capacities.owner());
        ExtCompileConfig { bounds,layout,domain:ExtDomain::GENERIC_ABSENT,
            fuel:self.fuel.owner(),stack:self.stack.owner() }
    }
}
fn failure(stage:&'static str,actual_error:Value)->CompileFailure {
    CompileFailure {stage,actual_error,evaluation_performed:false,events_forwarded:"0".into(),completed:false}
}

/// General source-to-admitted command artifact; compile-only, no fixture branch.
/// Nominal parser bounds and owner inputs are explicit. Generic expression body
/// typechecking is NOT performed by this route. Source semantics/correspondence
/// and native export remain unqualified; target JSON is descriptor-only.
/// Program/profile/layout identities are the existing owner records, not new
/// invented canonical digests. File digests use the existing exact-byte helper.
pub fn compile(source:&str,config_source:&str,target_source:&str)->Result<Value,CompileFailure> {
    let config_json=crate::json::parse(config_source).map_err(|e|CompileFailure::core("config",e))?;
    let config:NominalConfig=serde_json::from_value(config_json)
        .map_err(|e|CompileFailure::input("config",e.to_string()))?;
    let target=crate::json::parse(target_source).map_err(|e|CompileFailure::core("config",e))?;
    let artifact=compile_nominal_host(source,config.preprocessing.parser_budget(),config.owner(),None)
        .map_err(|e|match e {
            NominalCompileFailure::Parse(e)=>failure("parse",super::draft_diagnostics::source(e)),
            NominalCompileFailure::Compile(ExtCompileFailure::Preparation(e))=>failure("admit",e.owner_wire()),
            NominalCompileFailure::Compile(ExtCompileFailure::Genesis(e))=>failure("admit-run",e.owner_wire()),
        })?;
    let report=artifact.report().map_err(|e|failure("observation",e.owner_wire()))?;
    Ok(json!({
        "format":ARTIFACT_FORMAT,
        "qualification":"HostOnly nominal source preparation; no body typechecking, constraints, native export or target qualification",
        "source":{"language":NOMINAL_LANGUAGE,"selection":{"kind":"entry"},
            "source_digest":crate::intent::hash_bytes(source.as_bytes()),
            "semantics":null,"semantics_status":"not-separately-qualified"},
        "execution_profile":report.execution_profile,
        "body_typechecking":"not-performed","semantic_correspondence":"unproved",
        "evaluation_performed":report.evaluation_performed,
        "config":{"fields":config,"file_digest":crate::intent::hash_bytes(config_source.as_bytes())},
        "target":{"descriptor":target,"file_digest":crate::intent::hash_bytes(target_source.as_bytes())},
        "admission":wire::admission(&report.owner),
        "preparation":report.preparation.owner_wire(),
        "plans":report.plans,
        "evidence":{"source_preparation":"performed","host_observation":"admitted-tables-only",
            "native":"unrun","target":"unrun","acceptance":"unestablished"},
    }))
}
