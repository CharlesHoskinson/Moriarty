impl OwnerWire for CapacityKind {
    fn owner_wire(&self) -> Value {
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
            Self::CounterWidth { field } => json!({"CounterWidth": {"field": field.owner_wire()}}),
        }
    }
}
