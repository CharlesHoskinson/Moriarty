//! Exhaustive output-only projections of actual public candidate declarations.
//! No expected cells, decoder of executable images, evaluator or admission logic.
use mori_core::sherlock::helper_machine::{complete_observation as obs, shared::*};
use mori_core::sherlock::helper_machine::shared::{adt, adt_storage, owner_ext_type_builder as types};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::ExtDomain;
use mori_core::sherlock_compile_artifact::ext_compile as ext;
use serde_json::{Value, json};
use std::sync::Arc;
pub trait Wire { fn wire(&self) -> Value; }
impl<T: Wire + ?Sized> Wire for &T { fn wire(&self)->Value { (**self).wire() } }
impl<T: Wire> Wire for Arc<T> { fn wire(&self)->Value { (**self).wire() } }
impl<T: Wire> Wire for Box<T> { fn wire(&self)->Value { (**self).wire() } }
impl<T: Wire,E: Wire> Wire for Result<T,E> { fn wire(&self)->Value { match self { Ok(v)=>json!({"Ok":v.wire()}), Err(e)=>json!({"Err":e.wire()}) } } }
impl<T: Wire> Wire for Vec<T> { fn wire(&self)->Value { Value::Array(self.iter().map(Wire::wire).collect()) } }
impl<T: Wire> Wire for Option<T> { fn wire(&self)->Value { self.as_ref().map(Wire::wire).unwrap_or(Value::Null) } }
impl<A:Wire,B:Wire> Wire for (A,B) { fn wire(&self)->Value { json!([self.0.wire(), self.1.wire()]) } }
impl Wire for u128 { fn wire(&self)->Value { Value::String(self.to_string()) } }
macro_rules! primitive { ($($t:ty),*) => { $(impl Wire for $t { fn wire(&self)->Value { json!(self) } })* }; }
primitive!(usize,u8,u32,u64,i64,bool,String,str);
impl Wire for adt::TypeIdC { fn wire(&self)->Value { self.0.wire() } }
impl Wire for adt::CtorIdC { fn wire(&self)->Value { self.0.wire() } }
impl Wire for mori_core::sherlock::helper_machine::shared::host_implied_prefix::SegBase {
    fn wire(&self)->Value { json!({"active":self.active().wire(),"alloc":self.alloc().wire(),"consumed_work":self.consumed_work().wire()}) }
}
impl Wire for LayoutSchemaC {
    fn wire(&self)->Value { json!({
        "layout_version":self.layout_version.wire(),"field_id":self.field_id.wire(),
        "widths":self.widths.as_ref().map(|w| json!({
            "index_widths":w.index_width_entries(),"counter_widths":w.counter_width_entries(),"record_widths":w.record_width_entries()})),
        "scalar_layout_id":self.scalar_layout_id.wire(),"bounds":self.bounds.wire(),
        "capacities":self.capacities.wire(),"ext_capacities":self.ext_capacities.wire(),
        "boundary_capacities_present":self.boundary_capacities_present}) }
}

impl Wire for GrammarDomain { fn wire(&self)->Value {
match self {
Self::DedicatedSorts => json!("DedicatedSorts"),
Self::GenericNominalExtensionReserved => json!("GenericNominalExtensionReserved"),
}
} }

impl Wire for FinanceInput { fn wire(&self)->Value {
match self {
Self::Absent => json!("Absent"),
Self::TypedExtensionReserved => json!("TypedExtensionReserved"),
}
} }

impl Wire for SortTag { fn wire(&self)->Value {
match self {
Self::UInt => json!("UInt"),
Self::Bool => json!("Bool"),
Self::List => json!("List"),
Self::Tree => json!("Tree"),
}
} }

impl Wire for BinaryOp { fn wire(&self)->Value {
match self {
Self::Add => json!("Add"),
Self::Sub => json!("Sub"),
Self::Mul => json!("Mul"),
Self::Eq => json!("Eq"),
Self::Lt => json!("Lt"),
Self::Cons => json!("Cons"),
Self::Branch => json!("Branch"),
}
} }

impl Wire for UnaryOp { fn wire(&self)->Value {
match self {
Self::Leaf => json!("Leaf"),
Self::Not => json!("Not"),
}
} }

impl Wire for PatternTag { fn wire(&self)->Value {
match self {
Self::PVar => json!("PVar"),
Self::PNil => json!("PNil"),
Self::PCons => json!("PCons"),
Self::PLeaf => json!("PLeaf"),
Self::PBranch => json!("PBranch"),
Self::WildcardExtensionReserved => json!("WildcardExtensionReserved"),
Self::ConstructorExtensionReserved => json!("ConstructorExtensionReserved"),
}
} }

impl Wire for BoundaryKind { fn wire(&self)->Value {
match self {
Self::None => json!("None"),
Self::Terminal => json!("Terminal"),
Self::TechnicalCut => json!("TechnicalCut"),
Self::SourceSuspensionExtensionReserved => json!("SourceSuspensionExtensionReserved"),
}
} }

impl Wire for LogicalKind { fn wire(&self)->Value {
match self {
Self::UInt => json!("UInt"),
Self::Bool => json!("Bool"),
Self::Nil => json!("Nil"),
Self::Cons => json!("Cons"),
Self::Leaf => json!("Leaf"),
Self::Branch => json!("Branch"),
Self::Marker => json!("Marker"),
Self::ConstructorExtensionReserved => json!("ConstructorExtensionReserved"),
Self::Constructor{type_id,ctor_id} => json!({"Constructor":{"type_id":type_id.wire(),"ctor_id":ctor_id.wire()}}),
}
} }

impl Wire for ScalarMode { fn wire(&self)->Value {
match self {
Self::Add => json!("Add"),
Self::Sub => json!("Sub"),
Self::Mul => json!("Mul"),
Self::Lt => json!("Lt"),
Self::Cons => json!("Cons"),
Self::Branch => json!("Branch"),
Self::Leaf => json!("Leaf"),
Self::Not => json!("Not"),
Self::TestBool => json!("TestBool"),
Self::TestType => json!("TestType"),
}
} }

impl Wire for ScalarPhase { fn wire(&self)->Value {
match self {
Self::First => json!("First"),
Self::Second => json!("Second"),
Self::Done => json!("Done"),
}
} }

impl Wire for NodeKind { fn wire(&self)->Value {
match self {
Self::Cons => json!("Cons"),
Self::Leaf => json!("Leaf"),
Self::Branch => json!("Branch"),
}
} }

impl Wire for BuildPhase { fn wire(&self)->Value {
match self {
Self::Copy => json!("Copy"),
Self::Done => json!("Done"),
}
} }

impl Wire for PlanPhase { fn wire(&self)->Value {
match self {
Self::Walk => json!("Walk"),
Self::Test => json!("Test"),
Self::Bind => json!("Bind"),
Self::Ready => json!("Ready"),
Self::NoMatchReady => json!("NoMatchReady"),
Self::Done => json!("Done"),
}
} }

impl Wire for PlanAfterWalk { fn wire(&self)->Value {
match self {
Self::Test => json!("Test"),
Self::Bind => json!("Bind"),
}
} }

impl Wire for BoolSelector { fn wire(&self)->Value {
match self {
Self::True => json!("True"),
Self::False => json!("False"),
Self::NotBool => json!("NotBool"),
}
} }

impl Wire for TriStatus { fn wire(&self)->Value {
match self {
Self::Pending => json!("Pending"),
Self::Positive => json!("Positive"),
Self::Negative => json!("Negative"),
}
} }

impl Wire for BindPhase { fn wire(&self)->Value {
match self {
Self::Pairing => json!("Pairing"),
Self::UnwindType => json!("UnwindType"),
Self::UnwindDup => json!("UnwindDup"),
Self::UnwindInsert => json!("UnwindInsert"),
Self::Propagate => json!("Propagate"),
Self::Finished => json!("Finished"),
}
} }

impl Wire for BindStatus { fn wire(&self)->Value {
match self {
Self::Ok => json!("Ok"),
Self::Bad => json!("Bad"),
}
} }

impl Wire for AppendPhase { fn wire(&self)->Value {
match self {
Self::Copy => json!("Copy"),
Self::Tail => json!("Tail"),
Self::Done => json!("Done"),
}
} }

impl Wire for MatchPhase { fn wire(&self)->Value {
match self {
Self::Arm => json!("Arm"),
Self::Tag => json!("Tag"),
Self::Distinct => json!("Distinct"),
Self::Field => json!("Field"),
Self::Bind => json!("Bind"),
Self::Ready => json!("Ready"),
Self::NoMatchReady => json!("NoMatchReady"),
Self::Done => json!("Done"),
}
} }

impl Wire for HelperKind { fn wire(&self)->Value {
match self {
Self::Scalar => json!("Scalar"),
Self::EnvGet => json!("EnvGet"),
Self::EnvPut => json!("EnvPut"),
Self::Find => json!("Find"),
Self::Bind => json!("Bind"),
Self::Append => json!("Append"),
Self::Validate => json!("Validate"),
Self::Equal => json!("Equal"),
Self::Match => json!("Match"),
Self::Plan => json!("Plan"),
Self::Build => json!("Build"),
}
} }

impl Wire for FamilyId { fn wire(&self)->Value {
match self {
Self::Legacy => json!("Legacy"),
Self::Scalar => json!("Scalar"),
Self::EnvGet => json!("EnvGet"),
Self::EnvPut => json!("EnvPut"),
Self::Find => json!("Find"),
Self::Bind => json!("Bind"),
Self::Append => json!("Append"),
Self::Validate => json!("Validate"),
Self::Equal => json!("Equal"),
Self::Match => json!("Match"),
Self::BookkeepingRelease => json!("BookkeepingRelease"),
Self::Build => json!("Build"),
Self::Plan => json!("Plan"),
}
} }

impl Wire for EventKind { fn wire(&self)->Value {
match self {
Self::LegacyComplete => json!("LegacyComplete"),
Self::HelperEnter => json!("HelperEnter"),
Self::HelperVisit => json!("HelperVisit"),
Self::HelperExit => json!("HelperExit"),
Self::HelperFail => json!("HelperFail"),
Self::Bookkeeping => json!("Bookkeeping"),
Self::Publish => json!("Publish"),
Self::Startup => json!("Startup"),
}
} }

impl Wire for SiteTag { fn wire(&self)->Value {
match self {
Self::EvalLiteral => json!("EvalLiteral"),
Self::EvalVar => json!("EvalVar"),
Self::EvalLet => json!("EvalLet"),
Self::EvalBinary => json!("EvalBinary"),
Self::EvalUnary => json!("EvalUnary"),
Self::EvalIf => json!("EvalIf"),
Self::EvalMatch => json!("EvalMatch"),
Self::EvalCall => json!("EvalCall"),
Self::Push => json!("Push"),
Self::RetLet => json!("RetLet"),
Self::RetLeft => json!("RetLeft"),
Self::RetRight => json!("RetRight"),
Self::RetUnary => json!("RetUnary"),
Self::RetIf => json!("RetIf"),
Self::RetMatch => json!("RetMatch"),
Self::RetArgs => json!("RetArgs"),
Self::RetCall => json!("RetCall"),
Self::RetScope => json!("RetScope"),
Self::Invoke => json!("Invoke"),
Self::Enter => json!("Enter"),
Self::Bound => json!("Bound"),
Self::Cases => json!("Cases"),
Self::CaseResult => json!("CaseResult"),
Self::Prim => json!("Prim"),
Self::EvalConstruct => json!("EvalConstruct"),
Self::RetCtorFields => json!("RetCtorFields"),
Self::Materialize => json!("Materialize"),
}
} }

impl Wire for StepInvClauseId { fn wire(&self)->Value {
match self {
Self::SI01 => json!("SI01"),
Self::SI02 => json!("SI02"),
Self::SI03 => json!("SI03"),
Self::SI04 => json!("SI04"),
Self::SI05 => json!("SI05"),
Self::SI06 => json!("SI06"),
Self::SI07 => json!("SI07"),
Self::SI08 => json!("SI08"),
Self::SI09 => json!("SI09"),
Self::SI10 => json!("SI10"),
Self::SI11 => json!("SI11"),
Self::SI12 => json!("SI12"),
Self::SI13 => json!("SI13"),
Self::SI14 => json!("SI14"),
Self::SI15 => json!("SI15"),
Self::SI16 => json!("SI16"),
Self::SI17 => json!("SI17"),
Self::SI18 => json!("SI18"),
Self::SI19 => json!("SI19"),
Self::SI20 => json!("SI20"),
Self::SI21 => json!("SI21"),
Self::SI22 => json!("SI22"),
Self::SI23 => json!("SI23"),
Self::SI24 => json!("SI24"),
}
} }

impl Wire for ViewCheck { fn wire(&self)->Value {
match self {
Self::V1 => json!("V1"),
Self::V2 => json!("V2"),
Self::V3 => json!("V3"),
Self::V4 => json!("V4"),
Self::V5 => json!("V5"),
Self::V6 => json!("V6"),
Self::V7 => json!("V7"),
Self::V8 => json!("V8"),
}
} }

impl Wire for CounterWidthField { fn wire(&self)->Value {
match self {
Self::WorkCounter => json!("WorkCounter"),
Self::HelperCounter => json!("HelperCounter"),
Self::ReferenceCursor => json!("ReferenceCursor"),
}
} }

impl Wire for CapacityKind { fn wire(&self)->Value {
match self {
Self::CodeRows => json!("CodeRows"),
Self::FunctionRows => json!("FunctionRows"),
Self::CodeChildSlots => json!("CodeChildSlots"),
Self::ParameterSlots => json!("ParameterSlots"),
Self::PatternRows => json!("PatternRows"),
Self::MatchArmRows => json!("MatchArmRows"),
Self::ConstantNodes => json!("ConstantNodes"),
Self::ConstantFieldSlots => json!("ConstantFieldSlots"),
Self::MaxArity => json!("MaxArity"),
Self::NameRows => json!("NameRows"),
Self::NameBytes => json!("NameBytes"),
Self::OccurrencePathSlots => json!("OccurrencePathSlots"),
Self::ValueSlots => json!("ValueSlots"),
Self::MaxReferenceWork => json!("MaxReferenceWork"),
Self::Frames => json!("Frames"),
Self::SerializationBytes => json!("SerializationBytes"),
Self::PhysicalBoundaryEnvelope => json!("PhysicalBoundaryEnvelope"),
Self::StepCap => json!("StepCap"),
Self::CounterWidth{field} => json!({"CounterWidth":{"field":field.wire()}}),
}
} }

impl Wire for FeatureKind { fn wire(&self)->Value {
match self {
Self::GenericAdt => json!("GenericAdt"),
Self::FinancialSyntax => json!("FinancialSyntax"),
Self::FinancialInput => json!("FinancialInput"),
Self::BoundedEncodingWithoutWidths => json!("BoundedEncodingWithoutWidths"),
Self::BoundarySection => json!("BoundarySection"),
}
} }

impl Wire for AdmissionReason { fn wire(&self)->Value {
match self {
Self::LayoutWidthTooSmall => json!("LayoutWidthTooSmall"),
Self::ProgramViewMalformed => json!("ProgramViewMalformed"),
Self::GrammarDomainMismatch => json!("GrammarDomainMismatch"),
Self::ProfileMismatch => json!("ProfileMismatch"),
Self::LayoutBoundsMismatch => json!("LayoutBoundsMismatch"),
}
} }

impl Wire for HostViolationKind { fn wire(&self)->Value {
match self {
Self::StepInvViolated => json!("StepInvViolated"),
Self::SelectUndefined => json!("SelectUndefined"),
Self::ReadOutOfBounds => json!("ReadOutOfBounds"),
Self::WrongVariant => json!("WrongVariant"),
Self::DisabledRuleReached => json!("DisabledRuleReached"),
Self::ExtensionRuleReached => json!("ExtensionRuleReached"),
Self::PhysicalCapacityExceeded => json!("PhysicalCapacityExceeded"),
Self::ArithmeticUndefined => json!("ArithmeticUndefined"),
}
} }

impl Wire for GuardId { fn wire(&self)->Value {
match self {
Self::GScalarAllocNode => json!("GScalarAllocNode"),
Self::GScalarFields1 => json!("GScalarFields1"),
Self::GScalarFields2 => json!("GScalarFields2"),
Self::GValidateOccLeaf => json!("GValidateOccLeaf"),
Self::GValidateFieldNontail => json!("GValidateFieldNontail"),
Self::GEqualPairLeaf => json!("GEqualPairLeaf"),
Self::GEqualFieldNontail => json!("GEqualFieldNontail"),
Self::GBindTypeReject => json!("GBindTypeReject"),
Self::GBindDupHit => json!("GBindDupHit"),
Self::GBuildLast => json!("GBuildLast"),
}
} }

impl Wire for CapKind { fn wire(&self)->Value {
match self {
Self::DataNodes => json!("DataNodes"),
Self::DataFields => json!("DataFields"),
Self::EnvironmentCells => json!("EnvironmentCells"),
Self::HelperTasks => json!("HelperTasks"),
}
} }

impl Wire for ExhaustReason { fn wire(&self)->Value {
match self {
Self::Fuel => json!("Fuel"),
Self::DataNodes => json!("DataNodes"),
Self::DataFields => json!("DataFields"),
Self::EnvironmentCells => json!("EnvironmentCells"),
Self::HelperTasks => json!("HelperTasks"),
Self::HelperWork => json!("HelperWork"),
Self::SerializationBytes => json!("SerializationBytes"),
}
} }

impl Wire for StepDisposition { fn wire(&self)->Value {
match self {
Self::Applied => json!("Applied"),
Self::Published => json!("Published"),
Self::ExhaustedFuelUncharged => json!("ExhaustedFuelUncharged"),
Self::RefusedHelperWorkUncharged => json!("RefusedHelperWorkUncharged"),
Self::RefusedCapCharged => json!("RefusedCapCharged"),
}
} }

impl Wire for RunIntC { fn wire(&self)->Value {
match self {
Self::Negative => json!("Negative"),
Self::Nat(v0) => json!({"Nat":v0.wire()}),
Self::NatAboveU128 => json!("NatAboveU128"),
}
} }

impl Wire for TaskVisitC { fn wire(&self)->Value {
match self {
Self::Unvisited => json!("Unvisited"),
Self::Fields => json!("Fields"),
}
} }

impl Wire for RuleIdC { fn wire(&self)->Value {
match self {
Self::KMachineEvalLiteralValid => json!("KMachineEvalLiteralValid"),
Self::KMachineEvalLiteralInvalid => json!("KMachineEvalLiteralInvalid"),
Self::KMachineEvalVariablePresent => json!("KMachineEvalVariablePresent"),
Self::KMachineEvalVariableAbsent => json!("KMachineEvalVariableAbsent"),
Self::KMachineEvalLet => json!("KMachineEvalLet"),
Self::KMachineEvalBinary => json!("KMachineEvalBinary"),
Self::KMachineEvalUnary => json!("KMachineEvalUnary"),
Self::KMachineEvalIf => json!("KMachineEvalIf"),
Self::KMachineEvalMatch => json!("KMachineEvalMatch"),
Self::KMachineEvalCallEmpty => json!("KMachineEvalCallEmpty"),
Self::KMachineEvalCallArguments => json!("KMachineEvalCallArguments"),
Self::KMachinePushAccept => json!("KMachinePushAccept"),
Self::KMachinePushRefuse => json!("KMachinePushRefuse"),
Self::KMachineReturnLet => json!("KMachineReturnLet"),
Self::KMachineReturnLeft => json!("KMachineReturnLeft"),
Self::KMachineReturnRight => json!("KMachineReturnRight"),
Self::KMachineReturnUnary => json!("KMachineReturnUnary"),
Self::KMachineReturnIfTrue => json!("KMachineReturnIfTrue"),
Self::KMachineReturnIfFalse => json!("KMachineReturnIfFalse"),
Self::KMachineReturnIfInvalid => json!("KMachineReturnIfInvalid"),
Self::KMachineReturnMatch => json!("KMachineReturnMatch"),
Self::KMachineReturnArgumentsNext => json!("KMachineReturnArgumentsNext"),
Self::KMachineReturnArgumentsFinal => json!("KMachineReturnArgumentsFinal"),
Self::KMachineReturnCallValid => json!("KMachineReturnCallValid"),
Self::KMachineReturnCallInvalid => json!("KMachineReturnCallInvalid"),
Self::KMachineReturnScope => json!("KMachineReturnScope"),
Self::KMachineInvokeFind => json!("KMachineInvokeFind"),
Self::KMachineEnterMissing => json!("KMachineEnterMissing"),
Self::KMachineEnterBind => json!("KMachineEnterBind"),
Self::KMachineBoundInvalid => json!("KMachineBoundInvalid"),
Self::KMachineBoundValid => json!("KMachineBoundValid"),
Self::KMachineCasesEmpty => json!("KMachineCasesEmpty"),
Self::KMachineCasesTry => json!("KMachineCasesTry"),
Self::KMachineCasesMiss => json!("KMachineCasesMiss"),
Self::KMachineCasesHit => json!("KMachineCasesHit"),
Self::KMachinePrimitiveValue => json!("KMachinePrimitiveValue"),
Self::KMachinePrimitiveError => json!("KMachinePrimitiveError"),
Self::BHelperScalarEntry => json!("BHelperScalarEntry"),
Self::BHelperScalarTag => json!("BHelperScalarTag"),
Self::BHelperScalarArithmetic => json!("BHelperScalarArithmetic"),
Self::BHelperScalarConstruct => json!("BHelperScalarConstruct"),
Self::BHelperScalarExit => json!("BHelperScalarExit"),
Self::BHelperScalarFailure => json!("BHelperScalarFailure"),
Self::BHelperEnvGetEntry => json!("BHelperEnvGetEntry"),
Self::BHelperEnvGetCell => json!("BHelperEnvGetCell"),
Self::BHelperEnvGetEmpty => json!("BHelperEnvGetEmpty"),
Self::BHelperEnvGetExit => json!("BHelperEnvGetExit"),
Self::BHelperEnvGetFailure => json!("BHelperEnvGetFailure"),
Self::BHelperEnvPutEntry => json!("BHelperEnvPutEntry"),
Self::BHelperEnvPutCell => json!("BHelperEnvPutCell"),
Self::BHelperEnvPutInsert => json!("BHelperEnvPutInsert"),
Self::BHelperEnvPutEmpty => json!("BHelperEnvPutEmpty"),
Self::BHelperEnvPutExit => json!("BHelperEnvPutExit"),
Self::BHelperEnvPutFailure => json!("BHelperEnvPutFailure"),
Self::BHelperFindEntry => json!("BHelperFindEntry"),
Self::BHelperFindRow => json!("BHelperFindRow"),
Self::BHelperFindEmpty => json!("BHelperFindEmpty"),
Self::BHelperFindExit => json!("BHelperFindExit"),
Self::BHelperFindFailure => json!("BHelperFindFailure"),
Self::BHelperBindEntry => json!("BHelperBindEntry"),
Self::BHelperBindPair => json!("BHelperBindPair"),
Self::BHelperBindEmpty => json!("BHelperBindEmpty"),
Self::BHelperBindArity => json!("BHelperBindArity"),
Self::BHelperBindType => json!("BHelperBindType"),
Self::BHelperBindDuplicate => json!("BHelperBindDuplicate"),
Self::BHelperBindInsert => json!("BHelperBindInsert"),
Self::BHelperBindPropagate => json!("BHelperBindPropagate"),
Self::BHelperBindExit => json!("BHelperBindExit"),
Self::BHelperBindFailure => json!("BHelperBindFailure"),
Self::BHelperAppendEntry => json!("BHelperAppendEntry"),
Self::BHelperAppendCell => json!("BHelperAppendCell"),
Self::BHelperAppendEmpty => json!("BHelperAppendEmpty"),
Self::BHelperAppendTail => json!("BHelperAppendTail"),
Self::BHelperAppendCopy => json!("BHelperAppendCopy"),
Self::BHelperAppendExit => json!("BHelperAppendExit"),
Self::BHelperAppendFailure => json!("BHelperAppendFailure"),
Self::BHelperValidateEntry => json!("BHelperValidateEntry"),
Self::BHelperValidateOccurrence => json!("BHelperValidateOccurrence"),
Self::BHelperValidateField => json!("BHelperValidateField"),
Self::BHelperValidateDeclaration => json!("BHelperValidateDeclaration"),
Self::BHelperValidateEmpty => json!("BHelperValidateEmpty"),
Self::BHelperValidateInvalid => json!("BHelperValidateInvalid"),
Self::BHelperValidateExit => json!("BHelperValidateExit"),
Self::BHelperValidateFailure => json!("BHelperValidateFailure"),
Self::BHelperEqualEntry => json!("BHelperEqualEntry"),
Self::BHelperEqualPair => json!("BHelperEqualPair"),
Self::BHelperEqualField => json!("BHelperEqualField"),
Self::BHelperEqualEmpty => json!("BHelperEqualEmpty"),
Self::BHelperEqualDifferent => json!("BHelperEqualDifferent"),
Self::BHelperEqualExit => json!("BHelperEqualExit"),
Self::BHelperEqualFailure => json!("BHelperEqualFailure"),
Self::BHelperMatchEntry => json!("BHelperMatchEntry"),
Self::BHelperMatchArm => json!("BHelperMatchArm"),
Self::BHelperMatchOccurrence => json!("BHelperMatchOccurrence"),
Self::BHelperMatchField => json!("BHelperMatchField"),
Self::BHelperMatchBinding => json!("BHelperMatchBinding"),
Self::BHelperMatchDistinct => json!("BHelperMatchDistinct"),
Self::BHelperMatchDiscard => json!("BHelperMatchDiscard"),
Self::BHelperMatchNext => json!("BHelperMatchNext"),
Self::BHelperMatchSelected => json!("BHelperMatchSelected"),
Self::BHelperMatchEmpty => json!("BHelperMatchEmpty"),
Self::BHelperMatchExit => json!("BHelperMatchExit"),
Self::BHelperMatchFailure => json!("BHelperMatchFailure"),
Self::BBookkeepingTaskEnqueue => json!("BBookkeepingTaskEnqueue"),
Self::BBookkeepingTaskRelease => json!("BBookkeepingTaskRelease"),
Self::BBookkeepingResumeStore => json!("BBookkeepingResumeStore"),
Self::BBookkeepingResumeRestore => json!("BBookkeepingResumeRestore"),
Self::KAdminStartupValid => json!("KAdminStartupValid"),
Self::KAdminStartupInvalid => json!("KAdminStartupInvalid"),
Self::KAdminPublishValue => json!("KAdminPublishValue"),
Self::KAdminPublishError => json!("KAdminPublishError"),
Self::KAdminPublishExhausted => json!("KAdminPublishExhausted"),
Self::XAdtEvalConstructNullary => json!("XAdtEvalConstructNullary"),
Self::XAdtEvalConstructFields => json!("XAdtEvalConstructFields"),
Self::XAdtReturnCtorFieldsTyped => json!("XAdtReturnCtorFieldsTyped"),
Self::XAdtReturnCtorFieldsMistyped => json!("XAdtReturnCtorFieldsMistyped"),
Self::XAdtReturnCtorFieldsNext => json!("XAdtReturnCtorFieldsNext"),
Self::XAdtReturnCtorFieldsFinal => json!("XAdtReturnCtorFieldsFinal"),
Self::XAdtMaterializeDone => json!("XAdtMaterializeDone"),
Self::XAdtCasesPlanHit => json!("XAdtCasesPlanHit"),
Self::XAdtCasesPlanMiss => json!("XAdtCasesPlanMiss"),
Self::XAdtHelperBuildEntry => json!("XAdtHelperBuildEntry"),
Self::XAdtHelperBuildCell => json!("XAdtHelperBuildCell"),
Self::XAdtHelperBuildField => json!("XAdtHelperBuildField"),
Self::XAdtHelperBuildExit => json!("XAdtHelperBuildExit"),
Self::XAdtHelperPlanEntry => json!("XAdtHelperPlanEntry"),
Self::XAdtHelperPlanWalk => json!("XAdtHelperPlanWalk"),
Self::XAdtHelperPlanSwitch => json!("XAdtHelperPlanSwitch"),
Self::XAdtHelperPlanBind => json!("XAdtHelperPlanBind"),
Self::XAdtHelperPlanSelect => json!("XAdtHelperPlanSelect"),
Self::XAdtHelperPlanNoMatch => json!("XAdtHelperPlanNoMatch"),
Self::XAdtHelperPlanExit => json!("XAdtHelperPlanExit"),
Self::XAdtHelperPlanFailure => json!("XAdtHelperPlanFailure"),
}
} }

impl Wire for DigestC { fn wire(&self)->Value {
match self {
Self::Unselected => json!("Unselected"),
Self::Selected{format,bytes} => json!({"Selected":{"format":format.wire(),"bytes":bytes.wire()}}),
}
} }

impl Wire for IdentityC { fn wire(&self)->Value {
json!({"profile_id":self.profile_id.wire(),"grammar_domain":self.grammar_domain.wire(),"finance":self.finance.wire(),"layout_version":self.layout_version.wire(),"layout_digest":self.layout_digest.wire(),"bounds_digest":self.bounds_digest.wire(),"field_id":self.field_id.wire(),"program_membership_digest":self.program_membership_digest.wire()})
} }

impl Wire for RunIdC { fn wire(&self)->Value {
json!({"identity":self.identity.wire(),"fuel_input":self.fuel_input.wire(),"stack_limit_input":self.stack_limit_input.wire()})
} }

impl Wire for ExpandedStepIdC { fn wire(&self)->Value {
json!({"run_id":self.run_id.wire(),"ordinal":self.ordinal.wire()})
} }

impl Wire for HelperResultTagC { fn wire(&self)->Value {
match self {
Self::Validated => json!("Validated"),
Self::EnvFound => json!("EnvFound"),
Self::EnvMissing => json!("EnvMissing"),
Self::EnvUpdated => json!("EnvUpdated"),
Self::FindFound => json!("FindFound"),
Self::FindMissing => json!("FindMissing"),
Self::BindOk => json!("BindOk"),
Self::BindBad => json!("BindBad"),
Self::AppendDone => json!("AppendDone"),
Self::ScalarValue => json!("ScalarValue"),
Self::ScalarError => json!("ScalarError"),
Self::ScalarSelector => json!("ScalarSelector"),
Self::ScalarTypeTest => json!("ScalarTypeTest"),
Self::EqualDone => json!("EqualDone"),
Self::MatchMatched => json!("MatchMatched"),
Self::MatchNoMatch => json!("MatchNoMatch"),
Self::PlanMatched => json!("PlanMatched"),
Self::PlanNoMatch => json!("PlanNoMatch"),
Self::BuildDone => json!("BuildDone"),
}
} }

impl Wire for OutcomeTagC { fn wire(&self)->Value {
match self {
Self::Running => json!("Running"),
Self::Halted => json!("Halted"),
Self::Rejected => json!("Rejected"),
Self::Exhausted => json!("Exhausted"),
}
} }

impl Wire for HelperPhaseSummaryC { fn wire(&self)->Value {
match self {
Self::Scalar(v0) => json!({"Scalar":v0.wire()}),
Self::Tri(v0) => json!({"Tri":v0.wire()}),
Self::Bind(v0) => json!({"Bind":v0.wire()}),
Self::Append(v0) => json!({"Append":v0.wire()}),
Self::Match(v0) => json!({"Match":v0.wire()}),
Self::Build(v0) => json!({"Build":v0.wire()}),
Self::Plan(v0) => json!({"Plan":v0.wire()}),
}
} }

impl Wire for AllocDeltasC { fn wire(&self)->Value {
json!({"data_nodes":self.data_nodes.wire(),"data_fields":self.data_fields.wire(),"environment_cells":self.environment_cells.wire()})
} }

impl Wire for LocatorC { fn wire(&self)->Value {
match self {
Self::Occurrence{owner_is_function,function,ast_path} => json!({"Occurrence":{"owner_is_function":owner_is_function.wire(),"function":function.wire(),"ast_path":ast_path.wire()}}),
Self::Table{table,row} => json!({"Table":{"table":table.wire(),"row":row.wire()}}),
Self::Field{name} => json!({"Field":{"name":name.wire()}}),
}
} }

impl Wire for StepEventC { fn wire(&self)->Value {
json!({"step_id":self.step_id.wire(),"rule_id":self.rule_id.wire(),"event_kind":self.event_kind.wire(),"site":self.site.wire(),"helper_kind":self.helper_kind.wire(),"disposition":self.disposition.wire(),"work_delta":self.work_delta.wire(),"helper_work_delta":self.helper_work_delta.wire(),"helper_visits_delta":self.helper_visits_delta.wire(),"reference_cursor_delta":self.reference_cursor_delta.wire(),"alloc_deltas":self.alloc_deltas.wire(),"value_slot_writes":self.value_slot_writes.wire(),"logical_depth_before":self.logical_depth_before.wire(),"logical_depth_after":self.logical_depth_after.wire(),"task_cursor_before":self.task_cursor_before.wire(),"task_cursor_after":self.task_cursor_after.wire(),"helper_phase_before":self.helper_phase_before.wire(),"helper_phase_after":self.helper_phase_after.wire(),"helper_cursor_before":self.helper_cursor_before.wire(),"helper_cursor_after":self.helper_cursor_after.wire(),"result_tag":self.result_tag.wire(),"origin_reference_cursor":self.origin_reference_cursor.wire(),"source_expr_id":self.source_expr_id.wire(),"source_occurrence":self.source_occurrence.wire(),"exhausted_kind":self.exhausted_kind.wire(),"outcome_tag":self.outcome_tag.wire(),"outcome_payload":self.outcome_payload.wire(),"boundary":self.boundary.wire()})
} }

impl Wire for BoundsC { fn wire(&self)->Value {
json!({"code_rows":self.code_rows.wire(),"function_rows":self.function_rows.wire(),"code_child_slots":self.code_child_slots.wire(),"parameter_slots":self.parameter_slots.wire(),"pattern_rows":self.pattern_rows.wire(),"match_plan_rows":self.match_plan_rows.wire(),"match_arm_rows":self.match_arm_rows.wire(),"type_rows":self.type_rows.wire(),"constructor_rows":self.constructor_rows.wire(),"constructor_field_slots":self.constructor_field_slots.wire(),"max_arity":self.max_arity.wire(),"constant_nodes":self.constant_nodes.wire(),"constant_field_slots":self.constant_field_slots.wire(),"data_nodes":self.data_nodes.wire(),"data_field_slots":self.data_field_slots.wire(),"environment_cells":self.environment_cells.wire(),"frames":self.frames.wire(),"helper_tasks":self.helper_tasks.wire(),"serialization_bytes":self.serialization_bytes.wire(),"max_reference_work":self.max_reference_work.wire(),"max_helper_work":self.max_helper_work.wire(),"step_cap":self.step_cap.wire()})
} }

impl Wire for LayoutCapacitiesC { fn wire(&self)->Value {
json!({"name_rows":self.name_rows.wire(),"name_bytes":self.name_bytes.wire(),"occurrence_path_slots":self.occurrence_path_slots.wire(),"value_slots":self.value_slots.wire()})
} }

impl Wire for RefC { fn wire(&self)->Value {
match self {
Self::Const(v0) => json!({"Const":v0.wire()}),
Self::Data(v0) => json!({"Data":v0.wire()}),
}
} }

impl Wire for SliceC { fn wire(&self)->Value {
json!({"start":self.start.wire(),"len":self.len.wire()})
} }

impl Wire for AllocCounters { fn wire(&self)->Value {
json!({"data_nodes":self.data_nodes.wire(),"data_fields":self.data_fields.wire(),"environment_cells":self.environment_cells.wire()})
} }

impl Wire for ActiveLengths { fn wire(&self)->Value {
json!({"data":self.data.wire(),"data_fields":self.data_fields.wire(),"env":self.env.wire(),"value_slots":self.value_slots.wire()})
} }

impl Wire for HelperCommon { fn wire(&self)->Value {
json!({"site":self.site.wire(),"origin_reference_cursor":self.origin_reference_cursor.wire(),"parent_resume":self.parent_resume.wire()})
} }

impl Wire for ScalarStaged { fn wire(&self)->Value {
match self {
Self::None => json!("None"),
Self::U128Value{n} => json!({"U128Value":{"n":n.wire()}}),
Self::BoolValue{b} => json!({"BoolValue":{"b":b.wire()}}),
Self::PendingNode{tag} => json!({"PendingNode":{"tag":tag.wire()}}),
Self::PrimError{code} => json!({"PrimError":{"code":code.wire()}}),
Self::Selector{sel} => json!({"Selector":{"sel":sel.wire()}}),
Self::TypeTest{ok} => json!({"TypeTest":{"ok":ok.wire()}}),
}
} }

impl Wire for HelperControlC { fn wire(&self)->Value {
match self {
Self::Scalar{common,mode,operand_a,operand_b,expected,phase,reg_a,staged} => json!({"Scalar":{"common":common.wire(),"mode":mode.wire(),"operand_a":operand_a.wire(),"operand_b":operand_b.wire(),"expected":expected.wire(),"phase":phase.wire(),"reg_a":reg_a.wire(),"staged":staged.wire()}}),
Self::EnvGet{common,binder,cursor,found,status} => json!({"EnvGet":{"common":common.wire(),"binder":binder.wire(),"cursor":cursor.wire(),"found":found.wire(),"status":status.wire()}}),
Self::EnvPut{common,binder,value,base,new_root,status} => json!({"EnvPut":{"common":common.wire(),"binder":binder.wire(),"value":value.wire(),"base":base.wire(),"new_root":new_root.wire(),"status":status.wire()}}),
Self::Find{common,function_name,row_cursor,found,status} => json!({"Find":{"common":common.wire(),"function_name":function_name.wire(),"row_cursor":row_cursor.wire(),"found":found.wire(),"status":status.wire()}}),
Self::Bind{common,function,params,args,p_cursor,a_cursor,partial,dup_cursor,phase,status} => json!({"Bind":{"common":common.wire(),"function":function.wire(),"params":params.wire(),"args":args.wire(),"p_cursor":p_cursor.wire(),"a_cursor":a_cursor.wire(),"partial":partial.wire(),"dup_cursor":dup_cursor.wire(),"phase":phase.wire(),"status":status.wire()}}),
Self::Append{common,src,value,src_cursor,dest_start,staged,phase} => json!({"Append":{"common":common.wire(),"src":src.wire(),"value":value.wire(),"src_cursor":src_cursor.wire(),"dest_start":dest_start.wire(),"staged":staged.wire(),"phase":phase.wire()}}),
Self::Validate{common,root,status} => json!({"Validate":{"common":common.wire(),"root":root.wire(),"status":status.wire()}}),
Self::Equal{common,left,right,status} => json!({"Equal":{"common":common.wire(),"left":left.wire(),"right":right.wire(),"status":status.wire()}}),
Self::Match{common,scrutinee,arm,saved_env,partial,pat_tag,x,y,field_idx,staged,phase,status} => json!({"Match":{"common":common.wire(),"scrutinee":scrutinee.wire(),"arm":arm.wire(),"saved_env":saved_env.wire(),"partial":partial.wire(),"pat_tag":pat_tag.wire(),"x":x.wire(),"y":y.wire(),"field_idx":field_idx.wire(),"staged":staged.wire(),"phase":phase.wire(),"status":status.wire()}}),
Self::Plan{common,source_match,scrutinee,saved_env,partial,node,phase,after_walk,cur,path,path_cursor,arm,status} => json!({"Plan":{"common":common.wire(),"source_match":source_match.wire(),"scrutinee":scrutinee.wire(),"saved_env":saved_env.wire(),"partial":partial.wire(),"node":node.wire(),"phase":phase.wire(),"after_walk":after_walk.wire(),"cur":cur.wire(),"path":path.wire(),"path_cursor":path_cursor.wire(),"arm":arm.wire(),"status":status.wire()}}),
Self::Build{common,type_id,ctor_id,src,src_cursor,field_start,staged,node,phase} => json!({"Build":{"common":common.wire(),"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"src":src.wire(),"src_cursor":src_cursor.wire(),"field_start":field_start.wire(),"staged":staged.wire(),"node":node.wire(),"phase":phase.wire()}}),
}
} }

impl Wire for FrameRowC { fn wire(&self)->Value {
match self {
Self::LetF{binder,body,saved_env,origin} => json!({"LetF":{"binder":binder.wire(),"body":body.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::LeftF{op,right,saved_env,origin} => json!({"LeftF":{"op":op.wire(),"right":right.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::RightF{op,left,origin} => json!({"RightF":{"op":op.wire(),"left":left.wire(),"origin":origin.wire()}}),
Self::UnaryF{op,origin} => json!({"UnaryF":{"op":op.wire(),"origin":origin.wire()}}),
Self::IfF{then_expr,else_expr,saved_env,origin} => json!({"IfF":{"then_expr":then_expr.wire(),"else_expr":else_expr.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::MatchF{arms,saved_env,origin} => json!({"MatchF":{"arms":arms.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::ArgsF{function_name,completed,rest,saved_env,origin} => json!({"ArgsF":{"function_name":function_name.wire(),"completed":completed.wire(),"rest":rest.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::ReturnF{return_type,saved_env,origin} => json!({"ReturnF":{"return_type":return_type.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::ScopeF{saved_env,origin} => json!({"ScopeF":{"saved_env":saved_env.wire(),"origin":origin.wire()}}),
Self::ConstructorFieldsF{type_id,ctor_id,index,field_type,checked,completed,rest,saved_env,origin} => json!({"ConstructorFieldsF":{"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"index":index.wire(),"field_type":field_type.wire(),"checked":checked.wire(),"completed":completed.wire(),"rest":rest.wire(),"saved_env":saved_env.wire(),"origin":origin.wire()}}),
}
} }

impl Wire for BoundBinding { fn wire(&self)->Value {
match self {
Self::Bad => json!("Bad"),
Self::Ok{env} => json!({"Ok":{"env":env.wire()}}),
}
} }

impl Wire for CaseMatching { fn wire(&self)->Value {
match self {
Self::NoMatch => json!("NoMatch"),
Self::Matched{env} => json!({"Matched":{"env":env.wire()}}),
}
} }

impl Wire for PrimOutcome { fn wire(&self)->Value {
match self {
Self::Value{value} => json!({"Value":{"value":value.wire()}}),
Self::Error{code} => json!({"Error":{"code":code.wire()}}),
}
} }

impl Wire for ControlC { fn wire(&self)->Value {
match self {
Self::Eval{expr} => json!({"Eval":{"expr":expr.wire()}}),
Self::Return{value} => json!({"Return":{"value":value.wire()}}),
Self::ScheduleNext{frame,next,env} => json!({"ScheduleNext":{"frame":frame.wire(),"next":next.wire(),"env":env.wire()}}),
Self::Invoke{function_name,args} => json!({"Invoke":{"function_name":function_name.wire(),"args":args.wire()}}),
Self::Enter{function,args} => json!({"Enter":{"function":function.wire(),"args":args.wire()}}),
Self::Bound{binding,return_type,body} => json!({"Bound":{"binding":binding.wire(),"return_type":return_type.wire(),"body":body.wire()}}),
Self::Cases{value,arms,saved_env} => json!({"Cases":{"value":value.wire(),"arms":arms.wire(),"saved_env":saved_env.wire()}}),
Self::CaseResult{matching,value,body,rest,saved_env} => json!({"CaseResult":{"matching":matching.wire(),"value":value.wire(),"body":body.wire(),"rest":rest.wire(),"saved_env":saved_env.wire()}}),
Self::PrimResult{result} => json!({"PrimResult":{"result":result.wire()}}),
Self::Error{code} => json!({"Error":{"code":code.wire()}}),
Self::Materialize{type_id,ctor_id,fields} => json!({"Materialize":{"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"fields":fields.wire()}}),
}
} }

impl Wire for HelperTaskC { fn wire(&self)->Value {
match self {
Self::ValidateTask{r,visit,cursor} => json!({"ValidateTask":{"r":r.wire(),"visit":visit.wire(),"cursor":cursor.wire()}}),
Self::EqualTask{left,right,visit,cursor} => json!({"EqualTask":{"left":left.wire(),"right":right.wire(),"visit":visit.wire(),"cursor":cursor.wire()}}),
Self::PairTask{binder,sort,arg} => json!({"PairTask":{"binder":binder.wire(),"sort":sort.wire(),"arg":arg.wire()}}),
}
} }

impl Wire for HelperResultC { fn wire(&self)->Value {
match self {
Self::Validated{ok} => json!({"Validated":{"ok":ok.wire()}}),
Self::EnvFound{value} => json!({"EnvFound":{"value":value.wire()}}),
Self::EnvMissing => json!("EnvMissing"),
Self::EnvUpdated{cell} => json!({"EnvUpdated":{"cell":cell.wire()}}),
Self::FindFound{function} => json!({"FindFound":{"function":function.wire()}}),
Self::FindMissing => json!("FindMissing"),
Self::BindOk{env} => json!({"BindOk":{"env":env.wire()}}),
Self::BindBad => json!("BindBad"),
Self::AppendDone{seq} => json!({"AppendDone":{"seq":seq.wire()}}),
Self::ScalarValue{value} => json!({"ScalarValue":{"value":value.wire()}}),
Self::ScalarError{code} => json!({"ScalarError":{"code":code.wire()}}),
Self::ScalarSelector{sel} => json!({"ScalarSelector":{"sel":sel.wire()}}),
Self::ScalarTypeTest{ok} => json!({"ScalarTypeTest":{"ok":ok.wire()}}),
Self::EqualDone{value} => json!({"EqualDone":{"value":value.wire()}}),
Self::MatchMatched{env} => json!({"MatchMatched":{"env":env.wire()}}),
Self::MatchNoMatch => json!("MatchNoMatch"),
Self::PlanMatched{arm,env} => json!({"PlanMatched":{"arm":arm.wire(),"env":env.wire()}}),
Self::PlanNoMatch => json!("PlanNoMatch"),
Self::BuildDone{value} => json!({"BuildDone":{"value":value.wire()}}),
}
} }

impl Wire for OutcomeC { fn wire(&self)->Value {
match self {
Self::Running => json!("Running"),
Self::Halted{value} => json!({"Halted":{"value":value.wire()}}),
Self::Rejected{code} => json!({"Rejected":{"code":code.wire()}}),
Self::Exhausted{reason} => json!({"Exhausted":{"reason":reason.wire()}}),
}
} }

impl Wire for EnvCellC { fn wire(&self)->Value {
json!({"binder":self.binder.wire(),"value":self.value.wire(),"previous":self.previous.wire()})
} }

impl Wire for FunctionRowC { fn wire(&self)->Value {
json!({"function_name":self.function_name.wire(),"params":self.params.wire(),"return_type":self.return_type.wire(),"body":self.body.wire()})
} }

impl Wire for ParameterRowC { fn wire(&self)->Value {
json!({"binder":self.binder.wire(),"sort":self.sort.wire()})
} }

impl Wire for ArmRowC { fn wire(&self)->Value {
json!({"pattern":self.pattern.wire(),"body":self.body.wire()})
} }

impl Wire for PatternRowC { fn wire(&self)->Value {
match self {
Self::PVar{x} => json!({"PVar":{"x":x.wire()}}),
Self::PNil => json!("PNil"),
Self::PCons{x,y} => json!({"PCons":{"x":x.wire(),"y":y.wire()}}),
Self::PLeaf{x} => json!({"PLeaf":{"x":x.wire()}}),
Self::PBranch{x,y} => json!({"PBranch":{"x":x.wire(),"y":y.wire()}}),
Self::Wildcard => json!("Wildcard"),
Self::Constructor{type_id,ctor_id,children} => json!({"Constructor":{"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"children":children.wire()}}),
}
} }

impl Wire for PlanNodeC { fn wire(&self)->Value {
match self {
Self::Switch{source_match,path,type_id,cases,default} => json!({"Switch":{"source_match":source_match.wire(),"path":path.wire(),"type_id":type_id.wire(),"cases":cases.wire(),"default":default.wire()}}),
Self::Bind{source_match,path,binder,next} => json!({"Bind":{"source_match":source_match.wire(),"path":path.wire(),"binder":binder.wire(),"next":next.wire()}}),
Self::Select{source_match,arm} => json!({"Select":{"source_match":source_match.wire(),"arm":arm.wire()}}),
Self::NoMatch{source_match} => json!({"NoMatch":{"source_match":source_match.wire()}}),
}
} }

impl Wire for OccurrenceRowC { fn wire(&self)->Value {
json!({"owner_is_function":self.owner_is_function.wire(),"function":self.function.wire(),"path":self.path.wire()})
} }

impl Wire for HostViolation { fn wire(&self)->Value {
json!({"kind":self.kind.wire(),"clause":self.clause.wire(),"rule":self.rule.wire(),"locator":self.locator.wire()})
} }

impl Wire for NeededNatC { fn wire(&self)->Value {
match self {
Self::Exact(v0) => json!({"Exact":v0.wire()}),
Self::AboveU128 => json!("AboveU128"),
}
} }

impl Wire for Tooling { fn wire(&self)->Value {
match self {
Self::AdmissionRejected{reason,detail,locator} => json!({"AdmissionRejected":{"reason":reason.wire(),"detail":detail.wire(),"locator":locator.wire()}}),
Self::CapacityNotAdmitted{kind,needed,limit,locator} => json!({"CapacityNotAdmitted":{"kind":kind.wire(),"needed":needed.wire(),"limit":limit.wire(),"locator":locator.wire()}}),
Self::UnsupportedFeature{feature,locator} => json!({"UnsupportedFeature":{"feature":feature.wire(),"locator":locator.wire()}}),
}
} }

impl Wire for obs::WidthsSnapshot { fn wire(&self)->Value {
json!({"index_widths":self.index_widths.wire(),"counter_widths":self.counter_widths.wire(),"record_widths":self.record_widths.wire()})
} }

impl Wire for obs::LayoutSnapshot { fn wire(&self)->Value {
json!({"layout_version":self.layout_version.wire(),"field_id":self.field_id.wire(),"widths":self.widths.wire(),"scalar_layout_id":self.scalar_layout_id.wire(),"bounds":self.bounds.wire(),"capacities":self.capacities.wire(),"boundary_capacities_present":self.boundary_capacities_present.wire()})
} }

impl Wire for obs::ExpressionSnapshot { fn wire(&self)->Value {
match self {
Self::Literal{constant} => json!({"Literal":{"constant":constant.wire()}}),
Self::Var{binder} => json!({"Var":{"binder":binder.wire()}}),
Self::Let{binder,value,body} => json!({"Let":{"binder":binder.wire(),"value":value.wire(),"body":body.wire()}}),
Self::Binary{op,left,right} => json!({"Binary":{"op":op.wire(),"left":left.wire(),"right":right.wire()}}),
Self::Unary{op,operand} => json!({"Unary":{"op":op.wire(),"operand":operand.wire()}}),
Self::If{cond,then_expr,else_expr} => json!({"If":{"cond":cond.wire(),"then_expr":then_expr.wire(),"else_expr":else_expr.wire()}}),
Self::Call{function_name,args} => json!({"Call":{"function_name":function_name.wire(),"args":args.wire()}}),
Self::Match{scrutinee,arms,plan} => json!({"Match":{"scrutinee":scrutinee.wire(),"arms":arms.wire(),"plan":plan.wire()}}),
Self::Construct{type_id,ctor_id,fields,constant} => json!({"Construct":{"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"fields":fields.wire(),"constant":constant.wire()}}),
}
} }

impl Wire for obs::ConstFieldsSnapshot { fn wire(&self)->Value {
json!({"start":self.start.wire(),"len":self.len.wire()})
} }

impl Wire for obs::DataFieldsSnapshot { fn wire(&self)->Value {
json!({"start":self.start.wire(),"len":self.len.wire()})
} }

impl Wire for obs::ConstantSnapshot { fn wire(&self)->Value {
match self {
Self::U128{marker_free,n} => json!({"U128":{"marker_free":marker_free.wire(),"n":n.wire()}}),
Self::Bool{marker_free,b} => json!({"Bool":{"marker_free":marker_free.wire(),"b":b.wire()}}),
Self::Nil{marker_free} => json!({"Nil":{"marker_free":marker_free.wire()}}),
Self::Cons{marker_free,fields} => json!({"Cons":{"marker_free":marker_free.wire(),"fields":fields.wire()}}),
Self::Leaf{marker_free,fields} => json!({"Leaf":{"marker_free":marker_free.wire(),"fields":fields.wire()}}),
Self::Branch{marker_free,fields} => json!({"Branch":{"marker_free":marker_free.wire(),"fields":fields.wire()}}),
Self::Constructor{marker_free,type_id,ctor_id,fields} => json!({"Constructor":{"marker_free":marker_free.wire(),"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"fields":fields.wire()}}),
Self::U128OutOfRange{marker_free} => json!({"U128OutOfRange":{"marker_free":marker_free.wire()}}),
}
} }

impl Wire for obs::DataSnapshot { fn wire(&self)->Value {
match self {
Self::U128{n} => json!({"U128":{"n":n.wire()}}),
Self::Bool{b} => json!({"Bool":{"b":b.wire()}}),
Self::Nil => json!("Nil"),
Self::Cons{fields} => json!({"Cons":{"fields":fields.wire()}}),
Self::Leaf{fields} => json!({"Leaf":{"fields":fields.wire()}}),
Self::Branch{fields} => json!({"Branch":{"fields":fields.wire()}}),
Self::Constructor{type_id,ctor_id,fields} => json!({"Constructor":{"type_id":type_id.wire(),"ctor_id":ctor_id.wire(),"fields":fields.wire()}}),
}
} }

impl Wire for obs::ProgramSnapshot { fn wire(&self)->Value {
json!({"entry":self.entry.wire(),"expressions":self.expressions.wire(),"occurrences":self.occurrences.wire(),"occurrence_paths":self.occurrence_paths.wire(),"children":self.children.wire(),"functions":self.functions.wire(),"parameters":self.parameters.wire(),"arms":self.arms.wire(),"patterns":self.patterns.wire(),"pattern_children":self.pattern_children.wire(),"types":self.types.wire(),"type_args":self.type_args.wire(),"constructors":self.constructors.wire(),"ctor_fields":self.ctor_fields.wire(),"decl_param_counts":self.decl_param_counts.wire(),"decl_ctor_counts":self.decl_ctor_counts.wire(),"decl_templates":self.decl_templates.wire(),"plan_nodes":self.plan_nodes.wire(),"plan_cases":self.plan_cases.wire(),"field_paths":self.field_paths.wire(),"constants":self.constants.wire(),"constant_fields":self.constant_fields.wire(),"names":self.names.wire()})
} }

impl Wire for obs::AdmissionSnapshot { fn wire(&self)->Value {
json!({"identity":self.identity.wire(),"bounds":self.bounds.wire(),"view":self.view.wire(),"layout":self.layout.wire()})
} }

impl Wire for obs::ImageSnapshot { fn wire(&self)->Value {
json!({"run_id":self.run_id.wire(),"control":self.control.wire(),"control_origin":self.control_origin.wire(),"env_root":self.env_root.wire(),"stack_limit":self.stack_limit.wire(),"outcome":self.outcome.wire(),"initial_fuel":self.initial_fuel.wire(),"remaining_work":self.remaining_work.wire(),"consumed_work":self.consumed_work.wire(),"helper_work":self.helper_work.wire(),"helper_visits":self.helper_visits.wire(),"reference_cursor":self.reference_cursor.wire(),"alloc":self.alloc.wire(),"active":self.active.wire(),"helper":self.helper.wire(),"result":self.result.wire(),"seg_base":self.seg_base.wire(),"data":self.data.wire(),"data_fields":self.data_fields.wire(),"env":self.env.wire(),"frames":self.frames.wire(),"tasks":self.tasks.wire(),"value_slots":self.value_slots.wire()})
} }

impl Wire for obs::HostSnapshot { fn wire(&self)->Value {
json!({"admission":self.admission.wire(),"image":self.image.wire()})
} }

impl Wire for obs::CompleteStepRecord { fn wire(&self)->Value {
json!({"before":self.before.wire(),"event":self.event.wire(),"after":self.after.wire()})
} }

impl Wire for obs::ObservedStep { fn wire(&self)->Value {
match self {
Self::Event(v0) => json!({"Event":v0.wire()}),
Self::Terminal{before,after} => json!({"Terminal":{"before":before.wire(),"after":after.wire()}}),
}
} }

impl Wire for obs::CaptureStepError { fn wire(&self)->Value {
match self {
Self::BeforeObservation(v0) => json!({"BeforeObservation":v0.wire()}),
Self::Step{violation,before,after} => json!({"Step":{"violation":violation.wire(),"before":before.wire(),"after":after.wire()}}),
Self::AfterObservation{violation,before,event} => json!({"AfterObservation":{"violation":violation.wire(),"before":before.wire(),"event":event.wire()}}),
}
} }

impl Wire for adt::TypeRowC { fn wire(&self)->Value {
json!({"name_id":self.name_id.wire(),"decl":self.decl.wire(),"args":self.args.wire(),"ctor_start":self.ctor_start.wire(),"ctor_len":self.ctor_len.wire()})
} }

impl Wire for adt_storage::CtorRowExt { fn wire(&self)->Value {
json!({"owning_type":self.owning_type.wire(),"name_id":self.name_id.wire(),"source_order":self.source_order.wire(),"field_start":self.field_start.wire(),"field_len":self.field_len.wire()})
} }

impl Wire for adt_storage::CtorFieldRowExt { fn wire(&self)->Value {
json!({"name_id":self.name_id.wire(),"type_id":self.type_id.wire()})
} }

impl Wire for types::TypeExprSrc { fn wire(&self)->Value {
match self {
Self::UInt => json!("UInt"),
Self::Bool => json!("Bool"),
Self::List => json!("List"),
Self::Tree => json!("Tree"),
Self::Named{name} => json!({"Named":{"name":name.wire()}}),
Self::Apply{name,args} => json!({"Apply":{"name":name.wire(),"args":args.wire()}}),
Self::Param{name} => json!({"Param":{"name":name.wire()}}),
}
} }

impl Wire for types::CtorFieldSrc { fn wire(&self)->Value {
json!({"name":self.name.wire(),"ty":self.ty.wire()})
} }

impl Wire for types::CtorDeclSrc { fn wire(&self)->Value {
json!({"name":self.name.wire(),"fields":self.fields.wire()})
} }

impl Wire for types::TypeDeclSrc { fn wire(&self)->Value {
json!({"name":self.name.wire(),"parameters":self.parameters.wire(),"ctors":self.ctors.wire()})
} }

impl Wire for types::ExtTypeCapKind { fn wire(&self)->Value {
match self {
Self::TypeRows => json!("TypeRows"),
Self::ConstructorRows => json!("ConstructorRows"),
Self::ConstructorFieldSlots => json!("ConstructorFieldSlots"),
Self::PatternChildSlots => json!("PatternChildSlots"),
Self::MatchPlanRows => json!("MatchPlanRows"),
Self::PlanCaseSlots => json!("PlanCaseSlots"),
Self::FieldPathSlots => json!("FieldPathSlots"),
Self::InputRows => json!("InputRows"),
Self::AssetRows => json!("AssetRows"),
Self::StateRows => json!("StateRows"),
Self::AccountRows => json!("AccountRows"),
Self::LiabilityRows => json!("LiabilityRows"),
Self::AwaitSiteRows => json!("AwaitSiteRows"),
Self::AwaitBinderSlots => json!("AwaitBinderSlots"),
Self::BalanceCells => json!("BalanceCells"),
Self::JournalRows => json!("JournalRows"),
Self::PendingOpRows => json!("PendingOpRows"),
Self::ObservationsPerInvocation => json!("ObservationsPerInvocation"),
Self::MaxLifetimeWork => json!("MaxLifetimeWork"),
Self::ResumeRows => json!("ResumeRows"),
Self::EffectOrdinalWidth => json!("EffectOrdinalWidth"),
Self::SegmentOrdinalWidth => json!("SegmentOrdinalWidth"),
}
} }

impl Wire for types::ExtTypeRejectReason { fn wire(&self)->Value {
match self {
Self::DuplicateTypeName => json!("DuplicateTypeName"),
Self::ReservedTypeName => json!("ReservedTypeName"),
Self::UnknownType => json!("UnknownType"),
Self::TypeArity => json!("TypeArity"),
Self::ConstructorArity => json!("ConstructorArity"),
Self::DuplicateCtorName => json!("DuplicateCtorName"),
Self::DuplicateFieldName => json!("DuplicateFieldName"),
Self::UnknownCtor => json!("UnknownCtor"),
Self::BuiltinConstruct => json!("BuiltinConstruct"),
Self::PatternArity => json!("PatternArity"),
Self::DuplicatePatternBinder => json!("DuplicatePatternBinder"),
Self::ConstantFieldType => json!("ConstantFieldType"),
}
} }

impl Wire for types::ExtTypeFeatureKind { fn wire(&self)->Value {
match self {
Self::TypeVariableInSignature => json!("TypeVariableInSignature"),
}
} }

impl Wire for types::ExtTypeBuildError { fn wire(&self)->Value {
match self {
Self::CoreTooling(v0) => json!({"CoreTooling":v0.wire()}),
Self::ExtCapacityNotAdmitted{kind,needed,limit,locator} => json!({"ExtCapacityNotAdmitted":{"kind":kind.wire(),"needed":needed.wire(),"limit":limit.wire(),"locator":locator.wire()}}),
Self::ExtAdmissionRejected{reason,detail,locator} => json!({"ExtAdmissionRejected":{"reason":reason.wire(),"detail":detail.wire(),"locator":locator.wire()}}),
Self::ExtUnsupportedFeature{feature,locator} => json!({"ExtUnsupportedFeature":{"feature":feature.wire(),"locator":locator.wire()}}),
Self::Host(v0) => json!({"Host":v0.wire()}),
}
} }

impl Wire for ExtCapacitiesC { fn wire(&self)->Value {
json!({"plan_case_slots":self.plan_case_slots.wire(),"field_path_slots":self.field_path_slots.wire(),"pattern_child_slots":self.pattern_child_slots.wire(),"input_rows":self.input_rows.wire(),"asset_rows":self.asset_rows.wire(),"state_rows":self.state_rows.wire(),"account_rows":self.account_rows.wire(),"liability_rows":self.liability_rows.wire(),"await_site_rows":self.await_site_rows.wire(),"await_binder_slots":self.await_binder_slots.wire(),"balance_cells":self.balance_cells.wire(),"journal_rows":self.journal_rows.wire(),"pending_op_rows":self.pending_op_rows.wire(),"observations_per_invocation":self.observations_per_invocation.wire(),"max_lifetime_work":self.max_lifetime_work.wire(),"resume_rows":self.resume_rows.wire()})
} }

impl Wire for WorkMapEntryC { fn wire(&self)->Value {
json!({"source_match":self.source_match.wire(),"live_arms":self.live_arms.wire(),"field_path":self.field_path.wire(),"events_per_visit":self.events_per_visit.wire()})
} }

impl Wire for PreparationReportC { fn wire(&self)->Value {
json!({"work_map":self.work_map.wire()})
} }

impl Wire for ExtDomain { fn wire(&self)->Value {
json!({"grammar_domain":self.grammar_domain.wire(),"finance":self.finance.wire()})
} }

impl Wire for ext::ExtCompileConfig { fn wire(&self)->Value {
json!({"bounds":self.bounds.wire(),"layout":self.layout.wire(),"domain":self.domain.wire(),"fuel":self.fuel.wire(),"stack":self.stack.wire()})
} }

impl Wire for ext::ExtCompileFailure { fn wire(&self)->Value {
match self {
Self::Preparation(v0) => json!({"Preparation":v0.wire()}),
Self::Genesis(v0) => json!({"Genesis":v0.wire()}),
}
} }
