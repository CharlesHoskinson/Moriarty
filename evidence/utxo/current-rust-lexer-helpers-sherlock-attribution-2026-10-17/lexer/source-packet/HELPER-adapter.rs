//! UNADOPTED public-input / binding adapter boundary.
//!
//! The compiler produces a typed statement ([`StatementLayout`] + [`TransitionValues`])
//! and slot wires. How those become the public inputs of a proof is this boundary.
//! This crate does **not** define an on-chain claim ABI:
//!
//! - [`StatementAdapter::DirectFieldV0`] makes every statement field one public input.
//!   It exists for constraint-level tests. It binds no claimant, no settle outref and
//!   no ledger context, so a proof under it is re-wrappable by anyone (X7 review R-1);
//!   it must never be used as a claim ABI.
//! - [`StatementAdapter::ClaimantBoundAbiV1Pending`] names the separately frozen
//!   claimant-bound ABI v1 (`claim-utxo-abi-v1-source-20261015`, SOURCE-MANIFEST-V1
//!   `96e85df9…`, `MRG16-PVC-STMT-1`, public inputs = two big-endian 128-bit halves of
//!   BLAKE2b-256(statement)). That ABI is unreviewed and unadopted, so every use of
//!   this variant refuses. [`ABI_V1_MAPPING_REQUIREMENTS`] lists what its mapping must
//!   do once its own source review completes.
//!
//! Target obligations that stay outside the transition relation, whichever adapter
//! maps it, are listed in [`TARGET_OBLIGATIONS`].
use crate::error::MappingError;
use crate::gadgets::{BoolW, UIntW, WideW};
use crate::plan::ActionPlan;
use crate::r1cs::{Cs, Lc};
use crate::statement::{SlotEncoding, StatementLayout, TransitionValues};
use ark_ff::PrimeField;
use ark_relations::r1cs::SynthesisError;
use mori_core::{Program, Statement};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum StatementAdapter {
    /// TEST-ONLY direct field statement. Not a claim ABI (see module docs).
    DirectFieldV0,
    /// Placeholder for the frozen, unreviewed claimant-bound ABI v1. Always refuses.
    ClaimantBoundAbiV1Pending,
    /// P3S1 PROPOSED (unreviewed, unadopted): MRG16-PVC ABI v2. Two public inputs, the
    /// big-endian halves of an in-circuit BLAKE2b-256 of the ABI v2 statement (constant
    /// header, free context bytes, every body slot bound to its wire). See [`crate::abi2`]
    /// and `candidate/docs/ABI-V2.md`. Compile options are fixed by
    /// [`crate::abi2::options_ok`]; whole-program Emit is refused by
    /// [`initial_ledger_target_admits`], which this variant calls (review F-10 wiring for
    /// this adapter only). Proving needs a [`crate::abi2::ContextV2`]
    /// ([`crate::CompiledRelation::assignment_v2`], [`crate::CompiledRelation::circuit_v2`]).
    SettlementHashV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    /// The adapter names an interface that is not adopted.
    Unadopted(&'static str),
    /// A target policy refuses the program (for example whole-program Emit).
    TargetRefusal(String),
    /// Slot wires do not match the layout (internal invariant).
    Shape(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unadopted(m) => write!(f, "unadopted interface: {m}"),
            Self::TargetRefusal(m) => write!(f, "target refusal: {m}"),
            Self::Shape(m) => write!(f, "shape: {m}"),
        }
    }
}

impl std::error::Error for AdapterError {}

const ABI_V1_PENDING: &str = "claimant-bound ABI v1 (MRG16-PVC-STMT-1) is frozen but unreviewed and unadopted; \
     no mapping from this relation's statement into it is defined";

const V2_NEEDS_CONTEXT: &str = "SettlementHashV2 public inputs depend on a ContextV2; use CompiledRelation::public_inputs_v2";

const V2_BINDS_IN_CIRCUIT: &str = "SettlementHashV2 is bound by abi2::bind_v2 from CompiledRelation::circuit_v2";

/// What the ABI v1 mapping must establish after that ABI's own source review. None of
/// it is implemented here; listing it does not adopt it.
pub const ABI_V1_MAPPING_REQUIREMENTS: &[&str] = &[
    "In-circuit statement bytes: tag MRG16-PVC-STMT-1, family_id_v1, predicate_digest bound to this relation's RelationIdentity digest (catalogue row), vk_digest and claimant_vkh (28 bytes) as free witnesses at their ABI offsets, settle outref for context-bound rows.",
    "An in-circuit BLAKE2b-256 of those bytes bound to the two public 128-bit halves; no other public input.",
    "A transition section that carries every slot of StatementLayout byte-exactly: MSB0/MEB0 cannot (they lack Liability asset/debtor/creditor and present-zero rows, ABI-V1 VOTES.md P3-8), so a new reviewed snapshot/effect encoding is required.",
    "Program hash, profile digest, shape digest, compiler build digest and W as constants of the row, never prover-chosen.",
    "Every UInt byte image range-checked to 16 big-endian bytes; Bool bytes in {0,1}; no modular alias.",
    "Whole-program Emit refusal for the initial ledger target (initial_ledger_target_admits), with Emit retained as a required future track.",
    "Fresh setup for every relation; no VK, PK or claim from X7 v0 or attempt-2 is reused (orphan Class 0 VKs cannot be a real genesis).",
];

/// Obligations a pure transition proof does not discharge, whatever the adapter.
pub const TARGET_OBLIGATIONS: &[&str] = &[
    "Pre-state authenticity: pre states, cells and duties are statement inputs; a target must bind them to the consumed datum/UTxO (ScriptCustodied) or label them unauthenticated premises (KeyHeldExternal, projection.rs:415-430).",
    "Balance-row existence (premise, review F-04): Core refuses a scenario that lacks an explicit balance row for a touched account (runtime.rs:609-627), but the layout has no balance presence bit, so a claimed pre-balance of 0 for a missing row satisfies the relation. A target must authenticate the existence of every represented balance row, or a reviewed presence bit must be added; until then this is an unauthenticated premise, not a relation constant.",
    "Frame completeness: rows outside the represented cells are not in the statement; a target must represent them (FrameCells::Include) or establish that none exist, because Core's 65,536 trace bound counts every scenario row (runtime.rs:540-547). FrameCells::Include covers balance cells only: an unregistered incoming duty (no source Liability) cannot be represented at all (review F-05), so duty-frame completeness rests entirely on the target until a reviewed duty-frame option exists. No P3 or ABI mapping may claim a complete frame before then.",
    "Physical asset delta: Transfer/Fee effects are bookkeeping; payouts, deposits, custody totals, conservation and tagged outputs are separate target relations (ABI-V1 VOTES.md P3).",
    "Debit authority: authority for debited accounts and owner intent limits (intent::check_effects) are separate checks; the accounting summary only exposes gross debit and fees. The P3-7 scope is disputed and unreconciled: the later ABI review (f3ae8556, V6) requires authority for every debited account including custody-held balances, thread-advance authority and a signed intent in the statement, not only external (KeyHeldExternal) signers.",
    "Replay and consumption: one-time use of the consumed thread/claim, nonce and predecessor history.",
    "Ledger phases and failure outcomes: rejected executions have no success proof here; any charged failure transition needs its own relation.",
    "Setup provenance: the R1CS digest names the matrices; a VK is trusted only with a reviewed setup transcript over exactly those matrices.",
];

/// P3S1 PROPOSED: how the ABI v2 Cardano target (S2, `candidate/aiken/lib/pvc2/thread.ak`)
/// takes over each [`TARGET_OBLIGATIONS`] entry for [`StatementAdapter::SettlementHashV2`].
/// Source-level mapping only; nothing here is reviewed, compiled or executed.
pub const SETTLEMENT_V2_TARGET_DISCHARGE: &[(&str, &str)] = &[
    ("Pre-state authenticity", "S2 builds pre states, custody cells and duties from the spent thread datum; external cells use the canonical premise (gross debit in the step)."),
    ("Balance-row existence (F-04)", "S2 refuses a represented custody cell without a datum row; external cells have no row by design."),
    ("Frame completeness (F-05)", "The datum is the complete snapshot; S2 copies every unrepresented custody row and liability (registered or not) into the continuation and checks Core's scenario bounds on the whole snapshot."),
    ("Physical asset delta", "S2 checks exact custody value before and after, tagged or pinned-datum payouts per external credit, global output allocation, reserve top-ups and no own-policy mint."),
    ("Debit authority", "S2 requires a verified Core owner intent from every debited account (custody included), every accrual debtor, every discharge creditor and one action advancer; Core check_effects runs on the bound effects and the complete closing duties."),
    ("Replay and consumption", "The intent binds the consumed outref and full pre-snapshot; the claim names the outref; the ledger consumes it once; seq and the event chain advance."),
    ("Ledger phases and failure outcomes", "Cardano transactions are atomic; phase-2 failure consumes only key-locked collateral; a refusal leaves the datum (and every duty) unchanged. No charged failure transition is modelled."),
    ("Setup provenance", "Unchanged: the catalogue pins (predicate, vk) per action; VK provenance needs a reviewed setup transcript."),
];

/// A typed slot wire handed to the adapter, in layout order.
pub(crate) enum SlotWire<F: PrimeField> {
    Const(Vec<F>),
    U(UIntW<F>),
    B(BoolW<F>),
    W(WideW<F>),
}

impl<F: PrimeField> SlotWire<F> {
    /// Field-limb linear combinations in encoding order. Every UInt/Wide limb is a bit
    /// sum or a constant, so equating a public input with it forces the limb range.
    fn limbs(&self, encoding: SlotEncoding) -> Result<Vec<Lc<F>>, AdapterError> {
        let lcs = match (self, encoding) {
            (Self::Const(fields), SlotEncoding::Small | SlotEncoding::Digest | SlotEncoding::UInt) => {
                fields.iter().map(|f| Lc::constant(*f)).collect()
            }
            (Self::U(u), SlotEncoding::UInt) => vec![u.lo.clone(), u.hi.clone()],
            (Self::B(b), SlotEncoding::Bool) => vec![b.lc.clone()],
            (Self::W(w), SlotEncoding::Wide) => vec![w.lo.clone(), w.mid.clone(), w.top.clone()],
            _ => return Err(AdapterError::Shape(format!("slot wire does not match encoding {encoding:?}"))),
        };
        if lcs.len() != encoding.width() {
            return Err(AdapterError::Shape(format!("slot wire has {} limbs, encoding {encoding:?} needs {}", lcs.len(), encoding.width())));
        }
        Ok(lcs)
    }
}

pub(crate) enum BindError {
    Synthesis(SynthesisError),
    Adapter(AdapterError),
}

impl From<SynthesisError> for BindError {
    fn from(e: SynthesisError) -> Self {
        Self::Synthesis(e)
    }
}

impl StatementAdapter {
    /// Adapter-level admission of a program. The direct test adapter admits every
    /// program the compiler lowers (Emit included); the pending ABI admits none.
    pub fn admits(&self, program: &Program) -> Result<(), AdapterError> {
        match self {
            Self::DirectFieldV0 => Ok(()),
            Self::ClaimantBoundAbiV1Pending => Err(AdapterError::Unadopted(ABI_V1_PENDING)),
            Self::SettlementHashV2 => initial_ledger_target_admits(program),
        }
    }

    /// Number of public inputs a verifier supplies.
    pub fn public_input_count(&self, layout: &StatementLayout) -> Result<usize, AdapterError> {
        match self {
            Self::DirectFieldV0 => Ok(layout.field_count),
            Self::ClaimantBoundAbiV1Pending => Err(AdapterError::Unadopted(ABI_V1_PENDING)),
            Self::SettlementHashV2 => Ok(2),
        }
    }

    /// The public inputs of a claimed transition, in instance order.
    pub fn public_inputs<F: PrimeField>(&self, layout: &StatementLayout, plan: &ActionPlan, values: &TransitionValues) -> Result<Vec<F>, MappingError> {
        match self {
            Self::DirectFieldV0 => layout.encode(plan, values),
            Self::ClaimantBoundAbiV1Pending => Err(MappingError::Adapter(AdapterError::Unadopted(ABI_V1_PENDING))),
            Self::SettlementHashV2 => Err(MappingError::Adapter(AdapterError::Unadopted(V2_NEEDS_CONTEXT))),
        }
    }

    /// Binds every slot wire to the public statement. Direct adapter: one public input
    /// per field, allocated in slot order and equated with the wire limb.
    pub(crate) fn bind<F: PrimeField>(
        &self,
        cs: &mut Cs<F>,
        layout: &StatementLayout,
        wires: &[SlotWire<F>],
        claimed: Option<&[F]>,
    ) -> Result<(), BindError> {
        match self {
            Self::DirectFieldV0 => {
                if wires.len() != layout.slots.len() {
                    return Err(BindError::Adapter(AdapterError::Shape("wire count differs from slot count".into())));
                }
                if claimed.is_some_and(|c| c.len() != layout.field_count) {
                    return Err(BindError::Adapter(AdapterError::Shape("claimed public vector length differs from the layout".into())));
                }
                let start = cs.num_constraints();
                cs.push("bind".into());
                for (slot, wire) in layout.slots.iter().zip(wires) {
                    let limbs = wire.limbs(slot.encoding).map_err(BindError::Adapter)?;
                    for (k, limb) in limbs.iter().enumerate() {
                        let x = cs.input(claimed.map(|c| c[slot.offset + k]))?;
                        cs.enforce_eq(&Lc::var(x), limb)?;
                    }
                }
                cs.pop();
                cs.record("binding".into(), start);
                Ok(())
            }
            Self::ClaimantBoundAbiV1Pending => Err(BindError::Adapter(AdapterError::Unadopted(ABI_V1_PENDING))),
            Self::SettlementHashV2 => Err(BindError::Adapter(AdapterError::Unadopted(V2_BINDS_IN_CIRCUIT))),
        }
    }
}

/// Initial ledger target Emit policy (two adopted votes): a program containing any
/// Emit in any action is refused as a whole by the initial ledger target. This is a
/// target policy, not a compiler restriction: the relation itself lowers Emit and the
/// current staged UNIT codec admits it (`staged_projection.rs:192-232`); full Emit
/// remains a required future track.
///
/// Called by [`StatementAdapter::admits`] for [`StatementAdapter::SettlementHashV2`]
/// (P3S1 wiring of review F-10 for that adapter; the wiring itself is unreviewed).
/// [`StatementAdapter::DirectFieldV0`] still admits Emit (test-only adapter).
pub fn initial_ledger_target_admits(program: &Program) -> Result<(), AdapterError> {
    for action in &program.actions {
        for (ordinal, statement) in action.statements.iter().enumerate() {
            if let Statement::Emit { name, .. } = statement {
                return Err(AdapterError::TargetRefusal(format!(
                    "initial ledger target refuses whole-program Emit: action {:?} statement {ordinal} emits {name:?}",
                    action.name
                )));
            }
        }
    }
    Ok(())
}
