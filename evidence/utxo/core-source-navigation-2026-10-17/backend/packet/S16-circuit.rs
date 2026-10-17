//! Compilation entry points, the compile-time probe and constraint synthesis.
//!
//! `compile` admits nothing by itself: it takes an [`AdmittedCore1`] (actual Core
//! parse/validate), builds the plan and statement layout, and runs one setup-mode
//! synthesis (the probe) to refuse static failures and to record constraint counts and
//! the R1CS matrix digest. Those numbers exist only after an actual run; this source
//! records none.
//!
//! Synthesis order: entry wires (claimed inputs and pre-state, range-proven), the
//! statement-by-statement lowering, the cap constraint, the accounting summary, then
//! binding of every slot through the adapter. Internal wires are computed from the
//! claimed inputs and pre-state only; claimed post-state and effects enter solely as
//! public values that the binding equates with the computed wires.
use crate::abi2::{self, AbiV2Constants, ContextV2};
use crate::adapter::{AdapterError, BindError, SlotWire, StatementAdapter};
use crate::error::{CompileError, MappingError};
use crate::frontend::{self, Admitted, AdmittedCore1};
use crate::gadgets::{self, BoolW, CVal, GadgetError, UIntW, WideW};
use crate::identity::{self, Digest32, RelationIdentity};
use crate::lower::{self, DutyW, EffectW, Entry, LowerError};
use crate::plan::{self, AccountingBinding, ActionPlan, CapBinding, CompileOptions};
use crate::r1cs::{Cs, Lc, RegionCount, Tamper};
use crate::statement::{BodyValues, EffectRole, HeaderField, SlotKind, StatementLayout, TransitionValues};
use ark_ff::{BigInteger, PrimeField};
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, OptimizationGoal, SynthesisError, SynthesisMode,
};
use mori_core::{Program, Type, UInt, Value};
use serde::Serialize;

/// Facts recorded by the compile-time probe synthesis. Only an actual run fills them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeReport {
    pub field_modulus_bits: u32,
    /// Little-endian hex of the scalar modulus the relation was probed in.
    pub field_modulus_le_hex: String,
    pub constraints: usize,
    /// Includes arkworks' constant-one instance variable.
    pub instance_variables: usize,
    pub witness_variables: usize,
    pub regions: Vec<RegionCount>,
    /// SHA-256 of the finalized A/B/C matrices (format `mori-r1cs-matrices/1`), under
    /// `OptimizationGoal::Constraints` as ark-groth16 0.4 setup uses (UNVERIFIED-LIB).
    pub r1cs_sha256: Digest32,
}

/// Facts from one synthesis (tests read `overrides_used`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisReport {
    pub regions: Vec<RegionCount>,
    pub overrides_used: usize,
    pub constraints: usize,
}

/// One compiled relation: an admitted Program, one action, explicit options.
#[derive(Debug, Clone)]
pub struct CompiledRelation {
    admitted: AdmittedCore1,
    options: CompileOptions,
    plan: ActionPlan,
    layout: StatementLayout,
    identity: RelationIdentity,
    adapter: StatementAdapter,
    /// ABI v2 constants (P3S1); present exactly for [`StatementAdapter::SettlementHashV2`].
    abi2: Option<AbiV2Constants>,
    probe: ProbeReport,
}

/// A claimed transition and its exact public encoding.
#[derive(Debug, Clone)]
pub struct Assignment<F: PrimeField> {
    pub values: TransitionValues,
    /// Public inputs in instance order. Tests may mutate it to play a hostile claimant.
    pub public: Vec<F>,
}

impl CompiledRelation {
    pub fn program(&self) -> &Program {
        self.admitted.program()
    }
    pub fn admitted(&self) -> &AdmittedCore1 {
        &self.admitted
    }
    pub fn options(&self) -> &CompileOptions {
        &self.options
    }
    pub fn plan(&self) -> &ActionPlan {
        &self.plan
    }
    pub fn layout(&self) -> &StatementLayout {
        &self.layout
    }
    pub fn identity(&self) -> &RelationIdentity {
        &self.identity
    }
    pub fn adapter(&self) -> StatementAdapter {
        self.adapter
    }
    pub fn probe(&self) -> &ProbeReport {
        &self.probe
    }
    pub fn abi2(&self) -> Option<&AbiV2Constants> {
        self.abi2.as_ref()
    }

    /// P3S1: the ABI v2 statement of a claimed transition in a context. Checks shape and
    /// types only; acceptance is the constraint system's.
    pub fn statement_v2(&self, values: &TransitionValues, context: &ContextV2) -> Result<Vec<u8>, MappingError> {
        let consts = self
            .abi2
            .as_ref()
            .ok_or(MappingError::Adapter(AdapterError::Unadopted("statement_v2 needs the SettlementHashV2 adapter")))?;
        crate::statement::check_values(&self.plan, values)?;
        let body = abi2::body_bytes(&self.layout, &self.plan, values)?;
        abi2::statement_bytes(consts, context, &body)
    }

    /// P3S1: the two ABI v2 public inputs of a claimed transition in a context.
    pub fn public_inputs_v2<F: PrimeField>(&self, values: &TransitionValues, context: &ContextV2) -> Result<Vec<F>, MappingError> {
        self.check_field::<F>().map_err(MappingError::Field)?;
        Ok(abi2::public_inputs::<F>(&self.statement_v2(values, context)?))
    }

    /// P3S1: an ABI v2 assignment (public = the two digest halves).
    pub fn assignment_v2<F: PrimeField>(&self, values: TransitionValues, context: &ContextV2) -> Result<Assignment<F>, MappingError> {
        let public = self.public_inputs_v2::<F>(&values, context)?;
        Ok(Assignment { values, public })
    }

    /// P3S1: an ABI v2 circuit for proving (assignment and context) or setup (neither).
    pub fn circuit_v2<F: PrimeField>(
        &self,
        assignment: Option<Assignment<F>>,
        context: Option<ContextV2>,
        tamper: Tamper<F>,
    ) -> Result<TransitionCircuit<'_, F>, CompileError> {
        self.check_field::<F>().map_err(CompileError::Field)?;
        if self.adapter != StatementAdapter::SettlementHashV2 {
            return Err(CompileError::Option("circuit_v2 needs the SettlementHashV2 adapter".into()));
        }
        if assignment.is_some() != context.is_some() {
            return Err(CompileError::Option("SettlementHashV2 proving needs both an assignment and a ContextV2".into()));
        }
        if let Some(a) = &assignment {
            self.check_assignment_lengths(a)?;
        }
        Ok(TransitionCircuit { relation: self, assignment, context, tamper })
    }

    /// The public inputs a verifier supplies for `values`.
    pub fn public_inputs<F: PrimeField>(&self, values: &TransitionValues) -> Result<Vec<F>, MappingError> {
        self.check_field::<F>().map_err(MappingError::Field)?;
        self.adapter.public_inputs(&self.layout, &self.plan, values)
    }

    /// Encodes a claimed transition. This checks shape and types only; acceptance is
    /// the constraint system's.
    pub fn assignment<F: PrimeField>(&self, values: TransitionValues) -> Result<Assignment<F>, MappingError> {
        let public = self.public_inputs::<F>(&values)?;
        Ok(Assignment { values, public })
    }

    /// A circuit for proving (`Some`) or setup (`None`).
    pub fn circuit<F: PrimeField>(&self, assignment: Option<Assignment<F>>, tamper: Tamper<F>) -> Result<TransitionCircuit<'_, F>, CompileError> {
        self.check_field::<F>().map_err(CompileError::Field)?;
        if self.adapter == StatementAdapter::SettlementHashV2 && assignment.is_some() {
            return Err(CompileError::Option("SettlementHashV2 proving needs circuit_v2 with a ContextV2".into()));
        }
        if let Some(a) = &assignment {
            self.check_assignment_lengths(a)?;
        }
        Ok(TransitionCircuit { relation: self, assignment, context: None, tamper })
    }

    /// Refuses an assignment whose vectors do not have this relation's lengths (review
    /// F-12). `Assignment` fields are public, so a hand-built value could otherwise make
    /// synthesis index out of bounds and panic. Lengths only: value types, absent-duty
    /// canonicality and every value relation stay with the constraint system, so this
    /// adds no constraint and decides no well-shaped claim.
    fn check_assignment_lengths<F: PrimeField>(&self, a: &Assignment<F>) -> Result<(), CompileError> {
        let plan = &self.plan;
        let body = |b: &BodyValues| -> bool {
            b.states.len() == plan.states.len() && b.cells.len() == plan.cells.len() && b.duties.len() == plan.duties.len()
        };
        let checks = [
            ("inputs", a.values.inputs.len() == plan.inputs.len()),
            ("pre body", body(&a.values.pre)),
            ("post body", body(&a.values.post)),
            ("effects", a.values.effects.len() == plan.effects.len()),
            ("public inputs", self.adapter.public_input_count(&self.layout).is_ok_and(|n| a.public.len() == n)),
        ];
        match checks.iter().find(|(_, ok)| !*ok) {
            Some((what, _)) => Err(CompileError::AssignmentShape(format!("{what} length differs from the relation"))),
            None => Ok(()),
        }
    }

    fn check_field<F: PrimeField>(&self) -> Result<(), String> {
        let modulus = hex_le(&F::MODULUS.to_bytes_le());
        if modulus != self.probe.field_modulus_le_hex {
            return Err("circuit field differs from the field the relation was compiled and probed in".into());
        }
        Ok(())
    }
}

fn hex_le(bytes: &[u8]) -> String {
    mori_core::intent::hex(bytes)
}

/// Compiles one action of an admitted `/1` Program over the scalar field `F`.
pub fn compile<F: PrimeField>(admitted: AdmittedCore1, options: CompileOptions, adapter: StatementAdapter) -> Result<CompiledRelation, CompileError> {
    if F::MODULUS_BIT_SIZE < crate::MIN_MODULUS_BITS {
        return Err(CompileError::Field(format!(
            "scalar field has {} bits; the exact-integer gadgets need at least {}",
            F::MODULUS_BIT_SIZE,
            crate::MIN_MODULUS_BITS
        )));
    }
    let build = options.compiler.checked()?;
    adapter.admits(admitted.program()).map_err(CompileError::Adapter)?;
    if adapter == StatementAdapter::SettlementHashV2 {
        abi2::options_ok(&options)?;
    }
    let plan = plan::plan_action(admitted.program(), &options)?;
    let layout = StatementLayout::build(admitted.identity(), &plan, build)?;
    let program_identity = admitted.identity().clone();
    let identity = RelationIdentity {
        relation_format: crate::RELATION_FORMAT.into(),
        relation_version: crate::RELATION_VERSION,
        language: program_identity.language.clone(),
        semantics: program_identity.semantics.clone(),
        logical_cost_profile: mori_core::LOGICAL_COST_PROFILE.into(),
        profile_digest: layout.header.profile_digest,
        program_hash: program_identity.program_hash,
        source_sha256: program_identity.source_sha256,
        action: plan.action.clone(),
        action_index: plan.action_index,
        shape_digest: layout.header.shape_digest,
        compiler_build: build,
        compiler_build_digest: layout.header.compiler_build_digest,
        adapter,
        logical_work: plan.logical_work,
    };
    let placeholder = ProbeReport {
        field_modulus_bits: F::MODULUS_BIT_SIZE,
        field_modulus_le_hex: hex_le(&F::MODULUS.to_bytes_le()),
        constraints: 0,
        instance_variables: 0,
        witness_variables: 0,
        regions: Vec::new(),
        r1cs_sha256: Digest32([0; 32]),
    };
    let abi2 = if adapter == StatementAdapter::SettlementHashV2 {
        Some(abi2::constants(admitted.program(), &plan, &layout, identity.digest()?.0)?)
    } else {
        None
    };
    let mut relation = CompiledRelation { admitted, options, plan, layout, identity, adapter, abi2, probe: placeholder };
    relation.probe = probe::<F>(&relation)?;
    Ok(relation)
}

/// Routes source by its declared profile and compiles `/1`. `/2` is admitted by the
/// actual stage loader and then refused with its staged lowering plan.
pub fn compile_source<F: PrimeField>(source: &str, options: CompileOptions, adapter: StatementAdapter) -> Result<CompiledRelation, CompileError> {
    match frontend::admit_source(source)? {
        Admitted::Core1(admitted) => compile::<F>(admitted, options, adapter),
        Admitted::Staged2(staged) => Err(CompileError::StagedNotLowered(Box::new(staged.lowering_plan()?))),
    }
}

/// Setup-mode synthesis: refuses static failures and records counts and the matrix
/// digest. The same function synthesizes for proving, so the structure is identical.
fn probe<F: PrimeField>(relation: &CompiledRelation) -> Result<ProbeReport, CompileError> {
    let cs = ConstraintSystem::<F>::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    cs.set_mode(SynthesisMode::Setup);
    let report = synthesize(relation, None, None, Tamper::none(), cs.clone()).map_err(SynthFailure::into_compile_error)?;
    cs.finalize();
    let matrices = cs.to_matrices().ok_or_else(|| CompileError::Synthesis("constraint matrices were not constructed".into()))?;
    let mut bytes = b"mori-r1cs-matrices/1\0".to_vec();
    for n in [matrices.num_instance_variables, matrices.num_witness_variables, matrices.num_constraints] {
        bytes.extend_from_slice(&(n as u64).to_le_bytes());
    }
    for matrix in [&matrices.a, &matrices.b, &matrices.c] {
        bytes.extend_from_slice(&(matrix.len() as u64).to_le_bytes());
        for row in matrix {
            bytes.extend_from_slice(&(row.len() as u64).to_le_bytes());
            for (coefficient, index) in row {
                bytes.extend_from_slice(&(*index as u64).to_le_bytes());
                let le = coefficient.into_bigint().to_bytes_le();
                bytes.extend_from_slice(&(le.len() as u64).to_le_bytes());
                bytes.extend_from_slice(&le);
            }
        }
    }
    Ok(ProbeReport {
        field_modulus_bits: F::MODULUS_BIT_SIZE,
        field_modulus_le_hex: hex_le(&F::MODULUS.to_bytes_le()),
        constraints: matrices.num_constraints,
        instance_variables: matrices.num_instance_variables,
        witness_variables: matrices.num_witness_variables,
        regions: report.regions,
        r1cs_sha256: identity::digest_bytes(&bytes),
    })
}

/// The circuit for one claimed transition (or for setup when `assignment` is `None`).
pub struct TransitionCircuit<'r, F: PrimeField> {
    relation: &'r CompiledRelation,
    assignment: Option<Assignment<F>>,
    /// ABI v2 context (P3S1); `None` for the other adapters and in setup mode.
    context: Option<ContextV2>,
    tamper: Tamper<F>,
}

impl<F: PrimeField> TransitionCircuit<'_, F> {
    /// Synthesizes into `cs` and reports regions and tamper hits.
    pub fn synthesize_into(self, cs: ConstraintSystemRef<F>) -> Result<SynthesisReport, SynthesisError> {
        synthesize(self.relation, self.assignment.as_ref(), self.context.as_ref(), self.tamper, cs).map_err(SynthFailure::into_synthesis_error)
    }
}

impl<F: PrimeField> ConstraintSynthesizer<F> for TransitionCircuit<'_, F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        self.synthesize_into(cs).map(|_| ())
    }
}

enum SynthFailure {
    Synthesis(SynthesisError),
    Static { scope: String, reason: String },
    Shape(String),
    Adapter(AdapterError),
}

impl SynthFailure {
    fn into_compile_error(self) -> CompileError {
        match self {
            Self::Synthesis(e) => CompileError::Synthesis(e.to_string()),
            Self::Static { scope, reason } => CompileError::StaticFailure { scope, reason },
            Self::Shape(m) => CompileError::Synthesis(format!("internal shape invariant: {m}")),
            Self::Adapter(a) => CompileError::Adapter(a),
        }
    }
    /// After a successful compile probe only `Synthesis` can occur (structure does not
    /// depend on values). The others are internal invariant failures; arkworks 0.4 has
    /// no general variant, so they surface as `Unsatisfiable` and never as success.
    fn into_synthesis_error(self) -> SynthesisError {
        match self {
            Self::Synthesis(e) => e,
            Self::Static { .. } | Self::Shape(_) | Self::Adapter(_) => SynthesisError::Unsatisfiable,
        }
    }
}

impl From<SynthesisError> for SynthFailure {
    fn from(e: SynthesisError) -> Self {
        Self::Synthesis(e)
    }
}

impl From<GadgetError> for SynthFailure {
    fn from(e: GadgetError) -> Self {
        match e {
            GadgetError::Synthesis(s) => Self::Synthesis(s),
            GadgetError::Static(reason) => Self::Static { scope: "statement wiring".into(), reason },
        }
    }
}

impl From<LowerError> for SynthFailure {
    fn from(e: LowerError) -> Self {
        match e {
            LowerError::Gadget { error: GadgetError::Synthesis(s), .. } => Self::Synthesis(s),
            LowerError::Gadget { scope, error: GadgetError::Static(reason) } => Self::Static { scope, reason },
            LowerError::Shape(m) => Self::Shape(m),
        }
    }
}

impl From<BindError> for SynthFailure {
    fn from(e: BindError) -> Self {
        match e {
            BindError::Synthesis(s) => Self::Synthesis(s),
            BindError::Adapter(a) => Self::Adapter(a),
        }
    }
}

fn uint_of(v: &Value) -> u128 {
    match v {
        Value::UInt(UInt(n)) => *n,
        Value::Bool(_) => 0,
    }
}

fn bool_of(v: &Value) -> bool {
    matches!(v, Value::Bool(true))
}

/// Range-proven entry wire for a claimed value of type `ty`.
fn alloc_value<F: PrimeField>(cs: &mut Cs<F>, role: &str, ty: Type, v: Option<&Value>) -> Result<CVal<F>, GadgetError> {
    Ok(match ty {
        Type::UInt => CVal::U(gadgets::alloc_uint(cs, role, v.map(uint_of))?),
        Type::Bool => CVal::B(gadgets::alloc_bool(cs, role, v.map(bool_of))?),
    })
}

fn cval_wire<F: PrimeField>(v: &CVal<F>) -> SlotWire<F> {
    match v {
        CVal::U(u) => SlotWire::U(u.clone()),
        CVal::B(b) => SlotWire::B(b.clone()),
    }
}

fn synthesize<F: PrimeField>(
    relation: &CompiledRelation,
    assignment: Option<&Assignment<F>>,
    context: Option<&ContextV2>,
    tamper: Tamper<F>,
    cs_ref: ConstraintSystemRef<F>,
) -> Result<SynthesisReport, SynthFailure> {
    let plan = &relation.plan;
    let layout = &relation.layout;
    let program = relation.admitted.program();
    let mut cs = Cs::new(cs_ref, tamper);
    if cs.is_setup() != assignment.is_none() {
        return Err(SynthFailure::Shape("an assignment is required exactly when not in setup mode".into()));
    }
    let values = assignment.map(|a| &a.values);

    // Entry: claimed inputs and pre-state, each range-proven. Absent duties are
    // canonical: `(1 - present) * amount = 0`.
    let start = cs.num_constraints();
    cs.push("entry".into());
    let mut inputs = Vec::with_capacity(plan.inputs.len());
    for (i, slot) in plan.inputs.iter().enumerate() {
        inputs.push(alloc_value(&mut cs, &format!("input{i}"), slot.ty, values.map(|v| &v.inputs[i]))?);
    }
    let mut states = Vec::with_capacity(plan.states.len());
    for (i, slot) in plan.states.iter().enumerate() {
        states.push(alloc_value(&mut cs, &format!("state{i}"), slot.ty, values.map(|v| &v.pre.states[i]))?);
    }
    let mut cells = Vec::with_capacity(plan.cells.len());
    for i in 0..plan.cells.len() {
        cells.push(gadgets::alloc_uint(&mut cs, &format!("cell{i}"), values.map(|v| v.pre.cells[i].0))?);
    }
    let mut duties = Vec::with_capacity(plan.duties.len());
    for i in 0..plan.duties.len() {
        let claimed = values.map(|v| v.pre.duties[i]);
        let present = gadgets::alloc_bool(&mut cs, &format!("duty{i}.present"), claimed.map(|d| d.present))?;
        let amount = gadgets::alloc_uint(&mut cs, &format!("duty{i}.amount"), claimed.map(|d| d.amount.0))?;
        cs.enforce(Lc::one().minus(&present.lc), amount.lc(), Lc::zero())?;
        duties.push(DutyW { present, amount });
    }
    cs.pop();
    cs.record("entry".into(), start);

    // Lowering of every statement (records one region per statement).
    let entry = Entry { inputs: inputs.clone(), states: states.clone(), cells: cells.clone(), duties: duties.clone() };
    let lowered = lower::lower_action(&mut cs, program, plan, entry)?;

    // Cap: `W + slack = cap` with `slack` range-proven, i.e. `W <= cap`.
    let cap = if plan.cap == CapBinding::PublicCap {
        let start = cs.num_constraints();
        cs.push("cap".into());
        let cap = gadgets::alloc_uint(&mut cs, "cap", values.and_then(|v| v.cap).map(|c| c.0))?;
        gadgets::sub(&mut cs, "slack", &cap, &UIntW::constant(plan.logical_work.0))?;
        cs.pop();
        cs.record("cap".into(), start);
        Some(cap)
    } else {
        None
    };

    // Accounting summary from the lowered effect wires.
    let mut summary: Vec<(usize, WideW<F>, WideW<F>)> = Vec::new();
    if plan.accounting == AccountingBinding::GrossDebitAndFees {
        let start = cs.num_constraints();
        cs.push("accounting".into());
        for &cell in &plan.debit_cells {
            let mut gross: Vec<&UIntW<F>> = Vec::new();
            let mut fees: Vec<&UIntW<F>> = Vec::new();
            for (shape, wire) in plan.effects.iter().zip(&lowered.effects) {
                if shape.debit_cell() == Some(cell) {
                    let EffectW::Amount(amount) = wire else {
                        return Err(SynthFailure::Shape("Transfer/Fee effect without an amount wire".into()));
                    };
                    gross.push(amount);
                    if shape.is_fee() {
                        fees.push(amount);
                    }
                }
            }
            let g = gadgets::sum_wide(&mut cs, &format!("cell{cell}.gross"), &gross)?;
            let f = gadgets::sum_wide(&mut cs, &format!("cell{cell}.fees"), &fees)?;
            summary.push((cell, g, f));
        }
        cs.pop();
        cs.record("accounting".into(), start);
    }

    // Slot wires in layout order, then binding through the adapter.
    let duty_bool = |d: &DutyW<F>| -> BoolW<F> { d.present.clone() };
    let mut wires: Vec<SlotWire<F>> = Vec::with_capacity(layout.slots.len());
    for slot in &layout.slots {
        let wire = match slot.kind {
            SlotKind::Header(HeaderField::Cap) => {
                SlotWire::U(cap.clone().ok_or_else(|| SynthFailure::Shape("cap slot without a cap wire".into()))?)
            }
            SlotKind::Header(field) => {
                SlotWire::Const(layout.header_fields(field).ok_or_else(|| SynthFailure::Shape("header field".into()))?)
            }
            SlotKind::Input { index } => cval_wire(&inputs[index]),
            SlotKind::PreState { index } => cval_wire(&states[index]),
            SlotKind::PostState { index } => cval_wire(&lowered.states[index]),
            SlotKind::PreCell { index } => SlotWire::U(cells[index].clone()),
            SlotKind::PostCell { index } => SlotWire::U(lowered.cells[index].clone()),
            SlotKind::PreDutyPresent { index } => SlotWire::B(duty_bool(&duties[index])),
            SlotKind::PreDutyAmount { index } => SlotWire::U(duties[index].amount.clone()),
            SlotKind::PostDutyPresent { index } => SlotWire::B(duty_bool(&lowered.duties[index])),
            SlotKind::PostDutyAmount { index } => SlotWire::U(lowered.duties[index].amount.clone()),
            SlotKind::Effect { index, role } => match (&lowered.effects[index], role) {
                (EffectW::State { before, .. }, EffectRole::StateBefore) => cval_wire(before),
                (EffectW::State { after, .. }, EffectRole::StateAfter) => cval_wire(after),
                (EffectW::Amount(a), EffectRole::Amount) => SlotWire::U(a.clone()),
                (EffectW::Duty { before, .. }, EffectRole::DutyBefore) => SlotWire::U(before.clone()),
                (EffectW::Duty { after, .. }, EffectRole::DutyAfter) => SlotWire::U(after.clone()),
                (EffectW::Emit(v), EffectRole::EmitValue) => cval_wire(v),
                _ => return Err(SynthFailure::Shape(format!("effect {index} wire has no role {role:?}"))),
            },
            SlotKind::GrossDebit { cell } => SlotWire::W(
                summary.iter().find(|(c, _, _)| *c == cell).map(|(_, g, _)| g.clone()).ok_or_else(|| SynthFailure::Shape("gross row".into()))?,
            ),
            SlotKind::FeesPaid { cell } => SlotWire::W(
                summary.iter().find(|(c, _, _)| *c == cell).map(|(_, _, f)| f.clone()).ok_or_else(|| SynthFailure::Shape("fees row".into()))?,
            ),
        };
        wires.push(wire);
    }
    match relation.adapter {
        StatementAdapter::SettlementHashV2 => {
            let consts = relation.abi2.as_ref().ok_or_else(|| SynthFailure::Shape("missing ABI v2 constants".into()))?;
            let honest = match (assignment, context) {
                (None, _) => None,
                (Some(a), Some(ctx)) => {
                    let body = abi2::body_bytes(layout, plan, &a.values).map_err(|e| SynthFailure::Shape(e.to_string()))?;
                    Some(abi2::statement_bytes(consts, ctx, &body).map_err(|e| SynthFailure::Shape(e.to_string()))?)
                }
                (Some(_), None) => return Err(SynthFailure::Shape("SettlementHashV2 needs a ContextV2 to prove".into())),
            };
            abi2::bind_v2(&mut cs, consts, layout, &wires, honest.as_deref(), assignment.map(|a| a.public.as_slice()))?;
        }
        _ => relation.adapter.bind(&mut cs, layout, &wires, assignment.map(|a| a.public.as_slice()))?,
    }

    Ok(SynthesisReport { constraints: cs.num_constraints(), overrides_used: cs.overrides_used(), regions: cs.regions.clone() })
}
