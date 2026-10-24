impl OwnerWire for AdmissionReason {
    fn owner_wire(&self) -> Value {
        match self {
            Self::LayoutWidthTooSmall => json!("LayoutWidthTooSmall"),
            Self::ProgramViewMalformed => json!("ProgramViewMalformed"),
            Self::GrammarDomainMismatch => json!("GrammarDomainMismatch"),
            Self::ProfileMismatch => json!("ProfileMismatch"),
            Self::LayoutBoundsMismatch => json!("LayoutBoundsMismatch"),
        }
    }
}
