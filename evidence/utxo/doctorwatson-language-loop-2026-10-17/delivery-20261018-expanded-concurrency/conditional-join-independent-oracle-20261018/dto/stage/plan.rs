//! Source-derived scalar layouts. Admission and work costs belong to sibling modules.
use crate::{Error, InputDecl, Program, Statement, Type, UInt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    Input,
    Let,
    Event,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureSlot {
    pub name: String,
    pub ty: Type,
    pub origin: Origin,
    pub ordinal: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageBinder {
    pub name: String,
    pub ty: Type,
}
impl From<&InputDecl> for StageBinder {
    fn from(value: &InputDecl) -> Self {
        Self {
            name: value.name.clone(),
            ty: value.ty,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AwaitSite {
    pub site: u16,
    pub ordinal: u16,
    pub label: String,
    pub binders: Vec<StageBinder>,
    pub captures: Vec<CaptureSlot>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionPlan {
    pub action: String,
    pub statement_count: u16,
    pub sites: Vec<AwaitSite>,
    pub segment_cost: Vec<UInt>,
    pub committed_before: Vec<UInt>,
    pub plan_total: UInt,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StagePlan {
    pub format: String,
    pub program_hash: String,
    pub actions: Vec<ActionPlan>,
}

/// Builds layouts after validate_v2. Costs and hashes are filled by stage::load.
