impl OwnerWire for Tooling {
    fn owner_wire(&self) -> Value {
        match self {
            Self::AdmissionRejected { reason, detail, locator } => json!({"AdmissionRejected": {"reason": reason.owner_wire(), "detail": detail.owner_wire(), "locator": locator.owner_wire()}}),
            Self::CapacityNotAdmitted { kind, needed, limit, locator } => json!({"CapacityNotAdmitted": {"kind": kind.owner_wire(), "needed": needed.owner_wire(), "limit": limit.owner_wire(), "locator": locator.owner_wire()}}),
            Self::UnsupportedFeature { feature, locator } => json!({"UnsupportedFeature": {"feature": feature.owner_wire(), "locator": locator.owner_wire()}}),
        }
    }
}
