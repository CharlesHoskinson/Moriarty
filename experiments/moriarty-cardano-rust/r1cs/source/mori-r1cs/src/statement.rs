//! Typed statement layout and claimed transition values.
//!
//! The layout is the statement the relation is about: header constants first, then
//! every body slot in a fixed order derived from the plan (AUTHOR-EXPECTATIONS
//! `statement_layout_v0`, UNADOPTED):
//!
//! 1. header: relation version, program hash, profile digest, action index, shape
//!    digest, compiler build digest, logical work `W`, and the cap when bound;
//! 2. inputs in declaration order;
//! 3. pre states (by name), pre cells (by `(AssetId, account)`), pre duties (by name,
//!    each `present` then `amount`);
//! 4. post states, post cells, post duties in the same order;
//! 5. effects in statement order (State: before, after; Transfer/Fee: amount;
//!    Liability: before, after; Emit: value);
//! 6. the accounting summary when bound (per debit cell: gross, fees).
//!
//! Encodings are exact: a UInt is `[lo64, hi64]`, a Bool one field, a digest
//! `[hi128, lo128]` (big-endian halves), a small constant one field, a wide sum
//! `[lo64, mid64, top16]`. The circuit forces every limb into its range, so each
//! public vector denotes exactly one integer statement (no modular alias).
use crate::error::{CompileError, MappingError};
use crate::identity::{self, CompilerBuild, Digest32, ProgramIdentity};
use crate::plan::{AccountingBinding, ActionPlan, CapBinding, EffectShape};
use crate::r1cs::{fe, fe_bool};
use ark_ff::PrimeField;
use mori_core::{Type, UInt, Value};
use serde::Serialize;

const MASK64: u128 = u64::MAX as u128;

/// Exact integer `lo + 2^128 * top`, below 2^144.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Wide {
    pub lo: u128,
    pub top: u16,
}

impl Wide {
    pub const ZERO: Wide = Wide { lo: 0, top: 0 };

    /// Adds a u128. Callers sum at most 1,024 effect amounts (Core's statement bound),
    /// so `top` stays below 2^10 and the wrapping add never wraps.
    pub fn plus(self, x: u128) -> Wide {
        let (lo, carry) = self.lo.overflowing_add(x);
        Wide { lo, top: self.top.wrapping_add(u16::from(carry)) }
    }

    pub fn bit(self, i: u32) -> bool {
        if i < 128 { (self.lo >> i) & 1 == 1 } else { (self.top >> (i - 128)) & 1 == 1 }
    }

    pub fn from_u128(v: u128) -> Wide {
        Wide { lo: v, top: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SlotEncoding {
    /// One field holding a constant below 2^64.
    Small,
    /// Two fields `[hi128, lo128]`: big-endian halves of a 32-byte digest.
    Digest,
    /// Two fields `[lo64, hi64]`, each forced below 2^64.
    UInt,
    /// One field forced to {0, 1}.
    Bool,
    /// Three fields `[lo64, mid64, top16]` of an exact sum below 2^144.
    Wide,
}

impl SlotEncoding {
    pub fn width(self) -> usize {
        match self {
            Self::Small | Self::Bool => 1,
            Self::Digest | Self::UInt => 2,
            Self::Wide => 3,
        }
    }
    fn of(ty: Type) -> Self {
        match ty {
            Type::UInt => Self::UInt,
            Type::Bool => Self::Bool,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HeaderField {
    RelationVersion,
    ProgramHash,
    ProfileDigest,
    ActionIndex,
    ShapeDigest,
    CompilerBuild,
    LogicalWork,
    Cap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EffectRole {
    StateBefore,
    StateAfter,
    Amount,
    DutyBefore,
    DutyAfter,
    EmitValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SlotKind {
    Header(HeaderField),
    Input { index: usize },
    PreState { index: usize },
    PreCell { index: usize },
    PreDutyPresent { index: usize },
    PreDutyAmount { index: usize },
    PostState { index: usize },
    PostCell { index: usize },
    PostDutyPresent { index: usize },
    PostDutyAmount { index: usize },
    Effect { index: usize, role: EffectRole },
    GrossDebit { cell: usize },
    FeesPaid { cell: usize },
}

impl SlotKind {
    pub fn is_header(&self) -> bool {
        matches!(self, Self::Header(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Slot {
    pub kind: SlotKind,
    pub encoding: SlotEncoding,
    /// Index of the slot's first field in the encoded statement.
    pub offset: usize,
    pub label: String,
}

/// Header constants. All of them are bound in-circuit as constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeaderValues {
    pub relation_version: u64,
    pub program_hash: Digest32,
    pub profile_digest: Digest32,
    pub action_index: u16,
    pub shape_digest: Digest32,
    pub compiler_build_digest: Digest32,
    pub logical_work: UInt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatementLayout {
    pub header: HeaderValues,
    pub slots: Vec<Slot>,
    pub field_count: usize,
}

/// A claimed duty cell. Absent is exactly `(false, 0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DutyValue {
    pub present: bool,
    pub amount: UInt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BodyValues {
    /// Plan state order (by name).
    pub states: Vec<Value>,
    /// Plan cell order.
    pub cells: Vec<UInt>,
    /// Plan duty order.
    pub duties: Vec<DutyValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum EffectValues {
    State { before: Value, after: Value },
    Amount(UInt),
    Duty { before: UInt, after: UInt },
    Emit(Value),
}

/// The claimed transition: everything a prover asserts and a verifier sees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TransitionValues {
    /// Declaration order.
    pub inputs: Vec<Value>,
    pub pre: BodyValues,
    pub post: BodyValues,
    /// Statement order.
    pub effects: Vec<EffectValues>,
    /// Present exactly when the plan binds a public cap.
    pub cap: Option<UInt>,
}

#[derive(Serialize)]
struct ShapeDescriptor<'a> {
    format: &'static str,
    relation_format: &'static str,
    relation_version: u64,
    program_hash: Digest32,
    plan: &'a ActionPlan,
    slots: Vec<(SlotKind, SlotEncoding, &'a str)>,
}

struct Builder {
    slots: Vec<Slot>,
    offset: usize,
}

impl Builder {
    fn push(&mut self, kind: SlotKind, encoding: SlotEncoding, label: String) {
        self.slots.push(Slot { kind, encoding, offset: self.offset, label });
        self.offset += encoding.width();
    }
}

fn effect_label(i: usize, e: &EffectShape) -> String {
    match e {
        EffectShape::State { name, .. } => format!("effect.{i}.state.{name}"),
        EffectShape::Transfer { from, to, .. } => format!("effect.{i}.transfer.{from}.{to}"),
        EffectShape::Fee { from, to, .. } => format!("effect.{i}.fee.{from}.{to}"),
        EffectShape::Liability { name, .. } => format!("effect.{i}.liability.{name}"),
        EffectShape::Emit { name, .. } => format!("effect.{i}.emit.{name}"),
    }
}

impl StatementLayout {
    pub(crate) fn build(program: &ProgramIdentity, plan: &ActionPlan, build: CompilerBuild) -> Result<Self, CompileError> {
        use HeaderField as H;
        use SlotEncoding as E;
        let mut b = Builder { slots: Vec::new(), offset: 0 };
        b.push(SlotKind::Header(H::RelationVersion), E::Small, "header.relation_version".into());
        b.push(SlotKind::Header(H::ProgramHash), E::Digest, "header.program_hash".into());
        b.push(SlotKind::Header(H::ProfileDigest), E::Digest, "header.profile_digest".into());
        b.push(SlotKind::Header(H::ActionIndex), E::Small, "header.action_index".into());
        b.push(SlotKind::Header(H::ShapeDigest), E::Digest, "header.shape_digest".into());
        b.push(SlotKind::Header(H::CompilerBuild), E::Digest, "header.compiler_build".into());
        b.push(SlotKind::Header(H::LogicalWork), E::UInt, "header.logical_work".into());
        if plan.cap == CapBinding::PublicCap {
            b.push(SlotKind::Header(H::Cap), E::UInt, "header.cap".into());
        }
        for (i, s) in plan.inputs.iter().enumerate() {
            b.push(SlotKind::Input { index: i }, E::of(s.ty), format!("input.{}", s.name));
        }
        for (phase, post) in [("pre", false), ("post", true)] {
            for (i, s) in plan.states.iter().enumerate() {
                let kind = if post { SlotKind::PostState { index: i } } else { SlotKind::PreState { index: i } };
                b.push(kind, E::of(s.ty), format!("{phase}.state.{}", s.name));
            }
            for (i, c) in plan.cells.iter().enumerate() {
                let kind = if post { SlotKind::PostCell { index: i } } else { SlotKind::PreCell { index: i } };
                b.push(kind, E::UInt, format!("{phase}.cell.{}.{}", c.asset_name, c.account));
            }
            for (i, d) in plan.duties.iter().enumerate() {
                let (p, a) = if post {
                    (SlotKind::PostDutyPresent { index: i }, SlotKind::PostDutyAmount { index: i })
                } else {
                    (SlotKind::PreDutyPresent { index: i }, SlotKind::PreDutyAmount { index: i })
                };
                b.push(p, E::Bool, format!("{phase}.duty.{}.present", d.name));
                b.push(a, E::UInt, format!("{phase}.duty.{}.amount", d.name));
            }
        }
        for (i, e) in plan.effects.iter().enumerate() {
            let label = effect_label(i, e);
            match e {
                EffectShape::State { ty, .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::StateBefore }, E::of(*ty), format!("{label}.before"));
                    b.push(SlotKind::Effect { index: i, role: EffectRole::StateAfter }, E::of(*ty), format!("{label}.after"));
                }
                EffectShape::Transfer { .. } | EffectShape::Fee { .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::Amount }, E::UInt, format!("{label}.amount"));
                }
                EffectShape::Liability { .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::DutyBefore }, E::UInt, format!("{label}.before"));
                    b.push(SlotKind::Effect { index: i, role: EffectRole::DutyAfter }, E::UInt, format!("{label}.after"));
                }
                EffectShape::Emit { ty, .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::EmitValue }, E::of(*ty), format!("{label}.value"));
                }
            }
        }
        if plan.accounting == AccountingBinding::GrossDebitAndFees {
            for &cell in &plan.debit_cells {
                let c = &plan.cells[cell];
                b.push(SlotKind::GrossDebit { cell }, E::Wide, format!("accounting.{}.{}.gross_debit", c.asset_name, c.account));
                b.push(SlotKind::FeesPaid { cell }, E::Wide, format!("accounting.{}.{}.fees_paid", c.asset_name, c.account));
            }
        }
        let Builder { slots, offset } = b;
        let shape_digest = identity::digest_of(&ShapeDescriptor {
            format: "mori-r1cs-shape/1",
            relation_format: crate::RELATION_FORMAT,
            relation_version: crate::RELATION_VERSION,
            program_hash: program.program_hash,
            plan,
            slots: slots.iter().map(|s| (s.kind, s.encoding, s.label.as_str())).collect(),
        })?;
        let header = HeaderValues {
            relation_version: crate::RELATION_VERSION,
            program_hash: program.program_hash,
            profile_digest: identity::profile_digest()?,
            action_index: plan.action_index,
            shape_digest,
            compiler_build_digest: build.digest()?,
            logical_work: plan.logical_work,
        };
        Ok(Self { header, slots, field_count: offset })
    }

    /// Header constant fields, in slot order; `None` for the cap (a claimed value).
    pub(crate) fn header_fields<F: PrimeField>(&self, field: HeaderField) -> Option<Vec<F>> {
        let h = &self.header;
        Some(match field {
            HeaderField::RelationVersion => vec![fe(u128::from(h.relation_version))],
            HeaderField::ProgramHash => digest_fields(&h.program_hash),
            HeaderField::ProfileDigest => digest_fields(&h.profile_digest),
            HeaderField::ActionIndex => vec![fe(u128::from(h.action_index))],
            HeaderField::ShapeDigest => digest_fields(&h.shape_digest),
            HeaderField::CompilerBuild => digest_fields(&h.compiler_build_digest),
            HeaderField::LogicalWork => uint_fields(h.logical_work.0),
            HeaderField::Cap => return None,
        })
    }

    /// Exact public encoding of a claimed transition. Refuses values whose shape or
    /// types differ from the plan, and absent duties with a nonzero amount.
    pub fn encode<F: PrimeField>(&self, plan: &ActionPlan, values: &TransitionValues) -> Result<Vec<F>, MappingError> {
        check_values(plan, values)?;
        let summary = accounting_summary(plan, values);
        let mut out = Vec::with_capacity(self.field_count);
        for slot in &self.slots {
            let fields: Vec<F> = match slot.kind {
                SlotKind::Header(HeaderField::Cap) => {
                    uint_fields(values.cap.ok_or_else(|| MappingError::Cap("cap slot without a claimed cap".into()))?.0)
                }
                SlotKind::Header(field) => self.header_fields(field).expect("non-cap header field"),
                SlotKind::Input { index } => value_fields(&values.inputs[index]),
                SlotKind::PreState { index } => value_fields(&values.pre.states[index]),
                SlotKind::PostState { index } => value_fields(&values.post.states[index]),
                SlotKind::PreCell { index } => uint_fields(values.pre.cells[index].0),
                SlotKind::PostCell { index } => uint_fields(values.post.cells[index].0),
                SlotKind::PreDutyPresent { index } => vec![fe_bool(values.pre.duties[index].present)],
                SlotKind::PreDutyAmount { index } => uint_fields(values.pre.duties[index].amount.0),
                SlotKind::PostDutyPresent { index } => vec![fe_bool(values.post.duties[index].present)],
                SlotKind::PostDutyAmount { index } => uint_fields(values.post.duties[index].amount.0),
                SlotKind::Effect { index, role } => effect_fields(&values.effects[index], role)?,
                SlotKind::GrossDebit { cell } => wide_fields(summary_for(&summary, cell)?.1),
                SlotKind::FeesPaid { cell } => wide_fields(summary_for(&summary, cell)?.2),
            };
            if fields.len() != slot.encoding.width() {
                return Err(MappingError::Shape(format!("slot {} encodes {} fields, layout says {}", slot.label, fields.len(), slot.encoding.width())));
            }
            out.extend(fields);
        }
        debug_assert_eq!(out.len(), self.field_count);
        Ok(out)
    }

    pub fn slot(&self, label: &str) -> Option<&Slot> {
        self.slots.iter().find(|s| s.label == label)
    }
}

/// Host-side accounting summary of a claimed transition: `(cell, gross, fees)` per
/// debit cell. It is only the claimant's encoding of public slots; the circuit
/// recomputes both sums from its own effect wires and binds them.
pub fn accounting_summary(plan: &ActionPlan, values: &TransitionValues) -> Vec<(usize, Wide, Wide)> {
    plan.debit_cells
        .iter()
        .map(|&cell| {
            let mut gross = Wide::ZERO;
            let mut fees = Wide::ZERO;
            for (shape, value) in plan.effects.iter().zip(&values.effects) {
                if let (Some(from), EffectValues::Amount(amount)) = (shape.debit_cell(), value) {
                    if from == cell {
                        gross = gross.plus(amount.0);
                        if shape.is_fee() {
                            fees = fees.plus(amount.0);
                        }
                    }
                }
            }
            (cell, gross, fees)
        })
        .collect()
}

fn summary_for(summary: &[(usize, Wide, Wide)], cell: usize) -> Result<(usize, Wide, Wide), MappingError> {
    summary.iter().copied().find(|(c, _, _)| *c == cell).ok_or_else(|| MappingError::Shape(format!("no accounting row for cell {cell}")))
}

pub(crate) fn uint_fields<F: PrimeField>(v: u128) -> Vec<F> {
    vec![fe(v & MASK64), fe(v >> 64)]
}

pub(crate) fn digest_fields<F: PrimeField>(d: &Digest32) -> Vec<F> {
    let (hi, lo) = d.halves();
    vec![fe(hi), fe(lo)]
}

fn wide_fields<F: PrimeField>(w: Wide) -> Vec<F> {
    vec![fe(w.lo & MASK64), fe(w.lo >> 64), fe(u128::from(w.top))]
}

fn value_fields<F: PrimeField>(v: &Value) -> Vec<F> {
    match v {
        Value::UInt(UInt(n)) => uint_fields(*n),
        Value::Bool(b) => vec![fe_bool(*b)],
    }
}

fn effect_fields<F: PrimeField>(e: &EffectValues, role: EffectRole) -> Result<Vec<F>, MappingError> {
    Ok(match (e, role) {
        (EffectValues::State { before, .. }, EffectRole::StateBefore) => value_fields(before),
        (EffectValues::State { after, .. }, EffectRole::StateAfter) => value_fields(after),
        (EffectValues::Amount(a), EffectRole::Amount) => uint_fields(a.0),
        (EffectValues::Duty { before, .. }, EffectRole::DutyBefore) => uint_fields(before.0),
        (EffectValues::Duty { after, .. }, EffectRole::DutyAfter) => uint_fields(after.0),
        (EffectValues::Emit(v), EffectRole::EmitValue) => value_fields(v),
        _ => return Err(MappingError::Shape(format!("effect value {e:?} has no role {role:?}"))),
    })
}

fn check_type(v: &Value, ty: Type, what: &str) -> Result<(), MappingError> {
    if v.ty() == ty { Ok(()) } else { Err(MappingError::Shape(format!("{what}: expected {ty:?}, got {:?}", v.ty()))) }
}

fn check_body(plan: &ActionPlan, body: &BodyValues, phase: &str) -> Result<(), MappingError> {
    if body.states.len() != plan.states.len() || body.cells.len() != plan.cells.len() || body.duties.len() != plan.duties.len() {
        return Err(MappingError::Shape(format!("{phase} body counts differ from the plan")));
    }
    for (v, s) in body.states.iter().zip(&plan.states) {
        check_type(v, s.ty, &format!("{phase} state {}", s.name))?;
    }
    for (d, k) in body.duties.iter().zip(&plan.duties) {
        if !d.present && d.amount.0 != 0 {
            return Err(MappingError::Shape(format!("{phase} duty {} is absent with a nonzero amount", k.name)));
        }
    }
    Ok(())
}

pub(crate) fn check_values(plan: &ActionPlan, values: &TransitionValues) -> Result<(), MappingError> {
    if values.inputs.len() != plan.inputs.len() {
        return Err(MappingError::Shape("input count differs from the plan".into()));
    }
    for (v, s) in values.inputs.iter().zip(&plan.inputs) {
        check_type(v, s.ty, &format!("input {}", s.name))?;
    }
    check_body(plan, &values.pre, "pre")?;
    check_body(plan, &values.post, "post")?;
    if values.effects.len() != plan.effects.len() {
        return Err(MappingError::Shape("effect count differs from the plan".into()));
    }
    for (i, (e, s)) in values.effects.iter().zip(&plan.effects).enumerate() {
        let ok = match (s, e) {
            (EffectShape::State { ty, .. }, EffectValues::State { before, after }) => before.ty() == *ty && after.ty() == *ty,
            (EffectShape::Transfer { .. } | EffectShape::Fee { .. }, EffectValues::Amount(_)) => true,
            (EffectShape::Liability { .. }, EffectValues::Duty { .. }) => true,
            (EffectShape::Emit { ty, .. }, EffectValues::Emit(v)) => v.ty() == *ty,
            _ => false,
        };
        if !ok {
            return Err(MappingError::Shape(format!("effect {i} value does not match its shape")));
        }
    }
    match (plan.cap, values.cap) {
        (CapBinding::PublicCap, Some(_)) | (CapBinding::NotBound, None) => Ok(()),
        (CapBinding::PublicCap, None) => Err(MappingError::Cap("relation binds a public cap; none claimed".into())),
        (CapBinding::NotBound, Some(_)) => Err(MappingError::Cap("relation binds no cap; one was claimed".into())),
    }
}
