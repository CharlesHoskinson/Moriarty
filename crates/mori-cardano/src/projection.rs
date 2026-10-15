//! Authorized A1/A2 UNIT projection, in memory only. No target byte encoding.
use crate::{AccountBinding, AccountClass, CoreSourceExportV1, InspectionError, LayoutMetadataV1, layout};
use mori_core::{
    AssetId, Balance, CostReport, Error, Expr, ExprId, Liability,
    MeteredExecution, Program, Scenario, Snapshot, Statement, Type, UInt, Value,
};
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet};

/// Lossless human notation for abstract Constr/I values, not CBOR or wire data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionData {
    Constr {
        constructor: u64,
        fields: Vec<ProjectionData>,
    },
    Integer(UInt),
}

// UInt needs the explicit {integer:"n"} human notation in a Data field.
#[derive(Serialize)]
struct IntegerNotation {
    integer: UInt,
}
#[derive(Serialize)]
struct ConstructorNotation<'a> {
    constructor: u64,
    fields: &'a [ProjectionData],
}
impl Serialize for ProjectionData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Integer(integer) => IntegerNotation { integer: *integer }.serialize(serializer),
            Self::Constr {
                constructor,
                fields,
            } => ConstructorNotation {
                constructor: *constructor,
                fields,
            }
            .serialize(serializer),
        }
    }
}

/// Canonical in-memory duty: absence is exactly (false, 0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DutyCell {
    present: bool,
    amount: UInt,
}
impl DutyCell {
    pub fn new(present: bool, amount: UInt) -> Result<Self, ProjectionError> {
        if !present && amount.0 != 0 {
            return Err(shape("Absent duty amount is not zero"));
        }
        Ok(Self { present, amount })
    }
    pub fn present(&self) -> bool {
        self.present
    }
    pub fn amount(&self) -> UInt {
        self.amount
    }
}
impl Serialize for DutyCell {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_constr(
            0,
            vec![
                ProjectionData::Integer(UInt(u128::from(self.present))),
                ProjectionData::Integer(self.amount),
            ],
        )
        .serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectionBody {
    state_values: Vec<ProjectionData>,
    framed_amounts: Vec<UInt>,
    duties: Vec<DutyCell>,
}
impl ProjectionBody {
    pub fn new(
        state_values: Vec<ProjectionData>,
        framed_amounts: Vec<UInt>,
        duties: Vec<DutyCell>,
    ) -> Self {
        Self {
            state_values,
            framed_amounts,
            duties,
        }
    }
    pub fn state_values(&self) -> &[ProjectionData] {
        &self.state_values
    }
    pub fn framed_amounts(&self) -> &[UInt] {
        &self.framed_amounts
    }
    pub fn duties(&self) -> &[DutyCell] {
        &self.duties
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionProjectionMetadata {
    pub action: String,
    pub logical_work: UInt,
    pub required_signers: Vec<String>,
}

/// Additive /2 metadata; the adopted serialized metadata/1 is unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectionInspectionV2 {
    pub format: String,
    pub qualification: String,
    pub source: CoreSourceExportV1,
    pub layout: LayoutMetadataV1,
    pub actions: Vec<ActionProjectionMetadata>,
    pub unreferenced_accounts: Vec<String>,
    pub binding_rule: String,
}

/// Only source-owned preparation constructs this context. Public metadata is
/// never accepted as an alternative Program, layout, or projection authority.
pub struct ProjectionContext {
    inspection: ProjectionInspectionV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    Inspection(InspectionError),
    Shape(String),
    UnknownAction(String),
    CoreInvariant(String),
    ProjectionInvariant(Error),
    ReferenceRefused { error: Error, cost: CostReport },
}
impl std::fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ProjectionError {}
fn shape(message: &str) -> ProjectionError {
    ProjectionError::Shape(message.into())
}
fn data_constr(constructor: u64, fields: Vec<ProjectionData>) -> ProjectionData {
    ProjectionData::Constr {
        constructor,
        fields,
    }
}
fn encode_value(value: &Value) -> ProjectionData {
    match value {
        Value::UInt(n) => data_constr(0, vec![ProjectionData::Integer(*n)]),
        Value::Bool(b) => data_constr(1, vec![ProjectionData::Integer(UInt(u128::from(*b)))]),
    }
}
fn decode_value(data: &ProjectionData, ty: Type) -> Result<Value, ProjectionError> {
    match (ty, data) {
        (
            Type::UInt,
            ProjectionData::Constr {
                constructor: 0,
                fields,
            },
        ) if fields.len() == 1 => {
            if let ProjectionData::Integer(n) = fields[0] {
                Ok(Value::UInt(n))
            } else {
                Err(shape("UInt field is not an Integer"))
            }
        }
        (
            Type::Bool,
            ProjectionData::Constr {
                constructor: 1,
                fields,
            },
        ) if fields.len() == 1 => match fields[0] {
            ProjectionData::Integer(UInt(0)) => Ok(Value::Bool(false)),
            ProjectionData::Integer(UInt(1)) => Ok(Value::Bool(true)),
            _ => Err(shape("Bool field is not 0 or 1")),
        },
        _ => Err(shape("Value constructor, arity or declared type differs")),
    }
}

fn statement_root(statement: &Statement) -> Result<ExprId, ProjectionError> {
    Ok(match statement {
        Statement::Let { value, .. }
        | Statement::Next { value, .. }
        | Statement::Emit { value, .. } => *value,
        Statement::Requires { predicate } | Statement::Ensures { predicate } => *predicate,
        Statement::Transfer { amount, .. }
        | Statement::Fee { amount, .. }
        | Statement::Liability { amount, .. }
        | Statement::Discharge { amount, .. } => *amount,
        Statement::Await { .. } => {
            return Err(ProjectionError::CoreInvariant(
                "Atomic /1 logical work cannot represent Await; use the staged reference API".into(),
            ));
        }
    })
}
fn logical_work(program: &Program, statements: &[Statement]) -> Result<UInt, ProjectionError> {
    let mut total = 0u128;
    for statement in statements {
        let mut nodes = BTreeSet::new();
        let mut pending = vec![statement_root(statement)?];
        while let Some(id) = pending.pop() {
            if !nodes.insert(id) {
                continue;
            }
            match program.expressions.get(usize::from(id)).ok_or_else(|| {
                ProjectionError::CoreInvariant("Validated expression index missing".into())
            })? {
                Expr::Binary { lhs, rhs, .. } => {
                    pending.push(*rhs);
                    pending.push(*lhs);
                }
                Expr::Not { value } => pending.push(*value),
                _ => {}
            }
        }
        total += 1 + nodes.len() as u128;
    }
    Ok(UInt(total))
}

/// Reparse source and privately derive the A1/A2 context. Unit-only, no layout
/// hash, physical credentials, trace, or profile/ledger admission is produced.
pub fn prepare_projection(
    source: &str,
    accounts: &[AccountBinding],
) -> Result<ProjectionContext, ProjectionError> {
    let source = crate::export_source(source)
        .map_err(|e| ProjectionError::Inspection(InspectionError::Core(e)))?;
    let layout = layout::describe_projection(&source.program, accounts)
        .map_err(ProjectionError::Inspection)?;
    let classes: BTreeMap<_, _> = layout
        .accounts
        .iter()
        .map(|a| (a.name.as_str(), a.class))
        .collect();
    let mut referenced = BTreeSet::new();
    let mut actions = Vec::new();
    for action in &source.program.actions {
        let mut signers = BTreeSet::new();
        for statement in &action.statements {
            if let Statement::Transfer { from, to, .. } | Statement::Fee { from, to, .. } =
                statement
            {
                referenced.insert(from.as_str());
                referenced.insert(to.as_str());
                if classes.get(from.as_str()) == Some(&AccountClass::KeyHeldExternal) {
                    signers.insert(from.clone());
                }
            }
        }
        actions.push(ActionProjectionMetadata {
            action: action.name.clone(),
            logical_work: logical_work(&source.program, &action.statements)?,
            required_signers: signers.into_iter().collect(),
        });
    }
    let unreferenced_accounts = layout
        .accounts
        .iter()
        .filter(|a| !referenced.contains(a.name.as_str()))
        .map(|a| a.name.clone())
        .collect();
    Ok(ProjectionContext {
        inspection: ProjectionInspectionV2 {
            format: "moriarty-cardano-projection-inspection/2".into(),
            qualification: "UNIT".into(),
            source,
            layout,
            actions,
            unreferenced_accounts,
            binding_rule: "R5S-06 symbolic names; no physical credentials".into(),
        },
    })
}

impl ProjectionContext {
    pub fn inspection(&self) -> &ProjectionInspectionV2 {
        &self.inspection
    }
    fn balances(
        &self,
        balances: &[Balance],
    ) -> Result<BTreeMap<(AssetId, String), UInt>, ProjectionError> {
        let layout = &self.inspection.layout;
        let mut values = BTreeMap::new();
        for row in balances {
            if !layout.accounts.iter().any(|a| a.name == row.account)
                || !layout.assets.iter().any(|a| a.identity == row.asset)
            {
                return Err(shape("Balance outside the source account/asset matrix"));
            }
            if values
                .insert((row.asset.clone(), row.account.clone()), row.amount)
                .is_some()
            {
                return Err(shape("Duplicate balance cell"));
            }
        }
        if values.len() != layout.projected_balance_cells {
            return Err(shape("Incomplete balance matrix"));
        }
        Ok(values)
    }
    fn duties(&self, liabilities: &[Liability]) -> Result<BTreeMap<String, UInt>, ProjectionError> {
        let mut amounts = BTreeMap::new();
        for row in liabilities {
            let duty = self
                .inspection
                .layout
                .duties
                .iter()
                .find(|d| d.name == row.name)
                .ok_or_else(|| shape("Duty outside the source universe"))?;
            if row.asset != duty.asset || row.debtor != duty.debtor || row.creditor != duty.creditor
            {
                return Err(shape("Duty identity differs from source"));
            }
            if amounts.insert(row.name.clone(), row.amount).is_some() {
                return Err(shape("Duplicate duty row"));
            }
        }
        Ok(amounts)
    }
    pub fn project_body(&self, snapshot: &Snapshot) -> Result<ProjectionBody, ProjectionError> {
        let layout = &self.inspection.layout;
        if snapshot.state.len() != layout.states.len() {
            return Err(shape("State field count differs"));
        }
        let mut states = Vec::new();
        for state in &layout.states {
            let value = snapshot
                .state
                .get(&state.name)
                .ok_or_else(|| shape("Missing state field"))?;
            if value.ty() != state.ty {
                return Err(shape("State type differs"));
            }
            states.push(encode_value(value));
        }
        let balances = self.balances(&snapshot.balances)?;
        let amounts = layout
            .framed_cells
            .iter()
            .map(|c| {
                balances
                    .get(&(c.asset.clone(), c.account.clone()))
                    .copied()
                    .ok_or_else(|| shape("Missing framed balance"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let present = self.duties(&snapshot.liabilities)?;
        let duties = layout
            .duties
            .iter()
            .map(|d| match present.get(&d.name) {
                Some(n) => DutyCell::new(true, *n),
                None => DutyCell::new(false, UInt(0)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ProjectionBody::new(states, amounts, duties))
    }
    pub fn scenario_from_body(
        &self,
        body: &ProjectionBody,
        premises: &[Balance],
        action: &str,
        inputs: &[ProjectionData],
    ) -> Result<Scenario, ProjectionError> {
        let layout = &self.inspection.layout;
        let selected = self
            .inspection
            .source
            .program
            .actions
            .iter()
            .find(|a| a.name == action)
            .ok_or_else(|| ProjectionError::UnknownAction(action.into()))?;
        if body.state_values.len() != layout.states.len()
            || body.framed_amounts.len() != layout.framed_cells.len()
            || body.duties.len() != layout.duties.len()
            || inputs.len() != selected.inputs.len()
        {
            return Err(shape("Body or input field count differs"));
        }
        let state = layout
            .states
            .iter()
            .zip(&body.state_values)
            .map(|(s, v)| Ok((s.name.clone(), decode_value(v, s.ty)?)))
            .collect::<Result<BTreeMap<_, _>, ProjectionError>>()?;
        let inputs = selected
            .inputs
            .iter()
            .zip(inputs)
            .map(|(i, v)| Ok((i.name.clone(), decode_value(v, i.ty)?)))
            .collect::<Result<BTreeMap<_, _>, ProjectionError>>()?;
        let mut balances = BTreeMap::new();
        for (cell, amount) in layout.framed_cells.iter().zip(&body.framed_amounts) {
            balances.insert((cell.asset.clone(), cell.account.clone()), *amount);
        }
        for row in premises {
            if !layout
                .accounts
                .iter()
                .any(|a| a.name == row.account && a.class == AccountClass::KeyHeldExternal)
                || !layout.assets.iter().any(|a| a.identity == row.asset)
            {
                return Err(shape("Premise outside the KeyHeld matrix"));
            }
            if balances
                .insert((row.asset.clone(), row.account.clone()), row.amount)
                .is_some()
            {
                return Err(shape("Duplicate premise cell"));
            }
        }
        if balances.len() != layout.projected_balance_cells {
            return Err(shape("Incomplete premise matrix"));
        }
        let balances = balances
            .into_iter()
            .map(|((asset, account), amount)| Balance {
                account,
                asset,
                amount,
            })
            .collect();
        let liabilities = layout
            .duties
            .iter()
            .zip(&body.duties)
            .filter(|(_, c)| c.present)
            .map(|(d, c)| Liability {
                name: d.name.clone(),
                asset: d.asset.clone(),
                debtor: d.debtor.clone(),
                creditor: d.creditor.clone(),
                amount: c.amount,
            })
            .collect();
        Ok(Scenario {
            state,
            inputs,
            balances,
            liabilities,
        })
    }
    pub fn project_reference(
        &self,
        body: &ProjectionBody,
        premises: &[Balance],
        action: &str,
        inputs: &[ProjectionData],
    ) -> Result<MeteredExecution, ProjectionError> {
        let scenario = self.scenario_from_body(body, premises, action, inputs)?;
        let work = self
            .inspection
            .actions
            .iter()
            .find(|a| a.action == action)
            .ok_or_else(|| ProjectionError::UnknownAction(action.into()))?
            .logical_work;
        match mori_core::simulate_metered(&self.inspection.source.program, action, &scenario, work)
        {
            Ok(result) if result.cost.used == work => Ok(result),
            Ok(_) => Err(ProjectionError::CoreInvariant(
                "Completed logical work differs from W(a)".into(),
            )),
            Err(refused) if refused.error.code == "mori::core::meter" => Err(
                ProjectionError::CoreInvariant("Reference meter exhausted under W(a)".into()),
            ),
            Err(refused) if refused.error.step.is_none() => {
                Err(ProjectionError::ProjectionInvariant(refused.error))
            }
            Err(refused) => Err(ProjectionError::ReferenceRefused {
                error: refused.error,
                cost: refused.cost,
            }),
        }
    }
}
