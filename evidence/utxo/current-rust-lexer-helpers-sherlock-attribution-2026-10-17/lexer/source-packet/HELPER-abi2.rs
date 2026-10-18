//! `SettlementHashV2` adapter: MRG16-PVC ABI v2 statement bytes and their in-circuit
//! binding (P3S1, PROPOSED, unreviewed, never compiled or run).
//!
//! Normative text: `candidate/docs/ABI-V2.md` (sections 3 and 4). The statement is
//!
//! ```text
//! tag(16) family(32) predicate_digest(32) shape_digest(32)          -- circuit constants
//! vk_digest(32) claimant_vkh(28) network_id(1) settle_hash(28)
//! thread_id(32) consumed_txid(32) consumed_ix(2) seq(8)
//! catalogue_digest(32) intents_digest(32)                           -- free witness bytes
//! body_len(4)                                                       -- circuit constant
//! body                                                              -- slot wires
//! ```
//!
//! and the two public inputs are the big-endian 128-bit halves of BLAKE2b-256(statement).
//! The body is every non-header slot of the [`StatementLayout`] in layout order (UInt 16
//! bytes big-endian, Bool 1 byte, wide sums 18 bytes big-endian). The header slots are
//! covered by `predicate_digest` = [`crate::RelationIdentity::digest`], whose bound
//! identity includes this adapter's name. Context bytes are hashed, not interpreted:
//! the Cardano validator (S2) derives each one from its ScriptContext and recomputes the
//! statement; the proof only certifies that the transition values satisfy the relation.
use crate::adapter::{AdapterError, BindError, SlotWire};
use crate::blake2b::{self, Bit};
use crate::error::{CompileError, MappingError};
use crate::gadgets::{UIntW, WideW};
use crate::plan::{AccountingBinding, ActionPlan, CapBinding, CompileOptions, DutyOrigin, EffectShape, FrameCells};
use crate::r1cs::{Cs, Lc, fe};
use crate::statement::{EffectValues, SlotEncoding, SlotKind, StatementLayout, TransitionValues, Wide, accounting_summary};
use ark_ff::PrimeField;
use mori_core::{AssetId, Program, Statement, Type, UInt, Value};
use serde::Serialize;

pub const STMT_TAG: &[u8; 16] = b"MRG16-PVC-STMT-2";
pub const FAMILY_V2: &[u8; 32] = b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2";
pub const SHAPE_TAG: &[u8; 16] = b"MRG16-PVC-SHP-02";
/// Bytes before the body.
pub const HEADER_LEN: usize = 343;

/// Per-proof context bytes: hashed into the statement, never interpreted in-circuit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContextV2 {
    pub vk_digest: [u8; 32],
    pub claimant_vkh: [u8; 28],
    pub network_id: u8,
    pub settle_hash: [u8; 28],
    pub thread_id: [u8; 32],
    pub consumed_txid: [u8; 32],
    pub consumed_ix: u16,
    pub seq: u64,
    pub catalogue_digest: [u8; 32],
    pub intents_digest: [u8; 32],
}

impl ContextV2 {
    fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LEN - 116);
        out.extend_from_slice(&self.vk_digest);
        out.extend_from_slice(&self.claimant_vkh);
        out.push(self.network_id);
        out.extend_from_slice(&self.settle_hash);
        out.extend_from_slice(&self.thread_id);
        out.extend_from_slice(&self.consumed_txid);
        out.extend_from_slice(&self.consumed_ix.to_be_bytes());
        out.extend_from_slice(&self.seq.to_be_bytes());
        out.extend_from_slice(&self.catalogue_digest);
        out.extend_from_slice(&self.intents_digest);
        out
    }
}

/// Relation constants of the statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AbiV2Constants {
    pub predicate_digest: [u8; 32],
    pub shape_bytes: Vec<u8>,
    pub shape_digest: [u8; 32],
    pub body_len: u32,
}

/// ABI v2 fixes the compile options: no public cap (W is a constant inside the
/// predicate digest), the in-circuit gross-debit and fee sums (P3-3(d)), and no frame
/// cells (the Cardano datum carries the complete snapshot and S2 enforces the frame).
pub fn options_ok(options: &CompileOptions) -> Result<(), CompileError> {
    if options.cap != CapBinding::NotBound {
        return Err(CompileError::Option("SettlementHashV2 requires CapBinding::NotBound".into()));
    }
    if options.accounting != AccountingBinding::GrossDebitAndFees {
        return Err(CompileError::Option("SettlementHashV2 requires AccountingBinding::GrossDebitAndFees".into()));
    }
    if options.frame != FrameCells::TouchedOnly {
        return Err(CompileError::Option(
            "SettlementHashV2 requires FrameCells::TouchedOnly; the target enforces the frame over the complete datum".into(),
        ));
    }
    Ok(())
}

fn too_long(what: &str) -> CompileError {
    CompileError::Bound(format!("ABI v2 {what} exceeds a u16 length"))
}

fn push_u16(out: &mut Vec<u8>, n: usize, what: &str) -> Result<(), CompileError> {
    let n = u16::try_from(n).map_err(|_| too_long(what))?;
    out.extend_from_slice(&n.to_be_bytes());
    Ok(())
}

fn push_lp(out: &mut Vec<u8>, b: &[u8], what: &str) -> Result<(), CompileError> {
    push_u16(out, b.len(), what)?;
    out.extend_from_slice(b);
    Ok(())
}

fn ty_byte(ty: Type) -> u8 {
    match ty {
        Type::UInt => 0,
        Type::Bool => 1,
    }
}

/// Program assets sorted by Core's `AssetId` order (the order the shape and the Cardano
/// catalogue use).
pub fn sorted_assets(program: &Program) -> Vec<AssetId> {
    let mut ids: Vec<AssetId> = program.assets.iter().map(|a| a.identity.clone()).collect();
    ids.sort();
    ids.dedup();
    ids
}

/// `MRG16-PVC-SHP-02` bytes (ABI-V2.md section 4) of a planned action.
pub fn shape_bytes(program: &Program, plan: &ActionPlan) -> Result<Vec<u8>, CompileError> {
    let assets = sorted_assets(program);
    let asset_index = |id: &AssetId| -> Result<usize, CompileError> {
        assets.iter().position(|a| a == id).ok_or_else(|| CompileError::Bound("plan asset outside the program".into()))
    };
    let mut out = SHAPE_TAG.to_vec();
    push_u16(&mut out, assets.len(), "asset list")?;
    for a in &assets {
        push_lp(&mut out, a.network.as_bytes(), "asset network")?;
        push_lp(&mut out, a.issuer.as_bytes(), "asset issuer")?;
        push_lp(&mut out, a.reference.as_bytes(), "asset reference")?;
        out.push(a.scale);
    }
    push_u16(&mut out, plan.states.len(), "state list")?;
    for s in &plan.states {
        push_lp(&mut out, s.name.as_bytes(), "state name")?;
        out.push(ty_byte(s.ty));
    }
    push_u16(&mut out, plan.inputs.len(), "input list")?;
    for s in &plan.inputs {
        push_lp(&mut out, s.name.as_bytes(), "input name")?;
        out.push(ty_byte(s.ty));
    }
    push_u16(&mut out, plan.cells.len(), "cell list")?;
    for c in &plan.cells {
        push_u16(&mut out, asset_index(&c.asset)?, "asset index")?;
        push_lp(&mut out, c.account.as_bytes(), "account")?;
    }
    push_u16(&mut out, plan.duties.len(), "duty list")?;
    for d in &plan.duties {
        push_lp(&mut out, d.name.as_bytes(), "duty name")?;
        push_u16(&mut out, asset_index(&d.asset)?, "asset index")?;
        push_lp(&mut out, d.debtor.as_bytes(), "debtor")?;
        push_lp(&mut out, d.creditor.as_bytes(), "creditor")?;
    }
    push_u16(&mut out, plan.effects.len(), "effect list")?;
    for e in &plan.effects {
        match e {
            EffectShape::State { name, .. } => {
                out.push(0);
                let k = plan.state_index(name).ok_or_else(|| CompileError::Bound("state effect outside the plan".into()))?;
                push_u16(&mut out, k, "state index")?;
            }
            EffectShape::Transfer { from_cell, to_cell, .. } | EffectShape::Fee { from_cell, to_cell, .. } => {
                out.push(if e.is_fee() { 2 } else { 1 });
                push_u16(&mut out, *from_cell, "cell index")?;
                push_u16(&mut out, *to_cell, "cell index")?;
            }
            EffectShape::Liability { duty, origin, .. } => {
                out.push(3);
                push_u16(&mut out, *duty, "duty index")?;
                out.push(match origin {
                    DutyOrigin::Accrue => 0,
                    DutyOrigin::Discharge => 1,
                });
            }
            EffectShape::Emit { .. } => {
                return Err(CompileError::Unsupported {
                    construct: "Statement::Emit",
                    reason: "SettlementHashV2 applies the initial ledger target Emit refusal".into(),
                });
            }
        }
    }
    push_u16(&mut out, plan.debit_cells.len(), "debit cells")?;
    for c in &plan.debit_cells {
        push_u16(&mut out, *c, "cell index")?;
    }
    out.extend_from_slice(&plan.statement_count.to_be_bytes());
    let action = &program.actions[usize::from(plan.action_index)];
    let liability_statements = action.statements.iter().filter(|s| matches!(s, Statement::Liability { .. })).count();
    push_u16(&mut out, liability_statements, "liability statements")?;
    push_lp(&mut out, plan.action.as_bytes(), "action name")?;
    out.extend_from_slice(&plan.action_index.to_be_bytes());
    Ok(out)
}

/// Body length in bytes of a layout (every non-header slot).
pub fn body_len(layout: &StatementLayout) -> Result<usize, CompileError> {
    let mut n = 0usize;
    for slot in &layout.slots {
        n += match (slot.kind.is_header(), slot.encoding) {
            (true, _) => 0,
            (false, SlotEncoding::UInt) => 16,
            (false, SlotEncoding::Bool) => 1,
            (false, SlotEncoding::Wide) => 18,
            (false, other) => return Err(CompileError::Bound(format!("unexpected body slot encoding {other:?}"))),
        };
    }
    Ok(n)
}

pub(crate) fn constants(program: &Program, plan: &ActionPlan, layout: &StatementLayout, predicate: [u8; 32]) -> Result<AbiV2Constants, CompileError> {
    let shape_bytes = shape_bytes(program, plan)?;
    let shape_digest = blake2b::blake2b256(&shape_bytes);
    let body_len = u32::try_from(body_len(layout)?).map_err(|_| CompileError::Bound("ABI v2 body exceeds u32".into()))?;
    Ok(AbiV2Constants { predicate_digest: predicate, shape_bytes, shape_digest, body_len })
}

fn value_bytes(v: &Value) -> Vec<u8> {
    match v {
        Value::UInt(UInt(n)) => n.to_be_bytes().to_vec(),
        Value::Bool(b) => vec![u8::from(*b)],
    }
}

fn wide_bytes(w: Wide) -> Vec<u8> {
    let mut out = w.top.to_be_bytes().to_vec();
    out.extend_from_slice(&w.lo.to_be_bytes());
    out
}

/// Host encoding of the statement body for a claimed transition (honest proposal only;
/// the circuit binds every byte to its slot wire).
pub fn body_bytes(layout: &StatementLayout, plan: &ActionPlan, values: &TransitionValues) -> Result<Vec<u8>, MappingError> {
    let summary = accounting_summary(plan, values);
    let row = |cell: usize| summary.iter().find(|(c, _, _)| *c == cell).copied().ok_or_else(|| MappingError::Shape(format!("no accounting row for cell {cell}")));
    let mut out = Vec::new();
    for slot in &layout.slots {
        let bytes: Vec<u8> = match slot.kind {
            SlotKind::Header(_) => continue,
            SlotKind::Input { index } => value_bytes(&values.inputs[index]),
            SlotKind::PreState { index } => value_bytes(&values.pre.states[index]),
            SlotKind::PostState { index } => value_bytes(&values.post.states[index]),
            SlotKind::PreCell { index } => values.pre.cells[index].0.to_be_bytes().to_vec(),
            SlotKind::PostCell { index } => values.post.cells[index].0.to_be_bytes().to_vec(),
            SlotKind::PreDutyPresent { index } => vec![u8::from(values.pre.duties[index].present)],
            SlotKind::PreDutyAmount { index } => values.pre.duties[index].amount.0.to_be_bytes().to_vec(),
            SlotKind::PostDutyPresent { index } => vec![u8::from(values.post.duties[index].present)],
            SlotKind::PostDutyAmount { index } => values.post.duties[index].amount.0.to_be_bytes().to_vec(),
            SlotKind::Effect { index, role } => {
                use crate::statement::EffectRole as R;
                match (&values.effects[index], role) {
                    (EffectValues::State { before, .. }, R::StateBefore) => value_bytes(before),
                    (EffectValues::State { after, .. }, R::StateAfter) => value_bytes(after),
                    (EffectValues::Amount(a), R::Amount) => a.0.to_be_bytes().to_vec(),
                    (EffectValues::Duty { before, .. }, R::DutyBefore) => before.0.to_be_bytes().to_vec(),
                    (EffectValues::Duty { after, .. }, R::DutyAfter) => after.0.to_be_bytes().to_vec(),
                    _ => return Err(MappingError::Shape(format!("effect {index} has no role {role:?}"))),
                }
            }
            SlotKind::GrossDebit { cell } => wide_bytes(row(cell)?.1),
            SlotKind::FeesPaid { cell } => wide_bytes(row(cell)?.2),
        };
        out.extend(bytes);
    }
    Ok(out)
}

/// Complete statement bytes.
pub fn statement_bytes(consts: &AbiV2Constants, ctx: &ContextV2, body: &[u8]) -> Result<Vec<u8>, MappingError> {
    if body.len() != consts.body_len as usize {
        return Err(MappingError::Shape(format!("body has {} bytes, the relation fixes {}", body.len(), consts.body_len)));
    }
    let mut s = Vec::with_capacity(HEADER_LEN + body.len());
    s.extend_from_slice(STMT_TAG);
    s.extend_from_slice(FAMILY_V2);
    s.extend_from_slice(&consts.predicate_digest);
    s.extend_from_slice(&consts.shape_digest);
    s.extend(ctx.bytes());
    s.extend_from_slice(&consts.body_len.to_be_bytes());
    s.extend_from_slice(body);
    debug_assert_eq!(s.len(), HEADER_LEN + body.len());
    Ok(s)
}

/// The two public inputs `[int_be(d[0..16]), int_be(d[16..32])]`, d = BLAKE2b-256(stmt).
pub fn public_inputs<F: PrimeField>(statement: &[u8]) -> Vec<F> {
    let d = blake2b::blake2b256(statement);
    let mut hi = [0u8; 16];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&d[..16]);
    lo.copy_from_slice(&d[16..]);
    vec![fe(u128::from_be_bytes(hi)), fe(u128::from_be_bytes(lo))]
}

// ## In-circuit binding

fn const_bytes<F: PrimeField>(bytes: &[u8], bits: &mut Vec<Bit<F>>) {
    for b in bytes {
        for j in 0..8 {
            bits.push(Bit::constant((b >> j) & 1 == 1));
        }
    }
}

/// Free witness bytes (context): booleanized bits, honest values from `honest`.
fn free_bytes<F: PrimeField>(cs: &mut Cs<F>, honest: Option<&[u8]>, n: usize, bits: &mut Vec<Bit<F>>) -> Result<(), BindError> {
    for k in 0..n {
        for j in 0..8 {
            let val = honest.map(|h| (h[k] >> j) & 1 == 1);
            bits.push(blake2b::alloc_bit(cs, "abi2.ctx", val)?);
        }
    }
    Ok(())
}

/// Bytes of a limb (big-endian, `n` bytes) bound to its linear combination: booleanized
/// bits whose weighted sum equals the limb. A constant limb yields constant bits.
fn limb_bytes<F: PrimeField>(
    cs: &mut Cs<F>,
    limb: &Lc<F>,
    konst: Option<u128>,
    n: usize,
    honest: Option<&[u8]>,
    bits: &mut Vec<Bit<F>>,
) -> Result<(), BindError> {
    if let Some(k) = konst {
        let be = k.to_be_bytes();
        const_bytes(&be[16 - n..], bits);
        return Ok(());
    }
    let mut sum = Lc::zero();
    for k in 0..n {
        for j in 0..8 {
            let val = honest.map(|h| (h[k] >> j) & 1 == 1);
            let bit = blake2b::alloc_bit(cs, "abi2.body", val)?;
            let weight = 8 * (n - 1 - k) + j;
            sum = sum.plus(&bit.lc.scaled(fe(1u128 << weight)));
            bits.push(bit);
        }
    }
    cs.enforce_eq(&sum, limb)?;
    Ok(())
}

fn uint_bytes<F: PrimeField>(cs: &mut Cs<F>, u: &UIntW<F>, honest: Option<&[u8]>, bits: &mut Vec<Bit<F>>) -> Result<(), BindError> {
    let (khi, klo) = match u.konst {
        Some(k) => (Some(k >> 64), Some(k & u128::from(u64::MAX))),
        None => (None, None),
    };
    limb_bytes(cs, &u.hi, khi, 8, honest.map(|h| &h[0..8]), bits)?;
    limb_bytes(cs, &u.lo, klo, 8, honest.map(|h| &h[8..16]), bits)
}

fn wide_bytes_in<F: PrimeField>(cs: &mut Cs<F>, w: &WideW<F>, honest: Option<&[u8]>, bits: &mut Vec<Bit<F>>) -> Result<(), BindError> {
    limb_bytes(cs, &w.top, None, 2, honest.map(|h| &h[0..2]), bits)?;
    limb_bytes(cs, &w.mid, None, 8, honest.map(|h| &h[2..10]), bits)?;
    limb_bytes(cs, &w.lo, None, 8, honest.map(|h| &h[10..18]), bits)
}

/// Binds the statement: constant header bytes, free context bytes, every body byte tied
/// to its slot wire, BLAKE2b-256 in-circuit, and the two public inputs equal to the
/// digest halves. `honest` is the host statement (prove mode) or `None` (setup).
pub(crate) fn bind_v2<F: PrimeField>(
    cs: &mut Cs<F>,
    consts: &AbiV2Constants,
    layout: &StatementLayout,
    wires: &[SlotWire<F>],
    honest: Option<&[u8]>,
    claimed: Option<&[F]>,
) -> Result<(), BindError> {
    if wires.len() != layout.slots.len() {
        return Err(BindError::Adapter(AdapterError::Shape("wire count differs from slot count".into())));
    }
    let total = HEADER_LEN + consts.body_len as usize;
    if honest.is_some_and(|h| h.len() != total) {
        return Err(BindError::Adapter(AdapterError::Shape("honest statement length differs from the relation".into())));
    }
    if claimed.is_some_and(|c| c.len() != 2) {
        return Err(BindError::Adapter(AdapterError::Shape("SettlementHashV2 has exactly two public inputs".into())));
    }
    let start = cs.num_constraints();
    cs.push("abi2".into());
    let mut bits: Vec<Bit<F>> = Vec::with_capacity(8 * total);
    const_bytes(STMT_TAG, &mut bits);
    const_bytes(FAMILY_V2, &mut bits);
    const_bytes(&consts.predicate_digest, &mut bits);
    const_bytes(&consts.shape_digest, &mut bits);
    free_bytes(cs, honest.map(|h| &h[112..339]), 339 - 112, &mut bits)?;
    const_bytes(&consts.body_len.to_be_bytes(), &mut bits);
    let mut at = HEADER_LEN;
    for (slot, wire) in layout.slots.iter().zip(wires) {
        if slot.kind.is_header() {
            continue;
        }
        let width = match slot.encoding {
            SlotEncoding::UInt => 16,
            SlotEncoding::Bool => 1,
            SlotEncoding::Wide => 18,
            _ => return Err(BindError::Adapter(AdapterError::Shape("unexpected body slot encoding".into()))),
        };
        let h = honest.map(|h| &h[at..at + width]);
        match (wire, slot.encoding) {
            (SlotWire::U(u), SlotEncoding::UInt) => uint_bytes(cs, u, h, &mut bits)?,
            (SlotWire::B(b), SlotEncoding::Bool) => {
                // The Bool wire is already 0 or 1; its byte is (b, 0, ..., 0) LSB first.
                bits.push(Bit { lc: b.lc.clone(), val: b.val, konst: b.konst });
                for _ in 1..8 {
                    bits.push(Bit::constant(false));
                }
            }
            (SlotWire::W(w), SlotEncoding::Wide) => wide_bytes_in(cs, w, h, &mut bits)?,
            _ => return Err(BindError::Adapter(AdapterError::Shape(format!("slot {} wire does not match its encoding", slot.label)))),
        }
        at += width;
    }
    if bits.len() != 8 * total {
        return Err(BindError::Adapter(AdapterError::Shape("statement bit count differs from the relation".into())));
    }
    let digest = blake2b::blake2b256_gadget(cs, &bits)?;
    for (k, from) in [0usize, 16].into_iter().enumerate() {
        let x = cs.input(claimed.map(|c| c[k]))?;
        cs.enforce_eq(&Lc::var(x), &blake2b::digest_half_lc(&digest, from))?;
    }
    cs.pop();
    cs.record("abi2-binding".into(), start);
    Ok(())
}
