#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepInvClauseId { SI01, SI02, SI03, SI04, SI05, SI06, SI07, SI08,
    SI09, SI10, SI11, SI12, SI13, SI14, SI15, SI16, SI17, SI18, SI19, SI20,
    SI21, SI22, SI23, SI24 }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewCheck { V1, V2, V3, V4, V5, V6, V7, V8 }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CounterWidthField { WorkCounter, HelperCounter, ReferenceCursor }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CapacityKind { CodeRows, FunctionRows, CodeChildSlots, ParameterSlots,
    PatternRows, MatchArmRows, ConstantNodes, ConstantFieldSlots, MaxArity,
    NameRows, NameBytes, OccurrencePathSlots, ValueSlots, MaxReferenceWork,
    Frames, SerializationBytes, PhysicalBoundaryEnvelope, StepCap,
    CounterWidth { field: CounterWidthField } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeatureKind { GenericAdt, FinancialSyntax, FinancialInput,
    BoundedEncodingWithoutWidths, BoundarySection }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdmissionReason { LayoutWidthTooSmall, ProgramViewMalformed,
    GrammarDomainMismatch, ProfileMismatch, LayoutBoundsMismatch }
    // GrammarDomainMismatch/ProfileMismatch: reserved, no B-core producer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HostViolationKind { StepInvViolated, SelectUndefined, ReadOutOfBounds,
    WrongVariant, DisabledRuleReached, ExtensionRuleReached,
    PhysicalCapacityExceeded, ArithmeticUndefined }
    // R4 enum only. Owner maps a foreign family RuleIdC and a rejected plan
    // to WrongVariant{clause:None,rule:None,locator:Some("foreign rule for family"|"invalid plan")}.
