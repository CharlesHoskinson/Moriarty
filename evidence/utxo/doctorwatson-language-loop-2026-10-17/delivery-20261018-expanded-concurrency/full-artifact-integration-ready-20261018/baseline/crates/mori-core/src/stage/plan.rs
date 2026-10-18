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
pub(crate) fn build(program: &Program) -> Result<StagePlan, Error> {
    let mut actions = Vec::new();
    for action in &program.actions {
        let mut types: BTreeMap<_, _> = program
            .states
            .iter()
            .map(|state| (state.name.clone(), state.ty))
            .collect();
        let mut captures = Vec::new();
        for (ordinal, input) in action.inputs.iter().enumerate() {
            types.insert(input.name.clone(), input.ty);
            captures.push(CaptureSlot {
                name: input.name.clone(),
                ty: input.ty,
                origin: Origin::Input,
                ordinal: ordinal as u16,
            });
        }
        let mut sites = Vec::new();
        for (ordinal, statement) in action.statements.iter().enumerate() {
            match statement {
                Statement::Let { name, value } => {
                    let ty = crate::runtime::expr_type(program, *value, &types, 0)?;
                    types.insert(name.clone(), ty);
                    captures.push(CaptureSlot {
                        name: name.clone(),
                        ty,
                        origin: Origin::Let,
                        ordinal: ordinal as u16,
                    });
                }
                Statement::Await { label, binders, .. } => {
                    sites.push(AwaitSite {
                        site: sites.len() as u16 + 1,
                        ordinal: ordinal as u16,
                        label: label.clone(),
                        binders: binders.iter().map(StageBinder::from).collect(),
                        captures: captures.clone(),
                    });
                    for binder in binders {
                        types.insert(binder.name.clone(), binder.ty);
                        captures.push(CaptureSlot {
                            name: binder.name.clone(),
                            ty: binder.ty,
                            origin: Origin::Event,
                            ordinal: ordinal as u16,
                        });
                    }
                }
                _ => {}
            }
        }
        actions.push(ActionPlan {
            action: action.name.clone(),
            statement_count: action.statements.len() as u16,
            sites,
            segment_cost: Vec::new(),
            committed_before: Vec::new(),
            plan_total: UInt(0),
        });
    }
    Ok(StagePlan {
        format: "moriarty-stage-plan/1".into(),
        program_hash: String::new(),
        actions,
    })
}
